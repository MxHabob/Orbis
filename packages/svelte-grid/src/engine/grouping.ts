import type { AggregateFn, GridRow, GroupNode, OrbisColumnDef } from "../types";
import { columnId, getCellValue } from "../types";

function aggregate(values: unknown[], fn: AggregateFn): number | string | null {
  const nums = values.map((v) => Number(v)).filter((n) => Number.isFinite(n));
  switch (fn) {
    case "count":
      return values.length;
    case "count_distinct":
      return new Set(values.map((v) => String(v))).size;
    case "sum":
      return nums.reduce((a, b) => a + b, 0);
    case "avg":
      return nums.length ? nums.reduce((a, b) => a + b, 0) / nums.length : null;
    case "min":
      return nums.length ? Math.min(...nums) : null;
    case "max":
      return nums.length ? Math.max(...nums) : null;
    default:
      return null;
  }
}

export function groupRows<T extends GridRow>(
  rows: readonly T[],
  groupBy: string,
  columns: OrbisColumnDef[],
  collapsed = new Set<string>(),
): GroupNode<T>[] {
  const col = columns.find((c) => columnId(c) === groupBy);
  const buckets = new Map<string, T[]>();
  for (const row of rows) {
    const value = col ? getCellValue(row, col) : row[groupBy];
    const key = value == null || value === "" ? "(blank)" : String(value);
    const list = buckets.get(key);
    if (list) list.push(row);
    else buckets.set(key, [row]);
  }

  const aggregatesCols = columns.filter((c) => c.aggregate);
  const nodes: GroupNode<T>[] = [];
  for (const [key, group] of buckets) {
    const aggregates: Record<string, number | string | null> = {};
    for (const c of aggregatesCols) {
      const id = columnId(c);
      aggregates[id] = aggregate(
        group.map((row) => getCellValue(row, c)),
        c.aggregate!,
      );
    }
    nodes.push({
      key,
      value: group[0]
        ? col
          ? getCellValue(group[0], col)
          : group[0][groupBy]
        : key,
      rows: group,
      aggregates,
      collapsed: collapsed.has(key),
    });
  }
  return nodes;
}

export function flattenGroups<T extends GridRow>(nodes: GroupNode<T>[]): T[] {
  const out: T[] = [];
  for (const node of nodes) {
    if (!node.collapsed) out.push(...node.rows);
  }
  return out;
}

export function grandTotals(
  rows: GridRow[],
  columns: OrbisColumnDef[],
): Record<string, number | string | null> {
  const totals: Record<string, number | string | null> = {};
  for (const col of columns) {
    if (!col.aggregate) continue;
    const id = columnId(col);
    totals[id] = aggregate(
      rows.map((row) => getCellValue(row, col)),
      col.aggregate,
    );
  }
  return totals;
}
