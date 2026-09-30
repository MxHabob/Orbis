import type { InstitutionType } from "@orbis/blueprints";

export type OnboardingStep = "blueprint" | "organization" | "admin" | "review";

export interface TimezoneOption {
  value: string;
  labelEn: string;
  labelAr: string;
  offset: string;
}

export interface CurrencyOption {
  code: string;
  symbolEn: string;
  symbolAr: string;
  nameEn: string;
  nameAr: string;
}

export interface ColorPreset {
  name: string;
  hex: string;
  labelEn: string;
  labelAr: string;
}

export const TIMEZONE_OPTIONS: TimezoneOption[] = [
  { value: "Asia/Riyadh", labelEn: "Riyadh (KSA / GCC)", labelAr: "الرياض (السعودية والخليج)", offset: "UTC+3" },
  { value: "Asia/Dubai", labelEn: "Dubai (UAE / Oman)", labelAr: "دبي (الإمارات وعمان)", offset: "UTC+4" },
  { value: "Asia/Kuwait", labelEn: "Kuwait / Qatar / Bahrain", labelAr: "الكويت / قطر / البحرين", offset: "UTC+3" },
  { value: "Africa/Cairo", labelEn: "Cairo (Egypt)", labelAr: "القاهرة (مصر)", offset: "UTC+2" },
  { value: "Asia/Amman", labelEn: "Amman (Jordan)", labelAr: "عمّان (الأردن)", offset: "UTC+3" },
  { value: "Asia/Baghdad", labelEn: "Baghdad (Iraq)", labelAr: "بغداد (العراق)", offset: "UTC+3" },
  { value: "Europe/London", labelEn: "London (GMT / BST)", labelAr: "لندن (المملكة المتحدة)", offset: "UTC+0" },
  { value: "America/New_York", labelEn: "Eastern Time (US & Canada)", labelAr: "التوقيت الشرقي (أمريكا)", offset: "UTC-5" },
  { value: "UTC", labelEn: "Coordinated Universal Time (UTC)", labelAr: "التوقيت العالمي الموحد (UTC)", offset: "UTC+0" },
];

export const CURRENCY_OPTIONS: CurrencyOption[] = [
  { code: "SAR", symbolEn: "SAR", symbolAr: "ر.س", nameEn: "Saudi Riyal", nameAr: "ريال سعودي" },
  { code: "AED", symbolEn: "AED", symbolAr: "د.إ", nameEn: "UAE Dirham", nameAr: "درهم إماراتي" },
  { code: "KWD", symbolEn: "KWD", symbolAr: "د.ك", nameEn: "Kuwaiti Dinar", nameAr: "دينار كويتي" },
  { code: "QAR", symbolEn: "QAR", symbolAr: "ر.ق", nameEn: "Qatari Riyal", nameAr: "ريال قطري" },
  { code: "BHD", symbolEn: "BHD", symbolAr: "د.ب", nameEn: "Bahraini Dinar", nameAr: "دينار بحريني" },
  { code: "OMR", symbolEn: "OMR", symbolAr: "ر.ع", nameEn: "Omani Rial", nameAr: "ريال عماني" },
  { code: "EGP", symbolEn: "EGP", symbolAr: "ج.م", nameEn: "Egyptian Pound", nameAr: "جنيه مصري" },
  { code: "JOD", symbolEn: "JOD", symbolAr: "د.أ", nameEn: "Jordanian Dinar", nameAr: "دينار أردني" },
  { code: "USD", symbolEn: "USD ($)", symbolAr: "دولار ($)", nameEn: "US Dollar", nameAr: "دولار أمريكي" },
  { code: "EUR", symbolEn: "EUR (€)", symbolAr: "يورو (€)", nameEn: "Euro", nameAr: "يورو أوروبي" },
  { code: "GBP", symbolEn: "GBP (£)", symbolAr: "جنيه (£)", nameEn: "British Pound", nameAr: "جنيه إسترليني" },
];

export const COLOR_PRESETS: ColorPreset[] = [
  { name: "indigo", hex: "#4f46e5", labelEn: "Royal Indigo", labelAr: "نيلي ملكي" },
  { name: "blue", hex: "#2563eb", labelEn: "Ocean Blue", labelAr: "أزرق محيطي" },
  { name: "sky", hex: "#0284c7", labelEn: "Sky Blue", labelAr: "أزرق سماوي" },
  { name: "teal", hex: "#0d9488", labelEn: "Emerald Teal", labelAr: "زمردي بحري" },
  { name: "emerald", hex: "#059669", labelEn: "Forest Emerald", labelAr: "أخضر زمردي" },
  { name: "amber", hex: "#d97706", labelEn: "Warm Amber", labelAr: "كهرماني دافئ" },
  { name: "rose", hex: "#e11d48", labelEn: "Vibrant Rose", labelAr: "وردي حيوي" },
  { name: "purple", hex: "#9333ea", labelEn: "Noble Purple", labelAr: "أرجواني نبيل" },
];

export interface OnboardingFormModel {
  // Step 1: Blueprint
  institutionType: InstitutionType;

  // Step 2: Organization
  organizationName: string;
  organizationNameAr: string;
  organizationSlug: string;
  organizationCode: string;
  timezone: string;
  locale: string;
  dateFormat: string;
  isHijriEnabled: boolean;
  baseCurrencyCode: string;
  primaryColor: string;

  // Step 3: Administrator
  adminFirstName: string;
  adminLastName: string;
  adminFirstNameAr: string;
  adminLastNameAr: string;
  adminEmail: string;
  adminPassword: string;
  adminConfirmPassword: string;
  adminPhone: string;
}

export const INITIAL_ONBOARDING_VALUES: OnboardingFormModel = {
  institutionType: "k12_school",
  organizationName: "",
  organizationNameAr: "",
  organizationSlug: "",
  organizationCode: "",
  timezone: "Asia/Riyadh",
  locale: "ar-SA",
  dateFormat: "YYYY-MM-DD",
  isHijriEnabled: true,
  baseCurrencyCode: "SAR",
  primaryColor: "#4f46e5",
  adminFirstName: "",
  adminLastName: "",
  adminFirstNameAr: "",
  adminLastNameAr: "",
  adminEmail: "",
  adminPassword: "",
  adminConfirmPassword: "",
  adminPhone: "",
};
