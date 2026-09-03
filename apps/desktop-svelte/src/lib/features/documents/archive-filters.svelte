<script lang="ts">
  import { Button } from "@orbis/ui-svelte/button";
  import { Search, SlidersHorizontal, X } from "@lucide/svelte";

  type Category = readonly [string, string];

  let {
    searchQuery = $bindable(""),
    selectedCategory = $bindable(""),
    selectedAssetType = $bindable(""),
    selectedSchoolYear = $bindable(""),
    selectedTerm = $bindable(""),
    categories,
  }: {
    searchQuery?: string;
    selectedCategory?: string;
    selectedAssetType?: string;
    selectedSchoolYear?: string;
    selectedTerm?: string;
    categories: readonly Category[];
  } = $props();

  const hasFilters = $derived(Boolean(selectedCategory || selectedAssetType || selectedSchoolYear || selectedTerm));

  function clearFilters() {
    selectedCategory = "";
    selectedAssetType = "";
    selectedSchoolYear = "";
    selectedTerm = "";
  }
</script>

<section class="flex flex-col gap-3 rounded-xl border border-border/50 bg-card/60 p-3 sm:flex-row sm:items-center sm:justify-between">
  <div class="relative min-w-0 flex-1 sm:max-w-sm">
    <Search size={14} class="absolute inset-s-2.5 top-1/2 -translate-y-1/2 text-muted-foreground" />
    <input bind:value={searchQuery} placeholder="البحث في الأرشيف" aria-label="البحث في الأرشيف" class="h-9 w-full rounded-md border border-border bg-background pe-3 ps-8 text-xs text-foreground outline-none transition-colors placeholder:text-muted-foreground focus:border-primary" />
  </div>

  <div class="flex flex-wrap items-center gap-2">
    <div class="flex items-center gap-1 text-muted-foreground" aria-hidden="true">
      <SlidersHorizontal size={14} />
      <span class="text-[11px]">تصفية</span>
    </div>
    <select bind:value={selectedCategory} class="filter-select">
      <option value="">كل التصنيفات</option>
      {#each categories as [value, label]}
        <option value={value}>{label}</option>
      {/each}
    </select>
    <select bind:value={selectedAssetType} class="filter-select">
      <option value="">كل الأنواع</option>
      <option value="image">صور</option>
      <option value="video">فيديو</option>
      <option value="document">وثائق</option>
    </select>
    <select bind:value={selectedTerm} class="filter-select">
      <option value="">كل الفصول</option>
      <option value="first">الفصل الأول</option>
      <option value="second">الفصل الثاني</option>
      <option value="summer">الصيفي</option>
    </select>
    <input bind:value={selectedSchoolYear} placeholder="السنة الدراسية" aria-label="السنة الدراسية" class="filter-select w-32" />
    {#if hasFilters}
      <Button variant="ghost" size="sm" class="h-8 gap-1 text-xs" onclick={clearFilters}>
        <X size={13} />
        مسح
      </Button>
    {/if}
  </div>
</section>

<style>
  .filter-select {
    height: 2rem;
    max-width: 10rem;
    border: 1px solid hsl(var(--border));
    border-radius: 0.375rem;
    background: hsl(var(--background));
    padding: 0 0.5rem;
    color: hsl(var(--foreground));
    font-size: 0.75rem;
    outline: none;
  }

  .filter-select:focus {
    border-color: hsl(var(--primary));
  }
</style>
