import { downloadBlob, type RevealDownloadedLocation } from "./download";

/**
 * Data export utilities for tables and reports.
 * Supports CSV export with UTF-8 BOM (for Excel Arabic support) and JSON export.
 */

export interface ExportColumn<T = any> {
  header: string;
  accessor: (item: T) => string | number | null | undefined;
}

/**
 * Exports data array to a CSV file and triggers a browser/desktop download.
 *
 * @param filename - Base filename without extension
 * @param data - Array of records to export
 * @param columns - Column definitions with headers and accessor functions
 */
export function exportToCsv<T>(
  filename: string,
  data: T[],
  columns: ExportColumn<T>[],
): RevealDownloadedLocation | null {
  if (!data || data.length === 0) return null;

  const escapeCell = (val: string | number | null | undefined): string => {
    if (val === null || val === undefined) return '""';
    const str = String(val).replace(/"/g, '""');
    return `"${str}"`;
  };

  const headerRow = columns.map((col) => escapeCell(col.header)).join(",");
  const dataRows = data.map((row) =>
    columns.map((col) => escapeCell(col.accessor(row))).join(","),
  );

  // Prepend UTF-8 BOM (\uFEFF) to ensure Excel opens Arabic characters correctly
  const csvContent = "\uFEFF" + [headerRow, ...dataRows].join("\r\n");
  const blob = new Blob([csvContent], { type: "text/csv;charset=utf-8;" });
  return downloadBlob(
    blob,
    `${filename}_${new Date().toISOString().slice(0, 10)}.csv`,
  );
}

/**
 * Exports data array to a JSON file and triggers a download.
 */
export function exportToJson<T>(
  filename: string,
  data: T[],
): RevealDownloadedLocation | null {
  if (!data || data.length === 0) return null;

  const jsonContent = JSON.stringify(data, null, 2);
  const blob = new Blob([jsonContent], { type: "application/json;charset=utf-8;" });
  return downloadBlob(
    blob,
    `${filename}_${new Date().toISOString().slice(0, 10)}.json`,
  );
}
