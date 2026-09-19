<!--
  Bottom navigation bar for the onboarding wizard.
  Mirrors the luxury centered capsule aesthetic and design tokens of onboarding-stepper.
-->
<script lang="ts">
import { getLocale } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { ArrowRight, ArrowLeft, Loader2, Sparkles } from "@lucide/svelte";

let {
  activeStep,
  totalSteps = 9,
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

const isArabic = $derived(getLocale().startsWith("ar"));
const isLastStep = $derived(activeStep >= totalSteps - 1);
</script>

<div class="w-full flex items-center justify-center py-2 select-none shrink-0">
  <!-- Centered Luxury Capsule Footer Unit -->
  <div class="inline-flex items-center gap-3 px-3 py-1.5 rounded-full bg-card/90 border border-border/80 shadow-2xs backdrop-blur-md">
    
    <!-- Previous Button -->
    <Button
      variant="outline"
      size="xs"
      onclick={onPrev}
      disabled={activeStep === 0 || isLoading}
      class=" px-3.5 h-7 text-xs font-medium cursor-pointer border-border/70"
    >
      {#if isArabic}
        <ArrowRight class="w-3.5 h-3.5" />
      {:else}
        <ArrowLeft class="w-3.5 h-3.5" />
      {/if}
      <span>{m.common_onboarding_prev_btn?.() || (isArabic ? "السابق" : "Previous")}</span>
    </Button>

    <!-- Step Counter Pill -->
    <span class="font-mono text-xs font-semibold text-muted-foreground tracking-wider px-2.5 py-0.5 rounded-full bg-muted/40 border border-border/40">
      <span class="text-foreground font-bold">{String(activeStep + 1).padStart(2, "0")}</span>
      <span class="opacity-40">/</span>
      <span>{String(totalSteps).padStart(2, "0")}</span>
    </span>

    <!-- Next / Launch Button -->
    {#if !isLastStep}
      <Button
        variant="default"
        size="xs"
        onclick={onNext}
        disabled={isLoading}
        class="px-4 h-7 text-xs font-bold cursor-pointer shadow-xs"
      >
        {#if isLoading}
          <Loader2 class="w-3.5 h-3.5 animate-spin" />
        {:else}
          <span>{m.common_onboarding_next_btn?.() || (isArabic ? "التالي" : "Next")}</span>
          {#if isArabic}
            <ArrowLeft class="w-3.5 h-3.5" />
          {:else}
            <ArrowRight class="w-3.5 h-3.5" />
          {/if}
        {/if}
      </Button>
    {:else}
      <div class="inline-flex items-center gap-1.5 px-3.5 py-1 bg-emerald-500/10 text-emerald-600 dark:text-emerald-400 border border-emerald-500/20 text-xs font-bold">
        <Sparkles class="w-3.5 h-3.5" />
        <span>{isArabic ? "جاهز" : "Ready"}</span>
      </div>
    {/if}

  </div>
</div>
