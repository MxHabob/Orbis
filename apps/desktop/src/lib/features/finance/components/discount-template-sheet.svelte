<!--
  @component DiscountTemplateSheet
  Side panel for creating/editing a discount template.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import * as Sheet from "@orbis/ui/sheet";
import DiscountTemplateForm from "./discount-template-form.svelte";

let {
  open = $bindable(false),
  orgId,
  template,
  onSuccess,
}: {
  open: boolean;
  orgId: string;
  template?: any;
  onSuccess: () => void;
} = $props();
</script>

<Sheet.Root bind:open>
  <Sheet.Content side="right" class="w-105 sm:w-120 h-full p-0 flex flex-col overflow-hidden">
    <DiscountTemplateForm 
      {orgId} 
      {template}
      onSuccess={() => {
        open = false;
        onSuccess();
      }} 
      onCancel={() => (open = false)} 
    />
  </Sheet.Content>
</Sheet.Root>
