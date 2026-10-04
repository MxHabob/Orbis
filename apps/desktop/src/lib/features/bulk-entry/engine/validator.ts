import * as m from "$lib/paraglide/messages";
import type { BulkImportProfile, BulkReferenceData, BulkRowState } from "../types";

export interface ValidationSummary {
  totalRows: number;
  validCount: number;
  errorCount: number;
  warningCount: number;
  hasBlockingErrors: boolean;
}

export function validateAllRows<T = Record<string, unknown>>(
  rows: BulkRowState[],
  profile: BulkImportProfile<T, any>,
  refData: BulkReferenceData,
): ValidationSummary {
  const plainRows = rows.map((r) => r.data as T);
  let errorCount = 0;
  let warningCount = 0;
  let validCount = 0;

  for (let i = 0; i < rows.length; i++) {
    const row = rows[i]!;
    const rowErrors = new Map<string, string>();
    const rowWarnings = new Map<string, string>();
    const rowInfo = new Map<string, string>();

    // 1. Level 2: Per-cell validation
    for (const col of profile.columnMetadata) {
      const val = row.data[col.id];
      const issue = profile.validateCell(col.id, val, row.data as T, refData);
      if (issue) {
        if (issue.severity === "BLOCKING_ERROR") {
          rowErrors.set(col.id, issue.message);
        } else if (issue.severity === "WARNING") {
          rowWarnings.set(col.id, issue.message);
        } else {
          rowInfo.set(col.id, issue.message);
        }
      }
    }

    // 2. Level 3 & Level 4: Row & Cross-row validation
    const rowIssues = profile.validateRow(row.data as T, plainRows, i, refData);
    for (const issue of rowIssues) {
      if (issue.severity === "BLOCKING_ERROR") {
        rowErrors.set(issue.columnId, issue.message);
      } else if (issue.severity === "WARNING") {
        rowWarnings.set(issue.columnId, issue.message);
      } else {
        rowInfo.set(issue.columnId, issue.message);
      }
    }

    row.errors = rowErrors;
    row.warnings = rowWarnings;
    row.info = rowInfo;

    // Attach __errors directly into data row for OrbisGrid styling
    row.data.__errors = rowErrors;

    if (rowErrors.size > 0) {
      row.status = "error";
      row.data.status = m.bulk_entry_status_error({ count: rowErrors.size });
      errorCount++;
    } else if (rowWarnings.size > 0) {
      row.status = "warning";
      row.data.status = m.bulk_entry_status_warning({ count: rowWarnings.size });
      warningCount++;
      validCount++;
    } else {
      row.status = "valid";
      row.data.status = m.bulk_entry_status_valid();
      validCount++;
    }
  }

  return {
    totalRows: rows.length,
    validCount,
    errorCount,
    warningCount,
    hasBlockingErrors: errorCount > 0,
  };
}
