<!--
  @component OnboardingPage
  Route /onboarding — First-time organization setup and system bootstrap.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { BootstrapWizard } from "$lib/features/onboarding";
import { getLocale } from "$lib/paraglide/runtime";

const isRTL = $derived(getLocale().startsWith("ar"));
</script>

<svelte:head>
  <title>{m.ui_onboarding_entry_organization_setup_bootstrap_orbis()}</title>
</svelte:head>

<BootstrapWizard />
