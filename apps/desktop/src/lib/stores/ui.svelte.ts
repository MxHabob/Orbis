// ── UI Store ─────────────────────────────────────────────────────────

import { page } from "$app/state";

class UIStore {
  private aiProviderModels = $state<
    Record<string, { provider: string; model: string }>
  >({});

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

  getAiProviderModel(orgId: string): { provider: string; model: string } {
    const key = encodeURIComponent(orgId);
    const preferences = this.aiProviderModels[key];
    if (preferences) return preferences;

    return {
      provider: localStorage.getItem(`orbis_ai_provider_${key}`) ?? "",
      model: localStorage.getItem(`orbis_ai_model_${key}`) ?? "",
    };
  }

  setAiProviderModel(orgId: string, provider: string, model: string) {
    if (!orgId) return;
    const key = encodeURIComponent(orgId);
    this.aiProviderModels[key] = { provider, model };
    localStorage.setItem(`orbis_ai_provider_${key}`, provider);
    localStorage.setItem(`orbis_ai_model_${key}`, model);
  }
}

export const uiStore = new UIStore();
