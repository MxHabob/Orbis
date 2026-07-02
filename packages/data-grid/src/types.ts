/**
 * @orbis/data-grid — Type definitions
 * Shared types for the OrbisGrid component and its editors.
 */

// ── Column Definition ────────────────────────────────────────────────────────

export interface OrbisColumnDef {
  /** The data property key to bind to */
  prop: string;
  /** Display name (header label) */
  name: string;
  /** Column width in pixels */
  size?: number;
  /** Minimum column width */
  minSize?: number;
  /** Maximum column width */
  maxSize?: number;
  /** Whether the column is sortable */
  sortable?: boolean;
  /** Whether the column is resizable */
  resizable?: boolean;
  /** Whether the column is read-only */
  readonly?: boolean;
  /** Pin column to start or end */
  pin?: "colPinStart" | "colPinEnd";
  /** Cell type — determines editor and renderer */
  cellType?: "text" | "number" | "badge" | "custom";
  /** Custom editor name (maps to editors registry) */
  editor?: string;
  /** Column-specific metadata */
  meta?: Record<string, unknown>;
  /** Text alignment */
  align?: "start" | "center" | "end";
  /** Cell CSS class */
  cellClass?: string;
  /** Header CSS class */
  headerClass?: string;
  /** Custom properties function for the cell, receives row data */
  cellProperties?: (row: any) => Record<string, any>;
  /** Column-specific editor configuration */
  editorConfig?: Record<string, any>;
}

// ── Cell Edit Event ──────────────────────────────────────────────────────────

export interface CellEditEvent {
  /** Row data object */
  row: Record<string, unknown>;
  /** Row index in the source array */
  rowIndex: number;
  /** Column property key */
  prop: string;
  /** Previous cell value */
  oldValue: unknown;
  /** New cell value */
  newValue: unknown;
  /** Whether the edit was cancelled by validation */
  cancelled?: boolean;
}

// ── Cell Validation ──────────────────────────────────────────────────────────

export interface CellValidation {
  /** Column property key to validate */
  prop: string;
  /** Validation function — return error message string or null for valid */
  validate: (value: unknown, row: Record<string, unknown>) => string | null;
}

// ── Grade Matrix Types (mirror API response) ────────────────────────────────

export interface GradeMatrixStudent {
  enrollment_id: string;
  student_id: string;
  student_name: string;
  student_name_ar?: string | null;
  grades: GradeCell[];
  total: number;
  max_total: number;
  percentage: number;
  grade_letter: string;
}

export interface GradeCell {
  grade_id?: string | null;
  assessment_id?: string | null;
  exam_type: string;
  score?: number | null;
  max_score: number;
}

export interface GradeMatrixResponse {
  cycle_id: string;
  class_subject_id: string;
  students: GradeMatrixStudent[];
  assessment_types: string[];
}

// ── Grid Props ───────────────────────────────────────────────────────────────

export interface OrbisGridProps {
  /** Column definitions */
  columns: OrbisColumnDef[];
  /** Source data array */
  source: Record<string, unknown>[];
  /** Unique row identifier key */
  rowIdKey?: string;
  /** Grid height (CSS value) */
  height?: string;
  /** Enable RTL layout */
  rtl?: boolean;
  /** Make entire grid read-only */
  readonly?: boolean;
  /** RevoGrid theme — auto-mapped to S.C.R.A.P.S. */
  theme?: "compact" | "material" | "default";
  /** Enable range selection (Excel-like) */
  enableRange?: boolean;
  /** Enable clipboard operations */
  enableClipboard?: boolean;
  /** Enable column auto-size */
  enableAutoSize?: boolean;
  /** Validation rules */
  validations?: CellValidation[];
  /** Number of rows to render beyond visible area */
  overscan?: number;
}
