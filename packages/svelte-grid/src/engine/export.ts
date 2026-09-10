import type {
  ExportOptions,
  GridRow,
  OrbisColumnDef,
  SelectionSnapshot,
} from "../types";
import { columnId, columnHeader, getCellValue } from "../types";
import { isRowSelected } from "./selection";
import { toTsv } from "./clipboard";

export function exportRows(
  rows: GridRow[],
  columns: OrbisColumnDef[],
  options: ExportOptions,
  selection?: SelectionSnapshot,
  visibleIds?: string[],
  rowIdKey = "id",
): string {
  const cols = (
    options.columns
      ? columns.filter((c) => options.columns!.includes(columnId(c)))
      : columns
  ).filter((c) => !c.hidden && !columnId(c).startsWith("__"));

  let subset = rows;
  if (options.scope === "selected" && selection) {
    subset = rows.filter((row) =>
      isRowSelected(selection, String(row[rowIdKey] ?? ""), rows.length),
    );
  } else if (options.scope === "visible" && visibleIds) {
    const set = new Set(visibleIds);
    subset = rows.filter((row) => set.has(String(row[rowIdKey] ?? "")));
  }

  if (options.format === "json") {
    return JSON.stringify(
      subset.map((row) => {
        const obj: GridRow = {};
        for (const col of cols) obj[columnId(col)] = getCellValue(row, col);
        return obj;
      }),
    );
  }

  const header = cols.map(columnHeader);
  const matrix = [
    header,
    ...subset.map((row) => cols.map((col) => getCellValue(row, col))),
  ];
  const text = toTsv(matrix);
  return options.format === "csv" ? text.replace(/\t/g, ",") : text;
}

export function downloadText(
  filename: string,
  text: string,
  mime = "text/plain;charset=utf-8",
): void {
  if (typeof document === "undefined") return;
  const blob = new Blob([text], { type: mime });
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = filename;
  a.click();
  URL.revokeObjectURL(url);
}
