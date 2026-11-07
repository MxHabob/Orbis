<script lang="ts">
import FilterBuilder from "./FilterBuilder.svelte";
import FilterDialogFrame from "./FilterDialogFrame.svelte";
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

<FilterDialogFrame
  bind:open={isOpen}
  title={translations.advancedFilter}
  triggerLabel={translations.filter}
  count={activeCount}
  clearLabel={translations.clearAll}
  applyLabel={translations.applyFilter}
  onClear={handleClear}
  onApply={handleApply}
>
  <FilterBuilder bind:filter {fields} translations={translations} />
</FilterDialogFrame>
