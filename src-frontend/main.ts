import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";

interface HistoryEntry {
  text: string;
  timestamp: string;
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

  constructor() {
    this.textarea = document.getElementById("input-text") as HTMLTextAreaElement;
    this.historyList = document.getElementById("history-list") as HTMLUListElement;
    this.historySearch = document.getElementById("history-search") as HTMLInputElement;
    this.searchBtn = document.getElementById("btn-search") as HTMLButtonElement;

    this.setupEventListeners();
    this.loadHistory();
    this.focusTextarea();
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
      if (e.key === "Escape") {
        e.preventDefault();
        this.closeSearchMode();
      }
    });

    // Keyboard shortcuts (matching prompt-line)
    document.addEventListener("keydown", (e) => {
      // Ctrl+Enter: Paste and close
      if (e.key === "Enter" && e.ctrlKey) {
        e.preventDefault();
        this.handlePaste();
      }
      // Ctrl+j: Next history item
      else if (e.key === "j" && e.ctrlKey) {
        e.preventDefault();
        this.navigateHistory(1);
      }
      // Ctrl+k: Previous history item
      else if (e.key === "k" && e.ctrlKey) {
        e.preventDefault();
        this.navigateHistory(-1);
      }
      // Ctrl+f: Open search mode
      else if (e.key === "f" && e.ctrlKey) {
        e.preventDefault();
        this.openSearchMode();
      }
      // Escape: Close search or window
      else if (e.key === "Escape") {
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
      this.loadHistory();
      this.focusTextarea();
    });
  }

  private async handlePaste(): Promise<void> {
    const text = this.textarea.value;
    if (!text.trim()) return;

    try {
      await invoke("paste_and_save", { text });
      await this.hideWindow();
      await invoke("simulate_paste");
      this.textarea.value = "";
      this.historyIndex = -1;
    } catch (error) {
      console.error("Paste failed:", error);
    }
  }

  private handleClear(): void {
    this.textarea.value = "";
    this.historyIndex = -1;
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
    } catch (error) {
      console.error("Failed to load history:", error);
    }
  }

  private navigateHistory(direction: number): void {
    if (this.historyEntries.length === 0) return;

    const newIndex = this.historyIndex + direction;
    if (newIndex < -1 || newIndex >= this.historyEntries.length) return;

    this.historyIndex = newIndex;

    if (this.historyIndex === -1) {
      this.textarea.value = "";
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
}

// Initialize app when DOM is ready
document.addEventListener("DOMContentLoaded", () => {
  new PromptLineApp();
});
