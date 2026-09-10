<script lang="ts">
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "../components/ui/select";
import {
  FilterOperator,
  type FilterCondition,
  type FieldDefinition,
} from "./types";

let {
  condition = $bindable(),
  fields = [],
}: {
  condition: FilterCondition;
  fields: FieldDefinition[];
} = $props();

let selectedField = $derived(fields.find((f) => f.id === condition.field));

const operatorLabels: Record<FilterOperator, string> = {
  [FilterOperator.Eq]: "=",
  [FilterOperator.Ne]: "≠",
  [FilterOperator.Gt]: ">",
  [FilterOperator.Gte]: "≥",
  [FilterOperator.Lt]: "<",
  [FilterOperator.Lte]: "≤",
  [FilterOperator.Like]: "Contains",
  [FilterOperator.NotLike]: "Not Contains",
  [FilterOperator.In]: "In",
  [FilterOperator.NotIn]: "Not In",
  [FilterOperator.IsNull]: "Is Empty",
  [FilterOperator.IsNotNull]: "Is Not Empty",
};

// Available operators based on field type
let operators = $derived.by(() => {
  if (!selectedField) return Object.values(FilterOperator);
  switch (selectedField.type) {
    case "string":
      return [
        FilterOperator.Eq,
        FilterOperator.Ne,
        FilterOperator.Like,
        FilterOperator.NotLike,
        FilterOperator.IsNull,
        FilterOperator.IsNotNull,
      ];
    case "number":
    case "date":
      return [
        FilterOperator.Eq,
        FilterOperator.Ne,
        FilterOperator.Gt,
        FilterOperator.Gte,
        FilterOperator.Lt,
        FilterOperator.Lte,
        FilterOperator.IsNull,
        FilterOperator.IsNotNull,
      ];
    case "boolean":
      return [
        FilterOperator.Eq,
        FilterOperator.Ne,
        FilterOperator.IsNull,
        FilterOperator.IsNotNull,
      ];
    case "select":
      return [
        FilterOperator.Eq,
        FilterOperator.Ne,
        FilterOperator.In,
        FilterOperator.NotIn,
        FilterOperator.IsNull,
        FilterOperator.IsNotNull,
      ];
    default:
      return Object.values(FilterOperator);
  }
});
</script>

<Select type="single" value={condition.operator} onValueChange={(v) => { if(v) condition.operator = v as FilterOperator }}>
    <SelectTrigger class="w-full bg-background h-9 text-xs" disabled={!condition.field}>
        {operatorLabels[condition.operator] || condition.operator}
    </SelectTrigger>
    <SelectContent>
        {#each operators as op}
            <SelectItem value={op}>{operatorLabels[op]}</SelectItem>
        {/each}
    </SelectContent>
</Select>
