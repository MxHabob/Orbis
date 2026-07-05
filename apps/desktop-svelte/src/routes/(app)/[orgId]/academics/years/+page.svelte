<script lang="ts">
import YearManagement from "$lib/features/academics/YearManagement.svelte";

let { data }: { data: { orgId: string } } = $props();
</script>

<YearManagement orgId={data.orgId} />
