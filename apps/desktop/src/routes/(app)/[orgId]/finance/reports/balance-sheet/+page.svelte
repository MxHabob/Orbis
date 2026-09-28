<script lang="ts">
import { BalanceSheetReport } from "$lib/features/finance/components/reports";

let { data }: { data: { orgId: string } } = $props();
</script>

<BalanceSheetReport orgId={data.orgId} />
