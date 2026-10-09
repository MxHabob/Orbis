<script lang="ts">
  import * as m from "$lib/paraglide/messages";
  import { licenseStore, uiStore } from "$lib/stores";
  import { Button, toast } from "@orbis/ui";
  import type { Snippet } from "svelte";
  import type { EntitlementKey } from "$lib/api/types";

  let {
    feature = null,
    requireOperational = false,
    licenseReady,
    className = "",
    children,
  }: {
    feature?: EntitlementKey | null;
    requireOperational?: boolean;
    licenseReady: boolean;
    className?: string;
    children: Snippet;
  } = $props();

  let isStartingTrial = $state(false);
  let isRefreshingTrial = $state(false);
  const isTrialAwaitingOrganization = $derived(
    licenseStore.state === "pending" &&
      licenseStore.plan?.toUpperCase() === "TRIAL" &&
      !licenseStore.licenseState?.organization_id,
  );
  const locked = $derived(
    licenseReady &&
      ((requireOperational && !licenseStore.isOperational) ||
        (feature !== null && !licenseStore.isFeatureEnabled(feature))),
  );
  const canRenderChildren = $derived(
    !requireOperational || (licenseReady && licenseStore.isOperational),
  );
  const canStartTrial = $derived(
    licenseStore.state === "unavailable" &&
      (feature === null
        ? licenseStore.trialFeatures.length > 0
        : licenseStore.trialFeatures.includes(feature)),
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

  async function refreshTrialBinding() {
    isRefreshingTrial = true;
    try {
      const refreshed = await licenseStore.refresh();
      if (refreshed?.is_operational) {
        toast.success(m.license_trial_waiting_ready());
      } else if (refreshed?.state === "pending") {
        toast.info(m.license_trial_waiting_still_pending());
      } else if (licenseStore.error) {
        toast.error(licenseStore.error);
      }
    } finally {
      isRefreshingTrial = false;
    }
  }

  function openLicenseAction() {
    if (licenseStore.isOperational && !requireOperational) {
      uiStore.setUpgradeDialog(true);
      return;
    }
    uiStore.setActivationDialog(true);
  }
</script>

<div
  class="relative h-full min-h-0 min-w-0 overflow-hidden {className || 'flex-1'}"
>
  {#if canRenderChildren}
    <div
      class="h-full min-h-0 min-w-0 {locked ? 'pointer-events-none select-none blur-sm' : ''}"
      aria-hidden={locked}
      inert={locked}
    >
      {@render children()}
    </div>
  {/if}

  {#if locked}
    <div
      class="absolute inset-0 z-40 flex items-center justify-center overflow-auto bg-background/75 p-6 backdrop-blur-sm"
      role="status"
    >
      <div class="flex max-w-md flex-col items-center gap-3 rounded-xl border border-border bg-card/95 p-6 text-center shadow-xl">
        <h2 class="text-base font-semibold text-foreground">
          {#if isTrialAwaitingOrganization}
            {m.license_trial_waiting_title()}
          {:else if requireOperational}
            {m.license_setup_step_title()}
          {:else}
            {m.license_gate_title()}
          {/if}
        </h2>
        <p class="text-sm text-muted-foreground">
          {#if isTrialAwaitingOrganization}
            {m.license_trial_waiting_description()}
          {:else if requireOperational}
            {m.license_setup_step_description()}
          {:else}
            {m.license_gate_description()}
          {/if}
        </p>
        {#if isTrialAwaitingOrganization}
          <Button
            onclick={refreshTrialBinding}
            variant="outline"
            disabled={isRefreshingTrial}
          >
            {isRefreshingTrial
              ? m.license_trial_waiting_checking()
              : m.license_trial_waiting_check()}
          </Button>
        {/if}
        <Button
          onclick={openLicenseAction}
          variant={licenseStore.state === "unavailable" ? "outline" : "default"}
          disabled={isStartingTrial}
        >
          {#if requireOperational || !licenseStore.isOperational}
            {requireOperational
              ? m.license_setup_activate_product()
              : m.license_activate_btn()}
          {:else}
            {m.common_upgrade_btn_now()}
          {/if}
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
