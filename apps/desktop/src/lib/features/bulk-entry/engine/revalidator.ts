import type { BulkImportProfile, BulkReferenceData, BulkRowState } from "../types";

export interface RevalidationEvent {
  rowIndex: number;
  columnId: string;
  newValue: unknown;
}

export function revalidateCellChange<T = Record<string, unknown>>(
  rows: BulkRowState[],
  event: RevalidationEvent,
  profile: BulkImportProfile<T, any>,
  refData: BulkReferenceData,
): { updatedRows: BulkRowState[]; errorCount: number; warningCount: number; validCount: number } {
  const { rowIndex, columnId, newValue } = event;
  const targetRow = rows[rowIndex];
  if (!targetRow) return { updatedRows: rows, errorCount: 0, warningCount: 0, validCount: 0 };

  targetRow.data[columnId] = newValue;
  const plainRows = rows.map((r) => r.data as T);

  // 1. Revalidate the single cell
  const rowErrors = new Map<string, string>(targetRow.errors);
  const rowWarnings = new Map<string, string>(targetRow.warnings);
  rowErrors.delete(columnId);
  rowWarnings.delete(columnId);

  const cellIssue = profile.validateCell(columnId, newValue, targetRow.data as T, refData);
  if (cellIssue) {
    if (cellIssue.severity === "BLOCKING_ERROR") {
      rowErrors.set(columnId, cellIssue.message);
    } else if (cellIssue.severity === "WARNING") {
      rowWarnings.set(columnId, cellIssue.message);
    }
  }

  // 2. Row validation
  const rowIssues = profile.validateRow(targetRow.data as T, plainRows, rowIndex, refData);
  for (const issue of rowIssues) {
    if (issue.severity === "BLOCKING_ERROR") {
      rowErrors.set(issue.columnId, issue.message);
    } else if (issue.severity === "WARNING") {
      rowWarnings.set(issue.columnId, issue.message);
    }
  }

  targetRow.errors = rowErrors;
  targetRow.warnings = rowWarnings;
  targetRow.data.__errors = rowErrors;

  if (rowErrors.size > 0) {
    targetRow.status = "error";
    targetRow.data.status = `🔴 خطأ (${rowErrors.size})`;
  } else if (rowWarnings.size > 0) {
    targetRow.status = "warning";
    targetRow.data.status = `🟡 تنبيه (${rowWarnings.size})`;
  } else {
    targetRow.status = "valid";
    targetRow.data.status = "🟢 صالح";
  }

  // 3. If unique field changed (e.g. national_id), also recheck other rows that might have had duplicate errors
  if (columnId === "national_id" || columnId === "student_number") {
    for (let i = 0; i < rows.length; i++) {
      if (i === rowIndex) continue;
      const otherRow = rows[i]!;
      if (otherRow.errors.has(columnId)) {
        const otherIssues = profile.validateRow(otherRow.data as T, plainRows, i, refData);
        const hasDup = otherIssues.some((iss) => iss.columnId === columnId);
        if (!hasDup) {
          otherRow.errors.delete(columnId);
          otherRow.data.__errors = otherRow.errors;
          if (otherRow.errors.size === 0) {
            otherRow.status = otherRow.warnings.size > 0 ? "warning" : "valid";
            otherRow.data.status = otherRow.status === "warning" ? `🟡 تنبيه (${otherRow.warnings.size})` : "🟢 صالح";
          }
        }
      }
    }
  }

  let errorCount = 0;
  let warningCount = 0;
  let validCount = 0;

  for (const r of rows) {
    if (r.status === "error") errorCount++;
    else if (r.status === "warning") {
      warningCount++;
      validCount++;
    } else validCount++;
  }

  return {
    updatedRows: rows,
    errorCount,
    warningCount,
    validCount,
  };
}
