//! Tauri application library

mod clipboard;
mod config;
mod history;

use std::sync::Mutex;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

/// Application state shared across commands
pub struct AppState {
    pub history: Mutex<history::History>,
    pub config: config::Config,
}

/// Get history entries, optionally filtered by query
#[tauri::command]
fn get_history(query: String, state: tauri::State<'_, AppState>) -> Vec<history::HistoryEntry> {
    state.history.lock().unwrap().search(&query)
}

/// Save text to history and copy to clipboard
#[tauri::command]
fn paste_and_save(text: String, state: tauri::State<'_, AppState>) -> Result<(), String> {
    if text.trim().is_empty() {
        return Ok(());
    }

    // Save to history
    state.history.lock().unwrap().add(text.clone())?;

    // Copy to clipboard
    clipboard::copy_to_clipboard(&text)?;

    Ok(())
}

/// Simulate Ctrl+V to paste clipboard content
#[tauri::command]
fn simulate_paste() -> Result<(), String> {
    // Wait for window to hide and focus to return to previous app
    std::thread::sleep(std::time::Duration::from_millis(100));
    clipboard::simulate_paste()
}

/// Get current configuration
#[tauri::command]
fn get_config(state: tauri::State<'_, AppState>) -> config::Config {
    state.config.clone()
}

/// Toggle window visibility
fn toggle_window(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        if window.is_visible().unwrap_or(false) {
            let _ = window.hide();
        } else {
            let _ = window.show();
            let _ = window.set_focus();
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Load configuration
    let config = config::Config::load().expect("Failed to load config");

    // Initialize history
    let history_path = history::History::default_path().expect("Failed to get history path");
    let history = history::History::new(history_path, config.history.max_entries)
        .expect("Failed to initialize history");

    tauri::Builder::default()
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, _shortcut, event| {
                    if event.state() == ShortcutState::Pressed {
                        toggle_window(app);
                    }
                })
                .build(),
        )
        .setup(|app| {
            // Setup system tray
            let show_item = MenuItem::with_id(app, "show", "Show (Ctrl+Shift+Space)", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_item, &quit_item])?;

            let _tray = TrayIconBuilder::new()
                .icon(tauri::image::Image::from_bytes(include_bytes!("../icons/32x32.png")).expect("Failed to load icon"))
                .menu(&menu)
                .tooltip("prompt-line-rs (Ctrl+Shift+Space)")
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        toggle_window(app);
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        toggle_window(app);
                    }
                })
                .build(app)?;

            // Define shortcuts to try (in order of preference)
            // Primary: Ctrl+Shift+Space (matches prompt-line's Cmd+Shift+Space on Mac)
            let shortcuts = [
                (
                    Some(Modifiers::CONTROL | Modifiers::SHIFT),
                    Code::Space,
                    "Ctrl+Shift+Space",
                ),
                (
                    Some(Modifiers::SUPER | Modifiers::SHIFT),
                    Code::Space,
                    "Win+Shift+Space",
                ),
                (Some(Modifiers::ALT), Code::Space, "Alt+Space"),
                (
                    Some(Modifiers::CONTROL | Modifiers::ALT),
                    Code::KeyP,
                    "Ctrl+Alt+P",
                ),
            ];

            // Try to register shortcuts with fallback
            let mut registered = false;
            for (modifiers, code, name) in shortcuts {
                let shortcut = Shortcut::new(modifiers, code);
                if app.global_shortcut().register(shortcut).is_ok() {
                    println!("Registered hotkey: {}", name);
                    registered = true;
                    break;
                }
            }

            if !registered {
                eprintln!("Warning: Failed to register any hotkey");
            }

            Ok(())
        })
        .manage(AppState {
            history: Mutex::new(history),
            config,
        })
        .invoke_handler(tauri::generate_handler![
            get_history,
            paste_and_save,
            simulate_paste,
            get_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
