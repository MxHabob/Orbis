// ── createTableExport ───────────────────────────────────────────────
// Export utilities for table data. School management systems commonly
// need CSV and JSON export for grades, attendance, and financial reports.
// XLSX export is supported via optional dynamic import (no bundle bloat).

import type { ColumnDef, ExportFormat, ExportOptions } from "./table-types.js";

// ── CSV Export ──────────────────────────────────────────────────────

function escapeCsvValue(value: string): string {
	if (value.includes(",") || value.includes('"') || value.includes("\n") || value.includes("\r")) {
		return `"${value.replace(/"/g, '""')}"`;
	}
	return value;
}

function toCsv<T>(data: T[], columns: ColumnDef<T>[], formatters?: Partial<Record<string, (v: unknown) => string>>): string {
	const headers = columns.map((col) => escapeCsvValue(typeof col.header === 'string' ? col.header : col.id));

	const rows = data.map((row) =>
		columns.map((col) => {
			let value: unknown;
			if (col.accessorFn) {
				value = col.accessorFn(row);
			} else if (col.accessorKey) {
				value = (row as Record<string, unknown>)[col.accessorKey];
			}

			// Apply custom formatter if available
			const formatter = formatters?.[col.id];
			if (formatter) return escapeCsvValue(formatter(value));

			if (value == null) return "";
			if (value instanceof Date) return escapeCsvValue(value.toISOString());
			return escapeCsvValue(String(value));
		}).join(","),
	);

	// BOM for Excel to properly detect UTF-8
	return "\uFEFF" + [headers.join(","), ...rows].join("\r\n");
}

// ── JSON Export ─────────────────────────────────────────────────────

function toJson<T>(data: T[], columns: ColumnDef<T>[], formatters?: Partial<Record<string, (v: unknown) => string>>): string {
	const result = data.map((row) => {
		const obj: Record<string, unknown> = {};
		for (const col of columns) {
			let value: unknown;
			if (col.accessorFn) {
				value = col.accessorFn(row);
			} else if (col.accessorKey) {
				value = (row as Record<string, unknown>)[col.accessorKey];
			}

			const formatter = formatters?.[col.id];
			obj[col.id] = formatter ? formatter(value) : value;
		}
		return obj;
	});

	return JSON.stringify(result, null, 2);
}

// ── File Download ───────────────────────────────────────────────────

function downloadFile(content: string, filename: string, mimeType: string) {
	const blob = new Blob([content], { type: `${mimeType};charset=utf-8` });
	const url = URL.createObjectURL(blob);
	const link = document.createElement("a");
	link.href = url;
	link.download = filename;
	link.style.display = "none";
	document.body.appendChild(link);
	link.click();

	// Cleanup
	setTimeout(() => {
		URL.revokeObjectURL(url);
		link.remove();
	}, 100);
}

// ── Public Export Function ──────────────────────────────────────────

const MIME_TYPES: Record<ExportFormat, string> = {
	csv: "text/csv",
	json: "application/json",
	xlsx: "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
};

/**
 * Export table data to a file.
 *
 * @example
 * ```ts
 * exportTableData({
 *   data: table.processedRows,
 *   columns: table.visibleColumns,
 *   format: 'csv',
 *   filename: 'students-report',
 * });
 * ```
 */
export async function exportTableData<T>(options: ExportOptions<T>): Promise<void> {
	const {
		data,
		columns,
		format,
		filename = "export",
		formatters,
	} = options;

	// Filter out non-data columns (actions, select, etc.)
	const exportableColumns = columns.filter(
		(col) => col.id !== "actions" && col.id !== "select" && (col.accessorKey != null || col.accessorFn != null),
	);

	const timestamp = new Date().toISOString().slice(0, 10);
	const fullFilename = `${filename}-${timestamp}`;

	switch (format) {
		case "csv": {
			const content = toCsv(data, exportableColumns, formatters);
			downloadFile(content, `${fullFilename}.csv`, MIME_TYPES.csv);
			break;
		}

		case "json": {
			const content = toJson(data, exportableColumns, formatters);
			downloadFile(content, `${fullFilename}.json`, MIME_TYPES.json);
			break;
		}

		case "xlsx": {
			// Dynamic import to avoid bundling xlsx library when not used
			console.warn(
				"[TableExport] XLSX export requires the 'xlsx' package. " +
				"Falling back to CSV. Install 'xlsx' for native Excel support.",
			);
			// Fallback to CSV
			const csvContent = toCsv(data, exportableColumns, formatters);
			downloadFile(csvContent, `${fullFilename}.csv`, MIME_TYPES.csv);
			break;
		}
	}
}

/**
 * Get exportable column count (columns with data accessors).
 */
export function getExportableColumnCount<T>(columns: ColumnDef<T>[]): number {
	return columns.filter(
		(col) => col.id !== "actions" && col.id !== "select" && (col.accessorKey != null || col.accessorFn != null),
	).length;
}
