import type { InstitutionType } from "@orbis/blueprints";

export type OnboardingStep = "mulverse" | "blueprint" | "organization" | "admin" | "mfa" | "review";

export interface EducationalTrackOption {
  value: string;
  labelEn: string;
  labelAr: string;
  descriptionEn: string;
  descriptionAr: string;
  badge: string;
}

export interface OrganizationSizeOption {
  value: string;
  labelEn: string;
  labelAr: string;
  capacityRange: string;
}

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
  flag: string;
}

export interface CountryOption {
  code: string;
  nameEn: string;
  nameAr: string;
  flag: string;
  defaultCurrency: string;
  defaultTimezone: string;
}

export interface ColorPreset {
  name: string;
  value: string;
  hex: string;
  labelEn: string;
  labelAr: string;
  accentHex: string;
}

export const EDUCATIONAL_TRACK_OPTIONS: EducationalTrackOption[] = [
  {
    value: "national",
    labelEn: "National & Ministerial Curriculum",
    labelAr: "المنهج الوطني والوزاري المعتمد",
    descriptionEn: "Standard ministry-approved academic subjects, grading schemes, and term calendars.",
    descriptionAr: "المقررات الوزارية الرسمية، لوائح التقويم المعتمدة، والفترات الدراسية القياسية.",
    badge: "National Standard",
  },
  {
    value: "cambridge",
    labelEn: "British & Cambridge International",
    labelAr: "المنهج البريطاني والدولي (Cambridge / IGCSE)",
    descriptionEn: "Cambridge Primary, Lower Secondary, IGCSE, and AS/A Levels framework.",
    descriptionAr: "مراحل كامبردج الأساسية، المتوسطة، شهادة IGCSE، ومستويات A-Level الدولية.",
    badge: "Cambridge / IGCSE",
  },
  {
    value: "american_ib",
    labelEn: "American Curriculum & IB Diploma",
    labelAr: "المنهج الأمريكي ودبلوم البكالوريا الدولية (IB / AP)",
    descriptionEn: "US Common Core standards, AP courses, GPA credit system, and IB DP integration.",
    descriptionAr: "معايير Common Core، مقررات AP، نظام الساعات والمعدل GPA، ودبلوم البكالوريا.",
    badge: "American / IB",
  },
  {
    value: "dual",
    labelEn: "Dual Track (National & International)",
    labelAr: "المسار المزدوج (وطني + دولي موازي)",
    descriptionEn: "Seamless coexistence of ministerial requirements alongside international programs.",
    descriptionAr: "تطبيق المسار الوطني جنباً إلى جنب مع البرامج والمقررات الدولية الموازية.",
    badge: "Dual Track",
  },
  {
    value: "general",
    labelEn: "Vocational & Specialized Academy",
    labelAr: "المسار المهني والتطبيقي التخصصي",
    descriptionEn: "Skills-based certification tracks, training cohorts, and competency matrices.",
    descriptionAr: "المسارات المهنية، مصفوفات الكفايات، والدورات التدريبية المعتمدة للشهادات.",
    badge: "Professional Track",
  },
];

export const ORGANIZATION_SIZE_OPTIONS: OrganizationSizeOption[] = [
  {
    value: "small",
    labelEn: "Compact / Boutique (1 – 250 Students)",
    labelAr: "منشأة ناشئة / محدودة (١ - ٢٥٠ طالب)",
    capacityRange: "1-250",
  },
  {
    value: "medium",
    labelEn: "Standard Campus (250 – 800 Students)",
    labelAr: "مجمع نموذجي متوسط (٢٥٠ - ٨٠٠ طالب)",
    capacityRange: "250-800",
  },
  {
    value: "large",
    labelEn: "Major Educational Complex (800 – 2,500 Students)",
    labelAr: "مجمع تعليمي كبير (٨٠٠ - ٢,٥٠٠ طالب)",
    capacityRange: "800-2500",
  },
  {
    value: "enterprise",
    labelEn: "Multi-Branch Enterprise (2,500+ Students)",
    labelAr: "مجموعة وفروع متعددة (أكثر من ٢,٥٠٠ طالب)",
    capacityRange: "2500+",
  },
];

export const COUNTRIES_LIST: CountryOption[] = [
  { code: "SA", nameEn: "Saudi Arabia", nameAr: "المملكة العربية السعودية", flag: "🇸🇦", defaultCurrency: "SAR", defaultTimezone: "Asia/Riyadh" },
  { code: "AE", nameEn: "United Arab Emirates", nameAr: "الإمارات العربية المتحدة", flag: "🇦🇪", defaultCurrency: "AED", defaultTimezone: "Asia/Dubai" },
  { code: "YE", nameEn: "Yemen", nameAr: "اليمن", flag: "🇾🇪", defaultCurrency: "YER", defaultTimezone: "Asia/Riyadh" },
  { code: "KW", nameEn: "Kuwait", nameAr: "الكويت", flag: "🇰🇼", defaultCurrency: "KWD", defaultTimezone: "Asia/Kuwait" },
  { code: "QA", nameEn: "Qatar", nameAr: "قطر", flag: "🇶🇦", defaultCurrency: "QAR", defaultTimezone: "Asia/Qatar" },
  { code: "OM", nameEn: "Oman", nameAr: "سلطنة عمان", flag: "🇴🇲", defaultCurrency: "OMR", defaultTimezone: "Asia/Muscat" },
  { code: "BH", nameEn: "Bahrain", nameAr: "مملكة البحرين", flag: "🇧🇭", defaultCurrency: "BHD", defaultTimezone: "Asia/Bahrain" },
  { code: "EG", nameEn: "Egypt", nameAr: "جمهورية مصر العربية", flag: "🇪🇬", defaultCurrency: "EGP", defaultTimezone: "Africa/Cairo" },
  { code: "JO", nameEn: "Jordan", nameAr: "المملكة الأردنية الهاشمية", flag: "🇯🇴", defaultCurrency: "JOD", defaultTimezone: "Asia/Amman" },
  { code: "IQ", nameEn: "Iraq", nameAr: "جمهورية العراق", flag: "🇮🇶", defaultCurrency: "IQD", defaultTimezone: "Asia/Baghdad" },
  { code: "GB", nameEn: "United Kingdom", nameAr: "المملكة المتحدة", flag: "🇬🇧", defaultCurrency: "GBP", defaultTimezone: "Europe/London" },
  { code: "US", nameEn: "United States", nameAr: "الولايات المتحدة الأمريكية", flag: "🇺🇸", defaultCurrency: "USD", defaultTimezone: "America/New_York" },
];

export const TIMEZONE_OPTIONS: TimezoneOption[] = [
  { value: "Asia/Riyadh", labelEn: "Riyadh (KSA / GCC)", labelAr: "الرياض (السعودية والخليج)", offset: "UTC+3" },
  { value: "Asia/Dubai", labelEn: "Dubai (UAE / Oman)", labelAr: "دبي (الإمارات وعمان)", offset: "UTC+4" },
  { value: "Asia/Kuwait", labelEn: "Kuwait / Qatar / Bahrain", labelAr: "الكويت / قطر / البحرين", offset: "UTC+3" },
  { value: "Africa/Cairo", labelEn: "Cairo (Egypt)", labelAr: "القاهرة (مصر)", offset: "UTC+2" },
  { value: "Asia/Amman", labelEn: "Amman (Jordan)", labelAr: "عمّان (الأردن)", offset: "UTC+3" },
  { value: "Asia/Baghdad", labelEn: "Baghdad (Iraq)", labelAr: "بغداد (العراق)", offset: "UTC+3" },
  { value: "Africa/Algiers", labelEn: "Algiers / Tunis / Casablanca", labelAr: "الجزائر / تونس / الدار البيضاء", offset: "UTC+1" },
  { value: "Europe/London", labelEn: "London (GMT / BST)", labelAr: "لندن (المملكة المتحدة)", offset: "UTC+0" },
  { value: "Europe/Paris", labelEn: "Paris / Berlin / Rome", labelAr: "باريس / برلين / روما", offset: "UTC+1" },
  { value: "America/New_York", labelEn: "Eastern Time (US & Canada)", labelAr: "التوقيت الشرقي (أمريكا)", offset: "UTC-5" },
  { value: "UTC", labelEn: "Coordinated Universal Time (UTC)", labelAr: "التوقيت العالمي الموحد (UTC)", offset: "UTC+0" },
];

export const CURRENCY_OPTIONS: CurrencyOption[] = [
  { code: "SAR", symbolEn: "SAR", symbolAr: "ر.س", nameEn: "Saudi Riyal", nameAr: "ريال سعودي", flag: "🇸🇦" },
  { code: "AED", symbolEn: "AED", symbolAr: "د.إ", nameEn: "UAE Dirham", nameAr: "درهم إماراتي", flag: "🇦🇪" },
  { code: "YER", symbolEn: "YER", symbolAr: "ر.ي", nameEn: "Yemeni Rial", nameAr: "ريال يمني", flag: "🇾🇪" },
  { code: "KWD", symbolEn: "KWD", symbolAr: "د.ك", nameEn: "Kuwaiti Dinar", nameAr: "دينار كويتي", flag: "🇰🇼" },
  { code: "QAR", symbolEn: "QAR", symbolAr: "ر.ق", nameEn: "Qatari Riyal", nameAr: "ريال قطري", flag: "🇶🇦" },
  { code: "BHD", symbolEn: "BHD", symbolAr: "د.ب", nameEn: "Bahraini Dinar", nameAr: "دينار بحريني", flag: "🇧🇭" },
  { code: "OMR", symbolEn: "OMR", symbolAr: "ر.ع", nameEn: "Omani Rial", nameAr: "ريال عماني", flag: "🇴🇲" },
  { code: "EGP", symbolEn: "EGP", symbolAr: "ج.م", nameEn: "Egyptian Pound", nameAr: "جنيه مصري", flag: "🇪🇬" },
  { code: "JOD", symbolEn: "JOD", symbolAr: "د.أ", nameEn: "Jordanian Dinar", nameAr: "دينار أردني", flag: "🇯🇴" },
  { code: "USD", symbolEn: "USD ($)", symbolAr: "دولار ($)", nameEn: "US Dollar", nameAr: "دولار أمريكي", flag: "🇺🇸" },
  { code: "EUR", symbolEn: "EUR (€)", symbolAr: "يورو (€)", nameEn: "Euro", nameAr: "يورو أوروبي", flag: "🇪🇺" },
  { code: "GBP", symbolEn: "GBP (£)", symbolAr: "جنيه (£)", nameEn: "British Pound", nameAr: "جنيه إسترليني", flag: "🇬🇧" },
];

export const COLOR_PRESETS: ColorPreset[] = [
  { name: "indigo", value: "#4f46e5", hex: "#4f46e5", accentHex: "#818cf8", labelEn: "Royal Indigo", labelAr: "نيلي ملكي" },
  { name: "blue", value: "#2563eb", hex: "#2563eb", accentHex: "#60a5fa", labelEn: "Ocean Blue", labelAr: "أزرق محيطي" },
  { name: "emerald", value: "#059669", hex: "#059669", accentHex: "#34d399", labelEn: "Forest Emerald", labelAr: "أخضر زمردي" },
  { name: "teal", value: "#0d9488", hex: "#0d9488", accentHex: "#2dd4bf", labelEn: "Cyber Teal", labelAr: "سماوي بحري" },
  { name: "amber", value: "#d97706", hex: "#d97706", accentHex: "#fbbf24", labelEn: "Imperial Amber", labelAr: "كهرماني إمبراطوري" },
  { name: "rose", value: "#e11d48", hex: "#e11d48", accentHex: "#fb7185", labelEn: "Crimson Rose", labelAr: "وردي قرمزي" },
  { name: "purple", value: "#9333ea", hex: "#9333ea", accentHex: "#c084fc", labelEn: "Noble Violet", labelAr: "بنفسجي نبيل" },
];

export interface OnboardingFormModel {
  // Step 1: Mulverse Ecosystem & Node
  mulverseNodeId: string;

  // Step 2: Blueprint
  institutionType: InstitutionType;

  // Step 3: Enriched Organization
  organizationName: string;
  organizationNameAr: string;
  organizationSlug: string;
  organizationCode: string;
  educationalTrack: string;
  organizationSize: string;
  countryCode: string;
  city: string;
  phone: string;
  website: string;
  taxId: string;
  licenseNumber: string;
  timezone: string;
  locale: string;
  dateFormat: string;
  isHijriEnabled: boolean;
  baseCurrencyCode: string;
  primaryColor: string;

  // Step 4: Administrator
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
  mulverseNodeId: "",
  institutionType: "k12_school",
  organizationName: "",
  organizationNameAr: "",
  organizationSlug: "",
  organizationCode: "",
  educationalTrack: "national",
  organizationSize: "medium",
  countryCode: "SA",
  city: "",
  phone: "",
  website: "",
  taxId: "",
  licenseNumber: "",
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
