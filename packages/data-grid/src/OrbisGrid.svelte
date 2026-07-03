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
- S.C.R.A.P.S. design token integration (light/dark via oklch)
- Module-level caching for fast multi-instance rendering

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

  // ── Module-Level Cache ────────────────────────────────────────────────────
  // Shared across all OrbisGrid instances — avoids redundant dynamic imports.

  let _revoModule: any = null;
  let _modulePromise: Promise<any> | null = null;
  let _editorCache: Record<string, unknown> = {};

  async function loadRevoGridModule(): Promise<any> {
    if (_revoModule) return _revoModule;
    if (!_modulePromise) {
      _modulePromise = import("@revolist/svelte-datagrid");
    }
    _revoModule = await _modulePromise;
    return _revoModule;
  }

  async function loadNumberEditor(mod: any): Promise<unknown> {
    if (_editorCache["number"]) return _editorCache["number"];
    const editorMod = await import("./editors/NumberEditor.svelte");
    _editorCache["number"] = mod.Editor(editorMod.default);
    return _editorCache["number"];
  }

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
  let loadedEditors: Record<string, unknown> = $state({});

  // ── Dynamic Import (SSR-safe, module-cached) ──────────────────────────────

  onMount(async () => {
    if (!isBrowser) return;
    try {
      const mod = await loadRevoGridModule();
      RevoGridComponent = mod.RevoGrid;

      // Register editors from cache or load once
      const hasNumberEditor = columns.some((c) => c.editor === "number");
      if (hasNumberEditor) {
        const numEditor = await loadNumberEditor(mod);
        loadedEditors = { ...loadedEditors, number: numEditor };
      }

      gridReady = true;
    } catch (err) {
      console.error("[OrbisGrid] Failed to load RevoGrid:", err);
    }
  });

  // ── Map OrbisColumnDef → RevoGrid ColumnRegular ──────────────────────────

  let revoColumns = $derived.by(() => {
    if (!gridReady) return [];
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
          // Declarative variant → orbis-cell-{variant} class
          if (col.variant) classes.push(`orbis-cell-${col.variant}`);
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
  <!-- Loading skeleton using S.C.R.A.P.S. design tokens -->
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
    {#each Array(8) as _, i}
      <div class="skeleton-row" style="opacity: {1 - i * 0.06}">
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
    border-radius: var(--radius, 0.5rem);
    overflow: hidden;
  }

  /* ── Loading Skeleton — S.C.R.A.P.S. themed ───────────────────────────── */
  .orbis-grid-skeleton {
    border: 1px solid var(--border);
    border-radius: var(--radius, 0.5rem);
    overflow: hidden;
    background: var(--card);
    /* card-tactile shadow */
    box-shadow:
      0 0 0 1px oklch(0.3 0.05 280 / 0.04),
      0 1px 3px 0 oklch(0.3 0.05 280 / 0.08),
      0 2px 6px -1px oklch(0.3 0.05 280 / 0.05);
  }

  :global(.dark) .orbis-grid-skeleton {
    box-shadow:
      0 0 0 1px oklch(0.52 0.12 280 / 0.06),
      0 1px 3px 0 oklch(0 0 0 / 0.4),
      0 2px 8px -2px oklch(0 0 0 / 0.3);
  }

  .skeleton-header {
    display: flex;
    border-bottom: 1px solid var(--border);
    background: color-mix(in oklch, var(--muted), transparent 70%);
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
    border-bottom: 1px solid color-mix(in oklch, var(--border), transparent 50%);
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
    background: var(--muted);
    position: relative;
    overflow: hidden;
  }

  /* S.C.R.A.P.S. shimmer animation — matching globals.css skeleton */
  .skeleton-bar::after {
    content: "";
    position: absolute;
    inset: 0;
    background: linear-gradient(
      90deg,
      transparent 0%,
      oklch(0.52 0.12 280 / 0.06) 40%,
      oklch(0.52 0.12 280 / 0.12) 50%,
      oklch(0.52 0.12 280 / 0.06) 60%,
      transparent 100%
    );
    background-size: 200% 100%;
    animation: orbis-grid-shimmer 2.5s linear infinite;
  }

  :global(.dark) .skeleton-bar::after {
    background: linear-gradient(
      90deg,
      transparent 0%,
      oklch(0.6 0.12 280 / 0.08) 40%,
      oklch(0.6 0.12 280 / 0.16) 50%,
      oklch(0.6 0.12 280 / 0.08) 60%,
      transparent 100%
    );
  }

  @keyframes orbis-grid-shimmer {
    0% { background-position: 200% 0; }
    100% { background-position: -200% 0; }
  }
</style>
