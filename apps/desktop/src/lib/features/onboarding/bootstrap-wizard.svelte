<!--
  @component BootstrapWizard
  Master orchestrator for clean organization bootstrap, regional calibration,
  and initial administrator initialization.
  Floating minimal header & footer with 6 progressive steps:
  1. Mulverse Intro -> 2. Blueprint -> 3. Organization -> 4. Administrator -> 5. MFA (Skippable) -> 6. Launch Review.
-->
<script lang="ts">
import { onMount } from "svelte";
import { goto } from "$app/navigation";
import { api } from "$lib/api/client";
import type {
  BootstrapRequest,
  BootstrapResult,
} from "$lib/api/types";
import { authStore, systemStore } from "$lib/stores";
import { extractErrorMessage } from "$lib/api";
import { localizeHref, getLocale } from "$lib/paraglide/runtime";
import {
  type OnboardingStep,
  type OnboardingFormModel,
  INITIAL_ONBOARDING_VALUES,
} from "./types";
import OnboardingHeader from "./components/onboarding-header.svelte";
import OnboardingFooter from "./components/onboarding-footer.svelte";
import StepMulverseIntro from "./components/step-mulverse-intro.svelte";
import StepBlueprint from "./components/step-blueprint.svelte";
import StepOrganization from "./components/step-organization.svelte";
import StepAdmin from "./components/step-admin.svelte";
import StepMfa from "./components/step-mfa.svelte";
import StepReview from "./components/step-review.svelte";
import { SupportDialog } from "../system";

let currentStep = $state<OnboardingStep>("mulverse");
let form = $state<OnboardingFormModel>({ ...INITIAL_ONBOARDING_VALUES });
let errors = $state<Record<string, string>>({});
let isSubmitting = $state(false);
let isComplete = $state(false);
let bootstrapError = $state<string | null>(null);
let activeTaskIndex = $state(0);
let bootstrapResult = $state<BootstrapResult | null>(null);

let isSupportOpen = $state(false);

const isRTL = $derived(getLocale().startsWith("ar"));

const steps: Array<{ id: OnboardingStep }> = [
  { id: "mulverse" },
  { id: "blueprint" },
  { id: "organization" },
  { id: "admin" },
  { id: "mfa" },
  { id: "review" },
];

const currentStepIndex = $derived(steps.findIndex((s) => s.id === currentStep));
const canGoBack = $derived(currentStepIndex > 0);
const isSkippable = $derived(currentStep === "mfa");
const isLastStep = $derived(currentStep === "review");

// Silent background environment detection and calibration
async function loadEnvironment() {
  try {
    const data = await api.bootstrapDetectEnvironment();
    if (data) {
      form.countryCode = data.country_code || "SA";
      form.timezone = data.timezone || "Asia/Riyadh";
      form.baseCurrencyCode = data.currency_code || "SAR";
      form.isHijriEnabled = data.is_hijri_recommended ?? true;
      form.dateFormat = data.date_format || "YYYY-MM-DD";
    }
  } catch (err) {
    console.warn("Silent environment detection completed with defaults:", err);
  }
}

onMount(() => {
  loadEnvironment();
});

function validateCurrentStep(): boolean {
  const newErrors: Record<string, string> = {};

  if (currentStep === "mulverse") {
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
  } else if (currentStep === "mfa") {
    return true;
  }

  errors = newErrors;
  return Object.keys(newErrors).length === 0;
}

function scrollToTop() {
  const container = document.getElementById("onboarding-viewport");
  if (container) {
    container.scrollTo({ top: 0, behavior: "smooth" });
  } else {
    window.scrollTo({ top: 0, behavior: "smooth" });
  }
}

function nextStep() {
  if (!validateCurrentStep()) return;

  if (currentStepIndex < steps.length - 1) {
    currentStep = steps[currentStepIndex + 1].id;
    scrollToTop();
  }
}

function prevStep() {
  if (currentStepIndex > 0) {
    currentStep = steps[currentStepIndex - 1].id;
    scrollToTop();
  }
}

function skipStep() {
  if (currentStepIndex < steps.length - 1) {
    currentStep = steps[currentStepIndex + 1].id;
    scrollToTop();
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

<div
  id="onboarding-viewport"
  class="h-full w-full overflow-y-auto bg-background text-foreground flex flex-col justify-between selection:bg-primary/20 selection:text-primary font-sans relative px-4 sm:px-6 lg:px-8 py-3 space-y-4"
>
  <!-- Floating Minimal Header Bar -->
  <OnboardingHeader {currentStep} {steps} {isRTL} />

  <!-- Main Wizard Content Area with Full Screen Real Estate Utilization -->
  <main class="flex-1 w-full max-w-5xl mx-auto my-auto py-2">
    {#if currentStep === "mulverse"}
      <StepMulverseIntro {isRTL} />
    {:else if currentStep === "blueprint"}
      <StepBlueprint bind:selectedType={form.institutionType} {isRTL} />
    {:else if currentStep === "organization"}
      <StepOrganization bind:form {errors} {isRTL} />
    {:else if currentStep === "admin"}
      <StepAdmin bind:form {errors} {isRTL} />
    {:else if currentStep === "mfa"}
      <StepMfa
        adminEmail={form.adminEmail}
        orgName={form.organizationName || form.organizationNameAr}
        {isRTL}
        onMfaActivated={() => nextStep()}
        onSkip={skipStep}
      />
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

  <!-- Floating Minimal Footer Bar (Hidden when successfully completed) -->
  {#if !isComplete}
    <OnboardingFooter
      {canGoBack}
      {isSkippable}
      {isLastStep}
      {isSubmitting}
      onPrev={prevStep}
      onNext={nextStep}
      onSkip={skipStep}
      onHelp={() => (isSupportOpen = true)}
      {isRTL}
    />
  {/if}
</div>

<SupportDialog bind:open={isSupportOpen} />
