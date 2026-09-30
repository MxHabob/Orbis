<!--
  @component BootstrapWizard
  Master orchestrator for the clean initialization lifecycle:
  Blueprint Selection → Organization Creation → Admin Account → Bootstrap Execution → Readiness Verification.
-->
<script lang="ts">
import { goto } from "$app/navigation";
import { api } from "$lib/api/client";
import type { BootstrapRequest, BootstrapResult } from "$lib/api/types";
import { authStore, systemStore } from "$lib/stores";
import { extractErrorMessage } from "$lib/api";
import { localizeHref, getLocale } from "$lib/paraglide/runtime";
import {
  type OnboardingStep,
  type OnboardingFormModel,
  INITIAL_ONBOARDING_VALUES,
} from "./types";
import StepBlueprint from "./components/step-blueprint.svelte";
import StepOrganization from "./components/step-organization.svelte";
import StepAdmin from "./components/step-admin.svelte";
import StepReview from "./components/step-review.svelte";
import {
  AcademicCap,
  ArrowLeft,
  ArrowRight,
  BuildingOffice,
  Check,
  Sparkles,
  UserBadgeCheck,
} from "@orbis/icons";
import { Button } from "@orbis/ui/button";

let currentStep = $state<OnboardingStep>("blueprint");
let form = $state<OnboardingFormModel>({ ...INITIAL_ONBOARDING_VALUES });
let errors = $state<Record<string, string>>({});
let isSubmitting = $state(false);
let isComplete = $state(false);
let bootstrapError = $state<string | null>(null);
let activeTaskIndex = $state(0);
let bootstrapResult = $state<BootstrapResult | null>(null);

const isRTL = $derived(getLocale().startsWith("ar"));

const steps: Array<{ id: OnboardingStep; titleEn: string; titleAr: string; icon: any }> = [
  { id: "blueprint", titleEn: "Blueprint", titleAr: "النموذج", icon: AcademicCap },
  { id: "organization", titleEn: "Organization", titleAr: "المنشأة", icon: BuildingOffice },
  { id: "admin", titleEn: "Administrator", titleAr: "المشرف العام", icon: UserBadgeCheck },
  { id: "review", titleEn: "Launch", titleAr: "التشغيل", icon: Sparkles },
];

const currentStepIndex = $derived(steps.findIndex((s) => s.id === currentStep));

function validateCurrentStep(): boolean {
  const newErrors: Record<string, string> = {};

  if (currentStep === "blueprint") {
    if (!form.institutionType) {
      newErrors.institutionType = isRTL ? "يرجى اختيار نموذج المؤسسة" : "Please select a blueprint";
    }
  } else if (currentStep === "organization") {
    if (!form.organizationName.trim()) {
      newErrors.organizationName = isRTL ? "اسم المنشأة مطلوب" : "Organization name is required";
    }
    if (!form.organizationSlug.trim()) {
      newErrors.organizationSlug = isRTL ? "المعرف الفريد (Slug) مطلوب" : "Organization slug is required";
    } else if (!/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(form.organizationSlug.trim())) {
      newErrors.organizationSlug = isRTL
        ? "المعرف يجب أن يحتوي على أحرف إنجليزية صغيرة وأرقام وشرطات فقط"
        : "Slug must contain only lowercase letters, numbers, and hyphens";
    }
  } else if (currentStep === "admin") {
    if (!form.adminFirstName.trim()) {
      newErrors.adminFirstName = isRTL ? "الاسم الأول مطلوب" : "First name is required";
    }
    if (!form.adminLastName.trim()) {
      newErrors.adminLastName = isRTL ? "اسم العائلة مطلوب" : "Last name is required";
    }
    if (!form.adminEmail.trim()) {
      newErrors.adminEmail = isRTL ? "البريد الإلكتروني مطلوب" : "Email address is required";
    } else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(form.adminEmail.trim())) {
      newErrors.adminEmail = isRTL ? "صيغة البريد الإلكتروني غير صحيحة" : "Invalid email format";
    }
    if (!form.adminPassword) {
      newErrors.adminPassword = isRTL ? "كلمة المرور مطلوبة" : "Password is required";
    } else if (form.adminPassword.length < 8) {
      newErrors.adminPassword = isRTL
        ? "يجب ألا تقل كلمة المرور عن 8 خانات"
        : "Password must be at least 8 characters";
    }
    if (form.adminPassword !== form.adminConfirmPassword) {
      newErrors.adminConfirmPassword = isRTL
        ? "كلمتا المرور غير متطابقتين"
        : "Passwords do not match";
    }
  }

  errors = newErrors;
  return Object.keys(newErrors).length === 0;
}

function nextStep() {
  if (!validateCurrentStep()) return;

  if (currentStepIndex < steps.length - 1) {
    currentStep = steps[currentStepIndex + 1].id;
    window.scrollTo({ top: 0, behavior: "smooth" });
  }
}

function prevStep() {
  if (currentStepIndex > 0) {
    currentStep = steps[currentStepIndex - 1].id;
    window.scrollTo({ top: 0, behavior: "smooth" });
  }
}

async function handleLaunchBootstrap() {
  if (isSubmitting || isComplete) return;

  isSubmitting = true;
  bootstrapError = null;
  activeTaskIndex = 0;

  const req: BootstrapRequest = {
    institution_type: form.institutionType,
    organization_name: form.organizationName.trim(),
    organization_name_ar: form.organizationNameAr.trim() || undefined,
    organization_slug: form.organizationSlug.trim(),
    organization_code: form.organizationCode.trim() || undefined,
    timezone: form.timezone,
    locale: form.locale,
    date_format: form.dateFormat,
    is_hijri_enabled: form.isHijriEnabled,
    base_currency_code: form.baseCurrencyCode,
    primary_color: form.primaryColor,
    admin_first_name: form.adminFirstName.trim(),
    admin_last_name: form.adminLastName.trim(),
    admin_first_name_ar: form.adminFirstNameAr.trim() || undefined,
    admin_last_name_ar: form.adminLastNameAr.trim() || undefined,
    admin_email: form.adminEmail.trim().toLowerCase(),
    admin_password: form.adminPassword,
    admin_phone: form.adminPhone.trim() || undefined,
  };

  // Animated task progress ticker
  const interval = setInterval(() => {
    if (activeTaskIndex < 4) {
      activeTaskIndex++;
    }
  }, 700);

  try {
    const result = await api.bootstrapOrganization(req);
    bootstrapResult = result;
    activeTaskIndex = 5;
    clearInterval(interval);

    // Auto authenticate as newly created super administrator
    try {
      await authStore.login(form.adminEmail.trim().toLowerCase(), form.adminPassword);
      await systemStore.loadReadiness();
    } catch (authErr) {
      console.warn("Auto-login after bootstrap deferred:", authErr);
    }

    isComplete = true;
  } catch (err) {
    clearInterval(interval);
    bootstrapError = extractErrorMessage(err);
  } finally {
    isSubmitting = false;
  }
}

function enterWorkspace() {
  const orgId = bootstrapResult?.organization_id || authStore.orgId;
  if (orgId) {
    goto(localizeHref(`/${orgId}`), { replaceState: true });
  } else {
    goto(localizeHref("/login"), { replaceState: true });
  }
}
</script>

<div class="min-h-screen bg-background text-foreground flex flex-col selection:bg-primary/20 selection:text-primary relative overflow-x-hidden">
  <!-- Subtle background glowing accents -->
  <div class="fixed inset-0 pointer-events-none overflow-hidden">
    <div class="absolute -top-40 start-1/2 -translate-x-1/2 w-[600px] h-[300px] bg-primary/10 blur-[120px] rounded-full"></div>
    <div class="absolute -bottom-40 start-1/4 w-[400px] h-[250px] bg-sky-500/5 blur-[100px] rounded-full"></div>
  </div>

  <!-- Header Bar -->
  <header class="h-16 border-b border-border/80 bg-background/80 backdrop-blur-md sticky top-0 z-30 flex items-center justify-between px-6 sm:px-10">
    <div class="flex items-center gap-3">
      <div class="size-9 rounded-xl bg-primary/10 text-primary flex items-center justify-center font-bold text-lg border border-primary/20 shadow-sm">
        O
      </div>
      <div>
        <h1 class="text-sm font-bold tracking-tight text-foreground">Orbis Enterprise</h1>
        <p class="text-[10px] text-muted-foreground font-mono">
          {isRTL ? "مساعد التهيئة والتأسيس" : "Setup & Onboarding Suite"}
        </p>
      </div>
    </div>

    <!-- Step Progress Pills -->
    <nav aria-label="Onboarding Steps" class="hidden md:flex items-center gap-1.5 bg-muted/60 p-1 rounded-xl border border-border/60">
      {#each steps as s, i}
        {@const Icon = s.icon}
        {@const isPast = i < currentStepIndex}
        {@const isCurrent = i === currentStepIndex}
        
        <div
          class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-medium transition-all {isCurrent
            ? 'bg-background text-foreground shadow-sm shadow-black/5'
            : isPast
              ? 'text-primary hover:text-primary/80'
              : 'text-muted-foreground/60'}"
        >
          <div
            class="size-4 rounded-full flex items-center justify-center text-[10px] {isPast
              ? 'bg-primary text-primary-foreground'
              : isCurrent
                ? 'bg-foreground text-background font-bold'
                : 'bg-muted-foreground/20 text-muted-foreground'}"
          >
            {#if isPast}
              <Check size={10} strokeWidth={3} />
            {:else}
              {i + 1}
            {/if}
          </div>
          <span>{isRTL ? s.titleAr : s.titleEn}</span>
        </div>
      {/each}
    </nav>

    <div class="flex items-center gap-2">
      <div class="px-2.5 py-1 rounded-lg border border-border text-[11px] font-mono text-muted-foreground bg-card/50">
        v2.0
      </div>
    </div>
  </header>

  <!-- Main Wizard Content Area -->
  <main class="flex-1 max-w-5xl w-full mx-auto px-4 sm:px-6 py-8 sm:py-12 z-10">
    {#if currentStep === "blueprint"}
      <StepBlueprint bind:selectedType={form.institutionType} {isRTL} />
    {:else if currentStep === "organization"}
      <StepOrganization bind:form {errors} {isRTL} />
    {:else if currentStep === "admin"}
      <StepAdmin bind:form {errors} {isRTL} />
    {:else if currentStep === "review"}
      <StepReview
        {form}
        {isSubmitting}
        {isComplete}
        error={bootstrapError}
        {activeTaskIndex}
        onLaunch={handleLaunchBootstrap}
        onRetry={handleLaunchBootstrap}
        onEnterWorkspace={enterWorkspace}
        {isRTL}
      />
    {/if}
  </main>

  <!-- Sticky Bottom Navigation Bar (Hidden when executing or completed) -->
  {#if !isSubmitting && !isComplete}
    <footer class="sticky bottom-0 border-t border-border/80 bg-background/90 backdrop-blur-md py-4 px-6 sm:px-10 z-20">
      <div class="max-w-5xl mx-auto flex items-center justify-between">
        <!-- Back Button -->
        {#if currentStepIndex > 0}
          <Button
            type="button"
            variant="outline"
            onclick={prevStep}
            class="h-10 px-4 text-xs font-medium"
          >
            {#if isRTL}
              <ArrowRight size={14} class="ms-1.5" />
              <span>السابق</span>
            {:else}
              <ArrowLeft size={14} class="me-1.5" />
              <span>Back</span>
            {/if}
          </Button>
        {:else}
          <div></div>
        {/if}

        <!-- Next / Continue Button -->
        {#if currentStepIndex < steps.length - 1}
          <Button
            type="button"
            onclick={nextStep}
            class="h-10 px-6 text-xs font-semibold shadow-sm"
          >
            {#if isRTL}
              <span>المتابعة إلى {steps[currentStepIndex + 1].titleAr}</span>
              <ArrowLeft size={14} class="me-1.5" />
            {:else}
              <span>Continue to {steps[currentStepIndex + 1].titleEn}</span>
              <ArrowRight size={14} class="ms-1.5" />
            {/if}
          </Button>
        {/if}
      </div>
    </footer>
  {/if}
</div>
