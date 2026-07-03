/**
 * @orbis/desktop — Table Definitions
 *
 * Centralized column definitions for all system tables.
 * Each function returns OrbisColumnDef[] configured for a specific entity.
 *
 * Usage:
 *   import { students } from "$lib/tables/definitions";
 *   const cols = students.columns("ar");
 */

import type { OrbisColumnDef, ActionItem } from "@orbis/data-grid";
import { avatarRenderer, badgeRenderer, actionsRenderer } from "../renderers";

// ═══════════════════════════════════════════════════════════════════════════
// Shared column factories
// ═══════════════════════════════════════════════════════════════════════════

/** Frozen row-number column */
function idx(label = "#"): OrbisColumnDef {
  return {
    prop: "__idx",
    name: label,
    size: 48,
    readonly: true,
    pin: "colPinStart",
    cellProperties: () => ({ class: "orbis-cell-index" }),
  };
}

/** Frozen name column */
function nameCol(prop: string, label: string, size = 180): OrbisColumnDef {
  return {
    prop,
    name: label,
    size,
    readonly: true,
    pin: "colPinStart",
    cellProperties: () => ({ class: "orbis-cell-student" }),
  };
}

/** Frozen user/student name column with avatar renderer */
function userCol(
  prop: string,
  label: string,
  size = 180,
  srcKey = "avatar_url",
): OrbisColumnDef {
  return {
    prop,
    name: label,
    size,
    readonly: true,
    pin: "colPinStart",
    cellRenderer: avatarRenderer({
      srcKey,
      nameKey: prop,
      size: "md",
      showName: true,
    }),
  };
}

/** Standard read-only text column */
function text(prop: string, name: string, size = 120): OrbisColumnDef {
  return { prop, name, size, readonly: true };
}

/** Monospace number column */
function num(prop: string, name: string, size = 90): OrbisColumnDef {
  return {
    prop,
    name,
    size,
    readonly: true,
    cellProperties: () => ({ class: "orbis-cell-number" }),
  };
}

/** Color-coded status column */
function status(prop: string, name: string, size = 100): OrbisColumnDef {
  return {
    prop,
    name,
    size,
    readonly: true,
    cellRenderer: badgeRenderer({
      variantMap: {
        active: "success",
        present: "success",
        paid: "success",
        approved: "success",
        open: "success",
        inactive: "error",
        absent: "error",
        overdue: "error",
        failed: "error",
        closed: "error",
        locked: "error",
        withdrawn: "error",
        pending: "warning",
        late: "warning",
        partial: "warning",
      },
    }),
  };
}

/** Percentage column with grade coloring */
function pct(prop: string, name: string, size = 80): OrbisColumnDef {
  return {
    prop,
    name,
    size,
    readonly: true,
    cellProperties: (row: Record<string, unknown>) => {
      const v = Number(row[prop]);
      if (isNaN(v)) return {};
      if (v >= 90) return { class: "orbis-cell-excellent" };
      if (v >= 70) return { class: "orbis-cell-good" };
      if (v >= 50) return { class: "orbis-cell-warning" };
      return { class: "orbis-cell-error" };
    },
  };
}

/** Date column */
function date(prop: string, name: string, size = 110): OrbisColumnDef {
  return { prop, name, size, readonly: true };
}

// ═══════════════════════════════════════════════════════════════════════════
// Helper: add __idx to source data
// ═══════════════════════════════════════════════════════════════════════════

export function withIndex<T extends Record<string, unknown>>(
  data: T[],
): (T & { __idx: number })[] {
  return data.map((row, i) => ({ ...row, __idx: i + 1 }));
}

// ═══════════════════════════════════════════════════════════════════════════
// SETTINGS Tables
// ═══════════════════════════════════════════════════════════════════════════

export const users = {
  columns: (
    labels: {
      idx?: string;
      name?: string;
      email?: string;
      phone?: string;
      role?: string;
      status?: string;
      lastLogin?: string;
      actions?: string;
    } = {},
    actionsList?: ActionItem[],
  ): OrbisColumnDef[] => {
    const cols = [
      idx(labels.idx),
      userCol("full_name", labels.name || "Name", 180, "avatar_url"),
      text("email", labels.email || "Email", 200),
      text("phone", labels.phone || "Phone", 130),
      text("role", labels.role || "Role", 100),
      status("status", labels.status || "Status", 90),
      date("last_login_at", labels.lastLogin || "Last Login"),
    ];

    if (actionsList && actionsList.length > 0) {
      cols.push({
        prop: "__actions",
        name: labels.actions || "",
        size: 90,
        readonly: true,
        cellRenderer: actionsRenderer({ items: actionsList }),
      });
    }

    return cols;
  },
};

export const rooms = {
  columns: (
    labels: {
      idx?: string;
      name?: string;
      building?: string;
      floor?: string;
      capacity?: string;
      type?: string;
      status?: string;
    } = {},
  ): OrbisColumnDef[] => [
    idx(labels.idx),
    nameCol("name", labels.name || "Room", 160),
    text("building", labels.building || "Building", 120),
    text("floor", labels.floor || "Floor", 80),
    num("capacity", labels.capacity || "Capacity", 90),
    text("room_type", labels.type || "Type", 100),
    status("status", labels.status || "Status", 90),
  ],
};

export const units = {
  columns: (
    labels: {
      idx?: string;
      name?: string;
      type?: string;
      parent?: string;
      head?: string;
      status?: string;
    } = {},
  ): OrbisColumnDef[] => [
    idx(labels.idx),
    nameCol("name", labels.name || "Unit", 180),
    text("unit_type", labels.type || "Type", 110),
    text("parent_name", labels.parent || "Parent", 140),
    text("head_name", labels.head || "Head", 140),
    status("status", labels.status || "Status", 90),
  ],
};

export const currencies = {
  columns: (
    labels: {
      idx?: string;
      code?: string;
      name?: string;
      symbol?: string;
      rate?: string;
      isBase?: string;
    } = {},
  ): OrbisColumnDef[] => [
    idx(labels.idx),
    nameCol("code", labels.code || "Code", 80),
    text("name", labels.name || "Currency", 160),
    text("symbol", labels.symbol || "Symbol", 70),
    num("exchange_rate", labels.rate || "Rate", 100),
    status("is_base", labels.isBase || "Base", 70),
  ],
};

export const permissions = {
  columns: (
    labels: {
      idx?: string;
      user?: string;
      role?: string;
      module?: string;
      access?: string;
    } = {},
  ): OrbisColumnDef[] => [
    idx(labels.idx),
    nameCol("user_name", labels.user || "User", 180),
    text("role", labels.role || "Role", 120),
    text("module", labels.module || "Module", 140),
    text("access_level", labels.access || "Access", 100),
  ],
};

export const feeTemplates = {
  columns: (
    labels: {
      idx?: string;
      name?: string;
      amount?: string;
      frequency?: string;
      status?: string;
      classes?: string;
    } = {},
  ): OrbisColumnDef[] => [
    idx(labels.idx),
    nameCol("name", labels.name || "Template", 200),
    num("amount", labels.amount || "Amount", 110),
    text("frequency", labels.frequency || "Frequency", 110),
    status("status", labels.status || "Status", 90),
    text("class_count", labels.classes || "Classes", 80),
  ],
};

export const discountTemplates = {
  columns: (
    labels: {
      idx?: string;
      name?: string;
      type?: string;
      value?: string;
      status?: string;
    } = {},
  ): OrbisColumnDef[] => [
    idx(labels.idx),
    nameCol("name", labels.name || "Template", 200),
    text("discount_type", labels.type || "Type", 100),
    num("value", labels.value || "Value", 100),
    status("status", labels.status || "Status", 90),
  ],
};

export const auditLogs = {
  columns: (
    labels: {
      idx?: string;
      user?: string;
      action?: string;
      entity?: string;
      details?: string;
      timestamp?: string;
      ip?: string;
    } = {},
  ): OrbisColumnDef[] => [
    idx(labels.idx),
    date("created_at", labels.timestamp || "Time", 150),
    nameCol("user_name", labels.user || "User", 150),
    text("action", labels.action || "Action", 110),
    text("entity_type", labels.entity || "Entity", 120),
    text("details", labels.details || "Details", 250),
    text("ip_address", labels.ip || "IP", 110),
  ],
};

// ═══════════════════════════════════════════════════════════════════════════
// FINANCE Tables
// ═══════════════════════════════════════════════════════════════════════════

export const payments = {
  columns: (
    labels: {
      idx?: string;
      student?: string;
      amount?: string;
      method?: string;
      status?: string;
      date?: string;
      receipt?: string;
    } = {},
  ): OrbisColumnDef[] => [
    idx(labels.idx),
    date("created_at", labels.date || "Date", 110),
    userCol("student_name", labels.student || "Student", 180, "student_avatar"),
    num("amount", labels.amount || "Amount", 110),
    text("payment_method", labels.method || "Method", 110),
    status("status", labels.status || "Status", 90),
    text("receipt_number", labels.receipt || "Receipt #", 120),
  ],
};

export const fees = {
  columns: (
    labels: {
      idx?: string;
      student?: string;
      template?: string;
      total?: string;
      paid?: string;
      balance?: string;
      status?: string;
    } = {},
  ): OrbisColumnDef[] => [
    idx(labels.idx),
    userCol("student_name", labels.student || "Student", 180, "student_avatar"),
    text("template_name", labels.template || "Fee", 160),
    num("total_amount", labels.total || "Total", 100),
    {
      prop: "paid_amount",
      name: labels.paid || "Paid",
      size: 100,
      readonly: true,
      cellProperties: () => ({ class: "orbis-cell-success orbis-cell-number" }),
    },
    {
      prop: "balance",
      name: labels.balance || "Balance",
      size: 100,
      readonly: true,
      cellProperties: (row: Record<string, unknown>) => {
        const v = Number(row.balance);
        return {
          class: `orbis-cell-number ${v > 0 ? "orbis-cell-error" : "orbis-cell-success"}`,
        };
      },
    },
    status("payment_status", labels.status || "Status", 90),
  ],
};

export const ledger = {
  columns: (
    labels: {
      idx?: string;
      code?: string;
      name?: string;
      type?: string;
      debit?: string;
      credit?: string;
      balance?: string;
    } = {},
  ): OrbisColumnDef[] => [
    idx(labels.idx),
    text("code", labels.code || "Code", 80),
    nameCol("name", labels.name || "Account", 200),
    text("account_type", labels.type || "Type", 110),
    num("total_debit", labels.debit || "Debit", 110),
    num("total_credit", labels.credit || "Credit", 110),
    num("balance", labels.balance || "Balance", 110),
  ],
};

export const journalEntries = {
  columns: (
    labels: {
      idx?: string;
      date?: string;
      ref?: string;
      description?: string;
      debit?: string;
      credit?: string;
      createdBy?: string;
    } = {},
  ): OrbisColumnDef[] => [
    idx(labels.idx),
    date("entry_date", labels.date || "Date", 110),
    text("reference", labels.ref || "Ref #", 100),
    text("description", labels.description || "Description", 250),
    num("total_debit", labels.debit || "Debit", 110),
    num("total_credit", labels.credit || "Credit", 110),
    text("created_by_name", labels.createdBy || "By", 120),
  ],
};

export const treasuries = {
  columns: (
    labels: {
      idx?: string;
      name?: string;
      type?: string;
      balance?: string;
      currency?: string;
      status?: string;
    } = {},
  ): OrbisColumnDef[] => [
    idx(labels.idx),
    nameCol("name", labels.name || "Treasury", 200),
    text("treasury_type", labels.type || "Type", 110),
    num("balance", labels.balance || "Balance", 120),
    text("currency_code", labels.currency || "Currency", 90),
    status("status", labels.status || "Status", 90),
  ],
};

export const accountStatement = {
  columns: (
    labels: {
      idx?: string;
      date?: string;
      description?: string;
      debit?: string;
      credit?: string;
      balance?: string;
      ref?: string;
    } = {},
  ): OrbisColumnDef[] => [
    idx(labels.idx),
    date("entry_date", labels.date || "Date", 110),
    text("description", labels.description || "Description", 250),
    text("reference", labels.ref || "Ref", 110),
    num("debit", labels.debit || "Debit", 110),
    num("credit", labels.credit || "Credit", 110),
    num("running_balance", labels.balance || "Balance", 120),
  ],
};

// ═══════════════════════════════════════════════════════════════════════════
// ACADEMICS Tables
// ═══════════════════════════════════════════════════════════════════════════

export const classTeachers = {
  columns: (
    labels: {
      idx?: string;
      teacher?: string;
      subject?: string;
      role?: string;
      phone?: string;
      email?: string;
    } = {},
  ): OrbisColumnDef[] => [
    idx(labels.idx),
    userCol("teacher_name", labels.teacher || "Teacher", 180, "teacher_avatar"),
    text("subject_name", labels.subject || "Subject", 140),
    text("role", labels.role || "Role", 100),
    text("phone", labels.phone || "Phone", 130),
    text("email", labels.email || "Email", 180),
  ],
};

export const examSessions = {
  columns: (
    labels: {
      idx?: string;
      student?: string;
      seatNo?: string;
      secretNo?: string;
      status?: string;
      score?: string;
    } = {},
  ): OrbisColumnDef[] => [
    idx(labels.idx),
    userCol("student_name", labels.student || "Student", 180, "student_avatar"),
    text("seat_number", labels.seatNo || "Seat #", 80),
    text("secret_number", labels.secretNo || "Secret #", 90),
    status("status", labels.status || "Status", 90),
    num("score", labels.score || "Score", 80),
  ],
};

// ═══════════════════════════════════════════════════════════════════════════
// STUDENT DETAIL Tables
// ═══════════════════════════════════════════════════════════════════════════

export const studentGrades = {
  columns: (
    labels: {
      idx?: string;
      subject?: string;
      examType?: string;
      score?: string;
      maxScore?: string;
      pct?: string;
      grade?: string;
    } = {},
  ): OrbisColumnDef[] => [
    idx(labels.idx),
    nameCol("subject_name", labels.subject || "Subject", 180),
    text("exam_type", labels.examType || "Exam Type", 120),
    num("score", labels.score || "Score", 80),
    num("max_score", labels.maxScore || "Max", 70),
    pct("percentage", labels.pct || "%", 70),
    text("grade_letter", labels.grade || "Grade", 80),
  ],
};

export const studentFees = {
  columns: (
    labels: {
      idx?: string;
      fee?: string;
      total?: string;
      paid?: string;
      discount?: string;
      balance?: string;
      status?: string;
    } = {},
  ): OrbisColumnDef[] => [
    idx(labels.idx),
    nameCol("fee_name", labels.fee || "Fee", 200),
    num("total", labels.total || "Total", 100),
    num("paid", labels.paid || "Paid", 100),
    num("discount", labels.discount || "Discount", 90),
    num("balance", labels.balance || "Balance", 100),
    status("status", labels.status || "Status", 90),
  ],
};

export const studentAttendance = {
  columns: (
    labels: {
      idx?: string;
      date?: string;
      status?: string;
      subject?: string;
      notes?: string;
      recordedBy?: string;
    } = {},
  ): OrbisColumnDef[] => [
    idx(labels.idx),
    date("date", labels.date || "Date", 110),
    status("status", labels.status || "Status", 90),
    text("subject_name", labels.subject || "Subject", 140),
    text("notes", labels.notes || "Notes", 200),
    text("recorded_by_name", labels.recordedBy || "Recorded By", 130),
  ],
};

export const studentDocuments = {
  columns: (
    labels: {
      idx?: string;
      title?: string;
      type?: string;
      size?: string;
      uploadedAt?: string;
      uploadedBy?: string;
    } = {},
  ): OrbisColumnDef[] => [
    idx(labels.idx),
    nameCol("title", labels.title || "Document", 220),
    text("document_type", labels.type || "Type", 110),
    text("file_size", labels.size || "Size", 80),
    date("created_at", labels.uploadedAt || "Uploaded", 120),
    text("uploaded_by_name", labels.uploadedBy || "By", 120),
  ],
};

export const studentBehavior = {
  columns: (
    labels: {
      idx?: string;
      date?: string;
      type?: string;
      category?: string;
      description?: string;
      points?: string;
      status?: string;
    } = {},
  ): OrbisColumnDef[] => [
    idx(labels.idx),
    date("created_at", labels.date || "Date", 110),
    text("record_type", labels.type || "Type", 90),
    text("category", labels.category || "Category", 120),
    text("description", labels.description || "Description", 220),
    num("points", labels.points || "Points", 70),
    status("status", labels.status || "Status", 90),
  ],
};

// ═══════════════════════════════════════════════════════════════════════════
// SETTINGS: Ordering (drag-reorder table — special case)
// ═══════════════════════════════════════════════════════════════════════════

export const ordering = {
  columns: (
    labels: { idx?: string; name?: string; type?: string; order?: string } = {},
  ): OrbisColumnDef[] => [
    idx(labels.idx),
    nameCol("name", labels.name || "Item", 250),
    text("item_type", labels.type || "Type", 120),
    num("sort_order", labels.order || "Order", 80),
  ],
};
