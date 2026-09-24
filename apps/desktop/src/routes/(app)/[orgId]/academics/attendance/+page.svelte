<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { AttendanceTracker } from "$lib/features/attendance";

let { data }: { data: { orgId: string } } = $props();
</script>

<svelte:head>
  <title>تحضير الفصول اليومي | Orbis</title>
</svelte:head>

<AttendanceTracker orgId={data.orgId} />
