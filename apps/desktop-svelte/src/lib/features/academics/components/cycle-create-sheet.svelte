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
  <SheetContent class="flex flex-col h-full sm:max-w-xl overflow-hidden">
    <SheetHeader class="text-start select-none border-b border-border/40 pb-4">
      <SheetTitle class="text-lg font-bold text-foreground">
        {isEdit ? "Edit Academic Cycle" : m.cycle_form_title_create()}
      </SheetTitle>
      <SheetDescription class="text-xs text-muted-foreground mt-1">
        {m.cycle_form_desc()}
      </SheetDescription>
    </SheetHeader>

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
