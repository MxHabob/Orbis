<script lang="ts">
    import { Button } from "../components/ui/button";
    import { Trash2 } from "@lucide/svelte";
    import type { FilterCondition, FieldDefinition } from "./types";
    import FilterFieldPicker from "./FilterFieldPicker.svelte";
    import FilterOperatorPicker from "./FilterOperatorPicker.svelte";
    import FilterValueInput from "./FilterValueInput.svelte";

    let {
        condition = $bindable(),
        fields = [],
        onRemove
    }: {
        condition: FilterCondition;
        fields: FieldDefinition[];
        onRemove: () => void;
    } = $props();
</script>

<div class="flex flex-col sm:flex-row items-start sm:items-center gap-3 p-3 bg-card border rounded-lg shadow-sm w-full transition-all hover:border-primary/50">
    <div class="grid grid-cols-1 sm:grid-cols-[2fr_1fr_2fr] gap-3 flex-1 w-full">
        <!-- Field Selection -->
        <FilterFieldPicker bind:condition {fields} />

        <!-- Operator Selection -->
        <FilterOperatorPicker bind:condition {fields} />

        <!-- Value Input -->
        <FilterValueInput bind:condition {fields} />
    </div>

    <!-- Delete Button -->
    <Button variant="ghost" size="icon" onclick={onRemove} class="text-destructive hover:bg-destructive/10 hover:text-destructive shrink-0 mt-2 sm:mt-0">
        <Trash2 class="w-4 h-4" />
        <span class="sr-only">Remove Condition</span>
    </Button>
</div>
