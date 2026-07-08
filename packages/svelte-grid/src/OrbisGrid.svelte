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
    /** Dynamic cell style callback */
    cellStyle?: (row: any, col: any) => string;
  }

  let {
    columns,
    source = $bindable(),
    height = "100%",
    rtl = false,
    readonly = false,
    rowIdKey = "id",
    cellStyle,
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
        resize: col.resize !== false,
      };

      if (readonly || col.readonly) {
        columnDef.editor = null;
      }

      return columnDef;
    });
  });

  // Combine internal validation cell error styling with user-provided cellStyle
  const effectiveCellStyle = (row: any, col: any) => {
    let classes = "";
    const errors = row.__errors as Map<string, string> | undefined;
    const prop = col.id || col.prop || "";
    if (errors && errors.has(prop)) {
      classes += " orbis-cell-error";
    }
    if (cellStyle) {
      const customClass = cellStyle(row, col);
      if (customClass) {
        classes += " " + customClass;
      }
    }
    return classes.trim();
  };

  // Pinned column counts for SVAR split prop mapping
  let leftSplit = $derived(columns.filter(c => c.pin === "left").length);
  let rightSplit = $derived(columns.filter(c => c.pin === "right").length);
  let splitProp = $derived.by(() => {
    const s: Record<string, number> = {};
    if (leftSplit > 0) s.left = leftSplit;
    if (rightSplit > 0) s.right = rightSplit;
    return Object.keys(s).length > 0 ? s : undefined;
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

    api.on("update-cell", (ev: any) => {
      if (!oncelledit) return;

      const rowId = ev.id;
      const rowIndex = source.findIndex(r => String(r[rowIdKey]) === String(rowId));
      if (rowIndex === -1) return;

      const row = source[rowIndex];
      const columnId = ev.column || "";

      const editEvent: CellEditEvent & { prop: string } = {
        row,
        rowIndex,
        columnId,
        prop: columnId,
        oldValue: row[columnId] ?? "",
        newValue: ev.value,
        action: "update",
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
  class="orbis-grid-table-wrapper w-full h-full overflow-hidden {rtl ? 'grid-rtl' : ''}"
  style="height: {height};"
  ondblclick={handleDblClick}
  dir="ltr"
>
  {#key columns}
    {#if isDark}
      <WillowDark>
        <HeaderMenu {api}>
          <Grid
            data={source}
            columns={mappedColumns}
            init={handleInit}
            cellStyle={effectiveCellStyle}
            split={splitProp}
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
            cellStyle={effectiveCellStyle}
            split={splitProp}
            rowHeight={40}
            headerHeight={40}
          />
        </HeaderMenu>
      </Willow>
    {/if}
  {/key}
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

  /* Right-align text for RTL layout alignment inside the LTR scroll container */
  :global(.orbis-grid-table-wrapper.grid-rtl .wx-row .wx-cell) {
    text-align: right !important;
  }
  :global(.orbis-grid-table-wrapper.grid-rtl .wx-header .wx-cell) {
    text-align: right !important;
  }

  /* Red cell error highlight styling */
  :global(.orbis-grid-table-wrapper .wx-cell.orbis-cell-error) {
    background-color: rgba(239, 68, 68, 0.15) !important;
    border: 1px solid rgb(239, 68, 68) !important;
    color: rgb(248, 113, 113) !important;
  }
</style>
