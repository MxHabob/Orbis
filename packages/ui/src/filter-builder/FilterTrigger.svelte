<script lang="ts">
import { Button } from "../components/ui/button";
import { AdjustmentsHorizontal, XMark } from "@orbis/icons";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
  DialogTrigger,
} from "../components/ui/dialog";
import FilterBuilder from "./FilterBuilder.svelte";
import {
  defaultFilterTranslations,
  type UniversalFilter,
  type FieldDefinition,
  type FilterTranslations,
} from "./types";

let {
  filter = $bindable(),
  fields = [],
  onApply,
  translations = defaultFilterTranslations,
}: {
  filter: UniversalFilter;
  fields: FieldDefinition[];
  onApply?: (filter: UniversalFilter) => void;
  translations?: FilterTranslations;
} = $props();

let isOpen = $state(false);
let activeCount = $derived.by(() => {
  function countConditions(f: UniversalFilter): number {
    let count = f.conditions?.filter((c) => c.field)?.length || 0;
    if (f.groups) {
      for (const g of f.groups) count += countConditions(g);
    }
    return count;
  }
  return countConditions(filter);
});

function handleApply() {
  onApply?.(filter);
  isOpen = false;
}

function handleClear() {
  filter = { logic: "AND", conditions: [], groups: [] };
  onApply?.(filter);
}
</script>

<div class="flex items-center gap-1">
    <Dialog bind:open={isOpen}>
        <DialogTrigger>
            {#snippet child({ props })}
                <Button variant="ghost" size="sm" class="h-8 gap-1.5 text-xs relative" {...props}>
                    <AdjustmentsHorizontal class="w-3.5 h-3.5" />
                    {translations.filter}
                    {#if activeCount > 0}
                        <span class="absolute -top-1 -inset-e-1 bg-primary text-primary-foreground text-[10px] font-bold w-4 h-4 rounded-full flex items-center justify-center">
                            {activeCount}
                        </span>
                    {/if}
                </Button>
            {/snippet}
        </DialogTrigger>
        <DialogContent class="max-w-3xl max-h-[80vh] overflow-y-auto">
            <DialogHeader>
                <DialogTitle>{translations.advancedFilter}</DialogTitle>
            </DialogHeader>
            <div class="py-4">
                <FilterBuilder bind:filter {fields} translations={translations} />
            </div>
            <DialogFooter class="flex items-center justify-between gap-2">
                <Button variant="ghost" size="sm" class="text-xs text-destructive" onclick={handleClear}>
                    <XMark class="w-3.5 h-3.5 me-1" /> {translations.clearAll}
                </Button>
                <Button size="sm" class="text-xs" onclick={handleApply}>
                    {translations.applyFilter}
                </Button>
            </DialogFooter>
        </DialogContent>
    </Dialog>
</div>
