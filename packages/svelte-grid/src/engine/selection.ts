import type { CellRange, SelectScope, SelectionSnapshot } from "../types";

export function emptySelection(): SelectionSnapshot {
  return { mode: "explicit", ids: new Set(), range: null, active: null };
}

export function normalizeRange(range: CellRange): CellRange {
  return {
    startRow: Math.min(range.startRow, range.endRow),
    startCol: Math.min(range.startCol, range.endCol),
    endRow: Math.max(range.startRow, range.endRow),
    endCol: Math.max(range.startCol, range.endCol),
  };
}

export function rangeContains(
  range: CellRange | null,
  row: number,
  col: number,
): boolean {
  if (!range) return false;
  const n = normalizeRange(range);
  return (
    row >= n.startRow && row <= n.endRow && col >= n.startCol && col <= n.endCol
  );
}

export function rangeSize(range: CellRange | null): {
  rows: number;
  cols: number;
} {
  if (!range) return { rows: 0, cols: 0 };
  const n = normalizeRange(range);
  return { rows: n.endRow - n.startRow + 1, cols: n.endCol - n.startCol + 1 };
}

export function isRowSelected(
  sel: SelectionSnapshot,
  id: string,
  filteredCount: number,
): boolean {
  if (sel.mode === "allMatching") return !sel.ids.has(id);
  return sel.ids.has(id);
}

export function selectedCount(
  sel: SelectionSnapshot,
  filteredCount: number,
): number {
  if (sel.mode === "allMatching")
    return Math.max(0, filteredCount - sel.ids.size);
  return sel.ids.size;
}

export function toggleId(
  sel: SelectionSnapshot,
  id: string,
): SelectionSnapshot {
  const ids = new Set(sel.ids);
  if (sel.mode === "allMatching") {
    if (ids.has(id)) ids.delete(id);
    else ids.add(id);
    return { ...sel, ids };
  }
  if (ids.has(id)) ids.delete(id);
  else ids.add(id);
  return { ...sel, ids, mode: "explicit" };
}

export function selectIds(
  sel: SelectionSnapshot,
  ids: Iterable<string>,
  additive: boolean,
): SelectionSnapshot {
  const next = additive ? new Set(sel.ids) : new Set<string>();
  for (const id of ids) next.add(id);
  return { ...sel, mode: "explicit", ids: next };
}

export function selectScope(
  ids: string[],
  scope: SelectScope,
  visibleIds: string[],
  pageIds: string[],
): SelectionSnapshot {
  let chosen = ids;
  if (scope === "visible") chosen = visibleIds;
  if (scope === "page") chosen = pageIds;
  if (scope === "filtered" || scope === "allLoaded") {
    return { mode: "allMatching", ids: new Set(), range: null, active: null };
  }
  return { mode: "explicit", ids: new Set(chosen), range: null, active: null };
}

export function extendRange(
  origin: { row: number; col: number },
  target: { row: number; col: number },
): CellRange {
  return {
    startRow: origin.row,
    startCol: origin.col,
    endRow: target.row,
    endCol: target.col,
  };
}

export function collectRangeIds(ids: string[], range: CellRange): string[] {
  const n = normalizeRange(range);
  const out: string[] = [];
  for (let i = n.startRow; i <= n.endRow && i < ids.length; i++) {
    const id = ids[i];
    if (id !== undefined) out.push(id);
  }
  return out;
}
