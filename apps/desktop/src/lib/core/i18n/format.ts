/**
 * @module core/i18n/format
 *
 * Centralized, organization-aware formatting for dates, numbers, currency, and percentages.
 *
 * ALL formatting in the Desktop app MUST go through these functions.
 * They read locale, timezone, date format, and currency from the OrganizationContextStore,
 * NOT from hard-coded locale strings.
 *
 * RULES:
 * - Do NOT use `new Intl.DateTimeFormat("ar-YE", ...)` directly in pages
 * - Do NOT use `toLocaleDateString()` directly in pages
 * - Do NOT hard-code currency codes like "YER" or "SAR"
 * - Always use formatDate(), formatCurrency(), formatNumber(), formatPercent()
 */

import { getLocale } from "$lib/paraglide/runtime";
import { orgContext } from "$lib/stores/org-context.svelte";

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

const dateFormatterCache = new Map<string, Intl.DateTimeFormat>();

function getDateFormatter(
  locale: string,
  options: Intl.DateTimeFormatOptions,
): Intl.DateTimeFormat {
  const key = `${locale}:${JSON.stringify(options)}`;
  let formatter = dateFormatterCache.get(key);
  if (!formatter) {
    formatter = new Intl.DateTimeFormat(locale, options);
    dateFormatterCache.set(key, formatter);
  }
  return formatter;
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
    return (
      new Intl.NumberFormat(locale, {
        style: "decimal",
        minimumFractionDigits: 0,
        maximumFractionDigits: 2,
      }).format(amount) + ` ${currencyCode}`
    );
  }
}

/**
 * Formats a date/datetime value using organization formatting language,
 * date pattern, time preference, and timezone.
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
  value: string | number | Date | null | undefined,
  type: "date" | "datetime" | "time" | "relative" = "date",
  options?: Intl.DateTimeFormatOptions,
): string {
  if (value === null || value === undefined || value === "") return "";

  let d: Date;
  let dateOnly = false;
  if (value instanceof Date) {
    d = value;
  } else if (typeof value === "number") {
    const normalized =
      Number.isFinite(value) && value < 1e12 ? value * 1000 : value;
    d = new Date(normalized);
  } else if (/^\d{4}-\d{2}-\d{2}$/.test(value)) {
    const [year, month, day] = value.split("-").map(Number);
    d = new Date(Date.UTC(year, month - 1, day));
    dateOnly = true;
  } else {
    d = new Date(value);
  }

  if (Number.isNaN(d.getTime())) return "";

  const locale = getFormattingLocale();
  const timezone = getFormattingTimezone();

  if (type === "relative") {
    return formatRelativeTime(d, locale);
  }

  if (type === "date" && supportsOrganizationDateFormat(options)) {
    return formatWithOrganizationDateFormat(
      d,
      locale,
      orgContext.dateFormat ?? "DD/MM/YYYY",
      {
        ...options,
        timeZone: options?.timeZone ?? (dateOnly ? "UTC" : timezone),
      },
    );
  }

  if (type === "datetime" && !options) {
    return `${formatDate(d, "date", {
      timeZone: timezone,
      year: "numeric",
      month: "2-digit",
      day: "2-digit",
    })} ${formatDate(d, "time")}`;
  }

  const dateTimezone = options?.timeZone ?? (dateOnly ? "UTC" : timezone);
  const hasGranularFields = Boolean(
    options &&
      ("year" in options ||
        "month" in options ||
        "day" in options ||
        "hour" in options ||
        "minute" in options ||
        "second" in options ||
        "weekday" in options),
  );

  const hourCycle =
    orgContext.timeFormat === "12-hour"
      ? "h12"
      : orgContext.timeFormat === "24-hour"
        ? "h23"
        : undefined;
  const baseOptions: Intl.DateTimeFormatOptions = {
    ...(!hasGranularFields &&
      type === "date" && { dateStyle: "medium" as const }),
    ...(!hasGranularFields &&
      type === "datetime" && {
        dateStyle: "medium" as const,
        timeStyle: "short" as const,
        timeZone: timezone,
      }),
    ...(!hasGranularFields &&
      type === "time" && {
        hour: "numeric" as const,
        minute: "2-digit" as const,
        timeZone: timezone,
      }),
    ...(hasGranularFields && { timeZone: dateTimezone }),
    ...options,
    ...((type === "time" || type === "datetime") && hourCycle && { hourCycle }),
  };

  return getDateFormatter(locale, baseOptions).format(d);
}

const dateFormatToken = /YYYY|MMMM|MMM|MM|DD|D|EEEE/g;

function supportsOrganizationDateFormat(
  options?: Intl.DateTimeFormatOptions,
): boolean {
  if (!options) return true;
  const formatFields = new Set([
    "year",
    "month",
    "day",
    "weekday",
    "calendar",
    "timeZone",
    "numberingSystem",
  ]);
  return (
    Object.keys(options).every((key) => formatFields.has(key)) &&
    options.year === "numeric" &&
    options.day !== undefined &&
    options.month !== undefined
  );
}

function formatWithOrganizationDateFormat(
  date: Date,
  locale: string,
  dateFormat: string,
  options?: Intl.DateTimeFormatOptions,
): string {
  dateFormatToken.lastIndex = 0;
  if (!dateFormatToken.test(dateFormat)) {
    return getDateFormatter(locale, {
      ...options,
      ...(!options?.year && { dateStyle: "medium" }),
    }).format(date);
  }
  dateFormatToken.lastIndex = 0;

  const timezone = options?.timeZone ?? getFormattingTimezone();
  const parts = getDateFormatter(locale, {
    calendar: options?.calendar,
    numberingSystem: options?.numberingSystem,
    timeZone: timezone,
    year: "numeric",
    month: "long",
    day: "2-digit",
    weekday: "long",
  }).formatToParts(date);
  const values: Record<string, string> = {};
  for (const part of parts) {
    if (
      part.type === "day" ||
      part.type === "month" ||
      part.type === "year" ||
      part.type === "weekday"
    ) {
      values[part.type] = part.value;
    }
  }
  values.monthShort = getDateFormatter(locale, {
    calendar: options?.calendar,
    numberingSystem: options?.numberingSystem,
    timeZone: timezone,
    month: "short",
  }).format(date);
  values.monthNumeric = getDateFormatter(locale, {
    calendar: options?.calendar,
    numberingSystem: options?.numberingSystem,
    timeZone: timezone,
    month: "2-digit",
  }).format(date);
  values.dayNumeric = getDateFormatter(locale, {
    calendar: options?.calendar,
    numberingSystem: options?.numberingSystem,
    timeZone: timezone,
    day: "numeric",
  }).format(date);

  return dateFormat.replace(dateFormatToken, (token) => {
    switch (token) {
      case "YYYY":
        return values.year ?? "";
      case "MMMM":
        return values.month ?? "";
      case "MMM":
        return values.monthShort ?? "";
      case "MM":
        return values.monthNumeric ?? "";
      case "DD":
        return values.day ?? "";
      case "D":
        return values.dayNumeric ?? "";
      case "EEEE":
        return values.weekday ?? "";
      default:
        return token;
    }
  });
}

export function getWeekdayOptions(): Array<{ value: number; label: string }> {
  return Array.from({ length: 7 }, (_, index) => {
    const date = new Date(Date.UTC(2023, 0, index + 1));
    return {
      value: index + 1,
      label: formatDate(date, "date", {
        weekday: "long",
        timeZone: "UTC",
      }),
    };
  });
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
