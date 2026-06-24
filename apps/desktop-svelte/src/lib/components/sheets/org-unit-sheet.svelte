<!--
@component OrgUnitSheet
A slide-over Sheet component for creating or editing an organizational unit.
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
import OrgUnitForm from "../forms/org-unit-form.svelte";
import * as m from "$lib/paraglide/messages";

let {
  open = $bindable(false),
  orgId,
  unit = null,
  onSuccess,
}: {
  open: boolean;
  orgId: string;
  unit?: any;
  onSuccess: () => void;
} = $props();

const isEdit = $derived(!!unit);
</script>

<Sheet bind:open>
  <SheetContent class="flex flex-col h-full sm:max-w-xl overflow-hidden">
    <SheetHeader class="text-start select-none border-b border-border/40 pb-4">
      <SheetTitle class="text-lg font-bold text-foreground">
        {isEdit ? "Edit Organizational Unit" : "Create Organizational Unit"}
      </SheetTitle>
      <SheetDescription class="text-xs text-muted-foreground mt-1">
        Configure sites, buildings, departments, or classrooms.
      </SheetDescription>
    </SheetHeader>

    <OrgUnitForm
      {orgId}
      {unit}
      onSuccess={() => {
        open = false;
        onSuccess();
      }}
      onCancel={() => open = false}
    />
  </SheetContent>
</Sheet>
