<!--
  Organization Bootstrap Screen — Create the first organization and super admin.
  Spacious, high-efficiency widescreen layout utilizing screen real estate with dual-column symmetry.
-->
<script lang="ts">
import { goto } from "$app/navigation";
import { api } from "$lib/api/client";
import { authStore } from "$lib/stores";
import { localizeHref, getLocale } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { FormField } from "@orbis/ui";
import { Logo } from "@orbis/ui/logo";
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
  ShieldCheck,
  School,
  GraduationCap,
  BookOpen,
  Briefcase,
  CheckCircle2,
  Building2,
  UserCheck,
  Sparkles,
  KeyRound,
  Lock,
  Globe2,
  Coins,
  Check,
} from "@lucide/svelte";
import { z } from "zod";
import { FormController } from "$lib/shared/forms/form-controller.svelte";
import { l } from "$lib/core/i18n";

const isRTL = $derived(getLocale().startsWith("ar"));

let step = $state(1); // 1 = Organization Details, 2 = Admin Account

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

const form = new FormController<BootstrapFormValues>({
  initialValues: {
    orgName: "",
    orgNameAr: "",
    orgType: "school",
    timezone: "Asia/Riyadh",
    locale: "ar",
    currency: "SAR",
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
    title_en: "School",
    desc_ar: "تعليم أساسي وثانوي مع فترات وفصول قياسية",
    desc_en: "K-12 school with standard periods & classes",
    tags_ar: ["7 حصص يومية", "حتى 3 ثانوي", "قاعات ومعامل"],
    tags_en: ["7 Daily Periods", "Up to Grade 12", "Labs & Rooms"],
  },
  {
    value: "university",
    icon: GraduationCap,
    title_ar: "جامعة / كلية",
    title_en: "University",
    desc_ar: "تعليم عالي ومستويات أكاديمية وقاعات محاضرات",
    desc_en: "Higher education, academic levels & halls",
    tags_ar: ["ساعات معتمدة", "مدرجات ومختبرات", "أقسام وكليات"],
    tags_en: ["Credit Hours", "Lecture Halls", "Colleges & Depts"],
  },
  {
    value: "institute",
    icon: BookOpen,
    title_ar: "معهد تدريبي / لغات",
    title_en: "Institute",
    desc_ar: "مستويات تدريبية وشهادات مهنية متخصصة",
    desc_en: "Vocational modules & professional courses",
    tags_ar: ["مستويات تخصصية", "معامل لغات", "شهادات إتمام"],
    tags_en: ["Skill Levels", "Language Labs", "Certificates"],
  },
  {
    value: "training_center",
    icon: Briefcase,
    title_ar: "مركز تدريب وتطوير",
    title_en: "Training Center",
    desc_ar: "معسكرات مكثفة وبرامج احترافية قصيرة",
    desc_en: "Bootcamps & executive short programs",
    tags_ar: ["معسكرات مكثفة", "ورش عمل تفاعلية", "قاعات افتراضية"],
    tags_en: ["Bootcamps", "Workshops", "Virtual Rooms"],
  },
];

const currencies = [
  { code: "SAR", label: "Saudi Riyal (SAR)" },
  { code: "YER", label: "Yemeni Rial (YER)" },
  { code: "AED", label: "UAE Dirham (AED)" },
  { code: "EGP", label: "Egyptian Pound (EGP)" },
  { code: "KWD", label: "Kuwaiti Dinar (KWD)" },
  { code: "QAR", label: "Qatari Riyal (QAR)" },
  { code: "USD", label: "US Dollar (USD)" },
  { code: "EUR", label: "Euro (EUR)" },
  { code: "GBP", label: "British Pound (GBP)" },
];

const timezones = [
  { value: "Asia/Riyadh", label: "Riyadh (UTC+3)" },
  { value: "Asia/Aden", label: "Aden / Sana'a (UTC+3)" },
  { value: "Asia/Dubai", label: "Dubai (UTC+4)" },
  { value: "Africa/Cairo", label: "Cairo (UTC+2)" },
  { value: "Europe/London", label: "London (UTC+0)" },
  { value: "America/New_York", label: "New York (UTC-5)" },
];

const isPasswordLengthValid = $derived((form.values.adminPassword?.length || 0) >= 8);
const isPasswordMatch = $derived(
  form.values.adminPassword &&
  form.values.adminPassword === form.values.adminPasswordConfirm
);

function handleNext() {
  form.errors = {};
  form.formError = undefined;
  if (!form.values.orgName.trim() && !form.values.orgNameAr?.trim()) {
    form.errors.orgName = [m.setup_bootstrap_err_org_name_required()];
    return;
  }
  step = 2;
}

function handleBack() {
  form.formError = undefined;
  step = 1;
}

async function handleBootstrap(e: SubmitEvent | MouseEvent) {
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

    // Auto-authenticate session with effective context
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

    goto(localizeHref("/onboarding"), { replaceState: true });
  });
}
</script>

<svelte:head>
  <title>{m.setup_bootstrap_page_title()}</title>
</svelte:head>

<div class="w-full max-w-6xl 2xl:max-w-7xl mx-auto p-1 sm:p-2 animate-in fade-in duration-300">
  <div class="rounded-2xl sm:rounded-3xl border border-border/80 bg-card/95 shadow-2xl backdrop-blur-xl overflow-hidden flex flex-col justify-between">
    
    <!-- Top Header Bar with Branding & Step Pills -->
    <div class="flex items-center justify-between px-6 sm:px-8 py-3.5 border-b border-border/60 bg-muted/15">
      <div class="flex items-center gap-3">
        <div class="size-8 rounded-xl bg-primary/10 border border-primary/20 flex items-center justify-center">
          <Logo class="size-5 text-primary" dotClass="fill-primary" />
        </div>
        <div class="text-start">
          <h1 class="text-sm sm:text-base font-bold text-foreground leading-none">
            {m.setup_bootstrap_title()}
          </h1>
          <p class="text-[11px] text-muted-foreground mt-0.5 font-medium">
            {step === 1 ? m.setup_bootstrap_step_1_desc() : m.setup_bootstrap_step_2_desc()}
          </p>
        </div>
      </div>

      <!-- Luxury Step Counter & Capsule Indicators -->
      <div class="flex items-center gap-3">
        <div class="flex items-center gap-1.5 px-3.5 py-1.5 rounded-full bg-background/80 border border-border/70 shadow-2xs text-xs font-semibold">
          {#if step === 1}
            <Building2 size={13} class="text-primary" />
            <span class="text-foreground">{l("بيانات المنظمة", "Organization")}</span>
          {:else}
            <UserCheck size={13} class="text-primary" />
            <span class="text-foreground">{l("حساب المدير", "Administrator")}</span>
          {/if}
          <div class="flex items-center gap-1 ms-1">
            <span class="size-1.5 rounded-full {step === 1 ? 'bg-primary' : 'bg-primary/40'}"></span>
            <span class="size-1.5 rounded-full {step === 2 ? 'bg-primary' : 'bg-muted-foreground/30'}"></span>
          </div>
          <span class="text-muted-foreground font-mono text-[11px] ms-0.5">0{step} / 02</span>
        </div>
      </div>
    </div>

    <!-- Main Content Form Area -->
    <form onsubmit={handleBootstrap} class="p-5 sm:p-7 lg:p-8 flex flex-col justify-between space-y-5">
      <div>
        {#if form.formError}
          <div class="mb-4 flex items-center gap-2.5 p-3.5 rounded-xl bg-destructive/10 border border-destructive/20 text-destructive text-xs">
            <AlertCircle class="w-4 h-4 shrink-0" />
            <span>{form.formError}</span>
          </div>
        {/if}

        <!-- STEP 1: Organization Details (Widescreen 2-Column Grid) -->
        {#if step === 1}
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
                          <SelectItem value={c.code} class="text-xs">{c.label}</SelectItem>
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
                        {timezones.find(tz => tz.value === form.values.timezone)?.label || form.values.timezone}
                      </SelectTrigger>
                      <SelectContent class="max-h-56">
                        {#each timezones as tz}
                          <SelectItem value={tz.value} class="text-xs">{tz.label}</SelectItem>
                        {/each}
                      </SelectContent>
                    </Select>
                  {/snippet}
                </FormField>
              </div>

              <!-- Live Configuration Note -->
              <div class="flex items-start gap-2.5 p-3 rounded-xl bg-primary/5 border border-primary/15 text-[11px] text-muted-foreground leading-relaxed">
                <Sparkles size={14} class="text-primary shrink-0 mt-0.5" />
                <span>
                  {l(
                    "يتم ضبط المصطلحات والمراحل والحصص الدراسية والقاعات تلقائياً وفورياً بمجرد اختيار نوع المؤسسة.",
                    "Terminology, stages, time slots, and rooms will automatically calibrate based on selected institution type."
                  )}
                </span>
              </div>
            </div>

            <!-- Right Side: Organization Classification 2x2 Grid (7 Cols) -->
            <div class="lg:col-span-7 space-y-3">
              <div class="flex items-center justify-between pb-1 border-b border-border/40">
                <div class="flex items-center gap-2">
                  <Sparkles size={15} class="text-primary" />
                  <h3 class="font-bold text-xs sm:text-sm text-foreground">
                    {m.setup_bootstrap_org_type_label()}
                  </h3>
                </div>
                <span class="text-[11px] text-muted-foreground font-medium">
                  {l("حدد النموذج الأنسب لمنشأتك", "Select institutional structure")}
                </span>
              </div>

              <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
                {#each orgTypeCards as ot}
                  {@const isSelected = form.values.orgType === ot.value}
                  <button
                    type="button"
                    class="p-4 rounded-2xl border text-start flex flex-col justify-between gap-3 transition-all duration-200 cursor-pointer shadow-2xs hover:shadow-xs group {isSelected ? 'border-primary ring-2 ring-primary/20 bg-primary/5' : 'border-border/70 bg-card hover:bg-muted/15'}"
                    onclick={() => { form.values.orgType = ot.value; }}
                  >
                    <div class="flex items-center justify-between w-full">
                      <div class="size-8 rounded-xl flex items-center justify-center {isSelected ? 'bg-primary text-primary-foreground shadow-sm' : 'bg-muted text-foreground group-hover:bg-primary/10 group-hover:text-primary'} transition-colors">
                        <ot.icon size={16} />
                      </div>
                      {#if isSelected}
                        <Badge variant="default" class="text-[10px] h-5 gap-1 px-2 font-semibold">
                          <CheckCircle2 size={11} />
                          <span>{l("مفعل", "Selected")}</span>
                        </Badge>
                      {/if}
                    </div>

                    <div class="space-y-1">
                      <h4 class="font-bold text-xs sm:text-sm text-foreground {isSelected ? 'text-primary' : ''}">
                        {l(ot.title_ar, ot.title_en)}
                      </h4>
                      <p class="text-[11px] text-muted-foreground leading-relaxed line-clamp-2">
                        {l(ot.desc_ar, ot.desc_en)}
                      </p>
                    </div>

                    <!-- Micro Feature Tags -->
                    <div class="flex flex-wrap gap-1.5 pt-1 border-t border-border/30">
                      {#each (isRTL ? ot.tags_ar : ot.tags_en) as tag}
                        <span class="text-[10px] font-medium px-2 py-0.5 rounded-md {isSelected ? 'bg-primary/10 text-primary' : 'bg-muted/40 text-muted-foreground'}">
                          {tag}
                        </span>
                      {/each}
                    </div>
                  </button>
                {/each}
              </div>
            </div>

          </div>

        <!-- STEP 2: Super Admin Account (Widescreen 2-Column Grid) -->
        {:else}
          <div class="grid grid-cols-1 lg:grid-cols-12 gap-6 text-start items-start">
            
            <!-- Left Side: Administrator Identity & Role Card (6 Cols) -->
            <div class="lg:col-span-6 space-y-4 rounded-2xl bg-muted/10 border border-border/60 p-5 sm:p-6">
              <div class="flex items-center gap-2 pb-1 border-b border-border/40">
                <UserCheck size={16} class="text-primary" />
                <h3 class="font-bold text-xs sm:text-sm text-foreground">
                  {l("بيانات المدير المسؤول", "Super Administrator Profile")}
                </h3>
              </div>

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
                    class="h-10 text-xs bg-background border-input font-medium"
                    placeholder="مثال: عبدالله المنصور"
                  />
                {/snippet}
              </FormField>

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
                    class="h-10 text-xs bg-background border-input text-left"
                    style="direction: ltr;"
                    placeholder="admin@institution.edu"
                  />
                {/snippet}
              </FormField>

              <!-- Super Admin Role Explanation Card -->
              <div class="p-3.5 rounded-xl bg-card border border-border/70 space-y-2">
                <div class="flex items-center justify-between">
                  <div class="flex items-center gap-2 text-xs font-bold text-foreground">
                    <ShieldCheck size={15} class="text-primary" />
                    <span>{l("المدير العام الأعلى (Super Admin)", "Super Administrator")}</span>
                  </div>
                  <Badge variant="outline" class="text-[10px] bg-primary/5 text-primary border-primary/20">
                    {l("صلاحيات كاملة", "Root Authority")}
                  </Badge>
                </div>
                <p class="text-[11px] text-muted-foreground leading-relaxed">
                  {l(
                    "يمتلك هذا الحساب الصلاحيات الجذرية لإدارة كافة المنظومة، وتعيين الكادر الإداري، والتحكم بالهيكل المالي والأكاديمي.",
                    "This account holds root administrative privileges for full organizational structure, staff provisioning, and academic controls."
                  )}
                </p>
              </div>
            </div>

            <!-- Right Side: Security & Password Setup (6 Cols) -->
            <div class="lg:col-span-6 space-y-4 rounded-2xl bg-muted/10 border border-border/60 p-5 sm:p-6">
              <div class="flex items-center gap-2 pb-1 border-b border-border/40">
                <KeyRound size={16} class="text-primary" />
                <h3 class="font-bold text-xs sm:text-sm text-foreground">
                  {l("كلمة المرور والأمان", "Security & Password Setup")}
                </h3>
              </div>

              <div class="space-y-4">
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
                      class="h-10 text-xs bg-background border-input"
                      placeholder="••••••••"
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
                      class="h-10 text-xs bg-background border-input"
                      placeholder="••••••••"
                    />
                  {/snippet}
                </FormField>
              </div>

              <!-- Real-time Security Checklist -->
              <div class="p-3.5 rounded-xl bg-card border border-border/70 space-y-2">
                <div class="text-[11px] font-bold text-foreground">
                  {l("متطلبات الحماية الموصى بها", "Recommended Security Standards")}
                </div>
                <div class="space-y-1.5 text-[11px]">
                  <div class="flex items-center gap-2 {isPasswordLengthValid ? 'text-emerald-500 font-semibold' : 'text-muted-foreground'}">
                    {#if isPasswordLengthValid}
                      <Check size={13} class="text-emerald-500 shrink-0" />
                    {:else}
                      <div class="size-3 rounded-full border border-muted-foreground/40 shrink-0"></div>
                    {/if}
                    <span>{l("8 أحرف على الأقل", "Minimum 8 characters")}</span>
                  </div>

                  <div class="flex items-center gap-2 {isPasswordMatch ? 'text-emerald-500 font-semibold' : 'text-muted-foreground'}">
                    {#if isPasswordMatch}
                      <Check size={13} class="text-emerald-500 shrink-0" />
                    {:else}
                      <div class="size-3 rounded-full border border-muted-foreground/40 shrink-0"></div>
                    {/if}
                    <span>{l("تطابق كلمتي المرور", "Passwords match perfectly")}</span>
                  </div>

                  <div class="flex items-center gap-2 text-muted-foreground">
                    <Lock size={12} class="text-primary shrink-0" />
                    <span>{l("تشفير محلي وسحابي آمن (Argon2 / SHA-256)", "Secure encryption with Argon2 & SHA-256")}</span>
                  </div>
                </div>
              </div>
            </div>

          </div>
        {/if}
      </div>

      <!-- Action Footer -->
      <div class="pt-3 border-t border-border/50 flex items-center justify-between">
        {#if step === 2}
          <Button
            type="button"
            variant="outline"
            onclick={handleBack}
            disabled={form.isSubmitting}
            class="inline-flex items-center gap-1.5 px-4 h-9 text-xs font-medium cursor-pointer border-border"
          >
            {#if isRTL}
              <ArrowRight class="w-3.5 h-3.5" />
            {:else}
              <ArrowLeft class="w-3.5 h-3.5" />
            {/if}
            <span>{m.setup_bootstrap_prev_btn()}</span>
          </Button>
        {:else}
          <div></div>
        {/if}

        {#if step === 1}
          <Button
            type="button"
            variant="default"
            onclick={handleNext}
            class="inline-flex items-center gap-2 px-6 h-9 text-xs font-semibold cursor-pointer"
          >
            <span>{m.setup_bootstrap_next_btn()}</span>
            {#if isRTL}
              <ArrowLeft class="w-3.5 h-3.5" />
            {:else}
              <ArrowRight class="w-3.5 h-3.5" />
            {/if}
          </Button>
        {:else}
          <Button
            type="submit"
            variant="default"
            disabled={form.isSubmitting}
            class="inline-flex items-center gap-2 px-7 h-9 text-xs font-semibold cursor-pointer disabled:opacity-50"
          >
            {#if form.isSubmitting}
              <Loader2 class="w-3.5 h-3.5 animate-spin" />
              <span>{m.setup_bootstrap_submitting()}</span>
            {:else}
              <ShieldCheck class="w-3.5 h-3.5" />
              <span>{m.setup_bootstrap_submit_btn()}</span>
            {/if}
          </Button>
        {/if}
      </div>
    </form>
  </div>
</div>
