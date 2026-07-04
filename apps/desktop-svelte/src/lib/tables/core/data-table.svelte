<script lang="ts">
/**
 * DataTable — Universal table shell component
 *
 * Wraps OrbisGrid with a professional toolbar, search, empty states,
 * and loading skeletons. Used system-wide for all data tables.
 *
 * @example
 * ```svelte
 * <DataTable
 *   columns={studentColumns}
 *   source={students}
 *   loading={isLoading}
 *   searchable
 *   searchPlaceholder={m.search_students()}
 *   onSearch={handleSearch}
 *   emptyTitle={m.no_students()}
 *   rtl={getLocale() === "ar"}
 * />
 * ```
 */
import {
  OrbisGrid,
  type OrbisColumnDef,
  type CellEditEvent,
  type GridActionEvent,
} from "@orbis/data-grid";
import { Input } from "@orbis/ui-svelte/input";
import { Button } from "@orbis/ui-svelte/button";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { Search, Download, RefreshCcw, TableProperties } from "lucide-svelte";
import type { Snippet, Component } from "svelte";

// ── Props ──────────────────────────────────────────────────────────────

interface Props {
  /** Column definitions for the grid */
  columns: OrbisColumnDef[];
  /** Data source array */
  source: Record<string, unknown>[];

  /** Loading state — shows skeleton */
  loading?: boolean;

  /** Enable search bar */
  searchable?: boolean;
  /** Search input placeholder */
  searchPlaceholder?: string;
  /** Search callback — filters are handled by the parent */
  onSearch?: (query: string) => void;
  /** Current search value (for controlled mode) */
  searchValue?: string;

  /** Empty state configuration */
  emptyIcon?: any;
  emptyTitle?: string;
  emptyDescription?: string;

  /** Grid options */
  readonly?: boolean;
  rtl?: boolean;
  height?: string;
  rowSize?: number;
  theme?: "compact" | "default";
  range?: boolean;
  clipboard?: boolean;
  autoSize?: boolean;

  /** Cell edit callback */
  oncelledit?: (event: CellEditEvent) => void;
  /** Before edit callback (return false to prevent) */
  onbeforeedit?: (event: CellEditEvent) => boolean | void;
  /** Action click callback */
  onaction?: (event: GridActionEvent) => void;

  /** Toolbar customization */
  actions?: Snippet;
  exportable?: boolean;
  onExport?: () => void;
  refreshable?: boolean;
  onRefresh?: () => void;

  /** Title displayed above the grid */
  title?: string;
  /** Subtitle / description */
  subtitle?: string;

  /** Number of rows for skeleton loading */
  skeletonRows?: number;
}

let {
  columns,
  source,
  loading = false,
  searchable = false,
  searchPlaceholder = "Search...",
  onSearch,
  searchValue = $bindable(""),
  emptyIcon,
  emptyTitle = "No data",
  emptyDescription = "",
  readonly = true,
  rtl = false,
  height = "100%",
  rowSize = 42,
  theme = "compact",
  range = false,
  clipboard = false,
  autoSize = false,
  oncelledit,
  onbeforeedit,
  onaction,
  actions,
  exportable = false,
  onExport,
  refreshable = false,
  onRefresh,
  title,
  subtitle,
  skeletonRows = 8,
}: Props = $props();

// ── Internal State ────────────────────────────────────────────────────

let localSearch = $state("");

// ── Derived ───────────────────────────────────────────────────────────

let hasData = $derived(source.length > 0);
let showGrid = $derived(!loading && hasData);
let showEmpty = $derived(!loading && !hasData);
let showToolbar = $derived(
  searchable || exportable || refreshable || !!actions,
);
let effectiveSearch = $derived(searchValue || localSearch);

// ── Handlers ──────────────────────────────────────────────────────────

function handleSearchInput(e: Event) {
  const value = (e.target as HTMLInputElement).value;
  localSearch = value;
  searchValue = value;
  onSearch?.(value);
}

function handleSearchKeydown(e: KeyboardEvent) {
  if (e.key === "Escape") {
    localSearch = "";
    searchValue = "";
    onSearch?.("");
  }
}
</script>

<div
  class="orbis-data-table flex flex-col h-full w-full overflow-hidden"
  dir={rtl ? "rtl" : "ltr"}
>
  <!-- ── Header ───────────────────────────────────────────────────────── -->
  {#if title || subtitle}
    <div class="flex flex-col gap-0.5 px-1 pb-3">
      {#if title}
        <h2 class="text-sm font-semibold text-foreground">{title}</h2>
      {/if}
      {#if subtitle}
        <p class="text-[11px] text-muted-foreground">{subtitle}</p>
      {/if}
    </div>
  {/if}

  <!-- ── Toolbar ──────────────────────────────────────────────────────── -->
  {#if showToolbar}
    <div class="flex items-center gap-2 pb-3 flex-nowrap shrink-0">
      {#if searchable}
        <div class="relative flex-1 max-w-xs">
          <Search
            size={14}
            class="absolute inset-s-2.5 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none"
          />
          <Input
            type="text"
            placeholder={searchPlaceholder}
            value={effectiveSearch}
            oninput={handleSearchInput}
            onkeydown={handleSearchKeydown}
            class="h-8 ps-8 text-xs bg-card border-border/60"
          />
        </div>
      {/if}

      <div class="flex-1"></div>

      <div class="flex items-center gap-1.5 flex-nowrap shrink-0">
        {#if actions}
          {@render actions()}
        {/if}

        {#if refreshable}
          <Button
            variant="ghost"
            size="icon-sm"
            onclick={() => onRefresh?.()}
            class="text-muted-foreground"
          >
            <RefreshCcw size={14} />
          </Button>
        {/if}

        {#if exportable}
          <Button
            variant="ghost"
            size="icon-sm"
            onclick={() => onExport?.()}
            class="text-muted-foreground"
          >
            <Download size={14} />
          </Button>
        {/if}
      </div>
    </div>
  {/if}

  <!-- ── Content Area ─────────────────────────────────────────────────── -->
  <div class="flex-1 min-h-0 rounded-xl border border-border overflow-hidden bg-card">
    {#if loading}
      <!-- Skeleton Loading -->
      <div class="p-0">
        <!-- Skeleton header -->
        <div class="flex items-center h-10 border-b border-border bg-muted/30 px-3 gap-4">
          {#each columns.slice(0, 6) as col}
            <Skeleton class="h-3 rounded" style="width: {Math.min(col.size ?? 100, 150)}px" />
          {/each}
        </div>
        <!-- Skeleton rows -->
        {#each Array(skeletonRows) as _, i}
          <div
            class="flex items-center h-[{rowSize}px] border-b border-border/40 px-3 gap-4"
            style="opacity: {1 - i * 0.08}"
          >
            {#each columns.slice(0, 6) as col, j}
              <Skeleton
                class="h-3 rounded"
                style="width: {Math.min((col.size ?? 100) * (j === 0 ? 0.4 : 0.7), 120)}px"
              />
            {/each}
          </div>
        {/each}
      </div>
    {:else if showGrid}
      <!-- OrbisGrid -->
      <OrbisGrid
        {columns}
        {source}
        {readonly}
        {rtl}
        {height}
        {rowSize}
        {theme}
        {range}
        {clipboard}
        {autoSize}
        {oncelledit}
        {onbeforeedit}
        {onaction}
      />
    {:else if showEmpty}
      <!-- Empty State -->
      <div class="flex flex-col items-center justify-center py-16 gap-3 text-muted-foreground">
        {#if emptyIcon}
          {@const Icon = emptyIcon}
          <Icon size={36} class="opacity-30" />
        {:else}
          <TableProperties size={36} class="opacity-30" />
        {/if}
        <p class="text-sm font-medium text-foreground/70">{emptyTitle}</p>
        {#if emptyDescription}
          <p class="text-xs text-muted-foreground/70 max-w-xs text-center">{emptyDescription}</p>
        {/if}
      </div>
    {/if}
  </div>

  <!-- ── Footer / Status Bar ──────────────────────────────────────────── -->
  {#if !loading && hasData}
    <div class="flex items-center justify-between px-2 pt-2 text-[10px] text-muted-foreground">
      <span>{source.length} rows</span>
      {#if effectiveSearch}
        <span>Filtered by: "{effectiveSearch}"</span>
      {/if}
    </div>
  {/if}
</div>
