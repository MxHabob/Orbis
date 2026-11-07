<script lang="ts">
import { Button } from "../components/ui/button";
import { Trash } from "@orbis/icons";
import {
  defaultFilterTranslations,
  type FilterCondition,
  type FieldDefinition,
  type FilterTranslations,
} from "./types";
import FilterFieldPicker from "./FilterFieldPicker.svelte";
import FilterOperatorPicker from "./FilterOperatorPicker.svelte";
import FilterValueInput from "./FilterValueInput.svelte";

let {
  condition = $bindable(),
  fields = [],
  translations = defaultFilterTranslations,
  onRemove,
}: {
  condition: FilterCondition;
  fields: FieldDefinition[];
  translations?: FilterTranslations;
  onRemove: () => void;
} = $props();
</script>

<div
  class="grid w-full min-w-0 grid-cols-1 items-center gap-2 border-b border-border/60 pb-2 last:border-b-0 sm:grid-cols-[minmax(0,2fr)_minmax(0,1.3fr)_minmax(0,2fr)_2rem]"
>
  <FilterFieldPicker bind:condition {fields} {translations} />
  <FilterOperatorPicker bind:condition {fields} {translations} />
  <FilterValueInput bind:condition {fields} {translations} />
  <Button
    variant="ghost"
    size="icon-sm"
    onclick={onRemove}
    class="text-destructive hover:bg-destructive/10 hover:text-destructive"
  >
        <Trash class="size-4" />
        <span class="sr-only">{translations.removeCondition}</span>
  </Button>
</div>
