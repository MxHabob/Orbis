<script lang="ts">
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "../components/ui/select";
import {
  defaultFilterTranslations,
  type FieldDefinition,
  type FilterCondition,
  type FilterTranslations,
} from "./types";
import { FilterOperator } from "./types";

let {
  condition = $bindable(),
  fields = [],
  translations = defaultFilterTranslations,
}: {
  condition: FilterCondition;
  fields: FieldDefinition[];
  translations?: FilterTranslations;
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
        {selectedField?.label || translations.selectField}
    </SelectTrigger>
    <SelectContent>
        {#each fields as field (field.id)}
            <SelectItem value={field.id}>{field.label}</SelectItem>
        {/each}
    </SelectContent>
</Select>
