import { describe, expect, test } from "bun:test";
import { INITIAL_ONBOARDING_VALUES } from "./types";
import { validateBootstrapForm, validateBootstrapStep } from "./bootstrap-validation";

describe("Bootstrap validation", () => {
  test("requires the same core text values as the backend without requiring Arabic names", () => {
    const errors = validateBootstrapForm({
      ...INITIAL_ONBOARDING_VALUES,
      organizationName: " ",
      organizationNameAr: "",
      adminFirstName: " ",
      adminLastName: "",
      adminEmail: "",
      adminPassword: "",
      adminConfirmPassword: "",
      timezone: " ",
      locale: "",
    });

    expect(errors).toEqual({
      organizationName: "organizationNameRequired",
      timezone: "timezoneRequired",
      locale: "localeRequired",
      adminFirstName: "adminFirstNameRequired",
      adminLastName: "adminLastNameRequired",
      adminEmail: "adminEmailRequired",
      adminPassword: "adminPasswordRequired",
    });
  });

  test("uses the backend's basic email rule and UTF-8 password length", () => {
    const errors = validateBootstrapStep("admin", {
      ...INITIAL_ONBOARDING_VALUES,
      adminFirstName: "Ada",
      adminLastName: "Lovelace",
      adminEmail: "invalid",
      adminPassword: "éééé",
      adminConfirmPassword: "éééé",
    });

    expect(errors.adminEmail).toBe("adminEmailInvalid");
    expect(errors.adminPassword).toBeUndefined();
  });

  test("rejects password values shorter than eight backend bytes and mismatches", () => {
    const errors = validateBootstrapStep("admin", {
      ...INITIAL_ONBOARDING_VALUES,
      adminFirstName: "Ada",
      adminLastName: "Lovelace",
      adminEmail: "ada@example.com",
      adminPassword: "short",
      adminConfirmPassword: "different",
    });

    expect(errors.adminPassword).toBe("adminPasswordTooShort");
    expect(errors.adminConfirmPassword).toBe("adminPasswordMismatch");
  });

  test("does not treat optional Arabic organization name as required", () => {
    const errors = validateBootstrapStep("organization", {
      ...INITIAL_ONBOARDING_VALUES,
      organizationName: "Orbis School",
      organizationNameAr: "",
    });
    expect(errors).toEqual({});
  });

  test("does not validate unrelated fields while leaving the introduction step", () => {
    expect(
      validateBootstrapStep("intro", INITIAL_ONBOARDING_VALUES),
    ).toEqual({});
  });

  test("requires server-mandated regional values but permits optional locale details", () => {
    const errors = validateBootstrapStep("region", {
      ...INITIAL_ONBOARDING_VALUES,
      timezone: "",
      locale: "",
      countryCode: "",
      city: "",
      phone: "",
      website: "",
      taxId: "",
      licenseNumber: "",
    });

    expect(errors).toEqual({
      timezone: "timezoneRequired",
      locale: "localeRequired",
    });
  });
});
