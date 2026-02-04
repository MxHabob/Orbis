import type { StickyColumnConfig, TableConfig } from "@/components/tables/core";
import type { TableId } from "./table-settings";

/**
 * Sticky column configurations for each table
 */
export const STICKY_COLUMNS: Record<TableId, StickyColumnConfig[]> = {
  transactions: [
    { id: "select", width: 50 },
    { id: "date", width: 110 },
    { id: "description", width: 320 },
  ],
  invoices: [
    { id: "select", width: 50 },
    { id: "invoiceNumber", width: 180 },
  ],
  customers: [{ id: "name", width: 320 }],
  students: [{ id: "name", width: 250 }],
  vault: [
    { id: "select", width: 50 },
    { id: "title", width: 250 },
  ],
  grades: [
    { id: "studentName", width: 300 },
  ],
  classes: [
    { id: "select", width: 50 },
    { id: "name", width: 250 },
  ],
  reports: [
    { id: "select", width: 50 },
    { id: "name", width: 250 },
  ],
};

/**
 * Sort field mappings for each table
 * Maps column IDs to their backend sort field names
 */
export const SORT_FIELD_MAPS: Record<TableId, Record<string, string>> = {
  transactions: {
    date: "date",
    description: "name",
    amount: "amount",
    category: "category",
    counterparty: "counterparty",
    tags: "tags",
    bank_account: "bank_account",
    method: "method",
    assigned: "assigned",
    status: "attachment",
  },
  invoices: {
    invoiceNumber: "invoice_number",
    status: "status",
    dueDate: "due_date",
    customer: "customer",
    amount: "amount",
    issueDate: "issue_date",
  },
  customers: {
    name: "name",
    contact: "contact",
    email: "email",
    invoices: "invoices",
    projects: "projects",
    industry: "industry",
    country: "country",
    totalRevenue: "total_revenue",
    outstanding: "outstanding",
    lastInvoice: "last_invoice",
    tags: "tags",
  },
  students: {
    name: "name",
    studentNumber: "studentNumber",
    enrollmentDate: "enrollmentDate",
  },
  vault: {},
  grades: {},
  classes: {
    name: "name",
    description: "description",
  },
  reports: {
    name: "name",
    description: "description",
  },
};

export const NON_REORDERABLE_COLUMNS: Record<TableId, Set<string>> = {
  transactions: new Set(["select", "date", "description", "actions"]),
  invoices: new Set(["select", "invoiceNumber", "actions"]),
  customers: new Set(["name", "actions"]),
  students: new Set(["name", "actions"]),
  vault: new Set(["select", "title", "actions"]),
  grades: new Set(["studentName", "total"]),
  classes: new Set(["select", "name", "actions"]),
  reports: new Set(["select", "name", "actions"]),
};

/**
 * Row heights for each table
 */
export const ROW_HEIGHTS: Record<TableId, number> = {
  transactions: 45,
  invoices: 57,
  customers: 45,
  students: 45,
  vault: 45,
  grades: 45,
  classes: 57,
  reports: 57,
};

/**
 * Summary grid heights for tables with summary sections
 */
export const SUMMARY_GRID_HEIGHTS: Partial<Record<TableId, number>> = {
  invoices: 180,
  customers: 180,
  students: 0,
  grades: 0,
  classes: 0,
  reports: 0,
};

/**
 * Complete table configurations
 */
export const TABLE_CONFIGS: Record<TableId, TableConfig> = {
  transactions: {
    tableId: "transactions",
    stickyColumns: STICKY_COLUMNS.transactions,
    sortFieldMap: SORT_FIELD_MAPS.transactions,
    nonReorderableColumns: NON_REORDERABLE_COLUMNS.transactions,
    rowHeight: ROW_HEIGHTS.transactions,
  },
  invoices: {
    tableId: "invoices",
    stickyColumns: STICKY_COLUMNS.invoices,
    sortFieldMap: SORT_FIELD_MAPS.invoices,
    nonReorderableColumns: NON_REORDERABLE_COLUMNS.invoices,
    rowHeight: ROW_HEIGHTS.invoices,
    summaryGridHeight: SUMMARY_GRID_HEIGHTS.invoices,
  },
  customers: {
    tableId: "customers",
    stickyColumns: STICKY_COLUMNS.customers,
    sortFieldMap: SORT_FIELD_MAPS.customers,
    nonReorderableColumns: NON_REORDERABLE_COLUMNS.customers,
    rowHeight: ROW_HEIGHTS.customers,
    summaryGridHeight: SUMMARY_GRID_HEIGHTS.customers,
  },
  students: {
    tableId: "students",
    stickyColumns: STICKY_COLUMNS.students,
    sortFieldMap: SORT_FIELD_MAPS.students,
    nonReorderableColumns: NON_REORDERABLE_COLUMNS.students,
    rowHeight: ROW_HEIGHTS.students,
    summaryGridHeight: SUMMARY_GRID_HEIGHTS.students,
  },
  vault: {
    tableId: "vault",
    stickyColumns: STICKY_COLUMNS.vault,
    sortFieldMap: SORT_FIELD_MAPS.vault,
    nonReorderableColumns: NON_REORDERABLE_COLUMNS.vault,
    rowHeight: ROW_HEIGHTS.vault,
  },
  grades: {
    tableId: "grades",
    stickyColumns: STICKY_COLUMNS.grades,
    sortFieldMap: SORT_FIELD_MAPS.grades,
    nonReorderableColumns: NON_REORDERABLE_COLUMNS.grades,
    rowHeight: ROW_HEIGHTS.grades,
  },
  classes: {
    tableId: "classes",
    stickyColumns: STICKY_COLUMNS.classes,
    sortFieldMap: SORT_FIELD_MAPS.classes,
    nonReorderableColumns: NON_REORDERABLE_COLUMNS.classes,
    rowHeight: ROW_HEIGHTS.classes,
    summaryGridHeight: SUMMARY_GRID_HEIGHTS.classes,
  },
  reports: {
    tableId: "reports",
    stickyColumns: STICKY_COLUMNS.reports,
    sortFieldMap: SORT_FIELD_MAPS.reports,
    nonReorderableColumns: NON_REORDERABLE_COLUMNS.reports,
    rowHeight: ROW_HEIGHTS.reports,
    summaryGridHeight: SUMMARY_GRID_HEIGHTS.reports,
  },
};

/**
 * Get table configuration by ID
 */
export function getTableConfig(tableId: TableId): TableConfig {
  return TABLE_CONFIGS[tableId];
}
