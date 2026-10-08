import type { BootstrapRequest } from "$lib/api/types";
import type { OnboardingFormModel } from "./types";

export const BOOTSTRAP_REQUEST_FIELD_GROUPS = {
  institution: [
    "institution_type",
    "organization_name",
    "organization_name_ar",
    "organization_slug",
    "organization_code",
    "educational_track",
    "organization_size",
  ],
  locationAndPreferences: [
    "country_code",
    "city",
    "phone",
    "website",
    "tax_id",
    "license_number",
    "mulverse_node_id",
    "timezone",
    "locale",
    "date_format",
    "format_locale",
    "time_format",
    "is_hijri_enabled",
    "base_currency_code",
  ],
  branding: ["primary_color"],
  administrator: [
    "admin_first_name",
    "admin_last_name",
    "admin_first_name_ar",
    "admin_last_name_ar",
    "admin_email",
    "admin_password",
    "admin_phone",
  ],
} as const satisfies Record<string, readonly (keyof BootstrapRequest)[]>;

export const BOOTSTRAP_REQUIRED_FIELDS = [
  "institution_type",
  "organization_name",
  "timezone",
  "locale",
  "admin_first_name",
  "admin_last_name",
  "admin_email",
  "admin_password",
] as const satisfies readonly (keyof BootstrapRequest)[];

export const BOOTSTRAP_REQUEST_FIELDS = [
  "institution_type",
  "organization_name",
  "organization_name_ar",
  "organization_slug",
  "organization_code",
  "educational_track",
  "organization_size",
  "country_code",
  "city",
  "phone",
  "website",
  "tax_id",
  "license_number",
  "mulverse_node_id",
  "timezone",
  "locale",
  "date_format",
  "format_locale",
  "time_format",
  "is_hijri_enabled",
  "base_currency_code",
  "primary_color",
  "admin_first_name",
  "admin_last_name",
  "admin_first_name_ar",
  "admin_last_name_ar",
  "admin_email",
  "admin_password",
  "admin_phone",
] as const satisfies readonly (keyof BootstrapRequest)[];

function optionalText(value: string): string | undefined {
  const normalized = value.trim();
  return normalized || undefined;
}

export function buildBootstrapRequest(
  form: OnboardingFormModel,
): BootstrapRequest {
  return {
    institution_type: form.institutionType,
    organization_name: form.organizationName.trim(),
    organization_name_ar: optionalText(form.organizationNameAr),
    organization_slug: optionalText(form.organizationSlug),
    organization_code: optionalText(form.organizationCode),
    educational_track: optionalText(form.educationalTrack),
    organization_size: optionalText(form.organizationSize),
    country_code: optionalText(form.countryCode),
    city: optionalText(form.city),
    phone: optionalText(form.phone),
    website: optionalText(form.website),
    tax_id: optionalText(form.taxId),
    license_number: optionalText(form.licenseNumber),
    mulverse_node_id: undefined,
    timezone: form.timezone.trim(),
    locale: form.locale.trim(),
    date_format: optionalText(form.dateFormat),
    format_locale: optionalText(form.formatLocale),
    time_format: form.timeFormat,
    is_hijri_enabled: form.isHijriEnabled,
    base_currency_code: optionalText(form.baseCurrencyCode),
    primary_color: optionalText(form.primaryColor),
    admin_first_name: form.adminFirstName.trim(),
    admin_last_name: form.adminLastName.trim(),
    admin_first_name_ar: optionalText(form.adminFirstNameAr),
    admin_last_name_ar: optionalText(form.adminLastNameAr),
    admin_email: form.adminEmail.trim().toLowerCase(),
    admin_password: form.adminPassword,
    admin_phone: optionalText(form.adminPhone),
  };
}
