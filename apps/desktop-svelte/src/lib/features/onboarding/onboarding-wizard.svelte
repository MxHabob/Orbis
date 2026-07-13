<!--
  Onboarding Wizard Orchestrator
  Combines Stepper, Active Step component, and Footer navigation.
  Provides a clean full-screen experience without a top header or outer Card wrap.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onboardingState } from "./state.svelte";

// Import components
import OnboardingStepper from "./onboarding-stepper.svelte";
import OnboardingFooter from "./onboarding-footer.svelte";
import WelcomeStep from "./steps/welcome-step.svelte";
import InstitutionTypeStep from "./steps/institution-type-step.svelte";
import AcademicCalendarStep from "./steps/academic-calendar-step.svelte";
import BuildingsRoomsStep from "./steps/buildings-rooms-step.svelte";
import GradesClassesStep from "./steps/grades-classes-step.svelte";
import TimeSlotsStep from "./steps/time-slots-step.svelte";
import SubjectsStep from "./steps/subjects-step.svelte";
import FinancialSetupStep from "./steps/financial-setup-step.svelte";
import ReviewStep from "./steps/review-step.svelte";
import LaunchStep from "./steps/launch-step.svelte";
</script>

<div class="h-screen w-screen bg-background text-foreground flex flex-col overflow-hidden font-sans antialiased">
  <!-- Top Progress Indicator -->
  <div class="pt-10 shrink-0">
    <OnboardingStepper
      progressValue={onboardingState.progressValue}
    />
  </div>

  <!-- Step Content Area -->
  <div class="flex-1 max-w-5xl w-full mx-auto px-6 py-6 overflow-y-auto scrollbar-none min-h-0">
    {#if onboardingState.activeStep === 0}
      <WelcomeStep />
    {:else if onboardingState.activeStep === 1}
      <InstitutionTypeStep />
    {:else if onboardingState.activeStep === 2}
      <AcademicCalendarStep />
    {:else if onboardingState.activeStep === 3}
      <BuildingsRoomsStep />
    {:else if onboardingState.activeStep === 4}
      <GradesClassesStep />
    {:else if onboardingState.activeStep === 5}
      <TimeSlotsStep />
    {:else if onboardingState.activeStep === 6}
      <SubjectsStep />
    {:else if onboardingState.activeStep === 7}
      <FinancialSetupStep />
    {:else if onboardingState.activeStep === 8}
      <ReviewStep />
    {:else if onboardingState.activeStep === 9}
      <LaunchStep />
    {/if}
  </div>

  <!-- Bottom Actions Control Bar -->
  <OnboardingFooter
    activeStep={onboardingState.activeStep}
    totalSteps={onboardingState.totalSteps}
    isLoading={onboardingState.isLoading}
    onNext={() => onboardingState.goNext()}
    onPrev={() => onboardingState.goPrev()}
  />
</div>

<style>
/* Hide scrollbar for Chrome, Safari and Opera */
.scrollbar-none::-webkit-scrollbar {
  display: none;
}

/* Hide scrollbar for IE, Edge and Firefox */
.scrollbar-none {
  -ms-overflow-style: none;  /* IE and Edge */
  scrollbar-width: none;  /* Firefox */
}
</style>
