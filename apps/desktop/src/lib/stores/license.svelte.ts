// ── License Store ──────────────────────────────────────────────
// Svelte 5 Runes-based license state management.
// Single source of truth for Mulverse license state in the Desktop UI.

import { api } from "$lib/api/client";
import type {
  LicenseStateDto,
  LicenseState,
} from "$lib/api/types";
import { extractErrorMessage } from "$lib/api";

class LicenseStore {
  // Core license state from the backend
  licenseState = $state<LicenseStateDto | null>(null);
  isLoading = $state(false);
  hasLoaded = $state(false);
  error = $state<string | null>(null);
  private pendingActions = 0;
  private latestActionId = 0;

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
  planName = $derived(this.licenseState?.plan_name ?? null);
  edition = $derived(this.licenseState?.edition ?? null);
  product = $derived(this.licenseState?.product ?? "ORBIS");

  // Trial info
  trialDaysRemaining = $derived(this.licenseState?.trial_days_remaining ?? 0);

  // License timing
  daysUntilExpiry = $derived(this.licenseState?.days_until_expiry ?? null);
  validUntil = $derived(this.licenseState?.valid_until ?? null);
  lastSyncAt = $derived(this.licenseState?.last_sync_at ?? null);
  organizationId = $derived(this.licenseState?.organization_id ?? null);

  // Features & limits
  enabledFeatures = $derived(this.licenseState?.enabled_features ?? []);
  trialFeatures = $derived(this.licenseState?.trial_features ?? []);
  limits = $derived(this.licenseState?.limits ?? []);

  // Identity
  installationId = $derived(this.licenseState?.installation_id ?? "");
  appVersion = $derived(this.licenseState?.app_version ?? "");

  private async runAction<T>(
    action: () => Promise<T>,
    onSuccess: (value: T) => void,
  ): Promise<T | null> {
    const actionId = ++this.latestActionId;
    this.error = null;
    this.pendingActions++;
    this.isLoading = true;
    try {
      const value = await action();
      if (actionId === this.latestActionId) {
        onSuccess(value);
        this.hasLoaded = true;
      }
      return value;
    } catch (error) {
      if (actionId === this.latestActionId) {
        this.error = extractErrorMessage(error);
      }
      return null;
    } finally {
      this.pendingActions--;
      this.isLoading = this.pendingActions > 0;
    }
  }

  /**
   * Load the current license state from the backend.
   * This is the primary initialization method — call during app startup.
   */
  async loadState(): Promise<LicenseStateDto | null> {
    return this.runAction(api.licenseGetState, (state) => {
      this.licenseState = state;
    });
  }

  /**
   * Activate a product key with Mulverse Platform.
   */
  async activateKey(productKey: string): Promise<LicenseStateDto | null> {
    return this.runAction(
      () => api.licenseActivateKey({ product_key: productKey }),
      (state) => {
        this.licenseState = state;
      },
    );
  }

  /**
   * Claim a reseller key for the active organization and activate this device.
   */
  async redeemKey(productKey: string): Promise<LicenseStateDto | null> {
    return this.runAction(
      () => api.licenseRedeemKey({ product_key: productKey }),
      (state) => {
        this.licenseState = state;
      },
    );
  }

  /**
   * Start a trial evaluation period.
   */
  async startTrial(): Promise<LicenseStateDto | null> {
    return this.runAction(api.licenseStartTrial, (state) => {
      this.licenseState = state;
    });
  }

  /**
   * Deactivate the current license (release device seat).
   */
  async deactivate(): Promise<LicenseStateDto | null> {
    return this.runAction(api.licenseDeactivate, (state) => {
      this.licenseState = state;
    });
  }

  /**
   * Force a license refresh with Mulverse Platform.
   */
  async refresh(): Promise<LicenseStateDto | null> {
    return this.runAction(api.licenseRefresh, (state) => {
      this.licenseState = state;
    });
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
    this.latestActionId++;
    this.licenseState = null;
    this.isLoading = false;
    this.hasLoaded = false;
    this.error = null;
  }
}

export const licenseStore = new LicenseStore();
