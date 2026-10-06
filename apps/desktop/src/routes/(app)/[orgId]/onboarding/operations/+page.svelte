<script lang="ts">
import { page } from "$app/state";
import Step7Operations from "$lib/features/onboarding-wizard/steps/step-7-operations.svelte";

const orgId = $derived(page.params.orgId);
</script>

<Step7Operations {orgId} />
