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

// Some operators don't need a value input
let needsValue = $derived(
  ![FilterOperator.IsNull, FilterOperator.IsNotNull].includes(
    condition.operator,
  ),
);
</script>

{#if needsValue}
    {#if selectedField?.type === 'select' && selectedField.options}
        <Select type="single" value={String(condition.value ?? "")} onValueChange={(v) => { if(v !== undefined) condition.value = v }}>
            <SelectTrigger class="w-full bg-background h-9 text-xs" disabled={!condition.field}>
                {selectedField.options.find(o => String(o.value) === String(condition.value))?.label || "Select Value"}
            </SelectTrigger>
            <SelectContent>
                {#each selectedField.options as opt}
                    <SelectItem value={String(opt.value)}>{opt.label}</SelectItem>
                {/each}
            </SelectContent>
        </Select>
    {:else if selectedField?.type === 'date'}
        <Input type="date" bind:value={condition.value} class="w-full bg-background h-9 text-xs" disabled={!condition.field} />
    {:else if selectedField?.type === 'number'}
        <Input type="number" bind:value={condition.value} class="w-full bg-background h-9 text-xs" disabled={!condition.field} placeholder="Value..." />
    {:else if selectedField?.type === 'boolean'}
        <Select type="single" value={String(condition.value ?? "")} onValueChange={(v) => { if(v !== undefined) condition.value = v === 'true' }}>
            <SelectTrigger class="w-full bg-background h-9 text-xs" disabled={!condition.field}>
                {condition.value === true ? 'True' : condition.value === false ? 'False' : 'Select Value'}
            </SelectTrigger>
            <SelectContent>
                <SelectItem value="true">True</SelectItem>
                <SelectItem value="false">False</SelectItem>
            </SelectContent>
        </Select>
    {:else}
        <Input type="text" bind:value={condition.value} class="w-full bg-background h-9 text-xs" disabled={!condition.field} placeholder="Value..." />
    {/if}
{:else}
    <div class="flex items-center text-xs text-muted-foreground italic px-3 h-9 bg-muted/50 rounded-md border border-transparent">
        N/A
    </div>
{/if}
