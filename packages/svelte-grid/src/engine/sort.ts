import type { GridRow, OrbisColumnDef, SortSpec } from "../types";
import { columnId, getCellValue } from "../types";

function compareValues(a: unknown, b: unknown): number {
  if (a == null && b == null) return 0;
  if (a == null) return 1;
  if (b == null) return -1;
  if (typeof a === "number" && typeof b === "number") return a - b;
  if (typeof a === "boolean" && typeof b === "boolean") return Number(a) - Number(b);
  return String(a).localeCompare(String(b), undefined, { numeric: true, sensitivity: "base" });
}

export function applySort<T extends GridRow>(
  items: readonly T[],
  sorts: SortSpec[],
  columns: OrbisColumnDef[],
): T[] {
  if (!sorts.length) return items as T[];
  const colById = new Map(columns.map((c) => [columnId(c), c]));
  const indexed = items.map((row, index) => ({ row, index }));
  indexed.sort((left, right) => {
    for (const spec of sorts) {
      const col = colById.get(spec.id);
      const av = col ? getCellValue(left.row, col) : left.row[spec.id];
      const bv = col ? getCellValue(right.row, col) : right.row[spec.id];
      const cmp = typeof col?.sort === "function" ? col.sort(av, bv) : compareValues(av, bv);
      if (cmp !== 0) return spec.dir === "desc" ? -cmp : cmp;
    }
    return left.index - right.index;
  });
  return indexed.map((entry) => entry.row);
}

export function toggleSort(sorts: SortSpec[], id: string, additive: boolean): SortSpec[] {
  const existing = sorts.find((s) => s.id === id);
  if (!additive) {
    if (!existing) return [{ id, dir: "asc" }];
    if (existing.dir === "asc") return [{ id, dir: "desc" }];
    return [];
  }
  if (!existing) return [...sorts, { id, dir: "asc" }];
  if (existing.dir === "asc") {
    return sorts.map((s) => (s.id === id ? { id, dir: "desc" } : s));
  }
  return sorts.filter((s) => s.id !== id);
}
