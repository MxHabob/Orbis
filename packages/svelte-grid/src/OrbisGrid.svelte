<script lang="ts">
  import { onMount, setContext } from "svelte";
  import { Grid, Willow, WillowDark, HeaderMenu } from "@svar-ui/svelte-grid";
  import type { OrbisColumnDef, CellEditEvent, GridActionEvent } from "./types";

  // ── Props ────────────────────────────────────────────────────────────────

  interface Props {
    /** Column definitions */
    columns: OrbisColumnDef[];
    /** Source data array */
    source: Record<string, any>[];
    /** Grid height (CSS value) */
    height?: string;
    /** Enable RTL layout */
    rtl?: boolean;
    /** Make entire grid read-only */
    readonly?: boolean;
    /** Unique row identifier key */
    rowIdKey?: string;

    // ── Events ──────────────────────────────────────────────────────────
    /** Fired when a cell value is edited */
    oncelledit?: (event: CellEditEvent) => void;
    /** Fired when a custom cell button is clicked */
    onaction?: (event: GridActionEvent) => void;
  }

  let {
    columns,
    source = $bindable(),
    height = "100%",
    rtl = false,
    readonly = false,
    rowIdKey = "id",
    oncelledit,
    onaction,
  }: Props = $props();

  // ── State & API ──────────────────────────────────────────────────────────

  let api: any = $state(null);
  let isDark = $state(false);
  let lastSelectedRowId = $state<string | null>(null);

  // Map incoming columns to SVAR-compatible structure
  let mappedColumns = $derived.by(() => {
    return columns.map((col) => {
      const id = col.id || col.prop || "";
      const header = col.header || col.name || "";
      const width = col.width || col.size || 120;
      
      const columnDef: Record<string, any> = {
        ...col,
        id,
        header,
        width,
        sort: col.sort !== false,
      };

      if (readonly || col.readonly) {
        columnDef.editor = null;
      }

      return columnDef;
    });
  });

  // Provide parent action handler to all custom cells via Svelte Context
  setContext("orbis-grid-onaction", (event: GridActionEvent) => {
    if (onaction) {
      onaction(event);
    }
  });

  // ── Setup & Theme Observer ───────────────────────────────────────────────

  onMount(() => {
    if (typeof document !== "undefined") {
      isDark = document.documentElement.classList.contains("dark");
      
      const observer = new MutationObserver(() => {
        isDark = document.documentElement.classList.contains("dark");
      });
      observer.observe(document.documentElement, {
        attributes: true,
        attributeFilter: ["class"],
      });
      
      return () => observer.disconnect();
    }
  });

  // Listen to inline editor updates
  function handleInit(gridApi: any) {
    api = gridApi;

    api.on("editor", (ev: any) => {
      if (!oncelledit) return;

      const editEvent: CellEditEvent = {
        row: ev.row || {},
        rowIndex: source.findIndex(r => r[rowIdKey] === ev.row?.[rowIdKey]),
        columnId: ev.column?.id || "",
        oldValue: ev.oldValue,
        newValue: ev.value,
        action: ev.action,
        event: ev
      };

      oncelledit(editEvent);
    });

    api.on("select-row", (ev: any) => {
      if (ev && ev.id !== undefined) {
        lastSelectedRowId = String(ev.id);
        
        if (onaction) {
          const rowData = source.find(r => String(r[rowIdKey]) === lastSelectedRowId);
          const rowIndex = source.findIndex(r => String(r[rowIdKey]) === lastSelectedRowId);
          if (rowData && rowIndex !== -1) {
            onaction({
              actionId: "select",
              row: rowData,
              rowIndex,
              prop: "",
              originalEvent: new MouseEvent("click")
            });
          }
        }
      }
    });
  }

  function handleDblClick(e: MouseEvent) {
    if (!api || !onaction || !lastSelectedRowId) return;

    const rowData = source.find(r => String(r[rowIdKey]) === lastSelectedRowId);
    const rowIndex = source.findIndex(r => String(r[rowIdKey]) === lastSelectedRowId);
    if (rowData && rowIndex !== -1) {
      onaction({
        actionId: "dblclick",
        row: rowData,
        rowIndex,
        prop: "",
        originalEvent: e
      });
    }
  }

  // ── Exported API Methods ─────────────────────────────────────────────────

  export function getGridElement() {
    return api;
  }

  export function setDataAt(rowIndex: number, colId: string, val: any) {
    if (source[rowIndex]) {
      source[rowIndex][colId] = val;
      source = [...source];
    }
  }

  export function refresh() {
    source = [...source];
  }

  export function clearFocus() {
    if (api) {
      api.exec("select-row", { id: null });
    }
  }

  export function scrollToRow(index: number) {
    if (api && source[index]) {
      const rowId = source[index][rowIdKey];
      if (rowId) {
        api.exec("scroll", { row: rowId });
      }
    }
  }

  export function scrollToColumnIndex(index: number) {
    if (api && mappedColumns[index]) {
      const colId = mappedColumns[index].id;
      if (colId) {
        api.exec("scroll", { column: colId });
      }
    }
  }

  export function scrollToColumnProp(prop: string) {
    if (api) {
      api.exec("scroll", { column: prop });
    }
  }

  export function getColumns() {
    return mappedColumns;
  }

  export function getSource() {
    return source;
  }
</script>

<div
  class="orbis-grid-table-wrapper w-full h-full overflow-hidden"
  style="height: {height};"
  ondblclick={handleDblClick}
>
  {#if isDark}
    <WillowDark>
      <HeaderMenu {api}>
        <Grid
          data={source}
          columns={mappedColumns}
          init={handleInit}
          rowHeight={40}
          headerHeight={40}
        />
      </HeaderMenu>
    </WillowDark>
  {:else}
    <Willow>
      <HeaderMenu {api}>
        <Grid
          data={source}
          columns={mappedColumns}
          init={handleInit}
          rowHeight={40}
          headerHeight={40}
        />
      </HeaderMenu>
    </Willow>
  {/if}
</div>

<style>
  :global(.orbis-grid-table-wrapper) {
    position: relative;
    width: 100% !important;
    height: 100% !important;
    display: block;
    --svar-grid-font-family: "Rubik", system-ui, sans-serif;
    --svar-grid-font-size: 12.5px;
    --svar-grid-header-font-weight: 600;

    /* Theme Variable Overrides */
    --wx-background: var(--background);
    --wx-table-background: var(--background);
    --wx-table-header-background: var(--muted);
    --wx-table-border: 1px solid var(--border);
    --wx-table-cell-border: 1px solid var(--border);
    --wx-table-header-border: 1px solid var(--border);
    --wx-table-header-cell-border: var(--border);
    --wx-table-footer-cell-border: var(--border);
    
    /* Text colors */
    --wx-table-color: var(--foreground);
    --wx-table-header-color: var(--foreground);
    
    /* Hover and Selection */
    --wx-table-row-hover-background: var(--accent);
    --wx-table-select-background: var(--accent);
    --wx-table-select-focus-background: var(--accent);
    --wx-table-select-color: var(--foreground);
    --wx-table-select-border: inset 3px 0 var(--primary);
    
    /* editor & dropdowns */
    --wx-table-editor-dropdown-border: 1px solid var(--border);
    --wx-table-editor-dropdown-shadow: 0px 4px 20px rgba(0, 0, 0, 0.2);
  }

  /* Force background matching and remove SVAR hardcoded defaults */
  :global(.orbis-grid-table-wrapper .wx-grid) {
    background-color: var(--background) !important;
    border-color: var(--border) !important;
  }
  :global(.orbis-grid-table-wrapper .wx-header) {
    background-color: var(--muted) !important;
    color: var(--foreground) !important;
    border-bottom: 1px solid var(--border) !important;
  }
  :global(.orbis-grid-table-wrapper .wx-h-row) {
    background-color: var(--muted) !important;
  }
  :global(.orbis-grid-table-wrapper .wx-header .wx-cell) {
    background-color: var(--muted) !important;
    color: var(--foreground) !important;
    border-bottom: 1px solid var(--border) !important;
    border-right: 1px solid var(--border) !important;
    border-left: 1px solid var(--border) !important;
  }
  :global(.orbis-grid-table-wrapper .wx-row) {
    background-color: var(--background) !important;
    color: var(--foreground) !important;
    border-bottom: 1px solid var(--border) !important;
  }
  :global(.orbis-grid-table-wrapper .wx-row:hover) {
    background-color: var(--accent) !important;
  }
  :global(.orbis-grid-table-wrapper .wx-row .wx-cell) {
    border-right: 1px solid var(--border) !important;
    border-left: 1px solid var(--border) !important;
    color: var(--foreground) !important;
  }
</style>
