import { open } from "@tauri-apps/plugin-dialog";
import { readFile } from "@tauri-apps/plugin-fs";
import * as m from "$lib/paraglide/messages";
import type { BulkImportProfile, BulkRowState } from "../types";
import { getBulkEntryColumnHeader } from "../localization";
import {
  findSpreadsheetColumn,
  normalizeSpreadsheetHeader,
} from "./header-mapping";

const legacyNameColumns: Record<string, string> = {
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

export interface ParsedFileResult {
  fileName: string;
  totalRowsCount: number;
  rows: BulkRowState[];
  headers: string[];
}

export async function pickAndParseFile(
  profile: BulkImportProfile<any, any>,
): Promise<ParsedFileResult | null> {
  try {
    const selected = await open({
      multiple: false,
      filters: [
        {
          name: m.bulk_entry_file_filter_spreadsheets(),
          extensions: ["xlsx", "xls", "csv"],
        },
      ],
    });

    if (!selected || typeof selected !== "string") {
      return null;
    }

    const fileBytes = await readFile(selected);
    const fileName = selected.split(/[\\/]/).pop() || "imported_file.xlsx";
    return parseSpreadsheetBytes(fileBytes, fileName, profile);
  } catch (err) {
    console.warn(
      "Native file picker error, browser interaction required:",
      err,
    );
    throw err;
  }
}

export async function parseDroppedFile(
  file: File,
  profile: BulkImportProfile<any, any>,
): Promise<ParsedFileResult> {
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
  const workbook = XLSX.read(bytes, {
    type: "array",
    cellDates: true,
    raw: true,
  });

  const candidateSheets = workbook.SheetNames.filter(
    (name, index) =>
      !name.startsWith("_") &&
      !name.toLowerCase().includes("instruction") &&
      !workbook.Workbook?.Sheets?.[index]?.Hidden,
  );
  const sheetName = candidateSheets.find((name) => {
    const sheet = workbook.Sheets[name];
    const range = sheet?.["!ref"];
    if (!range) return false;
    const { s: start, e: end } = XLSX.utils.decode_range(range);
    for (let column = start.c; column <= end.c; column++) {
      const cell = sheet[XLSX.utils.encode_cell({ c: column, r: start.r })];
      if (
        findSpreadsheetColumn(
          String(cell?.v ?? ""),
          profile.columnMetadata,
          (meta) =>
            getBulkEntryColumnHeader(profile.id, meta.id, meta.headerAr),
        )
      )
        return true;
      if (
        (profile.id === "students" || profile.id === "employees") &&
        legacyNameColumns[normalizeSpreadsheetHeader(String(cell?.v ?? ""))]
      )
        return true;
    }
    return false;
  });

  if (!sheetName) throw new Error(m.bulk_entry_parser_unrecognized_columns());
  const worksheet = workbook.Sheets[sheetName]!;
  const rowOffset = XLSX.utils.decode_range(worksheet["!ref"]!).s.r;
  const rawData: unknown[][] = XLSX.utils.sheet_to_json(worksheet, {
    header: 1,
    defval: "",
    raw: false,
    dateNF: "YYYY-MM-DD",
  });
  const rawHeaders = (rawData[0] ?? []).map((header) =>
    String(header ?? "").trim(),
  );
  const columnMap: Record<number, string> = {};

  // Map header column indices to profile column metadata IDs
  for (let c = 0; c < rawHeaders.length; c++) {
    const headerText = rawHeaders[c]!;
    if (!headerText) continue;

    const matchedMeta = findSpreadsheetColumn(
      headerText,
      profile.columnMetadata,
      (col) => getBulkEntryColumnHeader(profile.id, col.id, col.headerAr),
    );

    if (matchedMeta) {
      columnMap[c] = matchedMeta.id;
      continue;
    }

    if (profile.id === "students" || profile.id === "employees") {
      const normalizedHeader = normalizeSpreadsheetHeader(headerText);
      const legacyColumnId = legacyNameColumns[normalizedHeader];
      if (legacyColumnId) columnMap[c] = legacyColumnId;
    }
  }

  const mappedColumns = Object.values(columnMap);
  if (new Set(mappedColumns).size !== mappedColumns.length) {
    throw new Error(m.bulk_entry_parser_duplicate_columns());
  }
  const mappedIndexes = Object.keys(columnMap).map(Number);
  const rows: BulkRowState[] = [];

  for (let r = 1; r < rawData.length; r++) {
    const line = rawData[r] as unknown[];
    if (
      !line ||
      mappedIndexes.every((index) => !String(line[index] ?? "").trim())
    )
      continue;
    const excelRow = rowOffset + r + 1;

    const rowData: Record<string, unknown> = {
      __idx: rows.length + 1,
      rowNumber: excelRow,
      id: `import-row-${excelRow}`,
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
      const legacyEnglishName = [
        rowData.legacy_first_name,
        rowData.legacy_last_name,
      ]
        .filter((part) => String(part ?? "").trim())
        .join(" ");

      if (!rowData.full_name_ar && legacyArabicName)
        rowData.full_name_ar = legacyArabicName;
      if (!rowData.full_name_en && legacyEnglishName)
        rowData.full_name_en = legacyEnglishName;
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
      rowNumber: excelRow,
      data: rowData,
      errors: new Map(),
      warnings: new Map(),
      info: new Map(),
      status: "valid",
    });
  }

  if (!rows.length) throw new Error(m.bulk_entry_parser_empty_file());
  return {
    fileName,
    totalRowsCount: rows.length,
    rows,
    headers: rawHeaders,
  };
}
