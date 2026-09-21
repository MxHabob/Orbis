<!--
  Step 0: Organization & Admin Bootstrap
  Unified first-step when no organization exists in the system yet.
  Collects Organization Identity & Super Administrator credentials to create the Organization shell.
-->
<script lang="ts">
import { api } from "$lib/api/client";
import { authStore } from "$lib/stores";
import { getLocale } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { FormField } from "@orbis/ui";
import { Badge } from "@orbis/ui/badge";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import {
  Loader2,
  AlertCircle,
  ArrowRight,
  ArrowLeft,
  Building2,
  UserCheck,
  School,
  GraduationCap,
  BookOpen,
  Briefcase,
  CheckCircle2,
  Shield,
  KeyRound,
  Coins,
  Globe2,
} from "@lucide/svelte";
import { z } from "zod";
import { FormController } from "$lib/shared/forms/form-controller.svelte";
import { l } from "$lib/core/i18n";
import { detectTimezone, detectCurrency, detectLocale } from "../utils";
import { onboardingState } from "../state.svelte";
import { orgContext } from "$lib/stores/org-context.svelte";

const isRTL = $derived(getLocale().startsWith("ar"));

let stepSubTab = $state(1); // 1 = Organization Details, 2 = Admin Account

const bootstrapSchema = z.object({
  orgName: z.string().trim().min(1, m.setup_bootstrap_err_org_name_required()),
  orgNameAr: z.string().optional(),
  orgType: z.string().min(1),
  timezone: z.string().min(1),
  locale: z.string().min(1),
  currency: z.string().min(1),
  adminFullName: z.string().trim().min(1, m.setup_bootstrap_err_admin_name_required()),
  adminEmail: z.string().trim().email(m.setup_bootstrap_err_admin_email_required()),
  adminPassword: z.string().min(8, m.setup_bootstrap_err_password_length()),
  adminPasswordConfirm: z.string().min(1),
}).refine((data) => data.adminPassword === data.adminPasswordConfirm, {
  message: m.setup_bootstrap_err_password_mismatch(),
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

const orgTypeCards = [
  {
    value: "school",
    icon: School,
    title_ar: "مدرسة تعليمية",
    title_en: "K-12 School",
    desc_ar: "صفوف دراسية، مراحل تعليمية، درجات واختبارات قياسية",
    desc_en: "Grade levels, standard classes, term exams",
  },
  {
    value: "university",
    icon: GraduationCap,
    title_ar: "جامعة / كلية",
    title_en: "University",
    desc_ar: "كليات، أقسام، ساعات معتمدة ومعدل تراكمي GPA",
    desc_en: "Colleges, departments, credit hours & GPA",
  },
  {
    value: "institute",
    icon: BookOpen,
    title_ar: "معهد مهني / تقني",
    title_en: "Institute",
    desc_ar: "برامج تدريبية وتأهيل مهني ودبلومات متخصصة",
    desc_en: "Vocational programs, diplomas, certifications",
  },
  {
    value: "training_center",
    icon: Briefcase,
    title_ar: "مركز تدريب وتطوير",
    title_en: "Training Center",
    desc_ar: "دورات تدريبية، ورش عمل، وشهادات إتمام دورة",
    desc_en: "Short courses, workshops, certificates",
  },
];

const currencies = [
  { code: "SAR", label: "SAR - ريال سعودي" },
  { code: "YER", label: "YER - ريال يمني" },
  { code: "USD", label: "USD - US Dollar" },
  { code: "AED", label: "AED - درهم إماراتي" },
  { code: "EGP", label: "EGP - جنيه مصري" },
  { code: "EUR", label: "EUR - Euro" },
  { code: "KWD", label: "KWD - دينار كويتي" },
  { code: "QAR", label: "QAR - ريال قطري" },
  { code: "OMR", label: "OMR - ريال عماني" },
  { code: "BHD", label: "BHD - دينار بحريني" },
  { code: "JOD", label: "JOD - دينار أردني" },
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

function handleNextSubTab() {
  form.clearError();
  if (!form.values.orgNameAr?.trim() && !form.values.orgName.trim()) {
    form.errors.orgNameAr = [m.setup_bootstrap_err_org_name_required()];
    return;
  }
  stepSubTab = 2;
}

function handlePrevSubTab() {
  form.clearError();
  stepSubTab = 1;
}

async function handleBootstrapSubmit(e?: Event) {
  if (e) e.preventDefault();

  await form.submit(async (values) => {
    const finalOrgName = values.orgName.trim() || values.orgNameAr?.trim() || "Orbis Organization";
    
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

<div class="w-full max-w-5xl 2xl:max-w-6xl mx-auto p-1 sm:p-2 animate-in fade-in duration-300">
  <div class="rounded-2xl sm:rounded-3xl border border-border/80 bg-card/95 shadow-xl backdrop-blur-xl overflow-hidden flex flex-col justify-between">
    
    <!-- Header Bar with Step Navigation -->
    <div class="flex items-center justify-between px-6 sm:px-8 py-3.5 border-b border-border/60 bg-muted/15">
      <div class="flex items-center gap-3">
        <div class="size-8 rounded-xl bg-primary/10 border border-primary/20 flex items-center justify-center">
          <Building2 class="size-4 text-primary" />
        </div>
        <div class="text-start">
          <h2 class="text-sm sm:text-base font-bold text-foreground leading-none">
            {m.setup_bootstrap_title()}
          </h2>
          <p class="text-[11px] text-muted-foreground mt-0.5 font-medium">
            {stepSubTab === 1 ? m.setup_bootstrap_step_1_desc() : m.setup_bootstrap_step_2_desc()}
          </p>
        </div>
      </div>

      <!-- Step Indicator -->
      <div class="flex items-center gap-1.5 px-3 py-1 rounded-full bg-background/80 border border-border/70 text-xs font-semibold">
        {#if stepSubTab === 1}
          <Building2 size={13} class="text-primary" />
          <span class="text-foreground">{l("بيانات المنشأة", "Organization")}</span>
        {:else}
          <UserCheck size={13} class="text-primary" />
          <span class="text-foreground">{l("حساب المدير", "Administrator")}</span>
        {/if}
        <div class="flex items-center gap-1 ms-1">
          <span class="size-1.5 rounded-full {stepSubTab === 1 ? 'bg-primary' : 'bg-primary/40'}"></span>
          <span class="size-1.5 rounded-full {stepSubTab === 2 ? 'bg-primary' : 'bg-muted-foreground/30'}"></span>
        </div>
      </div>
    </div>

    <!-- Form Area -->
    <form onsubmit={handleBootstrapSubmit} class="p-5 sm:p-7 lg:p-8 flex flex-col justify-between space-y-5">
      <div>
        {#if form.formError}
          <div class="mb-4 flex items-center gap-2.5 p-3.5 rounded-xl bg-destructive/10 border border-destructive/20 text-destructive text-xs">
            <AlertCircle class="w-4 h-4 shrink-0" />
            <span>{form.formError}</span>
          </div>
        {/if}

        <!-- SUBTAB 1: Organization Details -->
        {#if stepSubTab === 1}
          <div class="grid grid-cols-1 lg:grid-cols-12 gap-6 text-start items-start">
            
            <!-- Left Side: Identity & Localization (5 Cols) -->
            <div class="lg:col-span-5 space-y-4 rounded-2xl bg-muted/10 border border-border/60 p-4 sm:p-5">
              <div class="flex items-center gap-2 pb-1 border-b border-border/40">
                <Building2 size={15} class="text-primary" />
                <h3 class="font-bold text-xs sm:text-sm text-foreground">
                  {l("هوية وإعدادات المنشأة", "Identity & Regional Settings")}
                </h3>
              </div>

              <!-- Arabic Name -->
              <FormField
                id="orgNameAr"
                label={m.setup_bootstrap_org_name_ar_label()}
                error={fieldError("orgNameAr")}
                required
              >
                {#snippet children()}
                  <Input
                    id="orgNameAr"
                    type="text"
                    bind:value={form.values.orgNameAr}
                    disabled={form.isSubmitting}
                    class="h-9 text-xs font-bold font-sans bg-background border-input text-right"
                    placeholder="مثال: أكاديمية النور التعليمية"
                  />
                {/snippet}
              </FormField>

              <!-- English Name -->
              <FormField
                id="orgName"
                label={m.setup_bootstrap_org_name_label()}
                error={fieldError("orgName")}
              >
                {#snippet children()}
                  <Input
                    id="orgName"
                    type="text"
                    bind:value={form.values.orgName}
                    disabled={form.isSubmitting}
                    class="h-9 text-xs font-sans bg-background border-input text-left"
                    style="direction: ltr;"
                    placeholder="e.g. Al-Noor Academy"
                  />
                {/snippet}
              </FormField>

              <!-- Currency & Timezone Grid -->
              <div class="grid grid-cols-1 sm:grid-cols-2 gap-3 pt-1">
                <FormField
                  id="currency"
                  label={m.setup_bootstrap_currency_label()}
                  error={fieldError("currency")}
                  required
                >
                  {#snippet children()}
                    <Select
                      type="single"
                      value={form.values.currency}
                      onValueChange={(v : any) => { if (v) form.values.currency = v; }}
                      disabled={form.isSubmitting}
                    >
                      <SelectTrigger id="currency" class="h-9 text-xs w-full bg-background border-input font-sans">
                        {currencies.find(c => c.code === form.values.currency)?.label || form.values.currency}
                      </SelectTrigger>
                      <SelectContent class="max-h-56">
                        {#each currencies as c}
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
                  label={m.setup_bootstrap_timezone_label()}
                  error={fieldError("timezone")}
                  required
                >
                  {#snippet children()}
                    <Select
                      type="single"
                      value={form.values.timezone}
                      onValueChange={(v : any) => { if (v) form.values.timezone = v; }}
                      disabled={form.isSubmitting}
                    >
                      <SelectTrigger id="timezone" class="h-9 text-xs w-full bg-background border-input font-sans">
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

              <div class="pt-2 text-[11px] text-muted-foreground flex items-center gap-1.5 border-t border-border/30">
                <Shield size={12} class="text-primary shrink-0" />
                <span>{l("سيتم استخدام هذه العملة كأساس لجميع العمليات المالية", "Base currency for all financial operations")}</span>
              </div>
            </div>

            <!-- Right Side: Organization Type Blueprint (7 Cols) -->
            <div class="lg:col-span-7 space-y-3">
              <div class="flex items-center justify-between pb-1 border-b border-border/40">
                <label class="text-xs sm:text-sm font-bold text-foreground">
                  {m.setup_bootstrap_org_type_label()}
                </label>
                <span class="text-[11px] text-muted-foreground">
                  {l("يحدد المصطلحات والهيكل الافتراضي", "Sets terminology & structure blueprint")}
                </span>
              </div>

              <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
                {#each orgTypeCards as card}
                  {@const Icon = card.icon}
                  <button
                    type="button"
                    class="cursor-pointer text-start p-4 rounded-2xl border-2 transition-all flex flex-col justify-between gap-3 relative {form.values.orgType === card.value ? 'border-primary bg-primary/5 shadow-sm ring-1 ring-primary/20' : 'border-border/60 bg-card hover:border-border hover:bg-muted/10'}"
                    onclick={() => { form.values.orgType = card.value; }}
                  >
                    <div class="flex items-center justify-between w-full">
                      <div class="size-9 rounded-xl flex items-center justify-center {form.values.orgType === card.value ? 'bg-primary text-primary-foreground' : 'bg-muted text-muted-foreground'}">
                        <Icon size={16} />
                      </div>
                      {#if form.values.orgType === card.value}
                        <div class="size-5 rounded-full bg-primary text-primary-foreground flex items-center justify-center">
                          <CheckCircle2 size={12} />
                        </div>
                      {/if}
                    </div>

                    <div>
                      <h4 class="text-xs font-bold text-foreground">
                        {l(card.title_ar, card.title_en)}
                      </h4>
                      <p class="text-[11px] text-muted-foreground mt-0.5 line-clamp-2 leading-relaxed">
                        {l(card.desc_ar, card.desc_en)}
                      </p>
                    </div>
                  </button>
                {/each}
              </div>
            </div>
          </div>

        <!-- SUBTAB 2: Super Admin Account -->
        {:else}
          <div class="max-w-xl mx-auto space-y-5 text-start py-2">
            <div class="p-4 sm:p-5 rounded-2xl bg-muted/10 border border-border/60 space-y-4">
              <div class="flex items-center gap-2 pb-1 border-b border-border/40">
                <UserCheck size={15} class="text-primary" />
                <h3 class="font-bold text-xs sm:text-sm text-foreground">
                  {l("بيانات المسؤول الأعلى للنظام (Super Administrator)", "System Administrator Credentials")}
                </h3>
              </div>

              <!-- Full Name -->
              <FormField
                id="adminFullName"
                label={m.setup_bootstrap_admin_name_label()}
                error={fieldError("adminFullName")}
                required
              >
                {#snippet children()}
                  <Input
                    id="adminFullName"
                    type="text"
                    bind:value={form.values.adminFullName}
                    disabled={form.isSubmitting}
                    class="h-9 text-xs font-bold font-sans bg-background border-input"
                    placeholder={l("الاسم الثلاثي أو الرباعي للمدير", "Administrator Full Name")}
                  />
                {/snippet}
              </FormField>

              <!-- Email / Username -->
              <FormField
                id="adminEmail"
                label={m.setup_bootstrap_admin_email_label()}
                error={fieldError("adminEmail")}
                required
              >
                {#snippet children()}
                  <Input
                    id="adminEmail"
                    type="email"
                    bind:value={form.values.adminEmail}
                    disabled={form.isSubmitting}
                    class="h-9 text-xs font-mono bg-background border-input"
                    placeholder="admin@school.edu"
                    style="direction: ltr;"
                  />
                {/snippet}
              </FormField>

              <!-- Password Grid -->
              <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
                <FormField
                  id="adminPassword"
                  label={m.setup_bootstrap_admin_password_label()}
                  error={fieldError("adminPassword")}
                  required
                >
                  {#snippet children()}
                    <Input
                      id="adminPassword"
                      type="password"
                      bind:value={form.values.adminPassword}
                      disabled={form.isSubmitting}
                      class="h-9 text-xs font-mono bg-background border-input"
                      placeholder="••••••••"
                      style="direction: ltr;"
                    />
                  {/snippet}
                </FormField>

                <FormField
                  id="adminPasswordConfirm"
                  label={m.setup_bootstrap_admin_password_confirm_label()}
                  error={fieldError("adminPasswordConfirm")}
                  required
                >
                  {#snippet children()}
                    <Input
                      id="adminPasswordConfirm"
                      type="password"
                      bind:value={form.values.adminPasswordConfirm}
                      disabled={form.isSubmitting}
                      class="h-9 text-xs font-mono bg-background border-input"
                      placeholder="••••••••"
                      style="direction: ltr;"
                    />
                  {/snippet}
                </FormField>
              </div>

              <div class="p-3 rounded-xl bg-primary/5 border border-primary/20 text-[11px] text-muted-foreground flex items-center gap-2">
                <KeyRound size={14} class="text-primary shrink-0" />
                <span>{l("يمتلك هذا الحساب الصلاحيات الكاملة للتحكم في كافة إعدادات المنظومة والمستخدمين", "This account has full root permissions to manage the system")}</span>
              </div>
            </div>
          </div>
        {/if}
      </div>

      <!-- Action Navigation Buttons -->
      <div class="flex items-center justify-between pt-4 border-t border-border/40">
        {#if stepSubTab === 1}
          <div></div>
          <Button
            type="button"
            variant="default"
            class="h-9 px-6 text-xs font-bold gap-2 cursor-pointer"
            onclick={handleNextSubTab}
          >
            <span>{m.setup_bootstrap_next_btn()}</span>
            {#if isRTL}
              <ArrowLeft class="size-3.5" />
            {:else}
              <ArrowRight class="size-3.5" />
            {/if}
          </Button>
        {:else}
          <Button
            type="button"
            variant="outline"
            class="h-9 px-5 text-xs font-semibold gap-2 cursor-pointer border-border"
            onclick={handlePrevSubTab}
            disabled={form.isSubmitting}
          >
            {#if isRTL}
              <ArrowRight class="size-3.5" />
            {:else}
              <ArrowLeft class="size-3.5" />
            {/if}
            <span>{m.setup_bootstrap_prev_btn()}</span>
          </Button>

          <Button
            type="submit"
            variant="default"
            class="h-9 px-6 text-xs font-bold gap-2 cursor-pointer shadow-sm shadow-primary/20"
            disabled={form.isSubmitting}
          >
            {#if form.isSubmitting}
              <Loader2 class="size-3.5 animate-spin" />
              <span>{m.setup_bootstrap_submitting()}</span>
            {:else}
              <span>{l("إنشاء المنشأة ومتابعة الإعداد", "Create Organization & Continue")}</span>
              {#if isRTL}
                <ArrowLeft class="size-3.5" />
              {:else}
                <ArrowRight class="size-3.5" />
              {/if}
            {/if}
          </Button>
        {/if}
      </div>
    </form>
  </div>
</div>
