<script lang="ts">
import { Button } from "../components/ui/button";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "../components/ui/select";
import { Merge, Plus, Trash } from "@orbis/icons";
import {
  FilterOperator,
  type FilterBuilderProps,
  defaultFilterTranslations,
} from "./types";
import FilterBuilder from "./FilterBuilder.svelte";
import FilterRow from "./FilterRow.svelte";
import { slide } from "svelte/transition";

let {
  filter = $bindable(),
  fields = [],
  isRoot = true,
  onRemove = () => {},
  translations = defaultFilterTranslations,
}: FilterBuilderProps = $props();

function addCondition() {
  if (!filter.conditions) filter.conditions = [];
  filter.conditions = [
    ...filter.conditions,
    {
      field: fields[0]?.id || "",
      operator: FilterOperator.Eq,
      value: "",
    },
  ];
}

function addGroup() {
  if (!filter.groups) filter.groups = [];
  filter.groups = [
    ...filter.groups,
    {
      logic: "AND",
      conditions: [],
      groups: [],
    },
  ];
}

function removeCondition(index: number) {
  filter.conditions = filter.conditions.filter((_, i) => i !== index);
}

function removeGroup(index: number) {
  if (filter.groups) {
    filter.groups = filter.groups.filter((_, i) => i !== index);
  }
}
</script>

<div class="flex min-w-0 flex-col gap-3 {isRoot ? 'w-full' : 'ms-3 border-s border-border ps-3'}">
    <div class="flex flex-wrap items-center justify-between gap-3">
        <div class="flex items-center gap-2">
            <Select type="single" value={filter.logic} onValueChange={(v) => { if (v) filter.logic = v as "AND" | "OR" }}>
                <SelectTrigger class="w-[90px] font-semibold tracking-wide text-xs h-9">
                    {filter.logic}
                </SelectTrigger>
                <SelectContent>
                    <SelectItem value="AND">AND</SelectItem>
                    <SelectItem value="OR">OR</SelectItem>
                </SelectContent>
            </Select>
            
            <span class="text-xs text-muted-foreground font-medium hidden sm:inline-block">
                {filter.logic === 'AND' ? translations.matchAll : translations.matchAny}
            </span>
        </div>

        <div class="flex items-center gap-2">
            <Button variant="outline" size="sm" onclick={addCondition} class="h-8 gap-1 text-xs">
                <Plus class="w-3.5 h-3.5" />
                <span class="hidden sm:inline">{translations.rule}</span>
            </Button>
            
            <Button variant="outline" size="sm" onclick={addGroup} class="h-8 gap-1 text-xs">
                <Merge class="w-3.5 h-3.5" />
                <span class="hidden sm:inline">{translations.group}</span>
            </Button>

            {#if !isRoot}
                <Button variant="ghost" size="icon" onclick={onRemove} class="h-8 w-8 text-destructive hover:bg-destructive/10 hover:text-destructive transition-colors ml-1">
                    <Trash class="w-3.5 h-3.5" />
                    <span class="sr-only">{translations.removeGroup}</span>
                </Button>
            {/if}
        </div>
    </div>

    {#if (filter.conditions && filter.conditions.length > 0) || (filter.groups && filter.groups.length > 0)}
        <div class="flex min-w-0 flex-col gap-2">
            {#if filter.conditions}
                {#each filter.conditions as condition, i (i)}
                    <div transition:slide={{duration: 200}}>
                        <FilterRow
                            bind:condition={filter.conditions[i]}
                            fields={fields}
                            onRemove={() => removeCondition(i)}
                        />
                    </div>
                {/each}
            {/if}
            {#if filter.groups}
                {#each filter.groups as group, i (i)}
                    <div transition:slide={{duration: 200}}>
                        <FilterBuilder
                            bind:filter={filter.groups[i]}
                            fields={fields}
                            isRoot={false}
                            translations={translations}
                            onRemove={() => removeGroup(i)}
                        />
                    </div>
                {/each}
            {/if}
        </div>
    {:else}
        <div class="flex flex-col items-center justify-center gap-2 rounded-md border border-dashed border-border px-4 py-6 text-muted-foreground">
            <p class="text-sm mb-2">{translations.noRules}</p>
            <div class="flex gap-2">
                <Button variant="secondary" size="sm" onclick={addCondition} class="text-xs">{translations.addRule}</Button>
                <Button variant="outline" size="sm" onclick={addGroup} class="text-xs">{translations.addSubGroup}</Button>
            </div>
        </div>
    {/if}
</div>
