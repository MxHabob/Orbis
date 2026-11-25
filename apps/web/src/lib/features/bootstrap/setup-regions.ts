import setupRegions from "../../../../../api-rust/crates/orbis-domain/src/entities/setup-regions.json";
import type { OnboardingFormModel } from "./types";

type SetupRegion = (typeof setupRegions)[number];

export { setupRegions };
export const setupCurrencies = [...new Set(setupRegions.map((region) => region.currency)), "EUR"].sort();

export function regionName(region: SetupRegion, language: string): string {
  return language.startsWith("ar") ? region.name_ar : region.name;
}

export function applySetupRegion(form: OnboardingFormModel, code: string): void {
  const region = setupRegions.find((entry) => entry.code === code);
  if (!region) return;

  form.countryCode = region.code;
  form.timezone = region.timezone;
  form.baseCurrencyCode = region.currency;
  form.dateFormat = region.date_format;
  form.isHijriEnabled = region.hijri;
  form.formatLocale = `${new Intl.Locale(form.locale).language}-${region.code}`;
}

export function detectSetupRegion(): string | undefined {
  for (const language of navigator.languages) {
    const code = new Intl.Locale(language).region;
    if (setupRegions.some((region) => region.code === code)) return code;
  }
  return undefined;
}
