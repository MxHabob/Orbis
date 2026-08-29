<script lang="ts">
import { Checkbox } from "@orbis/ui-svelte/checkbox";

interface Props {
  row: Record<string, any>;
  column: { id: string; meta?: Record<string, any> };
  api?: any;
  onaction?: (ev: any) => void;
}

let { row, column, onaction }: Props = $props();

const isChecked = $derived(!!row.__selected || !!row[column.id]);

function handleToggle(val: boolean | "indeterminate") {
  onaction?.({
    actionId: "toggle-select",
    row,
    checked: val,
  });
}
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div 
  class="flex items-center justify-center w-full h-full"
  onclick={(e) => e.stopPropagation()}
  onkeydown={(e) => e.stopPropagation()}
>
  <Checkbox
    checked={isChecked}
    onCheckedChange={handleToggle}
    class="cursor-pointer h-4 w-4"
  />
</div>
