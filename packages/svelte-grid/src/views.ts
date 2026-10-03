/**
 * Optional local view snapshots (filters/sort/columns).
 */
import type { GridFilter, SortSpec } from "./types";

export interface TableViewState {
  id: string;
  name: string;
  filter?: GridFilter;
  sorts?: SortSpec[];
  hidden?: string[];
  widths?: Record<string, number>;
  density?: number;
}

const keyFor = (tableId: string) => `orbis.table.views.${tableId}`;

export function loadViews(tableId: string): TableViewState[] {
  if (typeof localStorage === "undefined") return [];
  try {
    const raw = localStorage.getItem(keyFor(tableId));
    return raw ? (JSON.parse(raw) as TableViewState[]) : [];
  } catch {
    return [];
  }
}

export function saveView(
  tableId: string,
  view: TableViewState,
): TableViewState[] {
  const views = loadViews(tableId).filter((v) => v.id !== view.id);
  views.push(view);
  localStorage.setItem(keyFor(tableId), JSON.stringify(views));
  return views;
}

export function deleteView(tableId: string, viewId: string): TableViewState[] {
  const views = loadViews(tableId).filter((v) => v.id !== viewId);
  localStorage.setItem(keyFor(tableId), JSON.stringify(views));
  return views;
}
