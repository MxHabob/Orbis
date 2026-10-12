import type { OnboardingFormModel, OnboardingStep } from "./types";
import { z } from "zod";

export type BootstrapValidationCode =
  | "organizationNameRequired"
  | "adminFirstNameRequired"
  | "adminLastNameRequired"
  | "adminEmailRequired"
  | "adminEmailInvalid"
  | "adminPasswordRequired"
  | "adminPasswordTooShort"
  | "adminPasswordMismatch"
  | "timezoneRequired"
  | "localeRequired";

export type BootstrapValidationErrors = Partial<
  Record<keyof OnboardingFormModel, BootstrapValidationCode>
>;

const required = (code: BootstrapValidationCode) =>
  z.string().trim().min(1, code);

export const bootstrapFormSchema: z.ZodType<OnboardingFormModel> = z
  .object({
    institutionType: z.enum([
      "k12_school",
      "university",
      "vocational_institute",
      "language_center",
    ]),
    organizationName: required("organizationNameRequired"),
    organizationNameAr: z.string(),
    organizationSlug: z.string(),
    organizationCode: z.string(),
    countryName: z.string(),
    ministryName: z.string(),
    regionName: z.string(),
    educationalTrack: z.string(),
    organizationSize: z.string(),
    countryCode: z.string(),
    city: z.string(),
    phone: z.string(),
    website: z.string(),
    taxId: z.string(),
    licenseNumber: z.string(),
    timezone: required("timezoneRequired"),
    locale: required("localeRequired"),
    dateFormat: z.string(),
    formatLocale: z.string(),
    timeFormat: z.enum(["locale", "12-hour", "24-hour"]),
    isHijriEnabled: z.boolean(),
    baseCurrencyCode: z.string(),
    primaryColor: z.string(),
    adminFirstName: required("adminFirstNameRequired"),
    adminLastName: required("adminLastNameRequired"),
    adminFirstNameAr: z.string(),
    adminLastNameAr: z.string(),
    adminEmail: z
      .string()
      .trim()
      .min(1, "adminEmailRequired")
      .refine(
        (value) => !value || (value.includes("@") && value.includes(".")),
        "adminEmailInvalid",
      ),
    adminPassword: z
      .string()
      .min(1, "adminPasswordRequired")
      .refine(
        (value) => !value || new TextEncoder().encode(value).length >= 8,
        "adminPasswordTooShort",
      ),
    adminConfirmPassword: z.string(),
    adminPhone: z.string(),
  })
  .superRefine((form, context) => {
    if (form.adminPassword !== form.adminConfirmPassword) {
      context.addIssue({
        code: "custom",
        path: ["adminConfirmPassword"],
        message: "adminPasswordMismatch",
      });
    }
  });

export function normalizeBootstrapForm(
  form: OnboardingFormModel,
): OnboardingFormModel {
  return {
    ...form,
    organizationName: form.organizationName.trim(),
    organizationNameAr: form.organizationNameAr.trim(),
    organizationSlug: form.organizationSlug.trim(),
    organizationCode: form.organizationCode.trim(),
    countryName: form.countryName.trim(),
    ministryName: form.ministryName.trim(),
    regionName: form.regionName.trim(),
    city: form.city.trim(),
    phone: form.phone.trim(),
    website: form.website.trim(),
    taxId: form.taxId.trim(),
    licenseNumber: form.licenseNumber.trim(),
    timezone: form.timezone.trim(),
    locale: form.locale.trim(),
    adminFirstName: form.adminFirstName.trim(),
    adminLastName: form.adminLastName.trim(),
    adminFirstNameAr: form.adminFirstNameAr.trim(),
    adminLastNameAr: form.adminLastNameAr.trim(),
    adminEmail: form.adminEmail.trim(),
    adminPhone: form.adminPhone.trim(),
  };
}

function validationErrorsFor(
  form: OnboardingFormModel,
  fields?: string[],
): BootstrapValidationErrors {
  const result = bootstrapFormSchema.safeParse(normalizeBootstrapForm(form));
  if (result.success) return {};

  return Object.fromEntries(
    result.error.issues
      .filter((issue) => issue.path.length === 1)
      .filter((issue) => !fields || fields.includes(String(issue.path[0])))
      .map((issue) => [String(issue.path[0]), issue.message]),
  ) as BootstrapValidationErrors;
}

const STEP_FIELDS: Partial<Record<OnboardingStep, string[]>> = {
  organization: ["organizationName"],
  region: ["timezone", "locale"],
  admin: [
    "adminFirstName",
    "adminLastName",
    "adminEmail",
    "adminPassword",
    "adminConfirmPassword",
  ],
};

export function validateBootstrapStep(
  step: OnboardingStep,
  form: OnboardingFormModel,
): BootstrapValidationErrors {
  const fields = STEP_FIELDS[step];
  return fields ? validationErrorsFor(form, fields) : {};
}

export function validateBootstrapForm(
  form: OnboardingFormModel,
): BootstrapValidationErrors {
  return validationErrorsFor(form);
}
