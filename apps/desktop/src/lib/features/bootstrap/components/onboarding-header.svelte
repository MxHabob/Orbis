<!--
  @component OnboardingHeader
  Floating minimal header with high-end segmented progress indicators (without noisy text),
  brand identity, language switch (EN/AR), and theme toggle.
-->
<script lang="ts">
import type { OnboardingStep } from "../types";
import * as m from "$lib/paraglide/messages";
import SetupWizardHeader from "$lib/shared/components/setup-wizard-header.svelte";

let {
  currentStep,
  steps,
}: {
  currentStep: OnboardingStep;
  steps: Array<{ id: OnboardingStep }>;
} = $props();

const currentStepIndex = $derived(steps.findIndex((s) => s.id === currentStep));
const stepLabels: Record<OnboardingStep, () => string> = {
  intro: m.ui_onboarding_intro_release_notes,
  blueprint: m.ui_onboarding_blueprint_select_institutional_blueprint,
  organization: m.ui_onboarding_organization_organization_profile_identity,
  region: m.ui_onboarding_organization_regional_fiscal_parameters,
  admin: m.ui_onboarding_admin_primary_administrator_account,
  mfa: m.ui_onboarding_mfa_two_factor_authentication_setup,
  review: m.ui_onboarding_review_launch_bootstrap,
};
const stepPosition = $derived(`${currentStepIndex + 1} / ${steps.length}`);
const progressValue = $derived(((currentStepIndex + 1) / steps.length) * 100);
</script>

<SetupWizardHeader
  contextLabel="Orbis"
  stepLabel={stepLabels[currentStep]()}
  {stepPosition}
  {progressValue}
  ariaLabel={m.onboarding_stages_title()}
/>
