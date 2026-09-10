/**
 * Thin Data Source helpers for Orbis grids.
 * TanStack Query remains the server-state cache; this only shapes list/bulk I/O.
 */
import type {
  BulkResult,
  DataPage,
  DataQuery,
  GridDataSource,
  GridRow,
} from "@orbis/svelte-grid";
import { shouldUseServerFilter } from "@orbis/svelte-grid";

export { shouldUseServerFilter };

const SERVER_PAGE_MAX = 100;

export function asPage<T>(
  items: T[],
  totalCount = items.length,
  page = 1,
  pageSize = items.length || 20,
): DataPage<T> {
  const size = Math.max(1, pageSize);
  return {
    items,
    totalCount,
    page,
    pageSize: size,
    totalPages: Math.max(1, Math.ceil(totalCount / size)),
  };
}

export function createClientSource<T extends GridRow>(
  loadAll: () => Promise<T[]>,
): GridDataSource<T> {
  return {
    mode: "client",
    async load() {
      const items = await loadAll();
      return asPage(items);
    },
  };
}

export function createServerSource<T extends GridRow>(
  fetchPage: (query: DataQuery) => Promise<DataPage<T>>,
  mutations?: Pick<GridDataSource<T>, "create" | "update" | "delete">,
): GridDataSource<T> {
  return {
    mode: "server",
    load: (query) =>
      fetchPage({
        ...query,
        pageSize: Math.min(query.pageSize, SERVER_PAGE_MAX),
      }),
    ...mutations,
  };
}

export function partitionBulk<T>(
  rows: T[],
  errors: Map<number, string[]>,
): BulkResult<T> {
  const accepted: T[] = [];
  const rejected: BulkResult<T>["rejected"] = [];
  rows.forEach((row, rowIndex) => {
    const msgs = errors.get(rowIndex);
    if (!msgs?.length) {
      accepted.push(row);
      return;
    }
    rejected.push({
      rowIndex,
      row,
      errors: msgs.map((message) => ({
        rowIndex,
        columnId: "",
        value: row,
        message,
      })),
    });
  });
  return {
    accepted,
    rejected,
    acceptedCount: accepted.length,
    rejectedCount: rejected.length,
  };
}

export type SelectAllIntent = "filtered" | "visible";

export function describeSelectAll(
  intent: SelectAllIntent,
  filteredCount: number,
  visibleCount: number,
): string {
  if (intent === "filtered") return `All ${filteredCount} filtered rows`;
  return `${visibleCount} visible rows`;
}
