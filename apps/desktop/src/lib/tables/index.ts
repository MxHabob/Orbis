export { DataTable } from "./core";
export { withIndex } from "./definitions";
export * as tableDefs from "./definitions";

// Cells
export { default as AvatarCell } from "./components/AvatarCell.svelte";
export { default as BadgeCell } from "./components/BadgeCell.svelte";
export { default as CheckboxCell } from "./components/CheckboxCell.svelte";
export { default as ActionsCell } from "./components/ActionsCell.svelte";
export { default as DefaultCell } from "./components/DefaultCell.svelte";
export { default as ActionOverlay } from "./components/ActionOverlay.svelte";

// Utilities
export { createInitials } from "./utils";

// Column Builders
export {
  indexColumn,
  selectionColumn,
  studentNameColumn,
  scoreColumn,
  textColumn,
  statusColumn,
  percentageColumn,
  addRowIndices,
  flattenForGrid,
  buildAttendanceColumns,
  buildStudentListColumns,
  buildFeesColumns,
} from "./definitions/builders";

// Grade Helpers
export {
  scoreToGradeLetter,
  scoreToGradeClass,
  buildGradeColumns,
  transformMatrixToSource,
  recalculateRowTotals,
} from "./definitions/grade-helpers";

export type {
  GradeCell,
  GradeMatrixStudent,
  GradeMatrixResponse,
  GradeScaleItem,
} from "./definitions/grade-helpers";
