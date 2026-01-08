//! Platform-specific global hotkey support

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[cfg(windows)]
mod windows_impl {
    use super::*;
    use windows::Win32::UI::Input::KeyboardAndMouse::{
        RegisterHotKey, UnregisterHotKey, MOD_ALT, MOD_CONTROL, MOD_NOREPEAT, MOD_SHIFT, MOD_WIN,
    };
    use windows::Win32::UI::WindowsAndMessaging::{
        GetMessageW, MSG, WM_HOTKEY,
    };

    pub fn listen_hotkey(toggle_flag: Arc<AtomicBool>) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            // Try multiple hotkey combinations in order of preference
            let hotkey_options = [
                (1, MOD_ALT | MOD_NOREPEAT, 0x20, "Alt+Space"),           // VK_SPACE
                (2, MOD_CONTROL | MOD_SHIFT | MOD_NOREPEAT, 0x20, "Ctrl+Shift+Space"),
                (3, MOD_WIN | MOD_SHIFT | MOD_NOREPEAT, 0x20, "Win+Shift+Space"),
                (4, MOD_CONTROL | MOD_ALT | MOD_NOREPEAT, 0x50, "Ctrl+Alt+P"), // VK_P
            ];

            let mut registered_hotkey = None;

            for (id, modifiers, vk, name) in hotkey_options.iter() {
                let result = RegisterHotKey(
                    None,
                    *id,
                    *modifiers,
                    *vk,
                );

                if result.is_ok() {
                    println!("✓ Registered hotkey: {}", name);
                    registered_hotkey = Some((*id, *name));
                    break;
                } else {
                    eprintln!("✗ Failed to register {}, trying next option...", name);
                }
            }

            if registered_hotkey.is_none() {
                return Err("Failed to register any hotkey. All hotkey combinations are in use.".into());
            }

            let (registered_id, hotkey_name) = registered_hotkey.unwrap();
            println!("Listening for hotkey: {}", hotkey_name);

            let mut msg = MSG::default();
            loop {
                let ret = GetMessageW(&mut msg, None, 0, 0);
                if ret.0 <= 0 {
                    break;
                }

                if msg.message == WM_HOTKEY && msg.wParam.0 as i32 == registered_id {
                    println!("Hotkey pressed: {}", hotkey_name);
                    toggle_flag.store(true, Ordering::SeqCst);
                }
            }

            let _ = UnregisterHotKey(None, registered_id);
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
