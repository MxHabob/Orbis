// ── Auth Store ───────────────────────────────────────────────────────
// Svelte 5 Runes-based authentication state (replaces Zustand).

import type { User, LoginResponse } from "$lib/services/ipc";

class AuthStore {
  user = $state<User | null>(null);
  token = $state<string | null>(null);
  isLoading = $state(false);
  error = $state<string | null>(null);

  isAuthenticated = $derived(!!this.token && !!this.user);
  orgId = $derived(this.user?.org_id ?? "");
  userId = $derived(this.user?.id ?? "");

  async login(email: string, password: string) {
    this.isLoading = true;
    this.error = null;
    try {
      const { commands } = await import("$lib/services/ipc");
      const res = await commands.auth.login(email, password);
      this.user = res.user;
      this.token = res.access_token;
      localStorage.setItem("orbis_token", res.access_token);
      localStorage.setItem("orbis_user", JSON.stringify(res.user));
    } catch (e) {
      this.error = e instanceof Error ? e.message : String(e);
      throw e;
    } finally {
      this.isLoading = false;
    }
  }

  async logout() {
    if (this.token) {
      const { commands } = await import("$lib/services/ipc");
      await commands.auth.logout(this.token).catch(() => {});
    }
    this.user = null;
    this.token = null;
    localStorage.removeItem("orbis_token");
    localStorage.removeItem("orbis_user");
  }

  restore() {
    try {
      const token = localStorage.getItem("orbis_token");
      const userStr = localStorage.getItem("orbis_user");
      if (token && userStr) {
        this.token = token;
        this.user = JSON.parse(userStr);
      }
    } catch {
      // Silently fail if localStorage is unavailable
    }
  }
}

export const authStore = new AuthStore();
