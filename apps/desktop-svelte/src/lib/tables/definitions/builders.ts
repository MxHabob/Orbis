import type { OrbisColumnDef } from "@orbis/svelte-grid";
import { AvatarCell, BadgeCell } from "..";

// ═══════════════════════════════════════════════════════════════════════════
// Generic Column Builders
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Build a standard index column (row number)
 */
export function indexColumn(label = "#"): OrbisColumnDef {
  return {
    id: "__rowIndex",
    prop: "__rowIndex",
    header: label,
    name: label,
    width: 50,
    readonly: true,
    cellClass: "orbis-cell-index",
    sort: false
  };
}

/**
 * Build a selection checkbox column for multi-row selection.
 */
export function selectionColumn(): OrbisColumnDef {
  return {
    id: "__selected",
    prop: "__selected",
    header: "",
    name: "",
    width: 40,
    editor: "checkbox",
    sort: false
  };
}

/**
 * Build a student name column with avatar Svelte component
 */
export function studentNameColumn(
  opts: { label?: string; locale?: string; size?: number } = {},
): OrbisColumnDef {
  const { label = "Student", locale = "en", size = 180 } = opts;
  const colId = locale === "ar" ? "student_name_ar" : "student_name";
  return {
    id: colId,
    prop: colId,
    header: label,
    name: label,
    width: size,
    readonly: true,
    cell: AvatarCell,
    sort: true
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
    id: prop,
    prop,
    header: name,
    name,
    width: size,
    editor: readonly ? undefined : {
      type: "text",
      config: {
        min: 0,
        max: maxScore
      }
    },
    sort: true,
    cellClass: "font-mono text-center"
  };
}

/**
 * Build a read-only text column
 */
export function textColumn(opts: {
  prop: string;
  name: string;
  size?: number;
}): OrbisColumnDef {
  const { prop, name, size = 120 } = opts;
  return {
    id: prop,
    prop,
    header: name,
    name,
    width: size,
    sort: true
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
    id: prop,
    prop,
    header: name,
    name,
    width: size,
    cell: BadgeCell,
    sort: true
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
    id: prop,
    prop,
    header: name,
    name,
    width: size,
    sort: true,
    cellClass: "font-medium"
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

  for (const day of days) {
    cols.push({
      id: `day_${day}`,
      prop: `day_${day}`,
      header: day,
      name: day,
      width: 55,
      editor: readonly ? undefined : "text",
      sort: false
    });
  }

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
      id: "total_fees",
      prop: "total_fees",
      header: labels.totalFees || "Total Fees",
      name: labels.totalFees || "Total Fees",
      width: 110,
      readonly: true,
      sort: true
    },
    {
      id: "total_paid",
      prop: "total_paid",
      header: labels.paid || "Paid",
      name: labels.paid || "Paid",
      width: 100,
      readonly: true,
      sort: true
    },
    {
      id: "total_discount",
      prop: "total_discount",
      header: labels.discount || "Discount",
      name: labels.discount || "Discount",
      width: 100,
      readonly: true,
      sort: true
    },
    {
      id: "balance",
      prop: "balance",
      header: labels.balance || "Balance",
      name: labels.balance || "Balance",
      width: 110,
      readonly: true,
      sort: true
    },
    statusColumn({
      prop: "payment_status",
      name: labels.status || "Status",
      size: 90,
    }),
  ];
}
