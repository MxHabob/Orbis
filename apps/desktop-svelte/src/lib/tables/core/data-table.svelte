<script lang="ts">
import * as m from "$lib/paraglide/messages";
/**
 * DataTable — Universal table shell component
 *
 * Wraps OrbisGrid with a professional toolbar, search, empty states,
 * and loading skeletons. Used system-wide for all data tables.
 */
import {
  OrbisGrid,
  type OrbisColumnDef,
  type CellEditEvent,
  type GridActionEvent,
} from "@orbis/svelte-grid";
import { Input } from "@orbis/ui-svelte/input";
import { Button } from "@orbis/ui-svelte/button";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import {
  Search,
  Download,
  RefreshCcw,
  Printer,
} from "@lucide/svelte";
import AiEmptyState from "$lib/shared/components/ai-empty-state.svelte";
import { openPrintDialog } from "$lib/stores/print.svelte";
import type { Snippet } from "svelte";
import { DefaultCell } from "../index";
import CheckboxCell from "../components/CheckboxCell.svelte";
import { FilterTrigger } from "@orbis/ui-svelte/filter-builder";
import type { UniversalFilter, FieldDefinition } from "@orbis/ui-svelte/filter-builder";

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

  /** Universal Filter configuration */
  filterable?: boolean;
  filterFields?: FieldDefinition[];
  filterValue?: UniversalFilter;
  onFilter?: (filter: UniversalFilter) => void;

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
  printable?: boolean;
  onPrint?: () => void;
  printType?: string; // Entity type for bulk print

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
  filterable = false,
  filterFields = [],
  filterValue = $bindable({ logic: "AND", conditions: [], groups: [] }),
  onFilter,
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
  printable = false,
  onPrint,
  printType,
  title,
  subtitle,
  skeletonRows = 8,
}: Props = $props();

// ── Internal State ────────────────────────────────────────────────────

let localSearch = $state("");

// ── Derived ───────────────────────────────────────────────────────────

let mappedColumns = $derived(
  columns.map((col) => {
    if (col.id !== "__selected") return { ...col, cell: col.cell || DefaultCell };
    const selectedAll = source.length > 0 && source.every((row) => row.__selected === true);
    return {
      ...col,
      cell: col.cell || CheckboxCell,
      headerCell: col.headerCell || CheckboxCell,
      meta: { ...col.meta, isSelectionHeader: true, selectedAll },
    };
  }),
);

let hasData = $derived(source.length > 0);
let showGrid = $derived(!loading && hasData);
let showEmpty = $derived(!loading && !hasData);
let showToolbar = $derived(
  searchable || exportable || refreshable || printable || filterable || !!actions,
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
        {#if filterable && filterFields.length > 0}
          <FilterTrigger 
            bind:filter={filterValue} 
            fields={filterFields} 
            translations={{
              filter: m.common_filter(),
              advancedFilter: m.common_filter_advanced(),
              matchAll: m.common_filter_match_all(),
              matchAny: m.common_filter_match_any(),
              rule: m.common_filter_rule(),
              group: m.common_filter_group(),
              removeGroup: m.common_filter_remove_group(),
              noRules: m.common_filter_no_rules(),
              addRule: m.common_filter_add_rule(),
              addSubGroup: m.common_filter_add_sub_group(),
              removeCondition: m.common_filter_remove_condition(),
              clearAll: m.common_filter_clear_all(),
              applyFilter: m.common_filter_apply()
            }}
            onApply={onFilter} 
          />
          <div class="w-px h-4 bg-border mx-1"></div>
        {/if}

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

        {#if printable}
          <Button
            variant="ghost"
            size="icon-sm"
            onclick={() => onPrint ? onPrint() : (printType ? openPrintDialog(printType, []) : null)}
            class="text-muted-foreground"
            title="Print"
          >
            <Printer size={14} />
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
  <div class="flex-1 min-h-0 rounded-sm overflow-hidden">
    {#if loading}
      <div class="p-0">
        <div class="flex items-center h-10  px-3 gap-4">
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
        columns={mappedColumns}
        {source}
        {readonly}
        {rtl}
        {height}
        {oncelledit}
        {onbeforeedit}
        {onaction}
      />
    {:else if showEmpty}
      <!-- Universal AI Avatar Empty State Centered Vertically & Horizontally -->
      <div class="h-full w-full flex flex-col items-center justify-center p-6">
        <AiEmptyState
          text={emptyDescription ? `${emptyTitle} ${emptyDescription}` : emptyTitle}
          hat="random"
        />
      </div>
    {/if}
  </div>
</div>
