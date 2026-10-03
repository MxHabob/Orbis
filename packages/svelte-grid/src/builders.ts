import type { OrbisColumnDef } from "./types";

// ═══════════════════════════════════════════════════════════════════════════
// Canonical Grid Column Builders (Native to @orbis/svelte-grid)
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Build a standard frozen index / row-number column.
 */
export function indexColumn(label = "#"): OrbisColumnDef {
  return {
    id: "__idx",
    prop: "__idx",
    header: label,
    name: label,
    width: 48,
    size: 48,
    readonly: true,
    pin: "left",
    align: "center",
    sort: false,
    cellClass: "font-mono text-muted-foreground",
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
    size: 40,
    editor: "checkbox",
    type: "boolean",
    align: "center",
    sort: false,
  };
}

/**
 * Build an avatar/name column that auto-renders avatar image and initials.
 */
export function studentNameColumn(
  opts: { label?: string; locale?: string; size?: number; prop?: string } = {},
): OrbisColumnDef {
  const { label = "Student", locale = "en", size = 200, prop } = opts;
  const colId = prop || (locale === "ar" ? "student_name_ar" : "student_name");
  return {
    id: colId,
    prop: colId,
    header: label,
    name: label,
    width: size,
    size,
    type: "avatar",
    readonly: true,
    pin: "left",
    sort: true,
  };
}

/**
 * Build a numeric score column with formatting and validation.
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
    size,
    type: "number",
    editor: readonly
      ? undefined
      : {
          type: "text",
          config: { min: 0, max: maxScore },
        },
    sort: true,
    align: "end",
    cellClass: "font-mono",
  };
}

/**
 * Build a standard text column.
 */
export function textColumn(opts: {
  prop: string;
  name: string;
  size?: number;
  align?: "start" | "center" | "end";
}): OrbisColumnDef {
  const { prop, name, size = 120, align = "start" } = opts;
  return {
    id: prop,
    prop,
    header: name,
    name,
    width: size,
    size,
    type: "text",
    align,
    sort: true,
  };
}

/**
 * Build a color-coded status badge column.
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
    size,
    type: "badge",
    align: "center",
    sort: true,
  };
}

/**
 * Build a percentage column.
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
    size,
    type: "percentage",
    align: "end",
    sort: true,
    cellClass: "font-medium",
  };
}

// ═══════════════════════════════════════════════════════════════════════════
// Source Transformers
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Add row index numbers to data array
 */
export function withIndex<T extends Record<string, unknown>>(
  data: T[],
): (T & { __idx: number; __rowIndex: number })[] {
  return data.map((row, i) => ({
    ...row,
    __idx: i + 1,
    __rowIndex: i + 1,
  }));
}

/**
 * Add row index numbers to flat data array
 */
export function addRowIndices<T extends Record<string, unknown>>(
  data: T[],
): (T & { __rowIndex: number; __idx: number })[] {
  return withIndex(data);
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
      sort: false,
    });
  }

  cols.push(
    textColumn({
      prop: "__presentCount",
      name: labels.present || "✓",
      size: 50,
      align: "center",
    }),
    textColumn({
      prop: "__absentCount",
      name: labels.absent || "✗",
      size: 50,
      align: "center",
    }),
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
      type: "currency",
      readonly: true,
      sort: true,
    },
    {
      id: "total_paid",
      prop: "total_paid",
      header: labels.paid || "Paid",
      name: labels.paid || "Paid",
      width: 100,
      type: "currency",
      readonly: true,
      sort: true,
    },
    {
      id: "total_discount",
      prop: "total_discount",
      header: labels.discount || "Discount",
      name: labels.discount || "Discount",
      width: 100,
      type: "currency",
      readonly: true,
      sort: true,
    },
    {
      id: "balance",
      prop: "balance",
      header: labels.balance || "Balance",
      name: labels.balance || "Balance",
      width: 110,
      type: "currency",
      readonly: true,
      sort: true,
    },
    statusColumn({
      prop: "payment_status",
      name: labels.status || "Status",
      size: 90,
    }),
  ];
}
