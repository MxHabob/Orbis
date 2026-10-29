<script lang="ts">
import { page } from "$app/state";
import * as m from "$lib/paraglide/messages";
import { ScheduleWorkspace } from "$lib/features/academics";

const orgId = $derived(page.params.orgId || "");
</script>

<svelte:head>
  <title>{m.common_exams_detail_schedule()} | Orbis</title>
</svelte:head>

<ScheduleWorkspace {orgId} />
