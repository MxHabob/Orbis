<script lang="ts">
import { page } from "$app/state";
import StepIntegrations from "$lib/features/onboarding-wizard/steps/step-integrations.svelte";

const orgId = $derived(page.params.orgId);
</script>

<StepIntegrations {orgId} />
