/**
 * @module @orbis/mobile/lib/i18n
 *
 * Core i18n & l10n Utilities for Orbis Mobile.
 * Complete 100% desktop API parity + reactive React hooks & Context.
 */

export {
  formatCurrency,
  formatDate,
  formatPercent,
  formatNumber,
} from "./format";

export {
  l,
  isAr,
  getLocale,
  setLocale,
  t,
  loadPersistedLocale,
} from "./locale";

export {
  gregorianToHijri,
  hijriToGregorian,
  getHijriMonths,
  type HijriDate,
} from "./hijri";

export { I18nProvider, useI18n } from "./i18n-provider";
export type { SupportedLocale, I18nContextValue } from "./types";
export type { TranslationKey } from "./dictionaries/ar";
