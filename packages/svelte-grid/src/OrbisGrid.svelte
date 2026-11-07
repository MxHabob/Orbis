<script lang="ts">
import { setContext } from "svelte";
import { Badge } from "@orbis/ui/badge";
import { Checkbox } from "@orbis/ui/checkbox";
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@orbis/ui/dialog";
import type {
  ActionItem,
  BulkResult,
  CellEditEvent,
  CellRange,
  DataQuery,
  GridActionEvent,
  GridFilter,
  GridRow,
  GridRowsChangeEvent,
  OrbisColumnDef,
  FilterCondition,
  FilterOperator,
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
  coerceValue,
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
  visibleVariableWindow,
  visibleWindow,
  autoscrollDelta,
  colFromOffset,
  rowFromOffset,
  type ContextMenuItem,
} from "./engine";
import { DEFAULT_PAGE_SIZE } from "./data-source";

interface Props {
  columns: OrbisColumnDef[];
  source: GridRow[];
  loading?: boolean;
  emptyTitle?: string;
  emptyDescription?: string;
  height?: string;
  rtl?: boolean;
  readonly?: boolean;
  rowIdKey?: string;
  rowSize?: number;
  range?: boolean;
  clipboard?: boolean;
  autoSize?: boolean;
  autoFit?: boolean;
  clientQuery?: boolean;
  serverQuery?: boolean;
  showSearch?: boolean;
  showFilterButton?: boolean;
  uniformColumns?: boolean;
  search?: string;
  searchPlaceholder?: string;
  filter?: GridFilter | null;
  toolbar?: boolean;
  onquerychange?: (query: DataQuery) => void;
  onfilterchange?: (filter: GridFilter | null) => void;
  oncelledit?: (event: CellEditEvent) => void;
  onbeforeedit?: (event: CellEditEvent) => boolean | void;
  onaction?: (event: GridActionEvent) => void;
  onrowschange?: (event: GridRowsChangeEvent) => void;
  onpasteResult?: (result: BulkResult) => void;
  onselection?: (sel: SelectionSnapshot) => void;
  cellStyle?: (row: GridRow, col: OrbisColumnDef) => string;
  createEmptyRow?: () => GridRow;
  hasMore?: boolean;
  onloadmore?: () => void | Promise<unknown>;
}

let {
  columns,
  source = $bindable([]),
  loading = false,
  emptyTitle = "No data",
  emptyDescription = "",
  height = "100%",
  rtl = false,
  readonly = false,
  rowIdKey = "id",
  rowSize = 36,
  range = true,
  clipboard = true,
  autoSize = false,
  autoFit = true,
  clientQuery = false,
  serverQuery = false,
  showSearch = true,
  showFilterButton = true,
  uniformColumns = true,
  search = $bindable(""),
  searchPlaceholder,
  filter = $bindable(null),
  toolbar = true,
  onquerychange,
  onfilterchange,
  oncelledit,
  onbeforeedit,
  onaction,
  onrowschange,
  onpasteResult,
  onselection,
  cellStyle,
  createEmptyRow,
  hasMore = false,
  onloadmore,
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
let editing = $state<{
  row: number;
  col: number;
  draft: string;
  error?: string;
} | null>(null);
let editorEl: HTMLInputElement | HTMLSelectElement | null = $state(null);
let filterPanelOpen = $state(false);
let filterColumn = $state("");
let filterOperator = $state<FilterOperator>("contains");
let filterValue = $state("");
let filterEndValue = $state("");
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
let dragging = $state(false);
let skipBlur = $state(false);
let isLoadingMore = $state(false);
let loadMoreError = $state<unknown>(null);
let serverQueryTimer: ReturnType<typeof setTimeout> | null = null;
let lastServerQueryKey = "";

setContext("orbis-grid-onaction", (event: GridActionEvent) => {
  onaction?.(event);
});

const usesServerQuery = $derived(serverQuery || Boolean(onquerychange));
const shouldRunClientQuery = $derived(
  clientQuery ||
    (!usesServerQuery &&
      (Boolean(search) || Boolean(filter) || sorts.length > 0)),
);

const safeSource = $derived(Array.isArray(source) ? source : []);

const visibleCols = $derived(columns.filter((c) => !c.hidden));
const filterableCols = $derived(
  visibleCols.filter(
    (col) =>
      col.filterable !== false &&
      !columnId(col).startsWith("__") &&
      col.type !== "action",
  ),
);
const filterDraftColumn = $derived(
  filterableCols.find((col) => columnId(col) === filterColumn) ??
    filterableCols[0],
);
const filterCount = $derived(countFilterConditions(filter));
const displayRows = $derived(
  shouldRunClientQuery
    ? queryClient(safeSource, visibleCols, {
        filter,
        search: usesServerQuery ? undefined : search,
        sorts: usesServerQuery && !clientQuery ? [] : sorts,
      })
    : safeSource,
);

const autoSizeWidths = $derived.by<Record<string, number>>(() => {
  if (!autoSize || displayRows.length === 0) return {};
  const widths: Record<string, number> = {};
  let sharedDataWidth = DEFAULT_DATA_COLUMN_WIDTH;

  for (const col of visibleCols) {
    let max = columnHeader(col).length * 8 + 24;
    for (let index = 0; index < Math.min(displayRows.length, 80); index++) {
      max = Math.max(
        max,
        formatCell(displayRows[index]!, col).length * 7 + 24,
      );
    }
    widths[columnId(col)] = Math.min(420, Math.max(64, max));
    if (!isUtilityColumn(col)) {
      sharedDataWidth = Math.max(sharedDataWidth, widths[columnId(col)]!);
    }
  }

  if (uniformColumns) {
    for (const col of visibleCols) {
      if (!isUtilityColumn(col)) {
        widths[columnId(col)] = sharedDataWidth;
      }
    }
  }
  return widths;
});

function countFilterConditions(value: GridFilter | null | undefined): number {
  if (!value) return 0;
  return (
    (value.conditions?.length ?? 0) +
    (value.groups ?? []).reduce(
      (count, group) => count + countFilterConditions(group),
      0,
    )
  );
}

function resetQueryView() {
  scrollTop = 0;
  if (bodyEl) bodyEl.scrollTop = 0;
}

function notifyQueryChange() {
  resetQueryView();
  if (!usesServerQuery || !onquerychange) return;
  const query: DataQuery = {
    page: 1,
    pageSize: DEFAULT_PAGE_SIZE,
    filter: filter ?? undefined,
    search: search || undefined,
    sorts,
  };
  const queryKey = JSON.stringify(query);
  if (serverQueryTimer) clearTimeout(serverQueryTimer);
  if (queryKey === lastServerQueryKey) return;
  serverQueryTimer = setTimeout(() => {
    lastServerQueryKey = queryKey;
    serverQueryTimer = null;
    onquerychange?.(query);
  }, 120);
}

function handleSearchInput(event: Event) {
  search = (event.currentTarget as HTMLInputElement).value;
  notifyQueryChange();
}

function clearSearch() {
  search = "";
  notifyQueryChange();
}

const filterOperators: Array<{ value: FilterOperator; label: string }> = [
  { value: "contains", label: "Contains" },
  { value: "not_contains", label: "Does not contain" },
  { value: "eq", label: "Is" },
  { value: "neq", label: "Is not" },
  { value: "starts_with", label: "Starts with" },
  { value: "ends_with", label: "Ends with" },
  { value: "gt", label: "Greater than" },
  { value: "gte", label: "Greater than or equal" },
  { value: "lt", label: "Less than" },
  { value: "lte", label: "Less than or equal" },
  { value: "between", label: "Between" },
  { value: "in", label: "Is one of" },
  { value: "not_in", label: "Is not one of" },
  { value: "is_null", label: "Is empty" },
  { value: "is_not_null", label: "Is not empty" },
  { value: "bool_true", label: "Is true" },
  { value: "bool_false", label: "Is false" },
];

const DEFAULT_DATA_COLUMN_WIDTH = 144;

function isUtilityColumn(col: OrbisColumnDef): boolean {
  const id = columnId(col);
  return (
    id === "__idx" ||
    id === "__rowIndex" ||
    id === "__selected" ||
    col.type === "action" ||
    id === "actions" ||
    col.prop === "actions"
  );
}

const rowWindow = $derived(
  visibleWindow(
    scrollTop,
    viewportH - HEADER_H,
    rowSize,
    displayRows.length,
    10,
  ),
);

const computedWidthMap = $derived.by<Record<string, number>>(() => {
  const map: Record<string, number> = {};
  if (!visibleCols.length) return map;

  let totalBase = 0;
  let totalFlex = 0;

  const colDetails = visibleCols.map((col) => {
    const id = columnId(col);
    const userW = columnWidths[id];
    const isUtility = isUtilityColumn(col);
    const isDedicatedFixed =
      isUtility || col.flex === 0 || col.flexgrow === 0;

    const baseW =
      userW ??
      autoSizeWidths[id] ??
      (uniformColumns && !isUtility
        ? DEFAULT_DATA_COLUMN_WIDTH
        : (col.width ??
          col.size ??
          (isDedicatedFixed
            ? id === "__idx" || id === "__rowIndex"
              ? 48
              : id === "__selected"
                ? 40
                : 90
            : col.type === "avatar"
              ? 220
              : col.type === "badge" || col.type === "status"
                ? 110
                : 140)));

    const minW =
      uniformColumns && !isUtility
        ? 60
        : (col.minWidth ??
          (id === "__idx" || id === "__rowIndex"
            ? 48
            : id === "__selected"
              ? 40
              : col.type === "action" || id === "actions"
                ? 80
                : 60));

    const maxW =
      uniformColumns && !isUtility ? Infinity : (col.maxWidth ?? Infinity);
    const hasManualResize = userW !== undefined;
    const flex = hasManualResize
      ? 0
      : (uniformColumns && !isUtility
        ? 1
        : (col.flex ??
          col.flexgrow ??
          (isDedicatedFixed ? 0 : col.type === "avatar" ? 2 : 1)));

    const finalBaseW = Math.max(minW, baseW);
    totalBase += finalBaseW;
    totalFlex += flex;

    return {
      id,
      col,
      baseW: finalBaseW,
      minW,
      maxW,
      flex,
      hasManualResize,
    };
  });

  const availableW = viewportW;
  const shouldAutoFit = autoFit || autoSize;
  const extraW =
    shouldAutoFit && availableW > totalBase ? availableW - totalBase : 0;

  if (extraW > 0 && totalFlex > 0) {
    let allocatedExtra = 0;
    for (let i = 0; i < colDetails.length; i++) {
      const item = colDetails[i]!;
      if (item.flex > 0) {
        const share = Math.floor((extraW * item.flex) / totalFlex);
        const targetW = Math.min(
          item.maxW,
          Math.max(item.minW, item.baseW + share),
        );
        map[item.id] = targetW;
        allocatedExtra += targetW - item.baseW;
      } else {
        map[item.id] = item.baseW;
      }
    }
    const remainder = Math.floor(extraW - allocatedExtra);
    if (remainder > 0) {
      const firstFlex = colDetails.find((d) => d.flex > 0);
      if (firstFlex && map[firstFlex.id] !== undefined) {
        map[firstFlex.id] = Math.min(
          firstFlex.maxW,
          map[firstFlex.id]! + remainder,
        );
      }
    }
  } else {
    for (const item of colDetails) {
      map[item.id] = item.baseW;
    }
  }

  return map;
});

function widthOf(col: OrbisColumnDef): number {
  const id = columnId(col);
  return computedWidthMap[id] ?? columnWidths[id] ?? columnWidth(col);
}

const columnSizes = $derived(visibleCols.map((col) => widthOf(col)));
const totalWidth = $derived(
  columnSizes.reduce((sum, width) => sum + width, 0),
);

const totalHeight = $derived(displayRows.length * rowSize);

const colWindow = $derived.by(() => {
  if (visibleCols.length < 24 || rtl)
    return {
      start: 0,
      end: visibleCols.length,
      offset: 0,
      count: visibleCols.length,
    };
  return visibleVariableWindow(scrollLeft, viewportW, columnSizes, 4);
});

const windowedCols = $derived(
  visibleCols.slice(colWindow.start, colWindow.end),
);

function rowKey(row: GridRow, index: number): string {
  const id = row[rowIdKey];
  return id == null ? `__i${index}` : String(id);
}

function setFilter(next: GridFilter | null) {
  filter = next;
  onfilterchange?.(next);
  notifyQueryChange();
}

function filterScalar(value: string, col: OrbisColumnDef): unknown {
  if (col.options?.length) {
    const option = col.options.find(
      (item) =>
        String(item.value).toLowerCase() === value.toLowerCase() ||
        item.label.toLowerCase() === value.toLowerCase(),
    );
    if (option) return option.value;
  }
  if (
    col.type === "number" ||
    col.type === "integer" ||
    col.type === "decimal" ||
    col.type === "currency" ||
    col.type === "percentage"
  ) {
    const number = Number(value.replace(/[%$,\s]/g, ""));
    return Number.isFinite(number) ? number : value;
  }
  return value;
}

function filterOperatorRequiresValue(operator: FilterOperator): boolean {
  return !["is_null", "is_not_null", "bool_true", "bool_false"].includes(
    operator,
  );
}

function addFilterCondition() {
  const col = filterDraftColumn;
  if (!canAddFilterCondition() || !col) return;
  const field = columnId(col);
  let value: unknown;
  if (filterOperatorRequiresValue(filterOperator)) {
    if (filterOperator === "between") {
      value = [
        filterScalar(filterValue, col),
        filterScalar(filterEndValue, col),
      ];
    } else if (filterOperator === "in" || filterOperator === "not_in") {
      value = filterValue
        .split(",")
        .map((item) => filterScalar(item.trim(), col))
        .filter((item) => item !== "");
    } else {
      value = filterScalar(filterValue, col);
    }
  }
  setFilter({
    logic: filter?.logic ?? "AND",
    conditions: [
      ...(filter?.conditions ?? []),
      { field, operator: filterOperator, value },
    ],
    groups: filter?.groups,
    search: filter?.search,
    sort_by: filter?.sort_by,
    sort_dir: filter?.sort_dir,
  });
  filterValue = "";
  filterEndValue = "";
}

function canAddFilterCondition(): boolean {
  if (!filterDraftColumn) return false;
  if (!filterOperatorRequiresValue(filterOperator)) return true;
  if (filterOperator === "between")
    return filterValue.trim() !== "" && filterEndValue.trim() !== "";
  return filterValue.trim() !== "";
}

function removeFilterCondition(index: number) {
  if (!filter) return;
  const conditions = filter.conditions.filter((_, itemIndex) => itemIndex !== index);
  setFilter({
    ...filter,
    conditions,
    groups: filter.groups,
  });
}

function filterConditionLabel(condition: FilterCondition): string {
  const col = filterableCols.find(
    (item) => columnId(item) === condition.field,
  );
  const operator = operatorLabel(condition.operator);
  const value = Array.isArray(condition.value)
    ? condition.value.join(" – ")
    : condition.value == null
      ? ""
      : String(condition.value);
  return [col ? columnHeader(col) : condition.field, operator, value]
    .filter(Boolean)
    .join(" ");
}

function operatorLabel(value: string): string {
  const labels: Record<string, string> = rtl
    ? {
        contains: "يحتوي على",
        not_contains: "لا يحتوي على",
        eq: "يساوي",
        neq: "لا يساوي",
        starts_with: "يبدأ بـ",
        ends_with: "ينتهي بـ",
        gt: "أكبر من",
        gte: "أكبر من أو يساوي",
        lt: "أقل من",
        lte: "أقل من أو يساوي",
        between: "بين",
        in: "أحد القيم",
        not_in: "ليس من القيم",
        is_null: "فارغ",
        is_not_null: "غير فارغ",
        bool_true: "نعم",
        bool_false: "لا",
      }
    : {};
  return (
    labels[value] ??
    filterOperators.find((item) => item.value === value)?.label ??
    value
  );
}

function filterInputType(col: OrbisColumnDef | undefined): string {
  if (col?.type === "date") return "date";
  if (col?.type === "datetime") return "datetime-local";
  return "text";
}

function createEmptyGridRow(): GridRow {
  const newRow = { ...(createEmptyRow?.() ?? {}) };
  if (!createEmptyRow) {
    for (const col of columns) {
      const id = columnId(col);
      if (
        id &&
        !id.startsWith("__") &&
        !col.readonly &&
        col.type !== "action" &&
        col.type !== "computed" &&
        col.type !== "formula"
      ) {
        newRow[id] = null;
      }
    }
  }
  if (
    rowIdKey &&
    !rowIdKey.startsWith("__") &&
    newRow[rowIdKey] == null &&
    typeof crypto !== "undefined" &&
    typeof crypto.randomUUID === "function"
  ) {
    newRow[rowIdKey] = crypto.randomUUID();
  }
  return newRow;
}

function appendEmptyRow(): GridRow {
  const newRow = createEmptyGridRow();
  updateRows([...safeSource, newRow], "insert", [newRow]);
  return newRow;
}

function isEditableColumn(col: OrbisColumnDef): boolean {
  const id = columnId(col);
  return (
    !col.readonly &&
    !id.startsWith("__") &&
    col.type !== "action" &&
    col.type !== "computed" &&
    col.type !== "formula"
  );
}

function editableColumnIndices(): number[] {
  return visibleCols.flatMap((col, index) =>
    isEditableColumn(col) ? [index] : [],
  );
}

function firstEditableColumnIndex(): number {
  return editableColumnIndices()[0] ?? 0;
}

function lastEditableColumnIndex(): number {
  const indices = editableColumnIndices();
  return indices[indices.length - 1] ?? 0;
}

function addRowFromToolbar() {
  search = "";
  if (filter) setFilter(null);
  filterPanelOpen = false;
  const newRow = appendEmptyRow();
  requestAnimationFrame(() => {
    const index = displayRows.indexOf(newRow);
    if (index >= 0) setActive(index, firstEditableColumnIndex());
  });
}

function moveActiveAfterEdit(
  row: number,
  col: number,
  rowDelta: number,
  colDelta: number,
) {
  let nextRow = row + rowDelta;
  let nextCol = col;
  const editableCols = editableColumnIndices();
  if (colDelta > 0) {
    const nextEditable = editableCols.find((index) => index > col);
    if (nextEditable !== undefined) nextCol = nextEditable;
    else {
      nextRow++;
      nextCol = firstEditableColumnIndex();
    }
  } else if (colDelta < 0) {
    let previousEditable: number | undefined;
    for (let index = editableCols.length - 1; index >= 0; index--) {
      if (editableCols[index]! < col) {
        previousEditable = editableCols[index]!;
        break;
      }
    }
    if (previousEditable !== undefined) nextCol = previousEditable;
    else {
      nextRow--;
      nextCol = lastEditableColumnIndex();
    }
  }
  if (nextRow >= displayRows.length) {
    if (
      !readonly &&
      displayRows.length === safeSource.length &&
      (rowDelta > 0 || colDelta > 0)
    ) {
      const newRow = appendEmptyRow();
      requestAnimationFrame(() => {
        const index = displayRows.indexOf(newRow);
        if (index >= 0) setActive(index, nextCol);
      });
      return;
    }
    nextRow = displayRows.length - 1;
  }
  setActive(nextRow, nextCol);
}

function isGridRowSelected(row: GridRow, index: number): boolean {
  if (Object.hasOwn(row, "__selected")) return row.__selected === true;
  return isRowSelected(selection, rowKey(row, index), displayRows.length);
}

function offsetOf(index: number): number {
  let x = 0;
  for (let i = 0; i < index; i++) x += widthOf(visibleCols[i]!);
  return x;
}

function measure() {
  const target = bodyEl || root;
  if (!target) return;
  if (target.clientHeight > 0) viewportH = target.clientHeight;
  if (target.clientWidth > 0) viewportW = target.clientWidth;
}

function onScroll() {
  if (!bodyEl) return;
  scrollTop = bodyEl.scrollTop;
  scrollLeft = bodyEl.scrollLeft;
  requestMoreIfNeeded();
}

function requestMoreIfNeeded(force = false) {
  if (
    (!bodyEl && !force) ||
    !hasMore ||
    !onloadmore ||
    loading ||
    isLoadingMore ||
    loadMoreError
  ) {
    return;
  }
  if (!force && bodyEl) {
    const remaining =
      bodyEl.scrollHeight - bodyEl.scrollTop - bodyEl.clientHeight;
    if (remaining > rowSize * 8) return;
  }

  isLoadingMore = true;
  loadMoreError = null;
  Promise.resolve()
    .then(() => onloadmore?.())
    .catch((error: unknown) => {
      console.error("OrbisGrid failed to load the next row batch:", error);
      loadMoreError = error;
    })
    .finally(() => {
      isLoadingMore = false;
      if (!loadMoreError) requestAnimationFrame(() => requestMoreIfNeeded());
    });
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
): boolean {
  const row = displayRows[rowIndex];
  if (!row) return false;
  const id = columnId(col);
  const oldValue = getCellValue(row, col);
  const validation = coerceValue(newValue, col, row);
  if (!validation.ok) {
    if (editing?.row === rowIndex && editing.col === visibleCols.indexOf(col)) {
      editing = { ...editing, error: validation.message ?? "Invalid value" };
    }
    return false;
  }
  const sourceIndex = source.indexOf(row);
  if (sourceIndex < 0) return false;
  const updatedRow = { ...row, [id]: validation.value };
  const evt: CellEditEvent = {
    row: updatedRow,
    rowIndex,
    columnId: id,
    prop: id,
    oldValue,
    newValue: validation.value,
    action: "update",
  };
  if (onbeforeedit && onbeforeedit(evt) === false) return true;
  const rows = source.slice();
  rows[sourceIndex] = updatedRow;
  updateRows(rows, "edit", [updatedRow]);
  oncelledit?.(evt);
  return true;
}

function updateRows(
  rows: GridRow[],
  action: GridRowsChangeEvent["action"],
  changedRows: GridRow[],
) {
  source = rows;
  onrowschange?.({ rows, changedRows, action });
}

function updateDisplayedRows(
  updatedDisplayRows: GridRow[],
  action: GridRowsChangeEvent["action"],
  changedStart: number,
  changedEnd: number,
) {
  const rows = safeSource.slice();
  const sourceIndexes = new Map(
    safeSource.map((row, index) => [row, index] as const),
  );
  const existingCount = Math.min(displayRows.length, updatedDisplayRows.length);
  for (let index = 0; index < existingCount; index++) {
    const sourceIndex = sourceIndexes.get(displayRows[index]!);
    if (
      sourceIndex !== undefined &&
      updatedDisplayRows[index] !== displayRows[index]
    ) {
      rows[sourceIndex] = updatedDisplayRows[index]!;
    }
  }
  if (updatedDisplayRows.length > displayRows.length) {
    rows.push(...updatedDisplayRows.slice(displayRows.length));
  }
  updateRows(
    rows,
    action,
    updatedDisplayRows.slice(
      changedStart,
      Math.min(changedEnd, updatedDisplayRows.length),
    ),
  );
}

function deleteSelectedRows() {
  if (readonly || displayRows.length === 0) return;
  const selectedRows = displayRows.filter((row, index) =>
    isGridRowSelected(row, index),
  );
  let removedRows: Set<GridRow>;
  if (selectedRows.length > 0) {
    removedRows = new Set(selectedRows);
  } else {
    const activeRow = menu?.row ?? selection.active?.row ?? 0;
    const selected = normalizeRange(currentRange());
    const hasMultiRowRange =
      selection.range && selected.startRow !== selected.endRow;
    removedRows = new Set(
      hasMultiRowRange
        ? displayRows.slice(selected.startRow, selected.endRow + 1)
        : displayRows.slice(activeRow, activeRow + 1),
    );
  }
  if (removedRows.size === 0) return;

  const rows = safeSource.filter((row) => !removedRows.has(row));
  if (rows.length === safeSource.length) return;
  updateRows(rows, "delete-rows", [...removedRows]);
  selection = emptySelection();
  onselection?.(selection);
}

function isCheckboxTarget(target: EventTarget | null): boolean {
  return target instanceof Element && Boolean(target.closest('[data-slot="checkbox"]'));
}

function startEdit(row: number, col: number, seed?: string) {
  const column = visibleCols[col];
  if (!column || readonly || column.readonly) return;
  const type = column.type;
  if (
    type === "boolean" ||
    column.editor === "checkbox" ||
    column.id === "__selected"
  ) {
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
  editing = {
    row,
    col,
    draft: seed ?? (value == null ? "" : String(value)),
  };
  requestAnimationFrame(() => editorEl?.focus());
}

function commitEdit(moveRow = 0, moveCol = 0) {
  if (!editing) return;
  const column = visibleCols[editing.col];
  if (column && !commitOverlay(editing.row, column, editing.draft)) return;
  const r = editing.row;
  const c = editing.col;
  editing = null;
  root?.focus();
  if (moveRow || moveCol) moveActiveAfterEdit(r, c, moveRow, moveCol);
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
  const selected = normalizeRange(currentRange());
  const rows = clearRange(displayRows, visibleCols, selected);
  updateDisplayedRows(
    rows,
    "cut",
    selected.startRow,
    Math.min(selected.endRow + 1, rows.length),
  );
}

function pasteText(text: string) {
  if (!clipboard || readonly) return;
  const matrix = parseClipboard(text);
  const start = normalizeRange(currentRange());
  const { rows, result } = applyPaste({
    rows: displayRows,
    columns: visibleCols,
    startRow: start.startRow,
    startCol: start.startCol,
    matrix,
    createEmptyRow: createEmptyGridRow,
  });
  updateDisplayedRows(
    rows,
    "paste",
    start.startRow,
    Math.min(start.startRow + matrix.length, rows.length),
  );
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
  if (isCheckboxTarget(e.target)) return;
  if ((e.target as HTMLElement | null)?.closest(".orbis-action-btn")) return;
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

function handleRootKeydown(event: KeyboardEvent) {
  if (
    event.target instanceof Element &&
    event.target.closest(".orbis-toolbar, .orbis-filter-panel")
  ) {
    return;
  }
  handleKeydown(event);
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
      if (!readonly) {
        const selected = normalizeRange(currentRange());
        const rows = clearRange(displayRows, visibleCols, selected);
        updateDisplayedRows(
          rows,
          "delete",
          selected.startRow,
          Math.min(selected.endRow + 1, rows.length),
        );
      }
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
      if (!readonly) {
        const selected = normalizeRange(currentRange());
        const rows = fillDown(displayRows, visibleCols, selected);
        updateDisplayedRows(
          rows,
          "fill-down",
          selected.startRow,
          Math.min(selected.endRow + 1, rows.length),
        );
      }
      break;
    case "fillRight":
      if (!readonly) {
        const selected = normalizeRange(currentRange());
        const rows = fillRight(displayRows, visibleCols, selected);
        updateDisplayedRows(
          rows,
          "fill-right",
          selected.startRow,
          Math.min(selected.endRow + 1, rows.length),
        );
      }
      break;
    default:
      break;
  }
}

function onHeaderClick(col: OrbisColumnDef, e: MouseEvent) {
  if (isCheckboxTarget(e.target)) return;
  sortColumn(col, e.shiftKey);
}

function sortColumn(col: OrbisColumnDef, shiftKey = false) {
  if (!isColumnSortable(col)) return;
  sorts = toggleSort(sorts, columnId(col), shiftKey);
  notifyQueryChange();
}

function onContext(row: number, col: number, e: MouseEvent) {
  e.preventDefault();
  setActive(row, col, e.shiftKey);
  const hasSelection =
    selectedCount(selection, displayRows.length) > 1 ||
    (selection.range &&
      normalizeRange(selection.range).endRow !==
        normalizeRange(selection.range).startRow);
  const items = contextMenuItems({
    context: "cell",
    hasSelection: !!hasSelection,
    canEdit: !readonly,
    filtered: (shouldRunClientQuery || usesServerQuery) && Boolean(filter),
  }).filter(
    (item) =>
      isColumnSortable(visibleCols[col] ?? {}) ||
      (item.id !== "sort-asc" && item.id !== "sort-desc"),
  );
  menu = {
    x: e.clientX,
    y: e.clientY,
    row,
    col,
    items,
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
      if (!readonly) {
        const selected = normalizeRange(currentRange());
        const rows = clearRange(displayRows, visibleCols, selected);
        updateDisplayedRows(
          rows,
          "delete",
          selected.startRow,
          Math.min(selected.endRow + 1, rows.length),
        );
      }
      break;
    case "delete-row":
      deleteSelectedRows();
      break;
    case "duplicate":
      if (!readonly) {
        const row = displayRows[active.row];
        const sourceIndex = row ? safeSource.indexOf(row) : -1;
        if (sourceIndex >= 0) {
          const rows = duplicateRows(safeSource, [sourceIndex], () =>
            crypto.randomUUID(),
          );
          const originalRows = new Set(safeSource);
          const addedRows = rows.filter((item) => !originalRows.has(item));
          updateRows(rows, "duplicate", addedRows);
        }
      }
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
      if (col) {
        sorts = [{ id: columnId(col), dir: "asc" }];
        notifyQueryChange();
      }
      break;
    case "sort-desc":
      if (col) {
        sorts = [{ id: columnId(col), dir: "desc" }];
        notifyQueryChange();
      }
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

function onResizeDblClick(col: OrbisColumnDef, e: MouseEvent) {
  e.preventDefault();
  e.stopPropagation();
  const id = columnId(col);
  const headerStr = columnHeader(col);
  let maxChars = headerStr.length;
  const sample = displayRows.slice(0, 100);
  for (const row of sample) {
    const val = getCellValue(row, col);
    if (val != null) {
      const str = String(val);
      if (str.length > maxChars) maxChars = str.length;
    }
  }
  const fitted = Math.min(
    col.maxWidth ?? 600,
    Math.max(col.minWidth ?? 60, Math.round(maxChars * 9 + 40)),
  );
  columnWidths = {
    ...columnWidths,
    [id]: fitted,
  };
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

function getBadgeVariant(
  val: unknown,
): "lime" | "destructive" | "secondary" | "outline" {
  const s = String(val ?? "")
    .toLowerCase()
    .trim();
  if (
    [
      "active",
      "present",
      "paid",
      "approved",
      "open",
      "success",
      "ناجح",
      "حاضر",
      "نشط",
      "مدفوع",
      "passing",
      "true",
      "1",
    ].includes(s)
  )
    return "lime";
  if (
    [
      "inactive",
      "absent",
      "overdue",
      "failed",
      "closed",
      "locked",
      "withdrawn",
      "error",
      "راسب",
      "غائب",
      "غير نشط",
      "معلق",
      "false",
      "0",
    ].includes(s)
  )
    return "destructive";
  if (
    [
      "pending",
      "late",
      "partial",
      "warning",
      "متأخر",
      "جزئي",
      "قيد الانتظار",
    ].includes(s)
  )
    return "secondary";
  return "outline";
}

function getInitials(name: string): string {
  if (!name) return "";
  const parts = name.trim().split(/\s+/);
  if (parts.length === 1) return parts[0]!.slice(0, 2).toUpperCase();
  const first = parts[0]?.slice(0, 1) || "";
  const last = parts[parts.length - 1]?.slice(0, 1) || "";
  return (first + last).toUpperCase();
}

function getAvatarSrc(row: GridRow): string {
  return String(
    row.avatar_url || row.avatar || row.photo_url || row.image || "",
  );
}

function isIndexColumn(col: OrbisColumnDef): boolean {
  const id = columnId(col);
  return id === "__idx" || id === "__rowIndex";
}

function isCheckboxColumn(col: OrbisColumnDef): boolean {
  const id = columnId(col);
  return (
    id === "__selected" || col.type === "boolean" || col.editor === "checkbox"
  );
}

function isBadgeColumn(col: OrbisColumnDef, row?: GridRow): boolean {
  const id = columnId(col);
  if (col.type === "badge" || col.type === "status") return true;
  if (
    id === "status" ||
    id === "payment_status" ||
    id === "attendance" ||
    id === "state" ||
    id === "role" ||
    id === "grade_letter"
  )
    return true;
  if (row) {
    const val = getCellValue(row, col);
    if (
      typeof val === "string" &&
      [
        "active",
        "inactive",
        "present",
        "absent",
        "late",
        "excused",
        "paid",
        "pending",
        "overdue",
        "partial",
        "draft",
        "posted",
        "unposted",
        "approved",
        "rejected",
        "success",
        "failed",
        "warning",
        "open",
        "closed",
        "ناجح",
        "راسب",
        "حاضر",
        "غائب",
        "متأخر",
        "معذور",
        "نشط",
        "غير نشط",
        "مدفوع",
        "معلق",
        "جزئي",
        "مسودة",
        "مرحل",
        "ملغى",
        "مقبول",
        "مرفوض",
      ].includes(val.toLowerCase().trim())
    ) {
      return true;
    }
  }
  return false;
}

function isAvatarColumn(col: OrbisColumnDef): boolean {
  const id = columnId(col);
  if (col.type === "avatar") return true;
  return (
    id === "student_name" ||
    id === "full_name" ||
    id === "user" ||
    id === "user_name" ||
    id === "student" ||
    id === "teacher"
  );
}

function isActionColumn(col: OrbisColumnDef): boolean {
  return (
    col.type === "action" ||
    (Array.isArray(col.meta?.items) && col.meta.items.length > 0)
  );
}

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
  const row = { ...source[rowIndex]!, [colId]: val };
  const rows = source.slice();
  rows[rowIndex] = row;
  source = rows;
  onrowschange?.({ rows, changedRows: [row], action: "edit" });
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
  onkeydown={handleRootKeydown}
  onfocus={() => measure()}
  {@attach (el) => {
    const node = el as HTMLDivElement;
    root = node;
    measure();
    notifyQueryChange();
    requestAnimationFrame(() => requestMoreIfNeeded());
    const ro = new ResizeObserver(measure);
    ro.observe(node);
    return () => {
      ro.disconnect();
      if (root === node) root = null;
    };
  }}
>
  {#if toolbar}
    <div
      class="orbis-toolbar"
      role="toolbar"
      aria-label={rtl ? "أدوات الجدول" : "Grid tools"}
    >
      {#if showSearch}
        <label class="orbis-search">
          <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
            <circle cx="8.75" cy="8.75" r="5.75" stroke="currentColor" stroke-width="1.5" />
            <path d="m13 13 4 4" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
          </svg>
          <input
            type="text"
            value={search}
            placeholder={searchPlaceholder ?? (rtl ? "بحث في السجلات…" : "Search records…")}
            aria-label={rtl ? "بحث في جميع الأعمدة" : "Search all columns"}
            oninput={handleSearchInput}
            onkeydown={(event) => {
              if (event.key === "Enter") event.preventDefault();
              event.stopPropagation();
            }}
          />
          {#if search}
            <button
              type="button"
              class="orbis-toolbar-clear"
              aria-label={rtl ? "مسح البحث" : "Clear search"}
              onclick={clearSearch}
            >×</button>
          {/if}
        </label>
      {/if}
      {#if showFilterButton}
        <button
          type="button"
          class="orbis-toolbar-button"
          class:active={filterPanelOpen || filterCount > 0}
          disabled={filterableCols.length === 0 && filterCount === 0}
          aria-expanded={filterPanelOpen}
          onclick={() => (filterPanelOpen = true)}
        >
          <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
            <path d="M3 5h14M5.5 10h9M8 15h4" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
          </svg>
          {rtl ? "تصفية" : "Filter"}
          {#if filterCount > 0}
            <span class="orbis-filter-count">{filterCount}</span>
          {/if}
        </button>
      {/if}
      {#if !readonly}
        <button
          type="button"
          class="orbis-toolbar-button orbis-add-row"
          disabled={loading}
          onclick={addRowFromToolbar}
        >
          <span aria-hidden="true">+</span>
          {rtl ? "إضافة صف" : "Add row"}
        </button>
      {/if}
      <span class="orbis-row-count" aria-live="polite">
        {displayRows.length} {rtl ? "صف" : "rows"}
      </span>
      {#if showFilterButton}
        <Dialog bind:open={filterPanelOpen}>
          <DialogContent class="flex max-h-[80vh] max-w-3xl flex-col overflow-hidden p-0 text-start">
            <DialogHeader class="border-b border-border/60 px-5 py-4">
              <DialogTitle>{rtl ? "تصفية البيانات" : "Filter records"}</DialogTitle>
            </DialogHeader>
            <div class="orbis-filter-panel">
              <div class="orbis-filter-builder">
                <select
                  class="orbis-filter-control"
                  bind:value={filterColumn}
                  aria-label={rtl ? "عمود التصفية" : "Filter column"}
                  onkeydown={(event) => event.stopPropagation()}
                >
                  {#each filterableCols as col (columnId(col))}
                    <option value={columnId(col)}>{columnHeader(col)}</option>
                  {/each}
                </select>
                <select
                  class="orbis-filter-control"
                  bind:value={filterOperator}
                  aria-label={rtl ? "نوع التصفية" : "Filter operator"}
                  onkeydown={(event) => event.stopPropagation()}
                >
                  {#each filterOperators as operator (operator.value)}
                    <option value={operator.value}>{operatorLabel(operator.value)}</option>
                  {/each}
                </select>
                {#if filterOperatorRequiresValue(filterOperator) && filterDraftColumn?.options?.length}
                  <select
                    class="orbis-filter-control filter-value"
                    bind:value={filterValue}
                    aria-label={rtl ? "قيمة التصفية" : "Filter value"}
                    onkeydown={(event) => event.stopPropagation()}
                  >
                    <option value="">{rtl ? "اختر قيمة" : "Choose a value"}</option>
                    {#each filterDraftColumn.options as option (String(option.value))}
                      <option value={String(option.value)}>{option.label}</option>
                    {/each}
                  </select>
                {:else if filterOperatorRequiresValue(filterOperator)}
                  <input
                    class="orbis-filter-control filter-value"
                    type={filterInputType(filterDraftColumn)}
                    bind:value={filterValue}
                    placeholder={filterOperator === "in" || filterOperator === "not_in"
                      ? (rtl ? "قيم مفصولة بفواصل" : "Comma-separated values")
                      : rtl ? "القيمة" : "Value"}
                    aria-label={rtl ? "قيمة التصفية" : "Filter value"}
                    onkeydown={(event) => {
                      if (event.key === "Enter") {
                        event.preventDefault();
                        addFilterCondition();
                      }
                      event.stopPropagation();
                    }}
                  />
                  {#if filterOperator === "between"}
                    <input
                      class="orbis-filter-control filter-value"
                      type={filterInputType(filterDraftColumn)}
                      bind:value={filterEndValue}
                      placeholder={rtl ? "إلى" : "To"}
                      aria-label={rtl ? "نهاية النطاق" : "Range end"}
                      onkeydown={(event) => event.stopPropagation()}
                    />
                  {/if}
                {/if}
                <button
                  type="button"
                  class="orbis-toolbar-button filter-add"
                  disabled={!canAddFilterCondition()}
                  onclick={addFilterCondition}
                >{rtl ? "إضافة" : "Add filter"}</button>
              </div>
              {#if (filter?.conditions.length ?? 0) > 1}
                <label class="orbis-filter-logic">
                  {rtl ? "مطابقة" : "Match"}
                  <select
                    class="orbis-filter-control"
                    value={filter?.logic ?? "AND"}
                    onchange={(event) =>
                      setFilter({
                        ...(filter ?? { conditions: [] }),
                        logic: event.currentTarget.value === "OR" ? "OR" : "AND",
                      })}
                    onkeydown={(event) => event.stopPropagation()}
                  >
                    <option value="AND">{rtl ? "كل الشروط" : "All conditions"}</option>
                    <option value="OR">{rtl ? "أي شرط" : "Any condition"}</option>
                  </select>
                </label>
              {/if}
              {#if filter?.conditions.length}
                <div class="orbis-filter-chips">
                  {#each filter.conditions as condition, index (`${condition.field}:${index}`)}
                    <span class="orbis-filter-chip">
                      {filterConditionLabel(condition)}
                      <button
                        type="button"
                        aria-label={rtl ? "إزالة شرط التصفية" : "Remove filter"}
                        onclick={() => removeFilterCondition(index)}
                      >×</button>
                    </span>
                  {/each}
                </div>
              {/if}
            </div>
            <DialogFooter class="border-t border-border/60 px-5 py-4">
              <button
                type="button"
                class="orbis-toolbar-button"
                disabled={filterCount === 0}
                onclick={() => setFilter(null)}
              >{rtl ? "مسح عوامل التصفية" : "Clear filters"}</button>
              <button
                type="button"
                class="orbis-toolbar-button active"
                onclick={() => (filterPanelOpen = false)}
              >{rtl ? "تم" : "Done"}</button>
            </DialogFooter>
          </DialogContent>
        </Dialog>
      {/if}
    </div>
  {/if}

  {#if pasteBanner}
    <div class="orbis-paste-banner" role="status">
      <span class="ok">{pasteBanner.acceptedCount} accepted</span>
      {#if pasteBanner.rejectedCount}
        <span class="rej">{pasteBanner.rejectedCount} rejected</span>
      {/if}
      <button type="button" class="banner-dismiss" onclick={() => (pasteBanner = null)}>Dismiss</button>
    </div>
  {/if}

  {#if loading}
    <div class="orbis-skeleton-wrap">
      <div class="orbis-grid-head" role="row">
        {#each visibleCols as col (columnId(col))}
          <div class="orbis-hcell" style:width="{widthOf(col)}px">
            <div class="orbis-skeleton-bar" style:width="{Math.min(widthOf(col) * 0.7, 90)}px"></div>
          </div>
        {/each}
      </div>
      {#each Array(8) as _, i (i)}
        <div class="orbis-skeleton-row" style:opacity="{1 - i * 0.1}">
          {#each visibleCols as col, j (columnId(col) || j)}
            <div
              class="orbis-skeleton-bar"
              style:width="{Math.min(widthOf(col) * (j === 0 ? 0.4 : 0.75), 110)}px"
            ></div>
          {/each}
        </div>
      {/each}
    </div>
  {:else if displayRows.length === 0}
    {#if loadMoreError}
      <div class="orbis-empty-wrap" role="alert">
        <div class="orbis-empty-title">{rtl ? "تعذر تحميل السجلات" : "Couldn't load records"}</div>
        <div class="orbis-empty-desc">
          {loadMoreError instanceof Error ? loadMoreError.message : String(loadMoreError)}
        </div>
        <button type="button" class="orbis-load-retry" onclick={() => requestMoreIfNeeded(true)}>
          {rtl ? "إعادة المحاولة" : "Retry"}
        </button>
      </div>
    {:else}
      <div class="orbis-empty-wrap">
        <div class="orbis-empty-title">{emptyTitle}</div>
        {#if emptyDescription}
          <div class="orbis-empty-desc">{emptyDescription}</div>
        {/if}
      </div>
    {/if}
  {:else}
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
            {@const isSelectCol = id === "__selected"}
            <div
              class="orbis-hcell align-{defaultAlign(col)}"
              class:pinned={isColumnPinnedLeft(col)}
              role="columnheader"
              tabindex="0"
              aria-colindex={c + 1}
              aria-sort={sorts.find((s) => s.id === id)?.dir === "desc" ? "descending" : sorts.find((s) => s.id === id) ? "ascending" : "none"}
              style:width="{widthOf(col)}px"
              onclick={(e) => onHeaderClick(col, e)}
              onkeydown={(e) => {
                if (
                  (e.key === "Enter" || e.key === " ") &&
                  !isCheckboxTarget(e.target)
                ) {
                  e.preventDefault();
                  sortColumn(col, e.shiftKey);
                }
              }}
            >
              {#if Header}
                <Header row={{}} column={{ ...col, id }} />
              {:else if isSelectCol}
                {@const isAllSelected = displayRows.length > 0 && displayRows.every((row, idx) => isGridRowSelected(row, idx))}
                {@const isSomeSelected = !isAllSelected && displayRows.some((row, idx) => isGridRowSelected(row, idx))}
                <Checkbox
                  checked={isAllSelected}
                  indeterminate={isSomeSelected}
                  aria-label="Select all"
                  onclick={(e: MouseEvent) => {
                    e.stopPropagation();
                    const selected = !isAllSelected;
                    const selectedIds = displayRows.map((row, idx) =>
                      rowKey(row, idx),
                    );
                    if (!displayRows.some((row) => Object.hasOwn(row, "__selected"))) {
                      if (selected) {
                        selection = selectScope(selectedIds, "filtered", [], []);
                      } else {
                        selection = emptySelection();
                      }
                      onselection?.(selection);
                    }
                    onaction?.({
                      actionId: "select-all",
                      row: {},
                      rowIndex: -1,
                      prop: id,
                      originalEvent: e,
                      selected,
                      selectedIds,
                    });
                  }}
                />
              {:else}
                <span class="hlabel">{columnHeader(col)}</span>
              {/if}
              <button
                type="button"
                class="resize-handle"
                aria-label="Resize column"
                onpointerdown={(e) => onResizeStart(col, e)}
                ondblclick={(e) => onResizeDblClick(col, e)}
              ></button>
            </div>
          {/each}
        </div>
        <div class="orbis-window" style:transform="translateY({rowWindow.offset}px)">
          {#each { length: rowWindow.count } as _, i (rowWindow.start + i)}
            {@const r = rowWindow.start + i}
            {@const row = displayRows[r]}
            {#if row}
              <div
                class="orbis-row"
                class:selected={isGridRowSelected(row, r)}
                role="row"
                aria-rowindex={r + 1}
                aria-selected={isGridRowSelected(row, r)}
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
                    class:is-editing={isEdit}
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
                          class:invalid={!!editing.error}
                          bind:this={editorEl}
                          bind:value={editing.draft}
                          aria-invalid={!!editing.error}
                          title={editing.error}
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
                          {#each col.options as opt (String(opt.value))}
                            <option value={String(opt.value)}>{opt.label}</option>
                          {/each}
                        </select>
                      {:else}
                        <input
                          class="orbis-editor"
                          class:invalid={!!editing.error}
                          type={editorInputType(col)}
                          bind:this={editorEl}
                          bind:value={editing.draft}
                          aria-invalid={!!editing.error}
                          title={editing.error}
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
                    {:else if isIndexColumn(col)}
                      <span class="orbis-index-text">{getCellValue(row, col) ?? (r + 1)}</span>
                    {:else if isCheckboxColumn(col)}
                      {@const isChecked = id === "__selected" ? isGridRowSelected(row, r) : getCellValue(row, col) === true || getCellValue(row, col) === 1 || getCellValue(row, col) === "true"}
                      <Checkbox
                        checked={isChecked}
                        class="orbis-grid-checkbox"
                        aria-label={id === "__selected" ? "Select row" : columnHeader(col)}
                        onclick={(e: MouseEvent) => {
                          e.stopPropagation();
                          if (id === "__selected") {
                            const selected = !isChecked;
                            if (!Object.hasOwn(row, "__selected")) {
                              selection = toggleId(selection, rowKey(row, r));
                              onselection?.(selection);
                            }
                            onaction?.({
                              actionId: "toggle-select",
                              row,
                              rowIndex: r,
                              prop: id,
                              originalEvent: e,
                              selected,
                            });
                          } else {
                            commitOverlay(r, col, !isChecked);
                          }
                        }}
                      />
                    {:else if isAvatarColumn(col)}
                      {@const val = String(getCellValue(row, col) ?? "")}
                      {@const src = getAvatarSrc(row)}
                      <div class="orbis-avatar-cell">
                        <div class="orbis-avatar-circle">
                          {#if src}
                            <img
                              src={src}
                              alt={val}
                              class="orbis-avatar-img"
                              onerror={(event) => {
                                const image = event.currentTarget;
                                if (image instanceof HTMLImageElement) image.hidden = true;
                                const fallback = image.nextElementSibling;
                                if (fallback instanceof HTMLElement) fallback.hidden = false;
                              }}
                            />
                            <span class="orbis-avatar-fallback" hidden>{getInitials(val)}</span>
                          {:else}
                            <span class="orbis-avatar-fallback">{getInitials(val)}</span>
                          {/if}
                        </div>
                        <span class="orbis-avatar-text">{val}</span>
                      </div>
                    {:else if isBadgeColumn(col, row)}
                      {@const val = getCellValue(row, col)}
                      {@const formatted = formatCell(row, col, rtl ? "ar" : "en")}
                      <Badge variant={getBadgeVariant(val)} class="orbis-grid-badge">
                        {formatted || "—"}
                      </Badge>
                    {:else if Array.isArray(getCellValue(row, col))}
                      {@const arr = getCellValue(row, col) as unknown[]}
                      <div class="orbis-tags-group">
                        {#each arr as item, index (index)}
                          <Badge variant="outline" class="orbis-grid-badge">{typeof item === 'object' && item ? (item as any).name || (item as any).label || JSON.stringify(item) : String(item)}</Badge>
                        {/each}
                      </div>
                    {:else if isActionColumn(col)}
                      {@const items = (col.meta?.items as ActionItem[]) || []}
                      <div class="orbis-actions-group">
                        {#each items as action (action.id)}
                          {#if !action.visible || action.visible(row)}
                            <button
                              type="button"
                              class="orbis-action-btn orbis-action-{action.variant || 'ghost'}"
                              disabled={action.disabled?.(row)}
                              title={action.label || action.id}
                              onclick={(e) => {
                                e.stopPropagation();
                                onaction?.({ actionId: action.id, row, rowIndex: r, prop: id, originalEvent: e });
                              }}
                            >
                              {#if action.icon === "Pencil" || action.id === "edit"}
                                <svg class="orbis-action-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M17 3a2.85 2.83 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5Z"/></svg>
                              {:else if action.icon === "Clipboard" || action.id === "duplicate"}
                                <svg class="orbis-action-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect width="14" height="14" x="8" y="8" rx="2"/><path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"/></svg>
                              {:else if action.icon === "Trash2" || action.icon === "Trash" || action.id === "delete"}
                                <svg class="orbis-action-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M3 6h18m-2 0v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6m3 0V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"/></svg>
                              {:else if action.icon === "Eye" || action.id === "preview" || action.id === "view"}
                                <svg class="orbis-action-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z"/><circle cx="12" cy="12" r="3"/></svg>
                              {:else if action.icon === "Download" || action.id === "download"}
                                <svg class="orbis-action-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><path d="m7 10 5 5 5-5"/><path d="M12 15V3"/></svg>
                              {:else if action.icon === "Refresh" || action.id === "restore"}
                                <svg class="orbis-action-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M20 7v5h-5"/><path d="M4 17v-5h5"/><path d="M5.64 9A8 8 0 0 1 19 7l1 5"/><path d="M4 12l1 5a8 8 0 0 0 13.36-2"/></svg>
                              {:else if action.icon === "ArchiveBox" || action.id === "archive"}
                                <svg class="orbis-action-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect width="20" height="5" x="2" y="3" rx="1"/><path d="M4 8v11a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8"/><path d="M10 12h4"/></svg>
                              {:else if action.icon === "User" || action.icon === "UserCheck" || action.icon === "UserBadgeCheck" || action.id === "profile"}
                                <svg class="orbis-action-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2"/><circle cx="12" cy="7" r="4"/></svg>
                              {:else if action.icon === "Lock" || action.icon === "LockClosed" || action.id === "reset_password"}
                                <svg class="orbis-action-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect width="18" height="11" x="3" y="11" rx="2" ry="2"/><path d="M7 11V7a5 5 0 0 1 10 0v4"/></svg>
                              {:else if action.icon === "Check" || action.icon === "CheckCircle" || action.id === "set_active"}
                                <svg class="orbis-action-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><path d="m9 11 3 3L22 4"/></svg>
                              {:else if action.icon === "Building" || action.icon === "BuildingOffice" || action.id === "set_on_leave"}
                                <svg class="orbis-action-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M6 22V4a2 2 0 0 1 2-2h8a2 2 0 0 1 2 2v18Z"/><path d="M6 12H4a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2h2"/><path d="M18 9h2a2 2 0 0 1 2 2v9a2 2 0 0 1-2 2h-2"/><path d="M10 6h4"/><path d="M10 10h4"/><path d="M10 14h4"/><path d="M10 18h4"/></svg>
                              {:else if action.icon === "Shield" || action.icon === "ShieldAlert" || action.icon === "ShieldCheck" || action.id === "set_suspended"}
                                <svg class="orbis-action-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z"/></svg>
                              {:else if action.icon === "MoreHorizontal" || action.icon === "Ellipsis"}
                                <svg class="orbis-action-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="1"/><circle cx="19" cy="12" r="1"/><circle cx="5" cy="12" r="1"/></svg>
                              {:else}
                                <svg class="orbis-action-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="1"/><circle cx="19" cy="12" r="1"/><circle cx="5" cy="12" r="1"/></svg>
                              {/if}
                            </button>
                          {/if}
                        {/each}
                      </div>
                    {:else}
                      <span class="orbis-text">{formatCell(row, col, rtl ? "ar" : "en")}</span>
                    {/if}
                  </div>
                {/each}
              </div>
            {/if}
          {/each}
        </div>
        {#if isLoadingMore || loadMoreError}
          <div class="orbis-load-status" role={loadMoreError ? "alert" : "status"}>
            {#if loadMoreError}
              <span>
                {rtl ? "تعذر تحميل المزيد" : "Couldn't load more"}
                {loadMoreError instanceof Error ? `: ${loadMoreError.message}` : ""}
              </span>
              <button type="button" class="orbis-load-retry" onclick={() => requestMoreIfNeeded(true)}>
                {rtl ? "إعادة المحاولة" : "Retry"}
              </button>
            {:else}
              {rtl ? "جارٍ تحميل المزيد…" : "Loading more…"}
            {/if}
          </div>
        {/if}
      </div>
    </div>
  {/if}
</div>

{#if menu}
  <ul
    class="orbis-menu"
    style:left="{menu.x}px"
    style:top="{menu.y}px"
    role="menu"
  >
    {#each menu.items as item (item.id)}
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
    min-width: 0;
    min-height: 0;
    background: var(--card);
    color: var(--card-foreground);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    overflow: hidden;
    font-family: var(--font-sans, "Rubik", system-ui, sans-serif);
    font-size: 0.75rem;
    line-height: 1.25;
    outline: none;
  }
  .orbis-grid:focus-visible {
    outline: 1px solid var(--ring);
    outline-offset: -2px;
  }
  .orbis-grid.is-selecting {
    user-select: none;
    cursor: cell;
  }
  .orbis-toolbar {
    display: flex;
    align-items: center;
    flex-wrap: wrap;
    gap: 0.375rem;
    padding: 0.375rem 0.5rem;
    min-height: 2.5rem;
    flex: 0 0 auto;
    min-width: 0;
    border-bottom: 1px solid var(--border);
    background: var(--card);
  }
  .orbis-search {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    flex: 1 1 12rem;
    max-width: 20rem;
    min-width: 8rem;
    height: 2rem;
    padding-inline: 0.5rem;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--card);
    color: var(--muted-foreground);
  }
  .orbis-search:focus-within {
    border-color: var(--ring);
    box-shadow: inset 0 0 0 1px var(--ring);
  }
  .orbis-search > svg,
  .orbis-toolbar-button > svg {
    flex: 0 0 auto;
    width: 0.875rem;
    height: 0.875rem;
  }
  .orbis-search input {
    width: 100%;
    min-width: 0;
    border: 0;
    border-radius: 0;
    padding: 0;
    outline: 0;
    background: transparent;
    box-shadow: none;
    color: var(--foreground);
    font: inherit;
    font-size: 0.75rem;
    appearance: none;
  }
  .orbis-search input::placeholder {
    color: var(--muted-foreground);
  }
  .orbis-toolbar-clear,
  .orbis-filter-chip button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 1.25rem;
    height: 1.25rem;
    border: 0;
    border-radius: var(--radius-sm);
    background: transparent;
    color: var(--muted-foreground);
    font: inherit;
    font-size: 1rem;
    line-height: 1;
    cursor: pointer;
  }
  .orbis-toolbar-clear:hover,
  .orbis-filter-chip button:hover {
    background: var(--accent);
    color: var(--foreground);
  }
  .orbis-toolbar-button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 0.375rem;
    min-height: 2rem;
    padding: 0 0.625rem;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--card);
    color: var(--foreground);
    font: inherit;
    font-size: 0.75rem;
    white-space: nowrap;
    cursor: pointer;
  }
  .orbis-toolbar-button:hover,
  .orbis-toolbar-button.active {
    background: var(--muted);
  }
  .orbis-toolbar-button:focus-visible,
  .orbis-filter-chip button:focus-visible {
    outline: 2px solid var(--ring);
    outline-offset: 0;
  }
  .orbis-toolbar-button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  .orbis-add-row > span {
    color: var(--primary);
    font-size: 1rem;
    font-weight: 600;
  }
  .orbis-filter-count {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-width: 1rem;
    height: 1rem;
    padding: 0 0.2rem;
    border-radius: var(--radius-sm);
    background: var(--primary);
    color: var(--primary-foreground);
    font-size: 0.625rem;
    line-height: 1;
  }
  .orbis-row-count {
    margin-inline-start: auto;
    padding-inline: 0.375rem;
    color: var(--muted-foreground);
    font-size: 0.6875rem;
    white-space: nowrap;
  }
  .orbis-filter-panel {
    display: flex;
    flex-direction: column;
    flex: 1 1 auto;
    align-items: stretch;
    gap: 0.75rem;
    width: 100%;
    min-width: 0;
    overflow-y: auto;
    padding: 1rem 1.25rem 0;
  }
  .orbis-filter-builder {
    display: flex;
    flex: 1 1 auto;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.375rem;
    width: 100%;
    min-width: 0;
  }
  .orbis-filter-control {
    min-width: 7rem;
    max-width: 100%;
    height: 2rem;
    padding: 0 0.5rem;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--card);
    color: var(--foreground);
    font: inherit;
    font-size: 0.6875rem;
  }
  .orbis-filter-control:focus {
    outline: none;
    border-color: var(--ring);
    box-shadow: inset 0 0 0 1px var(--ring);
  }
  .orbis-filter-control.filter-value {
    flex: 1 1 8rem;
  }
  .orbis-filter-logic {
    display: inline-flex;
    align-items: center;
    gap: 0.375rem;
    color: var(--muted-foreground);
    font-size: 0.6875rem;
    white-space: nowrap;
  }
  .orbis-filter-logic .orbis-filter-control {
    min-width: 6rem;
  }
  .orbis-filter-chips {
    display: flex;
    flex: 1 1 100%;
    flex-wrap: wrap;
    gap: 0.375rem;
  }
  .orbis-filter-chip {
    display: inline-flex;
    align-items: center;
    gap: 0.25rem;
    max-width: 100%;
    padding: 0.125rem 0.25rem 0.125rem 0.5rem;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--muted);
    color: var(--foreground);
    font-size: 0.6875rem;
  }
  .orbis-filter-chip button {
    flex: 0 0 auto;
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
    color: var(--foreground);
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
  .orbis-hcell.align-end {
    justify-content: flex-end;
    padding-inline-end: 0.875rem;
    text-align: end;
    font-variant-numeric: tabular-nums;
  }
  .orbis-hcell.align-center {
    justify-content: center;
    text-align: center;
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
    min-width: 0;
    overflow: auto;
    position: relative;
    background: var(--card);
    overscroll-behavior: contain;
    scrollbar-gutter: stable;
    scrollbar-width: thin;
    scrollbar-color: var(--border) var(--card);
    scroll-behavior: auto;
  }
  .orbis-grid-body::-webkit-scrollbar {
    width: 0.625rem;
    height: 0.625rem;
  }
  .orbis-grid-body::-webkit-scrollbar-track {
    background: var(--card);
  }
  .orbis-grid-body::-webkit-scrollbar-thumb {
    border: 2px solid var(--card);
    border-radius: var(--radius-sm);
    background: var(--border);
  }
  .orbis-grid-body::-webkit-scrollbar-thumb:hover {
    background: var(--muted-foreground);
  }
  .orbis-grid-body::-webkit-scrollbar-corner {
    background: var(--card);
  }
  .orbis-load-status {
    position: sticky;
    bottom: 0;
    z-index: 20;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.75rem;
    min-height: 36px;
    padding: 0.4rem 0.75rem;
    border-top: 1px solid var(--border);
    background: color-mix(in oklab, var(--card) 94%, transparent);
    color: var(--muted-foreground);
    font-size: 0.75rem;
  }
  .orbis-load-retry {
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--card);
    padding: 0.25rem 0.5rem;
    color: var(--foreground);
    cursor: pointer;
  }
  .orbis-spacer {
    position: relative;
    min-width: 100%;
    display: flex;
    flex-direction: column;
  }
  .orbis-window {
    contain: layout style;
  }
  .orbis-row {
    display: flex;
    height: var(--row-h);
    border-bottom: 1px solid var(--border);
    background: var(--card);
    contain: layout style;
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
  .orbis-cell.is-editing {
    padding: 0;
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
  .orbis-index-text {
    font-family: var(--font-mono, monospace);
    color: var(--muted-foreground);
  }
  .orbis-editor {
    width: 100%;
    height: 100%;
    border: 0;
    box-sizing: border-box;
    background: transparent;
    color: var(--foreground);
    font: inherit;
    outline: none;
    border-radius: 0;
    padding: 0 0.625rem;
  }
  .orbis-editor.invalid {
    color: var(--destructive);
  }
  .orbis-cell.is-editing.is-error {
    box-shadow: inset 0 0 0 1px var(--destructive);
  }

  .orbis-grid-badge {
    display: inline-flex;
    align-items: center;
    text-transform: capitalize;
    font-size: 0.6875rem;
    line-height: 1;
    padding: 0.125rem 0.375rem;
  }
  .orbis-tags-group {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    overflow: hidden;
    max-width: 100%;
  }

  /* Avatar Cell */
  .orbis-avatar-cell {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    width: 100%;
    min-width: 0;
  }
  .orbis-avatar-circle {
    width: 1.375rem;
    height: 1.375rem;
    border-radius: 9999px;
    background: var(--muted);
    color: var(--muted-foreground);
    display: flex;
    align-items: center;
    justify-content: center;
    overflow: hidden;
    flex-shrink: 0;
    font-size: 0.5625rem;
    font-weight: 700;
    border: 1px solid var(--border);
  }
  .orbis-avatar-img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }
  .orbis-avatar-fallback {
    text-transform: uppercase;
  }
  .orbis-avatar-text {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-weight: 500;
    color: var(--foreground);
  }

  .orbis-grid-checkbox {
    cursor: pointer;
  }

  /* Actions Group */
  .orbis-actions-group {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    width: 100%;
    justify-content: flex-start;
  }
  .orbis-action-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 1.5rem;
    height: 1.5rem;
    padding: 0;
    border-radius: var(--radius-sm, 3px);
    border: 1px solid transparent;
    background: transparent;
    color: var(--muted-foreground);
    cursor: pointer;
    font-size: 0.6875rem;
    transition: all 0.15s ease;
  }
  .orbis-action-btn:hover:not(:disabled) {
    background: var(--accent);
    color: var(--accent-foreground);
  }
  .orbis-action-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }
  .orbis-action-btn.orbis-action-destructive:hover:not(:disabled) {
    background: color-mix(in oklab, var(--destructive) 15%, transparent);
    color: var(--destructive);
  }
  .orbis-action-btn.orbis-action-lime:hover:not(:disabled) {
    background: color-mix(in oklab, var(--brand-lime) 18%, transparent);
    color: var(--brand-lime-dark);
  }
  .orbis-action-icon {
    width: 0.8125rem;
    height: 0.8125rem;
  }

  /* Skeleton Loading State */
  .orbis-skeleton-wrap {
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100%;
    overflow: hidden;
  }
  .orbis-skeleton-row {
    display: flex;
    align-items: center;
    height: var(--row-h, 36px);
    border-bottom: 1px solid var(--border);
    padding: 0 0.75rem;
    gap: 1rem;
  }
  .orbis-skeleton-bar {
    height: 0.625rem;
    border-radius: var(--radius-sm, 4px);
    background: linear-gradient(90deg, var(--muted) 25%, color-mix(in oklab, var(--accent) 70%, var(--muted)) 50%, var(--muted) 75%);
    background-size: 200% 100%;
    animation: orbis-shimmer 1.8s infinite;
  }
  @keyframes orbis-shimmer {
    0% { background-position: 200% 0; }
    100% { background-position: -200% 0; }
  }

  /* Empty State */
  .orbis-empty-wrap {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    width: 100%;
    height: 100%;
    padding: 2rem;
    gap: 0.375rem;
    color: var(--muted-foreground);
    text-align: center;
  }
  .orbis-empty-title {
    font-size: 0.875rem;
    font-weight: 600;
    color: var(--foreground);
  }
  .orbis-empty-desc {
    font-size: 0.75rem;
    max-width: 20rem;
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
