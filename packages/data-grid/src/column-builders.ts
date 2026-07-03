/**
 * @orbis/data-grid - Generic column/source helpers
 *
 * Reusable utilities for building OrbisGrid configurations
 * for common Orbis page patterns (students, attendance, fees, etc.)
 */

import type { OrbisColumnDef } from "./types";

// ═══════════════════════════════════════════════════════════════════════════
// Generic Column Builders
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Build a standard index column (row number)
 */
export function indexColumn(label = "#"): OrbisColumnDef {
  return {
    prop: "__rowIndex",
    name: label,
    size: 50,
    readonly: true,
    pin: "colPinStart",
    cellProperties: () => ({
      class: "orbis-cell-index",
    }),
  };
}

/**
 * Build a frozen student name column with RTL support
 */
export function studentNameColumn(
  opts: { label?: string; locale?: string; size?: number } = {},
): OrbisColumnDef {
  const { label = "Student", locale = "en", size = 200 } = opts;
  return {
    prop: locale === "ar" ? "student_name_ar" : "student_name",
    name: label,
    size,
    readonly: true,
    pin: "colPinStart",
    cellProperties: () => ({
      class: "orbis-cell-student",
    }),
  };
}

/**
 * Build a numeric score column with validation
 */
export function scoreColumn(opts: {
  prop: string;
  name: string;
  maxScore?: number;
  size?: number;
  readonly?: boolean;
}): OrbisColumnDef {
  const { prop, name, maxScore = 100, size = 90, readonly = false } = opts;
  return {
    prop,
    name,
    size,
    readonly,
    editor: readonly ? undefined : "number",
    cellProperties: (row: Record<string, unknown>) => {
      const val = Number(row[prop]);
      if (Number.isNaN(val)) return {};
      if (val < 0 || val > maxScore) return { class: "orbis-cell-error" };
      return {};
    },
    editorConfig: {
      min: 0,
      max: maxScore,
      step: 0.5,
    },
  };
}

/**
 * Build a read-only text column
 */
export function textColumn(opts: {
  prop: string;
  name: string;
  size?: number;
  pin?: "colPinStart" | "colPinEnd";
}): OrbisColumnDef {
  const { prop, name, size = 120, pin } = opts;
  return {
    prop,
    name,
    size,
    readonly: true,
    pin,
  };
}

/**
 * Build a badge-style status column
 */
export function statusColumn(opts: {
  prop: string;
  name: string;
  size?: number;
}): OrbisColumnDef {
  const { prop, name, size = 100 } = opts;
  return {
    prop,
    name,
    size,
    readonly: true,
    cellProperties: (row: Record<string, unknown>) => {
      const val = String(row[prop] || "");
      const cls =
        val === "active" || val === "present" || val === "paid"
          ? "orbis-cell-success"
          : val === "absent" || val === "failed" || val === "overdue"
            ? "orbis-cell-error"
            : "";
      return { class: cls };
    },
  };
}

/**
 * Build a percentage column with color coding
 */
export function percentageColumn(opts: {
  prop: string;
  name: string;
  size?: number;
}): OrbisColumnDef {
  const { prop, name, size = 80 } = opts;
  return {
    prop,
    name,
    size,
    readonly: true,
    cellProperties: (row: Record<string, unknown>) => {
      const val = Number(row[prop]);
      if (Number.isNaN(val)) return {};
      if (val >= 90) return { class: "orbis-cell-excellent" };
      if (val >= 70) return { class: "orbis-cell-good" };
      if (val >= 50) return { class: "orbis-cell-warning" };
      return { class: "orbis-cell-error" };
    },
  };
}

// ═══════════════════════════════════════════════════════════════════════════
// Source Transformers
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Add row index numbers to flat data array
 */
export function addRowIndices<T extends Record<string, unknown>>(
  data: T[],
): (T & { __rowIndex: number })[] {
  return data.map((row, i) => ({
    ...row,
    __rowIndex: i + 1,
  }));
}

/**
 * Flatten nested objects into a grid-friendly flat record.
 * e.g. { student: { name: "Ali" } } → { "student.name": "Ali" }
 */
export function flattenForGrid(
  data: Record<string, unknown>[],
  prefix = "",
): Record<string, unknown>[] {
  return data.map((row) => {
    const flat: Record<string, unknown> = {};
    for (const [key, val] of Object.entries(row)) {
      const fullKey = prefix ? `${prefix}.${key}` : key;
      if (val && typeof val === "object" && !Array.isArray(val)) {
        const nested = flattenForGrid(
          [val as Record<string, unknown>],
          fullKey,
        );
        Object.assign(flat, nested[0]);
      } else {
        flat[fullKey] = val;
      }
    }
    return flat;
  });
}

// ═══════════════════════════════════════════════════════════════════════════
// Attendance Grid Helpers
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Build columns for an attendance grid:
 * [#] [Student Name] [Day1] [Day2] ... [DayN] [Present Count] [Absent Count] [Rate %]
 */
export function buildAttendanceColumns(opts: {
  days: string[];
  readonly?: boolean;
  locale?: string;
  labels?: {
    rowIndex?: string;
    studentName?: string;
    present?: string;
    absent?: string;
    rate?: string;
  };
}): OrbisColumnDef[] {
  const { days, readonly = false, locale = "en", labels = {} } = opts;

  const cols: OrbisColumnDef[] = [
    indexColumn(labels.rowIndex || "#"),
    studentNameColumn({
      label: labels.studentName || "Student",
      locale,
      size: 180,
    }),
  ];

  // Day columns
  for (const day of days) {
    cols.push({
      prop: `day_${day}`,
      name: day,
      size: 45,
      readonly,
      cellProperties: (row: Record<string, unknown>) => {
        const val = String(row[`day_${day}`] || "");
        if (val === "P" || val === "present")
          return { class: "orbis-cell-success" };
        if (val === "A" || val === "absent")
          return { class: "orbis-cell-error" };
        if (val === "L" || val === "late")
          return { class: "orbis-cell-warning" };
        return {};
      },
    });
  }

  // Summary columns
  cols.push(
    textColumn({
      prop: "__presentCount",
      name: labels.present || "✓",
      size: 50,
    }),
    textColumn({ prop: "__absentCount", name: labels.absent || "✗", size: 50 }),
    percentageColumn({
      prop: "__attendanceRate",
      name: labels.rate || "%",
      size: 60,
    }),
  );

  return cols;
}

// ═══════════════════════════════════════════════════════════════════════════
// Student List Grid Helpers
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Build columns for a student list/directory view:
 * [#] [Name] [Class] [Status] [Phone] [Email] [Guardian]
 */
export function buildStudentListColumns(opts: {
  locale?: string;
  labels?: {
    rowIndex?: string;
    name?: string;
    class?: string;
    status?: string;
    phone?: string;
    email?: string;
    guardian?: string;
  };
}): OrbisColumnDef[] {
  const { locale = "en", labels = {} } = opts;

  return [
    indexColumn(labels.rowIndex || "#"),
    studentNameColumn({
      label: labels.name || "Student",
      locale,
      size: 200,
    }),
    textColumn({
      prop: "class_name",
      name: labels.class || "Class",
      size: 120,
    }),
    statusColumn({ prop: "status", name: labels.status || "Status", size: 90 }),
    textColumn({ prop: "phone", name: labels.phone || "Phone", size: 130 }),
    textColumn({ prop: "email", name: labels.email || "Email", size: 180 }),
    textColumn({
      prop: "guardian_name",
      name: labels.guardian || "Guardian",
      size: 150,
    }),
  ];
}

// ═══════════════════════════════════════════════════════════════════════════
// Fees Grid Helpers
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Build columns for a fees/payments summary grid:
 * [#] [Student] [Total Fees] [Paid] [Discount] [Balance] [Status]
 */
export function buildFeesColumns(opts: {
  locale?: string;
  currencySymbol?: string;
  labels?: {
    rowIndex?: string;
    student?: string;
    totalFees?: string;
    paid?: string;
    discount?: string;
    balance?: string;
    status?: string;
  };
}): OrbisColumnDef[] {
  const { locale = "en", labels = {} } = opts;

  return [
    indexColumn(labels.rowIndex || "#"),
    studentNameColumn({
      label: labels.student || "Student",
      locale,
      size: 200,
    }),
    {
      prop: "total_fees",
      name: labels.totalFees || "Total Fees",
      size: 110,
      readonly: true,
      cellProperties: () => ({ class: "orbis-cell-number" }),
    },
    {
      prop: "total_paid",
      name: labels.paid || "Paid",
      size: 100,
      readonly: true,
      cellProperties: () => ({ class: "orbis-cell-success" }),
    },
    {
      prop: "total_discount",
      name: labels.discount || "Discount",
      size: 100,
      readonly: true,
    },
    {
      prop: "balance",
      name: labels.balance || "Balance",
      size: 110,
      readonly: true,
      cellProperties: (row: Record<string, unknown>) => {
        const val = Number(row.balance);
        if (val > 0) return { class: "orbis-cell-error" };
        return { class: "orbis-cell-success" };
      },
    },
    statusColumn({
      prop: "payment_status",
      name: labels.status || "Status",
      size: 90,
    }),
  ];
}
