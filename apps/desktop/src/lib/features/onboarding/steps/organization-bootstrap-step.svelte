<!--
  Step 0: Organization & Admin Bootstrap
  Unified first-step in the Onboarding Wizard when no organization exists in the system yet.
  Clean, minimalist, elegant 2-column layout without visual noise or redundant icons.
-->
<script lang="ts">
import { onMount } from "svelte";
import { api } from "$lib/api/client";
import { authStore } from "$lib/stores";
import { getLocale } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { FormField } from "@orbis/ui";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import { AdjustmentsHorizontal, ArrowLeft, ArrowRight, Check, Eye, EyeSlash, InfoCircle, Sparkles } from "@orbis/icons";
import { z } from "zod";
import { FormController } from "$lib/shared/forms/form-controller.svelte";
import { l } from "$lib/core/i18n";
import { detectTimezone, detectCurrency, detectLocale } from "../utils";
import { onboardingState } from "../state.svelte";
import { INSTITUTION_PROFILES } from "../templates";
import { orgContext } from "$lib/stores/org-context.svelte";

const isRTL = $derived(getLocale().startsWith("ar"));

let showPassword = $state(false);
let showPasswordConfirm = $state(false);

interface CurrencyItem {
  code: string;
  label: string;
  symbol?: string;
}

const fallbackCurrencies: CurrencyItem[] = [
  { code: "SAR", label: "SAR - ريال سعودي (ر.س)", symbol: "ر.س" },
  { code: "YER", label: "YER - ريال يمني (﷼)", symbol: "﷼" },
  { code: "USD", label: "USD - US Dollar ($)", symbol: "$" },
  { code: "AED", label: "AED - درهم إماراتي (د.إ)", symbol: "د.إ" },
  { code: "EGP", label: "EGP - جنيه مصري (ج.م)", symbol: "ج.م" },
  { code: "EUR", label: "EUR - Euro (€)", symbol: "€" },
  { code: "KWD", label: "KWD - دينار كويتي (د.ك)", symbol: "د.ك" },
  { code: "QAR", label: "QAR - ريال قطري (ر.ق)", symbol: "ر.ق" },
  { code: "OMR", label: "OMR - ريال عماني (ر.ع)", symbol: "ر.ع" },
  { code: "BHD", label: "BHD - دينار بحريني (د.ب)", symbol: "د.ب" },
  { code: "JOD", label: "JOD - دينار أردني (د.أ)", symbol: "د.أ" },
];

let systemCurrencies = $state<CurrencyItem[]>(fallbackCurrencies);
let isLoadingCurrencies = $state(false);

onMount(async () => {
  try {
    isLoadingCurrencies = true;
    const dbList = await api.listCurrencies();
    if (Array.isArray(dbList) && dbList.length > 0) {
      systemCurrencies = dbList.map((c) => {
        const code = (c.code || "").toUpperCase();
        const symbolStr = c.symbol ? ` (${c.symbol})` : "";
        const nameStr = c.name ? ` - ${c.name}` : "";
        return {
          code,
          label: `${code}${nameStr}${symbolStr}`,
          symbol: c.symbol,
        };
      });
    }
  } catch (err) {
    console.warn("Could not fetch system currencies, using defaults:", err);
  } finally {
    isLoadingCurrencies = false;
  }
});

const bootstrapSchema = z
  .object({
    orgName: z.string().trim().optional(),
    orgNameAr: z.string().trim().optional(),
    orgType: z.string().min(1, "نوع المنشأة مطلوب"),
    timezone: z.string().min(1, "المنطقة الزمنية مطلوبة"),
    locale: z.string().min(1, "اللغة مطلوبة"),
    currency: z.string().min(1, "العملة مطلوبة"),
    adminFullName: z
      .string()
      .trim()
      .min(1, m.setup_bootstrap_err_admin_name_required?.() || "اسم المسؤول مطلوب"),
    adminEmail: z
      .string()
      .trim()
      .email(m.setup_bootstrap_err_admin_email_required?.() || "البريد الإلكتروني غير صالح"),
    adminPassword: z
      .string()
      .min(8, m.setup_bootstrap_err_password_length?.() || "يجب أن تكون كلمة المرور 8 أحرف على الأقل"),
    adminPasswordConfirm: z.string().min(1, "تأكيد كلمة المرور مطلوب"),
  })
  .refine((data) => Boolean(data.orgName?.trim() || data.orgNameAr?.trim()), {
    message: m.setup_bootstrap_err_org_name_required?.() || "اسم المؤسسة مطلوب",
    path: ["orgNameAr"],
  })
  .refine((data) => data.adminPassword === data.adminPasswordConfirm, {
    message: m.setup_bootstrap_err_password_mismatch?.() || "كلمتا المرور غير متطابقتين",
    path: ["adminPasswordConfirm"],
  });

type BootstrapFormValues = z.infer<typeof bootstrapSchema>;

const detectedTz = detectTimezone();
const detectedLoc = detectLocale();
const detectedCurr = detectCurrency(detectedTz);

const form = new FormController<BootstrapFormValues>({
  initialValues: {
    orgName: onboardingState.profile.orgName || "",
    orgNameAr: onboardingState.profile.orgNameAr || "",
    orgType: onboardingState.profile.institutionType || "school",
    timezone: onboardingState.profile.timezone || detectedTz,
    locale: onboardingState.profile.locale || detectedLoc,
    currency: onboardingState.profile.currency || detectedCurr,
    adminFullName: "",
    adminEmail: "",
    adminPassword: "",
    adminPasswordConfirm: "",
  },
  schema: bootstrapSchema,
});

function fieldError(field: keyof BootstrapFormValues): string | undefined {
  return form.errors[field]?.[0];
}

const selectedProfile = $derived(
  INSTITUTION_PROFILES[form.values.orgType] || INSTITUTION_PROFILES["school"],
);

const orgTypeCards = [
  {
    value: "school",
    title_ar: "مدرسة تعليمية (K-12)",
    title_en: "K-12 School",
    desc_ar: "صفوف دراسية، شعب، فصول ومراحل قياسية",
    desc_en: "Grade levels, standard classes, term exams",
  },
  {
    value: "university",
    title_ar: "جامعة / كلية أكاديمية",
    title_en: "University",
    desc_ar: "كليات، أقسام، ساعات معتمدة ومعدل GPA",
    desc_en: "Colleges, departments, credit hours & GPA",
  },
  {
    value: "institute",
    title_ar: "معهد مهني / تقني",
    title_en: "Institute",
    desc_ar: "دبلومات مهنية وتأهيل تخصصي معتمد",
    desc_en: "Vocational programs, diplomas, certifications",
  },
  {
    value: "training_center",
    title_ar: "مركز تدريب وتطوير",
    title_en: "Training Center",
    desc_ar: "دورات تدريبية، ورش عمل، وشهادات إتمام",
    desc_en: "Short courses, workshops, certificates",
  },
];

const timezones = [
  { value: "Asia/Riyadh", label: "Asia/Riyadh (GMT+3)" },
  { value: "Asia/Aden", label: "Asia/Aden (GMT+3)" },
  { value: "Asia/Dubai", label: "Asia/Dubai (GMT+4)" },
  { value: "Africa/Cairo", label: "Africa/Cairo (GMT+2)" },
  { value: "Asia/Amman", label: "Asia/Amman (GMT+3)" },
  { value: "Asia/Kuwait", label: "Asia/Kuwait (GMT+3)" },
  { value: "Asia/Qatar", label: "Asia/Qatar (GMT+3)" },
  { value: "Asia/Muscat", label: "Asia/Muscat (GMT+4)" },
  { value: "Asia/Bahrain", label: "Asia/Bahrain (GMT+3)" },
  { value: "UTC", label: "UTC (GMT+0)" },
  { value: "Europe/London", label: "Europe/London (GMT+0)" },
  { value: "America/New_York", label: "America/New_York (GMT-5)" },
];

async function handleBootstrapSubmit(e?: Event) {
  if (e) e.preventDefault();

  await form.submit(async (values) => {
    const finalOrgName = values.orgName?.trim() || values.orgNameAr?.trim() || "Orbis Organization";
    
    const result = await api.bootstrapOrganization({
      orgName: finalOrgName,
      orgNameAr: values.orgNameAr?.trim() || undefined,
      orgType: values.orgType,
      timezone: values.timezone,
      locale: values.locale,
      currency: values.currency,
      adminFullName: values.adminFullName.trim(),
      adminEmail: values.adminEmail.trim(),
      adminPassword: values.adminPassword,
    });

    // Auto-authenticate session
    await authStore.setSession({
      token: result.token,
      user: {
        id: result.admin_user_id,
        org_id: result.org_id,
        person_id: result.admin_user_id,
        email: values.adminEmail.trim(),
        full_name: values.adminFullName.trim(),
        full_name_ar: values.adminFullName.trim(),
        status: "active",
        is_email_verified: true,
        is_phone_verified: false,
        is_mfa_enabled: false,
      },
      effective_context: {
        effective_role: "super_admin",
        roles: ["super_admin"],
        permissions: ["*"],
        dependents: [],
      },
    });

    // Sync profile state with freshly bootstrapped parameters
    onboardingState.profile.orgName = finalOrgName;
    onboardingState.profile.orgNameAr = values.orgNameAr?.trim() || "";
    onboardingState.profile.currency = values.currency;
    onboardingState.profile.timezone = values.timezone;
    onboardingState.profile.locale = values.locale;
    onboardingState.selectInstitutionType(values.orgType);

    await orgContext.load();

    // Advance to academic configuration step!
    onboardingState.nextStep();
  });
}
</script>

<div class="w-full h-full flex flex-col min-h-0 justify-between select-none animate-in fade-in duration-300">

  <!-- Main Content Form Area -->
  <form onsubmit={handleBootstrapSubmit} class="flex-1 min-h-0 w-full flex flex-col justify-center overflow-y-auto px-1 py-1">
    
    {#if form.formError}
      <div class="mb-2.5 flex items-center gap-2 p-2.5 rounded-xl bg-destructive/10 border border-destructive/20 text-destructive text-xs animate-in fade-in shrink-0">
        <InfoCircle class="w-4 h-4 shrink-0" />
        <span class="font-medium">{form.formError}</span>
      </div>
    {/if}

    <div class="w-full grid grid-cols-1 lg:grid-cols-12 gap-3.5 text-start items-stretch">
      
      <!-- Left Column: Identity & Admin Account (7 cols) -->
      <div class="lg:col-span-7 flex flex-col justify-between gap-3">
        
        <!-- Card 1: هوية وإعدادات المنشأة -->
        <div class="bg-card border border-border/70 rounded-2xl p-4 flex flex-col justify-between shadow-2xs space-y-3">
          <div class="flex items-center justify-between border-b border-border/50 pb-2">
            <h3 class="text-xs font-bold text-foreground">
              {l("هوية وإعدادات المنشأة", "Organization Identity & Regional Settings")}
            </h3>
            <span class="text-[10px] font-mono font-bold text-primary bg-primary/10 px-2 py-0.5 rounded-full">
              {form.values.currency}
            </span>
          </div>

          <div class="space-y-2.5 flex-1 flex flex-col justify-center">
            <!-- Arabic & English Names Grid -->
            <div class="grid grid-cols-1 sm:grid-cols-2 gap-2.5">
              <FormField
                id="orgNameAr"
                label={m.setup_bootstrap_org_name_ar_label?.() || "اسم المؤسسة (بالعربي)"}
                error={fieldError("orgNameAr")}
                required
              >
                {#snippet children()}
                  <Input
                    id="orgNameAr"
                    type="text"
                    bind:value={form.values.orgNameAr}
                    disabled={form.isSubmitting}
                    class="h-8.5 text-xs font-bold font-sans bg-background text-right"
                    placeholder="مثال: أكاديمية النور التعليمية"
                  />
                {/snippet}
              </FormField>

              <FormField
                id="orgName"
                label={m.setup_bootstrap_org_name_label?.() || "اسم المؤسسة (بالإنجليزي)"}
                error={fieldError("orgName")}
              >
                {#snippet children()}
                  <Input
                    id="orgName"
                    type="text"
                    bind:value={form.values.orgName}
                    disabled={form.isSubmitting}
                    class="h-8.5 text-xs font-sans bg-background text-left"
                    style="direction: ltr;"
                    placeholder="e.g. Al-Noor Academy"
                  />
                {/snippet}
              </FormField>
            </div>

            <!-- Currency & Timezone Grid -->
            <div class="grid grid-cols-1 sm:grid-cols-2 gap-2.5">
              <FormField
                id="currency"
                label={m.setup_bootstrap_currency_label?.() || "العملة الأساسية"}
                error={fieldError("currency")}
                required
              >
                {#snippet children()}
                  <Select
                    type="single"
                    value={form.values.currency}
                    onValueChange={(v: any) => { if (v) form.values.currency = v; }}
                    disabled={form.isSubmitting || isLoadingCurrencies}
                  >
                    <SelectTrigger id="currency" class="w-full h-8.5 text-xs font-sans bg-background">
                      {systemCurrencies.find(c => c.code === form.values.currency)?.label || form.values.currency}
                    </SelectTrigger>
                    <SelectContent class="max-h-56">
                      {#each systemCurrencies as c}
                        <SelectItem value={c.code} label={c.label} class="text-xs font-sans">
                          {c.label}
                        </SelectItem>
                      {/each}
                    </SelectContent>
                  </Select>
                {/snippet}
              </FormField>

              <FormField
                id="timezone"
                label={m.setup_bootstrap_timezone_label?.() || "المنطقة الزمنية"}
                error={fieldError("timezone")}
                required
              >
                {#snippet children()}
                  <Select
                    type="single"
                    value={form.values.timezone}
                    onValueChange={(v: any) => { if (v) form.values.timezone = v; }}
                    disabled={form.isSubmitting}
                  >
                    <SelectTrigger id="timezone" class="w-full h-8.5 text-xs font-sans bg-background">
                      {timezones.find(t => t.value === form.values.timezone)?.label || form.values.timezone}
                    </SelectTrigger>
                    <SelectContent class="max-h-56">
                      {#each timezones as tz}
                        <SelectItem value={tz.value} label={tz.label} class="text-xs font-sans">
                          {tz.label}
                        </SelectItem>
                      {/each}
                    </SelectContent>
                  </Select>
                {/snippet}
              </FormField>
            </div>
          </div>
        </div>

        <!-- Card 2: حساب مسؤول النظام الأعلى -->
        <div class="bg-card border border-border/70 rounded-2xl p-4 flex flex-col justify-between shadow-2xs space-y-3">
          <div class="flex items-center justify-between border-b border-border/50 pb-2">
            <h3 class="text-xs font-bold text-foreground">
              {l("حساب مسؤول النظام (Super Administrator)", "System Administrator Profile")}
            </h3>
            <span class="text-[9.5px] font-mono font-bold text-muted-foreground bg-muted/60 px-2 py-0.5 rounded-full">
              Root
            </span>
          </div>

          <div class="space-y-2.5 flex-1 flex flex-col justify-center">
            <!-- Full Name & Email Grid -->
            <div class="grid grid-cols-1 sm:grid-cols-2 gap-2.5">
              <FormField
                id="adminFullName"
                label={m.setup_bootstrap_admin_name_label?.() || "اسم المدير الرباعي"}
                error={fieldError("adminFullName")}
                required
              >
                {#snippet children()}
                  <Input
                    id="adminFullName"
                    type="text"
                    bind:value={form.values.adminFullName}
                    disabled={form.isSubmitting}
                    class="h-8.5 text-xs font-bold font-sans bg-background"
                    placeholder={l("الاسم الكامل للمدير المسؤول", "Administrator Full Name")}
                  />
                {/snippet}
              </FormField>

              <FormField
                id="adminEmail"
                label={m.setup_bootstrap_admin_email_label?.() || "البريد الإلكتروني الرسمي"}
                error={fieldError("adminEmail")}
                required
              >
                {#snippet children()}
                  <Input
                    id="adminEmail"
                    type="email"
                    bind:value={form.values.adminEmail}
                    disabled={form.isSubmitting}
                    class="h-8.5 text-xs font-mono bg-background text-left"
                    placeholder="admin@school.edu"
                    style="direction: ltr;"
                  />
                {/snippet}
              </FormField>
            </div>

            <!-- Password & Confirm Password Grid with Toggle -->
            <div class="grid grid-cols-1 sm:grid-cols-2 gap-2.5">
              <FormField
                id="adminPassword"
                label={m.setup_bootstrap_admin_password_label?.() || "كلمة المرور"}
                error={fieldError("adminPassword")}
                required
              >
                {#snippet children()}
                  <div class="relative">
                    <Input
                      id="adminPassword"
                      type={showPassword ? "text" : "password"}
                      bind:value={form.values.adminPassword}
                      disabled={form.isSubmitting}
                      class="h-8.5 text-xs font-mono bg-background text-left pe-7"
                      placeholder="••••••••"
                      style="direction: ltr;"
                    />
                    <button
                      type="button"
                      tabindex="-1"
                      class="absolute top-1/2 -translate-y-1/2 end-2 text-muted-foreground hover:text-foreground cursor-pointer focus:outline-none"
                      onclick={() => (showPassword = !showPassword)}
                      aria-label="Toggle password visibility"
                    >
                      {#if showPassword}
                        <EyeSlash size={13} />
                      {:else}
                        <Eye size={13} />
                      {/if}
                    </button>
                  </div>
                {/snippet}
              </FormField>

              <FormField
                id="adminPasswordConfirm"
                label={m.setup_bootstrap_admin_password_confirm_label?.() || "تأكيد كلمة المرور"}
                error={fieldError("adminPasswordConfirm")}
                required
              >
                {#snippet children()}
                  <div class="relative">
                    <Input
                      id="adminPasswordConfirm"
                      type={showPasswordConfirm ? "text" : "password"}
                      bind:value={form.values.adminPasswordConfirm}
                      disabled={form.isSubmitting}
                      class="h-8.5 text-xs font-mono bg-background text-left pe-7"
                      placeholder="••••••••"
                      style="direction: ltr;"
                    />
                    <button
                      type="button"
                      tabindex="-1"
                      class="absolute top-1/2 -translate-y-1/2 end-2 text-muted-foreground hover:text-foreground cursor-pointer focus:outline-none"
                      onclick={() => (showPasswordConfirm = !showPasswordConfirm)}
                      aria-label="Toggle password confirm visibility"
                    >
                      {#if showPasswordConfirm}
                        <EyeSlash size={13} />
                      {:else}
                        <Eye size={13} />
                      {/if}
                    </button>
                  </div>
                {/snippet}
              </FormField>
            </div>
          </div>
        </div>

      </div>

      <!-- Right Column: Institution Blueprint & Terminology (5 cols) -->
      <div class="lg:col-span-5 bg-card border border-border/70 rounded-2xl p-4 flex flex-col justify-between shadow-2xs space-y-3">
        <div class="flex items-center justify-between border-b border-border/50 pb-2 shrink-0">
          <h3 class="text-xs font-bold text-foreground">
            {m.setup_bootstrap_org_type_label?.() || "نموذج المؤسسة التعليمية"}
          </h3>
          <span class="text-[10px] text-muted-foreground font-mono">
            {l("الهيكل والمصطلحات", "Blueprint")}
          </span>
        </div>

        <!-- 2x2 Grid of Org Types -->
        <div class="grid grid-cols-1 sm:grid-cols-2 gap-2 flex-1 items-stretch">
          {#each orgTypeCards as card}
            {@const isSelected = form.values.orgType === card.value}
            <button
              type="button"
              class="cursor-pointer text-start p-2.5 rounded-xl border transition-all duration-150 flex flex-col justify-between gap-1.5 relative
                {isSelected
                  ? 'border-primary bg-primary/5 text-primary shadow-2xs ring-1 ring-primary/20'
                  : 'border-border/60 bg-card hover:border-border hover:bg-muted/20'}"
              onclick={() => { form.values.orgType = card.value; }}
            >
              <div class="flex items-center justify-between w-full">
                <span class="text-xs font-bold text-foreground leading-tight">
                  {l(card.title_ar, card.title_en)}
                </span>
                {#if isSelected}
                  <div class="size-4 rounded-full bg-primary text-primary-foreground flex items-center justify-center shrink-0">
                    <Check size={10} />
                  </div>
                {/if}
              </div>

              <p class="text-[9.5px] text-muted-foreground line-clamp-2 leading-relaxed font-sans">
                {l(card.desc_ar, card.desc_en)}
              </p>
            </button>
          {/each}
        </div>

        <!-- Active Blueprint Terminology Preview Box -->
        <div class="p-2.5 rounded-xl border border-border/60 bg-muted/20 space-y-1.5 text-start shrink-0">
          <div class="flex items-center justify-between">
            <span class="text-[10px] font-bold text-foreground">
              {l("المصطلحات المعتمدة للنموذج:", "Active Terminology:")}
            </span>
            <span class="text-[9.5px] font-mono text-primary font-bold">{selectedProfile.name}</span>
          </div>
          <div class="grid grid-cols-2 sm:grid-cols-4 gap-1.5 pt-0.5 text-xs">
            <div class="p-1 rounded-lg bg-background border border-border/40 text-center">
              <span class="text-[8.5px] text-muted-foreground block">{l("المرحلة", "Stage")}</span>
              <span class="font-bold text-foreground text-[10px] truncate block">{isRTL ? selectedProfile.terminology.stage_ar : selectedProfile.terminology.stage}</span>
            </div>
            <div class="p-1 rounded-lg bg-background border border-border/40 text-center">
              <span class="text-[8.5px] text-muted-foreground block">{l("الفصل", "Classroom")}</span>
              <span class="font-bold text-foreground text-[10px] truncate block">{isRTL ? selectedProfile.terminology.classroom_ar : selectedProfile.terminology.classroom}</span>
            </div>
            <div class="p-1 rounded-lg bg-background border border-border/40 text-center">
              <span class="text-[8.5px] text-muted-foreground block">{l("الشعبة", "Division")}</span>
              <span class="font-bold text-foreground text-[10px] truncate block">{isRTL ? selectedProfile.terminology.division_ar : selectedProfile.terminology.division}</span>
            </div>
            <div class="p-1 rounded-lg bg-background border border-border/40 text-center">
              <span class="text-[8.5px] text-muted-foreground block">{l("المادة", "Subject")}</span>
              <span class="font-bold text-foreground text-[10px] truncate block">{isRTL ? selectedProfile.terminology.subject_ar : selectedProfile.terminology.subject}</span>
            </div>
          </div>
        </div>

      </div>

    </div>
  </form>

  <!-- Minimalist Luxury Footer Capsule -->
  <div class="w-full flex items-center justify-center py-2 select-none shrink-0">
    <div class="inline-flex items-center gap-3 px-3.5 py-1.5 rounded-full bg-card/90 border border-border/80 shadow-2xs backdrop-blur-md">
      
      <Button
        type="button"
        variant="default"
        size="xs"
        disabled={form.isSubmitting}
        onclick={() => handleBootstrapSubmit()}
        class="px-5 h-7 text-xs font-bold cursor-pointer shadow-xs gap-2"
      >
        {#if form.isSubmitting}
          <Loader2 class="w-3.5 h-3.5 animate-spin" />
          <span>{m.setup_bootstrap_submitting?.() || (isRTL ? "جاري إنشاء المؤسسة..." : "Creating Organization...")}</span>
        {:else}
          <Sparkles class="w-3.5 h-3.5" />
          <span>{l("إنشاء المؤسسة ومتابعة الإعداد", "Create Organization & Continue")}</span>
          {#if isRTL}
            <ArrowLeft class="w-3.5 h-3.5" />
          {:else}
            <ArrowRight class="w-3.5 h-3.5" />
          {/if}
        {/if}
      </Button>

    </div>
  </div>

</div>
