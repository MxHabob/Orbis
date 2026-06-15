import { useTranslation } from "react-i18next";
import { ReactNode } from "react";

export const languages = ["en", "ar"];

export function useI18n() {
  const { t } = useTranslation();
  return t;
}

export function useCurrentLocale() {
  const { i18n } = useTranslation();
  return (i18n.language || "ar") as "ar" | "en";
}

export function useChangeLocale() {
  const { i18n } = useTranslation();
  return (locale: "ar" | "en") => {
    i18n.changeLanguage(locale);
  };
}

export function useScopedI18n(scope: string) {
  const { t } = useTranslation();
  return (key: string, params?: any) => {
    return t(`${scope}.${key}`, params);
  };
}

export function I18nProviderClient({ children }: { children: ReactNode }) {
  return <>{children}</>;
}
