//! Clipboard operations module

use arboard::Clipboard;

/// Copy text to clipboard and return Result
pub fn copy_to_clipboard(text: &str) -> Result<(), String> {
    let mut clipboard =
        Clipboard::new().map_err(|e| format!("Failed to access clipboard: {}", e))?;

    // Clear clipboard first to remove any existing content (including images)
    clipboard
        .clear()
        .map_err(|e| format!("Failed to clear clipboard: {}", e))?;

    clipboard
        .set_text(text.to_string())
        .map_err(|e| format!("Failed to set clipboard text: {}", e))?;

    Ok(())
}

/// Parse shortcut string and simulate keypress
/// Supports: Ctrl, Shift, Alt modifiers with a single key (e.g., "Ctrl+V", "Ctrl+Shift+V")
#[cfg(windows)]
pub fn simulate_paste(shortcut: &str) -> Result<(), String> {
    use windows::Win32::UI::Input::KeyboardAndMouse::{
        SendInput, INPUT, VIRTUAL_KEY, VK_CONTROL, VK_LWIN, VK_MENU, VK_SHIFT,
    };

    // Parse shortcut string
    let parts: Vec<&str> = shortcut.split('+').map(|s| s.trim()).collect();
    if parts.is_empty() {
        return Err("Empty shortcut".to_string());
    }

    let mut modifiers: Vec<VIRTUAL_KEY> = Vec::new();
    let mut main_key: Option<VIRTUAL_KEY> = None;

    for part in parts {
        let upper = part.to_uppercase();
        match upper.as_str() {
            "CTRL" | "CONTROL" => modifiers.push(VK_CONTROL),
            "SHIFT" => modifiers.push(VK_SHIFT),
            "ALT" => modifiers.push(VK_MENU),
            "WIN" | "SUPER" | "META" => modifiers.push(VK_LWIN),
            _ => {
                // Assume it's the main key
                main_key = Some(parse_key(&upper)?);
            }
        }
    }

    let main_key = main_key.ok_or_else(|| "No main key specified in shortcut".to_string())?;

    // Build input sequence: modifiers down, key down, key up, modifiers up (reverse order)
    let mut inputs: Vec<INPUT> = Vec::new();

    // Modifiers down
    for &modifier in &modifiers {
        inputs.push(create_key_input(modifier, false));
    }

    // Main key down
    inputs.push(create_key_input(main_key, false));

    // Main key up
    inputs.push(create_key_input(main_key, true));

    // Modifiers up (reverse order)
    for &modifier in modifiers.iter().rev() {
        inputs.push(create_key_input(modifier, true));
    }

    unsafe {
        let result = SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);

        if result == 0 {
            return Err("Failed to send input events".to_string());
        }
    }

    Ok(())
}

/// Trigger Windows voice input (Win+H)
/// Spawns a thread with delay for better system shortcut handling
#[cfg(windows)]
pub fn trigger_voice_input(delay_ms: u32) -> Result<(), String> {
    std::thread::spawn(move || {
        use std::thread::sleep;
        use std::time::Duration;
        use windows::Win32::UI::Input::KeyboardAndMouse::{
            keybd_event, KEYEVENTF_EXTENDEDKEY, KEYEVENTF_KEYUP,
        };

        // Virtual key codes
        const VK_LWIN: u8 = 0x5B;
        const VK_H: u8 = 0x48;
        const VK_CONTROL: u8 = 0x11;
        const VK_SHIFT: u8 = 0x10;
        const VK_MENU: u8 = 0x12; // Alt key

        // Wait for window to fully settle
        sleep(Duration::from_millis(delay_ms as u64));

        unsafe {
            // Release any modifier keys that might be held from the hotkey
            keybd_event(VK_CONTROL, 0, KEYEVENTF_KEYUP, 0);
            keybd_event(VK_SHIFT, 0, KEYEVENTF_KEYUP, 0);
            keybd_event(VK_MENU, 0, KEYEVENTF_KEYUP, 0);
            sleep(Duration::from_millis(50));

            // Win key down
            keybd_event(VK_LWIN, 0, KEYEVENTF_EXTENDEDKEY, 0);
            sleep(Duration::from_millis(50));

            // H key down
            keybd_event(VK_H, 0, Default::default(), 0);
            sleep(Duration::from_millis(50));

            // H key up
            keybd_event(VK_H, 0, KEYEVENTF_KEYUP, 0);
            sleep(Duration::from_millis(50));

            // Win key up
            keybd_event(VK_LWIN, 0, KEYEVENTF_EXTENDEDKEY | KEYEVENTF_KEYUP, 0);
        }
    });

    Ok(())
}

/// Trigger Windows voice input (Win+H) - non-Windows stub
#[cfg(not(windows))]
pub fn trigger_voice_input(_delay_ms: u32) -> Result<(), String> {
    Err("Voice input is only supported on Windows".to_string())
}

#[cfg(windows)]
fn create_key_input(
    key: windows::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY,
    key_up: bool,
) -> windows::Win32::UI::Input::KeyboardAndMouse::INPUT {
    use windows::Win32::UI::Input::KeyboardAndMouse::{
        INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYBD_EVENT_FLAGS, KEYEVENTF_EXTENDEDKEY,
        KEYEVENTF_KEYUP, VK_LWIN, VK_RWIN,
    };

    // Extended keys need KEYEVENTF_EXTENDEDKEY flag
    let is_extended = key == VK_LWIN || key == VK_RWIN;

    let mut flags = KEYBD_EVENT_FLAGS(0);
    if is_extended {
        flags |= KEYEVENTF_EXTENDEDKEY;
    }
    if key_up {
        flags |= KEYEVENTF_KEYUP;
    }

    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: key,
                wScan: 0,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    }
}

#[cfg(windows)]
fn parse_key(
    key: &str,
) -> Result<windows::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY, String> {
    use windows::Win32::UI::Input::KeyboardAndMouse::*;

    match key {
        // Letters
        "A" => Ok(VIRTUAL_KEY(0x41)),
        "B" => Ok(VIRTUAL_KEY(0x42)),
        "C" => Ok(VIRTUAL_KEY(0x43)),
        "D" => Ok(VIRTUAL_KEY(0x44)),
        "E" => Ok(VIRTUAL_KEY(0x45)),
        "F" => Ok(VIRTUAL_KEY(0x46)),
        "G" => Ok(VIRTUAL_KEY(0x47)),
        "H" => Ok(VIRTUAL_KEY(0x48)),
        "I" => Ok(VIRTUAL_KEY(0x49)),
        "J" => Ok(VIRTUAL_KEY(0x4A)),
        "K" => Ok(VIRTUAL_KEY(0x4B)),
        "L" => Ok(VIRTUAL_KEY(0x4C)),
        "M" => Ok(VIRTUAL_KEY(0x4D)),
        "N" => Ok(VIRTUAL_KEY(0x4E)),
        "O" => Ok(VIRTUAL_KEY(0x4F)),
        "P" => Ok(VIRTUAL_KEY(0x50)),
        "Q" => Ok(VIRTUAL_KEY(0x51)),
        "R" => Ok(VIRTUAL_KEY(0x52)),
        "S" => Ok(VIRTUAL_KEY(0x53)),
        "T" => Ok(VIRTUAL_KEY(0x54)),
        "U" => Ok(VIRTUAL_KEY(0x55)),
        "V" => Ok(VIRTUAL_KEY(0x56)),
        "W" => Ok(VIRTUAL_KEY(0x57)),
        "X" => Ok(VIRTUAL_KEY(0x58)),
        "Y" => Ok(VIRTUAL_KEY(0x59)),
        "Z" => Ok(VIRTUAL_KEY(0x5A)),
        // Numbers
        "0" => Ok(VIRTUAL_KEY(0x30)),
        "1" => Ok(VIRTUAL_KEY(0x31)),
        "2" => Ok(VIRTUAL_KEY(0x32)),
        "3" => Ok(VIRTUAL_KEY(0x33)),
        "4" => Ok(VIRTUAL_KEY(0x34)),
        "5" => Ok(VIRTUAL_KEY(0x35)),
        "6" => Ok(VIRTUAL_KEY(0x36)),
        "7" => Ok(VIRTUAL_KEY(0x37)),
        "8" => Ok(VIRTUAL_KEY(0x38)),
        "9" => Ok(VIRTUAL_KEY(0x39)),
        // Special keys
        "INSERT" => Ok(VK_INSERT),
        _ => Err(format!("Unknown key: {}", key)),
    }
}

#[cfg(not(windows))]
pub fn simulate_paste(_shortcut: &str) -> Result<(), String> {
    Err("Keyboard simulation is only supported on Windows".to_string())
}
