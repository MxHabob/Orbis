<script lang="ts">
import * as m from "$lib/paraglide/messages";
import {
  InfoSettingsCard,
  BrandingSettingsCard,
  RegionalSettingsCard,
  AvatarSettingsCard,
  DangerZoneSettingsCard,
} from "$lib/features/settings";
import SettingsBottomBar from "$lib/shared/components/settings-bottom-bar.svelte";
import { toast } from "svelte-sonner";
import { l } from "$lib/core/i18n";
import { generalSettingsInputSchema } from "@orbis/schemas";
import { FormController } from "$lib/shared/forms/form-controller.svelte";

let { org, orgId, isSaving, onsave, onsaveavatar, onremove, onlanguagechange } =
  $props<{
    org: any;
    orgId: string;
    isSaving: boolean;
    onsave: (request: any) => void;
    onsaveavatar: (logo: string | null) => void;
    onremove: () => void;
    onlanguagechange: (newLocale: string, currentValues: any) => void;
  }>();

// Form state initialized once with props
let name = $state("");
let nameAr = $state("");
let slug = $state("");
let logo = $state("");
let primaryColor = $state("#84cc16");
let currency = $state("YER");
let timezone = $state("Asia/Aden");
let locale = $state("ar");
let dateFormat = $state("YYYY-MM-DD");
let hijriEnabled = $state(false);

let initialValues = $state<any>(null);
const settingsForm = new FormController({
  initialValues: {
    name: "",
    currency: "YER",
    timezone: "Asia/Aden",
    locale: "ar",
    dateFormat: "YYYY-MM-DD",
  },
  schema: generalSettingsInputSchema,
});

$effect(() => {
  if (org && !initialValues) {
    name = org.name || "";
    nameAr = org.name_ar || "";
    slug = org.slug || "";
    logo = org.logo || "";
    primaryColor = org.primary_color || "#84cc16";
    currency = org.currency || "YER";
    timezone = org.timezone || "Asia/Aden";
    locale = org.locale || "ar";
    dateFormat = org.date_format || "YYYY-MM-DD";
    hijriEnabled = org.hijri_enabled === true || org.hijri_enabled === 1;

    initialValues = {
      name,
      nameAr,
      primaryColor,
      currency,
      timezone,
      locale,
      dateFormat,
      hijriEnabled,
    };
  }
});

const isDirty = $derived.by(() => {
  if (!initialValues) return false;
  return (
    name !== initialValues.name ||
    nameAr !== initialValues.nameAr ||
    primaryColor !== initialValues.primaryColor ||
    currency !== initialValues.currency ||
    timezone !== initialValues.timezone ||
    locale !== initialValues.locale ||
    dateFormat !== initialValues.dateFormat ||
    hijriEnabled !== initialValues.hijriEnabled
  );
});

function handleReset() {
  if (!initialValues) return;
  name = initialValues.name;
  nameAr = initialValues.nameAr;
  primaryColor = initialValues.primaryColor;
  currency = initialValues.currency;
  timezone = initialValues.timezone;
  locale = initialValues.locale;
  dateFormat = initialValues.dateFormat;
  hijriEnabled = initialValues.hijriEnabled;
}

function handleSubmit(e?: Event) {
  e?.preventDefault();
  settingsForm.values = { name, currency, timezone, locale, dateFormat };
  settingsForm.submit(async (result) => {
    onsave({
      name: result.name,
      name_ar: nameAr || null,
      logo: logo || null,
      primary_color: primaryColor,
      currency: result.currency,
      timezone: result.timezone,
      locale: result.locale,
      date_format: result.dateFormat,
      hijri_enabled: hijriEnabled,
    });
  }).then((submitted) => {
    if (!submitted) {
      toast.error(l("بيانات الإعدادات غير صالحة", "Settings contain invalid values"));
    }
  });
  if (initialValues) {
    initialValues = {
      name,
      nameAr,
      primaryColor,
      currency,
      timezone,
      locale,
      dateFormat,
      hijriEnabled,
    };
  }
}

function handleLanguageChange(newLocale: string) {
  onlanguagechange(newLocale, {
    name,
    name_ar: nameAr || null,
    logo: logo || null,
    primary_color: primaryColor,
    currency,
    timezone,
    locale: newLocale,
    date_format: dateFormat,
    hijri_enabled: hijriEnabled,
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
        bind:name
        bind:nameAr
        organizationId={orgId}
        {isSaving}
      />

      <BrandingSettingsCard
        bind:primaryColor
        {isSaving}
      />

      <RegionalSettingsCard
        bind:currency
        bind:timezone
        bind:locale
        bind:dateFormat
        bind:hijriEnabled
        {isSaving}
        onLanguageChange={handleLanguageChange}
      />

            <div class="border border-border rounded-md overflow-hidden flex flex-col">
              <div class="bg-muted/50 px-6 py-3 border-b border-border text-[11px] font-bold text-muted-foreground tracking-wider uppercase text-start select-none">
                حدود المنظمة
              </div>
              <div class="grid grid-cols-1 md:grid-cols-3 divide-y md:divide-y-0 md:divide-x rtl:md:divide-x-reverse divide-border">
                <div class="px-6 py-5 flex flex-col gap-1 text-start">
                  <span class="text-xs font-semibold text-foreground">المستخدمون</span>
                  <span class="text-2xl font-semibold tabular-nums">{org.max_users ?? "—"}</span>
                  <span class="text-[11px] text-muted-foreground">الحد الأقصى المسموح به</span>
                </div>
                <div class="px-6 py-5 flex flex-col gap-1 text-start">
                  <span class="text-xs font-semibold text-foreground">الطلاب</span>
                  <span class="text-2xl font-semibold tabular-nums">{org.max_students ?? "—"}</span>
                  <span class="text-[11px] text-muted-foreground">الحد الأقصى المسموح به</span>
                </div>
                <div class="px-6 py-5 flex flex-col gap-1 text-start">
                  <span class="text-xs font-semibold text-foreground">التخزين</span>
                  <span class="text-2xl font-semibold tabular-nums">{org.max_storage_gb ?? "—"} GB</span>
                  <span class="text-[11px] text-muted-foreground">السعة المخصصة للمنظمة</span>
                </div>
              </div>
            </div>

            {#if org.features}
              {@const features = typeof org.features === "string" ? (() => { try { return JSON.parse(org.features); } catch { return {}; } })() : org.features}
              {#if Object.keys(features || {}).length > 0}
                <div class="border border-border rounded-md overflow-hidden flex flex-col">
                  <div class="bg-muted/50 px-6 py-3 border-b border-border text-[11px] font-bold text-muted-foreground tracking-wider uppercase text-start select-none">
                    الميزات المفعلة
                  </div>
                  <div class="px-6 py-5 flex flex-wrap gap-2">
                    {#each Object.entries(features) as [feature, enabled]}
                      {#if enabled}
                        <span class="rounded-md border border-border bg-muted/40 px-3 py-1.5 text-xs font-medium">{feature}</span>
                      {/if}
                    {/each}
                  </div>
                </div>
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
