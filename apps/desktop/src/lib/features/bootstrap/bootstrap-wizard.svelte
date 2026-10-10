<!--
  @component BootstrapWizard
  Master orchestrator for clean organization bootstrap, regional calibration,
  and initial administrator initialization.
  Floating minimal header & footer with 6 progressive steps:
  1. Mulverse Intro -> 2. Blueprint -> 3. Organization -> 4. Administrator -> 5. MFA (Skippable) -> 6. Launch Review.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount, untrack } from "svelte";
import { goto } from "$app/navigation";
import { resolve } from "$app/paths";
import { createMutation } from "@tanstack/svelte-query";
import { api } from "$lib/api/client";
import type {
  BootstrapRequest,
  BootstrapResult,
} from "$lib/api/types";
import { authStore, licenseStore, systemStore } from "$lib/stores";
import { toast } from "@orbis/ui";
import { extractErrorMessage } from "$lib/api";
import { localizeHref, getLocale } from "$lib/paraglide/runtime";
import {
  type OnboardingStep,
  type OnboardingFormModel,
  INITIAL_ONBOARDING_VALUES,
} from "./types";
import OnboardingFooter from "./components/onboarding-footer.svelte";
import StepReleaseIntro from "./components/step-release-intro.svelte";
import StepBlueprint from "./components/step-blueprint.svelte";
import StepOrganization from "./components/step-organization.svelte";
import StepRegion from "./components/step-region.svelte";
import StepAdmin from "./components/step-admin.svelte";
import StepMfa from "./components/step-mfa.svelte";
import StepReview from "./components/step-review.svelte";
import OnboardingHeader from "./components/onboarding-header.svelte";
import SetupWorkspace from "$lib/shared/components/setup-workspace.svelte";
import { buildBootstrapRequest } from "./bootstrap-request";
import {
  type BootstrapValidationCode,
  bootstrapFormSchema,
  normalizeBootstrapForm,
  validateBootstrapForm,
  validateBootstrapStep,
} from "./bootstrap-validation";
import { FormController } from "$lib/shared/forms/form-controller.svelte";

let currentStep = $state<OnboardingStep>("intro");
const form = untrack(() => new FormController<OnboardingFormModel>({
  initialValues: {
    ...INITIAL_ONBOARDING_VALUES,
    locale: getLocale().startsWith("ar") ? "ar-SA" : "en-US",
  },
  schema: bootstrapFormSchema,
  normalize: normalizeBootstrapForm,
}));
let errors = $state<Record<string, string>>({});
const bootstrapMutation = createMutation(() => ({
  mutationFn: (request: BootstrapRequest) => api.bootstrapOrganization(request),
}));
const isSubmitting = $derived(form.isSubmitting || bootstrapMutation.isPending);
let isComplete = $state(false);
let postBootstrapError = $state<string | null>(null);
let bootstrapResult = $state<BootstrapResult | null>(null);

const steps: Array<{ id: OnboardingStep }> = [
  { id: "intro" },
  { id: "blueprint" },
  { id: "organization" },
  { id: "region" },
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
      form.setField("countryCode", data.country_code || "SA");
      form.setField("timezone", data.timezone || "Asia/Riyadh");
      form.setField("baseCurrencyCode", data.currency_code || "SAR");
      form.setField("isHijriEnabled", data.is_hijri_recommended ?? true);
      form.setField("dateFormat", data.date_format || "YYYY-MM-DD");
    }
  } catch (err) {
    console.warn("Silent environment detection completed with defaults:", err);
  }
}

onMount(() => {
  loadEnvironment();
});

function validateCurrentStep(): boolean {
  const validationErrors = validateBootstrapStep(currentStep, form.values);
  errors = localizeValidationErrors(validationErrors);
  return Object.keys(validationErrors).length === 0;
}

function localizeValidationErrors(
  validationErrors: Partial<Record<string, string>>,
): Record<string, string> {
  const messages: Record<BootstrapValidationCode, () => string> = {
    organizationNameRequired: () => m.ui_onboarding_bootstrap_organization_name_in_english_is_required(),
    adminFirstNameRequired: () => m.ui_onboarding_bootstrap_first_name_is_required(),
    adminLastNameRequired: () => m.ui_onboarding_bootstrap_last_name_is_required(),
    adminEmailRequired: () => m.ui_onboarding_bootstrap_email_address_is_required(),
    adminEmailInvalid: () => m.ui_onboarding_bootstrap_invalid_email_format(),
    adminPasswordRequired: () => m.ui_onboarding_bootstrap_password_is_required(),
    adminPasswordTooShort: () =>
      m.ui_onboarding_bootstrap_password_min_length(),
    adminPasswordMismatch: () => m.setup_bootstrap_err_password_mismatch(),
    timezoneRequired: () => m.ui_onboarding_bootstrap_timezone_is_required(),
    localeRequired: () => m.ui_onboarding_bootstrap_locale_is_required(),
  };
  const isValidationCode = (
    code: string,
  ): code is BootstrapValidationCode => Object.hasOwn(messages, code);

  return Object.fromEntries(
    Object.entries(validationErrors).map(([field, code]) => {
      const localizedMessage =
        code && isValidationCode(code) ? messages[code]() : undefined;
      return [field, localizedMessage ?? code ?? ""];
    }),
  );
}

function firstInvalidStep(
  validationErrors: Partial<Record<string, string>>,
): OnboardingStep {
  if (validationErrors.organizationName) return "organization";
  if (validationErrors.timezone || validationErrors.locale) return "region";
  return "admin";
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
    errors = {};
    scrollToTop();
  }
}

function prevStep() {
  if (currentStepIndex > 0) {
    currentStep = steps[currentStepIndex - 1].id;
    errors = {};
    scrollToTop();
  }
}

function skipStep() {
  if (currentStepIndex < steps.length - 1) {
    currentStep = steps[currentStepIndex + 1].id;
    errors = {};
    scrollToTop();
  }
}

async function handleLaunchBootstrap() {
  if (isSubmitting || isComplete) return;

  const validationErrors = validateBootstrapForm(form.values);
  if (Object.keys(validationErrors).length > 0) {
    errors = localizeValidationErrors(validationErrors);
    currentStep = firstInvalidStep(validationErrors);
    scrollToTop();
    return;
  }

  postBootstrapError = null;
  const submitted = await form.submit(async (payload) => {
    const result = await bootstrapMutation.mutateAsync(
      buildBootstrapRequest(payload),
    );
    if (!result.success) {
      throw new Error(m.ui_onboarding_review_initialization_failed());
    }
    bootstrapResult = result;

    try {
      await authStore.login(payload.adminEmail.toLowerCase(), payload.adminPassword);
    } catch (authErr) {
      postBootstrapError = extractErrorMessage(authErr);
      toast.error(postBootstrapError);
    }
    if (!postBootstrapError) {
      try {
        const readiness = await systemStore.loadReadiness();
        if (!readiness) {
          postBootstrapError = m.common_action_failed();
          toast.error(postBootstrapError);
        }
      } catch (readinessError) {
        postBootstrapError = extractErrorMessage(readinessError);
        toast.error(postBootstrapError);
      }
    }
    if (!postBootstrapError && !licenseStore.isOperational) {
      const trial = await licenseStore.startTrial();
      if (!trial) {
        toast.error(
          licenseStore.error ?? m.license_activation_failed(),
        );
      }
    }
  });

  if (submitted) {
    isComplete = true;
  } else if (form.formError) {
    toast.error(form.formError);
  } else if (Object.keys(form.errors).length > 0) {
    const validationErrors = Object.fromEntries(
      Object.entries(form.errors).map(([field, messages]) => [
        field,
        messages[0],
      ]),
    );
    errors = localizeValidationErrors(validationErrors);
    const visibleFields = new Set([
      "organizationName",
      "timezone",
      "locale",
      "adminFirstName",
      "adminLastName",
      "adminEmail",
      "adminPassword",
      "adminConfirmPassword",
    ]);
    const hasVisibleFieldError = Object.keys(validationErrors).some((field) =>
      visibleFields.has(field),
    );
    if (!hasVisibleFieldError) {
      toast.error(
        Object.values(validationErrors).filter(Boolean).join(" ") ||
          m.common_action_failed(),
      );
    } else {
      currentStep = firstInvalidStep(validationErrors);
    }
  }
}

function enterWorkspace() {
  if (postBootstrapError && !authStore.isAuthenticated) {
    goto(resolve(localizeHref("/login")), { replaceState: true });
    return;
  }
  const orgId = bootstrapResult?.organization_id || authStore.orgId;
  if (orgId) {
    goto(resolve(localizeHref(`/${orgId}`)), { replaceState: true });
  } else {
    goto(resolve(localizeHref("/login")), { replaceState: true });
  }
}
</script>

<SetupWorkspace
  viewportId="onboarding-viewport"
>
  {#snippet header()}
    <OnboardingHeader {currentStep} {steps} />
  {/snippet}

  {#if currentStep === "intro"}
    <StepReleaseIntro />
  {:else if currentStep === "blueprint"}
    <StepBlueprint bind:selectedType={form.values.institutionType} />
  {:else if currentStep === "organization"}
    <StepOrganization bind:form={form.values} {errors} />
  {:else if currentStep === "region"}
    <StepRegion bind:form={form.values} {errors} />
  {:else if currentStep === "admin"}
    <StepAdmin bind:form={form.values} {errors} />
  {:else if currentStep === "mfa"}
    <StepMfa />
  {:else if currentStep === "review"}
    <StepReview
      form={form.values}
      {isSubmitting}
      {isComplete}
      authenticationRequired={Boolean(postBootstrapError && !authStore.isAuthenticated)}
      warnings={bootstrapResult?.warnings ?? []}
      result={bootstrapResult}
      onLaunch={handleLaunchBootstrap}
      onEnterWorkspace={enterWorkspace}
    />
  {/if}

  {#snippet footer()}
    {#if !isComplete}
      <OnboardingFooter
        {canGoBack}
        canGoNext={!isLastStep}
        {isSkippable}
        {isSubmitting}
        onPrev={prevStep}
        onNext={nextStep}
        onSkip={skipStep}
      />
    {/if}
  {/snippet}
</SetupWorkspace>
