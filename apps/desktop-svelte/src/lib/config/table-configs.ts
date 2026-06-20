// ── Desktop Table Config ────────────────────────────────────────────
// Central registry of table configurations for the school management system.
// Inspired by: apps/dashboard/src/utils/table-configs.ts

import type { TableConfig, StickyColumnConfig } from "@orbis/ui-svelte";

// ── Table IDs ──────────────────────────────────────────────────────

export type TableId =
	| "students"
	| "classes"
	| "subjects"
	| "teachers"
	| "grades"
	| "attendance"
	| "fees"
	| "parents"
	| "schedule"
	| "exams"
	| "certificates"
	| "reports";

// ── Sticky Columns ─────────────────────────────────────────────────

export const STICKY_COLUMNS: Record<TableId, StickyColumnConfig[]> = {
	students: [{ id: "name", width: 250 }],
	classes: [{ id: "name", width: 220 }],
	subjects: [{ id: "name", width: 200 }],
	teachers: [{ id: "name", width: 250 }],
	grades: [{ id: "studentName", width: 250 }],
	attendance: [{ id: "studentName", width: 250 }],
	fees: [{ id: "studentName", width: 250 }],
	parents: [{ id: "name", width: 250 }],
	schedule: [{ id: "time", width: 120 }],
	exams: [{ id: "name", width: 250 }],
	certificates: [{ id: "studentName", width: 300 }],
	reports: [{ id: "name", width: 250 }],
};

// ── Sort Field Maps ────────────────────────────────────────────────
// Maps column IDs → backend sort field names

export const SORT_FIELD_MAPS: Record<TableId, Record<string, string>> = {
	students: {
		name: "full_name",
		studentNumber: "student_number",
		class: "class_name",
		status: "status",
		enrollmentDate: "enrollment_date",
	},
	classes: {
		name: "name",
		studentsCount: "students_count",
		capacity: "capacity",
	},
	subjects: {
		name: "name",
		code: "code",
	},
	teachers: {
		name: "name",
		subject: "subject",
		email: "email",
	},
	grades: {
		studentName: "student_name",
		total: "total",
	},
	attendance: {
		studentName: "student_name",
		date: "date",
		status: "status",
	},
	fees: {
		studentName: "student_name",
		amount: "amount",
		dueDate: "due_date",
		status: "status",
	},
	parents: {
		name: "name",
		phone: "phone",
		email: "email",
	},
	schedule: {
		time: "time",
		subject: "subject",
		teacher: "teacher",
	},
	exams: {
		name: "name",
		date: "date",
		subject: "subject",
	},
	certificates: {
		studentName: "student_name",
		issuedDate: "issued_date",
	},
	reports: {
		name: "name",
		date: "date",
	},
};

// ── Non-Reorderable Columns ────────────────────────────────────────

export const NON_REORDERABLE_COLUMNS: Record<TableId, Set<string>> = {
	students: new Set(["name", "actions"]),
	classes: new Set(["name", "actions"]),
	subjects: new Set(["name", "actions"]),
	teachers: new Set(["name", "actions"]),
	grades: new Set(["studentName", "total"]),
	attendance: new Set(["studentName", "actions"]),
	fees: new Set(["studentName", "actions"]),
	parents: new Set(["name", "actions"]),
	schedule: new Set(["time", "actions"]),
	exams: new Set(["name", "actions"]),
	certificates: new Set(["studentName", "actions"]),
	reports: new Set(["name", "actions"]),
};

// ── Row Heights ────────────────────────────────────────────────────

export const ROW_HEIGHTS: Record<TableId, number> = {
	students: 45,
	classes: 50,
	subjects: 45,
	teachers: 45,
	grades: 45,
	attendance: 40,
	fees: 50,
	parents: 45,
	schedule: 40,
	exams: 50,
	certificates: 50,
	reports: 50,
};

// ── Full Table Configs ─────────────────────────────────────────────

export const TABLE_CONFIGS: Record<TableId, TableConfig> = Object.fromEntries(
	(Object.keys(STICKY_COLUMNS) as TableId[]).map((id) => [
		id,
		{
			tableId: id,
			stickyColumns: STICKY_COLUMNS[id],
			sortFieldMap: SORT_FIELD_MAPS[id],
			nonReorderableColumns: NON_REORDERABLE_COLUMNS[id],
			rowHeight: ROW_HEIGHTS[id],
		} satisfies TableConfig,
	]),
) as Record<TableId, TableConfig>;

// ── Default Hidden Columns ─────────────────────────────────────────

export const DEFAULT_HIDDEN_COLUMNS: Record<TableId, string[]> = {
	students: [],
	classes: [],
	subjects: [],
	teachers: [],
	grades: [],
	attendance: [],
	fees: [],
	parents: [],
	schedule: [],
	exams: [],
	certificates: [],
	reports: [],
};

/**
 * Get table configuration by ID.
 */
export function getTableConfig(tableId: TableId): TableConfig {
	return TABLE_CONFIGS[tableId];
}
