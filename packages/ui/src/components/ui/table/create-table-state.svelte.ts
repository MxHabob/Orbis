// ── createTableState ────────────────────────────────────────────────
// The core table engine built on Svelte 5 Runes.
// Manages sorting, filtering, column visibility, sizing, ordering,
// row selection, and computed row processing — all reactively.
//
// Design: Each feature is opt-in via TableOptions flags.
// The hook computes a final `processedRows` derived that chains:
//   data → filter → sort → (pagination-ready)

import type {
  ColumnDef,
  SortingState,
  TableOptions,
  CellContext,
  SortDirection,
} from "./table-types.js";

// ── Helpers ─────────────────────────────────────────────────────────

function getCellValue<T>(row: T, column: ColumnDef<T>): unknown {
  if (column.accessorFn) return column.accessorFn(row);
  if (column.accessorKey)
    return (row as Record<string, unknown>)[column.accessorKey];
  return undefined;
}

function defaultCompare(a: unknown, b: unknown): number {
  if (a == null && b == null) return 0;
  if (a == null) return -1;
  if (b == null) return 1;

  if (typeof a === "number" && typeof b === "number") return a - b;
  if (a instanceof Date && b instanceof Date) return a.getTime() - b.getTime();

  return String(a).localeCompare(String(b), undefined, {
    numeric: true,
    sensitivity: "base",
  });
}

// ── Main Factory ────────────────────────────────────────────────────

/**
 * Creates a fully-reactive table state using Svelte 5 Runes.
 *
 * @example
 * ```svelte
 * <script lang="ts">
import { AdjustmentsHorizontal } from "@orbis/icons";

 * import { createTableState } from '@orbis/ui-svelte/table';
 *
 * const table = createTableState({
 *   data: students,
 *   columns: studentColumns,
 *   getRowId: (s) => s.id,
 *   enableSorting: true,
 *   enableRowSelection: true,
 * });
 * </script>
 * ```
 */
export function createTableState<T>(options: TableOptions<T>) {
  // ── Source Data ─────────────────────────────────────────────────
  let data = $state<T[]>(options.data);
  let columns = $state<ColumnDef<T>[]>(options.columns);

  // ── Sorting State ──────────────────────────────────────────────
  let sorting = $state<SortingState>(
    options.initialSorting ?? { column: null, direction: "asc" },
  );

  // ── AdjustmentsHorizontal State ───────────────────────────────────────────────
  let globalFilter = $state("");
  let columnFilters = $state<Record<string, string>>({});

  // ── Column Visibility ──────────────────────────────────────────
  let columnVisibility = $state<Record<string, boolean>>(
    options.initialColumnVisibility ??
      Object.fromEntries(
        options.columns.map((c) => [c.id, c.visible !== false]),
      ),
  );

  // ── Column Sizing ──────────────────────────────────────────────
  let columnSizing = $state<Record<string, number>>(
    options.initialColumnSizing ??
      Object.fromEntries(
        options.columns
          .filter((c) => c.size != null)
          .map((c) => [c.id, c.size!]),
      ),
  );

  // ── Column Order ───────────────────────────────────────────────
  let columnOrder = $state<string[]>(
    options.initialColumnOrder ?? options.columns.map((c) => c.id),
  );

  // ── Row Selection ──────────────────────────────────────────────
  let selectedIds = $state<Set<string>>(new Set());

  // ── Derived: Visible Columns (ordered) ─────────────────────────

  const visibleColumns = $derived.by(() => {
    const orderMap = new Map(columnOrder.map((id, i) => [id, i]));

    return [...columns]
      .filter((col) => columnVisibility[col.id] !== false)
      .sort((a, b) => {
        const ia = orderMap.get(a.id) ?? Infinity;
        const ib = orderMap.get(b.id) ?? Infinity;
        return ia - ib;
      });
  });

  // ── Derived: Filtered Data ─────────────────────────────────────

  const filteredData = $derived.by(() => {
    if (
      options.enableFiltering === false &&
      !globalFilter &&
      Object.keys(columnFilters).length === 0
    ) {
      return data;
    }

    const query = globalFilter.toLowerCase().trim();

    return data.filter((row) => {
      // Global filter: search across all visible text columns
      if (query) {
        const match = visibleColumns.some((col) => {
          const value = getCellValue(row, col);
          return value != null && String(value).toLowerCase().includes(query);
        });
        if (!match) return false;
      }

      // Per-column filters
      for (const [colId, filterValue] of Object.entries(columnFilters)) {
        if (!filterValue) continue;
        const col = columns.find((c) => c.id === colId);
        if (!col) continue;
        const value = getCellValue(row, col);
        if (
          value == null ||
          !String(value).toLowerCase().includes(filterValue.toLowerCase())
        ) {
          return false;
        }
      }

      return true;
    });
  });

  // ── Derived: Sorted Data ───────────────────────────────────────

  const sortedData = $derived.by(() => {
    if (!sorting.column || options.enableSorting === false) {
      return filteredData;
    }

    const col = columns.find((c) => c.id === sorting.column);
    if (!col) return filteredData;

    const compareFn =
      col.sortFn ??
      ((a: T, b: T) => {
        const va = getCellValue(a, col);
        const vb = getCellValue(b, col);
        return defaultCompare(va, vb);
      });

    const direction = sorting.direction === "desc" ? -1 : 1;

    return [...filteredData].sort((a, b) => compareFn(a, b) * direction);
  });

  // ── Derived: Final Processed Rows ──────────────────────────────

  const processedRows = $derived(sortedData);

  // ── Derived: Row Count ─────────────────────────────────────────

  const rowCount = $derived(processedRows.length);
  const totalCount = $derived(data.length);
  const selectedCount = $derived(selectedIds.size);
  const isAllSelected = $derived(rowCount > 0 && selectedIds.size === rowCount);

  // ── Actions ────────────────────────────────────────────────────

  function toggleSort(columnId: string) {
    const col = columns.find((c) => c.id === columnId);
    if (!col?.sortable) return;

    if (sorting.column === columnId) {
      if (sorting.direction === "asc") {
        sorting = { column: columnId, direction: "desc" };
      } else {
        // Third click clears sort
        sorting = { column: null, direction: "asc" };
      }
    } else {
      sorting = { column: columnId, direction: "asc" };
    }

    options.onSortingChange?.(sorting);
  }

  function toggleColumnVisibility(columnId: string) {
    const current = columnVisibility[columnId];
    columnVisibility = {
      ...columnVisibility,
      [columnId]: current === false,
    };
  }

  function setColumnSize(columnId: string, size: number) {
    const col = columns.find((c) => c.id === columnId);
    if (!col || col.enableResizing === false) return;

    const min = col.minSize ?? 50;
    const max = col.maxSize ?? 1000;
    const clamped = Math.max(min, Math.min(max, size));

    columnSizing = { ...columnSizing, [columnId]: clamped };
  }

  function reorderColumn(fromId: string, toId: string) {
    const fromIndex = columnOrder.indexOf(fromId);
    const toIndex = columnOrder.indexOf(toId);
    if (fromIndex === -1 || toIndex === -1) return;

    const config = options.config;
    if (config?.nonReorderableColumns.has(fromId)) return;

    const next = [...columnOrder];
    next.splice(fromIndex, 1);
    next.splice(toIndex, 0, fromId);
    columnOrder = next;
  }

  function getColumnSize(columnId: string): number {
    return (
      columnSizing[columnId] ??
      columns.find((c) => c.id === columnId)?.size ??
      150
    );
  }

  // ── Row Selection ──────────────────────────────────────────────

  function getRowId(row: T, index: number): string {
    if (options.getRowId) return options.getRowId(row);
    return String(index);
  }

  function toggleRowSelection(rowId: string) {
    const next = new Set(selectedIds);
    if (next.has(rowId)) {
      next.delete(rowId);
    } else {
      next.add(rowId);
    }
    selectedIds = next;
    options.onSelectionChange?.(selectedIds);
  }

  function toggleAllSelection() {
    if (isAllSelected) {
      selectedIds = new Set();
    } else {
      const all = new Set<string>();
      processedRows.forEach((row, i) => all.add(getRowId(row, i)));
      selectedIds = all;
    }
    options.onSelectionChange?.(selectedIds);
  }

  function clearSelection() {
    selectedIds = new Set();
    options.onSelectionChange?.(selectedIds);
  }

  function isRowSelected(rowId: string): boolean {
    return selectedIds.has(rowId);
  }

  // ── Cell Context Builder ───────────────────────────────────────

  function getCellContext(
    row: T,
    column: ColumnDef<T>,
    rowIndex: number,
  ): CellContext<T> {
    return {
      value: getCellValue(row, column),
      row,
      rowIndex,
      column,
    };
  }

  // ── Public API ─────────────────────────────────────────────────

  return {
    // Reactive state (read)
    get data() {
      return data;
    },
    get columns() {
      return columns;
    },
    get visibleColumns() {
      return visibleColumns;
    },
    get processedRows() {
      return processedRows;
    },
    get sorting() {
      return sorting;
    },
    get globalFilter() {
      return globalFilter;
    },
    get columnVisibility() {
      return columnVisibility;
    },
    get columnSizing() {
      return columnSizing;
    },
    get columnOrder() {
      return columnOrder;
    },
    get selectedIds() {
      return selectedIds;
    },
    get rowCount() {
      return rowCount;
    },
    get totalCount() {
      return totalCount;
    },
    get selectedCount() {
      return selectedCount;
    },
    get isAllSelected() {
      return isAllSelected;
    },

    // Setters
    set data(value: T[]) {
      data = value;
    },
    set columns(value: ColumnDef<T>[]) {
      columns = value;
    },
    set globalFilter(value: string) {
      globalFilter = value;
    },
    set sorting(value: SortingState) {
      sorting = value;
    },
    set columnVisibility(value: Record<string, boolean>) {
      columnVisibility = value;
    },
    set columnSizing(value: Record<string, number>) {
      columnSizing = value;
    },
    set columnOrder(value: string[]) {
      columnOrder = value;
    },
    set columnFilters(value: Record<string, string>) {
      columnFilters = value;
    },

    // Actions
    toggleSort,
    toggleColumnVisibility,
    setColumnSize,
    reorderColumn,
    getColumnSize,
    getRowId,
    toggleRowSelection,
    toggleAllSelection,
    clearSelection,
    isRowSelected,
    getCellContext,
    getCellValue: (row: T, column: ColumnDef<T>) => getCellValue(row, column),
  };
}

/** Return type of createTableState */
export type TableState<T> = ReturnType<typeof createTableState<T>>;
