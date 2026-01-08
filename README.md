# prompt-line-rs

A floating text input tool for Windows, written in pure Rust. Quickly launch with a global hotkey, type your text, and paste it to any application with a single keystroke.

## Features

- **Global Hotkey**: Launch with `Alt+Space` (or alternative if in use) from anywhere
- **Quick Paste**: Press `Ctrl+Enter` to copy text and paste to the previously focused app
- **History**: Automatically saves all your inputs with search functionality
- **Japanese Support**: Full Japanese input with IME support (日本語入力対応)
- **Customizable**: Configure window size, font size, and maximum history entries
- **Lightweight**: Only ~5MB standalone executable
- **Pure Rust**: Fast, safe, and efficient

## Installation

### Pre-built Binary

Download `prompt-line-rs.exe` from the releases page and run it directly.

### Build from Source

```bash
# Clone the repository
git clone https://github.com/minoru/prompt-line-rs.git
cd prompt-line-rs

# Build release version
cargo build --release

# The executable will be at target/release/prompt-line-rs.exe
```

## Usage

1. **Launch**: Run `prompt-line-rs.exe`
2. **Show Window**: Press `Alt+Space` (configurable)
3. **Type**: Enter your text in the floating window
4. **Paste**: Press `Ctrl+Enter` to copy and paste to the previously focused application
5. **History**: Press `Ctrl+H` to toggle history panel and search previous inputs
6. **Close**: Press `Esc` to hide the window (app keeps running in background)

## Keyboard Shortcuts

| Shortcut     | Action                                  |
|--------------|-----------------------------------------|
| `Alt+Space`  | Show/hide floating window               |
| `Ctrl+Enter` | Copy text and paste to previous app     |
| `Ctrl+H`     | Toggle history panel                    |
| `Esc`        | Close window                            |

**Note**: If `Alt+Space` is already in use, the app will automatically try these alternatives:

- `Ctrl+Shift+Space`
- `Win+Shift+Space`
- `Ctrl+Alt+P`

## Configuration

Configuration file is located at:

- Windows: `%APPDATA%\prompt-line\prompt-line-rs\config\config.toml`

Default configuration:

```toml
[shortcuts]
launch = "Alt+Space"
paste = "Ctrl+Enter"
close = "Escape"

[history]
max_entries = 1000

[window]
width = 600.0
height = 400.0
font_size = 16.0
```

## History

History is automatically saved to:

- Windows: `%APPDATA%\prompt-line\prompt-line-rs\data\history.jsonl`

Each entry includes:

- The text content
- Timestamp

History can be searched using the built-in search functionality (`Ctrl+H`).

## Use Cases

Perfect for:

- CLI-based AI coding agents (Claude Code, Gemini CLI, etc.)
- Chat applications where Enter sends messages prematurely
- Text editors with slow input response
- Any situation where you need to compose text before pasting
- Japanese input in applications with limited IME support

## Technical Details

Built with:

- **eframe/egui**: Lightweight GUI framework with IME support
- **arboard**: Cross-platform clipboard access
- **windows**: Native Windows API for keyboard simulation and hotkey registration
- **serde/toml/serde_json**: Configuration and history management
- **chrono**: Timestamp handling
- **directories**: Platform-specific path resolution

## License

MIT

## Acknowledgments

Inspired by [prompt-line](https://github.com/nkmr-jp/prompt-line) by nkmr-jp.
