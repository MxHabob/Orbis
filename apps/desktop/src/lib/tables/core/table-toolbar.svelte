<!--
  @component TableToolbar
  Reusable toolbar for data tables — search, filters, and action buttons.
  Can be used standalone or is embedded inside DataTable.
-->
<script lang="ts">
import { Input } from "@orbis/ui/input";
import { Button } from "@orbis/ui/button";
import { AdjustmentsHorizontal, Download, Refresh, Search, XMark } from "@orbis/icons";
import type { Snippet } from "svelte";

interface Props {
  /** Enable search input */
  searchable?: boolean;
  searchPlaceholder?: string;
  searchValue?: string;
  onSearch?: (query: string) => void;

  /** Enable export button */
  exportable?: boolean;
  onExport?: () => void;

  /** Enable refresh button */
  refreshable?: boolean;
  onRefresh?: () => void;

  /** Enable print button */
  printable?: boolean;
  onPrint?: () => void;

  /** Row count to display */
  rowCount?: number;

  /** Custom actions slot (Svelte 5 snippet) */
  actions?: Snippet;

  /** Custom filters slot */
  filters?: Snippet;

  /** RTL direction */
  rtl?: boolean;
}

let {
  searchable = false,
  searchPlaceholder = "Search...",
  searchValue = $bindable(""),
  onSearch,
  exportable = false,
  onExport,
  refreshable = false,
  onRefresh,
  printable = false,
  onPrint,
  rowCount,
  actions,
  filters,
  rtl = false,
}: Props = $props();

function handleInput(e: Event) {
  const value = (e.target as HTMLInputElement).value;
  searchValue = value;
  onSearch?.(value);
}

function clearSearch() {
  searchValue = "";
  onSearch?.("");
}

function handleKeydown(e: KeyboardEvent) {
  if (e.key === "Escape") clearSearch();
}
</script>

<div
  class="flex items-center gap-2 flex-nowrap shrink-0"
>
  {#if searchable}
    <div class="relative flex-1 max-w-xs">
      <Search
        size={14}
        class="absolute start-2.5 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none"
      />
      <Input
        type="text"
        placeholder={searchPlaceholder}
        value={searchValue}
        oninput={handleInput}
        onkeydown={handleKeydown}
        class="h-8 ps-8 pe-8 text-xs bg-card border-border/60"
      />
      {#if searchValue}
        <button
          class="absolute end-2 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground transition-colors"
          onclick={clearSearch}
        >
          <XMark size={12} />
        </button>
      {/if}
    </div>
  {/if}

  {#if filters}
    {@render filters()}
  {/if}

  <div class="flex-1"></div>

  {#if rowCount !== undefined}
    <span class="text-[10px] text-muted-foreground tabular-nums shrink-0">
      {rowCount} rows
    </span>
  {/if}

  <div class="flex items-center gap-1 flex-nowrap shrink-0">
    {#if actions}
      {@render actions()}
    {/if}

    {#if refreshable}
      <Button
        variant="ghost"
        size="icon-sm"
        onclick={() => onRefresh?.()}
        class="text-muted-foreground hover:text-foreground"
        title="Refresh"
      >
        <Refresh size={14} />
      </Button>
    {/if}

    {#if printable}
      <Button
        variant="ghost"
        size="icon-sm"
        onclick={() => onPrint?.()}
        class="text-muted-foreground hover:text-foreground"
        title="Print"
      >
        <AdjustmentsHorizontal size={14} />
      </Button>
    {/if}

    {#if exportable}
      <Button
        variant="ghost"
        size="icon-sm"
        onclick={() => onExport?.()}
        class="text-muted-foreground hover:text-foreground"
        title="Export"
      >
        <Download size={14} />
      </Button>
    {/if}
  </div>
</div>
