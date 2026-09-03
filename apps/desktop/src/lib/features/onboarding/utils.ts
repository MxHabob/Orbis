export function detectTimezone(): string {
  try {
    return Intl.DateTimeFormat().resolvedOptions().timeZone || "Asia/Aden";
  } catch {
    return "Asia/Aden";
  }
}

export function detectLocale(): string {
  try {
    const lang =
      navigator.language ||
      (navigator.languages && navigator.languages[0]) ||
      "ar";
    const cleanLang = lang.split("-")[0]?.toLowerCase();
    return cleanLang === "en" || cleanLang === "ar" ? cleanLang : "ar";
  } catch {
    return "ar";
  }
}

export function detectCurrency(timezone: string): string {
  if (timezone.includes("Riyadh") || timezone.includes("Saudi")) return "SAR";
  if (timezone.includes("Dubai") || timezone.includes("Uae")) return "AED";
  if (timezone.includes("Cairo") || timezone.includes("Egypt")) return "EGP";
  if (timezone.includes("Aden") || timezone.includes("Yemen")) return "YER";
  return detectLocale() === "en" ? "USD" : "SAR"; // Smarter fallback based on locale
}

export function getDefaultBuildings(): string[] {
  return detectLocale() === "ar" ? ["المبنى الرئيسي"] : ["Main Building"];
}
