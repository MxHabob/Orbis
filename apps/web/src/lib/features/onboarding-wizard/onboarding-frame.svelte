<script lang="ts">
import { afterNavigate } from "$app/navigation";
import { page } from "$app/state";
import { onDestroy, onMount } from "svelte";
import { createOnboardingStateQuery } from "./onboarding-queries";
import type { Snippet } from "svelte";
import * as m from "$lib/paraglide/messages";
import { effectiveContext, systemStore } from "$lib/stores";
import { Button } from "@orbis/ui/button";
import { LoadingIndicator } from "@orbis/ui";
import { Skeleton } from "@orbis/ui/skeleton";
import { Card, CardContent } from "@orbis/ui/card";
import SetupWorkspace from "$lib/shared/components/setup-workspace.svelte";
import SetupWizardHeader from "$lib/shared/components/setup-wizard-header.svelte";
import SetupWizardFooter from "$lib/shared/components/setup-wizard-footer.svelte";
import { deriveSteps } from "./onboarding-steps";
import { navigateOnboardingRoute } from "./navigation";

let { orgId, children }: { orgId: string; children: Snippet } = $props();
let hasLoadedState = $state(false);
let isLoadingState = $state(true);
let stateLoadError = $state(false);
const onboardingStateQuery = createOnboardingStateQuery(() => orgId, false);

const currentPath = $derived(page.url.pathname.replace(/\/$/, ""));

afterNavigate(({ from, to }) => {
  if (from?.url.pathname !== to?.url.pathname) {
    document.getElementById("onboarding-viewport")?.scrollTo(0, 0);
  }
});

let disposed = false;
onDestroy(() => { disposed = true; });
onMount(() => { void loadOnboardingState(); });

async function loadOnboardingState() {
  isLoadingState = true;
  stateLoadError = false;
  hasLoadedState = false;
  systemStore.onboardingState = null;
  try {
    const result = await onboardingStateQuery.refetch();
    if (disposed) return;
    if (result.isError || result.data?.org_id !== orgId) {
      systemStore.onboardingState = null;
      stateLoadError = true;
      return;
    }
    systemStore.onboardingState = result.data;
    hasLoadedState = true;
  } catch (error) {
    if (disposed) return;
    console.error("Failed to load onboarding state:", error);
    systemStore.onboardingState = null;
    stateLoadError = true;
  } finally {
    isLoadingState = false;
  }
}

const steps = $derived(
  deriveSteps(systemStore.onboardingTasks, (permission) =>
    effectiveContext.hasPermission(permission),
  ),
);
const overallProgress = $derived(systemStore.onboardingProgressPercentage);

const previousStepIndex = $derived(
  steps.findLastIndex((step, index) => index < currentStepIndex && step.isAccessible),
);
const nextStepIndex = $derived(
  steps.findIndex((step, index) => index > currentStepIndex && step.isAccessible),
);

const currentStepIndex = $derived(
  steps.findIndex((step) => currentPath.endsWith(`/onboarding/${step.route}`)),
);
const currentStep = $derived(
  currentStepIndex >= 0 ? steps[currentStepIndex] : null,
);
const stepPosition = $derived(currentStep ? `${currentStepIndex + 1} / ${steps.length}` : "");

async function exitSetup() {
  await navigateOnboardingRoute(`/${orgId}`);
}

async function goToOverview() {
  await navigateOnboardingRoute(`/${orgId}/onboarding`);
}

async function goToStep(index: number) {
  const step = steps[index];
  if (step?.isAccessible) {
    await navigateOnboardingRoute(`/${orgId}/onboarding/${step.route}`);
  }
}

</script>

<SetupWorkspace
  viewportId="onboarding-viewport"
>
  {#snippet header()}
    <SetupWizardHeader
      contextLabel={systemStore.onboardingState?.org_name || orgId}
      stepLabel={currentStep?.label ?? m.common_nav_overview()}
      {stepPosition}
      progressValue={hasLoadedState ? overallProgress : 0}
      ariaLabel={m.onboarding_stages_title()}
      isLoading={isLoadingState}
      closeLabel={m.onboarding_exit_dashboard()}
      onClose={exitSetup}
    />
  {/snippet}

  {#if isLoadingState || stateLoadError}
    <div class="mx-auto flex min-h-64 w-full max-w-xl flex-col items-center justify-center gap-4 px-4 py-10 text-center">
      {#if isLoadingState}
        <Card class="w-full">
          <CardContent class="flex-row items-center gap-4 p-6">
            <LoadingIndicator size={28} class="size-7 text-primary" label={m.onboarding_state_loading()} />
            <div class="flex-1 space-y-3" aria-hidden="true">
              <Skeleton class="h-4 w-1/3" />
              <Skeleton class="h-3 w-full" />
              <Skeleton class="h-3 w-4/5" />
            </div>
          </CardContent>
        </Card>
        <p class="sr-only" aria-live="polite">{m.onboarding_state_loading()}</p>
      {:else}
        <Card class="w-full">
          <CardContent class="space-y-4 p-6 text-start">
          <p class="text-sm font-semibold text-destructive" role="alert">
            {m.onboarding_state_load_failed()}
          </p>
          <Button variant="outline" size="sm" onclick={loadOnboardingState}>
            {m.onboarding_retry()}
          </Button>
          </CardContent>
        </Card>
      {/if}
    </div>
  {:else}
    {#if currentStep && !currentStep.isAccessible}
      <div class="mx-auto flex min-h-64 w-full max-w-xl flex-col items-center justify-center gap-4 px-4 py-10 text-center">
        <Card class="w-full">
          <CardContent class="space-y-4 p-6">
          <div class="space-y-2">
            <h2 class="text-base font-semibold text-foreground">
              {m.onboarding_step_unavailable_title()}
            </h2>
            <p class="text-sm text-muted-foreground">
              {m.onboarding_step_unavailable_description()}
            </p>
          </div>
          <Button variant="outline" size="sm" onclick={goToOverview}>
            {m.common_nav_overview()}
          </Button>
          </CardContent>
        </Card>
      </div>
    {:else}
      {@render children()}
    {/if}
  {/if}

  {#snippet footer()}
    {#if hasLoadedState && currentStep}
      <SetupWizardFooter
        backLabel={m.ui_onboarding_onboarding_footer_back()}
        nextLabel={m.ui_onboarding_onboarding_footer_continue()}
        secondaryLabel={m.common_nav_overview()}
        canGoBack={previousStepIndex >= 0}
        canGoNext={nextStepIndex >= 0}
        onBack={previousStepIndex >= 0 ? () => goToStep(previousStepIndex) : undefined}
        onNext={nextStepIndex >= 0 ? () => goToStep(nextStepIndex) : undefined}
        onSecondary={goToOverview}
      />
    {/if}
  {/snippet}
</SetupWorkspace>
