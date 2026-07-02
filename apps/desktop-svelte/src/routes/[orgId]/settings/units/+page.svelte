<script lang="ts">
import { onMount } from "svelte";
import { commands } from "$lib/services/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import {
  Table,
  TableHeader,
  TableRow,
  TableHead,
  TableBody,
  TableCell,
} from "@orbis/ui-svelte/table";
import { Input } from "@orbis/ui-svelte/input";
import { toast } from "svelte-sonner";
import { getLocale } from "$lib/paraglide/runtime";
import { fade } from "svelte/transition";
import {
  Plus,
  Search,
  Pencil,
  Trash2,
  Network,
  School,
  Building2,
  MapPin,
  Briefcase,
  Box,
  ChevronRight,
  ChevronDown,
  List,
  FolderTree,
} from "lucide-svelte";
import OrgUnitSheet from "$lib/components/sheets/org-unit-sheet.svelte";
import * as m from "$lib/paraglide/messages";

let { data }: { data: { orgId: string } } = $props();

// State
let units = $state<any[]>([]);
let isLoading = $state(true);
let error = $state<string | null>(null);

// Filters & View Configuration
let searchQuery = $state("");
let viewMode = $state<"tree" | "flat">("tree");

// Expanded state tracking
let expandedIds = $state(new Set<string>());
let initializedExpanded = $state(false);

// Sheet controls
let isSheetOpen = $state(false);
let editingUnit = $state<any>(null);
let defaultParentId = $state("");

async function loadUnits() {
  isLoading = true;
  error = null;
  try {
    const list = await commands.orgUnits.getAll(data.orgId);
    units = list;
  } catch (e) {
    error = e instanceof Error ? e.message : String(e);
    toast.error("Failed to load organizational units");
  } finally {
    isLoading = false;
  }
}

onMount(() => {
  if (data.orgId) {
    loadUnits();
  }
});

// Auto-expand parents on initial data load
$effect(() => {
  if (units.length > 0 && !initializedExpanded) {
    const parentIds = new Set(units.map((u) => u.parent_id).filter(Boolean));
    expandedIds = new Set(parentIds);
    initializedExpanded = true;
  }
});

// Tree construction and query propagation
const unitsMap = $derived(new Map(units.map((u) => [String(u.id), u])));

const childrenMap = $derived.by(() => {
  const map = new Map<string, any[]>();
  for (const u of units) {
    if (u.parent_id) {
      const pId = String(u.parent_id);
      if (!map.has(pId)) map.set(pId, []);
      map.get(pId)!.push(u);
    }
  }
  // Sort children alphabetically by name
  const locale = getLocale();
  for (const [pId, list] of map.entries()) {
    list.sort((a, b) => {
      const nameA = (locale === "ar" ? a.name_ar || a.name : a.name) || "";
      const nameB = (locale === "ar" ? b.name_ar || b.name : b.name) || "";
      return nameA.localeCompare(nameB, locale);
    });
  }
  return map;
});

const rootUnits = $derived.by(() => {
  const roots = units.filter(
    (u) => !u.parent_id || !unitsMap.has(String(u.parent_id)),
  );
  const locale = getLocale();
  roots.sort((a, b) => {
    const nameA = (locale === "ar" ? a.name_ar || a.name : a.name) || "";
    const nameB = (locale === "ar" ? b.name_ar || b.name : b.name) || "";
    return nameA.localeCompare(nameB, locale);
  });
  return roots;
});

const searchResults = $derived.by(() => {
  const query = searchQuery.trim().toLowerCase();
  if (!query) {
    return {
      hasQuery: false,
      matchingIds: new Set<string>(),
      forceExpandedIds: new Set<string>(),
    };
  }

  const matchingIds = new Set<string>();
  const forceExpandedIds = new Set<string>();

  for (const u of units) {
    const nameVal = (u.name || "").toLowerCase();
    const nameArVal = (u.name_ar || "").toLowerCase();
    const codeVal = (u.code || "").toLowerCase();
    const typeVal = (u.unit_type || "").toLowerCase();

    if (
      nameVal.includes(query) ||
      nameArVal.includes(query) ||
      codeVal.includes(query) ||
      typeVal.includes(query)
    ) {
      matchingIds.add(String(u.id));

      // Cascade up and flag all parents as expanded
      let currentParentId = u.parent_id;
      while (currentParentId && unitsMap.has(String(currentParentId))) {
        const pIdStr = String(currentParentId);
        forceExpandedIds.add(pIdStr);
        const parentUnit = unitsMap.get(pIdStr);
        currentParentId = parentUnit?.parent_id;
      }
    }
  }

  return {
    hasQuery: true,
    matchingIds,
    forceExpandedIds,
  };
});

interface RenderNode {
  id: string;
  depth: number;
  hasChildren: boolean;
  isExpanded: boolean;
  unit: any;
  isMatching: boolean;
}

const visibleTreeNodes = $derived.by(() => {
  const list: RenderNode[] = [];
  const { hasQuery, matchingIds, forceExpandedIds } = searchResults;

  function traverse(unit: any, depth: number) {
    const unitId = String(unit.id);
    const children = childrenMap.get(unitId) || [];
    const hasChildren = children.length > 0;

    const isVisible =
      !hasQuery || matchingIds.has(unitId) || forceExpandedIds.has(unitId);
    const isExpanded = hasQuery
      ? forceExpandedIds.has(unitId)
      : expandedIds.has(unitId);

    if (isVisible) {
      list.push({
        id: unitId,
        depth,
        hasChildren,
        isExpanded,
        unit,
        isMatching: hasQuery && matchingIds.has(unitId),
      });
    }

    const shouldTraverseChildren = hasChildren && (hasQuery || isExpanded);
    if (shouldTraverseChildren) {
      for (const child of children) {
        traverse(child, depth + 1);
      }
    }
  }

  for (const root of rootUnits) {
    traverse(root, 0);
  }

  return list;
});

const flatFilteredUnits = $derived(
  units.filter((u) => {
    if (!searchQuery.trim()) return true;
    const q = searchQuery.toLowerCase();
    return (
      (u.name || "").toLowerCase().includes(q) ||
      (u.name_ar || "").toLowerCase().includes(q) ||
      (u.code || "").toLowerCase().includes(q) ||
      (u.unit_type || "").toLowerCase().includes(q)
    );
  }),
);

function toggleExpand(id: string) {
  const newSet = new Set(expandedIds);
  if (newSet.has(id)) {
    newSet.delete(id);
  } else {
    newSet.add(id);
  }
  expandedIds = newSet;
}

function toggleExpandAll(expand: boolean) {
  if (expand) {
    const parentIds = new Set(units.map((u) => u.parent_id).filter(Boolean));
    expandedIds = new Set(parentIds);
  } else {
    expandedIds = new Set();
  }
}

function openCreateSheet() {
  editingUnit = null;
  defaultParentId = "";
  isSheetOpen = true;
}

function openCreateSubSheet(parentUnit: any) {
  editingUnit = null;
  defaultParentId = String(parentUnit.id);
  isSheetOpen = true;
}

function openEditSheet(unit: any) {
  editingUnit = unit;
  defaultParentId = "";
  isSheetOpen = true;
}

async function handleDeleteUnit(unit: any) {
  if (
    !confirm(
      `Are you sure you want to delete org unit "${unit.name_ar || unit.name}"?`,
    )
  )
    return;

  try {
    await commands.orgUnits.delete(unit.id);
    toast.success("Organizational unit deleted successfully");
    loadUnits();
  } catch (e) {
    const errorMsg = e instanceof Error ? e.message : String(e);
    toast.error(errorMsg);
  }
}

function getUnitTypeBadgeClass(typeStr: string): string {
  switch (String(typeStr).toLowerCase()) {
    case "branch":
      return "bg-sky-500/10 text-sky-500 border-sky-500/20";
    case "building":
      return "bg-teal-500/10 text-teal-500 border-teal-500/20";
    case "department":
      return "bg-amber-500/10 text-amber-500 border-amber-500/20";
    case "classroom":
      return "bg-lime-500/10 text-lime-500 border-lime-500/20";
    case "office":
      return "bg-indigo-500/10 text-indigo-500 border-indigo-500/20";
    default:
      return "bg-slate-500/10 text-slate-500 border-slate-500/20";
  }
}

function getUnitTypeIcon(typeStr: string) {
  switch (String(typeStr).toLowerCase()) {
    case "branch":
      return MapPin;
    case "building":
      return Building2;
    case "department":
      return Network;
    case "classroom":
      return School;
    case "office":
      return Briefcase;
    default:
      return Box;
  }
}

function highlightText(text: string, search: string): string {
  if (!text) return "";
  if (!search.trim()) return escapeHtml(text);

  const escapedText = escapeHtml(text);
  const escapedSearch = escapeRegExp(search.toLowerCase());

  const regex = new RegExp(`(${escapedSearch})`, "gi");
  return escapedText.replace(
    regex,
    '<mark class="bg-lime-500/20 text-lime-600 dark:text-lime-300 font-semibold px-0.5 rounded-sm">$1</mark>',
  );
}

function escapeRegExp(string: string) {
  return string.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

function escapeHtml(text: string): string {
  return text
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&#039;");
}
</script>

<svelte:head>
  <title>{m.settings_units_window_title()} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-y-auto scrollbar-hide">
  <div class="p-6 flex flex-col gap-6">
  <!-- Header Area (Search, View controls and Add Button) -->
  <div class="flex flex-col lg:flex-row lg:items-center justify-between gap-4 font-sans border-b border-border/40 pb-5 select-none">
    <div class="flex flex-1 flex-col sm:flex-row gap-3 items-center w-full">
      <!-- Search Field -->
      <div class="relative flex-1 w-full">
        <Search size={14} class="absolute top-1/2 -translate-y-1/2 text-muted-foreground rtl:right-3 ltr:left-3" />
        <Input
          type="text"
          bind:value={searchQuery}
          placeholder={m.settings_units_search_placeholder()}
          class="w-full rtl:pr-9 ltr:pl-9 h-9 text-xs bg-background/50"
        />
      </div>

      <div class="flex items-center gap-3 w-full sm:w-auto shrink-0 justify-between sm:justify-end">
        <!-- View Toggle -->
        <div class="flex items-center bg-muted/50 p-0.5 rounded-lg border border-border/40 select-none">
          <Button
            variant={viewMode === "tree" ? "secondary" : "ghost"}
            size="sm"
            class="text-[10px] h-7 px-3 font-semibold gap-1.5"
            onclick={() => viewMode = "tree"}
          >
            <FolderTree size={12} />
            <span>{m.settings_units_hierarchy()}</span>
          </Button>
          <Button
            variant={viewMode === "flat" ? "secondary" : "ghost"}
            size="sm"
            class="text-[10px] h-7 px-3 font-semibold gap-1.5"
            onclick={() => viewMode = "flat"}
          >
            <List size={12} />
            <span>{m.settings_units_flat_list()}</span>
          </Button>
        </div>

        <!-- Expand/Collapse all -->
        {#if viewMode === "tree" && !searchQuery.trim()}
          <div class="flex items-center gap-1 select-none">
            <Button
              variant="outline"
              size="sm"
              class="text-[10px] h-7 px-2"
              onclick={() => toggleExpandAll(true)}
            >
              {m.settings_units_expand_all()}
            </Button>
            <Button
              variant="outline"
              size="sm"
              class="text-[10px] h-7 px-2"
              onclick={() => toggleExpandAll(false)}
            >
              {m.settings_units_collapse_all()}
            </Button>
          </div>
        {/if}
      </div>
    </div>

    <!-- Add button -->
    <Button variant="lime" class="text-xs h-9 px-4 font-semibold shrink-0 animate-in fade-in zoom-in-95 duration-150" onclick={openCreateSheet}>
      <Plus size={14} class={getLocale() === "ar" ? 'ml-1.5' : 'mr-1.5'} />
      {m.settings_units_add()}
    </Button>
  </div>

  {#if isLoading}
    <div class="flex flex-col gap-3">
      <Skeleton class="h-10 w-full" />
      <Skeleton class="h-12 w-full" />
      <Skeleton class="h-12 w-full" />
    </div>
  {:else if error}
    <div class="flex flex-col items-center justify-center p-8 border border-destructive/20 bg-destructive/5 rounded-xl text-center gap-3">
      <p class="text-xs text-destructive font-semibold">Failed to load organizational units</p>
      <p class="text-xs text-muted-foreground">{error}</p>
      <Button variant="outline" class="text-xs" onclick={loadUnits}>{m.error_retry()}</Button>
    </div>
  {:else if units.length === 0}
    <div class="flex flex-col items-center justify-center py-16 text-muted-foreground border border-border border-dashed rounded-xl gap-4 bg-card/10">
      <Network size={48} class="text-muted-foreground/30 animate-pulse" />
      <div class="flex flex-col gap-1 text-center">
        <p class="text-xs font-semibold">No units found</p>
        <p class="text-[10px] text-muted-foreground/80">Add your first site, building, department, or classroom location.</p>
      </div>
    </div>
  {:else if (viewMode === 'tree' ? visibleTreeNodes : flatFilteredUnits).length === 0}
    <div class="flex flex-col items-center justify-center py-16 text-muted-foreground border border-border border-dashed rounded-xl gap-4 bg-card/10">
      <Search size={48} class="text-muted-foreground/30" />
      <div class="flex flex-col gap-1 text-center">
        <p class="text-xs font-semibold">No units match your search</p>
        <p class="text-[10px] text-muted-foreground/80">Try adjusting your keywords or filters.</p>
      </div>
    </div>
  {:else}
    <div class="overflow-hidden" in:fade={{ duration: 150 }}>
      <div class="overflow-x-auto">
        <Table>
          <TableHeader>
            <TableRow class="hover:bg-transparent">
              <TableHead class="text-start text-xs font-bold text-muted-foreground">Code</TableHead>
              <TableHead class="text-start text-xs font-bold text-muted-foreground">Name</TableHead>
              <TableHead class="text-start text-xs font-bold text-muted-foreground">Type</TableHead>
              <TableHead class="text-start text-xs font-bold text-muted-foreground">Capacity</TableHead>
              <TableHead class="text-start text-xs font-bold text-muted-foreground">Location</TableHead>
              <TableHead class="text-end text-xs font-bold text-muted-foreground px-6">Actions</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {#if viewMode === "tree"}
              {#each visibleTreeNodes as node (node.id)}
                {@const Icon = getUnitTypeIcon(node.unit.unit_type)}
                <TableRow class="hover:bg-accent/5">
                  <!-- Code -->
                  <TableCell class="text-xs font-mono font-bold text-foreground">
                    {#if searchQuery.trim()}
                      {@html highlightText(node.unit.code || "—", searchQuery)}
                    {:else}
                      {node.unit.code || "—"}
                    {/if}
                  </TableCell>
                  
                  <!-- Name with indentation -->
                  <TableCell class="p-2">
                    <div 
                      class="flex items-center gap-1.5 text-start"
                      style="{getLocale() === 'ar' ? 'padding-right' : 'padding-left'}: {node.depth * 20}px"
                    >
                      <!-- Expand/Collapse Button -->
                      {#if node.hasChildren}
                        <button
                          type="button"
                          class="p-1 hover:bg-muted/80 rounded text-muted-foreground hover:text-foreground transition-colors cursor-pointer"
                          onclick={() => toggleExpand(node.id)}
                        >
                          {#if node.isExpanded}
                            <ChevronDown size={13} />
                          {:else}
                            <ChevronRight size={13} class="rtl:rotate-180" />
                          {/if}
                        </button>
                      {:else}
                        <div class="w-6 h-6 flex-shrink-0"></div>
                      {/if}

                      <!-- Icon -->
                      <div class="p-1 rounded bg-muted/40 text-muted-foreground border border-border/20 flex-shrink-0 flex items-center justify-center">
                        <Icon size={12} />
                      </div>

                      <!-- Name -->
                      <div class="flex flex-col leading-tight min-w-0">
                        <span class="text-xs font-bold text-foreground truncate">
                          {#if searchQuery.trim()}
                            {@html highlightText(getLocale() === "ar" && node.unit.name_ar ? node.unit.name_ar : node.unit.name, searchQuery)}
                          {:else}
                            {getLocale() === "ar" && node.unit.name_ar ? node.unit.name_ar : node.unit.name}
                          {/if}
                        </span>
                        {#if getLocale() === "ar" && node.unit.name_ar && node.unit.name}
                          <span class="text-[9px] text-muted-foreground truncate">
                            {#if searchQuery.trim()}
                              {@html highlightText(node.unit.name, searchQuery)}
                            {:else}
                              {node.unit.name}
                            {/if}
                          </span>
                        {/if}
                      </div>
                    </div>
                  </TableCell>
                  
                  <!-- Unit Type -->
                  <TableCell>
                    <span class="px-2 py-0.5 rounded border text-[9px] font-bold tracking-wide uppercase {getUnitTypeBadgeClass(node.unit.unit_type)}">
                      {node.unit.unit_type}
                    </span>
                  </TableCell>
                  
                  <!-- Capacity -->
                  <TableCell class="text-xs text-muted-foreground font-mono">
                    {node.unit.capacity !== null && node.unit.capacity !== undefined ? node.unit.capacity : "—"}
                  </TableCell>
                  
                  <!-- Location details -->
                  <TableCell class="text-xs text-muted-foreground">
                    {#if node.unit.building || node.unit.floor !== null || node.unit.room_number}
                      <div class="flex items-center gap-1.5 flex-wrap">
                        {#if node.unit.building}
                          <span class="bg-muted px-1.5 py-0.5 rounded text-[10px] font-bold">Bld {node.unit.building}</span>
                        {/if}
                        {#if node.unit.floor !== null}
                          <span class="bg-muted px-1.5 py-0.5 rounded text-[10px]">Floor {node.unit.floor}</span>
                        {/if}
                        {#if node.unit.room_number}
                          <span class="bg-muted px-1.5 py-0.5 rounded text-[10px] font-mono">Room {node.unit.room_number}</span>
                        {/if}
                      </div>
                    {:else}
                      <span class="text-muted-foreground/60">—</span>
                    {/if}
                  </TableCell>
                  
                  <!-- Actions -->
                  <TableCell class="text-end px-6">
                    <div class="flex items-center justify-end gap-1.5">
                      <Button
                        variant="ghost"
                        size="icon-xs"
                        title="Add Sub-unit"
                        onclick={() => openCreateSubSheet(node.unit)}
                      >
                        <Plus size={12} class="text-muted-foreground hover:text-foreground" />
                      </Button>
                      <Button
                        variant="ghost"
                        size="icon-xs"
                        title="Edit"
                        onclick={() => openEditSheet(node.unit)}
                      >
                        <Pencil size={12} class="text-muted-foreground hover:text-foreground" />
                      </Button>
                      <Button
                        variant="ghost"
                        size="icon-xs"
                        title="Delete"
                        onclick={() => handleDeleteUnit(node.unit)}
                      >
                        <Trash2 size={12} class="text-muted-foreground hover:text-destructive" />
                      </Button>
                    </div>
                  </TableCell>
                </TableRow>
              {/each}
            {:else}
              {#each flatFilteredUnits as unit (unit.id)}
                {@const Icon = getUnitTypeIcon(unit.unit_type)}
                <TableRow class="hover:bg-accent/5">
                  <!-- Code -->
                  <TableCell class="text-xs font-mono font-bold text-foreground">
                    {#if searchQuery.trim()}
                      {@html highlightText(unit.code || "—", searchQuery)}
                    {:else}
                      {unit.code || "—"}
                    {/if}
                  </TableCell>
                  
                  <!-- Name with icon -->
                  <TableCell class="p-2">
                    <div class="flex items-center gap-2 text-start">
                      <div class="p-1 rounded bg-muted/40 text-muted-foreground border border-border/20 flex-shrink-0 flex items-center justify-center">
                        <Icon size={12} />
                      </div>
                      <div class="flex flex-col leading-tight min-w-0">
                        <span class="text-xs font-bold text-foreground truncate">
                          {#if searchQuery.trim()}
                            {@html highlightText(getLocale() === "ar" && unit.name_ar ? unit.name_ar : unit.name, searchQuery)}
                          {:else}
                            {getLocale() === "ar" && unit.name_ar ? unit.name_ar : unit.name}
                          {/if}
                        </span>
                        {#if getLocale() === "ar" && unit.name_ar && unit.name}
                          <span class="text-[9px] text-muted-foreground truncate">
                            {#if searchQuery.trim()}
                              {@html highlightText(unit.name, searchQuery)}
                            {:else}
                              {unit.name}
                            {/if}
                          </span>
                        {/if}
                      </div>
                    </div>
                  </TableCell>
                  
                  <!-- Unit Type -->
                  <TableCell>
                    <span class="px-2 py-0.5 rounded border text-[9px] font-bold tracking-wide uppercase {getUnitTypeBadgeClass(unit.unit_type)}">
                      {unit.unit_type}
                    </span>
                  </TableCell>
                  
                  <!-- Capacity -->
                  <TableCell class="text-xs text-muted-foreground font-mono">
                    {unit.capacity !== null && unit.capacity !== undefined ? unit.capacity : "—"}
                  </TableCell>
                  
                  <!-- Location details -->
                  <TableCell class="text-xs text-muted-foreground">
                    {#if unit.building || unit.floor !== null || unit.room_number}
                      <div class="flex items-center gap-1.5 flex-wrap">
                        {#if unit.building}
                          <span class="bg-muted px-1.5 py-0.5 rounded text-[10px] font-bold">Bld {unit.building}</span>
                        {/if}
                        {#if unit.floor !== null}
                          <span class="bg-muted px-1.5 py-0.5 rounded text-[10px]">Floor {unit.floor}</span>
                        {/if}
                        {#if unit.room_number}
                          <span class="bg-muted px-1.5 py-0.5 rounded text-[10px] font-mono">Room {unit.room_number}</span>
                        {/if}
                      </div>
                    {:else}
                      <span class="text-muted-foreground/60">—</span>
                    {/if}
                  </TableCell>
                  
                  <!-- Actions -->
                  <TableCell class="text-end px-6">
                    <div class="flex items-center justify-end gap-1.5">
                      <Button
                        variant="ghost"
                        size="icon-xs"
                        title="Add Sub-unit"
                        onclick={() => openCreateSubSheet(unit)}
                      >
                        <Plus size={12} class="text-muted-foreground hover:text-foreground" />
                      </Button>
                      <Button
                        variant="ghost"
                        size="icon-xs"
                        title="Edit"
                        onclick={() => openEditSheet(unit)}
                      >
                        <Pencil size={12} class="text-muted-foreground hover:text-foreground" />
                      </Button>
                      <Button
                        variant="ghost"
                        size="icon-xs"
                        title="Delete"
                        onclick={() => handleDeleteUnit(unit)}
                      >
                        <Trash2 size={12} class="text-muted-foreground hover:text-destructive" />
                      </Button>
                    </div>
                  </TableCell>
                </TableRow>
              {/each}
            {/if}
          </TableBody>
        </Table>
      </div>
    </div>
  {/if}
  </div>
</div>

<!-- Modal Sheets -->
<OrgUnitSheet
  bind:open={isSheetOpen}
  orgId={data.orgId}
  unit={editingUnit}
  {defaultParentId}
  onSuccess={loadUnits}
/>

