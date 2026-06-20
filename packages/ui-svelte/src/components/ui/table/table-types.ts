// ── Table Infrastructure Types ──────────────────────────────────────
// Central type definitions for the Svelte 5 Runes-based table system.
// Inspired by TanStack Table patterns but native to Svelte 5.
// Supports RTL/LTR via CSS logical properties throughout.

import type { Snippet } from "svelte";

// ── Sorting ─────────────────────────────────────────────────────────

export type SortDirection = "asc" | "desc";

export interface SortingState {
	column: string | null;
	direction: SortDirection;
}

// ── Filtering ───────────────────────────────────────────────────────

export interface FilterState {
	globalFilter: string;
	columnFilters: Record<string, string>;
}

// ── Column Definition ───────────────────────────────────────────────

/** Skeleton variant used in loading states */
export type SkeletonType =
	| "text"
	| "avatar-text"
	| "icon-text"
	| "badge"
	| "tags"
	| "icon"
	| "checkbox";

export interface SkeletonConfig {
	type: SkeletonType;
	/** Tailwind width class, e.g. "w-20" */
	width?: string;
}

/**
 * Column definition — the declarative schema for each column.
 *
 * @template T - The row data type
 *
 * @example
 * ```ts
 * const cols: ColumnDef<Student>[] = [
 *   { id: 'name', header: 'Name', size: 250, sticky: true, sortable: true },
 *   { id: 'status', header: 'Status', size: 120, cell: statusSnippet },
 * ];
 * ```
 */
export interface ColumnDef<T> {
	/** Unique column identifier */
	id: string;

	/** Header label — plain string or Svelte 5 Snippet */
	header: string;

	// ── Data Access ──────────────────────────────────────────────────

	/** Key path into row data for simple access */
	accessorKey?: keyof T & string;

	/** Custom accessor function for computed values */
	accessorFn?: (row: T) => unknown;

	// ── Sizing ──────────────────────────────────────────────────────

	/** Default width in pixels */
	size?: number;
	/** Minimum width in pixels */
	minSize?: number;
	/** Maximum width in pixels */
	maxSize?: number;
	/** Allow user to resize this column (default: true) */
	enableResizing?: boolean;

	// ── Feature Flags ───────────────────────────────────────────────

	/** Allow sorting on this column */
	sortable?: boolean;
	/** Custom sort comparator */
	sortFn?: (a: T, b: T) => number;
	/** Backend sort field name (may differ from column id) */
	sortField?: string;

	/** Allow filtering on this column */
	filterable?: boolean;

	/** Pin this column (sticky) */
	sticky?: boolean;

	/** Initial visibility (default: true) */
	visible?: boolean;

	/** Allow user to reorder this column (default: true) */
	reorderable?: boolean;

	/** Allow user to hide this column (default: true) */
	enableHiding?: boolean;

	// ── Rendering ───────────────────────────────────────────────────

	/**
	 * Custom cell renderer as Svelte 5 Snippet.
	 * Receives `{ value, row, rowIndex }`.
	 */
	cell?: Snippet<[CellContext<T>]>;

	/**
	 * Custom header renderer as Svelte 5 Snippet.
	 * Receives `{ column }`.
	 */
	headerSnippet?: Snippet<[HeaderContext<T>]>;

	// ── Skeleton ────────────────────────────────────────────────────

	/** Skeleton config for loading state */
	skeleton?: SkeletonConfig;

	// ── Styling ─────────────────────────────────────────────────────

	/** Additional CSS class for cells in this column */
	className?: string;
	/** Text alignment: 'start' | 'center' | 'end' */
	align?: "start" | "center" | "end";

	// ── Meta ────────────────────────────────────────────────────────

	/** Arbitrary metadata for custom use */
	meta?: Record<string, unknown>;
}

/** Context passed to cell Snippets */
export interface CellContext<T> {
	/** The raw cell value (from accessorKey or accessorFn) */
	value: unknown;
	/** The full row data */
	row: T;
	/** Row index in the current view */
	rowIndex: number;
	/** Column definition */
	column: ColumnDef<T>;
}

/** Context passed to header Snippets */
export interface HeaderContext<T> {
	/** Column definition */
	column: ColumnDef<T>;
	/** Current sort state for this column */
	sorted: SortDirection | null;
	/** Trigger sort on this column */
	toggleSort: () => void;
}

// ── Sticky Column Config ────────────────────────────────────────────

export interface StickyColumnConfig {
	/** Column ID */
	id: string;
	/** Fixed width in pixels */
	width: number;
}

// ── Table Configuration ─────────────────────────────────────────────

/**
 * Central configuration for a table type.
 * Registered once per table kind (students, classes, etc).
 */
export interface TableConfig {
	/** Unique table identifier */
	tableId: string;
	/** Columns pinned to the inline-start/end edges */
	stickyColumns: StickyColumnConfig[];
	/** Maps column IDs to backend sort field names */
	sortFieldMap: Record<string, string>;
	/** Column IDs that cannot be reordered */
	nonReorderableColumns: Set<string>;
	/** Default row height in pixels */
	rowHeight: number;
	/** Height of summary/metrics section above the table */
	summaryGridHeight?: number;
}

// ── Table Settings (Persistence) ────────────────────────────────────

/** Persisted user preferences for a table */
export interface TableSettings {
	/** Column visibility state */
	columns: Record<string, boolean>;
	/** Column sizing overrides */
	sizing: Record<string, number>;
	/** Column display order */
	order: string[];
	/** Last applied sort */
	sorting?: SortingState;
}

// ── Virtual List ────────────────────────────────────────────────────

export interface VirtualItem {
	/** Index in the source array */
	index: number;
	/** Offset from the top of the container (px) */
	start: number;
	/** Height of this item (px) */
	size: number;
}

export interface VirtualListOptions {
	/** Total number of items */
	count: number;
	/** Height of each item (or estimator fn) */
	estimateSize: number | ((index: number) => number);
	/** Extra items rendered above/below the visible area */
	overscan?: number;
	/** The scrollable container element */
	getScrollElement: () => HTMLElement | null;
}

export interface VirtualListState {
	/** Items currently in the viewport + overscan */
	virtualItems: VirtualItem[];
	/** Total scrollable height */
	totalSize: number;
	/** Scroll to a specific index */
	scrollToIndex: (index: number, behavior?: ScrollBehavior) => void;
}

// ── Table State ─────────────────────────────────────────────────────

export interface TableOptions<T> {
	/** Source data array */
	data: T[];
	/** Column definitions */
	columns: ColumnDef<T>[];
	/** Function to extract a unique row key */
	getRowId?: (row: T) => string;
	/** Table configuration (from registry) */
	config?: TableConfig;

	// ── Initial State ─────────────────────────────────────────────

	initialSorting?: SortingState;
	initialColumnVisibility?: Record<string, boolean>;
	initialColumnSizing?: Record<string, number>;
	initialColumnOrder?: string[];

	// ── Callbacks ─────────────────────────────────────────────────

	/** Called when sorting changes */
	onSortingChange?: (sorting: SortingState) => void;
	/** Called when a row is clicked */
	onRowClick?: (row: T, index: number) => void;
	/** Called when row selection changes */
	onSelectionChange?: (selectedIds: Set<string>) => void;

	// ── Feature Flags ─────────────────────────────────────────────

	enableSorting?: boolean;
	enableFiltering?: boolean;
	enableColumnResizing?: boolean;
	enableColumnReordering?: boolean;
	enableRowSelection?: boolean;
	enableVirtualization?: boolean;
	enableExport?: boolean;
}

// ── Export Utilities ─────────────────────────────────────────────────

export type ExportFormat = "csv" | "xlsx" | "json";

export interface ExportOptions<T> {
	/** Data to export */
	data: T[];
	/** Columns to include */
	columns: ColumnDef<T>[];
	/** Export format */
	format: ExportFormat;
	/** File name (without extension) */
	filename?: string;
	/** Custom value formatter per column */
	formatters?: Partial<Record<string, (value: unknown) => string>>;
}
