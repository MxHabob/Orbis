<script lang="ts">
import * as m from "$lib/paraglide/messages";
import InfoSettingsCard from "./info-settings-card.svelte";
import BrandingSettingsCard from "./branding-settings-card.svelte";
import RegionalSettingsCard from "./regional-settings-card.svelte";
import AvatarSettingsCard from "./avatar-settings-card.svelte";
import DangerZoneSettingsCard from "./danger-zone-settings-card.svelte";
import BranchSettingsCard from "./branch-settings-card.svelte";
import OrganizationProfileSettingsCard from "./organization-profile-settings-card.svelte";
import { Card, CardHeader, CardTitle, CardContent } from "@orbis/ui/card";
import SettingsBottomBar from "$lib/shared/components/settings-bottom-bar.svelte";
import { toast } from "@orbis/ui";

import { api, type OrganizationSettingsDto } from "$lib/api";
import { generalSettingsInputSchema } from "@orbis/schemas";
import { FormController } from "$lib/shared/forms/form-controller.svelte";
import { systemStore } from "$lib/stores";
import { formatLicensePlan } from "$lib/utils/license";

let { org, orgId, isSaving, onsave, onsaveavatar, onremove, onlanguagechange } =
  $props<{
    org: any;
    orgId: string;
    isSaving: boolean;
    onsave: (request: any) => void | Promise<void>;
    onsaveavatar: (logo: string | null) => void;
    onremove: () => void;
    onlanguagechange: (newLocale: string, currentValues: any) => void | boolean | Promise<void | boolean>;
  }>();

// Org Profile Form state
let name = $state("");
let nameAr = $state("");
let slug = $state("");
let code = $state("");
let logo = $state("");
let favicon = $state("");
let primaryColor = $state("#84cc16");
let currency = $state("YER");
let timezone = $state("Asia/Aden");
let locale = $state("ar");
let dateFormat = $state("YYYY-MM-DD");
let formatLocale = $state("auto");
let timeFormat = $state("locale");
let hijriEnabled = $state(false);
let institutionType = $state("school");
let educationalTrack = $state("");
let organizationSize = $state("");
let countryCode = $state("");
let countryName = $state("");
let ministryName = $state("");
let regionName = $state("");
let city = $state("");
let phone = $state("");
let website = $state("");
let taxId = $state("");
let licenseNumber = $state("");

function entitlementStatusLabel(status?: string): string {
  switch (status) {
    case "active": return m.license_status_active();
    case "trial": return m.license_status_trial();
    case "pending": return m.license_status_pending();
    case "grace_period": return m.license_status_offline_grace();
    case "expired": return m.license_status_expired();
    case "suspended": return m.license_status_suspended();
    case "unlicensed": return m.license_status_unavailable();
    default: return m.license_status_unavailable();
  }
}

// Institutional Policy Settings State
let rawSettings = $state<OrganizationSettingsDto | null>(null);

let initialValues = $state<any>(null);
const settingsForm = new FormController({
  initialValues: {
    name: "",
    currency: "YER",
    timezone: "Asia/Aden",
    locale: "ar",
    dateFormat: "YYYY-MM-DD",
    formatLocale: "auto",
    timeFormat: "locale",
  },
  schema: generalSettingsInputSchema,
});

async function loadSettings() {
  try {
    const s = await api.getSettings();
    // Settings loaded
  } catch {
    // Non-fatal if settings fail to load initially
  }
}

$effect(() => {
  if (org && !initialValues) {
    name = org.name || "";
    nameAr = org.name_ar || "";
    slug = org.slug || "";
    code = org.code || "";
    logo = org.logo || "";
    favicon = org.favicon || "";
    primaryColor = org.primary_color || "#84cc16";
    currency = org.base_currency_code || org.currency || "YER";
    timezone = org.timezone || "Asia/Aden";
    locale = org.locale || "ar";
    dateFormat = org.date_format || "YYYY-MM-DD";
    formatLocale = org.format_locale || "auto";
    timeFormat = org.time_format || "locale";
    hijriEnabled = org.is_hijri_enabled !== undefined ? Boolean(org.is_hijri_enabled) : (org.hijri_enabled === true || org.hijri_enabled === 1);
    const metadata =
      org.metadata && typeof org.metadata === "object" && !Array.isArray(org.metadata)
        ? org.metadata
        : {};
    institutionType = typeof metadata.institution_type === "string" ? metadata.institution_type : org.institution_type || "school";
    educationalTrack = typeof metadata.educational_track === "string" ? metadata.educational_track : "";
    organizationSize = typeof metadata.organization_size === "string" ? metadata.organization_size : "";
    countryCode = typeof metadata.country_code === "string" ? metadata.country_code : "";
    countryName = typeof metadata.country_name === "string" ? metadata.country_name : "";
    ministryName = typeof metadata.ministry_name === "string" ? metadata.ministry_name : "";
    regionName = typeof metadata.region_name === "string" ? metadata.region_name : "";
    city = typeof metadata.city === "string" ? metadata.city : "";
    phone = typeof metadata.phone === "string" ? metadata.phone : "";
    website = typeof metadata.website === "string" ? metadata.website : "";
    taxId = typeof metadata.tax_id === "string" ? metadata.tax_id : "";
    licenseNumber = typeof metadata.license_number === "string" ? metadata.license_number : "";

    loadSettings().finally(() => {
      initialValues = {
        name,
        nameAr,
        code,
        primaryColor,
        favicon,
        currency,
        timezone,
        locale,
        dateFormat,
        formatLocale,
        timeFormat,
        hijriEnabled,
        institutionType,
        educationalTrack,
        organizationSize,
        countryCode,
        countryName,
        ministryName,
        regionName,
        city,
        phone,
        website,
        taxId,
        licenseNumber,
      };
    });
    systemStore.loadEntitlement();
  }
});

const isDirty = $derived.by(() => {
  if (!initialValues) return false;
  return (
    name !== initialValues.name ||
    nameAr !== initialValues.nameAr ||
    code !== initialValues.code ||
    primaryColor !== initialValues.primaryColor ||
    favicon !== initialValues.favicon ||
    currency !== initialValues.currency ||
    timezone !== initialValues.timezone ||
    locale !== initialValues.locale ||
    dateFormat !== initialValues.dateFormat ||
    formatLocale !== initialValues.formatLocale ||
    timeFormat !== initialValues.timeFormat ||
    hijriEnabled !== initialValues.hijriEnabled ||
    institutionType !== initialValues.institutionType ||
    educationalTrack !== initialValues.educationalTrack ||
    organizationSize !== initialValues.organizationSize ||
    countryCode !== initialValues.countryCode ||
    countryName !== initialValues.countryName ||
    ministryName !== initialValues.ministryName ||
    regionName !== initialValues.regionName ||
    city !== initialValues.city ||
    phone !== initialValues.phone ||
    website !== initialValues.website ||
    taxId !== initialValues.taxId ||
    licenseNumber !== initialValues.licenseNumber
  );
});

function handleReset() {
  if (!initialValues) return;
  name = initialValues.name;
  nameAr = initialValues.nameAr;
  code = initialValues.code;
  primaryColor = initialValues.primaryColor;
  favicon = initialValues.favicon;
  currency = initialValues.currency;
  timezone = initialValues.timezone;
  locale = initialValues.locale;
  dateFormat = initialValues.dateFormat;
  formatLocale = initialValues.formatLocale;
  timeFormat = initialValues.timeFormat;
  hijriEnabled = initialValues.hijriEnabled;
  institutionType = initialValues.institutionType;
  educationalTrack = initialValues.educationalTrack;
  organizationSize = initialValues.organizationSize;
  countryCode = initialValues.countryCode;
  countryName = initialValues.countryName;
  ministryName = initialValues.ministryName;
  regionName = initialValues.regionName;
  city = initialValues.city;
  phone = initialValues.phone;
  website = initialValues.website;
  taxId = initialValues.taxId;
  licenseNumber = initialValues.licenseNumber;
}

function getProfileMetadata() {
  return {
    institution_type: institutionType,
    educational_track: educationalTrack.trim() || null,
    organization_size: organizationSize.trim() || null,
    country_code: countryCode.trim() || null,
    country_name: countryName.trim() || null,
    ministry_name: ministryName.trim() || null,
    region_name: regionName.trim() || null,
    city: city.trim() || null,
    phone: phone.trim() || null,
    website: website.trim() || null,
    tax_id: taxId.trim() || null,
    license_number: licenseNumber.trim() || null,
  };
}

async function handleSubmit(e?: Event) {
  e?.preventDefault();
  settingsForm.values = { name, currency, timezone, locale, dateFormat, formatLocale, timeFormat };
  const submitted = await settingsForm.submit(async (result) => {
    await onsave({
      name: result.name,
      name_ar: nameAr || null,
      code: code.trim(),
      logo: logo || null,
      favicon: favicon.trim(),
      primary_color: primaryColor,
      base_currency_code: result.currency,
      timezone: result.timezone,
      locale: result.locale,
      date_format: result.dateFormat,
      format_locale: result.formatLocale,
      time_format: result.timeFormat,
      is_hijri_enabled: hijriEnabled,
      metadata: getProfileMetadata(),
    });

    if (rawSettings) {
      try {
        const updated = await api.updateSettings({
          ...rawSettings,
          enrollment: rawSettings.enrollment,
          finance: rawSettings.finance,
          autopilot: rawSettings.autopilot,
        });
        rawSettings = updated;
      } catch (err) {
        toast.error(m.ui_copy_failed_to_save_institutional_policy_settin_6ea63580());
      }
    }
  });

  if (submitted) {
    initialValues = {
      name,
      nameAr,
      primaryColor,
      currency,
      timezone,
      locale,
      dateFormat,
      formatLocale,
      timeFormat,
      hijriEnabled,
      institutionType,
      educationalTrack,
      organizationSize,
      countryCode,
      countryName,
      ministryName,
      regionName,
      city,
      phone,
      website,
      taxId,
      licenseNumber,
    };
  } else {
    toast.error(
      m.ui_copy_settings_contain_invalid_values_fae2c24c(),
    );
  }
}

async function handleLanguageChange(newLocale: string) {
  return await onlanguagechange(newLocale, {
    name,
    name_ar: nameAr || null,
    code: code.trim(),
    logo: logo || null,
    favicon: favicon.trim(),
    primary_color: primaryColor,
    currency,
    timezone,
    locale: newLocale,
    date_format: dateFormat,
    format_locale: formatLocale,
    time_format: timeFormat,
    is_hijri_enabled: hijriEnabled,
    metadata: getProfileMetadata(),
  });
}

function handleSaveAvatar(e: Event) {
  e.preventDefault();
  onsaveavatar(logo || null);
}
</script>

<div class="h-full w-full overflow-y-auto pb-28 scrollbar-hide">
  <div class="max-w-7xl mx-auto p-6 flex flex-col gap-6">
    <form onsubmit={handleSubmit} class="flex flex-col gap-6">
      <InfoSettingsCard
        {slug}
        bind:code
        bind:name
        bind:nameAr
        organizationId={orgId}
        {isSaving}
      />

      <BrandingSettingsCard
        bind:primaryColor
        bind:favicon
        {isSaving}
      />

      <OrganizationProfileSettingsCard
        bind:institutionType
        bind:educationalTrack
        bind:organizationSize
        bind:countryCode
        bind:countryName
        bind:ministryName
        bind:regionName
        bind:city
        bind:phone
        bind:website
        bind:taxId
        bind:licenseNumber
        {isSaving}
      />

      <RegionalSettingsCard
        bind:currency
        bind:timezone
        bind:locale
        bind:dateFormat
        bind:formatLocale
        bind:timeFormat
        bind:hijriEnabled
        {isSaving}
        onLanguageChange={handleLanguageChange}
      />

      <Card>
        <CardHeader class="flex flex-row items-center justify-between">
          <CardTitle>
            {m.settings_entitlement_title()}
          </CardTitle>
          <a
            href="/{orgId}/billing"
            class="text-xs font-semibold text-primary hover:underline flex items-center gap-1"
          >
            {m.license_nav_title()} &rarr;
          </a>
        </CardHeader>
        <CardContent class="p-0">
          <div class="grid grid-cols-1 md:grid-cols-3 divide-y md:divide-y-0 md:divide-x rtl:md:divide-x-reverse divide-border">
            <div class="px-6 py-5 flex flex-col gap-1 text-start">
              <span class="text-xs font-semibold text-foreground">{m.settings_entitlement_users()}</span>
              <span class="text-2xl font-semibold tabular-nums">{systemStore.entitlement?.max_users ?? m.settings_entitlement_unlimited()}</span>
              <span class="text-[11px] text-muted-foreground">{m.settings_entitlement_max_allowed()}</span>
            </div>
            <div class="px-6 py-5 flex flex-col gap-1 text-start">
              <span class="text-xs font-semibold text-foreground">{m.settings_entitlement_students()}</span>
              <span class="text-2xl font-semibold tabular-nums">{systemStore.entitlement?.max_students ?? m.settings_entitlement_unlimited()}</span>
              <span class="text-[11px] text-muted-foreground">{m.settings_entitlement_max_allowed()}</span>
            </div>
            <div class="px-6 py-5 flex flex-col gap-1 text-start">
              <span class="text-xs font-semibold text-foreground">{m.settings_entitlement_plan()}</span>
              <span class="text-2xl font-semibold">{formatLicensePlan(systemStore.entitlement?.plan) ?? m.settings_entitlement_plan_unavailable()}</span>
              <span class="text-[11px] text-muted-foreground">{entitlementStatusLabel(systemStore.entitlement?.status)}</span>
              <span class="text-[11px] text-muted-foreground">{m.settings_entitlement_current_status()}</span>
            </div>
          </div>
        </CardContent>
      </Card>

      {#if org.features}
        {@const features = typeof org.features === "string" ? (() => { try { return JSON.parse(org.features); } catch { return {}; } })() : org.features}
        {#if Object.keys(features || {}).length > 0}
          <Card>
            <CardHeader>
              <CardTitle>
                {m.settings_entitlement_features_title()}
              </CardTitle>
            </CardHeader>
            <CardContent class="px-6 py-5 flex flex-wrap gap-2">
              {#each Object.entries(features) as [feature, enabled]}
                {#if enabled}
                  <span class="rounded-md border border-border bg-muted/40 px-3 py-1.5 text-xs font-medium">{feature}</span>
                {/if}
              {/each}
            </CardContent>
          </Card>
        {/if}
      {/if}
    </form>

    <AvatarSettingsCard
      bind:logo
      {orgId}
      {name}
      {isSaving}
      onsave={handleSaveAvatar}
    />

    <DangerZoneSettingsCard
      onremove={onremove}
    />
  </div>

  <!-- Reusable Floating Settings Bottom Bar -->
  <SettingsBottomBar
    visible={isDirty}
    {isSaving}
    {isDirty}
    onsave={handleSubmit}
    onreset={handleReset}
  />
</div>
