<script lang="ts">
import PromotionHistory from "$lib/features/academics/promotion-history.svelte";

let { data }: { data: { orgId: string } } = $props();
</script>

<div class="h-full w-full overflow-y-auto scrollbar-hide">
  <div class="max-w-4xl mx-auto p-6">
    <PromotionHistory orgId={data.orgId} userId="" />
  </div>
</div>
