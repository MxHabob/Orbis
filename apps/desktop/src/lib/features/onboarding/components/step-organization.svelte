<!--
  @component StepOrganization
  Step 2: Organization Profile & Regional Configuration.
  Minimalist form using @orbis/ui components (Input, Label, Select, Checkbox, Card) and design tokens.
-->
<script lang="ts">
import {
  type OnboardingFormModel,
  TIMEZONE_OPTIONS,
  CURRENCY_OPTIONS,
} from "../types";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Checkbox } from "@orbis/ui/checkbox";
import { Card, CardContent } from "@orbis/ui/card";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";

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
  if (!form.organizationSlug || form.organizationSlug === slugify(form.organizationName.slice(0, -1))) {
    const slug = slugify(value);
    if (slug) {
      form.organizationSlug = slug;
    }
  }
}

const selectedTz = $derived(TIMEZONE_OPTIONS.find((t) => t.value === form.timezone));
const selectedCur = $derived(CURRENCY_OPTIONS.find((c) => c.code === form.baseCurrencyCode));
</script>

<div class="space-y-6 max-w-2xl mx-auto">
  <div class="space-y-1">
    <h2 class="text-lg font-semibold tracking-tight text-foreground">
      {isRTL ? "بيانات المنشأة والهوية الإقليمية" : "Organization Profile & Regional Settings"}
    </h2>
    <p class="text-xs text-muted-foreground">
      {isRTL
        ? "حدد الاسم الرسمي للمؤسسة والعملة الأساسية والمنطقة الزمنية."
        : "Set your official institution name, primary currency, and timezone."}
    </p>
  </div>

  <Card class="border-border/60 shadow-xs">
    <CardContent class="p-5 space-y-4">
      <!-- Names -->
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
        <div class="space-y-1.5">
          <Label for="org-name-en" class="text-xs font-medium text-foreground">
            {isRTL ? "اسم المنشأة (بالإنجليزية) *" : "Organization Name (English) *"}
          </Label>
          <Input
            id="org-name-en"
            type="text"
            placeholder="e.g. Al-Noor Academy"
            value={form.organizationName}
            oninput={(e) => handleNameChange((e.target as HTMLInputElement).value)}
            class="h-9 text-xs"
          />
          {#if errors.organizationName}
            <p class="text-[11px] text-destructive">{errors.organizationName}</p>
          {/if}
        </div>

        <div class="space-y-1.5">
          <Label for="org-name-ar" class="text-xs font-medium text-foreground">
            {isRTL ? "اسم المنشأة (بالعربية)" : "Organization Name (Arabic)"}
          </Label>
          <Input
            id="org-name-ar"
            type="text"
            dir="rtl"
            placeholder="مثال: أكاديمية النور"
            bind:value={form.organizationNameAr}
            class="h-9 text-xs"
          />
        </div>
      </div>

      <!-- Slug & Code -->
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
        <div class="space-y-1.5">
          <Label for="org-slug" class="text-xs font-medium text-foreground">
            {isRTL ? "المعرف الفريد (Slug) *" : "Organization Slug *"}
          </Label>
          <Input
            id="org-slug"
            type="text"
            placeholder="al-noor-academy"
            bind:value={form.organizationSlug}
            class="h-9 text-xs font-mono"
          />
          {#if errors.organizationSlug}
            <p class="text-[11px] text-destructive">{errors.organizationSlug}</p>
          {/if}
        </div>

        <div class="space-y-1.5">
          <Label for="org-code" class="text-xs font-medium text-foreground">
            {isRTL ? "رمز المؤسسة" : "Organization Code"}
          </Label>
          <Input
            id="org-code"
            type="text"
            placeholder="e.g. NOOR-01"
            bind:value={form.organizationCode}
            class="h-9 text-xs font-mono uppercase"
          />
        </div>
      </div>

      <div class="border-t border-border/40 pt-4 grid grid-cols-1 sm:grid-cols-2 gap-4">
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
            <SelectTrigger class="w-full h-9 text-xs">
              {selectedTz ? (isRTL ? selectedTz.labelAr : selectedTz.labelEn) + ` (${selectedTz.offset})` : form.timezone}
            </SelectTrigger>
            <SelectContent>
              {#each TIMEZONE_OPTIONS as tz}
                <SelectItem value={tz.value}>
                  {isRTL ? tz.labelAr : tz.labelEn} ({tz.offset})
                </SelectItem>
              {/each}
            </SelectContent>
          </Select>
        </div>

        <!-- Currency -->
        <div class="space-y-1.5">
          <Label class="text-xs font-medium text-foreground">
            {isRTL ? "العملة الأساسية *" : "Base Currency *"}
          </Label>
          <Select
            type="single"
            value={form.baseCurrencyCode}
            onValueChange={(v) => (form.baseCurrencyCode = v ?? "SAR")}
          >
            <SelectTrigger class="w-full h-9 text-xs">
              {selectedCur ? `${selectedCur.code} - ${isRTL ? selectedCur.nameAr : selectedCur.nameEn}` : form.baseCurrencyCode}
            </SelectTrigger>
            <SelectContent>
              {#each CURRENCY_OPTIONS as cur}
                <SelectItem value={cur.code}>
                  {cur.code} - {isRTL ? cur.nameAr : cur.nameEn}
                </SelectItem>
              {/each}
            </SelectContent>
          </Select>
        </div>
      </div>

      <!-- Language & Hijri Option -->
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-4 items-center pt-2">
        <div class="space-y-1.5">
          <Label class="text-xs font-medium text-foreground">
            {isRTL ? "لغة الواجهة الافتراضية" : "Default Locale"}
          </Label>
          <Select
            type="single"
            value={form.locale}
            onValueChange={(v) => (form.locale = v ?? "ar-SA")}
          >
            <SelectTrigger class="w-full h-9 text-xs">
              {form.locale === "ar-SA" ? "العربية (ar-SA)" : "English (en-US)"}
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="ar-SA">العربية (ar-SA)</SelectItem>
              <SelectItem value="en-US">English (en-US)</SelectItem>
            </SelectContent>
          </Select>
        </div>

        <div class="pt-4 flex items-center gap-2">
          <Checkbox bind:checked={form.isHijriEnabled} id="org-hijri" />
          <Label for="org-hijri" class="text-xs font-normal text-foreground cursor-pointer">
            {isRTL ? "تفعيل التقويم الهجري المزدوج" : "Enable Dual Hijri Calendar"}
          </Label>
        </div>
      </div>
    </CardContent>
  </Card>
</div>
