<!--
  Slim route page wrapper for school onboarding.
  Handles authentication check, setup status check, and orchestrates OnboardingWizard.
-->
<script lang="ts">
import { onMount } from "svelte";
import { goto } from "$app/navigation";
import { page } from "$app/state";
import { authStore } from "$lib/stores";
import { localizeHref, getLocale } from "$lib/paraglide/runtime";
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

  // Redirect to the correct localized URL based on onboardingState.locale
  const currentUrlLocale = getLocale();
  const stateLang = onboardingState.locale.split("-")[0]?.toLowerCase() || "ar";
  const targetLang = stateLang === "ar" ? "ar" : "en";
  if (currentUrlLocale !== targetLang) {
    const localizedUrl = localizeHref(page.url.pathname, {
      locale: targetLang,
    });
    goto(localizedUrl, { replaceState: true });
  }

});
</script>

<OnboardingWizard />