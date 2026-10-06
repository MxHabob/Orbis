<script lang="ts">
import { page } from "$app/state";
import Step8Review from "$lib/features/onboarding-wizard/steps/step-8-review.svelte";

const orgId = $derived(page.params.orgId);
</script>

<Step8Review {orgId} />
