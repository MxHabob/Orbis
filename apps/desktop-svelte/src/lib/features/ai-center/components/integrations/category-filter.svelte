<!--
@component CategoryFilter
Pill-style category filter bar — matches midday.ai/integrations design.
Horizontal row of pill buttons: active one is filled, others are outline.
-->
<script lang="ts">
import type { CategoryDef } from "@orbis/app-store";

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
    <button
      type="button"
      onclick={() => (active = cat.id)}
      class="px-3.5 py-1.5 text-xs font-medium border rounded-md transition-all duration-150 cursor-pointer select-none
        {active === cat.id
          ? 'bg-foreground text-background border-foreground shadow-sm'
          : 'bg-background text-muted-foreground border-border/60 hover:border-foreground/40 hover:text-foreground'}"
    >
      {cat.name}
    </button>
  {/each}
</div>
