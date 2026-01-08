//! Simple prompt-line-rs using native dialogs for full IME support

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

mod clipboard;
mod config;
mod history;
mod hotkey;

#[cfg(windows)]
mod win_dialog {
    use windows::core::PCWSTR;
    use windows::Win32::UI::WindowsAndMessaging::{
        MessageBoxW, MB_OK, MB_ICONINFORMATION,
    };

    /// Show a simple message box (for testing)
    pub fn show_message(title: &str, message: &str) {
        let title_wide: Vec<u16> = title.encode_utf16().chain(std::iter::once(0)).collect();
        let msg_wide: Vec<u16> = message.encode_utf16().chain(std::iter::once(0)).collect();

        unsafe {
            MessageBoxW(
                None,
                PCWSTR(msg_wide.as_ptr()),
                PCWSTR(title_wide.as_ptr()),
                MB_OK | MB_ICONINFORMATION,
            );
        }
    }
}

fn main() {
    // Load config
    let config = config::Config::load()
        .expect("Failed to load configuration");

    println!("prompt-line-rs - Native Dialog Version");
    println!("Config loaded from: {:?}", config::Config::default_path().unwrap());

    // Initialize history
    let history_path = history::History::default_path()
        .expect("Failed to get history path");
    let mut history = history::History::new(history_path, config.history.max_entries)
        .expect("Failed to initialize history");

    // Setup hotkey
    let toggle_flag = Arc::new(AtomicBool::new(false));
    let toggle_flag_clone = toggle_flag.clone();

    std::thread::spawn(move || {
        if let Err(e) = hotkey::listen_hotkey(toggle_flag_clone) {
            eprintln!("Hotkey listener error: {}", e);
        }
    });

    println!("Ready! Press your hotkey to enter text...");

    // Main loop
    loop {
        if toggle_flag.swap(false, Ordering::SeqCst) {
            println!("Hotkey detected! Opening dialog...");

            // Try tinyfiledialogs first
            let result = tinyfiledialogs::input_box(
                "prompt-line-rs",
                "Enter text (OK to paste, Cancel to close):",
                ""
            );

            println!("Dialog closed, result: {:?}", result.is_some());

            if let Some(text) = result {
                if !text.is_empty() {
                    // Save to history
                    if let Err(e) = history.add(text.clone()) {
                        eprintln!("Failed to save to history: {}", e);
                    }

                    // Copy to clipboard
                    if let Err(e) = clipboard::copy_to_clipboard(&text) {
                        eprintln!("Failed to copy to clipboard: {}", e);
                        continue;
                    }

                    println!("Copied to clipboard: {} chars", text.len());

                    // Wait a bit
                    std::thread::sleep(Duration::from_millis(200));

                    // Simulate paste
                    if let Err(e) = clipboard::simulate_paste() {
                        eprintln!("Failed to simulate paste: {}", e);
                    } else {
                        println!("Pasted successfully");
                    }
                }
            }
        }

        std::thread::sleep(Duration::from_millis(50));
    }
}
