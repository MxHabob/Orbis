// ── createTablePersistence ──────────────────────────────────────────
// Persists table settings (column visibility, sizing, order, sorting)
// to localStorage for desktop apps. Debounced to avoid excessive writes.
// Ported from: packages/excel-grid/src/hooks/use-table-settings.ts

import type { TableSettings, SortingState } from "./table-types.js";

const STORAGE_KEY = "orbis-table-settings";

export interface TablePersistenceOptions {
  /** Unique table identifier (e.g. "students", "classes") */
  tableId: string;
  /** Initial settings loaded from storage (optional override) */
  initialSettings?: Partial<TableSettings>;
  /** Columns hidden by default */
  defaultHiddenColumns?: string[];
  /** Debounce delay in ms (default: 300) */
  debounceMs?: number;
}

export interface TablePersistenceState {
  /** Current column visibility */
  columnVisibility: Record<string, boolean>;
  /** Current column sizing */
  columnSizing: Record<string, number>;
  /** Current column order */
  columnOrder: string[];
  /** Current sorting state */
  sorting: SortingState;
  /** Manually trigger a save */
  save: () => void;
}

/**
 * Creates a persistent table settings manager.
 * Automatically loads from localStorage on init and saves on changes.
 *
 * @example
 * ```svelte
 * <script>
 * const persistence = createTablePersistence({ tableId: 'students' });
 *
 * const table = createTableState({
 *   data: students,
 *   columns: studentColumns,
 *   initialColumnVisibility: persistence.columnVisibility,
 *   initialColumnSizing: persistence.columnSizing,
 *   initialColumnOrder: persistence.columnOrder,
 *   initialSorting: persistence.sorting,
 * });
 *
 * // Sync changes back to persistence
 * $effect(() => {
 *   persistence.columnVisibility = table.columnVisibility;
 *   persistence.columnSizing = table.columnSizing;
 *   persistence.columnOrder = table.columnOrder;
 *   persistence.sorting = table.sorting;
 * });
 * </script>
 * ```
 */
export function createTablePersistence(
  options: TablePersistenceOptions,
): TablePersistenceState {
  const debounceMs = options.debounceMs ?? 300;

  // ── Load from Storage ───────────────────────────────────────────

  function loadFromStorage(): Partial<TableSettings> | undefined {
    try {
      if (typeof localStorage === "undefined") return undefined;
      const raw = localStorage.getItem(STORAGE_KEY);
      if (!raw) return undefined;
      const all: Record<string, Partial<TableSettings>> = JSON.parse(raw);
      return all[options.tableId];
    } catch {
      return undefined;
    }
  }

  function getDefaults(): TableSettings {
    const visibility: Record<string, boolean> = {};
    for (const col of options.defaultHiddenColumns ?? []) {
      visibility[col] = false;
    }
    return {
      columns: visibility,
      sizing: {},
      order: [],
      sorting: { column: null, direction: "asc" },
    };
  }

  const defaults = getDefaults();
  const saved = options.initialSettings ?? loadFromStorage();

  // ── Reactive State ──────────────────────────────────────────────

  let columnVisibility = $state<Record<string, boolean>>(
    saved?.columns ?? defaults.columns,
  );
  let columnSizing = $state<Record<string, number>>(
    saved?.sizing ?? defaults.sizing,
  );
  let columnOrder = $state<string[]>(saved?.order ?? defaults.order);
  let sorting = $state<SortingState>(
    saved?.sorting ?? defaults.sorting ?? { column: null, direction: "asc" },
  );

  // ── Debounced Save ──────────────────────────────────────────────

  let debounceTimer: ReturnType<typeof setTimeout> | null = null;
  let isInitial = true;

  function saveToStorage() {
    if (typeof localStorage === "undefined") return;

    try {
      const raw = localStorage.getItem(STORAGE_KEY);
      const all: Record<string, Partial<TableSettings>> = raw
        ? JSON.parse(raw)
        : {};

      all[options.tableId] = {
        columns: columnVisibility,
        sizing: columnSizing,
        order: columnOrder,
        sorting,
      };

      localStorage.setItem(STORAGE_KEY, JSON.stringify(all));
    } catch (e) {
      console.error(
        `[TablePersistence] Failed to save settings for "${options.tableId}":`,
        e,
      );
    }
  }

  function scheduleSave() {
    // Skip the first reactivity trigger (initial mount)
    if (isInitial) {
      isInitial = false;
      return;
    }

    if (debounceTimer) clearTimeout(debounceTimer);
    debounceTimer = setTimeout(saveToStorage, debounceMs);
  }

  // ── Auto-save on changes ────────────────────────────────────────
  // The consuming component should call `save()` or set up an $effect
  // to sync table state → persistence state (which triggers auto-save).

  // ── Public API ──────────────────────────────────────────────────

  return {
    get columnVisibility() {
      return columnVisibility;
    },
    set columnVisibility(v: Record<string, boolean>) {
      columnVisibility = v;
      scheduleSave();
    },

    get columnSizing() {
      return columnSizing;
    },
    set columnSizing(v: Record<string, number>) {
      columnSizing = v;
      scheduleSave();
    },

    get columnOrder() {
      return columnOrder;
    },
    set columnOrder(v: string[]) {
      columnOrder = v;
      scheduleSave();
    },

    get sorting() {
      return sorting;
    },
    set sorting(v: SortingState) {
      sorting = v;
      scheduleSave();
    },

    save: saveToStorage,
  };
}
