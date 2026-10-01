<!--
  @component BootstrapWizard
  Master orchestrator for clean organization bootstrap & initial administrator initialization.
  Orchestrates 5 steps: Mulverse Intro -> Blueprint -> Organization -> Administrator -> Launch Review.
-->
<script lang="ts">
import { onMount } from "svelte";
import { goto } from "$app/navigation";
import { api } from "$lib/api/client";
import type {
  BootstrapRequest,
  BootstrapResult,
  EnvironmentDetectionResult,
} from "$lib/api/types";
import { authStore, systemStore } from "$lib/stores";
import { extractErrorMessage } from "$lib/api";
import { localizeHref, getLocale } from "$lib/paraglide/runtime";
import {
  type OnboardingStep,
  type OnboardingFormModel,
  INITIAL_ONBOARDING_VALUES,
} from "./types";
import StepMulverseIntro from "./components/step-mulverse-intro.svelte";
import StepBlueprint from "./components/step-blueprint.svelte";
import StepOrganization from "./components/step-organization.svelte";
import StepAdmin from "./components/step-admin.svelte";
import StepReview from "./components/step-review.svelte";
import { Button } from "@orbis/ui/button";
import {
  Cube,
  Sparkles,
  QuestionMarkCircle,
} from "@orbis/icons";
    import { SupportDialog } from "../system";

let currentStep = $state<OnboardingStep>("mulverse");
let form = $state<OnboardingFormModel>({ ...INITIAL_ONBOARDING_VALUES });
let errors = $state<Record<string, string>>({});
let isSubmitting = $state(false);
let isComplete = $state(false);
let bootstrapError = $state<string | null>(null);
let activeTaskIndex = $state(0);
let bootstrapResult = $state<BootstrapResult | null>(null);

let envData = $state<EnvironmentDetectionResult | null>(null);
let isLoadingEnv = $state(true);

let isSupportOpen = $state(false);

const isRTL = $derived(getLocale().startsWith("ar"));

const steps: Array<{ id: OnboardingStep; titleEn: string; titleAr: string }> = [
  { id: "mulverse", titleEn: "Overview", titleAr: "نظرة عامة" },
  { id: "blueprint", titleEn: "Blueprint", titleAr: "النموذج" },
  { id: "organization", titleEn: "Organization", titleAr: "المؤسسة" },
  { id: "admin", titleEn: "Administrator", titleAr: "المشرف" },
  { id: "review", titleEn: "Launch", titleAr: "التشغيل" },
];

const currentStepIndex = $derived(steps.findIndex((s) => s.id === currentStep));

async function loadEnvironment() {
  isLoadingEnv = true;
  try {
    const data = await api.bootstrapDetectEnvironment();
    envData = data;
    form.countryCode = data.country_code || "SA";
    form.timezone = data.timezone || "Asia/Riyadh";
    form.baseCurrencyCode = data.currency_code || "SAR";
    form.isHijriEnabled = data.is_hijri_recommended ?? true;
    form.dateFormat = data.date_format || "YYYY-MM-DD";
  } catch (err) {
    console.error("Failed to detect environment:", err);
  } finally {
    isLoadingEnv = false;
  }
}

onMount(() => {
  loadEnvironment();
});

function validateCurrentStep(): boolean {
  const newErrors: Record<string, string> = {};

  if (currentStep === "mulverse") {
    // Pure intro step, always valid
    return true;
  } else if (currentStep === "blueprint") {
    if (!form.institutionType) {
      newErrors.institutionType = isRTL ? "يرجى اختيار نموذج المؤسسة" : "Please select a blueprint";
    }
  } else if (currentStep === "organization") {
    if (!form.organizationName.trim()) {
      newErrors.organizationName = isRTL ? "اسم المنشأة بالإنجليزية مطلوب" : "Organization name in English is required";
    }
    if (!form.organizationNameAr.trim()) {
      newErrors.organizationNameAr = isRTL ? "اسم المنشأة بالعربية مطلوب" : "Organization name in Arabic is required";
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
    organization_code: form.organizationCode.trim() || undefined,
    educational_track: form.educationalTrack,
    organization_size: form.organizationSize,
    country_code: form.countryCode,
    city: form.city.trim() || undefined,
    phone: form.phone.trim() || undefined,
    website: form.website.trim() || undefined,
    tax_id: form.taxId.trim() || undefined,
    license_number: form.licenseNumber.trim() || undefined,
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

  const interval = setInterval(() => {
    if (activeTaskIndex < 4) {
      activeTaskIndex++;
    }
  }, 650);

  try {
    const result = await api.bootstrapOrganization(req);
    bootstrapResult = result;
    activeTaskIndex = 5;
    clearInterval(interval);

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

<div class="min-h-screen bg-background text-foreground flex flex-col justify-between selection:bg-primary/20 selection:text-primary font-sans">
  <!-- Minimal Header Bar -->
  <header class="h-14 border-b border-border/50 bg-background/95 backdrop-blur-md sticky top-0 z-30 flex items-center justify-between px-4 sm:px-8">
    <div class="flex items-center gap-3">
      <div class="size-7 rounded-lg bg-primary/10 border border-primary/25 flex items-center justify-center text-primary">
        <Cube size={16} />
      </div>
      <div class="flex items-center gap-2">
        <span class="text-sm font-bold tracking-tight text-foreground">Orbis</span>
        <span class="text-xs text-muted-foreground">•</span>
        <span class="text-xs text-muted-foreground font-medium">
          {isRTL ? "المرشد الذكي لتأسيس المنشأة" : "Autonomous Setup"}
        </span>
      </div>
    </div>

    <!-- Step Progress Pills -->
    <nav aria-label="Onboarding Steps" class="hidden md:flex items-center gap-1 bg-muted/40 p-1 rounded-lg border border-border/40">
      {#each steps as s, i}
        {@const isPast = i < currentStepIndex}
        {@const isCurrent = i === currentStepIndex}

        <div
          class="flex items-center gap-1.5 px-2.5 py-1 rounded-md text-xs transition-colors {isCurrent
            ? 'bg-background text-foreground shadow-xs font-medium'
            : isPast
              ? 'text-foreground/80'
              : 'text-muted-foreground/50'}"
        >
          <span class="size-3.5 rounded-full flex items-center justify-center text-[10px] font-mono {isCurrent
            ? 'bg-foreground text-background font-bold'
            : isPast
              ? 'bg-muted-foreground/20 text-foreground'
              : 'bg-muted-foreground/10 text-muted-foreground'}">
            {i + 1}
          </span>
          <span>{isRTL ? s.titleAr : s.titleEn}</span>
        </div>
      {/each}
    </nav>

    <!-- Top System Dialog Triggers -->
    <div class="flex items-center gap-1.5">
      <Button
        variant="ghost"
        size="sm"
        onclick={() => (isSupportOpen = true)}
        class="h-8 text-xs font-normal gap-1.5 px-2.5 text-muted-foreground hover:text-foreground"
      >
        <QuestionMarkCircle size={14} />
        <span class="hidden sm:inline">{isRTL ? "المساعدة" : "Help"}</span>
      </Button>
    </div>
  </header>

  <!-- Main Wizard Content Area -->
  <main class="flex-1 max-w-4xl w-full mx-auto px-4 sm:px-6 py-6 sm:py-8">
    {#if currentStep === "mulverse"}
      <StepMulverseIntro {envData} {isLoadingEnv} {isRTL} />
    {:else if currentStep === "blueprint"}
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
        onEnterWorkspace={enterWorkspace}
        {isRTL}
      />
    {/if}
  </main>

  <!-- Clean Bottom Navigation Bar (Hidden when executing or completed) -->
  {#if !isSubmitting && !isComplete}
    <footer class="sticky bottom-0 border-t border-border/50 bg-background/95 backdrop-blur-md py-3.5 px-6 sm:px-10 z-20">
      <div class="max-w-4xl mx-auto flex items-center justify-between">
        {#if currentStepIndex > 0}
          <Button
            type="button"
            variant="outline"
            size="sm"
            onclick={prevStep}
            class="h-9 px-5 text-xs font-normal"
          >
            <span>{isRTL ? "السابق" : "Back"}</span>
          </Button>
        {:else}
          <div></div>
        {/if}

        {#if currentStepIndex < steps.length - 1}
          <Button
            type="button"
            variant="default"
            size="sm"
            onclick={nextStep}
            class="h-9 px-6 text-xs font-medium gap-1.5 shadow-sm shadow-primary/20"
          >
            <span>{isRTL ? "المتابعة" : "Continue"}</span>
          </Button>
        {/if}
      </div>
    </footer>
  {/if}
</div>


<SupportDialog bind:open={isSupportOpen} />
