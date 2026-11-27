import * as m from "$lib/paraglide/messages";
import { getLocale } from "$lib/paraglide/runtime";
import { downloadSpreadsheet } from "./workbook-download";
import { getActiveImportColumns } from "./template-columns";
import { getBulkEntryColumnHeader } from "../localization";
import type {
  BulkImportProfile,
  BulkReferenceData,
  BulkRowState,
} from "../types";

export interface ExportDatasetOptions {
  profile: BulkImportProfile<any, any>;
  rows: BulkRowState[];
  filenamePrefix?: string;
  format?: "xlsx" | "csv";
  registrationFields?: BulkReferenceData["registrationFields"];
}

export async function exportCorrectedDataset(options: ExportDatasetOptions) {
  const XLSX = await import("xlsx");
  const {
    profile,
    rows,
    filenamePrefix = "Orbis_Corrected_Data",
    format = "xlsx",
    registrationFields,
  } = options;

  const arabic = getLocale() === "ar";
  const columns = getActiveImportColumns(profile, registrationFields);
  const headers = columns.map((col) =>
    getBulkEntryColumnHeader(
      profile.id,
      col.id,
      arabic ? col.headerAr : col.headerEn,
      false,
    ),
  );
  headers.push(m.bulk_entry_export_validation_status());

  const aoaData: unknown[][] = [headers];

  for (const r of rows) {
    const rowLine = columns.map((col) => r.data[col.id] ?? "");
    let statusText = m.bulk_entry_export_status_valid();
    if (r.status === "failed") {
      statusText = m.bulk_entry_export_status_error({
        details: r.commitError ?? "",
      });
    } else if (r.status === "error") {
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
  wb.Workbook = { Views: [{ RTL: arabic }] };
  const ws = XLSX.utils.aoa_to_sheet(aoaData);
  ws["!cols"] = headers.map((header, index) => ({
    wch:
      index === headers.length - 1
        ? 56
        : Math.min(36, Math.max(18, header.length + 6)),
  }));
  ws["!autofilter"] = { ref: ws["!ref"]! };
  XLSX.utils.book_append_sheet(wb, ws, m.bulk_entry_export_sheet());

  const rawBytes = XLSX.write(wb, {
    bookType: format === "xlsx" ? "xlsx" : "csv",
    type: "array",
  });

  return downloadSpreadsheet(rawBytes, filenamePrefix + "." + format, format);
}
