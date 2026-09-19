<!--
  Bottom navigation bar for the onboarding wizard.
  Shows Previous/Next buttons aligned with the form width.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { ArrowRight, ArrowLeft } from "@lucide/svelte";

let {
  activeStep,
  totalSteps = 10,
  isLoading = false,
  onNext,
  onPrev,
}: {
  activeStep: number;
  totalSteps?: number;
  isLoading?: boolean;
  onNext: () => void;
  onPrev: () => void;
} = $props();
</script>

<div class="w-full py-3.5 px-6 shrink-0 border-t border-border/60 bg-card/80 backdrop-blur-md z-30">
  <div class="max-w-4xl w-full mx-auto flex justify-between items-center">
    <!-- Back button -->
    <Button
      variant="outline"
      onclick={onPrev}
      disabled={activeStep === 0 || isLoading}
      class="flex gap-2 items-center text-xs h-9 px-4 rounded-xl cursor-pointer"
    >
      <ArrowLeft class="w-4 h-4 rtl:rotate-180" />
      <span>{m.common_onboarding_prev_btn?.() || "Previous"}</span>
    </Button>

    <!-- Next button (hidden on last step where launch button exists) -->
    {#if activeStep < totalSteps - 1}
      <Button
        onclick={onNext}
        disabled={isLoading}
        class="flex gap-2 items-center text-xs h-9 px-5 rounded-xl cursor-pointer shadow-xs font-semibold"
      >
        <span>{m.common_onboarding_next_btn?.() || "Next"}</span>
        <ArrowRight class="w-4 h-4 rtl:rotate-180" />
      </Button>
    {/if}
  </div>
</div>
