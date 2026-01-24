# prompt-line-rs

English | [日本語](README.ja.md)

A floating text input tool for Windows, inspired by [prompt-line](https://github.com/nkmr-jp/prompt-line). Quickly launch with a global hotkey, type your text, and paste it to any application with a single keystroke.

![UI](./prompt-line-rs.png)

## Features

- **Global Hotkey**: Launch with `Ctrl+Shift+Space` from anywhere
- **Quick Paste**: Press `Ctrl+Enter` to copy text and paste to the previously focused app
- **Voice Input**: Auto-trigger Windows voice input (Win+H) when window opens
- **Readline Bindings**: Emacs-style editing shortcuts (Ctrl+A/E, Ctrl+K/U, etc.)
- **History Navigation**: Use `Ctrl+P`/`Ctrl+N` to navigate through history
- **History Search**: Press `Ctrl+R` to search your input history
- **System Tray**: Runs quietly in system tray, always ready
- **Japanese Support**: Full Japanese input with IME support
- **Customizable**: Configure all shortcuts via Settings UI

## Installation

### Installer (Recommended)

Download from the [releases](https://github.com/jl1nie/prompt-line-rs/releases) page:

- **`prompt-line-rs_x.x.x_x64-setup.exe`** - NSIS installer (recommended)
- **`prompt-line-rs_x.x.x_x64_en-US.msi`** - MSI installer

### Portable

Download `prompt-line-rs.exe` and run directly. No installation required.

### Build from Source

```bash
# Clone the repository
git clone https://github.com/jl1nie/prompt-line-rs.git
cd prompt-line-rs

# Install dependencies
npm install

# Build release version
npm run tauri build

# Outputs:
#   src-tauri/target/release/prompt-line-rs.exe
#   src-tauri/target/release/bundle/nsis/*.exe
#   src-tauri/target/release/bundle/msi/*.msi
```

## Usage

1. **Launch**: Run `prompt-line-rs.exe` (icon appears in system tray)
2. **Show Window**: Press `Ctrl+Shift+Space`
3. **Type**: Enter your text (use readline shortcuts for editing)
4. **Paste**: Press `Ctrl+Enter` to paste to the previously focused application
5. **Navigate History**: Use `Ctrl+P` (previous) / `Ctrl+N` (next)
6. **Search History**: Press `Ctrl+R` and type to filter
7. **Close**: Press `Escape` to hide (app stays in system tray)

## Keyboard Shortcuts

### App Shortcuts

| Shortcut           | Action                              |
|--------------------|-------------------------------------|
| `Ctrl+Shift+Space` | Show/hide window (global)           |
| `Ctrl+Enter`       | Copy text and paste to previous app |
| `Escape`           | Close window / Exit search          |

### Readline Bindings

| Shortcut   | Action              |
|------------|---------------------|
| `Ctrl+P`   | Previous history    |
| `Ctrl+N`   | Next history        |
| `Ctrl+R`   | Search history      |
| `Ctrl+A`   | Line start          |
| `Ctrl+E`   | Line end            |
| `Ctrl+B`   | Char back           |
| `Ctrl+F`   | Char forward        |
| `Alt+B`    | Word back           |
| `Alt+F`    | Word forward        |
| `Ctrl+K`   | Kill to end         |
| `Ctrl+U`   | Kill to start       |
| `Ctrl+W`   | Kill word back      |
| `Ctrl+D`   | Delete char         |
| `Ctrl+Y`   | Yank (paste)        |
| `Ctrl+L`   | Clear text          |

All shortcuts are configurable via Settings (right-click tray icon).

**Fallback hotkeys**: If `Ctrl+Shift+Space` is unavailable, these are tried in order:

- `Win+Shift+Space`
- `Alt+Space`
- `Ctrl+Alt+P`

## Configuration

Configuration file location:

```text
%APPDATA%\prompt-line\prompt-line-rs\config\config.toml
```

Default settings:

```toml
[shortcuts]
launch = "Ctrl+Shift+Space"
paste = "Ctrl+Enter"
close = "Escape"
history_next = "Ctrl+n"
history_prev = "Ctrl+p"
search = "Ctrl+r"
clear = "Ctrl+l"
# Readline cursor movement
line_start = "Ctrl+a"
line_end = "Ctrl+e"
char_back = "Ctrl+b"
char_forward = "Ctrl+f"
word_back = "Alt+b"
word_forward = "Alt+f"
# Readline kill/delete
kill_to_end = "Ctrl+k"
kill_to_start = "Ctrl+u"
kill_word_back = "Ctrl+w"
delete_char = "Ctrl+d"
yank = "Ctrl+y"

[history]
max_entries = 1000

[window]
font_size = 14.0
history_font_size = 12.0
history_lines = 3
textarea_rows = 3
textarea_cols = 60

[behavior]
simulate_paste_shortcut = "Ctrl+V"

[[behavior.app_overrides]]
process_name = "alacritty.exe"
shortcut = "Ctrl+Shift+V"

[[behavior.app_overrides]]
process_name = "wezterm-gui.exe"
shortcut = "Ctrl+Shift+V"

[voice]
enabled = false
delay_ms = 500
```

### Behavior Section

The `[behavior]` section configures how text is pasted to target applications:

- **`simulate_paste_shortcut`**: Default paste shortcut sent to applications (default: `Ctrl+V`)
- **`app_overrides`**: Per-app paste shortcuts based on process name

This allows seamless pasting into terminal emulators like **Alacritty** and **WezTerm** that use `Ctrl+Shift+V` instead of `Ctrl+V`. The app automatically detects the target application and uses the appropriate shortcut.

### Voice Section

The `[voice]` section configures automatic Windows voice input:

- **`enabled`**: Enable/disable the voice input toggle in the main window (default: `false`)
- **`delay_ms`**: Delay in milliseconds before triggering voice input (default: `500`)

When enabled, a microphone toggle appears in the main window. When the toggle is ON, the app automatically triggers Windows voice input (Win+H) each time the window opens.

## History

History is saved to:

```text
%APPDATA%\prompt-line\prompt-line-rs\data\history.jsonl
```

You can clear all history from Settings (right-click tray icon → Settings → Clear All History).

## Use Cases

- CLI-based AI coding agents (Claude Code, Gemini CLI, etc.)
- Chat applications where Enter sends messages prematurely
- Text editors with slow input response
- Japanese input in applications with limited IME support

## Technical Details

Built with:

- **Tauri 2.0**: Lightweight desktop app framework
- **TypeScript/Vite**: Modern frontend tooling
- **Rust**: Backend with native Windows API integration
- **WebView2**: System WebView for minimal bundle size

## System Requirements

- Windows 10/11 (64-bit)
- WebView2 Runtime (included in Windows 11, auto-installed on Windows 10)

## License

MIT

## Acknowledgments

Inspired by [prompt-line](https://github.com/nkmr-jp/prompt-line) by nkmr-jp.
