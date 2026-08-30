<script lang="ts">
    import { Button } from "../components/ui/button";
    import { Filter, X } from "@lucide/svelte";
    import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogDescription, DialogFooter, DialogTrigger } from "../components/ui/dialog";
    import FilterBuilder from "./FilterBuilder.svelte";
    import FilterSummary from "./FilterSummary.svelte";
    import type { UniversalFilter, FieldDefinition } from "./types";

    let {
        filter = $bindable(),
        fields = [],
        onApply
    }: {
        filter: UniversalFilter;
        fields: FieldDefinition[];
        onApply?: (filter: UniversalFilter) => void;
    } = $props();

    let isOpen = $state(false);
    let activeCount = $derived.by(() => {
        function countConditions(f: UniversalFilter): number {
            let count = f.conditions?.filter(c => c.field)?.length || 0;
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
                    <Filter class="w-3.5 h-3.5" />
                    Filter
                    {#if activeCount > 0}
                        <span class="absolute -top-1 -end-1 bg-primary text-primary-foreground text-[10px] font-bold w-4 h-4 rounded-full flex items-center justify-center">
                            {activeCount}
                        </span>
                    {/if}
                </Button>
            {/snippet}
        </DialogTrigger>
        <DialogContent class="max-w-3xl max-h-[80vh] overflow-y-auto">
            <DialogHeader>
                <DialogTitle>Advanced Filter</DialogTitle>
                <DialogDescription>Build complex filters with multiple conditions and groups.</DialogDescription>
            </DialogHeader>
            <div class="py-4">
                <FilterBuilder bind:filter {fields} />
            </div>
            <DialogFooter class="flex items-center justify-between gap-2">
                <Button variant="ghost" size="sm" class="text-xs text-destructive" onclick={handleClear}>
                    <X class="w-3.5 h-3.5 me-1" /> Clear All
                </Button>
                <Button size="sm" class="text-xs" onclick={handleApply}>
                    Apply Filter
                </Button>
            </DialogFooter>
        </DialogContent>
    </Dialog>
</div>
