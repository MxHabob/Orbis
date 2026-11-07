<script lang="ts">
import { Button } from "../components/ui/button";
import { Trash } from "@orbis/icons";
import type { FilterCondition, FieldDefinition } from "./types";
import FilterFieldPicker from "./FilterFieldPicker.svelte";
import FilterOperatorPicker from "./FilterOperatorPicker.svelte";
import FilterValueInput from "./FilterValueInput.svelte";

let {
  condition = $bindable(),
  fields = [],
  onRemove,
}: {
  condition: FilterCondition;
  fields: FieldDefinition[];
  onRemove: () => void;
} = $props();
</script>

<div
  class="grid w-full min-w-0 grid-cols-1 items-center gap-2 border-b border-border/60 pb-2 last:border-b-0 sm:grid-cols-[minmax(0,2fr)_minmax(0,1.3fr)_minmax(0,2fr)_2rem]"
>
  <FilterFieldPicker bind:condition {fields} />
  <FilterOperatorPicker bind:condition {fields} />
  <FilterValueInput bind:condition {fields} />
  <Button
    variant="ghost"
    size="icon-sm"
    onclick={onRemove}
    class="text-destructive hover:bg-destructive/10 hover:text-destructive"
  >
        <Trash class="size-4" />
        <span class="sr-only">Remove Condition</span>
  </Button>
</div>
