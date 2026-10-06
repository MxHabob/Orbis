<script lang="ts">
  import { page } from "$app/state";
  import { onMount } from "svelte";
  import type { Snippet } from "svelte";
  import * as m from "$lib/paraglide/messages";
  import { authStore, systemStore } from "$lib/stores";
  import { Button } from "@orbis/ui/button";
  import { deriveSteps } from "./onboarding-steps";
  import { navigateOnboardingRoute } from "./navigation";

  let { orgId, children }: { orgId: string; children: Snippet } = $props();

  const currentPath = $derived(page.url.pathname);

  onMount(async () => {
    await systemStore.loadOnboardingState(orgId);
  });

  const onboardingState = $derived(systemStore.onboardingState);
  const steps = $derived(deriveSteps(systemStore.onboardingTasks));
  const overallProgress = $derived(systemStore.onboardingProgressPercentage);

  const currentStepIndex = $derived(
    steps.findIndex((step) => currentPath.endsWith(`/onboarding/${step.route}`))
  );

  const organizationName = $derived(
    onboardingState?.organization?.name || authStore.currentOrg?.name || "Orbis"
  );

  async function exitSetup() {
    await navigateOnboardingRoute(`/${orgId}`);
  }
</script>

<div class="flex min-h-0 h-full flex-col overflow-hidden bg-background font-sans text-foreground selection:bg-primary/20">
  <header class="z-20 shrink-0 border-b border-border/60 bg-background">
    <div class="mx-auto flex h-14 w-full max-w-[96rem] items-center gap-6 px-4 sm:px-8">
      <span class="min-w-0 shrink-0 truncate text-sm font-semibold tracking-tight">
        {organizationName}
      </span>

      <nav
        class="flex flex-1 items-center gap-1.5"
        aria-label={m.onboarding_stages_title()}
      >
        {#each steps as step, index (step.id)}
          <span
            class="h-1 flex-1 rounded-full transition-colors {step.status === 'completed' || index === currentStepIndex
              ? 'bg-primary'
              : 'bg-muted'}"
            aria-hidden="true"
          ></span>
          <span class="sr-only" aria-current={index === currentStepIndex ? "step" : undefined}>
            {step.getLabel()}
          </span>
        {/each}
      </nav>
    </div>
  </header>

  <main id="onboarding-viewport" class="min-h-0 flex-1 overflow-y-auto overscroll-contain">
    <div class="mx-auto w-full max-w-[96rem] px-4 py-6 sm:px-8 sm:py-8 lg:px-12">
      {@render children()}
    </div>
  </main>

  <footer class="z-20 shrink-0 border-t border-border/60 bg-background/95">
    <div class="mx-auto flex h-14 w-full max-w-[96rem] items-center justify-between px-4 sm:px-8">
      <span class="text-xs tabular-nums text-muted-foreground" aria-live="polite">
        {m.onboarding_complete_percent({ percent: overallProgress })}
      </span>
      <Button
        variant="ghost"
        size="sm"
        onclick={exitSetup}
        class="text-xs text-muted-foreground hover:text-foreground"
      >
        {m.onboarding_exit_dashboard()}
      </Button>
    </div>
  </footer>
</div>
