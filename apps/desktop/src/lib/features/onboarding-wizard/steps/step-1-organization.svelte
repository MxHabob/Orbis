<script lang="ts">
import { onMount } from "svelte";
import { goto } from "$app/navigation";
import { getLocale } from "$lib/paraglide/runtime";
import { api, type OrgDto, type OrganizationSettingsDto } from "$lib/api";
import { systemStore } from "$lib/stores";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Checkbox } from "@orbis/ui/checkbox";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import OnboardingStepHeader from "../onboarding-step-header.svelte";
import OnboardingSection from "../onboarding-section.svelte";
import { getTasksForStep } from "../onboarding-steps";

let { orgId }: { orgId: string } = $props();

const isArabic = $derived(getLocale().startsWith("ar"));

let isLoading = $state(true);
let isSaving = $state(false);
let errorMessage = $state<string | null>(null);
let successMessage = $state<string | null>(null);

// Form Fields
let name = $state("");
let nameAr = $state("");
let slug = $state("");
let baseCurrencyCode = $state("SAR");
let timezone = $state("Asia/Riyadh");
let locale = $state("ar");
let isHijriEnabled = $state(false);
let email = $state("");
let phone = $state("");
let address = $state("");

// Currency options
const CURRENCIES = [
  { code: "SAR", name: "Saudi Riyal (ر.س)", nameAr: "ريال سعودي" },
  { code: "AED", name: "UAE Dirham (د.إ)", nameAr: "درهم إماراتي" },
  { code: "QAR", name: "Qatari Riyal (ر.ق)", nameAr: "ريال قطري" },
  { code: "KWD", name: "Kuwaiti Dinar (د.ك)", nameAr: "دينار كويتي" },
  { code: "BHD", name: "Bahraini Dinar (د.ب)", nameAr: "دينار بحريني" },
  { code: "OMR", name: "Omani Rial (ر.ع)", nameAr: "ريال عماني" },
  { code: "EGP", name: "Egyptian Pound (ج.م)", nameAr: "جنيه مصري" },
  { code: "YER", name: "Yemeni Rial (ر.ي)", nameAr: "ريال يمني" },
  { code: "USD", name: "US Dollar ($)", nameAr: "دولار أمريكي" },
  { code: "EUR", name: "Euro (€)", nameAr: "يورو" },
  { code: "GBP", name: "British Pound (£)", nameAr: "جنيه إسترليني" },
];

// Timezone options
const TIMEZONES = [
  { value: "Asia/Riyadh", label: "Riyadh, Saudi Arabia (UTC+3)" },
  { value: "Asia/Dubai", label: "Dubai, UAE (UTC+4)" },
  { value: "Asia/Kuwait", label: "Kuwait (UTC+3)" },
  { value: "Asia/Bahrain", label: "Bahrain (UTC+3)" },
  { value: "Asia/Qatar", label: "Qatar (UTC+3)" },
  { value: "Asia/Muscat", label: "Muscat, Oman (UTC+4)" },
  { value: "Africa/Cairo", label: "Cairo, Egypt (UTC+2)" },
  { value: "Asia/Amman", label: "Amman, Jordan (UTC+3)" },
  { value: "Asia/Beirut", label: "Beirut, Lebanon (UTC+3)" },
  { value: "Asia/Aden", label: "Aden / Sanaa, Yemen (UTC+3)" },
  { value: "UTC", label: "Coordinated Universal Time (UTC)" },
  { value: "Europe/London", label: "London, UK (UTC+0 / UTC+1)" },
  { value: "America/New_York", label: "New York, USA (UTC-5 / UTC-4)" },
];

const stepTasks = $derived(getTasksForStep("organization", systemStore.onboardingTasks));
const isTaskComplete = $derived(stepTasks.every((t) => t.status === "completed"));

onMount(async () => {
  try {
    isLoading = true;
    const org = await api.getCurrentOrg();
    if (org) {
      name = org.name || "";
      nameAr = org.name_ar || "";
      slug = org.slug || "";
      baseCurrencyCode = org.base_currency_code || org.currency || "SAR";
      timezone = org.timezone || "Asia/Riyadh";
      locale = org.locale || "ar";
      isHijriEnabled = Boolean(org.is_hijri_enabled ?? org.hijri_enabled);
    }
  } catch (err: any) {
    console.error("Failed to load organization profile:", err);
    errorMessage = err?.message || "Failed to load organization settings";
  } finally {
    isLoading = false;
  }
});

async function saveOrganization() {
  if (!name.trim()) {
    errorMessage = isArabic ? "يرجى كتابة اسم المنشأة" : "Organization name is required";
    return;
  }

  isSaving = true;
  errorMessage = null;
  successMessage = null;

  try {
    await api.updateCurrentOrg({
      name: name.trim(),
      name_ar: nameAr.trim() || undefined,
      slug: slug.trim() || undefined,
      base_currency_code: baseCurrencyCode,
      timezone,
      locale,
      is_hijri_enabled: isHijriEnabled,
    });

    // Refresh onboarding DAG state
    await systemStore.loadOnboardingState(orgId);
    successMessage = isArabic ? "تم حفظ بيانات المنشأة بنجاح" : "Organization profile saved successfully";
  } catch (err: any) {
    console.error("Failed to save organization profile:", err);
    errorMessage = err?.message || "Failed to save profile";
  } finally {
    isSaving = false;
  }
}

async function handleContinue() {
  if (!isTaskComplete) {
    await saveOrganization();
  }
  await goto(`/${orgId}/onboarding/structure`);
}
</script>

<div class="space-y-6">
  <OnboardingStepHeader
    stepNumber={1}
    title="Organization Profile & Regional Settings"
    titleAr="الملف التعريفي والإعدادات الإقليمية للمنشأة"
    description="Set up your institution's formal identity, primary currency, operational timezone, and regional calendar preferences."
    descriptionAr="حدد الهوية الرسمية للمنشأة والعملة الأساسية للنظام والنطاق الزمني والتقويم المعتمد."
    isRequired={true}
    status={isTaskComplete ? "completed" : "in_progress"}
  />

  {#if errorMessage}
    <div class="rounded-lg border border-destructive/30 bg-destructive/10 p-4 text-sm text-destructive font-medium flex items-center justify-between">
      <span>{errorMessage}</span>
      <Button variant="ghost" size="sm" onclick={() => errorMessage = null}>✕</Button>
    </div>
  {/if}

  {#if successMessage}
    <div class="rounded-lg border border-emerald-500/30 bg-emerald-500/10 p-4 text-sm text-emerald-600 dark:text-emerald-400 font-medium flex items-center justify-between">
      <span>{successMessage}</span>
      <Button variant="ghost" size="sm" onclick={() => successMessage = null}>✕</Button>
    </div>
  {/if}

  {#if isLoading}
    <div class="flex items-center justify-center p-12 text-muted-foreground text-sm gap-2">
      <span class="size-4 animate-spin rounded-full border-2 border-primary border-t-transparent"></span>
      <span>{isArabic ? "جاري تحميل البيانات..." : "Loading organization profile..."}</span>
    </div>
  {:else}
    <!-- Basic Identity Section -->
    <OnboardingSection
      title="Institution Identity"
      titleAr="هوية المنشأة التعليمية"
      description="The primary name displayed across certificates, reports, portals, and student documents."
      descriptionAr="الاسم المعتمد الذي يظهر في الشهادات والتقارير وبوابات الطلاب والموظفين."
      isComplete={Boolean(name.trim())}
    >
      <div class="grid gap-4 sm:grid-cols-2">
        <div class="space-y-2">
          <Label for="org-name" class="text-xs font-semibold">
            {isArabic ? "اسم المنشأة (بالإنجليزية / اللاتينية) *" : "Institution Name (English / Latin) *"}
          </Label>
          <Input
            id="org-name"
            bind:value={name}
            placeholder="e.g. Al-Noor Academy"
            dir="ltr"
            required
          />
        </div>

        <div class="space-y-2">
          <Label for="org-name-ar" class="text-xs font-semibold">
            {isArabic ? "اسم المنشأة بالعربية" : "Institution Name (Arabic)"}
          </Label>
          <Input
            id="org-name-ar"
            bind:value={nameAr}
            placeholder="مثال: أكاديمية النور الأهلية"
            dir="rtl"
          />
        </div>

        <div class="space-y-2 sm:col-span-2">
          <Label for="org-slug" class="text-xs font-semibold">
            {isArabic ? "المعرف الفريد / النطاق الداخلي" : "Organization Code / Subdomain Identifier"}
          </Label>
          <Input
            id="org-slug"
            bind:value={slug}
            placeholder="alnoor-academy"
            dir="ltr"
            class="font-mono text-xs"
          />
          <p class="text-[11px] text-muted-foreground">
            {isArabic ? "يُستخدم كرمز تعريفي في تقارير النظام وقواعد البيانات." : "Used as a system identifier in reports and data exports."}
          </p>
        </div>
      </div>
    </OnboardingSection>

    <!-- Regional & Financial Preferences -->
    <OnboardingSection
      title="Regional & Financial Defaults"
      titleAr="الإعدادات الإقليمية والمالية الأساسية"
      description="Define the base accounting currency and operational timezone for accurate scheduling and accounting."
      descriptionAr="حدد العملة الأساسية للحسابات والتقويم الزمني لجداول الحصص والعمليات المالية."
      isComplete={Boolean(baseCurrencyCode && timezone)}
    >
      <div class="grid gap-4 sm:grid-cols-2">
        <!-- Currency Select -->
        <div class="space-y-2">
          <Label class="text-xs font-semibold">
            {isArabic ? "العملة الأساسية للحسابات والرسوم *" : "Base Operational Currency *"}
          </Label>
          <Select type="single" bind:value={baseCurrencyCode}>
            <SelectTrigger class="w-full">
              {CURRENCIES.find((c) => c.code === baseCurrencyCode)?.[isArabic ? "nameAr" : "name"] || baseCurrencyCode}
            </SelectTrigger>
            <SelectContent>
              {#each CURRENCIES as curr}
                <SelectItem value={curr.code}>
                  {isArabic ? `${curr.nameAr} (${curr.code})` : curr.name}
                </SelectItem>
              {/each}
            </SelectContent>
          </Select>
        </div>

        <!-- Timezone Select -->
        <div class="space-y-2">
          <Label class="text-xs font-semibold">
            {isArabic ? "المنطقة الزمنية *" : "Operational Timezone *"}
          </Label>
          <Select type="single" bind:value={timezone}>
            <SelectTrigger class="w-full">
              {TIMEZONES.find((t) => t.value === timezone)?.label || timezone}
            </SelectTrigger>
            <SelectContent>
              {#each TIMEZONES as tz}
                <SelectItem value={tz.value}>{tz.label}</SelectItem>
              {/each}
            </SelectContent>
          </Select>
        </div>

        <!-- System Locale Select -->
        <div class="space-y-2">
          <Label class="text-xs font-semibold">
            {isArabic ? "اللغة الافتراضية للنظام" : "Default Interface Language"}
          </Label>
          <Select type="single" bind:value={locale}>
            <SelectTrigger class="w-full">
              {locale === "ar" ? "العربية (Arabic)" : "English"}
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="ar">العربية (Arabic)</SelectItem>
              <SelectItem value="en">English</SelectItem>
            </SelectContent>
          </Select>
        </div>

        <!-- Hijri Calendar Toggle -->
        <div class="flex items-center space-x-3 space-x-reverse rounded-lg border border-border/50 p-3 self-end">
          <Checkbox id="hijri-toggle" bind:checked={isHijriEnabled} />
          <div class="space-y-0.5">
            <Label for="hijri-toggle" class="text-xs font-medium cursor-pointer">
              {isArabic ? "تفعيل التقويم الهجري بالتوازي" : "Enable Dual Hijri Calendar"}
            </Label>
            <p class="text-[11px] text-muted-foreground">
              {isArabic ? "عرض التواريخ الهجرية بجانب الميلادية" : "Display Islamic Hijri dates alongside Gregorian dates"}
            </p>
          </div>
        </div>
      </div>
    </OnboardingSection>

    <!-- Bottom Actions Bar -->
    <div class="flex items-center justify-between border-t border-border/50 pt-6">
      <Button
        variant="outline"
        onclick={() => goto(`/${orgId}/onboarding`)}
        class="text-xs"
      >
        {isArabic ? "← الرجوع للمركز" : "← Setup Overview"}
      </Button>

      <div class="flex items-center gap-3">
        <Button
          variant="secondary"
          disabled={isSaving}
          onclick={saveOrganization}
          class="text-xs"
        >
          {isSaving ? (isArabic ? "جاري الحفظ..." : "Saving...") : (isArabic ? "حفظ التغييرات" : "Save Changes")}
        </Button>

        <Button
          variant="default"
          disabled={isSaving || !name.trim()}
          onclick={handleContinue}
          class="text-xs font-semibold shadow-xs"
        >
          {isArabic ? "حفظ ومتابعة إلى الهيكل التنظيمي →" : "Save & Continue to Structure →"}
        </Button>
      </div>
    </div>
  {/if}
</div>
