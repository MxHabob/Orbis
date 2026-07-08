/**
 * @module grade-columns
 * Column definitions for bulk grade import grid.
 */

import type { OrbisColumnDef } from "@orbis/svelte-grid";
import { indexColumn } from "$lib/tables";

export const GRADE_DATA_PROPS = [
  "student_identifier",
  "subject_name",
  "score",
  "max_score",
  "exam_type",
  "semester",
  "notes",
] as const;

function errorCellProps(prop: string) {
  return (row: Record<string, unknown>) => {
    const errors = row.__errors as Map<string, string> | undefined;
    if (errors?.has(prop)) {
      return { class: "orbis-cell-error", title: errors.get(prop) };
    }
    return {};
  };
}

export function buildGradeColumns(
  labels: Record<string, string>,
): OrbisColumnDef[] {
  return [
    indexColumn("#"),
    {
      prop: "student_identifier",
      name: labels.student_id ?? "Student (Email/ID/Name)",
      size: 220,
      cellProperties: errorCellProps("student_identifier"),
    },
    {
      prop: "subject_name",
      name: labels.subject ?? "Subject",
      size: 160,
      cellProperties: errorCellProps("subject_name"),
    },
    {
      prop: "score",
      name: labels.score ?? "Score",
      size: 95,
      editor: "number",
      editorConfig: { min: 0, max: 999, step: 0.5 },
      cellProperties: errorCellProps("score"),
    },
    {
      prop: "max_score",
      name: labels.max_score ?? "Max Score",
      size: 105,
      editor: "number",
      editorConfig: { min: 1, max: 999, step: 1 },
      cellProperties: errorCellProps("max_score"),
    },
    {
      prop: "exam_type",
      name: labels.exam_type ?? "Exam Type",
      size: 120,
    },
    {
      prop: "semester",
      name: labels.semester ?? "Semester",
      size: 120,
      cellProperties: errorCellProps("semester"),
    },
    {
      prop: "notes",
      name: labels.notes ?? "Notes",
      size: 160,
    },
  ];
}

export function createEmptyGradeRow(): Record<string, unknown> {
  const row: Record<string, unknown> = {};
  for (const prop of GRADE_DATA_PROPS) {
    row[prop] =
      prop === "max_score" ? "100" : prop === "exam_type" ? "final" : "";
  }
  row.__errors = new Map<string, string>();
  return row;
}
