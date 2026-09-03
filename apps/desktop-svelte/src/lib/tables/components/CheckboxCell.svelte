<script lang="ts">
import { Checkbox } from "@orbis/ui-svelte/checkbox";
import { getContext } from "svelte";

interface Props {
  row?: Record<string, any>;
  column: { id: string; meta?: Record<string, any> };
  api?: any;
}

let { row, column }: Props = $props();

const gridAction = getContext<((ev: any) => void) | undefined>("orbis-grid-onaction");

const isHeader = $derived(column?.meta?.isSelectionHeader === true);
const isChecked = $derived(isHeader ? column?.meta?.selectedAll === true : !!row?.__selected || !!row?.[column?.id]);

function toggle() {
  gridAction?.({
    actionId: isHeader ? "select-all" : "toggle-select",
    row: row ?? {},
    rowIndex: row?.__rowIndex ?? -1,
    prop: column.id,
    originalEvent: new MouseEvent("click"),
  });
}
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<div 
  class="flex items-center justify-center w-full h-full cursor-pointer select-none py-1"
  onclick={(e) => {
    e.stopPropagation();
    toggle();
  }}
>
  <Checkbox
    checked={isChecked}
    class="cursor-pointer h-4 w-4 pointer-events-none"
  />
</div>
