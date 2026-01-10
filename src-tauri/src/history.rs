//! History management module
//!
//! Stores input history in JSONL format (one JSON object per line)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub text: String,
    pub timestamp: DateTime<Utc>,
}

impl HistoryEntry {
    pub fn new(text: String) -> Self {
        Self {
            text,
            timestamp: Utc::now(),
        }
    }
}

pub struct History {
    file_path: PathBuf,
    entries: Vec<HistoryEntry>,
    max_entries: usize,
}

impl History {
    /// Create a new History instance with the given file path
    pub fn new(file_path: PathBuf, max_entries: usize) -> Result<Self, String> {
        // Ensure parent directory exists
        if let Some(parent) = file_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create history directory: {}", e))?;
        }

        let mut history = Self {
            file_path,
            entries: Vec::new(),
            max_entries,
        };

        history.load()?;
        Ok(history)
    }

    /// Load history from file
    fn load(&mut self) -> Result<(), String> {
        if !self.file_path.exists() {
            return Ok(());
        }

        let file = File::open(&self.file_path)
            .map_err(|e| format!("Failed to open history file: {}", e))?;

        let reader = BufReader::new(file);
        self.entries.clear();

        for line in reader.lines() {
            let line = line.map_err(|e| format!("Failed to read line: {}", e))?;
            if line.trim().is_empty() {
                continue;
            }

            match serde_json::from_str::<HistoryEntry>(&line) {
                Ok(entry) => self.entries.push(entry),
                Err(e) => eprintln!("Failed to parse history entry: {}", e),
            }
        }

        // Keep only the most recent entries
        if self.entries.len() > self.max_entries {
            self.entries.drain(0..self.entries.len() - self.max_entries);
        }

        Ok(())
    }

    /// Save history to file
    fn save(&self) -> Result<(), String> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.file_path)
            .map_err(|e| format!("Failed to open history file for writing: {}", e))?;

        for entry in &self.entries {
            let json = serde_json::to_string(entry)
                .map_err(|e| format!("Failed to serialize entry: {}", e))?;
            writeln!(file, "{}", json).map_err(|e| format!("Failed to write entry: {}", e))?;
        }

        Ok(())
    }

    /// Add a new entry to history
    pub fn add(&mut self, text: String) -> Result<(), String> {
        if text.trim().is_empty() {
            return Ok(());
        }

        let entry = HistoryEntry::new(text);
        self.entries.push(entry);

        // Trim old entries if exceeding max
        if self.entries.len() > self.max_entries {
            self.entries.drain(0..self.entries.len() - self.max_entries);
        }

        self.save()
    }

    /// Get all entries (most recent first)
    pub fn entries(&self) -> Vec<HistoryEntry> {
        let mut entries = self.entries.clone();
        entries.reverse();
        entries
    }

    /// Search history entries by text
    pub fn search(&self, query: &str) -> Vec<HistoryEntry> {
        if query.trim().is_empty() {
            return self.entries();
        }

        let query_lower = query.to_lowercase();
        let mut results: Vec<_> = self
            .entries
            .iter()
            .filter(|e| e.text.to_lowercase().contains(&query_lower))
            .cloned()
            .collect();

        results.reverse();
        results
    }

    /// Get the default history file path
    pub fn default_path() -> Result<PathBuf, String> {
        let config_dir = directories::ProjectDirs::from("com", "prompt-line", "prompt-line-rs")
            .ok_or_else(|| "Failed to get config directory".to_string())?;

        Ok(config_dir.data_dir().join("history.jsonl"))
    }
}
