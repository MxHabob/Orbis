<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Checkbox } from "@orbis/ui/checkbox";
import type { GridRow, OrbisColumnDef } from "@orbis/svelte-grid";

let { row }: { row: GridRow; column: OrbisColumnDef } = $props();

const id = $derived(String(row.id ?? ""));
const name = $derived(String(row.name ?? ""));
const selected = $derived(row.selected === true);

function isToggleHandler(
  value: unknown,
): value is (id: string, selected: boolean) => void {
  return typeof value === "function";
}
</script>

<Checkbox
  checked={selected}
  onCheckedChange={(value) => {
    if (isToggleHandler(row.toggleSelection)) {
      row.toggleSelection(id, value === true);
    }
  }}
  aria-label={m.operations_ui_document_select({ name })} />
