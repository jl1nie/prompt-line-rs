//! Platform-specific global hotkey support

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[cfg(windows)]
mod windows_impl {
    use super::*;
    use windows::Win32::UI::Input::KeyboardAndMouse::{
        RegisterHotKey, UnregisterHotKey, MOD_ALT, MOD_NOREPEAT,
    };
    use windows::Win32::UI::WindowsAndMessaging::{
        GetMessageW, MSG, WM_HOTKEY,
    };

    const HOTKEY_ID: i32 = 1;

    pub fn listen_hotkey(toggle_flag: Arc<AtomicBool>) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            // Register Alt+Space (VK_SPACE = 0x20)
            let result = RegisterHotKey(
                None,
                HOTKEY_ID,
                MOD_ALT | MOD_NOREPEAT,
                0x20, // VK_SPACE
            );

            if result.is_err() {
                return Err("Failed to register hotkey. Alt+Space may be in use by another application.".into());
            }

            println!("Registered hotkey: Alt+Space");

            let mut msg = MSG::default();
            loop {
                let ret = GetMessageW(&mut msg, None, 0, 0);
                if ret.0 <= 0 {
                    break;
                }

                if msg.message == WM_HOTKEY && msg.wParam.0 as i32 == HOTKEY_ID {
                    println!("Alt+Space pressed!");
                    toggle_flag.store(true, Ordering::SeqCst);
                }
            }

            let _ = UnregisterHotKey(None, HOTKEY_ID);
        }
        Ok(())
    }
}

#[cfg(not(windows))]
mod stub_impl {
    use super::*;

    pub fn listen_hotkey(_toggle_flag: Arc<AtomicBool>) -> Result<(), Box<dyn std::error::Error>> {
        eprintln!("Global hotkey not supported on this platform. Use the window directly.");
        // Just sleep forever to keep the thread alive
        loop {
            std::thread::sleep(std::time::Duration::from_secs(3600));
        }
    }
}

#[cfg(windows)]
pub use windows_impl::listen_hotkey;

#[cfg(not(windows))]
pub use stub_impl::listen_hotkey;
