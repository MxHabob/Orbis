import * as XLSX from "xlsx";
import { save } from "@tauri-apps/plugin-dialog";
import { writeFile } from "@tauri-apps/plugin-fs";
import { openPath } from "@tauri-apps/plugin-opener";
import type { BulkImportProfile, BulkRowState } from "../types";

export interface ExportDatasetOptions {
  profile: BulkImportProfile<any, any>;
  rows: BulkRowState[];
  filenamePrefix?: string;
  format?: "xlsx" | "csv";
}

export async function exportCorrectedDataset(options: ExportDatasetOptions): Promise<string | null> {
  const { profile, rows, filenamePrefix = "Orbis_Corrected_Data", format = "xlsx" } = options;

  const headers = profile.columnMetadata.map((col) => col.headerAr);
  headers.push("حالة التحقق (Validation Status)");

  const aoaData: unknown[][] = [headers];

  for (const r of rows) {
    const rowLine = profile.columnMetadata.map((col) => r.data[col.id] ?? "");
    let statusText = "صالح (Valid)";
    if (r.status === "error") {
      const errMsgs = Array.from(r.errors.entries())
        .map(([k, v]) => `${k}: ${v}`)
        .join("; ");
      statusText = `خطأ: ${errMsgs}`;
    } else if (r.status === "warning") {
      const warnMsgs = Array.from(r.warnings.entries())
        .map(([k, v]) => `${k}: ${v}`)
        .join("; ");
      statusText = `تنبيه: ${warnMsgs}`;
    }
    rowLine.push(statusText);
    aoaData.push(rowLine);
  }

  const wb = XLSX.utils.book_new();
  const ws = XLSX.utils.aoa_to_sheet(aoaData);
  XLSX.utils.book_append_sheet(wb, ws, "البيانات_المصححة");

  const rawBytes = XLSX.write(wb, {
    bookType: format === "xlsx" ? "xlsx" : "csv",
    type: "array",
  });

  const defaultFilename = `${filenamePrefix}_${Date.now()}.${format}`;

  try {
    const savePath = await save({
      defaultPath: defaultFilename,
      filters: [{ name: format === "xlsx" ? "Excel Spreadsheet" : "CSV Document", extensions: [format] }],
    });

    if (savePath) {
      await writeFile(savePath, new Uint8Array(rawBytes));
      return savePath;
    }
  } catch (err) {
    console.warn("Native save dialog fallback:", err);
  }

  // Browser fallback
  const mimeType =
    format === "xlsx"
      ? "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
      : "text/csv;charset=utf-8";

  const blob = new Blob([rawBytes], { type: mimeType });
  const url = URL.createObjectURL(blob);
  const link = document.createElement("a");
  link.href = url;
  link.download = defaultFilename;
  document.body.appendChild(link);
  link.click();
  setTimeout(() => {
    URL.revokeObjectURL(url);
    link.remove();
  }, 100);

  return null;
}

export async function openExportedFile(filePath: string): Promise<void> {
  try {
    await openPath(filePath);
  } catch (err) {
    console.warn("Could not open exported file via native opener:", err);
  }
}
