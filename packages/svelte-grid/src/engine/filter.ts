import type {
  FilterCondition,
  GridFilter,
  GridRow,
  OrbisColumnDef,
} from "../types";
import { columnId, getCellValue } from "../types";

function normalizeOp(op: string): string {
  return op
    .replace(/([a-z])([A-Z])/g, "$1_$2")
    .replace(/-/g, "_")
    .toLowerCase();
}

function isEmpty(val: unknown): boolean {
  return val === null || val === undefined || val === "";
}

function asNumber(val: unknown): number {
  if (typeof val === "number") return val;
  if (typeof val === "boolean") return val ? 1 : 0;
  return Number(val);
}

function asString(val: unknown): string {
  if (val === null || val === undefined) return "";
  return String(val);
}

function compareFilterNumbers(
  val: unknown,
  target: unknown,
  col?: OrbisColumnDef,
): [number, number] {
  if (col?.type === "date" || col?.type === "datetime") {
    return [Date.parse(asString(val)), Date.parse(asString(target))];
  }
  return [asNumber(val), asNumber(target)];
}

function matchCondition(
  row: GridRow,
  cond: FilterCondition,
  columns?: ReadonlyMap<string, OrbisColumnDef>,
): boolean {
  const col = columns?.get(cond.field);
  const val = col ? getCellValue(row, col) : row[cond.field];
  const target = cond.value;
  const op = normalizeOp(String(cond.operator));

  switch (op) {
    case "eq":
      return asString(val).toLowerCase() === asString(target).toLowerCase();
    case "ne":
    case "neq":
      return asString(val).toLowerCase() !== asString(target).toLowerCase();
    case "like":
    case "contains":
      return asString(val)
        .toLowerCase()
        .includes(asString(target).toLowerCase());
    case "notlike":
    case "not_like":
    case "not_contains":
      return !asString(val)
        .toLowerCase()
        .includes(asString(target).toLowerCase());
    case "starts_with":
      return asString(val)
        .toLowerCase()
        .startsWith(asString(target).toLowerCase());
    case "ends_with":
      return asString(val)
        .toLowerCase()
        .endsWith(asString(target).toLowerCase());
    case "isnull":
    case "is_null":
      return isEmpty(val);
    case "isnotnull":
    case "is_not_null":
      return !isEmpty(val);
    case "gt":
    case "gte":
    case "lt":
    case "lte": {
      const [actual, expected] = compareFilterNumbers(val, target, col);
      if (op === "gt") return actual > expected;
      if (op === "gte") return actual >= expected;
      if (op === "lt") return actual < expected;
      return actual <= expected;
    }
    case "between": {
      const [num] = compareFilterNumbers(val, target, col);
      const [min, max] = Array.isArray(target)
        ? compareFilterNumbers(target[0], target[1], col)
        : compareFilterNumbers(target, target, col);
      return num >= min && num <= max;
    }
    case "in": {
      const arr = Array.isArray(target) ? target : [target];
      return arr.some(
        (t) => asString(t).toLowerCase() === asString(val).toLowerCase(),
      );
    }
    case "notin":
    case "not_in": {
      const arr = Array.isArray(target) ? target : [target];
      return !arr.some(
        (t) => asString(t).toLowerCase() === asString(val).toLowerCase(),
      );
    }
    case "bool_true":
      return val === true || val === 1 || val === "true";
    case "bool_false":
      return val === false || val === 0 || val === "false";
    default:
      return true;
  }
}

function matchesSearch(
  row: GridRow,
  search: string,
  columns?: ReadonlyMap<string, OrbisColumnDef>,
): boolean {
  const q = search.toLowerCase();
  if (columns) {
    for (const col of columns.values()) {
      const val = getCellValue(row, col);
      if (isSearchMatch(val, q)) return true;
    }
    return false;
  }
  for (const val of Object.values(row)) {
    if (isSearchMatch(val, q)) return true;
  }
  return false;
}

function isSearchMatch(value: unknown, query: string): boolean {
  if (value == null || (typeof value === "object" && !(value instanceof Date)))
    return false;
  return String(value).toLowerCase().includes(query);
}

export function matchesFilter(
  row: GridRow,
  filter: GridFilter | null | undefined,
  columns?: OrbisColumnDef[],
): boolean {
  const columnMap = columns
    ? new Map(columns.map((col) => [columnId(col), col]))
    : undefined;
  return matchesFilterWithColumns(row, filter, columnMap);
}

function matchesFilterWithColumns(
  row: GridRow,
  filter: GridFilter | null | undefined,
  columns?: ReadonlyMap<string, OrbisColumnDef>,
): boolean {
  if (!filter) return true;
  if (filter.search && !matchesSearch(row, filter.search, columns))
    return false;

  const groups = filter.groups ?? [];
  const conditions = filter.conditions ?? [];
  if (conditions.length === 0 && groups.length === 0) return true;

  const logic = filter.logic === "OR" ? "OR" : "AND";
  const parts: boolean[] = [];
  for (const cond of conditions) parts.push(matchCondition(row, cond, columns));
  for (const group of groups)
    parts.push(matchesFilterWithColumns(row, group, columns));
  if (parts.length === 0) return true;
  return logic === "AND" ? parts.every(Boolean) : parts.some(Boolean);
}

export function applyFilter<T extends GridRow>(
  items: readonly T[],
  filter: GridFilter | null | undefined,
  columns?: OrbisColumnDef[],
): T[] {
  if (!filter) return items as T[];
  const hasSearch = !!filter.search;
  const hasRules =
    (filter.conditions?.length ?? 0) > 0 || (filter.groups?.length ?? 0) > 0;
  if (!hasSearch && !hasRules) return items as T[];
  const columnMap = columns
    ? new Map(columns.map((col) => [columnId(col), col]))
    : undefined;
  const out: T[] = [];
  for (let i = 0; i < items.length; i++) {
    const row = items[i]!;
    if (matchesFilterWithColumns(row, filter, columnMap)) out.push(row);
  }
  return out;
}

export function isFilterActive(filter: GridFilter | null | undefined): boolean {
  if (!filter) return false;
  return (
    !!filter.search ||
    (filter.conditions?.length ?? 0) > 0 ||
    (filter.groups?.length ?? 0) > 0
  );
}
