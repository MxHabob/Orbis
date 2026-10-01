<!--
  @component BootstrapWizard
  Master orchestrator for clean organization bootstrap & initial administrator initialization.
  Designed with minimalist enterprise aesthetics, design tokens, and uncluttered layout.
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

const steps: Array<{ id: OnboardingStep; titleEn: string; titleAr: string }> = [
  { id: "blueprint", titleEn: "Blueprint", titleAr: "النموذج" },
  { id: "organization", titleEn: "Organization", titleAr: "المؤسسة" },
  { id: "admin", titleEn: "Administrator", titleAr: "المشرف" },
  { id: "review", titleEn: "Launch", titleAr: "التشغيل" },
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

<div class="min-h-screen bg-background text-foreground flex flex-col justify-between selection:bg-primary/20 selection:text-primary">
  <!-- Minimal Header Bar -->
  <header class="h-14 border-b border-border/50 bg-background/95 backdrop-blur-xs sticky top-0 z-30 flex items-center justify-between px-6 sm:px-10">
    <div class="flex items-center gap-2.5">
      <span class="text-sm font-semibold tracking-tight text-foreground">Orbis</span>
      <span class="text-xs text-muted-foreground">•</span>
      <span class="text-xs text-muted-foreground">
        {isRTL ? "تهيئة المنشأة" : "Setup"}
      </span>
    </div>

    <!-- Step Progress Pills -->
    <nav aria-label="Onboarding Steps" class="flex items-center gap-1 bg-muted/40 p-1 rounded-lg border border-border/40">
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
          <span class="hidden sm:inline">{isRTL ? s.titleAr : s.titleEn}</span>
        </div>
      {/each}
    </nav>
  </header>

  <!-- Main Wizard Content Area -->
  <main class="flex-1 max-w-4xl w-full mx-auto px-4 sm:px-6 py-8 sm:py-10">
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

  <!-- Clean Bottom Navigation Bar (Hidden when executing or completed) -->
  {#if !isSubmitting && !isComplete}
    <footer class="sticky bottom-0 border-t border-border/50 bg-background/95 backdrop-blur-xs py-3.5 px-6 sm:px-10 z-20">
      <div class="max-w-4xl mx-auto flex items-center justify-between">
        {#if currentStepIndex > 0}
          <Button
            type="button"
            variant="outline"
            size="sm"
            onclick={prevStep}
            class="h-9 px-4 text-xs font-normal"
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
            class="h-9 px-5 text-xs font-medium"
          >
            <span>{isRTL ? "المتابعة" : "Continue"}</span>
          </Button>
        {/if}
      </div>
    </footer>
  {/if}
</div>
