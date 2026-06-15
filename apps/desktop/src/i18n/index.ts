import i18n from "i18next";
import { initReactI18next } from "react-i18next";
import ar from "../locales/ar";
import en from "../locales/en";

i18n.use(initReactI18next).init({
  resources: {
    ar: { translation: ar },
    en: { translation: en },
  },
  lng: localStorage.getItem("locale") || "ar",
  fallbackLng: "en",
  interpolation: {
    escapeValue: false,
    prefix: "{",
    suffix: "}",
  },
  react: {
    useSuspense: false,
  },
});

export default i18n;
