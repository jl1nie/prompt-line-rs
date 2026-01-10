import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";

interface Shortcuts {
  launch: string;
  paste: string;
  close: string;
  history_next: string;
  history_prev: string;
  search: string;
  clear: string;
  // Readline cursor movement
  line_start: string;
  line_end: string;
  char_back: string;
  char_forward: string;
  word_back: string;
  word_forward: string;
  // Readline kill/delete
  kill_to_end: string;
  kill_to_start: string;
  kill_word_back: string;
  delete_char: string;
  yank: string;
}

interface HistoryConfig {
  max_entries: number;
}

interface WindowConfig {
  width: number;
  height: number;
  font_size: number;
}

interface Config {
  shortcuts: Shortcuts;
  history: HistoryConfig;
  window: WindowConfig;
}

class SettingsApp {
  private config: Config | null = null;

  // Form elements
  private windowWidth: HTMLInputElement;
  private windowHeight: HTMLInputElement;
  private fontSize: HTMLInputElement;
  private maxEntries: HTMLInputElement;
  private statusMessage: HTMLElement;

  // Shortcut elements
  private shortcutLaunch: HTMLInputElement;
  private shortcutPaste: HTMLInputElement;
  private shortcutClose: HTMLInputElement;
  private shortcutHistoryNext: HTMLInputElement;
  private shortcutHistoryPrev: HTMLInputElement;
  private shortcutSearch: HTMLInputElement;
  private shortcutClear: HTMLInputElement;
  // Readline cursor movement
  private shortcutLineStart: HTMLInputElement;
  private shortcutLineEnd: HTMLInputElement;
  private shortcutCharBack: HTMLInputElement;
  private shortcutCharForward: HTMLInputElement;
  private shortcutWordBack: HTMLInputElement;
  private shortcutWordForward: HTMLInputElement;
  // Readline kill/delete
  private shortcutKillToEnd: HTMLInputElement;
  private shortcutKillToStart: HTMLInputElement;
  private shortcutKillWordBack: HTMLInputElement;
  private shortcutDeleteChar: HTMLInputElement;
  private shortcutYank: HTMLInputElement;

  constructor() {
    this.windowWidth = document.getElementById("window-width") as HTMLInputElement;
    this.windowHeight = document.getElementById("window-height") as HTMLInputElement;
    this.fontSize = document.getElementById("font-size") as HTMLInputElement;
    this.maxEntries = document.getElementById("max-entries") as HTMLInputElement;
    this.statusMessage = document.getElementById("status-message") as HTMLElement;

    // Shortcut inputs
    this.shortcutLaunch = document.getElementById("shortcut-launch") as HTMLInputElement;
    this.shortcutPaste = document.getElementById("shortcut-paste") as HTMLInputElement;
    this.shortcutClose = document.getElementById("shortcut-close") as HTMLInputElement;
    this.shortcutHistoryNext = document.getElementById("shortcut-history-next") as HTMLInputElement;
    this.shortcutHistoryPrev = document.getElementById("shortcut-history-prev") as HTMLInputElement;
    this.shortcutSearch = document.getElementById("shortcut-search") as HTMLInputElement;
    this.shortcutClear = document.getElementById("shortcut-clear") as HTMLInputElement;
    // Readline cursor movement
    this.shortcutLineStart = document.getElementById("shortcut-line-start") as HTMLInputElement;
    this.shortcutLineEnd = document.getElementById("shortcut-line-end") as HTMLInputElement;
    this.shortcutCharBack = document.getElementById("shortcut-char-back") as HTMLInputElement;
    this.shortcutCharForward = document.getElementById("shortcut-char-forward") as HTMLInputElement;
    this.shortcutWordBack = document.getElementById("shortcut-word-back") as HTMLInputElement;
    this.shortcutWordForward = document.getElementById("shortcut-word-forward") as HTMLInputElement;
    // Readline kill/delete
    this.shortcutKillToEnd = document.getElementById("shortcut-kill-to-end") as HTMLInputElement;
    this.shortcutKillToStart = document.getElementById("shortcut-kill-to-start") as HTMLInputElement;
    this.shortcutKillWordBack = document.getElementById("shortcut-kill-word-back") as HTMLInputElement;
    this.shortcutDeleteChar = document.getElementById("shortcut-delete-char") as HTMLInputElement;
    this.shortcutYank = document.getElementById("shortcut-yank") as HTMLInputElement;

    this.setupEventListeners();
    this.loadConfig();
  }

  private setupEventListeners(): void {
    document.getElementById("btn-save")?.addEventListener("click", () => this.handleSave());
    document.getElementById("btn-cancel")?.addEventListener("click", () => this.handleCancel());

    // Escape to close
    document.addEventListener("keydown", (e) => {
      if (e.key === "Escape") {
        e.preventDefault();
        this.handleCancel();
      }
    });
  }

  private async loadConfig(): Promise<void> {
    try {
      this.config = await invoke<Config>("get_config");
      this.populateForm();
    } catch (error) {
      console.error("Failed to load config:", error);
      this.showStatus("Failed to load configuration", "error");
    }
  }

  private populateForm(): void {
    if (!this.config) return;

    // Window settings
    this.windowWidth.value = String(this.config.window.width);
    this.windowHeight.value = String(this.config.window.height);
    this.fontSize.value = String(this.config.window.font_size);

    // History settings
    this.maxEntries.value = String(this.config.history.max_entries);

    // Shortcuts
    this.shortcutLaunch.value = this.config.shortcuts.launch;
    this.shortcutPaste.value = this.config.shortcuts.paste;
    this.shortcutClose.value = this.config.shortcuts.close;
    this.shortcutHistoryNext.value = this.config.shortcuts.history_next;
    this.shortcutHistoryPrev.value = this.config.shortcuts.history_prev;
    this.shortcutSearch.value = this.config.shortcuts.search;
    this.shortcutClear.value = this.config.shortcuts.clear;
    // Readline cursor movement
    this.shortcutLineStart.value = this.config.shortcuts.line_start;
    this.shortcutLineEnd.value = this.config.shortcuts.line_end;
    this.shortcutCharBack.value = this.config.shortcuts.char_back;
    this.shortcutCharForward.value = this.config.shortcuts.char_forward;
    this.shortcutWordBack.value = this.config.shortcuts.word_back;
    this.shortcutWordForward.value = this.config.shortcuts.word_forward;
    // Readline kill/delete
    this.shortcutKillToEnd.value = this.config.shortcuts.kill_to_end;
    this.shortcutKillToStart.value = this.config.shortcuts.kill_to_start;
    this.shortcutKillWordBack.value = this.config.shortcuts.kill_word_back;
    this.shortcutDeleteChar.value = this.config.shortcuts.delete_char;
    this.shortcutYank.value = this.config.shortcuts.yank;
  }

  private async handleSave(): Promise<void> {
    if (!this.config) return;

    // Update config from form
    const newConfig: Config = {
      shortcuts: {
        launch: this.shortcutLaunch.value || "Ctrl+Shift+Space",
        paste: this.shortcutPaste.value || "Ctrl+Enter",
        close: this.shortcutClose.value || "Escape",
        history_next: this.shortcutHistoryNext.value || "Ctrl+n",
        history_prev: this.shortcutHistoryPrev.value || "Ctrl+p",
        search: this.shortcutSearch.value || "Ctrl+r",
        clear: this.shortcutClear.value || "Ctrl+l",
        // Readline cursor movement
        line_start: this.shortcutLineStart.value || "Ctrl+a",
        line_end: this.shortcutLineEnd.value || "Ctrl+e",
        char_back: this.shortcutCharBack.value || "Ctrl+b",
        char_forward: this.shortcutCharForward.value || "Ctrl+f",
        word_back: this.shortcutWordBack.value || "Alt+b",
        word_forward: this.shortcutWordForward.value || "Alt+f",
        // Readline kill/delete
        kill_to_end: this.shortcutKillToEnd.value || "Ctrl+k",
        kill_to_start: this.shortcutKillToStart.value || "Ctrl+u",
        kill_word_back: this.shortcutKillWordBack.value || "Ctrl+w",
        delete_char: this.shortcutDeleteChar.value || "Ctrl+d",
        yank: this.shortcutYank.value || "Ctrl+y",
      },
      history: {
        max_entries: parseInt(this.maxEntries.value, 10) || 1000,
      },
      window: {
        width: parseFloat(this.windowWidth.value) || 600,
        height: parseFloat(this.windowHeight.value) || 300,
        font_size: parseFloat(this.fontSize.value) || 16,
      },
    };

    try {
      await invoke("save_config", { newConfig });
      this.config = newConfig;
      this.showStatus("Settings saved successfully! Some changes require restart.", "success");
    } catch (error) {
      console.error("Failed to save config:", error);
      this.showStatus(`Failed to save: ${error}`, "error");
    }
  }

  private async handleCancel(): Promise<void> {
    try {
      await getCurrentWindow().close();
    } catch (error) {
      console.error("Failed to close window:", error);
    }
  }

  private showStatus(message: string, type: "success" | "error"): void {
    this.statusMessage.textContent = message;
    this.statusMessage.className = type;

    // Auto-hide after 3 seconds for success
    if (type === "success") {
      setTimeout(() => {
        this.statusMessage.className = "hidden";
      }, 3000);
    }
  }
}

// Initialize app when DOM is ready
document.addEventListener("DOMContentLoaded", () => {
  new SettingsApp();
});
