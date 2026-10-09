<script lang="ts">
  import * as m from "$lib/paraglide/messages";
  import { licenseStore, uiStore } from "$lib/stores";
  import { Button, toast } from "@orbis/ui";
  import type { Snippet } from "svelte";
  import type { EntitlementKey } from "$lib/api/types";

  let {
    feature = null,
    licenseReady,
    className = "",
    children,
  }: {
    feature?: EntitlementKey | null;
    licenseReady: boolean;
    className?: string;
    children: Snippet;
  } = $props();

  let isStartingTrial = $state(false);
  const locked = $derived(
    licenseReady &&
      feature !== null &&
      !licenseStore.isFeatureEnabled(feature),
  );
  const canStartTrial = $derived(
    licenseStore.state === "unavailable" &&
      feature !== null &&
      licenseStore.trialFeatures.includes(feature),
  );

  async function startTrial() {
    isStartingTrial = true;
    const trial = await licenseStore.startTrial();
    if (trial) {
      toast.success(m.license_trial_started());
    } else if (licenseStore.error) {
      toast.error(licenseStore.error);
    }
    isStartingTrial = false;
  }
</script>

<div
  class="relative h-full min-h-0 min-w-0 overflow-hidden {className || 'flex-1'}"
>
  <div
    class="h-full min-h-0 min-w-0 {locked ? 'pointer-events-none select-none blur-sm' : ''}"
    aria-hidden={locked}
    inert={locked}
  >
    {@render children()}
  </div>

  {#if locked}
    <div
      class="absolute inset-0 z-40 flex items-center justify-center overflow-auto bg-background/75 p-6 backdrop-blur-sm"
      role="status"
    >
      <div class="flex max-w-md flex-col items-center gap-3 rounded-xl border border-border bg-card/95 p-6 text-center shadow-xl">
        <h2 class="text-base font-semibold text-foreground">
          {m.license_gate_title()}
        </h2>
        <p class="text-sm text-muted-foreground">
          {m.license_gate_description()}
        </p>
        <Button
          onclick={() => uiStore.setActivationDialog(true)}
          variant={licenseStore.state === "unavailable" ? "outline" : "default"}
          disabled={isStartingTrial}
        >
          {m.license_activate_btn()}
        </Button>
        {#if canStartTrial}
          <Button onclick={startTrial} disabled={isStartingTrial}>
            {#if isStartingTrial}
              {m.common_saving()}
            {:else}
              {m.license_start_trial_btn()}
            {/if}
          </Button>
        {/if}
      </div>
    </div>
  {/if}
</div>
