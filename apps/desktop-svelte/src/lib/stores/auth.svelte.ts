import * as m from "$lib/paraglide/messages";
// ── Auth Store ───────────────────────────────────────────────────────
// Svelte 5 Runes-based authentication state.

import { commands } from "$lib/core/ipc";
import type { User } from "$lib/core/ipc";
import { extractErrorMessage } from "$lib/core/ipc";

class AuthStore {
  user = $state<User | null>(null);
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
      const res = await commands.platform.login(email, password);
      this.user = res.user;
      this.token = res.access_token;
      localStorage.setItem("orbis_token", res.access_token);
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
    if (this.token) {
      await commands.platform.logout(this.token).catch(() => {});
    }
    this.user = null;
    this.token = null;
    this.isSetupComplete = null;
    localStorage.removeItem("orbis_token");
    localStorage.removeItem("orbis_user");
    localStorage.removeItem("orbis_is_setup_complete");
  }

  restore() {
    try {
      const token = localStorage.getItem("orbis_token");
      const userStr = localStorage.getItem("orbis_user");
      const isSetupCompleteStr = localStorage.getItem(
        "orbis_is_setup_complete",
      );
      if (token && userStr) {
        this.token = token;
        this.user = JSON.parse(userStr);
        if (isSetupCompleteStr !== null) {
          this.isSetupComplete = isSetupCompleteStr === "true";
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
      const status = await commands.platform.getSetupStatus(this.orgId);
      this.isSetupComplete = status;
      localStorage.setItem("orbis_is_setup_complete", String(status));
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
