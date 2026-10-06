<!--
  @component BootstrapPage
  Route /bootstrap — First-time organization bootstrap and system foundation initialization.
  This is NOT onboarding (operational configuration). Bootstrap creates the technical foundation:
  Organization, Administrator, IAM, Currencies, Chart of Accounts.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { BootstrapWizard } from "$lib/features/bootstrap";
</script>

<svelte:head>
  <title>{m.ui_onboarding_entry_organization_setup_bootstrap_orbis()}</title>
</svelte:head>

<BootstrapWizard />
