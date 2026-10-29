<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import ReportingWorkspace from "$lib/features/reporting/reporting-workspace.svelte";

let { data }: { data: { orgId: string } } = $props();
const orgId = $derived(data?.orgId || page.params.orgId || "");
</script>

<svelte:head>
  <title>{m.nav_reports()} - Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-hidden">
  <ReportingWorkspace {orgId} />
</div>
