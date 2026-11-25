<script lang="ts">
import { TrialBalanceReport } from "$lib/features/finance/components/reports";

let { data }: { data: { orgId: string } } = $props();
</script>

<TrialBalanceReport orgId={data.orgId} />
