import { parseCsv } from "./csv";
import { parseExcel } from "./excel"; // You might need to adjust import once excel.ts is created
import type { ImportCsvResult } from "./csv";

export async function parseFile<T>(
  file: File, 
  options?: any
): Promise<ImportCsvResult<T> | { data: T[]; meta: { fields: string[] } }> {
  const fileType = file.type;
  const fileName = file.name.toLowerCase();

  if (
    fileType === "text/csv" || 
    fileType === "application/csv" || 
    fileType === "text/plain" || 
    fileName.endsWith(".csv")
  ) {
    return parseCsv<T>(file, options);
  }

  if (
    fileType === "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet" || 
    fileType === "application/vnd.ms-excel" ||
    fileName.endsWith(".xlsx") ||
    fileName.endsWith(".xls")
  ) {
    // For Excel, we ignore most csv options for now and assume strict tabular data
    return parseExcel<T>(file);
  }

  throw new Error("Unsupported file type. Please upload a CSV or Excel file.");
}
