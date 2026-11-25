<!--
@component PeriodCreateSheet
A slide-over Sheet component for creating or editing an academic period.
Uses the shared @orbis/ui/sheet component.
-->
<script lang="ts">
import { Sheet, SheetContent, SheetHeader, SheetTitle } from "@orbis/ui/sheet";
import PeriodForm from "./period-form.svelte";
import * as m from "$lib/paraglide/messages";

let {
  open = $bindable(false),
  orgId,
  period = null,
  onSuccess,
}: {
  open: boolean;
  orgId: string;
  period?: any;
  onSuccess: () => void;
} = $props();

const isEdit = $derived(!!period);
</script>

<Sheet bind:open>
  <SheetContent
    contentClass="data-[side=end]:sm:!max-w-xl"
    class="flex flex-col h-full overflow-hidden p-0 gap-0"
  >
    <PeriodForm
      {orgId}
      {period}
      onSuccess={() => {
        open = false;
        onSuccess();
      }}
      onCancel={() => open = false}
    />
  </SheetContent>
</Sheet>
