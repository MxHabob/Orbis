<script lang="ts">
import { page } from "$app/state";
import Step4Academics from "$lib/features/onboarding-wizard/steps/step-4-academics.svelte";

const orgId = $derived(page.params.orgId);
</script>

<Step4Academics {orgId} />
