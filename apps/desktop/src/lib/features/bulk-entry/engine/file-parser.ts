import { open } from "@tauri-apps/plugin-dialog";
import { readFile } from "@tauri-apps/plugin-fs";
import * as m from "$lib/paraglide/messages";
import type { BulkImportProfile, BulkRowState } from "../types";
import { getBulkEntryColumnHeader } from "../localization";
import { findSpreadsheetColumn, normalizeSpreadsheetHeader } from "./header-mapping";

export interface ParsedFileResult {
  fileName: string;
  totalRowsCount: number;
  rows: BulkRowState[];
  headers: string[];
}

export async function pickAndParseFile(profile: BulkImportProfile<any, any>): Promise<ParsedFileResult | null> {
  try {
    const selected = await open({
      multiple: false,
      filters: [{ name: m.bulk_entry_file_filter_spreadsheets(), extensions: ["xlsx", "xls", "csv"] }],
    });

    if (!selected || typeof selected !== "string") {
      return null;
    }

    const fileBytes = await readFile(selected);
    const fileName = selected.split(/[\\/]/).pop() || "imported_file.xlsx";
    return parseSpreadsheetBytes(fileBytes, fileName, profile);
  } catch (err) {
    console.warn("Native file picker error, browser interaction required:", err);
    throw err;
  }
}

export async function parseDroppedFile(file: File, profile: BulkImportProfile<any, any>): Promise<ParsedFileResult> {
  const arrayBuffer = await file.arrayBuffer();
  const fileBytes = new Uint8Array(arrayBuffer);
  return parseSpreadsheetBytes(fileBytes, file.name, profile);
}

export async function parseSpreadsheetBytes(
  bytes: Uint8Array,
  fileName: string,
  profile: BulkImportProfile<any, any>,
): Promise<ParsedFileResult> {
  const XLSX = await import("xlsx");
  const workbook = XLSX.read(bytes, { type: "array", cellDates: true });

  // Use the first non-reference sheet
  const targetSheetName =
    workbook.SheetNames.find((name) => !name.startsWith("_") && !name.toLowerCase().includes("instruction")) ||
    workbook.SheetNames[0] ||
    m.bulk_entry_parser_default_sheet();

  const worksheet = workbook.Sheets[targetSheetName];
  if (!worksheet) {
    throw new Error(m.bulk_entry_parser_no_data_sheet());
  }

  // Convert worksheet to raw array of rows
  const rawData: unknown[][] = XLSX.utils.sheet_to_json(worksheet, {
    header: 1,
    defval: "",
    raw: false,
    dateNF: "YYYY-MM-DD",
  });

  if (rawData.length < 2) {
    throw new Error(m.bulk_entry_parser_empty_file());
  }

  const rawHeaders = (rawData[0] as string[]).map((h) => String(h || "").trim());
  const columnMap: Record<number, string> = {};

  // Map header column indices to profile column metadata IDs
  for (let c = 0; c < rawHeaders.length; c++) {
    const headerText = rawHeaders[c]!;
    if (!headerText) continue;

    const matchedMeta = findSpreadsheetColumn(
      headerText,
      profile.columnMetadata,
      (col) =>
        getBulkEntryColumnHeader(profile.id, col.id, col.headerAr),
    );

    if (matchedMeta) {
      columnMap[c] = matchedMeta.id;
      continue;
    }

    if (profile.id === "students" || profile.id === "employees") {
      const normalizedHeader = normalizeSpreadsheetHeader(headerText);
      const legacyStudentHeaders: Record<string, string> = {
        firstnamear: "legacy_first_name_ar",
        الاسمالأولعربي: "legacy_first_name_ar",
        firstnamearabic: "legacy_first_name_ar",
        middlenamear: "legacy_middle_name_ar",
        اسمالأبعربي: "legacy_middle_name_ar",
        middlenamarabic: "legacy_middle_name_ar",
        lastnamear: "legacy_last_name_ar",
        اسمالعائلةعربي: "legacy_last_name_ar",
        lastnamearabic: "legacy_last_name_ar",
        firstname: "legacy_first_name",
        firstnameenglish: "legacy_first_name",
        lastname: "legacy_last_name",
        lastnameenglish: "legacy_last_name",
      };
      const legacyColumnId = legacyStudentHeaders[normalizedHeader];
      if (legacyColumnId) columnMap[c] = legacyColumnId;
    }
  }

  const rows: BulkRowState[] = [];

  for (let r = 1; r < rawData.length; r++) {
    const line = rawData[r] as unknown[];
    if (!line || line.every((cell) => cell === null || cell === undefined || String(cell).trim() === "")) {
      continue; // skip completely empty rows
    }

    const rowData: Record<string, unknown> = {
      __idx: rows.length + 1,
      rowNumber: r + 1,
      id: `import-row-${r + 1}`,
    };

    for (let c = 0; c < line.length; c++) {
      const colId = columnMap[c];
      if (colId) {
        let cellVal = line[c];
        if (cellVal instanceof Date) {
          cellVal = cellVal.toISOString().slice(0, 10);
        } else if (cellVal != null) {
          cellVal = String(cellVal).trim();
        }
        rowData[colId] = cellVal;
      }
    }

    if (profile.id === "students" || profile.id === "employees") {
      const legacyArabicName = [
        rowData.legacy_first_name_ar,
        rowData.legacy_middle_name_ar,
        rowData.legacy_last_name_ar,
      ]
        .filter((part) => String(part ?? "").trim())
        .join(" ");
      const legacyEnglishName = [rowData.legacy_first_name, rowData.legacy_last_name]
        .filter((part) => String(part ?? "").trim())
        .join(" ");

      if (!rowData.full_name_ar && legacyArabicName) rowData.full_name_ar = legacyArabicName;
      if (!rowData.full_name_en && legacyEnglishName) rowData.full_name_en = legacyEnglishName;
      for (const legacyId of [
        "legacy_first_name_ar",
        "legacy_middle_name_ar",
        "legacy_last_name_ar",
        "legacy_first_name",
        "legacy_last_name",
      ]) {
        delete rowData[legacyId];
      }
    }

    rows.push({
      rowNumber: r + 1,
      data: rowData,
      errors: new Map(),
      warnings: new Map(),
      info: new Map(),
      status: "valid",
    });
  }

  return {
    fileName,
    totalRowsCount: rows.length,
    rows,
    headers: rawHeaders,
  };
}
