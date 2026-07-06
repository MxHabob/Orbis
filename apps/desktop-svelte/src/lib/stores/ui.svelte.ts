// ── UI Store ─────────────────────────────────────────────────────────
// Svelte 5 Runes-based UI state for layout management.
// Manages sidebar, panel widths, and global loading state.

import { page } from "$app/state";

class UIStore {
  globalLoading = $state(false);
  _showAskOrbis = $state(false);
  showUpgradeDialog = $state(false);
  focusMode = $state(localStorage.getItem("orbis_focus_mode") === "true");
  showSearch = $state(false);

  // Panel resize preferences are persisted via Resizer component
  // to localStorage with `resizer:{persistId}` keys.

  get showAskOrbis() {
    try {
      return this._showAskOrbis || !!page.url.searchParams.get("ask_orbis");
    } catch {
      return this._showAskOrbis;
    }
  }

  setGlobalLoading(loading: boolean) {
    this.globalLoading = loading;
  }

  toggleAskOrbis() {
    this._showAskOrbis = !this.showAskOrbis;
  }

  setAskOrbis(show: boolean) {
    this._showAskOrbis = show;
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
