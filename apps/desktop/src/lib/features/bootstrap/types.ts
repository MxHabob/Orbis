import type { InstitutionType } from "@orbis/blueprints";
import * as m from "$lib/paraglide/messages";

type LocalizedText = () => string;

export type OnboardingStep =
  | "intro"
  | "blueprint"
  | "organization"
  | "region"
  | "admin"
  | "mfa"
  | "review";

export interface EducationalTrackOption {
  value: string;
  label: LocalizedText;
  description: LocalizedText;
  badge: LocalizedText;
}

export interface OrganizationSizeOption {
  value: string;
  label: LocalizedText;
  capacityRange: string;
}

export interface TimezoneOption {
  value: string;
  label: LocalizedText;
  offset: string;
}

export interface CurrencyOption {
  code: string;
  symbol: LocalizedText;
  name: LocalizedText;
  flag: string;
}

export interface CountryOption {
  code: string;
  name: LocalizedText;
  flag: string;
  defaultCurrency: string;
  defaultTimezone: string;
}

export interface ColorPreset {
  name: string;
  value: string;
  hex: string;
  label: LocalizedText;
  accentHex: string;
}

export const EDUCATIONAL_TRACK_OPTIONS: EducationalTrackOption[] = [
  {
    value: "national",
    label: m.onboarding_track_national_label,
    description: m.onboarding_track_national_description,
    badge: m.onboarding_track_national_badge,
  },
  {
    value: "cambridge",
    label: m.onboarding_track_cambridge_label,
    description: m.onboarding_track_cambridge_description,
    badge: m.onboarding_track_cambridge_badge,
  },
  {
    value: "american_ib",
    label: m.onboarding_track_american_ib_label,
    description: m.onboarding_track_american_ib_description,
    badge: m.onboarding_track_american_ib_badge,
  },
  {
    value: "dual",
    label: m.onboarding_track_dual_label,
    description: m.onboarding_track_dual_description,
    badge: m.onboarding_track_dual_badge,
  },
  {
    value: "general",
    label: m.onboarding_track_general_label,
    description: m.onboarding_track_general_description,
    badge: m.onboarding_track_general_badge,
  },
];

export const ORGANIZATION_SIZE_OPTIONS: OrganizationSizeOption[] = [
  {
    value: "small",
    label: m.onboarding_size_small_label,
    capacityRange: "1-250",
  },
  {
    value: "medium",
    label: m.onboarding_size_medium_label,
    capacityRange: "250-800",
  },
  {
    value: "large",
    label: m.onboarding_size_large_label,
    capacityRange: "800-2500",
  },
  {
    value: "enterprise",
    label: m.onboarding_size_enterprise_label,
    capacityRange: "2500+",
  },
];

export const COUNTRIES_LIST: CountryOption[] = [
  { code: "SA", name: m.onboarding_country_sa, flag: "🇸🇦", defaultCurrency: "SAR", defaultTimezone: "Asia/Riyadh" },
  { code: "AE", name: m.onboarding_country_ae, flag: "🇦🇪", defaultCurrency: "AED", defaultTimezone: "Asia/Dubai" },
  { code: "YE", name: m.onboarding_country_ye, flag: "🇾🇪", defaultCurrency: "YER", defaultTimezone: "Asia/Riyadh" },
  { code: "KW", name: m.onboarding_country_kw, flag: "🇰🇼", defaultCurrency: "KWD", defaultTimezone: "Asia/Kuwait" },
  { code: "QA", name: m.onboarding_country_qa, flag: "🇶🇦", defaultCurrency: "QAR", defaultTimezone: "Asia/Qatar" },
  { code: "OM", name: m.onboarding_country_om, flag: "🇴🇲", defaultCurrency: "OMR", defaultTimezone: "Asia/Muscat" },
  { code: "BH", name: m.onboarding_country_bh, flag: "🇧🇭", defaultCurrency: "BHD", defaultTimezone: "Asia/Bahrain" },
  { code: "EG", name: m.onboarding_country_eg, flag: "🇪🇬", defaultCurrency: "EGP", defaultTimezone: "Africa/Cairo" },
  { code: "JO", name: m.onboarding_country_jo, flag: "🇯🇴", defaultCurrency: "JOD", defaultTimezone: "Asia/Amman" },
  { code: "IQ", name: m.onboarding_country_iq, flag: "🇮🇶", defaultCurrency: "IQD", defaultTimezone: "Asia/Baghdad" },
  { code: "GB", name: m.onboarding_country_gb, flag: "🇬🇧", defaultCurrency: "GBP", defaultTimezone: "Europe/London" },
  { code: "US", name: m.onboarding_country_us, flag: "🇺🇸", defaultCurrency: "USD", defaultTimezone: "America/New_York" },
];

export const TIMEZONE_OPTIONS: TimezoneOption[] = [
  { value: "Asia/Riyadh", label: m.onboarding_timezone_riyadh, offset: "UTC+3" },
  { value: "Asia/Dubai", label: m.onboarding_timezone_dubai, offset: "UTC+4" },
  { value: "Asia/Kuwait", label: m.onboarding_timezone_kuwait, offset: "UTC+3" },
  { value: "Africa/Cairo", label: m.onboarding_timezone_cairo, offset: "UTC+2" },
  { value: "Asia/Amman", label: m.onboarding_timezone_amman, offset: "UTC+3" },
  { value: "Asia/Baghdad", label: m.onboarding_timezone_baghdad, offset: "UTC+3" },
  { value: "Africa/Algiers", label: m.onboarding_timezone_north_africa, offset: "UTC+1" },
  { value: "Europe/London", label: m.onboarding_timezone_london, offset: "UTC+0" },
  { value: "Europe/Paris", label: m.onboarding_timezone_central_europe, offset: "UTC+1" },
  { value: "America/New_York", label: m.onboarding_timezone_eastern_north_america, offset: "UTC-5" },
  { value: "UTC", label: m.onboarding_timezone_utc, offset: "UTC+0" },
];

export const CURRENCY_OPTIONS: CurrencyOption[] = [
  { code: "SAR", symbol: m.onboarding_currency_sar_symbol, name: m.onboarding_currency_sar_name, flag: "🇸🇦" },
  { code: "AED", symbol: m.onboarding_currency_aed_symbol, name: m.onboarding_currency_aed_name, flag: "🇦🇪" },
  { code: "YER", symbol: m.onboarding_currency_yer_symbol, name: m.onboarding_currency_yer_name, flag: "🇾🇪" },
  { code: "KWD", symbol: m.onboarding_currency_kwd_symbol, name: m.onboarding_currency_kwd_name, flag: "🇰🇼" },
  { code: "QAR", symbol: m.onboarding_currency_qar_symbol, name: m.onboarding_currency_qar_name, flag: "🇶🇦" },
  { code: "BHD", symbol: m.onboarding_currency_bhd_symbol, name: m.onboarding_currency_bhd_name, flag: "🇧🇭" },
  { code: "OMR", symbol: m.onboarding_currency_omr_symbol, name: m.onboarding_currency_omr_name, flag: "🇴🇲" },
  { code: "EGP", symbol: m.onboarding_currency_egp_symbol, name: m.onboarding_currency_egp_name, flag: "🇪🇬" },
  { code: "JOD", symbol: m.onboarding_currency_jod_symbol, name: m.onboarding_currency_jod_name, flag: "🇯🇴" },
  { code: "USD", symbol: m.onboarding_currency_usd_symbol, name: m.onboarding_currency_usd_name, flag: "🇺🇸" },
  { code: "EUR", symbol: m.onboarding_currency_eur_symbol, name: m.onboarding_currency_eur_name, flag: "🇪🇺" },
  { code: "GBP", symbol: m.onboarding_currency_gbp_symbol, name: m.onboarding_currency_gbp_name, flag: "🇬🇧" },
];

export const COLOR_PRESETS: ColorPreset[] = [
  { name: "indigo", value: "#4f46e5", hex: "#4f46e5", accentHex: "#818cf8", label: m.onboarding_color_indigo },
  { name: "blue", value: "#2563eb", hex: "#2563eb", accentHex: "#60a5fa", label: m.onboarding_color_blue },
  { name: "emerald", value: "#059669", hex: "#059669", accentHex: "#34d399", label: m.onboarding_color_emerald },
  { name: "teal", value: "#0d9488", hex: "#0d9488", accentHex: "#2dd4bf", label: m.onboarding_color_teal },
  { name: "amber", value: "#d97706", hex: "#d97706", accentHex: "#fbbf24", label: m.onboarding_color_amber },
  { name: "rose", value: "#e11d48", hex: "#e11d48", accentHex: "#fb7185", label: m.onboarding_color_rose },
  { name: "purple", value: "#9333ea", hex: "#9333ea", accentHex: "#c084fc", label: m.onboarding_color_purple },
];

export interface OnboardingFormModel {
  // Step 1: Application introduction
  institutionType: InstitutionType;

  // Step 2: Organization
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
  formatLocale: string;
  timeFormat: "locale" | "12-hour" | "24-hour";
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
  formatLocale: "ar-SA",
  timeFormat: "locale",
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
