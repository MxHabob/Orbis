<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Checkbox } from "@orbis/ui/checkbox";

let { row }: Record<string, unknown> = $props();
const record = $derived(row && typeof row === "object" ? row as Record<string, unknown> : {});

const id = $derived(String(record.id ?? ""));
const name = $derived(String(record.name ?? ""));
const selected = $derived(record.selected === true);

function isToggleHandler(
  value: unknown,
): value is (id: string, selected: boolean) => void {
  return typeof value === "function";
}
</script>

<Checkbox
  checked={selected}
  onCheckedChange={(value) => {
    if (isToggleHandler(record.toggleSelection)) {
      record.toggleSelection(id, value === true);
    }
  }}
  aria-label={m.operations_ui_document_select({ name })} />
