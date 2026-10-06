<script lang="ts">
import { Badge } from "@orbis/ui/badge";
import * as m from "$lib/paraglide/messages";
import type { OnboardingStepStatus } from "./onboarding-steps";

let {
  stepNumber,
  totalSteps = 8,
  title,
  description,
  isRequired = true,
  status = "not_started" as OnboardingStepStatus,
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

<div class="mb-8 flex flex-col gap-4 border-b border-border/40 pb-6 sm:flex-row sm:items-start sm:justify-between">
  <div class="space-y-2">
    <div class="flex flex-wrap items-center gap-2.5">
      <span class="inline-flex items-center rounded-md bg-primary/10 px-2.5 py-0.5 text-xs font-semibold tracking-wider text-primary uppercase">
        {m.onboarding_step_prefix({ step: stepNumber, total: totalSteps })}
      </span>

      {#if isRequired}
        <Badge variant="outline" class="border-amber-500/30 bg-amber-500/10 text-xs font-medium text-amber-600 dark:text-amber-400">
          {m.onboarding_required_badge()}
        </Badge>
      {:else}
        <Badge variant="secondary" class="text-xs text-muted-foreground">
          {m.onboarding_optional_badge()}
        </Badge>
      {/if}

      {#if status === "completed"}
        <Badge variant="outline" class="border-emerald-500/30 bg-emerald-500/10 text-xs text-emerald-600 dark:text-emerald-400">
          <span class="mr-1 inline-block size-1.5 rounded-full bg-emerald-500"></span>
          {m.onboarding_completed_badge()}
        </Badge>
      {:else if status === "in_progress"}
        <Badge variant="outline" class="border-blue-500/30 bg-blue-500/10 text-xs text-blue-600 dark:text-blue-400">
          <span class="mr-1 inline-block size-1.5 rounded-full bg-blue-500 animate-pulse"></span>
          {m.onboarding_in_progress_badge()}
        </Badge>
      {:else if status === "blocked"}
        <Badge variant="destructive" class="text-xs">
          {m.onboarding_blocked_badge()}
        </Badge>
      {/if}
    </div>

    <h1 class="text-2xl font-bold tracking-tight text-foreground sm:text-3xl">
      {title}
    </h1>

    <p class="max-w-2xl text-sm leading-relaxed text-muted-foreground sm:text-base">
      {description}
    </p>
  </div>

  {#if actionButton}
    <div class="shrink-0 pt-2 sm:pt-0">
      {@render actionButton()}
    </div>
  {/if}
</div>
