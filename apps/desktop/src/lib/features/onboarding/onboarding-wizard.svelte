<!--
  Onboarding Wizard — Streamlined Unified Academic Setup.
  Coordinates all steps from organization bootstrap to calendar, spaces, curriculum, fees, review, and launch.
-->
<script lang="ts">
import { onMount } from "svelte";
import { getLocale } from "$lib/paraglide/runtime";
import { onboardingState } from "./state.svelte";
import { authStore, systemStore } from "$lib/stores";

import OnboardingStepper from "./onboarding-stepper.svelte";
import OnboardingFooter from "./onboarding-footer.svelte";

import OrganizationBootstrapStep from "./steps/organization-bootstrap-step.svelte";
import WelcomeStep from "./steps/welcome-step.svelte";
import AcademicCalendarStep from "./steps/academic-calendar-step.svelte";
import TimeSlotsStep from "./steps/time-slots-step.svelte";
import FacilitiesRoomsStep from "./steps/facilities-rooms-step.svelte";
import GradesClassesStep from "./steps/grades-classes-step.svelte";
import SubjectsStep from "./steps/subjects-step.svelte";
import FinancialSetupStep from "./steps/financial-setup-step.svelte";
import ReviewStep from "./steps/review-step.svelte";
import LaunchStep from "./steps/launch-step.svelte";

import { InfoCircle } from "@orbis/icons";

const isArabic = $derived(getLocale().startsWith("ar"));
let needsBootstrap = $derived(!authStore.isAuthenticated);

onMount(async () => {
  await authStore.restore();
  await systemStore.loadReadiness();
  onboardingState.init();
});

function handleStepClick(step: number) {
  if (needsBootstrap) return;
  onboardingState.goToStep(step);
}
</script>

<div class="h-full w-full bg-background text-foreground flex flex-col overflow-hidden font-sans select-none">

  {#if !needsBootstrap}
    <OnboardingStepper
      currentStep={onboardingState.currentStep}
      totalSteps={onboardingState.totalSteps}
      onStepClick={handleStepClick}
    />
  {/if}

  <!-- Main Content Container (Direct, flat, zero nested container clutter) -->
  <main class="flex-1 min-h-0 w-full px-4 sm:px-6 lg:px-8 py-2.5 flex flex-col overflow-hidden justify-center">
    {#if onboardingState.errorMessage}
      <div class="mb-2 flex items-center gap-2.5 p-2.5 rounded-xl bg-destructive/10 border border-destructive/20 text-destructive text-xs animate-in fade-in shrink-0">
        <InfoCircle class="w-4 h-4 shrink-0" />
        <span class="font-medium">{onboardingState.errorMessage}</span>
      </div>
    {/if}

    <div class="w-full h-full flex flex-col overflow-y-auto justify-center">
      {#if needsBootstrap}
        <OrganizationBootstrapStep />
      {:else if onboardingState.currentStep === 0}
        <WelcomeStep />
      {:else if onboardingState.currentStep === 1}
        <AcademicCalendarStep />
      {:else if onboardingState.currentStep === 2}
        <TimeSlotsStep />
      {:else if onboardingState.currentStep === 3}
        <FacilitiesRoomsStep />
      {:else if onboardingState.currentStep === 4}
        <GradesClassesStep />
      {:else if onboardingState.currentStep === 5}
        <SubjectsStep />
      {:else if onboardingState.currentStep === 6}
        <FinancialSetupStep />
      {:else if onboardingState.currentStep === 7}
        <ReviewStep />
      {:else if onboardingState.currentStep === 8}
        <LaunchStep />
      {/if}
    </div>
  </main>

  <!-- Bottom Navigation Footer -->
  {#if !needsBootstrap}
    <OnboardingFooter
      activeStep={onboardingState.currentStep}
      totalSteps={onboardingState.totalSteps}
      isLoading={onboardingState.isLoading}
      onNext={() => onboardingState.nextStep()}
      onPrev={() => onboardingState.prevStep()}
    />
  {/if}

</div>
