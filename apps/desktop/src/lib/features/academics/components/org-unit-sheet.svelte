<!--
@component OrgUnitSheet
A slide-over Sheet component for creating or editing an organizational unit.
Uses the shared @orbis/ui/sheet component.
-->
<script lang="ts">
import { Sheet, SheetContent, SheetHeader, SheetTitle } from "@orbis/ui/sheet";
import OrgUnitForm from "./org-unit-form.svelte";
import * as m from "$lib/paraglide/messages";

let {
  open = $bindable(false),
  orgId,
  unit = null,
  defaultParentId = "",
  onSuccess,
}: {
  open: boolean;
  orgId: string;
  unit?: any;
  defaultParentId?: string;
  onSuccess: () => void;
} = $props();

const isEdit = $derived(!!unit);
</script>

<Sheet bind:open>
  <SheetContent
    contentClass="data-[side=end]:sm:!max-w-xl"
    class="flex flex-col h-full overflow-hidden p-0 gap-0"
  >
    <OrgUnitForm
      {orgId}
      {unit}
      {defaultParentId}
      onSuccess={() => {
        open = false;
        onSuccess();
      }}
      onCancel={() => open = false}
    />
  </SheetContent>
</Sheet>
