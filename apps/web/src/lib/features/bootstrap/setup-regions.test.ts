import { expect, test } from "bun:test";
import { applySetupRegion } from "./setup-regions";
import type { OnboardingFormModel } from "./types";

test("switching country replaces regional defaults while preserving language and identity", () => {
  const form = {
    locale: "ar-SA",
    organizationName: "Orbis School",
  } as OnboardingFormModel;

  applySetupRegion(form, "YE");
  expect(form).toMatchObject({
    countryCode: "YE",
    timezone: "Asia/Aden",
    baseCurrencyCode: "YER",
    isHijriEnabled: true,
  });

  applySetupRegion(form, "US");
  expect(form).toMatchObject({
    countryCode: "US",
    timezone: "America/New_York",
    baseCurrencyCode: "USD",
    dateFormat: "MM/DD/YYYY",
    formatLocale: "ar-US",
    isHijriEnabled: false,
    locale: "ar-SA",
    organizationName: "Orbis School",
  });
});
