<script lang="ts">
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "../components/ui/select";
import type { FieldDefinition, FilterCondition } from "./types";
import { FilterOperator } from "./types";

let {
  condition = $bindable(),
  fields = [],
}: {
  condition: FilterCondition;
  fields: FieldDefinition[];
} = $props();

let selectedField = $derived(fields.find((f) => f.id === condition.field));

function handleFieldChange(v: string | undefined) {
  if (!v) return;
  const newField = fields.find((f) => f.id === v);
  const oldField = fields.find((f) => f.id === condition.field);
  condition.field = v;

  if (newField?.type !== oldField?.type) {
    condition.value = "";
    condition.operator = FilterOperator.Eq;
  }
}
</script>

<Select type="single" value={condition.field} onValueChange={handleFieldChange}>
    <SelectTrigger class="w-full bg-background h-9 text-xs">
        {selectedField?.label || "Select Field"}
    </SelectTrigger>
    <SelectContent>
        {#each fields as field}
            <SelectItem value={field.id}>{field.label}</SelectItem>
        {/each}
    </SelectContent>
</Select>
