<script lang="ts">
import { page } from "$app/state";
import ScheduleGeneratorStudio from "$lib/features/academics/components/schedule-generator-studio.svelte";

const orgId = $derived(page.params.orgId || "");
</script>

<div class="h-full w-full overflow-hidden">
  <ScheduleGeneratorStudio {orgId} />
</div>
