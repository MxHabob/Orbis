<script lang="ts">
import FinancialReportPage from "$lib/features/finance/components/financial-report-page.svelte";

let { data }: { data: { orgId: string } } = $props();
</script>

<FinancialReportPage kind="trial_balance" orgId={data.orgId} />
