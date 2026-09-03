<!--
@component CategoryFilter
Pill-style category filter bar using official UI Button component.
-->
<script lang="ts">
import type { CategoryDef } from "@orbis/app-store";
import { Button } from "@orbis/ui/button";
import { l } from "$lib/core/i18n";

let {
  categories,
  active = $bindable("all"),
}: {
  categories: CategoryDef[];
  active: string;
} = $props();
</script>

<div class="flex flex-wrap gap-2 justify-center">
  {#each categories as cat (cat.id)}
    <Button
      type="button"
      size="sm"
      variant={active === cat.id ? "default" : "outline"}
      onclick={() => (active = cat.id)}
      class="text-xs cursor-pointer"
    >
      {l(cat.name_ar, cat.name)}
    </Button>
  {/each}
</div>
