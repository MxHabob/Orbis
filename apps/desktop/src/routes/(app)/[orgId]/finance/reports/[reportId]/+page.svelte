<script lang="ts">
import { page } from "$app/state";
import ReportViewer from "$lib/features/reporting/components/report-viewer.svelte";
</script>

<ReportViewer definitionId={page.params.reportId} />
