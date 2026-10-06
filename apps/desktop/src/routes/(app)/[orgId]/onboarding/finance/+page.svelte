<script lang="ts">
import { page } from "$app/state";
import Step5Finance from "$lib/features/onboarding-wizard/steps/step-5-finance.svelte";

const orgId = $derived(page.params.orgId);
</script>

<Step5Finance {orgId} />
