<!--
  Onboarding Wizard — Streamlined 9-step academic setup.
  Coordinates all steps from calendar and spaces to curriculum, fees, review, and launch.
-->
<script lang="ts">
import { onMount } from "svelte";
import { getLocale } from "$lib/paraglide/runtime";
import { onboardingState } from "./state.svelte";
import { authStore } from "$lib/stores/auth.svelte";

import OnboardingStepper from "./onboarding-stepper.svelte";
import OnboardingFooter from "./onboarding-footer.svelte";

import WelcomeStep from "./steps/welcome-step.svelte";
import AcademicCalendarStep from "./steps/academic-calendar-step.svelte";
import TimeSlotsStep from "./steps/time-slots-step.svelte";
import FacilitiesRoomsStep from "./steps/facilities-rooms-step.svelte";
import GradesClassesStep from "./steps/grades-classes-step.svelte";
import SubjectsStep from "./steps/subjects-step.svelte";
import FinancialSetupStep from "./steps/financial-setup-step.svelte";
import ReviewStep from "./steps/review-step.svelte";
import LaunchStep from "./steps/launch-step.svelte";

import { AlertCircle } from "@lucide/svelte";

const isArabic = $derived(getLocale().startsWith("ar"));

onMount(() => {
  onboardingState.init();
});

function handleStepClick(step: number) {
  onboardingState.goToStep(step);
}
</script>

<div class="h-full w-full bg-gradient-to-br from-background via-background/98 to-primary/5 text-foreground flex flex-col justify-between overflow-hidden font-sans select-none">

  <!-- Stepper Indicator -->
  <div class="w-full pt-2.5 pb-1 px-4 shrink-0">
    <OnboardingStepper
      currentStep={onboardingState.currentStep}
      totalSteps={onboardingState.totalSteps}
      onStepClick={handleStepClick}
    />
  </div>

  <!-- Step Content Container (Full screen real estate, zero wasted top margin) -->
  <main class="flex-1 min-h-0 w-full max-w-7xl 2xl:max-w-[1550px] mx-auto px-4 sm:px-6 lg:px-8 pt-1.5 pb-2 flex flex-col justify-center overflow-hidden">
    
    {#if onboardingState.errorMessage}
      <div class="mb-2 flex items-center gap-2.5 p-3 rounded-xl bg-destructive/10 border border-destructive/20 text-destructive text-xs animate-in fade-in shrink-0">
        <AlertCircle class="w-4 h-4 shrink-0" />
        <span class="font-medium">{onboardingState.errorMessage}</span>
      </div>
    {/if}

    <div class="w-full flex-1 min-h-0 flex flex-col justify-center overflow-hidden">
      {#if onboardingState.currentStep === 0}
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
  <OnboardingFooter
    activeStep={onboardingState.currentStep}
    totalSteps={onboardingState.totalSteps}
    isLoading={onboardingState.isLoading}
    onNext={() => onboardingState.nextStep()}
    onPrev={() => onboardingState.prevStep()}
  />

</div>
