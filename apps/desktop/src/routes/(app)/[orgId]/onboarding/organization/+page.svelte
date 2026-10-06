<script lang="ts">
import { page } from "$app/state";
import Step1Organization from "$lib/features/onboarding-wizard/steps/step-1-organization.svelte";

const orgId = $derived(page.params.orgId);
</script>

<Step1Organization {orgId} />
