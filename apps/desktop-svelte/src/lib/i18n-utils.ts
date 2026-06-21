import { getLocale } from "$lib/paraglide/runtime";

/**
 * Formats a currency amount to the current locale's format.
 * Defaults to YER.
 */
export function formatCurrency(amount: number | null | undefined, currency: string = "YER"): string {
  if (amount === null || amount === undefined) return "";
  const lang = getLocale();
  // Clean up currency code if it has modifiers
  const cleanCurrency = String(currency || "YER").trim().toUpperCase();
  try {
    return new Intl.NumberFormat(lang === "ar" ? "ar-YE" : "en-US", {
      style: "currency",
      currency: cleanCurrency,
      minimumFractionDigits: 0,
      maximumFractionDigits: 2,
    }).format(amount);
  } catch (e) {
    // Fallback if the currency code is unrecognized
    return new Intl.NumberFormat(lang === "ar" ? "ar-YE" : "en-US", {
      style: "currency",
      currency: "YER",
      minimumFractionDigits: 0,
      maximumFractionDigits: 2,
    }).format(amount);
  }
}

/**
 * Formats a date string or object to the current locale's format.
 */
export function formatDate(date: string | Date | null | undefined): string {
  if (!date) return "";
  const d = typeof date === "string" ? new Date(date) : date;
  if (isNaN(d.getTime())) return "";
  const lang = getLocale();
  return new Intl.DateTimeFormat(lang === "ar" ? "ar-YE" : "en-US", {
    dateStyle: "medium",
  }).format(d);
}

/**
 * Formats a percentage value (e.g. 95) to the current locale's format.
 */
export function formatPercent(value: number | null | undefined): string {
  if (value === null || value === undefined) return "";
  const lang = getLocale();
  // Divide by 100 if value is e.g. 95 for 95%
  return new Intl.NumberFormat(lang === "ar" ? "ar-YE" : "en-US", {
    style: "percent",
    minimumFractionDigits: 0,
    maximumFractionDigits: 1,
  }).format(value / 100);
}

/**
 * Formats a number to the current locale's format.
 */
export function formatNumber(value: number | null | undefined): string {
  if (value === null || value === undefined) return "";
  const lang = getLocale();
  return new Intl.NumberFormat(lang === "ar" ? "ar-YE" : "en-US").format(value);
}
