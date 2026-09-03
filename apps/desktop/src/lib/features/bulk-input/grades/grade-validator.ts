/**
 * @module grade-validator
 * Client-side validation for bulk grade import.
 */

import * as m from "$lib/paraglide/messages";

export interface RowValidation {
  rowIndex: number;
  isValid: boolean;
  errors: Map<string, string>;
}

/**
 * Validate all grade rows.
 */
export function validateGradeRows(
  rows: Record<string, unknown>[],
): RowValidation[] {
  const results: RowValidation[] = [];

  for (let i = 0; i < rows.length; i++) {
    const row = rows[i];
    const errors = new Map<string, string>();

    if (isRowEmpty(row)) {
      row.__errors = errors;
      results.push({ rowIndex: i, isValid: true, errors });
      continue;
    }

    // ── student_identifier: required ──
    const sid = str(row.student_identifier);
    if (!sid) {
      errors.set(
        "student_identifier",
        m.common_validation_student_id_required(),
      );
    }

    // ── subject_name: required ──
    const subj = str(row.subject_name);
    if (!subj) {
      errors.set("subject_name", m.common_validation_subject_name_required());
    }

    // ── score: required, numeric, 0 ≤ score ≤ max_score ──
    const scoreStr = str(row.score);
    const maxScore = parseFloat(str(row.max_score) || "100") || 100;

    if (!scoreStr) {
      errors.set("score", m.common_validation_score_required());
    } else {
      const score = parseFloat(scoreStr);
      if (isNaN(score)) {
        errors.set("score", m.common_validation_score_number());
      } else if (score < 0 || score > maxScore) {
        errors.set("score", m.common_validation_score_range({ maxScore }));
      }
    }

    // ── max_score: if present, > 0 ──
    const maxStr = str(row.max_score);
    if (maxStr) {
      const ms = parseFloat(maxStr);
      if (isNaN(ms) || ms <= 0) {
        errors.set("max_score", m.common_validation_max_score_positive());
      }
    }

    // ── semester: required ──
    const sem = str(row.semester);
    if (!sem) {
      errors.set("semester", m.common_validation_semester_required());
    }

    row.__errors = errors;
    results.push({ rowIndex: i, isValid: errors.size === 0, errors });
  }

  return results;
}

export function getGradeValidationSummary(
  results: RowValidation[],
  rows: Record<string, unknown>[],
) {
  let totalFilled = 0;
  let valid = 0;
  let errorCount = 0;
  const errorDetails: Array<{
    rowIndex: number;
    field: string;
    message: string;
  }> = [];

  for (const r of results) {
    if (isRowEmpty(rows[r.rowIndex])) continue;
    totalFilled++;
    if (r.isValid) {
      valid++;
    } else {
      errorCount++;
      for (const [field, message] of r.errors) {
        errorDetails.push({ rowIndex: r.rowIndex + 1, field, message });
      }
    }
  }

  return { totalFilled, valid, errorCount, errorDetails };
}

function str(v: unknown): string {
  return String(v ?? "").trim();
}

function isRowEmpty(row: Record<string, unknown>): boolean {
  for (const [k, v] of Object.entries(row)) {
    if (k.startsWith("__")) continue;
    if (v !== "" && v !== null && v !== undefined) return false;
  }
  return true;
}
