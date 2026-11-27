import type {
  BulkCellError,
  BulkResult,
  CellRange,
  GridRow,
  OrbisColumnDef,
} from "../types";
import { columnId, getCellValue } from "../types";
import { normalizeRange } from "./selection";
import { coerceValue } from "./coerce";

export function parseClipboard(text: string, delimiter?: string): string[][] {
  const raw = text.replace(/^\uFEFF/, "").replace(/\r\n?/g, "\n");
  if (!raw.trim()) return [];
  const separator = delimiter ?? (raw.includes("\t") ? "\t" : ",");
  const rows: string[][] = [];
  let row: string[] = [];
  let cell = "";
  let quoted = false;
  for (let index = 0; index < raw.length; index++) {
    const char = raw[index]!;
    if (quoted) {
      if (char === '"' && raw[index + 1] === '"') {
        cell += '"';
        index++;
      } else if (char === '"') {
        quoted = false;
      } else {
        cell += char;
      }
    } else if (char === '"' && cell === "") {
      quoted = true;
    } else if (char === separator || char === "\n") {
      row.push(cell);
      cell = "";
      if (char === "\n") {
        rows.push(row);
        row = [];
      }
    } else {
      cell += char;
    }
  }
  if (row.length || cell || !raw.endsWith("\n")) rows.push([...row, cell]);
  return rows;
}

export function toTsv(matrix: unknown[][]): string {
  return matrix
    .map((row) =>
      row
        .map((cell) => {
          const s = cell == null ? "" : String(cell);
          if (/[\t\n\r"]/.test(s)) return `"${s.replace(/"/g, '""')}"`;
          return s;
        })
        .join("\t"),
    )
    .join("\n");
}

export function extractMatrix(
  rows: GridRow[],
  columns: OrbisColumnDef[],
  range: CellRange,
): unknown[][] {
  const n = normalizeRange(range);
  const matrix: unknown[][] = [];
  for (let r = n.startRow; r <= n.endRow && r < rows.length; r++) {
    const row = rows[r]!;
    const line: unknown[] = [];
    for (let c = n.startCol; c <= n.endCol && c < columns.length; c++) {
      line.push(getCellValue(row, columns[c]!));
    }
    matrix.push(line);
  }
  return matrix;
}

export function applyPaste(args: {
  rows: GridRow[];
  columns: OrbisColumnDef[];
  startRow: number;
  startCol: number;
  matrix: string[][];
  createEmptyRow?: () => GridRow;
  fixedRows?: boolean;
}): { rows: GridRow[]; result: BulkResult } {
  const { columns, startRow, startCol, matrix, createEmptyRow, fixedRows } =
    args;
  const rows = args.rows.slice();
  const accepted: GridRow[] = [];
  const rejected: BulkResult["rejected"] = [];

  for (let r = 0; r < matrix.length; r++) {
    const targetIndex = startRow + r;
    if (fixedRows && targetIndex >= rows.length) break;
    while (targetIndex >= rows.length) {
      rows.push(createEmptyRow ? createEmptyRow() : {});
    }
    const line = matrix[r] ?? [];
    const next = { ...rows[targetIndex] };
    const errors: BulkCellError[] = [];
    for (let c = 0; c < line.length; c++) {
      const col = columns[startCol + c];
      if (!col || col.readonly) continue;
      const id = columnId(col);
      if (!id || id.startsWith("__")) continue;
      const raw = line[c] ?? "";
      const coerced = coerceValue(raw, col, next);
      if (!coerced.ok) {
        errors.push({
          rowIndex: targetIndex,
          columnId: id,
          value: raw,
          message: coerced.message ?? "Invalid value",
        });
        if (!fixedRows) next[id] = raw;
        continue;
      }
      next[id] = coerced.value as GridRow[string];
    }
    const map = new Map<string, string>();
    for (const err of errors) map.set(err.columnId, err.message);
    next.__errors = map;
    rows[targetIndex] = next;
    if (errors.length)
      rejected.push({ rowIndex: targetIndex, row: next, errors });
    else accepted.push(next);
  }

  return {
    rows,
    result: {
      accepted,
      rejected,
      acceptedCount: accepted.length,
      rejectedCount: rejected.length,
    },
  };
}

export function fillDown(
  rows: GridRow[],
  columns: OrbisColumnDef[],
  range: CellRange,
): GridRow[] {
  const n = normalizeRange(range);
  if (n.startRow >= rows.length) return rows;
  const source = rows[n.startRow]!;
  const next = rows.slice();
  for (let r = n.startRow + 1; r <= n.endRow && r < next.length; r++) {
    const row = { ...next[r] };
    for (let c = n.startCol; c <= n.endCol && c < columns.length; c++) {
      const col = columns[c]!;
      if (col.readonly) continue;
      const value = coerceValue(getCellValue(source, col), col, row);
      if (value.ok) row[columnId(col)] = value.value;
    }
    next[r] = row;
  }
  return next;
}

export function fillRight(
  rows: GridRow[],
  columns: OrbisColumnDef[],
  range: CellRange,
): GridRow[] {
  const n = normalizeRange(range);
  const next = rows.slice();
  for (let r = n.startRow; r <= n.endRow && r < next.length; r++) {
    const row = { ...next[r] };
    const sourceCol = columns[n.startCol];
    if (!sourceCol) continue;
    const sourceVal = getCellValue(row, sourceCol);
    for (let c = n.startCol + 1; c <= n.endCol && c < columns.length; c++) {
      const col = columns[c]!;
      if (col.readonly) continue;
      const value = coerceValue(sourceVal, col, row);
      if (value.ok) row[columnId(col)] = value.value;
    }
    next[r] = row;
  }
  return next;
}

export function duplicateRows(
  rows: GridRow[],
  indexes: number[],
  createId?: () => string,
): GridRow[] {
  const next = rows.slice();
  const copies: GridRow[] = [];
  const unique = [...new Set(indexes)].sort((a, b) => a - b);
  for (const index of unique) {
    const src = rows[index];
    if (!src) continue;
    const copy: GridRow = { ...src };
    if (createId) copy.id = createId();
    delete copy.__errors;
    copies.push(copy);
  }
  const insertAt = (unique[unique.length - 1] ?? rows.length - 1) + 1;
  next.splice(insertAt, 0, ...copies);
  return next;
}

export function clearRange(
  rows: GridRow[],
  columns: OrbisColumnDef[],
  range: CellRange,
): GridRow[] {
  const n = normalizeRange(range);
  const next = rows.slice();
  for (let r = n.startRow; r <= n.endRow && r < next.length; r++) {
    const row = { ...next[r] };
    for (let c = n.startCol; c <= n.endCol && c < columns.length; c++) {
      const col = columns[c]!;
      if (col.readonly) continue;
      const value = coerceValue(null, col, row);
      if (value.ok) row[columnId(col)] = null;
    }
    next[r] = row;
  }
  return next;
}
