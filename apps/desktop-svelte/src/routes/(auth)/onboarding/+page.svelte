<!--
  Slim route page wrapper for school onboarding.
  Handles authentication check, setup status check, and orchestrates OnboardingWizard.
-->
<script lang="ts">
import { onMount } from "svelte";
import { goto } from "$app/navigation";
import { authStore } from "$lib/stores";
import { localizeHref } from "$lib/paraglide/runtime";
import OnboardingWizard from "$lib/components/onboarding/onboarding-wizard.svelte";
import { onboardingState } from "$lib/components/onboarding/onboarding-state.svelte";

onMount(async () => {
  authStore.restore();
  if (!authStore.isAuthenticated) {
    goto(localizeHref("/login"), { replaceState: true });
    return;
  }
  await authStore.checkSetupStatus();
  if (authStore.isSetupComplete) {
    goto(localizeHref(`/${authStore.orgId}`), { replaceState: true });
    return;
  }
  
  onboardingState.load();
  await onboardingState.loadExistingOrg(authStore.orgId);
  
  if (onboardingState.rooms.length === 0) {
    onboardingState.resetRoomsToDefault();
  }
});
</script>

<OnboardingWizard />