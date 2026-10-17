// ── Auth Store ───────────────────────────────────────────────────────
// Svelte 5 Runes-based authentication state.

import { api } from "$lib/api/client";
import type { UserDto, EffectiveContextDto, LoginResponse } from "$lib/api/types";
import { extractErrorMessage } from "$lib/api";
import { effectiveContext } from "./effective-context.svelte";
import { systemStore } from "./system.svelte";

class AuthStore {
  user = $state<UserDto | null>(null);
  token = $state<string | null>(null);
  isLoading = $state(false);
  error = $state<string | null>(null);

  isAuthenticated = $derived(!!this.token && !!this.user);
  orgId = $derived(this.user?.org_id ?? "");
  userId = $derived(this.user?.id ?? "");
  
  // Organization readiness integration
  isSetupComplete = $derived(
    systemStore.orgReadiness ? systemStore.isOperational : null
  );
  needsSetup = $derived(
    this.isAuthenticated && !systemStore.isOperational
  );

  async login(email: string, password: string): Promise<LoginResponse> {
    this.isLoading = true;
    this.error = null;
    try {
      const res = await api.login({ email, password });
      if (res.mfa_required) return res;
      if (!res.token) {
        throw new Error("The login response did not contain an access token.");
      }
      this.user = res.user;
      this.token = res.token;
      this.restorePromise = null;
      localStorage.setItem("orbis_token", res.token);
      localStorage.setItem("orbis_user", JSON.stringify(res.user));
      if (res.effective_context) {
        effectiveContext.setEffectiveContext(res.effective_context);
      }
      await systemStore.loadReadiness();
      return res;
    } catch (e) {
      this.error = extractErrorMessage(e);
      throw e;
    } finally {
      this.isLoading = false;
    }
  }

  async setSession(res: {
    token: string;
    user: UserDto;
    effective_context?: EffectiveContextDto | null;
  }) {
    this.user = res.user;
    this.token = res.token;
    this.error = null;
    this.restorePromise = null;
    localStorage.setItem("orbis_token", res.token);
    localStorage.setItem("orbis_user", JSON.stringify(res.user));
    if (res.effective_context) {
      effectiveContext.setEffectiveContext(res.effective_context);
    }
    await systemStore.loadReadiness();
  }

  async logout() {
    try {
      await api.logout();
    } catch {
      // Ignore network errors on logout
    }
    this.user = null;
    this.token = null;
    this.restorePromise = null;
    effectiveContext.reset();
    systemStore.reset();
    localStorage.removeItem("orbis_token");
    localStorage.removeItem("orbis_user");
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
      if (token && userStr) {
        try {
          this.token = token;
          this.user = JSON.parse(userStr);
          const res = await api.getSession();
          if (res?.user) {
            this.user = res.user;
            localStorage.setItem("orbis_user", JSON.stringify(res.user));
            if (res.effective_context) {
              effectiveContext.setEffectiveContext(res.effective_context);
            }
          }
          await systemStore.loadReadiness();
        } catch (e) {
          console.warn("Session revalidation notice:", e);
          const errMessage = extractErrorMessage(e);
          if (
            errMessage.toLowerCase().includes("unauthorized") ||
            errMessage.toLowerCase().includes("revoked") ||
            errMessage.toLowerCase().includes("invalid token")
          ) {
            this.token = null;
            this.user = null;
            localStorage.removeItem("orbis_token");
            localStorage.removeItem("orbis_user");
          }
        }
      }
    } catch {
      // Silently fail if localStorage is unavailable
    }
  }

  markSetupComplete() {
    // Reload readiness from the backend to get the authoritative state
    // instead of manually overriding the derived value.
    systemStore.loadReadiness();
  }
}

export const authStore = new AuthStore();
