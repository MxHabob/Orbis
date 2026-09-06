// ── Auth Store ───────────────────────────────────────────────────────
// Svelte 5 Runes-based authentication state.

import { api } from "$lib/api/client";
import type { UserDto } from "$lib/api/types";
import { extractErrorMessage } from "$lib/core/ipc";

class AuthStore {
  user = $state<UserDto | null>(null);
  token = $state<string | null>(null);
  isLoading = $state(false);
  error = $state<string | null>(null);
  isSetupComplete = $state<boolean | null>(null);

  isAuthenticated = $derived(!!this.token && !!this.user);
  orgId = $derived(this.user?.org_id ?? "");
  userId = $derived(this.user?.id ?? "");
  needsOnboarding = $derived(
    this.isAuthenticated && this.isSetupComplete === false,
  );

  async login(email: string, password: string) {
    this.isLoading = true;
    this.error = null;
    try {
      const res = await api.login({ email, password });
      this.user = res.user;
      this.token = res.token;
      this.restorePromise = null;
      localStorage.setItem("orbis_token", res.token);
      localStorage.setItem("orbis_user", JSON.stringify(res.user));
      await this.checkSetupStatus();
    } catch (e) {
      this.error = extractErrorMessage(e);
      throw e;
    } finally {
      this.isLoading = false;
    }
  }

  async logout() {
    try {
      await api.logout();
    } catch {
      // Ignore network errors on logout
    }
    this.user = null;
    this.token = null;
    this.isSetupComplete = null;
    this.restorePromise = null;
    localStorage.removeItem("orbis_token");
    localStorage.removeItem("orbis_user");
    localStorage.removeItem("orbis_is_setup_complete");
  }

  private restorePromise: Promise<void> | null = null;

  async restore() {
    if (this.restorePromise) {
      return this.restorePromise;
    }
    this.restorePromise = this._restore();
    return this.restorePromise;
  }

  private async _restore() {
    try {
      const token = localStorage.getItem("orbis_token");
      const userStr = localStorage.getItem("orbis_user");
      const isSetupCompleteStr = localStorage.getItem(
        "orbis_is_setup_complete",
      );
      if (token && userStr) {
        try {
          const res = await api.getSession();
          this.token = token;
          this.user = res.user;
          if (isSetupCompleteStr !== null) {
            this.isSetupComplete = isSetupCompleteStr === "true";
          }
        } catch (e) {
          console.error("Failed to restore session on backend:", e);
          this.token = null;
          this.user = null;
        }
      }
    } catch {
      // Silently fail if localStorage is unavailable
    }
  }

  async checkSetupStatus() {
    if (!this.isAuthenticated || !this.orgId) {
      this.isSetupComplete = null;
      return;
    }
    try {
      const org = await api.getCurrentOrg();
      const isComplete = Boolean(org?.id);
      this.isSetupComplete = isComplete;
      localStorage.setItem("orbis_is_setup_complete", String(isComplete));
    } catch (e) {
      console.error("Error checking setup status:", e);
      const cached = localStorage.getItem("orbis_is_setup_complete");
      if (cached !== null) {
        this.isSetupComplete = cached === "true";
      } else {
        this.isSetupComplete = true;
      }
    }
  }

  markSetupComplete() {
    this.isSetupComplete = true;
    localStorage.setItem("orbis_is_setup_complete", "true");
  }
}

export const authStore = new AuthStore();
