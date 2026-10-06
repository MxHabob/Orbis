<script lang="ts">
import { page } from "$app/state";
import Step3Calendar from "$lib/features/onboarding-wizard/steps/step-3-calendar.svelte";

const orgId = $derived(page.params.orgId);
</script>

<Step3Calendar {orgId} />
