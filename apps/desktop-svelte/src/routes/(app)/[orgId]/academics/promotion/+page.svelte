<script lang="ts">
import PromotionWizard from "$lib/features/academics/PromotionWizard.svelte";
import PromotionHistory from "$lib/features/academics/PromotionHistory.svelte";
import {
  Tabs,
  TabsList,
  TabsTrigger,
  TabsContent,
} from "@orbis/ui-svelte/tabs";
import * as m from "$lib/paraglide/messages";

let { data }: { data: { orgId: string } } = $props();
</script>

<div class="h-full w-full bg-background/50 overflow-y-auto scrollbar-hide">
  <div class="max-w-4xl mx-auto p-6 flex flex-col gap-6">
    <Tabs value="wizard" class="w-full">
      <TabsList class="grid w-full grid-cols-2">
        <TabsTrigger value="wizard">{m.promo_wizard_title()}</TabsTrigger>
        <TabsTrigger value="history">{m.promo_history()}</TabsTrigger>
      </TabsList>
      
      <TabsContent value="wizard" class="mt-4">
        <PromotionWizard orgId={data.orgId} />
      </TabsContent>
      
      <TabsContent value="history" class="mt-4">
        <PromotionHistory orgId={data.orgId} userId="" />
      </TabsContent>
    </Tabs>
  </div>
</div>
