// ── License Store ──────────────────────────────────────────────
// Svelte 5 Runes-based license state management.
// Single source of truth for Mulverse license state in the Desktop UI.

import { api } from "$lib/api/client";
import type {
  LicenseStateDto,
  LicenseState,
  ActivationResultDto,
} from "$lib/api/types";
import { extractErrorMessage } from "$lib/api";

class LicenseStore {
  // Core license state from the backend
  licenseState = $state<LicenseStateDto | null>(null);
  isLoading = $state(false);
  error = $state<string | null>(null);

  // Derived state
  state = $derived<LicenseState>(this.licenseState?.state ?? "unavailable");
  isOperational = $derived(this.licenseState?.is_operational ?? false);
  isRestricted = $derived(this.licenseState?.is_restricted ?? false);
  isTrial = $derived(this.licenseState?.state === "trial");
  isActive = $derived(
    this.licenseState?.state === "active" ||
      this.licenseState?.state === "expiring" ||
      this.licenseState?.state === "offline_grace",
  );
  isExpiring = $derived(this.licenseState?.state === "expiring");
  isExpired = $derived(this.licenseState?.state === "expired");
  isRevoked = $derived(this.licenseState?.state === "revoked");
  needsActivation = $derived(this.licenseState?.state === "unavailable");

  // Plan & edition
  plan = $derived(this.licenseState?.plan ?? null);
  edition = $derived(this.licenseState?.edition ?? null);
  product = $derived(this.licenseState?.product ?? "ORBIS");

  // Trial info
  trialDaysRemaining = $derived(this.licenseState?.trial_days_remaining ?? 0);

  // License timing
  daysUntilExpiry = $derived(this.licenseState?.days_until_expiry ?? null);
  validUntil = $derived(this.licenseState?.valid_until ?? null);
  lastSyncAt = $derived(this.licenseState?.last_sync_at ?? null);

  // Features & limits
  enabledFeatures = $derived(this.licenseState?.enabled_features ?? []);
  trialFeatures = $derived(this.licenseState?.trial_features ?? []);
  limits = $derived(this.licenseState?.limits ?? []);

  // Identity
  installationId = $derived(this.licenseState?.installation_id ?? "");
  appVersion = $derived(this.licenseState?.app_version ?? "");

  /**
   * Load the current license state from the backend.
   * This is the primary initialization method — call during app startup.
   */
  async loadState(): Promise<LicenseStateDto | null> {
    this.error = null;
    try {
      this.licenseState = await api.licenseGetState();
      return this.licenseState;
    } catch (e) {
      console.warn("Failed to load license state:", e);
      this.error = extractErrorMessage(e);
      return null;
    }
  }

  /**
   * Activate a product key with Mulverse Platform.
   */
  async activateKey(productKey: string): Promise<ActivationResultDto> {
    this.isLoading = true;
    this.error = null;
    try {
      const result = await api.licenseActivateKey({
        product_key: productKey,
      });
      this.licenseState = result.license_state;
      if (!result.success) {
        this.error = result.message;
      }
      return result;
    } catch (e) {
      this.error = extractErrorMessage(e);
      return {
        success: false,
        license_state: this.licenseState!,
        message: this.error ?? "Activation failed",
      };
    } finally {
      this.isLoading = false;
    }
  }

  /**
   * Start a trial evaluation period.
   */
  async startTrial(): Promise<LicenseStateDto | null> {
    this.isLoading = true;
    this.error = null;
    try {
      this.licenseState = await api.licenseStartTrial();
      return this.licenseState;
    } catch (e) {
      this.error = extractErrorMessage(e);
      return null;
    } finally {
      this.isLoading = false;
    }
  }

  /**
   * Deactivate the current license (release device seat).
   */
  async deactivate(): Promise<LicenseStateDto | null> {
    this.isLoading = true;
    this.error = null;
    try {
      this.licenseState = await api.licenseDeactivate();
      return this.licenseState;
    } catch (e) {
      this.error = extractErrorMessage(e);
      return null;
    } finally {
      this.isLoading = false;
    }
  }

  /**
   * Force a license refresh with Mulverse Platform.
   */
  async refresh(): Promise<LicenseStateDto | null> {
    this.isLoading = true;
    this.error = null;
    try {
      this.licenseState = await api.licenseRefresh();
      return this.licenseState;
    } catch (e) {
      this.error = extractErrorMessage(e);
      return null;
    } finally {
      this.isLoading = false;
    }
  }

  /**
   * Check if a specific feature is enabled by the license.
   */
  isFeatureEnabled(key: string): boolean {
    return this.enabledFeatures.includes(key);
  }

  /**
   * Get the limit value for a given key, or null if unlimited.
   */
  getLimit(key: string): number | null {
    const entry = this.limits.find((l) => l.key === key);
    return entry?.max_value ?? null;
  }

  /**
   * Reset the store state.
   */
  reset() {
    this.licenseState = null;
    this.isLoading = false;
    this.error = null;
  }
}

export const licenseStore = new LicenseStore();
