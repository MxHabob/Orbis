/**
 * @orbis/data-grid
 * Enterprise data grid for Orbis desktop — wraps RevoGrid with S.C.R.A.P.S. theming.
 */

// Generic column builders (for students, attendance, fees pages)
export {
  addRowIndices,
  buildAttendanceColumns,
  buildFeesColumns,
  buildStudentListColumns,
  flattenForGrid,
  indexColumn,
  percentageColumn,
  scoreColumn,
  statusColumn,
  studentNameColumn,
  textColumn,
} from "./column-builders";

// Editors
export { NumberEditor } from "./editors/index";
// Grade-specific helpers
export {
  buildGradeColumns,
  recalculateRowTotals,
  scoreToGradeClass,
  scoreToGradeColor,
  scoreToGradeLetter,
  transformMatrixToSource,
} from "./grade-helpers";
// Main component
export { default as OrbisGrid } from "./OrbisGrid.svelte";
// Types
export type {
  CellEditEvent,
  CellValidation,
  GradeCell,
  GradeMatrixResponse,
  GradeMatrixStudent,
  OrbisColumnDef,
  OrbisGridProps,
} from "./types";
