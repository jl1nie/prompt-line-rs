import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";

interface HistoryEntry {
  text: string;
  timestamp: string;
}

interface Shortcuts {
  launch: string;
  paste: string;
  close: string;
  history_next: string;
  history_prev: string;
  search: string;
  clear: string;
  line_start: string;
  line_end: string;
  char_back: string;
  char_forward: string;
  word_back: string;
  word_forward: string;
  kill_to_end: string;
  kill_to_start: string;
  kill_word_back: string;
  delete_char: string;
  yank: string;
}

interface WindowConfig {
  font_size: number;
  history_font_size: number;
  history_lines: number;
  textarea_rows: number;
  textarea_cols: number;
}

interface Config {
  shortcuts: Shortcuts;
  window: WindowConfig;
}

// Parse shortcut string like "Ctrl+A" into { ctrl, alt, shift, key }
function parseShortcut(shortcut: string): { ctrl: boolean; alt: boolean; shift: boolean; key: string } {
  const parts = shortcut.toLowerCase().split("+");
  const key = parts[parts.length - 1];
  return {
    ctrl: parts.includes("ctrl"),
    alt: parts.includes("alt"),
    shift: parts.includes("shift"),
    key: key,
  };
}

// Check if keyboard event matches shortcut
function matchShortcut(e: KeyboardEvent, shortcut: string): boolean {
  const parsed = parseShortcut(shortcut);
  return (
    e.key.toLowerCase() === parsed.key &&
    e.ctrlKey === parsed.ctrl &&
    e.altKey === parsed.alt &&
    e.shiftKey === parsed.shift
  );
}

class PromptLineApp {
  private textarea: HTMLTextAreaElement;
  private historyList: HTMLUListElement;
  private historySearch: HTMLInputElement;
  private searchBtn: HTMLButtonElement;
  private historyEntries: HistoryEntry[] = [];
  private historyIndex = -1;
  private searchMode = false;
  private searchQuery = "";
  private draftSaveTimeout: number | null = null;
  private killRing: string = ""; // For Ctrl+Y (yank)
  private savedInput: string = ""; // For history navigation (readline behavior)
  private shortcuts!: Shortcuts;

  constructor() {
    this.textarea = document.getElementById("input-text") as HTMLTextAreaElement;
    this.historyList = document.getElementById("history-list") as HTMLUListElement;
    this.historySearch = document.getElementById("history-search") as HTMLInputElement;
    this.searchBtn = document.getElementById("btn-search") as HTMLButtonElement;

    this.init();
  }

  private async init(): Promise<void> {
    await this.loadConfig();
    this.setupEventListeners();
    this.loadHistory();
    this.loadDraft();
    this.focusTextarea();
  }

  private async loadConfig(): Promise<void> {
    try {
      const config = await invoke<Config>("get_config");
      this.shortcuts = config.shortcuts;
      this.applyWindowConfig(config.window);
    } catch (error) {
      console.error("Failed to load config:", error);
      // Use defaults if config fails to load
      this.shortcuts = {
        launch: "Ctrl+Shift+Space",
        paste: "Ctrl+Enter",
        close: "Escape",
        history_next: "Ctrl+n",
        history_prev: "Ctrl+p",
        search: "Ctrl+r",
        clear: "Ctrl+l",
        line_start: "Ctrl+a",
        line_end: "Ctrl+e",
        char_back: "Ctrl+b",
        char_forward: "Ctrl+f",
        word_back: "Alt+b",
        word_forward: "Alt+f",
        kill_to_end: "Ctrl+k",
        kill_to_start: "Ctrl+u",
        kill_word_back: "Ctrl+w",
        delete_char: "Ctrl+d",
        yank: "Ctrl+y",
      };
      // Apply default window config
      this.applyWindowConfig({ font_size: 14, history_font_size: 12, history_lines: 3, textarea_rows: 3, textarea_cols: 60 });
    }
  }

  private applyWindowConfig(window: WindowConfig): void {
    const root = document.documentElement;
    root.style.setProperty("--font-size", `${window.font_size}px`);
    root.style.setProperty("--history-font-size", `${window.history_font_size}px`);

    // Set textarea rows
    const lineHeight = window.font_size * 1.4;
    const textareaHeight = window.textarea_rows * lineHeight + 20; // 20px padding
    this.textarea.style.height = `${textareaHeight}px`;
  }

  private focusTextarea(): void {
    this.textarea.focus();
  }

  private setupEventListeners(): void {
    // Buttons
    document.getElementById("btn-paste")?.addEventListener("click", () => this.handlePaste());
    document.getElementById("btn-clear")?.addEventListener("click", () => this.handleClear());
    this.searchBtn.addEventListener("click", () => this.toggleSearchMode());

    // Search input
    this.historySearch.addEventListener("input", () => {
      this.searchQuery = this.historySearch.value;
      this.loadHistory();
    });

    this.historySearch.addEventListener("keydown", (e) => {
      // Escape: Close search mode
      if (matchShortcut(e, this.shortcuts.close)) {
        e.preventDefault();
        this.closeSearchMode();
        return;
      }
      // Navigate history while searching (readline: prev=older, next=newer)
      if (matchShortcut(e, this.shortcuts.history_prev)) {
        e.preventDefault();
        this.navigateHistory(1); // +1 = older entries
        return;
      }
      if (matchShortcut(e, this.shortcuts.history_next)) {
        e.preventDefault();
        this.navigateHistory(-1); // -1 = newer entries
        return;
      }
      // Enter: Select current history item and close search
      if (e.key === "Enter") {
        e.preventDefault();
        if (this.historyIndex >= 0) {
          this.selectHistoryItem(this.historyIndex);
        }
        this.closeSearchMode();
        return;
      }
    });

    // Keyboard shortcuts (readline bindings + app shortcuts)
    this.textarea.addEventListener("keydown", (e) => {
      // === App shortcuts ===
      // Paste and close
      if (matchShortcut(e, this.shortcuts.paste)) {
        e.preventDefault();
        this.handlePaste();
        return;
      }

      // Search history
      if (matchShortcut(e, this.shortcuts.search)) {
        e.preventDefault();
        this.toggleSearchMode();
        return;
      }

      // === Readline: History ===
      // Previous history (go back to older entries)
      if (matchShortcut(e, this.shortcuts.history_prev)) {
        e.preventDefault();
        this.navigateHistory(1); // +1 = older entries (higher index in newest-first array)
        return;
      }
      // Next history (go forward to newer entries)
      if (matchShortcut(e, this.shortcuts.history_next)) {
        e.preventDefault();
        this.navigateHistory(-1); // -1 = newer entries (lower index in newest-first array)
        return;
      }

      // === Readline: Cursor Movement ===
      // Beginning of line
      if (matchShortcut(e, this.shortcuts.line_start)) {
        e.preventDefault();
        this.moveCursorToLineStart();
        return;
      }
      // End of line
      if (matchShortcut(e, this.shortcuts.line_end)) {
        e.preventDefault();
        this.moveCursorToLineEnd();
        return;
      }
      // Back one character
      if (matchShortcut(e, this.shortcuts.char_back)) {
        e.preventDefault();
        this.moveCursor(-1);
        return;
      }
      // Forward one character
      if (matchShortcut(e, this.shortcuts.char_forward)) {
        e.preventDefault();
        this.moveCursor(1);
        return;
      }
      // Back one word
      if (matchShortcut(e, this.shortcuts.word_back)) {
        e.preventDefault();
        this.moveCursorByWord(-1);
        return;
      }
      // Forward one word
      if (matchShortcut(e, this.shortcuts.word_forward)) {
        e.preventDefault();
        this.moveCursorByWord(1);
        return;
      }

      // === Readline: Deletion ===
      // Kill to end of line
      if (matchShortcut(e, this.shortcuts.kill_to_end)) {
        e.preventDefault();
        this.killToEnd();
        return;
      }
      // Kill to beginning of line
      if (matchShortcut(e, this.shortcuts.kill_to_start)) {
        e.preventDefault();
        this.killToStart();
        return;
      }
      // Kill word backward
      if (matchShortcut(e, this.shortcuts.kill_word_back)) {
        e.preventDefault();
        this.killWordBackward();
        return;
      }
      // Delete character or close if empty
      if (matchShortcut(e, this.shortcuts.delete_char)) {
        e.preventDefault();
        if (this.textarea.value === "") {
          this.hideWindow();
        } else {
          this.deleteCharacter();
        }
        return;
      }

      // === Readline: Other ===
      // Yank (paste from kill ring)
      if (matchShortcut(e, this.shortcuts.yank)) {
        e.preventDefault();
        this.yank();
        return;
      }
      // Clear textarea
      if (matchShortcut(e, this.shortcuts.clear)) {
        e.preventDefault();
        this.handleClear();
        return;
      }
    });

    // Global shortcuts (work even when textarea not focused)
    document.addEventListener("keydown", (e) => {
      // Close: Close search or window
      if (matchShortcut(e, this.shortcuts.close)) {
        e.preventDefault();
        if (this.searchMode) {
          this.closeSearchMode();
        } else {
          this.hideWindow();
        }
      }
    });

    // Window focus listener
    window.addEventListener("focus", () => {
      this.loadConfig(); // Reload config (may have changed in settings)
      this.loadHistory();
      this.focusTextarea();
    });

    // Draft autosave on text change
    this.textarea.addEventListener("input", () => {
      this.scheduleDraftSave();
    });
  }

  private async handlePaste(): Promise<void> {
    const text = this.textarea.value;
    if (!text.trim()) return;

    try {
      await invoke("paste_and_save", { text });
      await this.clearDraft();
      await this.hideWindow();
      await invoke("simulate_paste");
      this.textarea.value = "";
      this.historyIndex = -1;
      this.savedInput = "";
    } catch (error) {
      console.error("Paste failed:", error);
    }
  }

  private handleClear(): void {
    this.textarea.value = "";
    this.historyIndex = -1;
    this.savedInput = "";
    this.clearDraft();
    this.focusTextarea();
  }

  private async hideWindow(): Promise<void> {
    try {
      await getCurrentWindow().hide();
    } catch (error) {
      console.error("Failed to hide window:", error);
    }
  }

  private toggleSearchMode(): void {
    if (this.searchMode) {
      this.closeSearchMode();
    } else {
      this.openSearchMode();
    }
  }

  private openSearchMode(): void {
    this.searchMode = true;
    this.historySearch.classList.remove("hidden");
    this.historySearch.focus();
  }

  private closeSearchMode(): void {
    this.searchMode = false;
    this.searchQuery = "";
    this.historySearch.value = "";
    this.historySearch.classList.add("hidden");
    this.loadHistory();
    this.focusTextarea();
  }

  private async loadHistory(): Promise<void> {
    try {
      this.historyEntries = await invoke<HistoryEntry[]>("get_history", { query: this.searchQuery });
      this.renderHistory();
      // Scroll to bottom (newest entry) on load
      this.scrollHistoryToBottom();
    } catch (error) {
      console.error("Failed to load history:", error);
    }
  }

  private scrollHistoryToBottom(): void {
    const historySection = document.getElementById("history-section");
    if (historySection) {
      historySection.scrollTop = historySection.scrollHeight;
    }
  }

  private navigateHistory(direction: number): void {
    if (this.historyEntries.length === 0) return;

    const newIndex = this.historyIndex + direction;
    if (newIndex < -1 || newIndex >= this.historyEntries.length) return;

    // Save current input when first entering history (readline behavior)
    if (this.historyIndex === -1 && newIndex >= 0) {
      this.savedInput = this.textarea.value;
    }

    this.historyIndex = newIndex;

    if (this.historyIndex === -1) {
      // Restore saved input when returning from history
      this.textarea.value = this.savedInput;
    } else {
      this.textarea.value = this.historyEntries[this.historyIndex].text;
    }

    // Move cursor to end
    this.textarea.setSelectionRange(this.textarea.value.length, this.textarea.value.length);
    this.updateHistorySelection();
  }

  private selectHistoryItem(index: number): void {
    if (index < 0 || index >= this.historyEntries.length) return;

    this.historyIndex = index;
    this.textarea.value = this.historyEntries[index].text;
    this.textarea.setSelectionRange(this.textarea.value.length, this.textarea.value.length);
    this.updateHistorySelection();
    this.focusTextarea();
  }

  private updateHistorySelection(): void {
    const items = this.historyList.querySelectorAll("li");
    items.forEach((item, i) => {
      if (i === this.historyIndex) {
        item.classList.add("selected");
        item.scrollIntoView({ block: "nearest", behavior: "smooth" });
      } else {
        item.classList.remove("selected");
      }
    });
  }

  private renderHistory(): void {
    const maxEntries = 50;
    const displayEntries = this.historyEntries.slice(0, maxEntries);

    if (displayEntries.length === 0) {
      this.historyList.innerHTML = `<li class="empty-message">${
        this.searchQuery ? "No matching history" : "No history yet"
      }</li>`;
      return;
    }

    this.historyList.innerHTML = displayEntries
      .map((entry, index) => {
        const preview = entry.text.length > 80
          ? entry.text.substring(0, 80) + "..."
          : entry.text;
        const timestamp = new Date(entry.timestamp).toLocaleString("ja-JP", {
          month: "short",
          day: "numeric",
          hour: "2-digit",
          minute: "2-digit",
        });
        const escapedPreview = this.escapeHtml(preview).replace(/\n/g, " ");
        const selectedClass = index === this.historyIndex ? "selected" : "";
        const highlightedPreview = this.searchQuery
          ? this.highlightSearch(escapedPreview, this.searchQuery)
          : escapedPreview;

        return `<li data-index="${index}" class="${selectedClass}">
          <span class="timestamp">${timestamp}</span>
          <span class="preview">${highlightedPreview}</span>
        </li>`;
      })
      .join("");

    // Add click handlers
    this.historyList.querySelectorAll("li[data-index]").forEach((li) => {
      li.addEventListener("click", () => {
        const index = parseInt((li as HTMLElement).dataset.index || "0", 10);
        this.selectHistoryItem(index);
      });
    });
  }

  private highlightSearch(text: string, query: string): string {
    if (!query) return text;
    const regex = new RegExp(`(${this.escapeRegex(query)})`, "gi");
    return text.replace(regex, '<span class="search-highlight">$1</span>');
  }

  private escapeRegex(str: string): string {
    return str.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  }

  private escapeHtml(text: string): string {
    const div = document.createElement("div");
    div.textContent = text;
    return div.innerHTML;
  }

  // Draft autosave methods
  private async loadDraft(): Promise<void> {
    try {
      const draft = await invoke<string>("load_draft");
      if (draft && !this.textarea.value) {
        this.textarea.value = draft;
        // Move cursor to end
        this.textarea.setSelectionRange(draft.length, draft.length);
      }
    } catch (error) {
      console.error("Failed to load draft:", error);
    }
  }

  private scheduleDraftSave(): void {
    // Debounce: save after 500ms of no typing
    if (this.draftSaveTimeout !== null) {
      clearTimeout(this.draftSaveTimeout);
    }
    this.draftSaveTimeout = window.setTimeout(() => {
      this.saveDraft();
    }, 500);
  }

  private async saveDraft(): Promise<void> {
    try {
      const text = this.textarea.value;
      if (text) {
        await invoke("save_draft", { text });
      } else {
        await this.clearDraft();
      }
    } catch (error) {
      console.error("Failed to save draft:", error);
    }
  }

  private async clearDraft(): Promise<void> {
    try {
      await invoke("clear_draft");
    } catch (error) {
      console.error("Failed to clear draft:", error);
    }
  }

  // === Readline: Cursor Movement ===
  private moveCursorToLineStart(): void {
    const pos = this.textarea.selectionStart;
    const text = this.textarea.value;
    // Find the start of the current line
    let lineStart = pos;
    while (lineStart > 0 && text[lineStart - 1] !== "\n") {
      lineStart--;
    }
    this.textarea.setSelectionRange(lineStart, lineStart);
  }

  private moveCursorToLineEnd(): void {
    const pos = this.textarea.selectionStart;
    const text = this.textarea.value;
    // Find the end of the current line
    let lineEnd = pos;
    while (lineEnd < text.length && text[lineEnd] !== "\n") {
      lineEnd++;
    }
    this.textarea.setSelectionRange(lineEnd, lineEnd);
  }

  private moveCursor(delta: number): void {
    const pos = this.textarea.selectionStart;
    const newPos = Math.max(0, Math.min(this.textarea.value.length, pos + delta));
    this.textarea.setSelectionRange(newPos, newPos);
  }

  private moveCursorByWord(direction: number): void {
    const pos = this.textarea.selectionStart;
    const text = this.textarea.value;
    let newPos = pos;

    if (direction < 0) {
      // Move backward: skip spaces, then skip word characters
      while (newPos > 0 && /\s/.test(text[newPos - 1])) {
        newPos--;
      }
      while (newPos > 0 && !/\s/.test(text[newPos - 1])) {
        newPos--;
      }
    } else {
      // Move forward: skip word characters, then skip spaces
      while (newPos < text.length && !/\s/.test(text[newPos])) {
        newPos++;
      }
      while (newPos < text.length && /\s/.test(text[newPos])) {
        newPos++;
      }
    }

    this.textarea.setSelectionRange(newPos, newPos);
  }

  // === Readline: Kill (Delete) ===
  private killToEnd(): void {
    const pos = this.textarea.selectionStart;
    const text = this.textarea.value;
    // Find the end of the current line
    let lineEnd = pos;
    while (lineEnd < text.length && text[lineEnd] !== "\n") {
      lineEnd++;
    }
    // Save to kill ring
    this.killRing = text.substring(pos, lineEnd);
    // Delete
    this.textarea.value = text.substring(0, pos) + text.substring(lineEnd);
    this.textarea.setSelectionRange(pos, pos);
    this.scheduleDraftSave();
  }

  private killToStart(): void {
    const pos = this.textarea.selectionStart;
    const text = this.textarea.value;
    // Find the start of the current line
    let lineStart = pos;
    while (lineStart > 0 && text[lineStart - 1] !== "\n") {
      lineStart--;
    }
    // Save to kill ring
    this.killRing = text.substring(lineStart, pos);
    // Delete
    this.textarea.value = text.substring(0, lineStart) + text.substring(pos);
    this.textarea.setSelectionRange(lineStart, lineStart);
    this.scheduleDraftSave();
  }

  private killWordBackward(): void {
    const pos = this.textarea.selectionStart;
    const text = this.textarea.value;
    let newPos = pos;

    // Skip trailing spaces
    while (newPos > 0 && /\s/.test(text[newPos - 1])) {
      newPos--;
    }
    // Skip word characters
    while (newPos > 0 && !/\s/.test(text[newPos - 1])) {
      newPos--;
    }

    // Save to kill ring
    this.killRing = text.substring(newPos, pos);
    // Delete
    this.textarea.value = text.substring(0, newPos) + text.substring(pos);
    this.textarea.setSelectionRange(newPos, newPos);
    this.scheduleDraftSave();
  }

  private deleteCharacter(): void {
    const pos = this.textarea.selectionStart;
    const text = this.textarea.value;
    if (pos < text.length) {
      this.textarea.value = text.substring(0, pos) + text.substring(pos + 1);
      this.textarea.setSelectionRange(pos, pos);
      this.scheduleDraftSave();
    }
  }

  // === Readline: Yank ===
  private yank(): void {
    if (!this.killRing) return;
    const pos = this.textarea.selectionStart;
    const text = this.textarea.value;
    this.textarea.value = text.substring(0, pos) + this.killRing + text.substring(pos);
    const newPos = pos + this.killRing.length;
    this.textarea.setSelectionRange(newPos, newPos);
    this.scheduleDraftSave();
  }
}

// Initialize app when DOM is ready
document.addEventListener("DOMContentLoaded", () => {
  new PromptLineApp();
});
