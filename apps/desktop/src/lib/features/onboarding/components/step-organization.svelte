<!--
  @component StepOrganization
  Step 3: Enriched Organization Profile & Identity Configuration.
  Spacious 2-column layout for official names, academic track, capacity, regional/fiscal settings,
  and contact details. (Slug is generated automatically on backend).
-->
<script lang="ts">
import {
  type OnboardingFormModel,
  TIMEZONE_OPTIONS,
  CURRENCY_OPTIONS,
  COUNTRIES_LIST,
  EDUCATIONAL_TRACK_OPTIONS,
  ORGANIZATION_SIZE_OPTIONS,
  COLOR_PRESETS,
} from "../types";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Checkbox } from "@orbis/ui/checkbox";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import { Check } from "@orbis/icons";

let {
  form = $bindable<OnboardingFormModel>(),
  errors = {},
  isRTL = false,
}: {
  form: OnboardingFormModel;
  errors?: Record<string, string>;
  isRTL?: boolean;
} = $props();

function selectCountry(countryCode: string) {
  form.countryCode = countryCode;
  const country = COUNTRIES_LIST.find((c) => c.code === countryCode);
  if (country) {
    if (country.defaultCurrency) form.baseCurrencyCode = country.defaultCurrency;
    if (country.defaultTimezone) form.timezone = country.defaultTimezone;
  }
}
</script>

<div class="space-y-6 max-w-5xl mx-auto py-2">
  <div class="space-y-2 text-start">
    <div class="flex items-center gap-2">
      <span class="text-xs font-semibold text-primary uppercase font-mono tracking-wider">
        {isRTL ? "المرحلة ٣ من ٥" : "Step 3 of 5"}
      </span>
    </div>
    <h2 class="text-2xl font-bold tracking-tight text-foreground">
      {isRTL ? "بيانات المنشأة والهوية الأكاديمية" : "Organization Profile & Identity"}
    </h2>
    <p class="text-sm text-muted-foreground max-w-2xl leading-relaxed">
      {isRTL
        ? "حدد الاسم الرسمي، المسار التعليمي، العملة والمنطقة الجغرافية للمؤسسة."
        : "Configure your official name, educational track, base currency, and regional operating parameters."}
    </p>
  </div>

  <div class="grid grid-cols-1 lg:grid-cols-2 gap-6 text-start">
    <!-- Left Card: Institution Identity & Track -->
    <div class="rounded-2xl border border-border/70 bg-card p-6 space-y-5 shadow-xs">
      <div class="border-b border-border/40 pb-3">
        <h3 class="text-sm font-bold text-foreground uppercase tracking-wider font-mono">
          {isRTL ? "الهوية الأكاديمية والمسار" : "Identity & Track"}
        </h3>
      </div>

      <!-- Arabic Name -->
      <div class="space-y-1.5">
        <Label for="org-name-ar" class="text-xs font-medium text-foreground">
          {isRTL ? "اسم المنشأة (بالعربية) *" : "Organization Name (Arabic) *"}
        </Label>
        <Input
          id="org-name-ar"
          type="text"
          dir="rtl"
          placeholder="مثال: مدارس النور الأهلية"
          bind:value={form.organizationNameAr}
          class="h-9 text-xs"
        />
        {#if errors.organizationNameAr}
          <p class="text-[11px] text-destructive">{errors.organizationNameAr}</p>
        {/if}
      </div>

      <!-- English Name -->
      <div class="space-y-1.5">
        <Label for="org-name-en" class="text-xs font-medium text-foreground">
          {isRTL ? "اسم المنشأة (بالإنجليزية) *" : "Organization Name (English) *"}
        </Label>
        <Input
          id="org-name-en"
          type="text"
          placeholder="e.g. Al-Noor Academy"
          bind:value={form.organizationName}
          class="h-9 text-xs"
        />
        {#if errors.organizationName}
          <p class="text-[11px] text-destructive">{errors.organizationName}</p>
        {/if}
      </div>

      <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
        <!-- Short Code -->
        <div class="space-y-1.5">
          <Label for="org-code" class="text-xs font-medium text-foreground">
            {isRTL ? "الرمز الأكاديمي المختصر" : "Short Code"}
          </Label>
          <Input
            id="org-code"
            type="text"
            placeholder="e.g. NOOR-01"
            bind:value={form.organizationCode}
            class="h-9 text-xs font-mono uppercase"
          />
        </div>

        <!-- Educational Track -->
        <div class="space-y-1.5">
          <Label class="text-xs font-medium text-foreground">
            {isRTL ? "المسار التعليمي والمنهج *" : "Educational Track *"}
          </Label>
          <Select
            type="single"
            value={form.educationalTrack}
            onValueChange={(v) => (form.educationalTrack = v ?? "national")}
          >
            <SelectTrigger class="w-full h-9 text-xs">
              {EDUCATIONAL_TRACK_OPTIONS.find((t) => t.value === form.educationalTrack)?.labelAr || form.educationalTrack}
            </SelectTrigger>
            <SelectContent>
              {#each EDUCATIONAL_TRACK_OPTIONS as opt}
                <SelectItem value={opt.value}>
                  <div class="space-y-0.5">
                    <div class="text-xs font-medium">{isRTL ? opt.labelAr : opt.labelEn}</div>
                  </div>
                </SelectItem>
              {/each}
            </SelectContent>
          </Select>
        </div>
      </div>

      <!-- Capacity Size -->
      <div class="space-y-1.5">
        <Label class="text-xs font-medium text-foreground">
          {isRTL ? "الطاقة الاستيعابية المتوقعة" : "Expected Student Capacity"}
        </Label>
        <Select
          type="single"
          value={form.organizationSize}
          onValueChange={(v) => (form.organizationSize = v ?? "medium")}
        >
          <SelectTrigger class="w-full h-9 text-xs">
            {ORGANIZATION_SIZE_OPTIONS.find((s) => s.value === form.organizationSize)?.labelAr || form.organizationSize}
          </SelectTrigger>
          <SelectContent>
            {#each ORGANIZATION_SIZE_OPTIONS as opt}
              <SelectItem value={opt.value}>
                <span class="text-xs">{isRTL ? opt.labelAr : opt.labelEn}</span>
              </SelectItem>
            {/each}
          </SelectContent>
        </Select>
      </div>
    </div>

    <!-- Right Card: Regional & Localization Settings -->
    <div class="rounded-2xl border border-border/70 bg-card p-6 space-y-5 shadow-xs">
      <div class="border-b border-border/40 pb-3">
        <h3 class="text-sm font-bold text-foreground uppercase tracking-wider font-mono">
          {isRTL ? "النطاق الجغرافي والمالي" : "Regional & Fiscal Parameters"}
        </h3>
      </div>

      <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
        <!-- Country -->
        <div class="space-y-1.5">
          <Label class="text-xs font-medium text-foreground">
            {isRTL ? "الدولة / النطاق *" : "Country / Region *"}
          </Label>
          <Select
            type="single"
            value={form.countryCode}
            onValueChange={(v) => v && selectCountry(v)}
          >
            <SelectTrigger class="w-full h-9 text-xs">
              {COUNTRIES_LIST.find((c) => c.code === form.countryCode) ? `${COUNTRIES_LIST.find((c) => c.code === form.countryCode)?.flag} ${isRTL ? COUNTRIES_LIST.find((c) => c.code === form.countryCode)?.nameAr : COUNTRIES_LIST.find((c) => c.code === form.countryCode)?.nameEn}` : form.countryCode}
            </SelectTrigger>
            <SelectContent>
              {#each COUNTRIES_LIST as country}
                <SelectItem value={country.code}>
                  <span class="text-xs">{country.flag} {isRTL ? country.nameAr : country.nameEn}</span>
                </SelectItem>
              {/each}
            </SelectContent>
          </Select>
        </div>

        <!-- City -->
        <div class="space-y-1.5">
          <Label for="org-city" class="text-xs font-medium text-foreground">
            {isRTL ? "المدينة / الفرع الرئيسي" : "City / Main Campus"}
          </Label>
          <Input
            id="org-city"
            type="text"
            placeholder={isRTL ? "مثال: الرياض" : "e.g. Riyadh"}
            bind:value={form.city}
            class="h-9 text-xs"
          />
        </div>
      </div>

      <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
        <!-- Base Currency -->
        <div class="space-y-1.5">
          <Label class="text-xs font-medium text-foreground">
            {isRTL ? "العملة الأساسية للحسابات *" : "Base Currency *"}
          </Label>
          <Select
            type="single"
            value={form.baseCurrencyCode}
            onValueChange={(v) => (form.baseCurrencyCode = v ?? "SAR")}
          >
            <SelectTrigger class="w-full h-9 text-xs font-mono">
              {form.baseCurrencyCode} ({isRTL ? CURRENCY_OPTIONS.find((c) => c.code === form.baseCurrencyCode)?.symbolAr : CURRENCY_OPTIONS.find((c) => c.code === form.baseCurrencyCode)?.symbolEn})
            </SelectTrigger>
            <SelectContent>
              {#each CURRENCY_OPTIONS as cur}
                <SelectItem value={cur.code}>
                  <span class="text-xs font-mono">{cur.code} - {isRTL ? cur.nameAr : cur.nameEn}</span>
                </SelectItem>
              {/each}
            </SelectContent>
          </Select>
        </div>

        <!-- Timezone -->
        <div class="space-y-1.5">
          <Label class="text-xs font-medium text-foreground">
            {isRTL ? "المنطقة الزمنية *" : "Timezone *"}
          </Label>
          <Select
            type="single"
            value={form.timezone}
            onValueChange={(v) => (form.timezone = v ?? "Asia/Riyadh")}
          >
            <SelectTrigger class="w-full h-9 text-xs font-mono">
              {form.timezone}
            </SelectTrigger>
            <SelectContent>
              {#each TIMEZONE_OPTIONS as tz}
                <SelectItem value={tz.value}>
                  <span class="text-xs font-mono">{tz.label} ({tz.offset})</span>
                </SelectItem>
              {/each}
            </SelectContent>
          </Select>
        </div>
      </div>

      <!-- Hijri Calendar Preference -->
      <div class="p-3.5 rounded-xl bg-muted/20 border border-border/50 flex items-center justify-between">
        <div class="space-y-0.5">
          <Label for="hijri-toggle" class="text-xs font-semibold text-foreground cursor-pointer">
            {isRTL ? "تفعيل التقويم الهجري المزدوج" : "Dual Hijri & Gregorian Calendar"}
          </Label>
          <p class="text-[11px] text-muted-foreground">
            {isRTL ? "عرض ومزامنة التاريخ الهجري تلقائياً بجانب الميلادي." : "Display corresponding Umm al-Qura Hijri dates across modules."}
          </p>
        </div>
        <Checkbox id="hijri-toggle" bind:checked={form.isHijriEnabled} />
      </div>

      <!-- Theme Color Palette -->
      <div class="space-y-2 pt-1">
        <Label class="text-xs font-medium text-foreground">
          {isRTL ? "السمة اللونية للواجهة" : "Accent Color Theme"}
        </Label>
        <div class="flex items-center gap-2.5">
          {#each COLOR_PRESETS as color}
            {@const isColorActive = form.primaryColor === color.value}
            <button
              type="button"
              onclick={() => (form.primaryColor = color.value)}
              class="size-7 rounded-full transition-transform flex items-center justify-center {isColorActive ? 'scale-110 ring-2 ring-foreground ring-offset-2 ring-offset-background' : 'hover:scale-105'}"
              style="background-color: {color.value};"
              title={isRTL ? color.nameAr : color.nameEn}
            >
              {#if isColorActive}
                <Check size={12} class="text-white" strokeWidth={3} />
              {/if}
            </button>
          {/each}
        </div>
      </div>
    </div>
  </div>
</div>
