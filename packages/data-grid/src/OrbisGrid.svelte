<!--
@component OrbisGrid
Enterprise data grid component wrapping RevoGrid with S.C.R.A.P.S. theming.
Designed for spreadsheet-like data entry (grades, attendance, etc.)

Features:
- Virtual scrolling (handles 10,000+ rows at 60fps)
- Frozen/pinned columns (student name, totals)
- Full RTL support (Arabic/Hebrew)
- Excel-like keyboard navigation (Arrow, Tab, Enter)
- Copy/Paste clipboard support
- Custom cell editors (number, text)
- Cell validation with visual feedback
- S.C.R.A.P.S. design token integration (light/dark)

Usage:
```svelte
<OrbisGrid
  columns={gradeColumns}
  source={students}
  height="calc(100vh - 280px)"
  rtl={true}
  readonly={false}
  oncelledit={handleEdit}
/>
```
-->
<script lang="ts">
  import { onMount } from "svelte";
  import type { OrbisColumnDef, CellEditEvent } from "./types";
  import "./orbis-grid.css";

  const isBrowser = typeof window !== "undefined";

  // ── Props ────────────────────────────────────────────────────────────────

  interface Props {
    /** Column definitions */
    columns: OrbisColumnDef[];
    /** Source data array */
    source: Record<string, unknown>[];
    /** Grid height (CSS value) */
    height?: string;
    /** Enable RTL layout */
    rtl?: boolean;
    /** Make entire grid read-only */
    readonly?: boolean;
    /** RevoGrid theme */
    theme?: "compact" | "material" | "default";
    /** Enable range selection */
    range?: boolean;
    /** Enable clipboard operations */
    clipboard?: boolean;
    /** Enable column auto-size */
    autoSize?: boolean;
    /** Number of extra rows beyond viewport */
    overscan?: number;
    /** Row height in pixels */
    rowSize?: number;

    // ── Events ──────────────────────────────────────────────────────────
    /** Fired when a cell value is edited */
    oncelledit?: (event: CellEditEvent) => void;
    /** Fired before cell edit — return false to cancel */
    onbeforeedit?: (event: CellEditEvent) => boolean | void;
    /** Fired when user pastes from clipboard */
    onpaste?: (data: string[][], startRow: number, startCol: number) => void;
  }

  let {
    columns,
    source,
    height = "500px",
    rtl = false,
    readonly = false,
    theme = "compact",
    range = true,
    clipboard = true,
    autoSize = false,
    overscan = 15,
    rowSize = 42,
    oncelledit,
    onbeforeedit,
    onpaste,
  }: Props = $props();

  // ── Internal State ───────────────────────────────────────────────────────

  let gridReady = $state(false);
  let RevoGridComponent: any = $state(null);
  let EditorWrapper: any = $state(null);
  let TemplateWrapper: any = $state(null);
  let loadedEditors: Record<string, unknown> = $state({});

  // ── Dynamic Import (SSR-safe) ────────────────────────────────────────────

  onMount(async () => {
    if (!isBrowser) return;
    try {
      const mod = await import("@revolist/svelte-datagrid");
      RevoGridComponent = mod.RevoGrid;
      EditorWrapper = mod.Editor;
      TemplateWrapper = mod.Template;

      // Register editors after module is loaded
      const hasNumberEditor = columns.some((c) => c.editor === "number");
      if (hasNumberEditor) {
        const editorMod = await import("./editors/NumberEditor.svelte");
        loadedEditors["number"] = mod.Editor(editorMod.default);
      }

      gridReady = true;
    } catch (err) {
      console.error("[OrbisGrid] Failed to load RevoGrid:", err);
    }
  });

  // ── Map OrbisColumnDef → RevoGrid ColumnRegular ──────────────────────────

  let revoColumns = $derived.by(() => {
    if (!EditorWrapper) return [];
    return columns.map((col) => {
      const revoCol: Record<string, unknown> = {
        prop: col.prop,
        name: col.name,
        size: col.size ?? 120,
        sortable: col.sortable ?? false,
        resizable: col.resizable !== false,
        readonly: readonly || (col.readonly ?? false),
        cellProperties: (params?: any) => {
          const classes: string[] = [];
          if (col.cellClass) classes.push(col.cellClass);
          if (col.align === "center") classes.push("cell-number-input");

          let customProps: Record<string, any> = {};
          if (col.cellProperties) {
            const row = params?.model || {};
            customProps = col.cellProperties(row) || {};
          }

          if (customProps.class) {
            classes.push(customProps.class);
          }

          return {
            ...customProps,
            class: classes.join(" ").trim()
          };
        },
      };

      // Pinned columns
      if (col.pin) {
        revoCol.pin = col.pin;
      }

      // Min/Max size
      if (col.minSize) revoCol.minSize = col.minSize;
      if (col.maxSize) revoCol.maxSize = col.maxSize;

      // Column meta for editors
      if (col.meta || col.editorConfig) {
        revoCol.meta = {
          ...(col.meta || {}),
          ...(col.editorConfig || {}),
        };
      }

      // Editor registration
      if (col.editor && !readonly && !col.readonly) {
        revoCol.editor = col.editor;
      }

      return revoCol;
    });
  });

  // ── Editors Registry ─────────────────────────────────────────────────────

  // Editors are loaded in onMount and stored in loadedEditors
  let editors = $derived(loadedEditors);

  // ── Event Handlers ───────────────────────────────────────────────────────

  function handleAfterEdit(event: CustomEvent) {
    const detail = event.detail;
    if (!detail || !oncelledit) return;

    // RevoGrid provides different event shapes depending on version
    const editEvent: CellEditEvent = {
      row: detail.model || detail.row || {},
      rowIndex: detail.rowIndex ?? detail.index ?? 0,
      prop: detail.prop || "",
      oldValue: detail.oldValue ?? detail.val,
      newValue: detail.value ?? detail.val,
    };

    oncelledit(editEvent);
  }

  function handleBeforeEdit(event: CustomEvent) {
    if (!onbeforeedit) return;

    const detail = event.detail;
    const editEvent: CellEditEvent = {
      row: detail.model || detail.row || {},
      rowIndex: detail.rowIndex ?? detail.index ?? 0,
      prop: detail.prop || "",
      oldValue: detail.val,
      newValue: detail.val,
    };

    const result = onbeforeedit(editEvent);
    if (result === false) {
      event.preventDefault();
    }
  }

  function handleBeforePaste(event: CustomEvent) {
    if (!onpaste) return;
    // Let the default handler work, but also notify parent
    const detail = event.detail;
    if (detail?.parsed) {
      onpaste(detail.parsed, detail.rowIndex ?? 0, detail.colIndex ?? 0);
    }
  }
</script>

{#if gridReady && RevoGridComponent && isBrowser}
  <div
    class="orbis-grid-container"
    style="height: {height}; width: 100%;"
    dir={rtl ? "rtl" : "ltr"}
  >
    <svelte:component
      this={RevoGridComponent}
      columns={revoColumns}
      {source}
      {theme}
      {range}
      {readonly}
      resize={true}
      rowSize={rowSize}
      useClipboard={clipboard}
      autoSizeColumn={autoSize}
      {editors}
      on:afteredit={handleAfterEdit}
      on:beforeedit={handleBeforeEdit}
      on:beforepaste={handleBeforePaste}
    />
  </div>
{:else}
  <!-- Loading skeleton matching grid dimensions -->
  <div
    class="orbis-grid-skeleton"
    style="height: {height}; width: 100%;"
  >
    <div class="skeleton-header">
      {#each columns.slice(0, 6) as col}
        <div class="skeleton-header-cell" style="width: {col.size ?? 120}px;">
          <div class="skeleton-bar" style="width: 60%;"></div>
        </div>
      {/each}
    </div>
    {#each Array(8) as _}
      <div class="skeleton-row">
        {#each columns.slice(0, 6) as col}
          <div class="skeleton-cell" style="width: {col.size ?? 120}px;">
            <div class="skeleton-bar" style="width: {30 + Math.random() * 50}%;"></div>
          </div>
        {/each}
      </div>
    {/each}
  </div>
{/if}

<style>
  .orbis-grid-container {
    position: relative;
    border-radius: var(--radius, 0.75rem);
    overflow: hidden;
  }

  /* ── Loading Skeleton ─────────────────────────────────────────────────── */
  .orbis-grid-skeleton {
    border: 1px solid hsl(var(--border, 0 0% 90%));
    border-radius: var(--radius, 0.75rem);
    overflow: hidden;
    background: hsl(var(--card, 0 0% 100%));
  }

  .skeleton-header {
    display: flex;
    border-bottom: 1px solid hsl(var(--border, 0 0% 90%));
    background: hsl(var(--muted, 0 0% 96%) / 0.3);
    height: 42px;
    align-items: center;
  }

  .skeleton-header-cell {
    padding: 0 12px;
    flex-shrink: 0;
    display: flex;
    align-items: center;
  }

  .skeleton-row {
    display: flex;
    border-bottom: 1px solid hsl(var(--border, 0 0% 90%) / 0.5);
    height: 42px;
    align-items: center;
  }

  .skeleton-cell {
    padding: 0 12px;
    flex-shrink: 0;
    display: flex;
    align-items: center;
  }

  .skeleton-bar {
    height: 10px;
    border-radius: 4px;
    background: hsl(var(--muted, 0 0% 96%));
    animation: skeleton-pulse 1.5s ease-in-out infinite;
  }

  @keyframes skeleton-pulse {
    0%, 100% { opacity: 0.4; }
    50% { opacity: 0.8; }
  }
</style>
