import type { Component } from "svelte";

export interface OrbisColumnDef {
  /** Column property key (maps to SVAR 'id') */
  id?: string;
  /** Display name / header label (maps to SVAR 'header') */
  header?: string;
  /** Width in pixels */
  width?: number;
  /** Flex grow sizing */
  flexgrow?: number;
  /** Whether the column is hidden */
  hidden?: boolean;
  /** Enable sorting */
  sort?: boolean | ((a: any, b: any) => number);
  /** Inline editor type or configuration */
  editor?: string | any;
  /** Custom Svelte component for rendering */
  cell?: Component<any>;
  /** Alignment of text in the column */
  align?: "start" | "center" | "end";
  /** Optional cell CSS class */
  cellClass?: string;
  /** Whether column is read-only */
  readonly?: boolean;
  /** Extra custom meta information */
  meta?: Record<string, any>;
  /** Compatibility fields for old pages */
  prop?: string;
  name?: string;
  size?: number;
  cellRenderer?: any;
  pin?: string;
  cellProperties?: (row: any) => Record<string, any>;
  sortable?: boolean;
  resize?: boolean;
  options?: any[];
  editorConfig?: Record<string, any>;
}

export interface CellEditEvent {
  /** Row data object */
  row: Record<string, any>;
  /** Row index in the data source */
  rowIndex: number;
  /** Column ID */
  columnId: string;
  /** Previous value */
  oldValue: any;
  /** New value */
  newValue: any;
  /** Event action */
  action?: string;
  /** Event source object */
  event?: any;
}

export interface GridActionEvent {
  /** Action identifier */
  actionId: string;
  /** Row data */
  row: Record<string, any>;
  /** Row index in source array */
  rowIndex: number;
  /** Column property key */
  prop: string;
  /** Original DOM MouseEvent */
  originalEvent: MouseEvent;
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
  visible?: (row: any) => boolean;
  disabled?: (row: any) => boolean;
}

export interface CellValidation {
  validate: (value: any, row: any) => boolean | string;
  errorMessage?: string;
}
