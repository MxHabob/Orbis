// ── UI Store ─────────────────────────────────────────────────────────
// Svelte 5 Runes-based UI state for layout management.
// Manages sidebar, panel widths, and global loading state.

class UIStore {
  globalLoading = $state(false);
  showAskOrbis = $state(false);
  showUpgradeDialog = $state(false);
  focusMode = $state(localStorage.getItem("orbis_focus_mode") === "true");
  showSearch = $state(false);

  // Panel resize preferences are persisted via Resizer component
  // to localStorage with `resizer:{persistId}` keys.

  setGlobalLoading(loading: boolean) {
    this.globalLoading = loading;
  }

  toggleAskOrbis() {
    this.showAskOrbis = !this.showAskOrbis;
  }

  setAskOrbis(show: boolean) {
    this.showAskOrbis = show;
  }

  toggleFocusMode() {
    this.focusMode = !this.focusMode;
    localStorage.setItem("orbis_focus_mode", String(this.focusMode));
  }

  toggleUpgradeDialog() {
    this.showUpgradeDialog = !this.showUpgradeDialog;
  }

  setUpgradeDialog(show: boolean) {
    this.showUpgradeDialog = show;
  }

  toggleSearch() {
    this.showSearch = !this.showSearch;
  }

  setSearch(show: boolean) {
    this.showSearch = show;
  }
}

export const uiStore = new UIStore();
