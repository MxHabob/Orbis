/**
 * @module @orbis/mobile/lib/i18n/locale
 *
 * Core locale state, detection, and translation functions.
 * 100% parity with desktop i18n APIs (l, isAr, getLocale, setLocale).
 */

import * as Localization from "expo-localization";
import { SupportedLocale } from "./types";
import { ar, TranslationKey } from "./dictionaries/ar";
import { en } from "./dictionaries/en";
import { getSecureItem, setSecureItem } from "@/lib/api/storage";

const LOCALE_STORAGE_KEY = "orbis_app_language";

const dictionaries: Record<SupportedLocale, Record<string, string>> = {
  ar,
  en,
};

function getDeviceLocale(): SupportedLocale {
  try {
    for (const deviceLocale of Localization.getLocales()) {
      const languageCode = deviceLocale.languageCode?.toLowerCase();
      if (languageCode?.startsWith("ar")) return "ar";
      if (languageCode?.startsWith("en")) return "en";
    }
  } catch (error) {
    console.error(
      "[Orbis Mobile I18n] Failed to read the device locale.",
      error,
    );
    return "ar";
  }

  return "ar";
}

let currentLocale: SupportedLocale = getDeviceLocale();

/**
 * Returns the currently active locale ('ar' | 'en').
 */
export function getLocale(): SupportedLocale {
  return currentLocale;
}

/**
 * Checks if the active locale is Arabic.
 */
export function isAr(): boolean {
  return currentLocale === "ar";
}

/**
 * Returns the localized value based on the current locale.
 * Fallbacks to the other language if the current locale's value is missing.
 * Matches desktop `l(arValue, enValue)` 100%.
 */
export function l<T>(arValue: T, enValue: T): T {
  const isArabic = currentLocale === "ar";
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
 * Translates a key with optional parameter substitution.
 * e.g. t('dashboard_greeting', { name: 'Ali' }) -> "أهلاً بك، Ali"
 */
export function t(
  key: TranslationKey,
  params?: Record<string, string | number>,
): string {
  const dict = dictionaries[currentLocale] || dictionaries.ar;
  let text =
    dict[key] || (dictionaries.ar as Record<string, string>)[key] || key;

  if (params) {
    for (const [paramKey, paramVal] of Object.entries(params)) {
      text = text.replace(
        new RegExp(`\\{${paramKey}\\}`, "g"),
        String(paramVal),
      );
    }
  }
  return text;
}

/**
 * Internal setter used during initialization or switching.
 */
export function setCurrentLocaleInternal(locale: SupportedLocale): void {
  currentLocale = locale;
}

/**
 * Persists and updates the application locale.
 */
export async function setLocale(locale: SupportedLocale): Promise<void> {
  currentLocale = locale;
  await setSecureItem(LOCALE_STORAGE_KEY, locale);
}

/**
 * Reads persisted locale from secure storage if available.
 */
export async function loadPersistedLocale(): Promise<SupportedLocale> {
  try {
    const saved = await getSecureItem(LOCALE_STORAGE_KEY);
    if (saved === "ar" || saved === "en") {
      currentLocale = saved;
      return saved;
    }
  } catch (error) {
    console.error("[Orbis Mobile I18n] Failed to read the saved locale.", error);
  }
  return currentLocale;
}
