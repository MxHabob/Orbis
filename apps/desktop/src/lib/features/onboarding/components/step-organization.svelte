<!--
  @component StepOrganization
  Step 2: Organization Profile, Regional Settings & Brand Identity.
-->
<script lang="ts">
import {
  type OnboardingFormModel,
  TIMEZONE_OPTIONS,
  CURRENCY_OPTIONS,
  COLOR_PRESETS,
} from "../types";
import { BuildingOffice, GlobeAlt, Sparkles } from "@orbis/icons";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";

let {
  form = $bindable<OnboardingFormModel>(),
  errors = {},
  isRTL = false,
}: {
  form: OnboardingFormModel;
  errors?: Record<string, string>;
  isRTL?: boolean;
} = $props();

function slugify(text: string): string {
  return text
    .toString()
    .toLowerCase()
    .trim()
    .replace(/[\s_]+/g, "-")
    .replace(/[^\w\-]+/g, "")
    .replace(/\-\-+/g, "-")
    .replace(/^-+/, "")
    .replace(/-+$/, "");
}

function handleNameChange(value: string) {
  form.organizationName = value;
  // If slug hasn't been manually customized or was empty, auto-update
  if (!form.organizationSlug || form.organizationSlug === slugify(form.organizationName.slice(0, -1))) {
    const slug = slugify(value);
    if (slug) {
      form.organizationSlug = slug;
    }
  }
}
</script>

<div class="space-y-6">
  <div class="text-center max-w-xl mx-auto space-y-2">
    <div class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full bg-primary/10 text-primary text-xs font-medium">
      <BuildingOffice size={14} />
      <span>{isRTL ? "الخطوة الثانية" : "Step 2 of 4"}</span>
    </div>
    <h2 class="text-2xl font-bold tracking-tight text-foreground">
      {isRTL ? "بيانات المنشأة والهوية المؤسسية" : "Organization Profile & Localization"}
    </h2>
    <p class="text-sm text-muted-foreground">
      {isRTL
        ? "حدد الاسم الرسمي، العملة الأساسية، والمنطقة الزمنية لمنشأتك."
        : "Set your official institution name, primary currency, and timezone."}
    </p>
  </div>

  <div class="max-w-2xl mx-auto space-y-6 bg-card/60 border border-border/80 rounded-2xl p-6 shadow-sm backdrop-blur-sm">
    <!-- Section: Identity -->
    <div class="space-y-4">
      <div class="flex items-center gap-2 pb-2 border-b border-border/60">
        <BuildingOffice size={16} class="text-primary" />
        <h3 class="text-sm font-semibold text-foreground">
          {isRTL ? "الأسماء والرمز التعريفي" : "Names & Identifiers"}
        </h3>
      </div>

      <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
        <!-- English Name -->
        <div class="space-y-1.5">
          <Label for="org-name-en" class="text-xs font-medium text-foreground">
            {isRTL ? "اسم المنشأة (بالإنجليزية) *" : "Organization Name (English) *"}
          </Label>
          <Input
            id="org-name-en"
            type="text"
            placeholder="e.g. Al-Noor International Academy"
            value={form.organizationName}
            oninput={(e) => handleNameChange((e.target as HTMLInputElement).value)}
            class="h-10 text-sm"
          />
          {#if errors.organizationName}
            <p class="text-[11px] text-destructive">{errors.organizationName}</p>
          {/if}
        </div>

        <!-- Arabic Name -->
        <div class="space-y-1.5">
          <Label for="org-name-ar" class="text-xs font-medium text-foreground">
            {isRTL ? "اسم المنشأة (بالعربية)" : "Organization Name (Arabic)"}
          </Label>
          <Input
            id="org-name-ar"
            type="text"
            dir="rtl"
            placeholder="مثال: أكاديمية النور العالمية"
            bind:value={form.organizationNameAr}
            class="h-10 text-sm"
          />
        </div>
      </div>

      <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
        <!-- Slug -->
        <div class="space-y-1.5">
          <div class="flex items-center justify-between">
            <Label for="org-slug" class="text-xs font-medium text-foreground">
              {isRTL ? "المعرف الفريد (Slug) *" : "Organization Slug *"}
            </Label>
            <span class="text-[10px] text-muted-foreground font-mono">orbis.app/{form.organizationSlug || "..."}</span>
          </div>
          <Input
            id="org-slug"
            type="text"
            placeholder="al-noor-academy"
            bind:value={form.organizationSlug}
            class="h-10 text-sm font-mono"
          />
          {#if errors.organizationSlug}
            <p class="text-[11px] text-destructive">{errors.organizationSlug}</p>
          {/if}
        </div>

        <!-- Code -->
        <div class="space-y-1.5">
          <Label for="org-code" class="text-xs font-medium text-foreground">
            {isRTL ? "رمز المؤسسة (اختياري)" : "Organization Code (Optional)"}
          </Label>
          <Input
            id="org-code"
            type="text"
            placeholder="e.g. NOOR-01"
            bind:value={form.organizationCode}
            class="h-10 text-sm uppercase font-mono"
          />
        </div>
      </div>
    </div>

    <!-- Section: Regional & Formatting -->
    <div class="space-y-4 pt-2">
      <div class="flex items-center gap-2 pb-2 border-b border-border/60">
        <GlobeAlt size={16} class="text-primary" />
        <h3 class="text-sm font-semibold text-foreground">
          {isRTL ? "الإعدادات الإقليمية والعملة" : "Regional & Financial Settings"}
        </h3>
      </div>

      <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
        <!-- Timezone -->
        <div class="space-y-1.5">
          <Label for="org-timezone" class="text-xs font-medium text-foreground">
            {isRTL ? "المنطقة الزمنية *" : "Timezone *"}
          </Label>
          <select
            id="org-timezone"
            bind:value={form.timezone}
            class="w-full h-10 px-3 rounded-lg border border-input bg-background text-sm text-foreground focus:outline-none focus:ring-2 focus:ring-primary/20 focus:border-primary transition-colors"
          >
            {#each TIMEZONE_OPTIONS as tz}
              <option value={tz.value}>
                {isRTL ? tz.labelAr : tz.labelEn} ({tz.offset})
              </option>
            {/each}
          </select>
        </div>

        <!-- Currency -->
        <div class="space-y-1.5">
          <Label for="org-currency" class="text-xs font-medium text-foreground">
            {isRTL ? "العملة الأساسية *" : "Base Currency *"}
          </Label>
          <select
            id="org-currency"
            bind:value={form.baseCurrencyCode}
            class="w-full h-10 px-3 rounded-lg border border-input bg-background text-sm text-foreground focus:outline-none focus:ring-2 focus:ring-primary/20 focus:border-primary transition-colors"
          >
            {#each CURRENCY_OPTIONS as cur}
              <option value={cur.code}>
                {cur.code} - {isRTL ? cur.nameAr : cur.nameEn} ({isRTL ? cur.symbolAr : cur.symbolEn})
              </option>
            {/each}
          </select>
        </div>
      </div>

      <div class="grid grid-cols-1 sm:grid-cols-2 gap-4 items-center">
        <!-- Language / Locale -->
        <div class="space-y-1.5">
          <Label for="org-locale" class="text-xs font-medium text-foreground">
            {isRTL ? "اللغة الافتراضية للنظام" : "Default System Language"}
          </Label>
          <select
            id="org-locale"
            bind:value={form.locale}
            class="w-full h-10 px-3 rounded-lg border border-input bg-background text-sm text-foreground focus:outline-none focus:ring-2 focus:ring-primary/20 focus:border-primary transition-colors"
          >
            <option value="ar-SA">العربية (المملكة العربية السعودية)</option>
            <option value="en-US">English (United States)</option>
          </select>
        </div>

        <!-- Hijri Calendar Option -->
        <div class="pt-4 sm:pt-6">
          <label class="flex items-center gap-3 cursor-pointer p-2.5 rounded-lg border border-border/80 hover:bg-muted/40 transition-colors">
            <input
              type="checkbox"
              bind:checked={form.isHijriEnabled}
              class="size-4 rounded border-input text-primary focus:ring-primary"
            />
            <div class="flex flex-col">
              <span class="text-xs font-medium text-foreground">
                {isRTL ? "تفعيل التقويم الهجري المزدوج" : "Enable Dual Hijri Calendar"}
              </span>
              <span class="text-[10px] text-muted-foreground">
                {isRTL ? "عرض التواريخ الهجرية بجانب الميلادية" : "Show Islamic dates alongside Gregorian"}
              </span>
            </div>
          </label>
        </div>
      </div>
    </div>

    <!-- Section: Brand Color -->
    <div class="space-y-3 pt-2">
      <div class="flex items-center gap-2 pb-2 border-b border-border/60">
        <Sparkles size={16} class="text-primary" />
        <h3 class="text-sm font-semibold text-foreground">
          {isRTL ? "السمة اللونية للمؤسسة" : "Brand Accent Color"}
        </h3>
      </div>

      <div class="flex flex-wrap items-center gap-2.5">
        {#each COLOR_PRESETS as color}
          {@const isSelected = form.primaryColor.toLowerCase() === color.hex.toLowerCase()}
          <button
            type="button"
            onclick={() => (form.primaryColor = color.hex)}
            class="flex items-center gap-2 px-3 py-1.5 rounded-xl border text-xs transition-all {isSelected
              ? 'border-foreground shadow-sm bg-muted font-medium'
              : 'border-border/80 hover:bg-muted/50 text-muted-foreground'}"
          >
            <span
              class="size-3.5 rounded-full border border-black/10 shrink-0"
              style="background-color: {color.hex};"
            ></span>
            <span>{isRTL ? color.labelAr : color.labelEn}</span>
          </button>
        {/each}
      </div>
    </div>
  </div>
</div>
