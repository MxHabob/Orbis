<script lang="ts">
import { Input } from "../components/ui/input";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "../components/ui/select";
import {
  FilterOperator,
  type FilterOperatorType,
  type FilterCondition,
  type FieldDefinition,
  type FilterTranslations,
  defaultFilterTranslations,
} from "./types";

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

// Some operators don't need a value input
let needsValue = $derived(
  !(
    [
      FilterOperator.IsNull,
      FilterOperator.IsNotNull,
      FilterOperator.BoolTrue,
      FilterOperator.BoolFalse,
    ] as FilterOperatorType[]
  ).includes(condition.operator),
);

function rangeValue(index: number): string {
  const value = condition.value;
  return Array.isArray(value) ? String(value[index] ?? "") : "";
}

function setRangeValue(index: number, value: string) {
  const range = Array.isArray(condition.value)
    ? [...condition.value]
    : ["", ""];
  range[index] =
    selectedField?.type === "number" && value !== "" ? Number(value) : value;
  condition.value = range;
}

function setInValue(value: string) {
  condition.value =
    condition.operator === FilterOperator.In ||
    condition.operator === FilterOperator.NotIn
      ? value
          .split(",")
          .map((item) => item.trim())
          .filter(Boolean)
      : value;
}

const isRange = $derived(condition.operator === FilterOperator.Between);
const isList = $derived(
  condition.operator === FilterOperator.In ||
    condition.operator === FilterOperator.NotIn,
);
</script>

{#if needsValue}
    {#if isRange}
        <div class="grid min-w-0 grid-cols-2 gap-2">
            <Input
                type={selectedField?.type === "date" ? "date" : "number"}
                value={rangeValue(0)}
                oninput={(event) => setRangeValue(0, event.currentTarget.value)}
                class="h-9 w-full bg-background text-xs"
                disabled={!condition.field}
                placeholder={translations.valuePlaceholder}
                aria-label={translations.operatorBetween}
            />
            <Input
                type={selectedField?.type === "date" ? "date" : "number"}
                value={rangeValue(1)}
                oninput={(event) => setRangeValue(1, event.currentTarget.value)}
                class="h-9 w-full bg-background text-xs"
                disabled={!condition.field}
                placeholder={translations.valuePlaceholder}
                aria-label={translations.operatorBetween}
            />
        </div>
    {:else if selectedField?.type === 'select' && selectedField.options}
        <Select type="single" value={String(condition.value ?? "")} onValueChange={(v) => { if(v !== undefined) condition.value = v }}>
            <SelectTrigger class="w-full bg-background h-9 text-xs" disabled={!condition.field}>
                {selectedField.options.find(o => String(o.value) === String(condition.value))?.label || translations.selectValue}
            </SelectTrigger>
            <SelectContent>
                {#each selectedField.options as opt (String(opt.value))}
                    <SelectItem value={String(opt.value)}>{opt.label}</SelectItem>
                {/each}
            </SelectContent>
        </Select>
    {:else if selectedField?.type === 'date'}
        <Input type="date" bind:value={condition.value} class="w-full bg-background h-9 text-xs" disabled={!condition.field} />
    {:else if selectedField?.type === 'number'}
        <Input
          type="number"
          value={String(condition.value ?? "")}
          oninput={(event) => setInValue(event.currentTarget.value)}
          class="h-9 w-full bg-background text-xs"
          disabled={!condition.field}
          placeholder={isList ? `${translations.valuePlaceholder} (comma separated)` : translations.valuePlaceholder}
        />
    {:else if selectedField?.type === 'boolean'}
        <Select type="single" value={String(condition.value ?? "")} onValueChange={(v) => { if(v !== undefined) condition.value = v === 'true' }}>
            <SelectTrigger class="w-full bg-background h-9 text-xs" disabled={!condition.field}>
                {condition.value === true ? translations.trueValue : condition.value === false ? translations.falseValue : translations.selectValue}
            </SelectTrigger>
            <SelectContent>
                <SelectItem value="true">{translations.trueValue}</SelectItem>
                <SelectItem value="false">{translations.falseValue}</SelectItem>
            </SelectContent>
        </Select>
    {:else}
        <Input
          type="text"
          value={Array.isArray(condition.value) ? condition.value.join(", ") : String(condition.value ?? "")}
          oninput={(event) => setInValue(event.currentTarget.value)}
          class="h-9 w-full bg-background text-xs"
          disabled={!condition.field}
          placeholder={isList ? `${translations.valuePlaceholder} (comma separated)` : translations.valuePlaceholder}
        />
    {/if}
{:else}
    <div class="flex items-center text-xs text-muted-foreground italic px-3 h-9 bg-muted/50 rounded-md border border-transparent">
        {translations.notApplicable}
    </div>
{/if}
