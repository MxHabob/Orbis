import { open } from "@tauri-apps/plugin-dialog";
import { readFile } from "@tauri-apps/plugin-fs";
import type { BulkImportProfile, BulkRowState } from "../types";

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
      filters: [{ name: "Spreadsheet Files", extensions: ["xlsx", "xls", "csv"] }],
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
    "Sheet1";

  const worksheet = workbook.Sheets[targetSheetName];
  if (!worksheet) {
    throw new Error("لم يتم العثور على ورقة بيانات صالحة في الملف");
  }

  // Convert worksheet to raw array of rows
  const rawData: unknown[][] = XLSX.utils.sheet_to_json(worksheet, {
    header: 1,
    defval: "",
    raw: false,
    dateNF: "YYYY-MM-DD",
  });

  if (rawData.length < 2) {
    throw new Error("الملف فارغ أو لا يحتوي على صف ترويسة وبيانات صالحة");
  }

  const rawHeaders = (rawData[0] as string[]).map((h) => String(h || "").trim());
  const columnMap: Record<number, string> = {};

  // Map header column indices to profile column metadata IDs
  for (let c = 0; c < rawHeaders.length; c++) {
    const headerText = rawHeaders[c]!;
    if (!headerText) continue;

    const matchedMeta = profile.columnMetadata.find(
      (col) =>
        col.headerAr.toLowerCase() === headerText.toLowerCase() ||
        col.headerAr.replace(/\*/g, "").trim().toLowerCase() === headerText.replace(/\*/g, "").trim().toLowerCase() ||
        col.headerEn.toLowerCase() === headerText.toLowerCase() ||
        col.headerEn.replace(/\*/g, "").trim().toLowerCase() === headerText.replace(/\*/g, "").trim().toLowerCase() ||
        col.id.toLowerCase() === headerText.toLowerCase(),
    );

    if (matchedMeta) {
      columnMap[c] = matchedMeta.id;
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
