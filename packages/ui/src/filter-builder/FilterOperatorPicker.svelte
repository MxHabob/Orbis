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
  type FilterTranslations,
} from "./types";
import { defaultFilterTranslations } from "./types";

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

const operatorLabels = $derived<Record<string, string>>({
  [FilterOperator.Eq]: translations.operatorEqual,
  eq: translations.operatorEqual,
  equals: translations.operatorEqual,
  is: translations.operatorIs,
  [FilterOperator.Ne]: translations.operatorNotEqual,
  [FilterOperator.Gt]: translations.operatorGreaterThan,
  [FilterOperator.Gte]: translations.operatorGreaterThanOrEqual,
  [FilterOperator.Lt]: translations.operatorLessThan,
  [FilterOperator.Lte]: translations.operatorLessThanOrEqual,
  [FilterOperator.Like]: translations.operatorContains,
  [FilterOperator.NotLike]: translations.operatorNotContains,
  [FilterOperator.In]: translations.operatorIn,
  [FilterOperator.NotIn]: translations.operatorNotIn,
  [FilterOperator.StartsWith]: translations.operatorStartsWith,
  [FilterOperator.EndsWith]: translations.operatorEndsWith,
  [FilterOperator.Between]: translations.operatorBetween,
  [FilterOperator.IsNull]: translations.operatorIsEmpty,
  [FilterOperator.IsNotNull]: translations.operatorIsNotEmpty,
  [FilterOperator.BoolTrue]: translations.trueValue,
  [FilterOperator.BoolFalse]: translations.falseValue,
});

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
        FilterOperator.StartsWith,
        FilterOperator.EndsWith,
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
        FilterOperator.Between,
        FilterOperator.IsNull,
        FilterOperator.IsNotNull,
      ];
    case "boolean":
      return [
        FilterOperator.Eq,
        FilterOperator.Ne,
        FilterOperator.BoolTrue,
        FilterOperator.BoolFalse,
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
        {#each operators as op (op)}
            <SelectItem value={op}>{operatorLabels[op]}</SelectItem>
        {/each}
    </SelectContent>
</Select>
