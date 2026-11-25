<script lang="ts">
import { page } from "$app/state";
import ReportingWorkspace from "../reporting-workspace.svelte";

let { definitionId }: { definitionId: string } = $props();
</script>

<ReportingWorkspace orgId={page.params.orgId ?? ""} reportId={definitionId} />
