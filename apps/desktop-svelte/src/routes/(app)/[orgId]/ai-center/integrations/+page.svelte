<!--
@component IntegrationsPage
Integrations store page — Midday inspired centered layout header using shared SearchField component.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { apps, categories, getAppsByCategory } from "@orbis/app-store";
import type { OrbisApp } from "@orbis/app-store";
import AppCard from "$lib/features/ai-center/components/integrations/app-card.svelte";
import CategoryFilter from "$lib/features/ai-center/components/integrations/category-filter.svelte";
import { PackageX } from "lucide-svelte";
import SearchField from "$lib/shared/components/search-field.svelte";

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

<div class="h-full overflow-y-auto font-sans p-6 scrollbar-hide space-y-6">
  <!-- Centered Header (Midday Style) -->
  <header class="pt-4 pb-2 text-center space-y-4">
    <div class="max-w-2xl mx-auto space-y-2">
      <h1 class="text-2xl sm:text-3xl font-bold tracking-tight text-foreground">
        {m.ai_center_integrations_title ? m.ai_center_integrations_title() : "Integrations"}
      </h1>
      <p class="text-xs sm:text-sm text-muted-foreground max-w-lg mx-auto leading-relaxed">
        {m.ai_center_integrations_subtitle ? m.ai_center_integrations_subtitle() : "Connect Orbis with the tools you already use. From messaging and cameras to accounting software, our integrations help you streamline your workflow."}
      </p>
    </div>

    <!-- Centered Search Bar using Shared SearchField Component -->
    <div class="max-w-xl mx-auto pt-2">
      <SearchField
        bind:value={searchQuery}
        placeholder={m.ai_center_integrations_search_placeholder ? m.ai_center_integrations_search_placeholder() : "Search devices, services, or AI providers..."}
        class="max-w-xl mx-auto"
      />
    </div>
  </header>

  <!-- Category Filter Pills Bar -->
  <div class="flex justify-center py-1">
    <CategoryFilter {categories} bind:active={activeCategory} />
  </div>

  <!-- Apps Grid (Max 3 Columns) -->
  <main>
    {#if filteredApps.length > 0}
      <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
        {#each filteredApps as app (app.id)}
          <AppCard {app} href="{basePath}/{app.id}" />
        {/each}
      </div>
    {:else}
      <div class="flex flex-col items-center justify-center py-20 text-center">
        <div class="w-12 h-12 rounded-full bg-muted/60 flex items-center justify-center mb-3">
          <PackageX size={20} class="text-muted-foreground" />
        </div>
        <h3 class="text-sm font-bold text-foreground mb-1">
          {m.ai_center_integrations_empty_title ? m.ai_center_integrations_empty_title() : "No integrations found"}
        </h3>
        <p class="text-xs text-muted-foreground max-w-xs leading-relaxed">
          {#if searchQuery}
            {m.ai_center_integrations_empty_search ? m.ai_center_integrations_empty_search() : "No results matching your query. Try searching with different terms."}
          {:else}
            {m.ai_center_integrations_empty_category ? m.ai_center_integrations_empty_category() : "No integrations available in this category currently."}
          {/if}
        </p>
      </div>
    {/if}
  </main>
</div>
