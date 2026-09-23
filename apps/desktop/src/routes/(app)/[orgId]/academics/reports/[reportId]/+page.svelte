<script lang="ts">
import { page } from "$app/state";
import DataReportViewer from "$lib/features/reporting/components/data-report-viewer.svelte";
</script>

<DataReportViewer definitionId={page.params.reportId} />
