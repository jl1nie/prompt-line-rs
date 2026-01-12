//! Tauri application library

mod clipboard;
mod config;
mod history;

use std::sync::Mutex;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, WebviewUrl, WebviewWindowBuilder,
};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

/// Application state shared across commands
pub struct AppState {
    pub history: Mutex<history::History>,
    pub config: Mutex<config::Config>,
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
    state.config.lock().unwrap().clone()
}

/// Get draft file path
fn draft_path() -> Result<std::path::PathBuf, String> {
    let config_dir = directories::ProjectDirs::from("com", "prompt-line", "prompt-line-rs")
        .ok_or_else(|| "Failed to get config directory".to_string())?;
    Ok(config_dir.data_dir().join("draft.txt"))
}

/// Save draft text
#[tauri::command]
fn save_draft(text: String) -> Result<(), String> {
    let path = draft_path()?;

    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    std::fs::write(&path, &text).map_err(|e| format!("Failed to save draft: {}", e))?;

    Ok(())
}

/// Load draft text
#[tauri::command]
fn load_draft() -> Result<String, String> {
    let path = draft_path()?;

    if !path.exists() {
        return Ok(String::new());
    }

    std::fs::read_to_string(&path).map_err(|e| format!("Failed to load draft: {}", e))
}

/// Clear draft
#[tauri::command]
fn clear_draft() -> Result<(), String> {
    let path = draft_path()?;

    if path.exists() {
        std::fs::remove_file(&path).map_err(|e| format!("Failed to clear draft: {}", e))?;
    }

    Ok(())
}

/// Save configuration and apply window size
#[tauri::command]
fn save_config(
    new_config: config::Config,
    state: tauri::State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    // Resize main window if it exists
    if let Some(window) = app.get_webview_window("main") {
        let width = new_config.window.width_pixels();
        let height = new_config.window.height_pixels();
        let size = tauri::LogicalSize::new(width, height);
        let _ = window.set_size(size);
    }

    new_config.save()?;
    let mut config = state.config.lock().unwrap();
    *config = new_config;
    Ok(())
}

/// Show settings window
fn show_settings_window(app: &tauri::AppHandle) {
    // Check if settings window already exists
    if let Some(window) = app.get_webview_window("settings") {
        let _ = window.show();
        let _ = window.set_focus();
        return;
    }

    // Create new settings window
    let _window =
        WebviewWindowBuilder::new(app, "settings", WebviewUrl::App("settings.html".into()))
            .title("Settings - prompt-line-rs")
            .inner_size(500.0, 450.0)
            .resizable(true)
            .center()
            .build();
}

/// Parse a shortcut string like "Ctrl+Shift+Space" into Modifiers and Code
fn parse_shortcut(shortcut_str: &str) -> Option<(Option<Modifiers>, Code)> {
    let parts: Vec<&str> = shortcut_str.split('+').map(|s| s.trim()).collect();
    if parts.is_empty() {
        return None;
    }

    let mut modifiers = Modifiers::empty();
    let mut key_code = None;

    for part in &parts {
        match part.to_lowercase().as_str() {
            "ctrl" | "control" => modifiers |= Modifiers::CONTROL,
            "shift" => modifiers |= Modifiers::SHIFT,
            "alt" => modifiers |= Modifiers::ALT,
            "win" | "super" | "cmd" | "command" => modifiers |= Modifiers::SUPER,
            "space" => key_code = Some(Code::Space),
            "enter" | "return" => key_code = Some(Code::Enter),
            "escape" | "esc" => key_code = Some(Code::Escape),
            "tab" => key_code = Some(Code::Tab),
            "a" => key_code = Some(Code::KeyA),
            "b" => key_code = Some(Code::KeyB),
            "c" => key_code = Some(Code::KeyC),
            "d" => key_code = Some(Code::KeyD),
            "e" => key_code = Some(Code::KeyE),
            "f" => key_code = Some(Code::KeyF),
            "g" => key_code = Some(Code::KeyG),
            "h" => key_code = Some(Code::KeyH),
            "i" => key_code = Some(Code::KeyI),
            "j" => key_code = Some(Code::KeyJ),
            "k" => key_code = Some(Code::KeyK),
            "l" => key_code = Some(Code::KeyL),
            "m" => key_code = Some(Code::KeyM),
            "n" => key_code = Some(Code::KeyN),
            "o" => key_code = Some(Code::KeyO),
            "p" => key_code = Some(Code::KeyP),
            "q" => key_code = Some(Code::KeyQ),
            "r" => key_code = Some(Code::KeyR),
            "s" => key_code = Some(Code::KeyS),
            "t" => key_code = Some(Code::KeyT),
            "u" => key_code = Some(Code::KeyU),
            "v" => key_code = Some(Code::KeyV),
            "w" => key_code = Some(Code::KeyW),
            "x" => key_code = Some(Code::KeyX),
            "y" => key_code = Some(Code::KeyY),
            "z" => key_code = Some(Code::KeyZ),
            _ => {}
        }
    }

    key_code.map(|code| {
        let mods = if modifiers.is_empty() {
            None
        } else {
            Some(modifiers)
        };
        (mods, code)
    })
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
    let launch_shortcut = config.shortcuts.launch.clone();

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
        .setup(move |app| {
            let launch_shortcut = launch_shortcut.clone();

            // Setup system tray
            let show_label = format!("Show ({})", &launch_shortcut);
            let show_item = MenuItem::with_id(app, "show", &show_label, true, None::<&str>)?;
            let settings_item =
                MenuItem::with_id(app, "settings", "Settings...", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_item, &settings_item, &quit_item])?;

            let tooltip = format!("prompt-line-rs ({})", &launch_shortcut);
            let _tray = TrayIconBuilder::new()
                .icon(
                    tauri::image::Image::from_bytes(include_bytes!("../icons/32x32.png"))
                        .expect("Failed to load icon"),
                )
                .menu(&menu)
                .tooltip(&tooltip)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        toggle_window(app);
                    }
                    "settings" => {
                        show_settings_window(app);
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

            // Try to register the configured shortcut first
            let mut registered = false;

            if let Some((modifiers, code)) = parse_shortcut(&launch_shortcut) {
                let shortcut = Shortcut::new(modifiers, code);
                if app.global_shortcut().register(shortcut).is_ok() {
                    println!("Registered hotkey: {}", launch_shortcut);
                    registered = true;
                }
            }

            // Fallback shortcuts if configured one fails
            if !registered {
                let fallback_shortcuts = [
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

                for (modifiers, code, name) in fallback_shortcuts {
                    let shortcut = Shortcut::new(modifiers, code);
                    if app.global_shortcut().register(shortcut).is_ok() {
                        println!("Registered fallback hotkey: {}", name);
                        registered = true;
                        break;
                    }
                }
            }

            if !registered {
                eprintln!("Warning: Failed to register any hotkey");
            }

            Ok(())
        })
        .manage(AppState {
            history: Mutex::new(history),
            config: Mutex::new(config),
        })
        .invoke_handler(tauri::generate_handler![
            get_history,
            paste_and_save,
            simulate_paste,
            get_config,
            save_config,
            save_draft,
            load_draft,
            clear_draft,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
