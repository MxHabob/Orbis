// ── UI Store ─────────────────────────────────────────────────────────

import { page } from "$app/state";

class UIStore {
  globalLoading = $state(false);
  _showAskOrbis = $state(false);
  showUpgradeDialog = $state(false);
  focusMode = $state(localStorage.getItem("orbis_focus_mode") === "true");
  showSearch = $state(false);
  showSystemClock = $state(localStorage.getItem("orbis_clock_open") !== "false");

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

  showActivationDialog = $state(false);

  setActivationDialog(show: boolean) {
    this.showActivationDialog = show;
  }

  toggleSearch() {
    this.showSearch = !this.showSearch;
  }

  setSearch(show: boolean) {
    this.showSearch = show;
  }

  setSystemClock(show: boolean) {
    this.showSystemClock = show;
    localStorage.setItem("orbis_clock_open", String(show));
  }
}

export const uiStore = new UIStore();
