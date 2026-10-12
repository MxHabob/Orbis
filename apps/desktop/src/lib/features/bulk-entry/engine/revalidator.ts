import * as m from "$lib/paraglide/messages";
import type { BulkImportProfile, BulkReferenceData, BulkRowState } from "../types";
import { createBulkValidationContext } from "./duplicate-index";

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
  const validationContext = createBulkValidationContext(
    profile.id,
    plainRows as Record<string, unknown>[],
  );

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
  const rowIssues = profile.validateRow(
    targetRow.data as T,
    plainRows,
    rowIndex,
    refData,
    validationContext,
  );
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
    targetRow.data.status = m.bulk_entry_status_error({ count: rowErrors.size });
  } else if (rowWarnings.size > 0) {
    targetRow.status = "warning";
    targetRow.data.status = m.bulk_entry_status_warning({ count: rowWarnings.size });
  } else {
    targetRow.status = "valid";
    targetRow.data.status = m.bulk_entry_status_valid();
  }

  // 3. A changed unique value can create or resolve a duplicate for any other row.
  if (columnId === "national_id" || columnId === "student_number") {
    for (let i = 0; i < rows.length; i++) {
      if (i === rowIndex) continue;
      const otherRow = rows[i]!;
      const duplicateIssue = profile
        .validateRow(
          otherRow.data as T,
          plainRows,
          i,
          refData,
          validationContext,
        )
        .find((issue) => issue.columnId === columnId);
      if (duplicateIssue) {
        otherRow.errors.set(columnId, duplicateIssue.message);
      } else {
        otherRow.errors.delete(columnId);
      }
      otherRow.data.__errors = otherRow.errors;
      otherRow.status = otherRow.errors.size > 0
        ? "error"
        : otherRow.warnings.size > 0
          ? "warning"
          : "valid";
      otherRow.data.status = otherRow.status === "error"
        ? m.bulk_entry_status_error({ count: otherRow.errors.size })
        : otherRow.status === "warning"
          ? m.bulk_entry_status_warning({ count: otherRow.warnings.size })
          : m.bulk_entry_status_valid();
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
