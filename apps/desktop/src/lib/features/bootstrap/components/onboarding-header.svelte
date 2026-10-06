<!--
  @component OnboardingHeader
  Floating minimal header with high-end segmented progress indicators (without noisy text),
  brand identity, language switch (EN/AR), and theme toggle.
-->
<script lang="ts">
import type { OnboardingStep } from "../types";

let {
  currentStep,
  steps,
}: {
  currentStep: OnboardingStep;
  steps: Array<{ id: OnboardingStep }>;
} = $props();

const currentStepIndex = $derived(steps.findIndex((s) => s.id === currentStep));

</script>

<header class="sticky top-2 z-30 w-full mx-auto flex backdrop-blur-2xl px-2 justify-center transition-all shrink-0">
  <!-- Minimalist Visual Progress (Pure Progress Segments Without Text) -->
  <nav aria-label="Progress" class="flex items-center gap-2 justify-center">
    {#each steps as s, idx}
      {@const isDone = idx < currentStepIndex}
      {@const isCurrent = idx === currentStepIndex}

      <div
        class="h-1.5 rounded-full transition-all duration-300 {isCurrent
          ? 'w-7 sm:w-10 bg-primary shadow-xs shadow-primary/30 ring-2 ring-primary/20'
          : isDone
            ? 'w-3.5 sm:w-5 bg-foreground/60'
            : 'w-2 sm:w-3 bg-muted-foreground/20'}"
        title={`Step ${idx + 1} of ${steps.length}`}
      ></div>
    {/each}
  </nav>
</header>
