<!--
  Onboarding Wizard — Full 10-step initial academic setup.
  Coordinates all steps from welcome and presets to calendar, spaces, curriculum, and launch.
-->
<script lang="ts">
import { onMount } from "svelte";
import { getLocale } from "$lib/paraglide/runtime";
import { onboardingState } from "./state.svelte";
import { authStore } from "$lib/stores/auth.svelte";

import OnboardingStepper from "./onboarding-stepper.svelte";
import OnboardingFooter from "./onboarding-footer.svelte";

import WelcomeStep from "./steps/welcome-step.svelte";
import InstitutionTypeStep from "./steps/institution-type-step.svelte";
import AcademicCalendarStep from "./steps/academic-calendar-step.svelte";
import TimeSlotsStep from "./steps/time-slots-step.svelte";
import FacilitiesRoomsStep from "./steps/facilities-rooms-step.svelte";
import GradesClassesStep from "./steps/grades-classes-step.svelte";
import SubjectsStep from "./steps/subjects-step.svelte";
import FinancialSetupStep from "./steps/financial-setup-step.svelte";
import ReviewStep from "./steps/review-step.svelte";
import LaunchStep from "./steps/launch-step.svelte";

import { RotateCcw, AlertCircle } from "@lucide/svelte";
import { Button } from "@orbis/ui/button";

const isArabic = $derived(getLocale().startsWith("ar"));

onMount(() => {
  onboardingState.init();
});

function handleStepClick(step: number) {
  onboardingState.goToStep(step);
}

function handleReset() {
  if (confirm(isArabic ? "هل أنت متأكد من إعادة تعيين جميع بيانات التهيئة؟" : "Are you sure you want to reset all onboarding data?")) {
    onboardingState.reset();
  }
}
</script>

<div class="min-h-screen bg-gradient-to-br from-background via-background/98 to-primary/5 text-foreground flex flex-col justify-between font-sans">
  
  <!-- Header Bar -->
  <header class="w-full border-b border-border/40 py-3.5 px-6 shrink-0 bg-card/20 backdrop-blur-xs">
    <div class="max-w-4xl w-full mx-auto flex items-center justify-between">
      <div class="flex items-center gap-3">
        <div class="w-2.5 h-2.5 rounded-full bg-primary animate-pulse"></div>
        <div class="flex flex-col">
          <h1 class="text-sm font-bold tracking-tight text-foreground leading-none">
            {isArabic ? "معالج التهيئة الأكاديمية" : "Academic Setup Wizard"}
          </h1>
          <span class="text-[10.5px] text-muted-foreground mt-0.5">
            {onboardingState.profile.orgNameAr || onboardingState.profile.orgName || "Orbis"}
          </span>
        </div>
      </div>

      <!-- Quick Reset Button -->
      <Button
        variant="ghost"
        size="sm"
        onclick={handleReset}
        class="h-8 px-2.5 text-xs text-muted-foreground hover:text-destructive gap-1.5 rounded-lg cursor-pointer"
        title={isArabic ? "إعادة ضبط المعالج" : "Reset Wizard"}
      >
        <RotateCcw size={13} />
        <span class="hidden sm:inline">{isArabic ? "إعادة ضبط" : "Reset"}</span>
      </Button>
    </div>
  </header>

  <!-- Stepper Indicator -->
  <div class="w-full pt-4 shrink-0">
    <OnboardingStepper
      currentStep={onboardingState.currentStep}
      totalSteps={onboardingState.totalSteps}
      onStepClick={handleStepClick}
    />
  </div>

  <!-- Step Content Container -->
  <main class="flex-1 max-w-4xl w-full mx-auto px-4 py-2 overflow-y-auto min-h-[50vh] flex flex-col justify-center">
    
    {#if onboardingState.errorMessage}
      <div class="mb-4 flex items-center gap-3 p-4 rounded-xl bg-destructive/10 border border-destructive/20 text-destructive text-xs animate-in fade-in">
        <AlertCircle class="w-4 h-4 shrink-0" />
        <span class="font-medium">{onboardingState.errorMessage}</span>
      </div>
    {/if}

    {#if onboardingState.currentStep === 0}
      <WelcomeStep />
    {:else if onboardingState.currentStep === 1}
      <InstitutionTypeStep />
    {:else if onboardingState.currentStep === 2}
      <AcademicCalendarStep />
    {:else if onboardingState.currentStep === 3}
      <TimeSlotsStep />
    {:else if onboardingState.currentStep === 4}
      <FacilitiesRoomsStep />
    {:else if onboardingState.currentStep === 5}
      <GradesClassesStep />
    {:else if onboardingState.currentStep === 6}
      <SubjectsStep />
    {:else if onboardingState.currentStep === 7}
      <FinancialSetupStep />
    {:else if onboardingState.currentStep === 8}
      <ReviewStep />
    {:else if onboardingState.currentStep === 9}
      <LaunchStep />
    {/if}
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
