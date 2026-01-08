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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shortcuts {
    #[serde(default = "default_launch")]
    pub launch: String,

    #[serde(default = "default_paste")]
    pub paste: String,

    #[serde(default = "default_close")]
    pub close: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryConfig {
    #[serde(default = "default_max_entries")]
    pub max_entries: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    #[serde(default = "default_width")]
    pub width: f32,

    #[serde(default = "default_height")]
    pub height: f32,

    #[serde(default = "default_font_size")]
    pub font_size: f32,
}

// Default values
fn default_shortcuts() -> Shortcuts {
    Shortcuts {
        launch: "Alt+Space".to_string(),
        paste: "Ctrl+Enter".to_string(),
        close: "Escape".to_string(),
    }
}

fn default_history() -> HistoryConfig {
    HistoryConfig { max_entries: 1000 }
}

fn default_window() -> WindowConfig {
    WindowConfig {
        width: 600.0,
        height: 400.0,
        font_size: 16.0,
    }
}

fn default_launch() -> String {
    "Alt+Space".to_string()
}

fn default_paste() -> String {
    "Ctrl+Enter".to_string()
}

fn default_close() -> String {
    "Escape".to_string()
}

fn default_max_entries() -> usize {
    1000
}

fn default_width() -> f32 {
    600.0
}

fn default_height() -> f32 {
    400.0
}

fn default_font_size() -> f32 {
    16.0
}

impl Default for Config {
    fn default() -> Self {
        Self {
            shortcuts: default_shortcuts(),
            history: default_history(),
            window: default_window(),
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

        let contents = fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read config file: {}", e))?;

        toml::from_str(&contents)
            .map_err(|e| format!("Failed to parse config file: {}", e))
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

        fs::write(&path, toml)
            .map_err(|e| format!("Failed to write config file: {}", e))?;

        Ok(())
    }

    /// Get default config file path
    pub fn default_path() -> Result<PathBuf, String> {
        let config_dir = directories::ProjectDirs::from("com", "prompt-line", "prompt-line-rs")
            .ok_or_else(|| "Failed to get config directory".to_string())?;

        Ok(config_dir.config_dir().join("config.toml"))
    }
}
