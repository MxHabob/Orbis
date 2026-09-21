<!--
  Unified Onboarding Route Wrapper.
  Handles both first-time Organization Bootstrap + Academic Onboarding.
-->
<script lang="ts">
import { onMount } from "svelte";
import { goto } from "$app/navigation";
import { authStore, systemStore } from "$lib/stores";
import { api } from "$lib/api/client";
import { localizeHref } from "$lib/paraglide/runtime";
import OnboardingWizard from "$lib/features/onboarding/onboarding-wizard.svelte";

let isChecking = $state(true);

onMount(async () => {
  try {
    await authStore.restore();
    const hasOrg = await api.systemHasOrganization();

    // If org exists but user is not logged in, redirect to login
    if (hasOrg && !authStore.isAuthenticated) {
      goto(localizeHref("/login"), { replaceState: true });
      return;
    }

    // If already operational, redirect to dashboard
    if (authStore.isAuthenticated && authStore.orgId) {
      const readiness = await systemStore.loadReadiness();
      if (readiness && (readiness.state === "operational" || readiness.state === "Operational")) {
        goto(localizeHref(`/${authStore.orgId}`), { replaceState: true });
        return;
      }
    }
  } catch (e) {
    console.warn("Onboarding mount check:", e);
  } finally {
    isChecking = false;
  }
});
</script>

{#if !isChecking}
  <OnboardingWizard />
{/if}