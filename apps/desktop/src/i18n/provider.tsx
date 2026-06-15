import { ReactNode, useEffect, useState } from "react";
import { I18nextProvider } from "react-i18next";
import i18n from "./index";

export function I18nProvider({ children }: { children: ReactNode }) {
  const [, setLocale] = useState(i18n.language);

  useEffect(() => {
    const handleLanguageChange = (lng: string) => {
      setLocale(lng);
      localStorage.setItem("locale", lng);
      
      const dir = lng === "ar" ? "rtl" : "ltr";
      document.documentElement.dir = dir;
      document.documentElement.lang = lng;
    };

    i18n.on("languageChanged", handleLanguageChange);
    
    // Trigger initially
    handleLanguageChange(i18n.language);

    return () => {
      i18n.off("languageChanged", handleLanguageChange);
    };
  }, []);

  return <I18nextProvider i18n={i18n}>{children}</I18nextProvider>;
}
