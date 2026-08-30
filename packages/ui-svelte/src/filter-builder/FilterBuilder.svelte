<script lang="ts">
    import { Button } from "../components/ui/button";
    import { Select, SelectTrigger, SelectContent, SelectItem } from "../components/ui/select";
    import { Plus, Trash2, GitBranch } from "@lucide/svelte";
    import { FilterOperator, type UniversalFilter, type FieldDefinition } from "./types";
    import FilterRow from "./FilterRow.svelte";
    import FilterBuilderSelf from "./FilterBuilder.svelte";
    import { slide } from "svelte/transition";

    let {
        filter = $bindable(),
        fields = [],
        isRoot = true,
        onRemove = () => {}
    }: {
        filter: UniversalFilter;
        fields: FieldDefinition[];
        isRoot?: boolean;
        onRemove?: () => void;
    } = $props();

    function addCondition() {
        if (!filter.conditions) filter.conditions = [];
        filter.conditions = [...filter.conditions, {
            field: fields[0]?.id || "",
            operator: FilterOperator.Eq,
            value: ""
        }];
    }

    function addGroup() {
        if (!filter.groups) filter.groups = [];
        filter.groups = [...filter.groups, {
            logic: "AND",
            conditions: [],
            groups: []
        }];
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

<div
    class="relative flex flex-col gap-4 p-4 sm:p-5 rounded-xl border shadow-sm transition-all duration-300 {isRoot ? 'border-primary/40 bg-background' : 'border-border bg-muted/30 ml-4 mt-2'}"
>
    <!-- Group Header (Logic selector + Add buttons) -->
    <div class="flex flex-wrap items-center justify-between gap-3 relative z-10">
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
                Match {filter.logic === 'AND' ? 'ALL' : 'ANY'} of the following:
            </span>
        </div>

        <div class="flex items-center gap-2">
            <Button variant="outline" size="sm" onclick={addCondition} class="h-8 gap-1 text-xs">
                <Plus class="w-3.5 h-3.5" />
                <span class="hidden sm:inline">Rule</span>
            </Button>
            
            <Button variant="outline" size="sm" onclick={addGroup} class="h-8 gap-1 text-xs">
                <GitBranch class="w-3.5 h-3.5" />
                <span class="hidden sm:inline">Group</span>
            </Button>

            {#if !isRoot}
                <Button variant="ghost" size="icon" onclick={onRemove} class="h-8 w-8 text-destructive hover:bg-destructive/10 hover:text-destructive transition-colors ml-1">
                    <Trash2 class="w-3.5 h-3.5" />
                    <span class="sr-only">Remove Group</span>
                </Button>
            {/if}
        </div>
    </div>

    <!-- Conditions List -->
    {#if (filter.conditions && filter.conditions.length > 0) || (filter.groups && filter.groups.length > 0)}
        <div class="flex flex-col gap-3 relative z-10">
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
            
            <!-- Nested Groups List (Recursive rendering) -->
            {#if filter.groups}
                {#each filter.groups as group, i (i)}
                    <div transition:slide={{duration: 200}}>
                        <FilterBuilderSelf 
                            bind:filter={filter.groups[i]} 
                            fields={fields} 
                            isRoot={false} 
                            onRemove={() => removeGroup(i)} 
                        />
                    </div>
                {/each}
            {/if}
        </div>
    {:else}
        <div class="flex flex-col items-center justify-center p-8 border-2 border-dashed rounded-lg bg-muted/20 text-muted-foreground">
            <p class="text-sm mb-2">No rules defined in this group.</p>
            <div class="flex gap-2">
                <Button variant="secondary" size="sm" onclick={addCondition} class="text-xs">Add Rule</Button>
                <Button variant="outline" size="sm" onclick={addGroup} class="text-xs">Add Sub-Group</Button>
            </div>
        </div>
    {/if}
</div>
