/**
 * @module student-validator
 * Client-side validation for bulk student import.
 * Mirrors the business rules in create_user_in_org() and create_enrollment().
 */

export interface RowValidation {
  rowIndex: number;
  isValid: boolean;
  errors: Map<string, string>;
}

const EMAIL_RE = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
const DATE_RE = /^\d{4}-\d{2}-\d{2}$/;
const VALID_SEX = ["male", "female"];
const VALID_BLOOD = ["A+", "A-", "B+", "B-", "AB+", "AB-", "O+", "O-"];
const VALID_STATUS = ["draft", "pending_review", "active", "approved"];

/**
 * Validate all student rows. Returns per-row results and applies
 * __errors Map on each row for cell-level styling.
 */
export function validateStudentRows(
  rows: Record<string, unknown>[],
  availableClassIds?: Set<string>,
): RowValidation[] {
  // Pre-collect for batch duplicate detection
  const emailIndex = new Map<string, number[]>();
  const nidIndex = new Map<string, number[]>();

  rows.forEach((row, idx) => {
    const email = str(row.email).toLowerCase();
    if (email) {
      if (!emailIndex.has(email)) emailIndex.set(email, []);
      emailIndex.get(email)!.push(idx);
    }
    const nid = str(row.national_id);
    if (nid) {
      if (!nidIndex.has(nid)) nidIndex.set(nid, []);
      nidIndex.get(nid)!.push(idx);
    }
  });

  const results: RowValidation[] = [];

  for (let i = 0; i < rows.length; i++) {
    const row = rows[i];
    const errors = new Map<string, string>();

    // Skip completely empty rows
    if (isRowEmpty(row)) {
      row.__errors = errors;
      results.push({ rowIndex: i, isValid: true, errors });
      continue;
    }

    // ── full_name: required, ≥2 chars ──
    const name = str(row.full_name);
    if (!name) {
      errors.set("full_name", "الاسم مطلوب");
    } else if (name.length < 2) {
      errors.set("full_name", "الاسم يجب أن يكون حرفان على الأقل");
    }

    // ── email: required, valid format, no batch duplicates ──
    const email = str(row.email).toLowerCase();
    if (!email) {
      errors.set("email", "البريد الإلكتروني مطلوب");
    } else if (!EMAIL_RE.test(email)) {
      errors.set("email", "صيغة البريد غير صحيحة");
    } else {
      const dups = emailIndex.get(email);
      if (dups && dups.length > 1) {
        errors.set("email", "بريد إلكتروني مكرر في هذه الدفعة");
      }
    }

    // ── phone: optional, ≥7 digits ──
    const phone = str(row.phone);
    if (phone && phone.replace(/[\s\-+()]/g, "").length < 7) {
      errors.set("phone", "رقم الهاتف غير صحيح (7 أرقام على الأقل)");
    }

    // ── national_id: optional, no batch duplicates ──
    const nid = str(row.national_id);
    if (nid) {
      const nidDups = nidIndex.get(nid);
      if (nidDups && nidDups.length > 1) {
        errors.set("national_id", "رقم وطني مكرر في هذه الدفعة");
      }
    }

    // ── sex: optional, must be male/female ──
    const sex = str(row.sex).toLowerCase();
    if (sex && !VALID_SEX.includes(sex)) {
      errors.set("sex", "الجنس يجب أن يكون male أو female");
    }

    // ── date_of_birth: optional, YYYY-MM-DD ──
    const dob = str(row.date_of_birth);
    if (dob && !DATE_RE.test(dob)) {
      errors.set("date_of_birth", "التاريخ بصيغة YYYY-MM-DD");
    }

    // ── class_id: validate against available classes ──
    const classId = str(row.class_id);
    if (classId && availableClassIds && !availableClassIds.has(classId)) {
      errors.set("class_id", "الفصل غير موجود في النظام");
    }

    // ── enrollment_status: optional, valid values ──
    const status = str(row.enrollment_status).toLowerCase();
    if (status && !VALID_STATUS.includes(status)) {
      errors.set("enrollment_status", `الحالة غير صحيحة. المتاح: ${VALID_STATUS.join(", ")}`);
    }

    // ── blood_type: optional, valid values ──
    const blood = str(row.blood_type).toUpperCase();
    if (blood && !VALID_BLOOD.includes(blood)) {
      errors.set("blood_type", `فصيلة الدم غير صحيحة. المتاح: ${VALID_BLOOD.join(", ")}`);
    }

    // Apply to row for cell styling
    row.__errors = errors;
    results.push({ rowIndex: i, isValid: errors.size === 0, errors });
  }

  return results;
}

/**
 * Summary stats from validation results.
 */
export function getStudentValidationSummary(
  results: RowValidation[],
  rows: Record<string, unknown>[],
) {
  let totalFilled = 0;
  let valid = 0;
  let errorCount = 0;
  const errorDetails: Array<{ rowIndex: number; field: string; message: string }> = [];

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

// ── Helpers ──
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
