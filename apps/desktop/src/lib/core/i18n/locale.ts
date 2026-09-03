import { getLocale } from "$lib/paraglide/runtime";

/**
 * Returns the localized value based on the current locale.
 * Fallbacks to the other language if the current locale's value is missing.
 */
export function l<T>(arValue: T, enValue: T): T {
  const isArabic = getLocale() === "ar";
  if (isArabic) {
    return arValue !== undefined && arValue !== null && (arValue as any) !== ""
      ? arValue
      : enValue;
  } else {
    return enValue !== undefined && enValue !== null && (enValue as any) !== ""
      ? enValue
      : arValue;
  }
}

/**
 * Checks if the current locale is Arabic.
 */
export function isAr(): boolean {
  return getLocale() === "ar";
}
