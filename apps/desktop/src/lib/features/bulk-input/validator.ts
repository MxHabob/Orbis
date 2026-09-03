/**
 * @module bulk-input-validator
 * Client-side validation logic for bulk input rows.
 * Runs in real-time as users edit cells or paste data.
 */

import type { BulkInputType } from "./config";
import * as m from "$lib/paraglide/messages";

export interface RowValidationResult {
  rowIndex: number;
  isValid: boolean;
  errors: Map<string, string>;
}

const EMAIL_REGEX = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
const DATE_REGEX = /^\d{4}-\d{2}-\d{2}$/;

/**
 * Validate all rows for a given bulk input type.
 * Returns per-row validation results with field-level error messages.
 */
export function validateRows(
  type: BulkInputType,
  rows: Record<string, unknown>[],
): RowValidationResult[] {
  const results: RowValidationResult[] = [];

  // Collect emails and national_ids for duplicate detection within the batch
  const emailCount = new Map<string, number[]>();
  const nidCount = new Map<string, number[]>();

  rows.forEach((row, idx) => {
    const email = String(row.email ?? row.student_email ?? "")
      .trim()
      .toLowerCase();
    if (email) {
      if (!emailCount.has(email)) emailCount.set(email, []);
      emailCount.get(email)!.push(idx);
    }
    const nid = String(row.national_id ?? "").trim();
    if (nid) {
      if (!nidCount.has(nid)) nidCount.set(nid, []);
      nidCount.get(nid)!.push(idx);
    }
  });

  for (let i = 0; i < rows.length; i++) {
    const row = rows[i];
    const errors = new Map<string, string>();

    // Skip completely empty rows
    if (isRowEmpty(row)) {
      results.push({ rowIndex: i, isValid: true, errors });
      continue;
    }

    switch (type) {
      case "students":
      case "teachers":
        validateUserRow(row, i, errors, emailCount, nidCount);
        break;
      case "grades":
        validateGradeRow(row, i, errors, emailCount);
        break;
    }

    results.push({
      rowIndex: i,
      isValid: errors.size === 0,
      errors,
    });
  }

  return results;
}

function isRowEmpty(row: Record<string, unknown>): boolean {
  for (const [key, value] of Object.entries(row)) {
    if (key.startsWith("__")) continue; // Skip internal fields
    if (value !== "" && value !== null && value !== undefined) return false;
  }
  return true;
}

function validateUserRow(
  row: Record<string, unknown>,
  idx: number,
  errors: Map<string, string>,
  emailCount: Map<string, number[]>,
  nidCount: Map<string, number[]>,
): void {
  // full_name: required, min 2 chars
  const name = String(row.full_name ?? "").trim();
  if (!name) {
    errors.set("full_name", m.common_validation_name_required());
  } else if (name.length < 2) {
    errors.set("full_name", m.common_validation_name_length());
  }

  // email: required, valid format, no duplicates
  const email = String(row.email ?? "")
    .trim()
    .toLowerCase();
  if (!email) {
    errors.set("email", m.common_validation_email_required());
  } else if (!EMAIL_REGEX.test(email)) {
    errors.set("email", m.common_validation_email_format());
  } else {
    const duplicates = emailCount.get(email);
    if (duplicates && duplicates.length > 1) {
      errors.set("email", m.common_validation_email_duplicate());
    }
  }

  // phone: if present, basic check (at least 7 digits)
  const phone = String(row.phone ?? "").trim();
  if (phone && phone.replace(/[\s\-+()]/g, "").length < 7) {
    errors.set("phone", m.common_validation_phone_format());
  }

  // national_id: if present, check duplicates
  const nid = String(row.national_id ?? "").trim();
  if (nid) {
    const nidDups = nidCount.get(nid);
    if (nidDups && nidDups.length > 1) {
      errors.set("national_id", m.common_validation_nid_duplicate());
    }
  }

  // sex: if present, must be male or female
  const sex = String(row.sex ?? "")
    .trim()
    .toLowerCase();
  if (sex && sex !== "male" && sex !== "female") {
    errors.set("sex", m.common_validation_sex_invalid());
  }

  // date_of_birth: if present, must be YYYY-MM-DD
  const dob = String(row.date_of_birth ?? "").trim();
  if (dob && !DATE_REGEX.test(dob)) {
    errors.set("date_of_birth", m.common_validation_dob_format());
  }
}

function validateGradeRow(
  row: Record<string, unknown>,
  idx: number,
  errors: Map<string, string>,
  emailCount: Map<string, number[]>,
): void {
  // student_email: required, valid format
  const email = String(row.student_email ?? "")
    .trim()
    .toLowerCase();
  if (!email) {
    errors.set("student_email", m.common_validation_student_email_required());
  } else if (!EMAIL_REGEX.test(email)) {
    errors.set("student_email", m.common_validation_email_format());
  }

  // subject_name: required
  const subject = String(row.subject_name ?? "").trim();
  if (!subject) {
    errors.set("subject_name", m.common_validation_subject_name_required());
  }

  // score: required, number, 0 ≤ score ≤ max_score
  const scoreStr = String(row.score ?? "").trim();
  const maxScore = parseFloat(String(row.max_score ?? "100").trim()) || 100;

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

  // max_score: if present, must be > 0
  const maxScoreStr = String(row.max_score ?? "").trim();
  if (maxScoreStr) {
    const ms = parseFloat(maxScoreStr);
    if (isNaN(ms) || ms <= 0) {
      errors.set("max_score", m.common_validation_max_score_positive());
    }
  }

  // semester: required
  const semester = String(row.semester ?? "").trim();
  if (!semester) {
    errors.set("semester", m.common_validation_semester_required());
  }
}

/**
 * Count summary statistics from validation results.
 */
export function getValidationSummary(
  results: RowValidationResult[],
  rows: Record<string, unknown>[],
): {
  totalFilledRows: number;
  validCount: number;
  errorCount: number;
  errorDetails: Array<{ rowIndex: number; field: string; message: string }>;
} {
  let totalFilledRows = 0;
  let validCount = 0;
  let errorCount = 0;
  const errorDetails: Array<{
    rowIndex: number;
    field: string;
    message: string;
  }> = [];

  for (const result of results) {
    // Skip empty rows from stats
    if (isRowEmpty(rows[result.rowIndex])) continue;
    totalFilledRows++;

    if (result.isValid) {
      validCount++;
    } else {
      errorCount++;
      for (const [field, message] of result.errors) {
        errorDetails.push({
          rowIndex: result.rowIndex + 1, // 1-indexed for display
          field,
          message,
        });
      }
    }
  }

  return { totalFilledRows, validCount, errorCount, errorDetails };
}
