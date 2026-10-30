import type { Component } from "svelte";

export type ColumnType =
  | "text"
  | "number"
  | "integer"
  | "decimal"
  | "currency"
  | "percentage"
  | "date"
  | "datetime"
  | "boolean"
  | "enum"
  | "relation"
  | "status"
  | "badge"
  | "avatar"
  | "computed"
  | "formula"
  | "action"
  | "custom";

export type FilterOperator =
  | "eq"
  | "neq"
  | "contains"
  | "not_contains"
  | "starts_with"
  | "ends_with"
  | "gt"
  | "gte"
  | "lt"
  | "lte"
  | "between"
  | "in"
  | "not_in"
  | "is_null"
  | "is_not_null"
  | "bool_true"
  | "bool_false";

export type SortDir = "asc" | "desc";

export type Align = "start" | "center" | "end";

export type PinSide = "left" | "right" | "colPinStart" | "colPinEnd";

export type GridRow = Record<string, unknown>;

export interface FilterCondition {
  field: string;
  operator: FilterOperator | string;
  value?: unknown;
}

export interface GridFilter {
  logic: "AND" | "OR";
  conditions: FilterCondition[];
  search?: string;
  groups?: GridFilter[];
  sort_by?: string;
  sort_dir?: SortDir;
}

export interface SortSpec {
  id: string;
  dir: SortDir;
}

export interface EnumOption {
  value: string | number | boolean;
  label: string;
}

export interface ColumnValidateResult {
  ok: boolean;
  message?: string;
  value?: unknown;
}

export interface OrbisColumnDef {
  id?: string;
  header?: string;
  width?: number;
  minWidth?: number;
  maxWidth?: number;
  flex?: number;
  flexgrow?: number;
  hidden?: boolean;
  sort?: boolean | ((a: unknown, b: unknown) => number);
  editor?: string | Record<string, unknown> | null;
  cell?: Component<Record<string, unknown>>;
  headerCell?: Component<Record<string, unknown>>;
  align?: Align;
  cellClass?: string;
  readonly?: boolean;
  meta?: Record<string, unknown>;
  prop?: string;
  name?: string;
  size?: number;
  cellRenderer?: unknown;
  pin?: PinSide;
  cellProperties?: (row: GridRow) => Record<string, unknown>;
  sortable?: boolean;
  resize?: boolean;
  resizable?: boolean;
  options?: EnumOption[];
  editorConfig?: Record<string, unknown>;
  type?: ColumnType;
  format?: string;
  currency?: string;
  accessor?: (row: GridRow) => unknown;
  validate?: (
    value: unknown,
    row: GridRow,
  ) => boolean | string | ColumnValidateResult;
  filterable?: boolean;
  aggregate?: AggregateFn;
}

export type AggregateFn =
  | "sum"
  | "avg"
  | "min"
  | "max"
  | "count"
  | "count_distinct";

export interface CellEditEvent {
  row: GridRow;
  rowIndex: number;
  columnId: string;
  oldValue: unknown;
  newValue: unknown;
  action?: string;
  event?: unknown;
  prop?: string;
}

export interface GridActionEvent {
  actionId: string;
  row: GridRow;
  rowIndex: number;
  prop: string;
  originalEvent: MouseEvent | KeyboardEvent | PointerEvent;
  selected?: boolean;
  selectedIds?: string[];
}

export interface ActionItem {
  id: string;
  icon?: string;
  label?: string;
  variant?:
    | "default"
    | "destructive"
    | "success"
    | "secondary"
    | "outline"
    | "ghost"
    | "link";
  visible?: (row: GridRow) => boolean;
  disabled?: (row: GridRow) => boolean;
}

export interface CellValidation {
  validate: (value: unknown, row: GridRow) => boolean | string;
  errorMessage?: string;
}

export interface CellRange {
  startRow: number;
  startCol: number;
  endRow: number;
  endCol: number;
}

export type SelectScope = "filtered" | "visible" | "page" | "allLoaded";

export interface SelectionSnapshot {
  /** explicit: only ids. allMatching: all filtered except ids (exclusions). */
  mode: "explicit" | "allMatching";
  ids: Set<string>;
  range: CellRange | null;
  active: { row: number; col: number } | null;
}

export interface BulkCellError {
  rowIndex: number;
  columnId: string;
  value: unknown;
  message: string;
}

export interface BulkResult<T = GridRow> {
  accepted: T[];
  rejected: Array<{ rowIndex: number; row: T; errors: BulkCellError[] }>;
  acceptedCount: number;
  rejectedCount: number;
}

export interface GridRowsChangeEvent {
  rows: GridRow[];
  changedRows: GridRow[];
  action:
    | "edit"
    | "paste"
    | "cut"
    | "delete"
    | "fill-down"
    | "fill-right"
    | "duplicate";
}

export interface VirtualWindow {
  start: number;
  end: number;
  offset: number;
  count: number;
}

export interface GroupNode<T = GridRow> {
  key: string;
  value: unknown;
  rows: T[];
  aggregates: Record<string, number | string | null>;
  collapsed: boolean;
}

export interface ExportOptions {
  format: "csv" | "tsv" | "json";
  scope: "all" | "visible" | "selected" | "filtered";
  columns?: string[];
}

export interface DataPage<T = GridRow> {
  items: T[];
  totalCount: number;
  page: number;
  pageSize: number;
  totalPages: number;
}

export interface GridDataSource<T = GridRow> {
  mode: "client" | "server";
  load: (query: DataQuery) => Promise<DataPage<T>>;
  create?: (rows: Partial<T>[]) => Promise<BulkResult<T>>;
  update?: (
    rows: Array<{ id: string; patch: Partial<T> }>,
  ) => Promise<BulkResult<T>>;
  delete?: (ids: string[]) => Promise<BulkResult<{ id: string }>>;
}

export interface DataQuery {
  page: number;
  pageSize: number;
  filter?: GridFilter;
  sorts?: SortSpec[];
  search?: string;
}

export function columnId(col: OrbisColumnDef): string {
  return String(col.id || col.prop || "");
}

export function columnHeader(col: OrbisColumnDef): string {
  return String(col.header || col.name || columnId(col));
}

export function columnWidth(col: OrbisColumnDef): number {
  return col.width ?? col.size ?? 120;
}

export function isColumnPinnedLeft(col: OrbisColumnDef): boolean {
  return col.pin === "left" || col.pin === "colPinStart";
}

export function isColumnPinnedRight(col: OrbisColumnDef): boolean {
  return col.pin === "right" || col.pin === "colPinEnd";
}

export function isColumnSortable(col: OrbisColumnDef): boolean {
  if (col.sortable === false || col.sort === false) return false;
  return col.sort !== undefined || col.sortable === true || !!col.type;
}

export function getCellValue(row: GridRow, col: OrbisColumnDef): unknown {
  if (col.accessor) return col.accessor(row);
  return row[columnId(col)];
}
