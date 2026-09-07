/**
 * @module @orbis/mobile/lib/i18n/types
 *
 * Types for Orbis Mobile Localization & Internationalization.
 */

import { TranslationKey } from './dictionaries/ar';

export type SupportedLocale = 'ar' | 'en';

export type { TranslationKey };

export interface I18nContextValue {
  locale: SupportedLocale;
  isArabic: boolean;
  setLocale: (nextLocale: SupportedLocale) => Promise<void>;
  t: (key: TranslationKey, params?: Record<string, string | number>) => string;
  l: <T>(arValue: T, enValue: T) => T;
  formatCurrency: (amount: number | null | undefined, currency?: string) => string;
  formatDate: (date: string | Date | null | undefined) => string;
  formatPercent: (value: number | null | undefined) => string;
  formatNumber: (value: number | null | undefined) => string;
}
