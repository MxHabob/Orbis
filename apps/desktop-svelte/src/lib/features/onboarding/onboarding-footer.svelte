<!--
  Bottom navigation bar for the onboarding wizard.
  Shows Previous/Next buttons aligned with the forms width.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui-svelte/button";
import { ArrowRight, ArrowLeft } from "lucide-svelte";

let {
  activeStep,
  totalSteps,
  isLoading = false,
  onNext,
  onPrev,
}: {
  activeStep: number;
  totalSteps: number;
  isLoading?: boolean;
  onNext: () => void;
  onPrev: () => void;
} = $props();
</script>

<div class="w-full border-t border-border bg-muted/5 py-4 px-6 shrink-0">
  <div class="max-w-4xl w-full mx-auto flex justify-between items-center">
    <!-- Back button -->
    <Button
      variant="outline"
      onclick={onPrev}
      disabled={activeStep === 0 || isLoading}
      class="border-border text-muted-foreground hover:text-foreground hover:bg-muted flex gap-2 items-center"
    >
      <ArrowLeft class="w-4 h-4 rtl:rotate-180" />
      <span>{m.common_onboarding_prev_btn()}</span>
    </Button>

    <!-- Next button -->
    {#if activeStep < totalSteps}
      <Button
        onclick={onNext}
        disabled={isLoading}
        class="bg-primary text-primary-foreground hover:bg-primary/90 flex gap-2 items-center"
      >
        <span>{m.common_onboarding_next_btn()}</span>
        <ArrowRight class="w-4 h-4 rtl:rotate-180" />
      </Button>
    {/if}
  </div>
</div>
