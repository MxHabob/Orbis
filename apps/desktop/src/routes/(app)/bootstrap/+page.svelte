<!--
  @component BootstrapPage
  Route /bootstrap — First-time organization bootstrap and system foundation initialization.
  This is NOT onboarding (operational configuration). Bootstrap creates the technical foundation:
  Organization, Administrator, IAM, Currencies, Chart of Accounts.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { licenseStore, uiStore } from "$lib/stores";
import LicenseFeatureGate from "$lib/shell/license-feature-gate.svelte";
import { LoadingIndicator } from "@orbis/ui";
import { BootstrapWizard } from "$lib/features/bootstrap";

let licenseReady = $state(false);

onMount(async () => {
  await licenseStore.loadState();
  licenseReady = true;
});
</script>

<svelte:head>
  <title>{m.ui_onboarding_entry_organization_setup_bootstrap_orbis()}</title>
</svelte:head>

{#if licenseReady}
  <LicenseFeatureGate requireOperational {licenseReady} className="h-full w-full">
    <BootstrapWizard />
  </LicenseFeatureGate>
{:else}
  <div class="flex h-full w-full items-center justify-center bg-background">
    <LoadingIndicator size="lg" label={m.onboarding_state_loading()} />
  </div>
{/if}

{#if uiStore.showActivationDialog}
  {#await import("$lib/shell/activation-dialog.svelte") then { default: ActivationDialog }}
    <ActivationDialog bind:open={uiStore.showActivationDialog} />
  {/await}
{/if}
