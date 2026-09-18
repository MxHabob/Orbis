<!--
  Initial Setup route wrapper for school onboarding.
  Checks authentication and loads readiness.
-->
<script lang="ts">
import { onMount } from "svelte";
import { goto } from "$app/navigation";
import { authStore, systemStore } from "$lib/stores";
import { localizeHref } from "$lib/paraglide/runtime";
import OnboardingWizard from "$lib/features/onboarding/onboarding-wizard.svelte";

onMount(async () => {
  await authStore.restore();
  if (!authStore.isAuthenticated) {
    goto(localizeHref("/login"), { replaceState: true });
    return;
  }

  const readiness = await systemStore.loadReadiness();
  if (readiness && readiness.state === "Operational") {
    goto(localizeHref(`/${authStore.orgId}`), { replaceState: true });
  }
});
</script>

<OnboardingWizard />