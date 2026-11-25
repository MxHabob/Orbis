<script lang="ts">
import { FinancialOverviewReport } from "$lib/features/finance/components/reports";

let { data }: { data: { orgId: string } } = $props();
</script>

<FinancialOverviewReport orgId={data.orgId} />
