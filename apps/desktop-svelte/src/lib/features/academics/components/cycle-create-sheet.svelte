<!--
@component CycleCreateSheet
A slide-over Sheet component for creating or editing an academic cycle.
Uses the shared @orbis/ui-svelte/sheet component.
-->
<script lang="ts">
import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
  SheetDescription,
} from "@orbis/ui-svelte/sheet";
import CycleForm from "./cycle-form.svelte";
import * as m from "$lib/paraglide/messages";

let {
  open = $bindable(false),
  orgId,
  cycle = null,
  onSuccess,
}: {
  open: boolean;
  orgId: string;
  cycle?: any;
  onSuccess: () => void;
} = $props();

const isEdit = $derived(!!cycle);
</script>

<Sheet bind:open>
  <SheetContent class="flex flex-col h-full sm:max-w-xl overflow-hidden p-0 gap-0">
    <CycleForm
      {orgId}
      {cycle}
      onSuccess={() => {
        open = false;
        onSuccess();
      }}
      onCancel={() => open = false}
    />
  </SheetContent>
</Sheet>
