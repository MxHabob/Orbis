<script lang="ts">
import { page } from "$app/state";
import Step2Structure from "$lib/features/onboarding-wizard/steps/step-2-structure.svelte";

const orgId = $derived(page.params.orgId);
</script>

<Step2Structure {orgId} />
