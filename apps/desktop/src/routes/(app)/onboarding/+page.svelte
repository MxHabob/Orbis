<!--
  @component OnboardingPage
  Route /onboarding — First-time organization setup and system bootstrap.
-->
<script lang="ts">
import { BootstrapWizard } from "$lib/features/onboarding";
import { getLocale } from "$lib/paraglide/runtime";

const isRTL = $derived(getLocale().startsWith("ar"));
</script>

<svelte:head>
  <title>{isRTL ? "تهيئة وتأسيس المنشأة — Orbis" : "Organization Setup & Bootstrap — Orbis"}</title>
</svelte:head>

<BootstrapWizard />
