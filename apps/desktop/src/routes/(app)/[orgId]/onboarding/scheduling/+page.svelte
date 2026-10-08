<script lang="ts">
  import { page } from "$app/state";
  import StepScheduling from "$lib/features/onboarding-wizard/steps/step-scheduling.svelte";

  const orgId = $derived(page.params.orgId);
</script>

<StepScheduling {orgId} />
