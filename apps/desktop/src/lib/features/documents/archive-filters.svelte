<script lang="ts">
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
} from "@orbis/ui/select";
import { Search, SlidersHorizontal, X } from "@lucide/svelte";

type Category = readonly [string, string];

let {
  searchQuery = $bindable(""),
  selectedCategory = $bindable(""),
  selectedAssetType = $bindable(""),
  selectedSchoolYear = $bindable(""),
  selectedTerm = $bindable(""),
  categories,
  terms = [],
  schoolYears = [],
}: {
  searchQuery?: string;
  selectedCategory?: string;
  selectedAssetType?: string;
  selectedSchoolYear?: string;
  selectedTerm?: string;
  categories: readonly Category[];
  terms?: readonly Category[];
  schoolYears?: readonly Category[];
} = $props();

const hasFilters = $derived(
  Boolean(
    selectedCategory || selectedAssetType || selectedSchoolYear || selectedTerm,
  ),
);

function clearFilters() {
  selectedCategory = "";
  selectedAssetType = "";
  selectedSchoolYear = "";
  selectedTerm = "";
}
</script>

<section class="flex min-w-0 flex-1 flex-col gap-2 sm:flex-row sm:items-center">
  <div class="relative min-w-0 flex-1 sm:max-w-52">
    <Search size={14} class="absolute inset-s-2.5 top-1/2 -translate-y-1/2 text-muted-foreground" />
    <Input bind:value={searchQuery} placeholder="البحث في الأرشيف" aria-label="البحث في الأرشيف" class="h-9 w-full pe-3 ps-8 text-xs" />
  </div>

  <div class="flex flex-wrap items-center gap-1.5">
    <div class="flex items-center gap-1 px-1 text-muted-foreground" aria-hidden="true">
      <SlidersHorizontal size={14} />
      <span class="text-[11px]">تصفية</span>
    </div>
    <Select type="single" value={selectedCategory || "all"} onValueChange={(value) => selectedCategory = value === "all" ? "" : value ?? ""}>
      <SelectTrigger class="h-8 w-30 max-w-40 text-xs"><span class="truncate">{selectedCategory ? categories.find(([value]) => value === selectedCategory)?.[1] : "كل التصنيفات"}</span></SelectTrigger>
      <SelectContent align="end">
        <SelectItem value="all">كل التصنيفات</SelectItem>
        {#each categories as [value, label]}
          <SelectItem value={value}>{label}</SelectItem>
        {/each}
      </SelectContent>
    </Select>
    <Select type="single" value={selectedAssetType || "all"} onValueChange={(value) => selectedAssetType = value === "all" ? "" : value ?? ""}>
      <SelectTrigger class="h-8 w-30 max-w-40 text-xs"><span class="truncate">{selectedAssetType || "كل الأنواع"}</span></SelectTrigger>
      <SelectContent align="end">
        <SelectItem value="all">كل الأنواع</SelectItem>
        <SelectItem value="image">صور</SelectItem>
        <SelectItem value="video">فيديو</SelectItem>
        <SelectItem value="document">وثائق</SelectItem>
      </SelectContent>
    </Select>
    <Select type="single" value={selectedTerm || "all"} onValueChange={(value) => selectedTerm = value === "all" ? "" : value ?? ""}>
      <SelectTrigger class="h-8 w-30 max-w-40 text-xs"><span class="truncate">{terms.find(([value]) => value === selectedTerm)?.[1] || "كل الفصول"}</span></SelectTrigger>
      <SelectContent align="end">
        <SelectItem value="all">كل الفصول</SelectItem>
        {#each terms as [value, label]}
          <SelectItem value={value}>{label}</SelectItem>
        {/each}
      </SelectContent>
    </Select>
    <Select type="single" value={selectedSchoolYear || "all"} onValueChange={(value) => selectedSchoolYear = value === "all" ? "" : value ?? ""}>
      <SelectTrigger class="h-8 w-30 max-w-40 text-xs"><span class="truncate">{schoolYears.find(([value]) => value === selectedSchoolYear)?.[1] || "كل السنوات"}</span></SelectTrigger>
      <SelectContent align="end">
        <SelectItem value="all">كل السنوات</SelectItem>
        {#each schoolYears as [value, label]}
          <SelectItem value={value}>{label}</SelectItem>
        {/each}
      </SelectContent>
    </Select>
    {#if hasFilters}
      <Button variant="ghost" size="sm" class="h-8 gap-1 text-xs" onclick={clearFilters}>
        <X size={13} />
        مسح
      </Button>
    {/if}
  </div>
</section>

