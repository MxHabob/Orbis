/**
 * @module core/i18n/format
 *
 * Centralized, organization-aware formatting for dates, numbers, currency, and percentages.
 *
 * ALL formatting in the Desktop app MUST go through these functions.
 * They read locale, timezone, and currency from the OrganizationContextStore,
 * NOT from hard-coded locale strings.
 *
 * RULES:
 * - Do NOT use `new Intl.DateTimeFormat("ar-YE", ...)` directly in pages
 * - Do NOT use `toLocaleDateString()` directly in pages
 * - Do NOT hard-code currency codes like "YER" or "SAR"
 * - Always use formatDate(), formatCurrency(), formatNumber(), formatPercent()
 */

import { orgContext } from "$lib/stores/org-context.svelte";
import { getLocale } from "$lib/paraglide/runtime";

// ── Internal helpers ─────────────────────────────────────────────

/**
 * Returns the best Intl locale tag for formatting.
 *
 * Priority: Organization intlLocale → paraglide UI locale fallback.
 * This ensures data formatting follows the organization's configuration,
 * while UI translations follow paraglide.
 */
function getFormattingLocale(): string {
  // Use org context if available (configuration-driven)
  if (orgContext.isInitialized) {
    return orgContext.intlLocale;
  }
  // Fallback: derive from paraglide UI locale during app startup
  const uiLocale = getLocale();
  return uiLocale === "ar" ? "ar-SA" : "en-US";
}

/**
 * Returns the organization's timezone for display purposes.
 * Falls back to browser timezone if org context is not loaded.
 */
function getFormattingTimezone(): string {
  if (orgContext.isInitialized) {
    return orgContext.timezone;
  }
  return Intl.DateTimeFormat().resolvedOptions().timeZone;
}

// ── Public API ───────────────────────────────────────────────────

/**
 * Formats a currency amount using the organization's base currency and locale.
 *
 * @param amount - Numeric amount to format
 * @param currency - Override currency code; defaults to org base currency
 * @param options - Additional Intl.NumberFormat options
 */
export function formatCurrency(
  amount: number | null | undefined,
  currency?: string,
  options?: { minimumFractionDigits?: number; maximumFractionDigits?: number },
): string {
  if (amount === null || amount === undefined) return "";

  const locale = getFormattingLocale();
  const currencyCode = (currency ?? orgContext.baseCurrency ?? "SAR")
    .trim()
    .toUpperCase();

  try {
    return new Intl.NumberFormat(locale, {
      style: "currency",
      currency: currencyCode,
      minimumFractionDigits: options?.minimumFractionDigits ?? 0,
      maximumFractionDigits: options?.maximumFractionDigits ?? 2,
    }).format(amount);
  } catch {
    // Fallback if currency code is unrecognized by Intl
    return new Intl.NumberFormat(locale, {
      style: "decimal",
      minimumFractionDigits: 0,
      maximumFractionDigits: 2,
    }).format(amount) + ` ${currencyCode}`;
  }
}

/**
 * Formats a date/datetime value using organization timezone and locale.
 *
 * Distinguishes between semantic date types:
 * - "date": Date only (e.g., birth_date, academic dates) — timezone-independent
 * - "datetime": Full date and time — displayed in org timezone
 * - "time": Time only — displayed in org timezone
 * - "relative": Relative time (e.g., "3 days ago")
 *
 * @param value - Date string (ISO 8601) or Date object
 * @param type - Semantic type of the date
 * @param options - Additional Intl.DateTimeFormat options
 */
export function formatDate(
  value: string | Date | null | undefined,
  type: "date" | "datetime" | "time" | "relative" = "date",
  options?: Intl.DateTimeFormatOptions,
): string {
  if (!value) return "";

  const d = typeof value === "string" ? new Date(value) : value;
  if (isNaN(d.getTime())) return "";

  const locale = getFormattingLocale();
  const timezone = getFormattingTimezone();

  if (type === "relative") {
    return formatRelativeTime(d, locale);
  }

  const baseOptions: Intl.DateTimeFormatOptions = {
    ...(type === "date" && { dateStyle: "medium" as const }),
    ...(type === "datetime" && {
      dateStyle: "medium" as const,
      timeStyle: "short" as const,
      timeZone: timezone,
    }),
    ...(type === "time" && {
      timeStyle: "short" as const,
      timeZone: timezone,
    }),
    ...options,
  };

  return new Intl.DateTimeFormat(locale, baseOptions).format(d);
}

/**
 * Formats a percentage value (e.g., 95.5 → "95.5%").
 *
 * @param value - The percentage value (e.g., 95 for 95%)
 */
export function formatPercent(value: number | null | undefined): string {
  if (value === null || value === undefined) return "";
  const locale = getFormattingLocale();
  return new Intl.NumberFormat(locale, {
    style: "percent",
    minimumFractionDigits: 0,
    maximumFractionDigits: 1,
  }).format(value / 100);
}

/**
 * Formats a number using the organization's locale.
 *
 * @param value - Number to format
 * @param options - Additional Intl.NumberFormat options
 */
export function formatNumber(
  value: number | null | undefined,
  options?: Intl.NumberFormatOptions,
): string {
  if (value === null || value === undefined) return "";
  const locale = getFormattingLocale();
  return new Intl.NumberFormat(locale, options).format(value);
}

/**
 * Formats a decimal number with explicit precision control.
 * Useful for financial amounts, GPA values, etc.
 *
 * @param value - Number to format
 * @param decimals - Number of decimal places
 */
export function formatDecimal(
  value: number | null | undefined,
  decimals: number = 2,
): string {
  if (value === null || value === undefined) return "";
  const locale = getFormattingLocale();
  return new Intl.NumberFormat(locale, {
    minimumFractionDigits: decimals,
    maximumFractionDigits: decimals,
  }).format(value);
}

// ── Internal ─────────────────────────────────────────────────────

function formatRelativeTime(date: Date, locale: string): string {
  const now = Date.now();
  const diffMs = now - date.getTime();
  const diffSec = Math.floor(diffMs / 1000);
  const diffMin = Math.floor(diffSec / 60);
  const diffHour = Math.floor(diffMin / 60);
  const diffDay = Math.floor(diffHour / 24);

  try {
    const rtf = new Intl.RelativeTimeFormat(locale, { numeric: "auto" });
    if (diffDay > 30) return formatDate(date, "date");
    if (diffDay > 0) return rtf.format(-diffDay, "day");
    if (diffHour > 0) return rtf.format(-diffHour, "hour");
    if (diffMin > 0) return rtf.format(-diffMin, "minute");
    return rtf.format(-diffSec, "second");
  } catch {
    // Fallback
    if (diffDay > 0) return `${diffDay}d ago`;
    if (diffHour > 0) return `${diffHour}h ago`;
    return `${diffMin}m ago`;
  }
}
