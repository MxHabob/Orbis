/**
 * Organization Context Store — Centralized org configuration for the Desktop app.
 *
 * This is the SINGLE SOURCE OF TRUTH for:
 * - Organization identity (name, logo, branding)
 * - Locale & formatting configuration (timezone, locale, date format, currency)
 * - Feature flags and limits
 * - Academic context reference (current_period_id)
 *
 * All formatting functions (dates, numbers, currency) MUST read from this store,
 * NOT from hard-coded locale strings.
 */

import { api } from "$lib/api/client";
import { extractErrorMessage } from "$lib/api";

export interface OrgConfig {
  id: string;
  name: string;
  nameAr: string | null;
  slug: string;
  code: string | null;
  timezone: string;
  locale: string;
  dateFormat: string | null;
  isHijriEnabled: boolean;
  baseCurrencyCode: string | null;
  primaryColor: string | null;
  logo: string | null;
  favicon: string | null;
  currentPeriodId: string | null;
  isActive: boolean;
  isVerified: boolean;
  settings: Record<string, unknown> | null;
  features: Record<string, unknown> | null;
}

/**
 * Resolves the Intl-compatible locale tag from the organization locale.
 *
 * Organization.locale stores values like "ar", "en", "ar-SA", "en-US".
 * This function returns a valid BCP 47 locale tag for Intl APIs.
 */
function resolveIntlLocale(orgLocale: string): string {
  // If it already has a region (e.g., "ar-SA", "en-US"), use as-is
  if (orgLocale.includes("-")) return orgLocale;

  // Map bare language codes to sensible defaults
  const defaults: Record<string, string> = {
    ar: "ar-SA",
    en: "en-US",
    fr: "fr-FR",
    es: "es-ES",
    tr: "tr-TR",
    ur: "ur-PK",
  };

  return defaults[orgLocale] ?? orgLocale;
}

class OrganizationContextStore {
  // ── Raw org data ───────────────────────────────────────────
  private _config = $state<OrgConfig | null>(null);
  isLoading = $state(false);
  isInitialized = $state(false);

  // ── Derived accessors ──────────────────────────────────────

  /** Organization ID */
  orgId = $derived(this._config?.id ?? "");

  /** Display name (uses Arabic name when locale is Arabic) */
  displayName = $derived.by(() => {
    if (!this._config) return "";
    const isAr = this._config.locale.startsWith("ar");
    if (isAr && this._config.nameAr) return this._config.nameAr;
    return this._config.name;
  });

  /** Organization English name */
  name = $derived(this._config?.name ?? "");

  /** Organization Arabic name */
  nameAr = $derived(this._config?.nameAr ?? null);

  /** Organization timezone (e.g., "Asia/Riyadh", "Asia/Aden") */
  timezone = $derived(this._config?.timezone ?? "UTC");

  /** Organization locale (e.g., "ar", "en", "ar-SA") */
  locale = $derived(this._config?.locale ?? "ar");

  /** Intl-compatible locale tag for formatting APIs */
  intlLocale = $derived(resolveIntlLocale(this._config?.locale ?? "ar"));

  /** Organization date format string (e.g., "DD/MM/YYYY") */
  dateFormat = $derived(this._config?.dateFormat ?? "DD/MM/YYYY");

  /** Whether Hijri calendar display is enabled */
  isHijriEnabled = $derived(this._config?.isHijriEnabled ?? false);

  /** Base currency code (e.g., "SAR", "YER", "USD") */
  baseCurrency = $derived(this._config?.baseCurrencyCode ?? "SAR");

  /** Primary brand color */
  primaryColor = $derived(this._config?.primaryColor ?? "#6366f1");

  /** Organization logo URL */
  logo = $derived(this._config?.logo ?? null);

  /** Whether the UI language direction should be RTL */
  isRtl = $derived(this._config?.locale.startsWith("ar") ?? true);

  /** Organization current period ID from backend */
  currentPeriodId = $derived(this._config?.currentPeriodId ?? null);

  /** Full config object for components that need multiple fields */
  config = $derived(this._config);

  // ── Methods ────────────────────────────────────────────────

  /**
   * Load organization configuration from Backend.
   * Should be called once during app initialization (after login).
   */
  async load() {
    this.isLoading = true;
    try {
      const org = await api.getCurrentOrg();
      this._config = {
        id: org.id,
        name: org.name,
        nameAr: (org as any).name_ar ?? null,
        slug: org.slug,
        code: (org as any).code ?? null,
        timezone: org.timezone,
        locale: org.locale,
        dateFormat: (org as any).date_format ?? null,
        isHijriEnabled: (org as any).is_hijri_enabled ?? false,
        baseCurrencyCode: (org as any).base_currency_code ?? null,
        primaryColor: (org as any).primary_color ?? null,
        logo: (org as any).logo ?? null,
        favicon: (org as any).favicon ?? null,
        currentPeriodId: (org as any).current_period_id ?? null,
        isActive: (org as any).is_active ?? true,
        isVerified: (org as any).is_verified ?? false,
        settings: (org as any).settings ?? null,
        features: (org as any).features ?? null,
      };
      this.isInitialized = true;
    } catch (e: unknown) {
      console.warn("Failed to load organization context:", extractErrorMessage(e));
    } finally {
      this.isLoading = false;
    }
  }

  /**
   * Refresh org config (call after settings update).
   */
  async refresh() {
    await this.load();
  }

  /**
   * Reset store (call on logout).
   */
  reset() {
    this._config = null;
    this.isInitialized = false;
  }
}

export const orgContext = new OrganizationContextStore();
