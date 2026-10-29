/**
 * @module @orbis/mobile/lib/i18n/i18n-provider
 *
 * React Context and Hook for Reactive Internationalization in Orbis Mobile.
 */

import React, {
  createContext,
  useContext,
  useEffect,
  useState,
  useMemo,
  useCallback,
} from "react";
import { I18nContextValue, SupportedLocale } from "./types";
import {
  getLocale,
  setLocale as setStoredLocale,
  loadPersistedLocale,
  setCurrentLocaleInternal,
  synchronizeLocaleDirection,
  t as translateKey,
  l as localizeValue,
  isAr as checkIsAr,
} from "./locale";
import {
  formatCurrency as formatCurr,
  formatDate as formatDt,
  formatPercent as formatPct,
  formatNumber as formatNum,
} from "./format";

const I18nContext = createContext<I18nContextValue | null>(null);

export function I18nProvider({ children }: { children: React.ReactNode }) {
  const [locale, setLocaleState] = useState<SupportedLocale>(getLocale());
  const [isReady, setIsReady] = useState(false);

  useEffect(() => {
    let isMounted = true;
    loadPersistedLocale()
      .then(async (savedLocale) => {
        if (!isMounted) return;

        if (savedLocale !== locale) {
          setLocaleState(savedLocale);
        }

        await synchronizeLocaleDirection(savedLocale);
      })
      .catch((error: unknown) => {
        console.error(
          "[Orbis Mobile I18n] Failed to load saved locale.",
          error,
        );
      })
      .finally(() => {
        if (isMounted) {
          setIsReady(true);
        }
      });
    return () => {
      isMounted = false;
    };
  }, []);

  const handleSetLocale = useCallback(async (nextLocale: SupportedLocale) => {
    setCurrentLocaleInternal(nextLocale);
    setLocaleState(nextLocale);
    await setStoredLocale(nextLocale);
  }, []);

  const contextValue: I18nContextValue = useMemo(() => {
    return {
      locale,
      isArabic: locale === "ar",
      isReady,
      setLocale: handleSetLocale,
      t: (key, params) => translateKey(key, params),
      l: (arVal, enVal) => localizeValue(arVal, enVal),
      formatCurrency: (amount, currency) => formatCurr(amount, currency),
      formatDate: (date) => formatDt(date),
      formatPercent: (val) => formatPct(val),
      formatNumber: (val) => formatNum(val),
    };
  }, [isReady, locale, handleSetLocale]);

  return (
    <I18nContext.Provider value={contextValue}>{children}</I18nContext.Provider>
  );
}

export function useI18n(): I18nContextValue {
  const context = useContext(I18nContext);
  if (!context) {
    // Graceful fallback for components rendered outside provider
    const activeLocale = getLocale();
    return {
      locale: activeLocale,
      isArabic: checkIsAr(),
      isReady: true,
      setLocale: async (loc) => {
        setCurrentLocaleInternal(loc);
        await setStoredLocale(loc);
      },
      t: (key, params) => translateKey(key, params),
      l: (arVal, enVal) => localizeValue(arVal, enVal),
      formatCurrency: (amount, currency) => formatCurr(amount, currency),
      formatDate: (date) => formatDt(date),
      formatPercent: (val) => formatPct(val),
      formatNumber: (val) => formatNum(val),
    };
  }
  return context;
}
