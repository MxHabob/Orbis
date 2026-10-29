import { save } from "@tauri-apps/plugin-dialog";
import { writeFile } from "@tauri-apps/plugin-fs";
import * as m from "$lib/paraglide/messages";
import {
  downloadBlob,
  revealDownloadedPath,
  type RevealDownloadedLocation,
} from "$lib/shared/utils/download";
import { getBulkEntryColumnHeader } from "../localization";
import type { BulkImportProfile, BulkRowState } from "../types";

export interface ExportDatasetOptions {
  profile: BulkImportProfile<any, any>;
  rows: BulkRowState[];
  filenamePrefix?: string;
  format?: "xlsx" | "csv";
}

export async function exportCorrectedDataset(
  options: ExportDatasetOptions,
): Promise<RevealDownloadedLocation | null> {
  const XLSX = await import("xlsx");
  const {
    profile,
    rows,
    filenamePrefix = "Orbis_Corrected_Data",
    format = "xlsx",
  } = options;

  const headers = profile.columnMetadata.map((col) =>
    getBulkEntryColumnHeader(profile.id, col.id, col.headerAr),
  );
  headers.push(m.bulk_entry_export_validation_status());

  const aoaData: unknown[][] = [headers];

  for (const r of rows) {
    const rowLine = profile.columnMetadata.map((col) => r.data[col.id] ?? "");
    let statusText = m.bulk_entry_export_status_valid();
    if (r.status === "error") {
      const errMsgs = Array.from(r.errors.entries())
        .map(([k, v]) => `${k}: ${v}`)
        .join("; ");
      statusText = m.bulk_entry_export_status_error({ details: errMsgs });
    } else if (r.status === "warning") {
      const warnMsgs = Array.from(r.warnings.entries())
        .map(([k, v]) => `${k}: ${v}`)
        .join("; ");
      statusText = m.bulk_entry_export_status_warning({ details: warnMsgs });
    }
    rowLine.push(statusText);
    aoaData.push(rowLine);
  }

  const wb = XLSX.utils.book_new();
  const ws = XLSX.utils.aoa_to_sheet(aoaData);
  XLSX.utils.book_append_sheet(wb, ws, m.bulk_entry_export_sheet());

  const rawBytes = XLSX.write(wb, {
    bookType: format === "xlsx" ? "xlsx" : "csv",
    type: "array",
  });

  const defaultFilename = `${filenamePrefix}_${Date.now()}.${format}`;

  try {
    const savePath = await save({
      defaultPath: defaultFilename,
      filters: [
        {
          name:
            format === "xlsx"
              ? m.bulk_entry_file_filter_excel()
              : m.bulk_entry_file_filter_csv(),
          extensions: [format],
        },
      ],
    });

    if (savePath) {
      await writeFile(savePath, new Uint8Array(rawBytes));
      return revealDownloadedPath(savePath);
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
  return downloadBlob(blob, defaultFilename);
}
