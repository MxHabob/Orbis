import { isTauri } from "@tauri-apps/api/core";
import { save } from "@tauri-apps/plugin-dialog";
import { writeFile } from "@tauri-apps/plugin-fs";
import * as m from "$lib/paraglide/messages";
import {
  downloadBlob,
  revealDownloadedPath,
  type RevealDownloadedLocation,
} from "$lib/shared/utils/download";

/** Undefined means the native save dialog was cancelled. */
export async function downloadSpreadsheet(
  bytes: ArrayBuffer,
  filename: string,
  format: "xlsx" | "csv",
): Promise<RevealDownloadedLocation | null | undefined> {
  if (isTauri()) {
    const path = await save({
      defaultPath: filename,
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
    if (!path) return undefined;
    await writeFile(path, new Uint8Array(bytes));
    return revealDownloadedPath(path);
  }
  const type =
    format === "xlsx"
      ? "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
      : "text/csv;charset=utf-8";
  return downloadBlob(new Blob([bytes], { type }), filename);
}
