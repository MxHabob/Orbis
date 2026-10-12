import { describe, expect, it } from "bun:test";
import {
  BOOTSTRAP_REQUEST_FIELD_GROUPS,
  BOOTSTRAP_REQUEST_FIELDS,
  BOOTSTRAP_REQUIRED_FIELDS,
  buildBootstrapRequest,
} from "./bootstrap-request";

const EMPTY_FORM = {
  institutionType: "k12_school",
  organizationName: "",
  organizationNameAr: "",
  organizationSlug: "",
  organizationCode: "",
  countryName: "",
  ministryName: "",
  regionName: "",
  educationalTrack: "",
  organizationSize: "",
  countryCode: "",
  city: "",
  phone: "",
  website: "",
  taxId: "",
  licenseNumber: "",
  timezone: "",
  locale: "",
  dateFormat: "",
  formatLocale: "",
  timeFormat: "locale" as const,
  isHijriEnabled: false,
  baseCurrencyCode: "",
  primaryColor: "",
  adminFirstName: "",
  adminLastName: "",
  adminFirstNameAr: "",
  adminLastNameAr: "",
  adminEmail: "",
  adminPassword: "",
  adminConfirmPassword: "",
  adminPhone: "",
};

describe("Bootstrap request contract", () => {
  it("accounts for each backend request field exactly once", () => {
    const groupedFields = Object.values(BOOTSTRAP_REQUEST_FIELD_GROUPS).flat();

    expect(groupedFields).toHaveLength(BOOTSTRAP_REQUEST_FIELDS.length);
    expect(new Set(groupedFields).size).toBe(groupedFields.length);
    expect(new Set(groupedFields)).toEqual(new Set(BOOTSTRAP_REQUEST_FIELDS));
  });

  it("maps required and optional values without promoting optional fields", () => {
    const request = buildBootstrapRequest({
      ...EMPTY_FORM,
      organizationName: "  Orbis Academy  ",
      adminFirstName: "  Sam  ",
      adminLastName: "  Lee  ",
      adminEmail: " ADMIN@EXAMPLE.COM ",
    });

    expect(request.organization_name).toBe("Orbis Academy");
    expect(request.admin_first_name).toBe("Sam");
    expect(request.admin_last_name).toBe("Lee");
    expect(request.admin_email).toBe("admin@example.com");
    expect(request.organization_name_ar).toBeUndefined();
    expect(request.admin_phone).toBeUndefined();
    expect(request.organization_slug).toBeUndefined();
  });

  it("preserves the server-supported regional formats and secrets", () => {
    const request = buildBootstrapRequest({
      ...EMPTY_FORM,
      formatLocale: "en-GB",
      timeFormat: "24-hour",
      adminPassword: "  Strong-Pass-42  ",
    });

    expect(request.format_locale).toBe("en-GB");
    expect(request.time_format).toBe("24-hour");
    expect(request.admin_password).toBe("  Strong-Pass-42  ");
    expect(BOOTSTRAP_REQUIRED_FIELDS).toContain("admin_password");
  });

  it("maps optional organization and administrator contact fields", () => {
    const request = buildBootstrapRequest({
      ...EMPTY_FORM,
      phone: " +966500000000 ",
      website: " https://orbis.example ",
      taxId: " VAT-123 ",
      licenseNumber: " LIC-456 ",
      countryName: " Republic of Example ",
      ministryName: " Ministry of Education ",
      regionName: " Northern Region ",
      adminPhone: " +966511111111 ",
    });

    expect(request.phone).toBe("+966500000000");
    expect(request.website).toBe("https://orbis.example");
    expect(request.tax_id).toBe("VAT-123");
    expect(request.license_number).toBe("LIC-456");
    expect(request.country_name).toBe("Republic of Example");
    expect(request.ministry_name).toBe("Ministry of Education");
    expect(request.region_name).toBe("Northern Region");
    expect(request.admin_phone).toBe("+966511111111");
  });
});
