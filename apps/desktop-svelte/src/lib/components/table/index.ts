// ── Desktop Table Components ────────────────────────────────────────
// Re-export the main DataTable and related utilities.

export { default as DataTable } from "./data-table.svelte";

// Re-export table types and hooks from @orbis/ui-svelte for convenience
export type {
	ColumnDef,
	CellContext,
	TableConfig,
	SortDirection,
	ExportFormat,
} from "@orbis/ui-svelte";
