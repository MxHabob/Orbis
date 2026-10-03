<script lang="ts">
import { IncomeStatementReport } from "$lib/features/finance/components/reports";

let { data }: { data: { orgId: string } } = $props();
</script>

<IncomeStatementReport orgId={data.orgId} />
