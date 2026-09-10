<script lang="ts">
import { setContext } from "svelte";
import type {
  BulkResult,
  CellEditEvent,
  CellRange,
  GridActionEvent,
  GridRow,
  OrbisColumnDef,
  SelectionSnapshot,
  SortSpec,
} from "./types";
import {
  columnHeader,
  columnId,
  columnWidth,
  getCellValue,
  isColumnPinnedLeft,
  isColumnSortable,
} from "./types";
import {
  applyPaste,
  clampIndex,
  clearRange,
  contextMenuItems,
  defaultAlign,
  duplicateRows,
  emptySelection,
  extendRange,
  extractMatrix,
  fillDown,
  fillRight,
  formatCell,
  interpretKey,
  isRowSelected,
  normalizeRange,
  parseClipboard,
  queryClient,
  rangeContains,
  selectScope,
  selectedCount,
  toggleId,
  toggleSort,
  toTsv,
  visibleWindow,
  autoscrollDelta,
  colFromOffset,
  rowFromOffset,
  type ContextMenuItem,
} from "./engine";

interface Props {
  columns: OrbisColumnDef[];
  source: GridRow[];
  height?: string;
  rtl?: boolean;
  readonly?: boolean;
  rowIdKey?: string;
  rowSize?: number;
  range?: boolean;
  clipboard?: boolean;
  autoSize?: boolean;
  clientQuery?: boolean;
  search?: string;
  filter?: import("./types").GridFilter | null;
  oncelledit?: (event: CellEditEvent) => void;
  onbeforeedit?: (event: CellEditEvent) => boolean | void;
  onaction?: (event: GridActionEvent) => void;
  onpasteResult?: (result: BulkResult) => void;
  onselection?: (sel: SelectionSnapshot) => void;
  cellStyle?: (row: GridRow, col: OrbisColumnDef) => string;
  createEmptyRow?: () => GridRow;
}

let {
  columns,
  source = $bindable(),
  height = "100%",
  rtl = false,
  readonly = false,
  rowIdKey = "id",
  rowSize = 36,
  range = true,
  clipboard = true,
  autoSize = false,
  clientQuery = false,
  search = "",
  filter = null,
  oncelledit,
  onbeforeedit,
  onaction,
  onpasteResult,
  onselection,
  cellStyle,
  createEmptyRow,
}: Props = $props();

const HEADER_H = 36;
let root: HTMLDivElement | null = $state(null);
let bodyEl: HTMLDivElement | null = $state(null);
let scrollTop = $state(0);
let scrollLeft = $state(0);
let viewportH = $state(400);
let viewportW = $state(800);
let sorts = $state<SortSpec[]>([]);
let selection = $state<SelectionSnapshot>(emptySelection());
let editing = $state<{ row: number; col: number; draft: string } | null>(null);
let menu = $state<{
  x: number;
  y: number;
  items: ContextMenuItem[];
  row: number;
  col: number;
} | null>(null);
let pasteBanner = $state<BulkResult | null>(null);
let columnWidths = $state<Record<string, number>>({});
let resizing = $state<{ id: string; startX: number; startW: number } | null>(
  null,
);
let origin = $state<{ row: number; col: number } | null>(null);
let editorEl: HTMLInputElement | HTMLSelectElement | null = $state(null);
let dragging = $state(false);
let skipBlur = $state(false);

setContext("orbis-grid-onaction", (event: GridActionEvent) => {
  onaction?.(event);
});

const visibleCols = $derived(columns.filter((c) => !c.hidden));
const displayRows = $derived(
  clientQuery
    ? queryClient(source, visibleCols, { filter, search, sorts })
    : source,
);
const rowWindow = $derived(
  visibleWindow(
    scrollTop,
    viewportH - HEADER_H,
    rowSize,
    displayRows.length,
    10,
  ),
);
const totalWidth = $derived(
  visibleCols.reduce(
    (sum, col) => sum + (columnWidths[columnId(col)] ?? columnWidth(col)),
    0,
  ),
);
const totalHeight = $derived(displayRows.length * rowSize);
const colWindow = $derived.by(() => {
  if (visibleCols.length < 24)
    return {
      start: 0,
      end: visibleCols.length,
      offset: 0,
      count: visibleCols.length,
    };
  return visibleWindow(
    rtl ? 0 : scrollLeft,
    viewportW,
    120,
    visibleCols.length,
    4,
  );
});
const windowedCols = $derived(
  visibleCols.slice(colWindow.start, colWindow.end),
);

function rowKey(row: GridRow, index: number): string {
  const id = row[rowIdKey];
  return id == null ? `__i${index}` : String(id);
}

function widthOf(col: OrbisColumnDef): number {
  return columnWidths[columnId(col)] ?? columnWidth(col);
}

function offsetOf(index: number): number {
  let x = 0;
  for (let i = 0; i < index; i++) x += widthOf(visibleCols[i]!);
  return x;
}

function measure() {
  if (!bodyEl) return;
  viewportH = bodyEl.clientHeight;
  viewportW = bodyEl.clientWidth;
}

function onScroll() {
  if (!bodyEl) return;
  scrollTop = bodyEl.scrollTop;
  scrollLeft = bodyEl.scrollLeft;
}

function setActive(row: number, col: number, extend = false) {
  const r = clampIndex(row, Math.max(0, displayRows.length - 1));
  const c = clampIndex(col, Math.max(0, visibleCols.length - 1));
  if (extend && origin) {
    selection = {
      ...selection,
      active: { row: r, col: c },
      range: extendRange(origin, { row: r, col: c }),
    };
  } else {
    origin = { row: r, col: c };
    selection = {
      ...selection,
      active: { row: r, col: c },
      range: range
        ? { startRow: r, startCol: c, endRow: r, endCol: c }
        : selection.range,
    };
  }
  onselection?.(selection);
  const top = r * rowSize;
  if (bodyEl) {
    if (top < scrollTop) bodyEl.scrollTop = top;
    if (top + rowSize > scrollTop + viewportH - HEADER_H)
      bodyEl.scrollTop = top - (viewportH - HEADER_H) + rowSize;
  }
}

function commitOverlay(
  rowIndex: number,
  col: OrbisColumnDef,
  newValue: unknown,
) {
  const row = displayRows[rowIndex];
  if (!row) return;
  const id = columnId(col);
  const oldValue = getCellValue(row, col);
  const evt: CellEditEvent = {
    row,
    rowIndex,
    columnId: id,
    prop: id,
    oldValue,
    newValue,
    action: "update",
  };
  if (onbeforeedit && onbeforeedit(evt) === false) return;
  row[id] = newValue;
  source = source;
  oncelledit?.(evt);
}

function startEdit(row: number, col: number, seed?: string) {
  const column = visibleCols[col];
  if (!column || readonly || column.readonly) return;
  const type = column.type;
  if (type === "boolean" || column.editor === "checkbox") {
    const current = displayRows[row]
      ? getCellValue(displayRows[row]!, column)
      : false;
    commitOverlay(
      row,
      column,
      !(current === true || current === 1 || current === "true"),
    );
    return;
  }
  const value = displayRows[row] ? getCellValue(displayRows[row]!, column) : "";
  editing = { row, col, draft: seed ?? (value == null ? "" : String(value)) };
}

function commitEdit(moveRow = 0, moveCol = 0) {
  if (!editing) return;
  const column = visibleCols[editing.col];
  if (column) commitOverlay(editing.row, column, editing.draft);
  const r = editing.row;
  const c = editing.col;
  editing = null;
  if (moveRow || moveCol) setActive(r + moveRow, c + moveCol);
}

function cancelEdit() {
  skipBlur = true;
  editing = null;
}

function currentRange(): CellRange {
  return (
    selection.range ?? {
      startRow: selection.active?.row ?? 0,
      startCol: selection.active?.col ?? 0,
      endRow: selection.active?.row ?? 0,
      endCol: selection.active?.col ?? 0,
    }
  );
}

function copySelection() {
  if (!clipboard) return;
  const matrix = extractMatrix(displayRows, visibleCols, currentRange());
  const text = toTsv(matrix);
  void navigator.clipboard?.writeText(text);
}

function cutSelection() {
  copySelection();
  if (readonly) return;
  source = clearRange(source, visibleCols, currentRange());
}

function pasteText(text: string) {
  if (!clipboard || readonly) return;
  const matrix = parseClipboard(text);
  const start = normalizeRange(currentRange());
  const { rows, result } = applyPaste({
    rows: source,
    columns: visibleCols,
    startRow: start.startRow,
    startCol: start.startCol,
    matrix,
    createEmptyRow,
  });
  source = rows;
  pasteBanner = result;
  onpasteResult?.(result);
}

async function pasteFromClipboard() {
  const text = await navigator.clipboard?.readText();
  if (text) pasteText(text);
}

function hitFromEvent(e: PointerEvent): { row: number; col: number } | null {
  const node = document.elementFromPoint(e.clientX, e.clientY);
  const cell = node instanceof Element ? node.closest("[data-grid-r]") : null;
  if (
    cell instanceof HTMLElement &&
    cell.dataset.gridR != null &&
    cell.dataset.gridC != null
  ) {
    return { row: Number(cell.dataset.gridR), col: Number(cell.dataset.gridC) };
  }
  if (!bodyEl || !displayRows.length) return null;
  const rect = bodyEl.getBoundingClientRect();
  const relY = e.clientY - rect.top + bodyEl.scrollTop;
  const localX = e.clientX - rect.left;
  const relX = rtl
    ? bodyEl.scrollWidth - (localX + Math.max(0, bodyEl.scrollLeft))
    : localX + bodyEl.scrollLeft;
  return {
    row: rowFromOffset(relY, HEADER_H, rowSize, displayRows.length),
    col: colFromOffset(relX, visibleCols.map(widthOf)),
  };
}

function maybeAutoScroll(e: PointerEvent) {
  if (!bodyEl) return;
  const rect = bodyEl.getBoundingClientRect();
  const dy = autoscrollDelta(e.clientY, rect.top + HEADER_H, rect.bottom);
  const dx = autoscrollDelta(e.clientX, rect.left, rect.right);
  if (dy) bodyEl.scrollTop += dy;
  if (dx) bodyEl.scrollLeft += rtl ? -dx : dx;
}

function handlePointer(row: number, col: number, e: PointerEvent) {
  if (e.button !== 0) return;
  if ((e.target as HTMLElement | null)?.closest(".resize-handle")) return;
  e.preventDefault();
  root?.focus();
  if (e.ctrlKey || e.metaKey) {
    const data = displayRows[row];
    if (data) {
      selection = toggleId(selection, rowKey(data, row));
      onselection?.(selection);
    }
    return;
  }
  setActive(row, col, e.shiftKey && !!origin);
  if (range && !editing) dragging = true;
  const data = displayRows[row];
  if (data) {
    onaction?.({
      actionId: "select",
      row: data,
      rowIndex: row,
      prop: columnId(visibleCols[col] ?? {}),
      originalEvent: e,
    });
  }
}

function handleDblClick(row: number, col: number, e: MouseEvent) {
  startEdit(row, col);
  const data = displayRows[row];
  if (data) {
    onaction?.({
      actionId: "dblclick",
      row: data,
      rowIndex: row,
      prop: columnId(visibleCols[col] ?? {}),
      originalEvent: e,
    });
  }
}

function handleKeydown(e: KeyboardEvent) {
  const cmd = interpretKey(e, !!editing, rtl);
  if (cmd.preventDefault) e.preventDefault();
  const active = selection.active ?? { row: 0, col: 0 };
  const page = Math.max(1, Math.floor((viewportH - HEADER_H) / rowSize) - 1);

  switch (cmd.action) {
    case "move":
      setActive(
        Number.isFinite(cmd.rowDelta)
          ? active.row + cmd.rowDelta
          : cmd.rowDelta,
        Number.isFinite(cmd.colDelta)
          ? active.col + cmd.colDelta
          : cmd.colDelta,
      );
      break;
    case "extend":
      setActive(active.row + cmd.rowDelta, active.col + cmd.colDelta, true);
      break;
    case "tab":
      if (editing) commitEdit(0, cmd.colDelta);
      else setActive(active.row, active.col + cmd.colDelta);
      break;
    case "edit":
      startEdit(active.row, active.col, e.key.length === 1 ? e.key : undefined);
      break;
    case "commit":
      commitEdit(cmd.rowDelta, cmd.colDelta);
      break;
    case "cancel":
      cancelEdit();
      menu = null;
      break;
    case "delete":
      if (!readonly) source = clearRange(source, visibleCols, currentRange());
      break;
    case "copy":
      copySelection();
      break;
    case "cut":
      cutSelection();
      break;
    case "paste":
      void pasteFromClipboard();
      break;
    case "selectAll":
      selection = selectScope(
        displayRows.map((r, i) => rowKey(r, i)),
        "filtered",
        [],
        [],
      );
      onselection?.(selection);
      break;
    case "home":
      setActive(
        cmd.rowDelta < 0 ? 0 : active.row,
        cmd.colDelta < 0 ? 0 : active.col,
      );
      break;
    case "end":
      setActive(
        cmd.rowDelta > 0 ? displayRows.length - 1 : active.row,
        cmd.colDelta > 0 ? visibleCols.length - 1 : active.col,
      );
      break;
    case "page":
      setActive(active.row + cmd.rowDelta * page, active.col);
      break;
    case "fillDown":
      if (!readonly) source = fillDown(source, visibleCols, currentRange());
      break;
    case "fillRight":
      if (!readonly) source = fillRight(source, visibleCols, currentRange());
      break;
    default:
      break;
  }
}

function onHeaderClick(col: OrbisColumnDef, e: MouseEvent) {
  if (!isColumnSortable(col)) return;
  sorts = toggleSort(sorts, columnId(col), e.shiftKey);
}

function onContext(row: number, col: number, e: MouseEvent) {
  e.preventDefault();
  setActive(row, col, e.shiftKey);
  const hasSelection =
    selectedCount(selection, displayRows.length) > 1 ||
    (selection.range &&
      normalizeRange(selection.range).endRow !==
        normalizeRange(selection.range).startRow);
  menu = {
    x: e.clientX,
    y: e.clientY,
    row,
    col,
    items: contextMenuItems({
      context: "cell",
      hasSelection: !!hasSelection,
      canEdit: !readonly,
      filtered: clientQuery && !!filter,
    }),
  };
}

function runMenu(id: string) {
  const active = selection.active ?? {
    row: menu?.row ?? 0,
    col: menu?.col ?? 0,
  };
  const col = visibleCols[active.col];
  menu = null;
  switch (id) {
    case "copy":
    case "copy-selection":
      copySelection();
      break;
    case "copy-row":
      if (selection.active) {
        selection = {
          ...selection,
          range: {
            startRow: selection.active.row,
            startCol: 0,
            endRow: selection.active.row,
            endCol: visibleCols.length - 1,
          },
        };
        copySelection();
      }
      break;
    case "paste":
      void pasteFromClipboard();
      break;
    case "edit":
      startEdit(active.row, active.col);
      break;
    case "delete":
      if (!readonly) source = clearRange(source, visibleCols, currentRange());
      break;
    case "duplicate":
      if (!readonly) source = duplicateRows(source, [active.row]);
      break;
    case "select-row": {
      const row = displayRows[active.row];
      if (row)
        selection = toggleId(
          { ...selection, mode: "explicit", ids: new Set() },
          rowKey(row, active.row),
        );
      break;
    }
    case "select-all":
      selection = selectScope([], "filtered", [], []);
      break;
    case "sort-asc":
      if (col) sorts = [{ id: columnId(col), dir: "asc" }];
      break;
    case "sort-desc":
      if (col) sorts = [{ id: columnId(col), dir: "desc" }];
      break;
    case "hide-column":
      if (col) col.hidden = true;
      break;
    case "pin-column":
      if (col) col.pin = isColumnPinnedLeft(col) ? undefined : "left";
      break;
    default:
      onaction?.({
        actionId: id,
        row: displayRows[active.row] ?? {},
        rowIndex: active.row,
        prop: col ? columnId(col) : "",
        originalEvent: new MouseEvent("contextmenu"),
      });
  }
}

function onResizeStart(col: OrbisColumnDef, e: PointerEvent) {
  e.preventDefault();
  e.stopPropagation();
  resizing = { id: columnId(col), startX: e.clientX, startW: widthOf(col) };
}

function onPointerMove(e: PointerEvent) {
  if (resizing) {
    const delta = rtl
      ? resizing.startX - e.clientX
      : e.clientX - resizing.startX;
    columnWidths = {
      ...columnWidths,
      [resizing.id]: Math.max(48, resizing.startW + delta),
    };
    return;
  }
  if (!dragging || !range || editing) return;
  maybeAutoScroll(e);
  const hit = hitFromEvent(e);
  if (hit) setActive(hit.row, hit.col, true);
}

function onPointerUp() {
  resizing = null;
  dragging = false;
}

function editorInputType(col: OrbisColumnDef): string {
  switch (col.type) {
    case "integer":
    case "number":
    case "decimal":
    case "currency":
    case "percentage":
      return "number";
    case "date":
      return "date";
    case "datetime":
      return "datetime-local";
    default:
      return "text";
  }
}

function cellClass(
  row: GridRow,
  col: OrbisColumnDef,
  r: number,
  c: number,
): string {
  const parts: string[] = ["orbis-cell"];
  const errors = row.__errors as Map<string, string> | undefined;
  if (errors?.has(columnId(col))) parts.push("is-error");
  if (selection.active?.row === r && selection.active?.col === c)
    parts.push("is-active");
  if (rangeContains(selection.range, r, c)) parts.push("is-ranged");
  if (isRowSelected(selection, rowKey(row, r), displayRows.length))
    parts.push("is-row-selected");
  if (cellStyle) {
    const extra = cellStyle(row, col);
    if (extra) parts.push(extra);
  }
  const align = defaultAlign(col);
  parts.push(`align-${align}`);
  if (col.cellClass) parts.push(String(col.cellClass));
  return parts.join(" ");
}

let sizedFor = $state("");
$effect(() => {
  void source;
  void columns;
  const key = visibleCols.map((c) => columnId(c)).join("|");
  if (autoSize && key && key !== sizedFor && displayRows.length) {
    const next: Record<string, number> = {};
    for (const col of visibleCols) {
      let max = columnHeader(col).length * 8 + 24;
      const sample = Math.min(displayRows.length, 80);
      for (let i = 0; i < sample; i++) {
        max = Math.max(max, formatCell(displayRows[i]!, col).length * 7 + 24);
      }
      next[columnId(col)] = Math.min(420, Math.max(64, max));
    }
    columnWidths = next;
    sizedFor = key;
  }
  queueMicrotask(measure);
});

$effect(() => {
  if (editing && editorEl) editorEl.focus();
});

export function getGridElement() {
  return root;
}
export function getSource() {
  return source;
}
export function getColumns() {
  return columns;
}
export function refresh() {
  source = source;
}
export function setDataAt(rowIndex: number, colId: string, val: unknown) {
  if (!source[rowIndex]) return;
  source[rowIndex]![colId] = val;
  source = source;
}
export function clearFocus() {
  selection = emptySelection();
}
export function scrollToRow(index: number) {
  if (bodyEl) bodyEl.scrollTop = index * rowSize;
}
export function scrollToColumnIndex(index: number) {
  if (bodyEl) bodyEl.scrollLeft = offsetOf(index);
}
export function scrollToColumnProp(prop: string) {
  const i = visibleCols.findIndex((c) => columnId(c) === prop);
  if (i >= 0) scrollToColumnIndex(i);
}
</script>

<svelte:window
  onpointermove={onPointerMove}
  onpointerup={onPointerUp}
  onpointerdown={(e) => {
    if (menu && e.target instanceof Node && !(e.target as HTMLElement).closest(".orbis-menu")) {
      menu = null;
    }
  }}
/>

<div
  bind:this={root}
  class="orbis-grid"
  class:is-rtl={rtl}
  class:is-selecting={dragging}
  style:height
  style:--row-h="{rowSize}px"
  style:--header-h="{HEADER_H}px"
  dir={rtl ? "rtl" : "ltr"}
  role="grid"
  tabindex="0"
  aria-rowcount={displayRows.length}
  aria-colcount={visibleCols.length}
  onkeydown={handleKeydown}
  onfocus={() => measure()}
>
  {#if pasteBanner}
    <div class="orbis-paste-banner" role="status">
      <span class="ok">{pasteBanner.acceptedCount} accepted</span>
      {#if pasteBanner.rejectedCount}
        <span class="rej">{pasteBanner.rejectedCount} rejected</span>
      {/if}
      <button type="button" class="banner-dismiss" onclick={() => (pasteBanner = null)}>Dismiss</button>
    </div>
  {/if}

  <div
    class="orbis-grid-body"
    onscroll={onScroll}
    {@attach (el) => {
      const node = el as HTMLDivElement;
      bodyEl = node;
      measure();
      const ro = new ResizeObserver(measure);
      ro.observe(node);
      return () => {
        ro.disconnect();
        if (bodyEl === node) bodyEl = null;
      };
    }}
  >
    <div class="orbis-spacer" style:height="{HEADER_H + totalHeight}px" style:width="{totalWidth}px">
      <div class="orbis-grid-head" role="row" style:padding-inline-start="{colWindow.offset}px">
        {#each windowedCols as col, i (columnId(col) || i)}
          {@const c = colWindow.start + i}
          {@const id = columnId(col)}
          {@const Header = col.headerCell}
          <div
            class="orbis-hcell"
            class:pinned={isColumnPinnedLeft(col)}
            role="columnheader"
            aria-colindex={c + 1}
            aria-sort={sorts.find((s) => s.id === id)?.dir === "desc" ? "descending" : sorts.find((s) => s.id === id) ? "ascending" : "none"}
            style:width="{widthOf(col)}px"
            onclick={(e) => onHeaderClick(col, e)}
          >
            {#if Header}
              <Header row={{}} column={{ ...col, id, meta: { ...col.meta, isSelectionHeader: id === "__selected", selectedAll: displayRows.length > 0 && displayRows.every((row, idx) => isRowSelected(selection, rowKey(row, idx), displayRows.length)) } }} />
            {:else}
              <span class="hlabel">{columnHeader(col)}</span>
            {/if}
            <button
              type="button"
              class="resize-handle"
              aria-label="Resize column"
              onpointerdown={(e) => onResizeStart(col, e)}
            ></button>
          </div>
        {/each}
      </div>
      <div class="orbis-window" style:transform="translateY({HEADER_H + rowWindow.offset}px)">
        {#each { length: rowWindow.count } as _, i}
          {@const r = rowWindow.start + i}
          {@const row = displayRows[r]}
          {#if row}
            <div
              class="orbis-row"
              class:selected={isRowSelected(selection, rowKey(row, r), displayRows.length)}
              role="row"
              aria-rowindex={r + 1}
              aria-selected={isRowSelected(selection, rowKey(row, r), displayRows.length)}
              data-row={rowKey(row, r)}
              style:padding-inline-start="{colWindow.offset}px"
            >
              {#each windowedCols as col, i (columnId(col) || i)}
                {@const c = colWindow.start + i}
                {@const id = columnId(col)}
                {@const Cell = col.cell}
                {@const isEdit = editing?.row === r && editing?.col === c}
                <div
                  class={cellClass(row, col, r, c)}
                  role="gridcell"
                  tabindex="-1"
                  data-grid-r={r}
                  data-grid-c={c}
                  aria-colindex={c + 1}
                  aria-selected={selection.active?.row === r && selection.active?.col === c}
                  style:width="{widthOf(col)}px"
                  title={row.__errors instanceof Map ? row.__errors.get(id) : undefined}
                  onpointerdown={(e) => handlePointer(r, c, e)}
                  ondblclick={(e) => handleDblClick(r, c, e)}
                  oncontextmenu={(e) => onContext(r, c, e)}
                >
                  {#if isEdit && editing}
                    {#if col.options?.length}
                      <select
                        class="orbis-editor"
                        bind:this={editorEl}
                        bind:value={editing.draft}
                        onblur={() => {
                          if (skipBlur) {
                            skipBlur = false;
                            return;
                          }
                          commitEdit();
                        }}
                        onkeydown={(e) => {
                          if (e.key === "Escape") {
                            e.preventDefault();
                            cancelEdit();
                          }
                          e.stopPropagation();
                        }}
                      >
                        {#each col.options as opt}
                          <option value={String(opt.value)}>{opt.label}</option>
                        {/each}
                      </select>
                    {:else}
                      <input
                        class="orbis-editor"
                        type={editorInputType(col)}
                        bind:this={editorEl}
                        bind:value={editing.draft}
                        onblur={() => {
                          if (skipBlur) {
                            skipBlur = false;
                            return;
                          }
                          commitEdit();
                        }}
                        onkeydown={(e) => {
                          if (e.key === "Enter") {
                            e.preventDefault();
                            commitEdit(e.shiftKey ? -1 : 1, 0);
                          } else if (e.key === "Tab") {
                            e.preventDefault();
                            commitEdit(0, e.shiftKey ? (rtl ? 1 : -1) : rtl ? -1 : 1);
                          } else if (e.key === "Escape") {
                            e.preventDefault();
                            cancelEdit();
                          }
                          e.stopPropagation();
                        }}
                      />
                    {/if}
                  {:else if Cell}
                    <Cell {row} column={{ ...col, id }} />
                  {:else}
                    <span class="orbis-text">{formatCell(row, col)}</span>
                  {/if}
                </div>
              {/each}
            </div>
          {/if}
        {/each}
      </div>
    </div>
  </div>
</div>

{#if menu}
  <ul
    class="orbis-menu"
    style:left="{menu.x}px"
    style:top="{menu.y}px"
    role="menu"
  >
    {#each menu.items as item}
      <li role="none">
        <button
          type="button"
          role="menuitem"
          class:danger={item.danger}
          disabled={item.disabled}
          onclick={() => runMenu(item.id)}
        >
          {item.label}
        </button>
      </li>
    {/each}
  </ul>
{/if}

<style>
  .orbis-grid {
    position: relative;
    display: flex;
    flex-direction: column;
    width: 100%;
    min-height: 0;
    background: var(--card);
    color: var(--card-foreground);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    overflow: hidden;
    font-family: var(--font-sans, "Rubik", system-ui, sans-serif);
    font-size: 0.75rem;
    line-height: 1.25;
    outline: none;
  }
  .orbis-grid:focus-visible {
    box-shadow: 0 0 0 3px color-mix(in oklab, var(--ring) 35%, transparent);
  }
  .orbis-grid.is-selecting {
    user-select: none;
    cursor: cell;
  }
  .orbis-paste-banner {
    display: flex;
    gap: 0.5rem;
    align-items: center;
    padding: 0.375rem 0.75rem;
    font-size: 0.6875rem;
    font-weight: 500;
    background: var(--secondary);
    color: var(--secondary-foreground);
    border-bottom: 1px solid var(--border);
  }
  .orbis-paste-banner .ok {
    color: var(--brand-lime-dark, var(--foreground));
  }
  .orbis-paste-banner .rej {
    color: var(--destructive);
  }
  .banner-dismiss {
    margin-inline-start: auto;
    border: 0;
    background: transparent;
    color: var(--muted-foreground);
    font: inherit;
    cursor: pointer;
    border-radius: var(--radius-sm);
    padding: 0.125rem 0.375rem;
  }
  .banner-dismiss:hover {
    background: var(--accent);
    color: var(--accent-foreground);
  }
  .orbis-grid-head {
    display: flex;
    position: sticky;
    top: 0;
    height: var(--header-h);
    background: var(--muted);
    color: var(--muted-foreground);
    border-bottom: 1px solid var(--border);
    flex-shrink: 0;
    z-index: 3;
  }
  .orbis-hcell {
    position: relative;
    display: flex;
    align-items: center;
    padding: 0 0.625rem;
    font-weight: 600;
    letter-spacing: 0.01em;
    border-inline-end: 1px solid var(--border);
    user-select: none;
    flex: 0 0 auto;
    min-width: 0;
    cursor: pointer;
  }
  .orbis-hcell:hover {
    color: var(--foreground);
    background: color-mix(in oklab, var(--accent) 80%, var(--muted));
  }
  .hlabel {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .resize-handle {
    position: absolute;
    inset-inline-end: 0;
    top: 0;
    width: 6px;
    height: 100%;
    cursor: col-resize;
    border: 0;
    background: transparent;
  }
  .resize-handle:hover {
    background: color-mix(in oklab, var(--ring) 45%, transparent);
  }
  .orbis-grid-body {
    flex: 1;
    min-height: 0;
    overflow: auto;
    position: relative;
    background: var(--card);
  }
  .orbis-window {
    will-change: transform;
  }
  .orbis-row {
    display: flex;
    height: var(--row-h);
    border-bottom: 1px solid var(--border);
    background: var(--card);
  }
  .orbis-row:hover {
    background: var(--accent);
  }
  .orbis-row.selected {
    background: color-mix(in oklab, var(--primary) 8%, var(--card));
  }
  .orbis-cell {
    display: flex;
    align-items: center;
    height: 100%;
    padding: 0 0.625rem;
    border-inline-end: 1px solid var(--border);
    min-width: 0;
    overflow: hidden;
    flex: 0 0 auto;
    color: var(--foreground);
  }
  .orbis-cell.align-end {
    justify-content: flex-end;
    text-align: end;
    font-variant-numeric: tabular-nums;
  }
  .orbis-cell.align-center {
    justify-content: center;
    text-align: center;
  }
  .orbis-cell.is-ranged {
    background: color-mix(in oklab, var(--primary) 14%, transparent);
  }
  .orbis-cell.is-active {
    box-shadow: inset 0 0 0 1.5px var(--ring);
    z-index: 1;
    background: var(--card);
  }
  .orbis-cell.is-error {
    background: color-mix(in oklab, var(--destructive) 16%, var(--card));
    box-shadow: inset 0 0 0 1px var(--destructive);
  }
  .orbis-text {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    width: 100%;
  }
  .orbis-editor {
    width: 100%;
    height: calc(100% - 2px);
    border: 0;
    background: var(--input);
    color: var(--foreground);
    font: inherit;
    outline: none;
    border-radius: var(--radius-sm);
    padding: 0 0.25rem;
  }
  .orbis-editor:focus {
    box-shadow: inset 0 0 0 1px var(--ring);
  }
  .orbis-menu {
    position: fixed;
    z-index: 80;
    margin: 0;
    padding: 0.25rem;
    list-style: none;
    min-width: 11.5rem;
    background: var(--popover);
    color: var(--popover-foreground);
    border: 1px solid var(--border);
    border-radius: var(--radius-md);
    box-shadow: 0 10px 28px var(--overlay);
  }
  .orbis-menu button {
    display: block;
    width: 100%;
    text-align: start;
    border: 0;
    background: transparent;
    color: inherit;
    font: inherit;
    font-size: 0.75rem;
    padding: 0.375rem 0.5rem;
    border-radius: var(--radius-sm);
    cursor: pointer;
  }
  .orbis-menu button:hover {
    background: var(--accent);
    color: var(--accent-foreground);
  }
  .orbis-menu button.danger {
    color: var(--destructive);
  }
  .orbis-menu button.danger:hover {
    background: color-mix(in oklab, var(--destructive) 12%, var(--popover));
  }
</style>
