//! Configuration management module

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_shortcuts")]
    pub shortcuts: Shortcuts,

    #[serde(default = "default_history")]
    pub history: HistoryConfig,

    #[serde(default = "default_window")]
    pub window: WindowConfig,

    #[serde(default = "default_behavior")]
    pub behavior: BehaviorConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shortcuts {
    /// Global hotkey to show/hide window (Cmd+Shift+Space on Mac)
    #[serde(default = "default_launch")]
    pub launch: String,

    /// Paste text and close window (Cmd+Enter on Mac)
    #[serde(default = "default_paste")]
    pub paste: String,

    /// Close window without pasting
    #[serde(default = "default_close")]
    pub close: String,

    /// Navigate to next history item (readline: Ctrl+N)
    #[serde(default = "default_history_next")]
    pub history_next: String,

    /// Navigate to previous history item (readline: Ctrl+P)
    #[serde(default = "default_history_prev")]
    pub history_prev: String,

    /// Open search/filter
    #[serde(default = "default_search")]
    pub search: String,

    /// Clear text (readline: Ctrl+L)
    #[serde(default = "default_clear")]
    pub clear: String,

    // === Readline cursor movement ===
    /// Move to beginning of line (readline: Ctrl+A)
    #[serde(default = "default_line_start")]
    pub line_start: String,

    /// Move to end of line (readline: Ctrl+E)
    #[serde(default = "default_line_end")]
    pub line_end: String,

    /// Move back one character (readline: Ctrl+B)
    #[serde(default = "default_char_back")]
    pub char_back: String,

    /// Move forward one character (readline: Ctrl+F)
    #[serde(default = "default_char_forward")]
    pub char_forward: String,

    /// Move back one word (readline: Alt+B)
    #[serde(default = "default_word_back")]
    pub word_back: String,

    /// Move forward one word (readline: Alt+F)
    #[serde(default = "default_word_forward")]
    pub word_forward: String,

    // === Readline kill/delete ===
    /// Kill to end of line (readline: Ctrl+K)
    #[serde(default = "default_kill_to_end")]
    pub kill_to_end: String,

    /// Kill to start of line (readline: Ctrl+U)
    #[serde(default = "default_kill_to_start")]
    pub kill_to_start: String,

    /// Kill word backward (readline: Ctrl+W)
    #[serde(default = "default_kill_word_back")]
    pub kill_word_back: String,

    /// Delete character (readline: Ctrl+D)
    #[serde(default = "default_delete_char")]
    pub delete_char: String,

    /// Yank (paste from kill ring) (readline: Ctrl+Y)
    #[serde(default = "default_yank")]
    pub yank: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryConfig {
    #[serde(default = "default_max_entries")]
    pub max_entries: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    #[serde(default = "default_font_size")]
    pub font_size: f32,

    #[serde(default = "default_history_font_size")]
    pub history_font_size: f32,

    #[serde(default = "default_history_lines")]
    pub history_lines: u32,

    #[serde(default = "default_textarea_rows")]
    pub textarea_rows: u32,

    #[serde(default = "default_textarea_cols")]
    pub textarea_cols: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorConfig {
    /// Shortcut to simulate for pasting (sent to target application)
    #[serde(default = "default_simulate_paste_shortcut")]
    pub simulate_paste_shortcut: String,

    /// Per-app paste shortcut overrides
    #[serde(default = "default_app_overrides")]
    pub app_overrides: Vec<AppPasteOverride>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppPasteOverride {
    /// Process name (e.g., "alacritty.exe")
    pub process_name: String,
    /// Shortcut to use for this app (e.g., "Ctrl+Shift+V")
    pub shortcut: String,
}

// Default values (matching prompt-line + readline)
fn default_shortcuts() -> Shortcuts {
    Shortcuts {
        launch: "Ctrl+Shift+Space".to_string(), // Cmd+Shift+Space on Mac
        paste: "Ctrl+Enter".to_string(),        // Cmd+Enter on Mac
        close: "Escape".to_string(),
        history_next: "Ctrl+n".to_string(), // readline standard
        history_prev: "Ctrl+p".to_string(), // readline standard
        search: "Ctrl+r".to_string(),       // readline reverse search
        clear: "Ctrl+l".to_string(),
        // Readline cursor movement
        line_start: "Ctrl+a".to_string(),
        line_end: "Ctrl+e".to_string(),
        char_back: "Ctrl+b".to_string(),
        char_forward: "Ctrl+f".to_string(),
        word_back: "Alt+b".to_string(),
        word_forward: "Alt+f".to_string(),
        // Readline kill/delete
        kill_to_end: "Ctrl+k".to_string(),
        kill_to_start: "Ctrl+u".to_string(),
        kill_word_back: "Ctrl+w".to_string(),
        delete_char: "Ctrl+d".to_string(),
        yank: "Ctrl+y".to_string(),
    }
}

fn default_history() -> HistoryConfig {
    HistoryConfig { max_entries: 1000 }
}

fn default_window() -> WindowConfig {
    WindowConfig {
        font_size: default_font_size(),
        history_font_size: default_history_font_size(),
        history_lines: default_history_lines(),
        textarea_rows: default_textarea_rows(),
        textarea_cols: default_textarea_cols(),
    }
}

fn default_behavior() -> BehaviorConfig {
    BehaviorConfig {
        simulate_paste_shortcut: default_simulate_paste_shortcut(),
        app_overrides: default_app_overrides(),
    }
}

fn default_simulate_paste_shortcut() -> String {
    "Ctrl+V".to_string()
}

fn default_app_overrides() -> Vec<AppPasteOverride> {
    vec![
        AppPasteOverride {
            process_name: "alacritty.exe".to_string(),
            shortcut: "Ctrl+Shift+V".to_string(),
        },
        AppPasteOverride {
            process_name: "wezterm-gui.exe".to_string(),
            shortcut: "Ctrl+Shift+V".to_string(),
        },
        AppPasteOverride {
            process_name: String::new(),
            shortcut: String::new(),
        },
    ]
}

fn default_launch() -> String {
    "Ctrl+Shift+Space".to_string()
}

fn default_paste() -> String {
    "Ctrl+Enter".to_string()
}

fn default_close() -> String {
    "Escape".to_string()
}

fn default_history_next() -> String {
    "Ctrl+n".to_string()
}

fn default_history_prev() -> String {
    "Ctrl+p".to_string()
}

fn default_search() -> String {
    "Ctrl+r".to_string()
}

fn default_clear() -> String {
    "Ctrl+l".to_string()
}

// Readline cursor movement defaults
fn default_line_start() -> String {
    "Ctrl+a".to_string()
}

fn default_line_end() -> String {
    "Ctrl+e".to_string()
}

fn default_char_back() -> String {
    "Ctrl+b".to_string()
}

fn default_char_forward() -> String {
    "Ctrl+f".to_string()
}

fn default_word_back() -> String {
    "Alt+b".to_string()
}

fn default_word_forward() -> String {
    "Alt+f".to_string()
}

// Readline kill/delete defaults
fn default_kill_to_end() -> String {
    "Ctrl+k".to_string()
}

fn default_kill_to_start() -> String {
    "Ctrl+u".to_string()
}

fn default_kill_word_back() -> String {
    "Ctrl+w".to_string()
}

fn default_delete_char() -> String {
    "Ctrl+d".to_string()
}

fn default_yank() -> String {
    "Ctrl+y".to_string()
}

fn default_max_entries() -> usize {
    1000
}

fn default_font_size() -> f32 {
    14.0
}

fn default_history_font_size() -> f32 {
    12.0
}

fn default_history_lines() -> u32 {
    3
}

fn default_textarea_rows() -> u32 {
    3
}

fn default_textarea_cols() -> u32 {
    60
}

impl WindowConfig {
    /// Calculate window width in pixels
    pub fn width_pixels(&self) -> f64 {
        // Monospace char width ≈ font_size * 0.6
        let char_width = self.font_size as f64 * 0.6;
        let padding = 24.0; // Left + right padding
        (self.textarea_cols as f64 * char_width) + padding
    }

    /// Calculate window height in pixels
    pub fn height_pixels(&self) -> f64 {
        let font_size = self.font_size as f64;
        let history_font_size = self.history_font_size as f64;

        // Header: title + padding + border
        let header = 35.0;

        // History items: timestamp + preview + padding + gap + border
        // Both timestamp and preview use history_font_size
        let timestamp_height = history_font_size * 1.4;
        let preview_height = history_font_size * 1.3;
        let history_padding = 16.0; // 8px top + 8px bottom
        let history_gap = 2.0;
        let history_border = 1.0;
        let history_item_height =
            timestamp_height + preview_height + history_padding + history_gap + history_border;
        let history_area = self.history_lines as f64 * history_item_height;

        // Textarea: rows * line_height + padding
        let textarea_line_height = font_size * 1.4;
        let textarea_padding = 20.0;
        let textarea_area = (self.textarea_rows as f64 * textarea_line_height) + textarea_padding;

        // Button bar + main padding
        let button_area = 28.0;
        let main_padding = 24.0; // Top + bottom padding in main

        header + history_area + textarea_area + button_area + main_padding
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            shortcuts: default_shortcuts(),
            history: default_history(),
            window: default_window(),
            behavior: default_behavior(),
        }
    }
}

impl Config {
    /// Load config from file, or create default if not exists
    pub fn load() -> Result<Self, String> {
        let path = Self::default_path()?;

        if !path.exists() {
            // Create default config
            let config = Config::default();
            config.save()?;
            return Ok(config);
        }

        let contents =
            fs::read_to_string(&path).map_err(|e| format!("Failed to read config file: {}", e))?;

        toml::from_str(&contents).map_err(|e| format!("Failed to parse config file: {}", e))
    }

    /// Save config to file
    pub fn save(&self) -> Result<(), String> {
        let path = Self::default_path()?;

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create config directory: {}", e))?;
        }

        let toml = toml::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;

        fs::write(&path, toml).map_err(|e| format!("Failed to write config file: {}", e))?;

        Ok(())
    }

    /// Get default config file path
    pub fn default_path() -> Result<PathBuf, String> {
        let config_dir = directories::ProjectDirs::from("com", "prompt-line", "prompt-line-rs")
            .ok_or_else(|| "Failed to get config directory".to_string())?;

        Ok(config_dir.config_dir().join("config.toml"))
    }
}
