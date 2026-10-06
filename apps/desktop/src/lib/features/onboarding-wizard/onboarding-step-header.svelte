<script lang="ts">
import * as m from "$lib/paraglide/messages";
import type { OnboardingStepStatus } from "./onboarding-steps";

let {
  stepNumber,
  totalSteps = 8,
  title,
  actionButton = undefined,
}: {
  stepNumber: number;
  totalSteps?: number;
  title: string;
  description: string;
  isRequired?: boolean;
  status?: OnboardingStepStatus;
  actionButton?: any;
} = $props();
</script>

<div class="mb-6 flex items-center justify-between gap-4 border-b border-border/40 pb-4">
  <div class="flex min-w-0 items-baseline gap-3">
    <span class="shrink-0 text-xs font-medium tabular-nums text-muted-foreground">
      {m.onboarding_step_prefix({ step: stepNumber, total: totalSteps })}
    </span>
    <h1 class="truncate text-lg font-semibold tracking-tight text-foreground sm:text-xl">
      {title}
    </h1>
  </div>

  {#if actionButton}
    <div class="shrink-0">
      {@render actionButton()}
    </div>
  {/if}
</div>
