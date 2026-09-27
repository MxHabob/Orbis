<!--
  @component FeeTemplateSheet
  Side panel for creating/editing a fee template.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import * as Sheet from "@orbis/ui/sheet";
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
  <Sheet.Content side="right" class="w-105 sm:w-120 h-full p-0 flex flex-col overflow-hidden">
    {#if open}
      {#key (template?.id || "new")}
        <FeeTemplateForm 
          {orgId} 
          {template}
          onSuccess={() => {
            open = false;
            onSuccess();
          }} 
          onCancel={() => (open = false)} 
        />
      {/key}
    {/if}
  </Sheet.Content>
</Sheet.Root>
