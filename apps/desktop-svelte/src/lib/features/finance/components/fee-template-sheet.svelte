<!--
  @component FeeTemplateSheet
  Side panel for creating/editing a fee template.
-->
<script lang="ts">
import * as Sheet from "@orbis/ui-svelte/sheet";
import FeeTemplateForm from "./fee-template-form.svelte";

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
  <Sheet.Content side="right" class="w-[420px] sm:w-[480px] h-full p-0 flex flex-col overflow-hidden">
    <FeeTemplateForm 
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
