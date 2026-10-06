<script lang="ts">
import { page } from "$app/state";
import Step6Assessment from "$lib/features/onboarding-wizard/steps/step-6-assessment.svelte";

const orgId = $derived(page.params.orgId);
</script>

<Step6Assessment {orgId} />
