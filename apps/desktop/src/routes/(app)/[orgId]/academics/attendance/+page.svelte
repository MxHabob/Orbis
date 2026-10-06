<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { AttendanceTracker } from "$lib/features/attendance";

let { data }: { data: { orgId: string } } = $props();
</script>

<svelte:head>
  <title>{m.ui_page_academics_attendance()} | Orbis</title>
</svelte:head>

<AttendanceTracker orgId={data.orgId} />
