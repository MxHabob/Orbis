<script lang="ts">
import PromotionWizard from "$lib/features/academics/promotion-wizard.svelte";

let { data }: { data: { orgId: string } } = $props();
</script>

<div class="h-full w-full overflow-y-auto scrollbar-hide">
  <div class="max-w-6xl mx-auto p-6 flex flex-col gap-6">
    <PromotionWizard orgId={data.orgId} />
  </div>
</div>

