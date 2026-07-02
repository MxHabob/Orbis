/**
 * @orbis/data-grid
 * Enterprise data grid for Orbis desktop — wraps RevoGrid with S.C.R.A.P.S. theming.
 */

// Main component
export { default as OrbisGrid } from "./OrbisGrid.svelte";

// Editors
export { NumberEditor } from "./editors/index";

// Types
export type {
  OrbisColumnDef,
  CellEditEvent,
  CellValidation,
  GradeMatrixStudent,
  GradeCell,
  GradeMatrixResponse,
  OrbisGridProps,
} from "./types";

// Grade-specific helpers
export {
  scoreToGradeLetter,
  scoreToGradeColor,
  scoreToGradeClass,
  buildGradeColumns,
  transformMatrixToSource,
  recalculateRowTotals,
} from "./grade-helpers";
