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

export const COLOR_PRESETS: ColorPreset[] = [
  {
    name: "indigo",
    value: "#4f46e5",
    hex: "#4f46e5",
    accentHex: "#818cf8",
    label: m.onboarding_color_indigo,
  },
  {
    name: "blue",
    value: "#2563eb",
    hex: "#2563eb",
    accentHex: "#60a5fa",
    label: m.onboarding_color_blue,
  },
  {
    name: "emerald",
    value: "#059669",
    hex: "#059669",
    accentHex: "#34d399",
    label: m.onboarding_color_emerald,
  },
  {
    name: "teal",
    value: "#0d9488",
    hex: "#0d9488",
    accentHex: "#2dd4bf",
    label: m.onboarding_color_teal,
  },
  {
    name: "amber",
    value: "#d97706",
    hex: "#d97706",
    accentHex: "#fbbf24",
    label: m.onboarding_color_amber,
  },
  {
    name: "rose",
    value: "#e11d48",
    hex: "#e11d48",
    accentHex: "#fb7185",
    label: m.onboarding_color_rose,
  },
  {
    name: "purple",
    value: "#9333ea",
    hex: "#9333ea",
    accentHex: "#c084fc",
    label: m.onboarding_color_purple,
  },
];

export interface OnboardingFormModel {
  // Step 1: Application introduction
  institutionType: InstitutionType;

  // Step 2: Organization
  organizationName: string;
  organizationNameAr: string;
  organizationSlug: string;
  organizationCode: string;
  countryName: string;
  ministryName: string;
  regionName: string;
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

  // Step 4: Administrator security
  enableMfa: boolean;
}

export const INITIAL_ONBOARDING_VALUES: OnboardingFormModel = {
  institutionType: "k12_school",
  organizationName: "",
  organizationNameAr: "",
  organizationSlug: "",
  organizationCode: "",
  countryName: "",
  ministryName: "",
  regionName: "",
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
  enableMfa: true,
};
