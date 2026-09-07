import type { GridFilter, GridRow, OrbisColumnDef, SortSpec } from "../types";
import { applyFilter } from "./filter";
import { applySort } from "./sort";

export interface ClientQuery {
  filter?: GridFilter | null;
  sorts?: SortSpec[];
  search?: string;
}

export function queryClient<T extends GridRow>(
  rows: readonly T[],
  columns: OrbisColumnDef[],
  query: ClientQuery,
): T[] {
  const filter: GridFilter | null = query.filter
    ? { ...query.filter, search: query.search ?? query.filter.search }
    : query.search
      ? { logic: "AND", conditions: [], search: query.search }
      : null;
  const filtered = applyFilter(rows, filter, columns);
  return applySort(filtered, query.sorts ?? [], columns);
}

export function shouldUseServerFilter(totalHint: number | undefined, pageSize: number): boolean {
  if (totalHint == null) return false;
  return totalHint > Math.max(pageSize, 2000);
}
