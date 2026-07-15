<!--
@component IntegrationsPage
Main integrations page — matches midday.ai/integrations design.
Header + Category Filter Pills + 3-column App Cards Grid.
-->
<script lang="ts">
import { apps, categories, getAppsByCategory } from "@orbis/app-store";
import type { OrbisApp } from "@orbis/app-store";
import AppCard from "$lib/features/ai-center/components/integrations/app-card.svelte";
import CategoryFilter from "$lib/features/ai-center/components/integrations/category-filter.svelte";

let { data }: { data: { orgId: string } } = $props();

let activeCategory = $state("all");
let searchQuery = $state("");

const filteredApps = $derived.by(() => {
  let result: OrbisApp[] =
    activeCategory === "all" ? apps : getAppsByCategory(activeCategory as any);

  if (searchQuery.trim()) {
    const q = searchQuery.toLowerCase();
    result = result.filter(
      (app) =>
        app.name.toLowerCase().includes(q) ||
        app.id.toLowerCase().includes(q) ||
        app.short_description.toLowerCase().includes(q)
    );
  }

  return result;
});

const basePath = $derived(`/${data.orgId}/ai-center/integrations`);
</script>

<div class="h-full flex flex-col">
  <!-- Header -->
  <div class="px-8 pt-8 pb-5 shrink-0">
    <div class="max-w-2xl mx-auto text-center">
      <h1 class="text-xl font-semibold text-foreground mb-2">
        Integrations
      </h1>
      <p class="text-xs text-muted-foreground leading-relaxed max-w-lg mx-auto">
        Connect Orbis with cameras, AI providers, communication channels,
        and hardware devices to automate your school operations.
      </p>
    </div>
  </div>

  <!-- Search -->
  <div class="px-8 pb-4 shrink-0">
    <div class="max-w-sm mx-auto">
      <input
        type="search"
        bind:value={searchQuery}
        placeholder="Search integrations..."
        class="w-full text-xs border border-border/60 rounded-md px-3 py-2
               bg-background text-foreground placeholder:text-muted-foreground/50
               focus:border-primary focus:ring-1 focus:ring-primary/20
               outline-none transition-colors"
      />
    </div>
  </div>

  <!-- Category Filter -->
  <div class="px-8 pb-5 shrink-0">
    <CategoryFilter {categories} bind:active={activeCategory} />
  </div>

  <!-- Apps Grid -->
  <div class="px-8 pb-8 overflow-y-auto flex-1">
    {#if filteredApps.length > 0}
      <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
        {#each filteredApps as app (app.id)}
          <AppCard {app} href="{basePath}/{app.id}" />
        {/each}
      </div>
    {:else}
      <!-- Empty State -->
      <div class="flex flex-col items-center justify-center py-20">
        <div class="w-12 h-12 rounded-full bg-muted flex items-center justify-center mb-4">
          <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24"
               fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"
               stroke-linejoin="round" class="text-muted-foreground">
            <circle cx="11" cy="11" r="8"/><path d="m21 21-4.3-4.3"/>
          </svg>
        </div>
        <h3 class="text-sm font-medium text-foreground mb-1">No integrations found</h3>
        <p class="text-xs text-muted-foreground text-center max-w-xs">
          {#if searchQuery}
            No integrations match "{searchQuery}". Try a different search.
          {:else}
            No integrations in this category yet.
          {/if}
        </p>
      </div>
    {/if}
  </div>
</div>
