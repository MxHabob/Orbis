<!--
@component IntegrationsPage
Integrations store page — Midday inspired centered layout header using shared SearchField and AiEmptyState components.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { apps, categories, getAppsByCategory } from "@orbis/app-store";
import type { OrbisApp } from "@orbis/app-store";
import AppCard from "$lib/features/ai-center/components/integrations/app-card.svelte";
import CategoryFilter from "$lib/features/ai-center/components/integrations/category-filter.svelte";
import SearchField from "$lib/shared/components/search-field.svelte";
import AiEmptyState from "$lib/shared/components/ai-empty-state.svelte";
import { getLocale } from "$lib/paraglide/runtime";

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
        (app.name_ar && app.name_ar.toLowerCase().includes(q)) ||
        app.id.toLowerCase().includes(q) ||
        app.short_description.toLowerCase().includes(q) ||
        (app.short_description_ar && app.short_description_ar.toLowerCase().includes(q))
    );
  }

  return result;
});

const basePath = $derived(`/${data.orgId}/ai-center/integrations`);
</script>

<div class="h-full w-full overflow-y-auto font-sans p-6 scrollbar-hide space-y-6">
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

  <!-- Apps Grid or AI Interactive Empty State -->
  <main>
    {#if filteredApps.length > 0}
      <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
        {#each filteredApps as app (app.id)}
          <AppCard {app} href="{basePath}/{app.id}" />
        {/each}
      </div>
    {:else}
      <AiEmptyState
        text={searchQuery
          ? `أهلاً بك! لم أستطع العثور على أي خدمة أو جهاز يطابق "${searchQuery}". حاول تغيير كلمات البحث أو اختيار تصنيف مختلف.`
          : `أهلاً بك! لا توجد تكاملات مسجلة في هذا التصنيف حالياً. يمكنك استكشاف بقية التصنيفات أو إضافة جهاز جديد.`}
        actionLabel={searchQuery ? "إعادة ضبط الفلاتر والبحث" : "عرض كافة التكاملات"}
        onaction={() => { searchQuery = ""; activeCategory = "all"; }}
        hat="random"
      />
    {/if}
  </main>
</div>
