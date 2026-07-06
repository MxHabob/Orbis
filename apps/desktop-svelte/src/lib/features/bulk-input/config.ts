/**
 * @module bulk-input-config
 * Column configurations and metadata for each bulk input type.
 * Dynamically switches columns based on selected data type.
 */

import type { OrbisColumnDef, CellValidation } from "@orbis/data-grid";
import { indexColumn } from "@orbis/data-grid";

export type BulkInputType = "students" | "teachers" | "grades";

export interface BulkInputColumnConfig {
  columns: OrbisColumnDef[];
  requiredFields: string[];
  emptyRow: () => Record<string, unknown>;
}

/**
 * Get column configuration for a given bulk input type.
 * Columns change dynamically based on the selected type.
 */
export function getColumnsForType(
  type: BulkInputType,
  labels: Record<string, string>,
): BulkInputColumnConfig {
  switch (type) {
    case "students":
      return {
        columns: [
          indexColumn("#"),
          {
            prop: "full_name",
            name: labels.full_name ?? "Full Name",
            size: 200,
            cellProperties: (row: Record<string, unknown>) => {
              if (
                row.__errors &&
                (row.__errors as Map<string, string>).has("full_name")
              ) {
                return {
                  class: "orbis-cell-error",
                  title: (row.__errors as Map<string, string>).get("full_name"),
                };
              }
              return {};
            },
          },
          {
            prop: "email",
            name: labels.email ?? "Email",
            size: 220,
            cellProperties: (row: Record<string, unknown>) => {
              if (
                row.__errors &&
                (row.__errors as Map<string, string>).has("email")
              ) {
                return {
                  class: "orbis-cell-error",
                  title: (row.__errors as Map<string, string>).get("email"),
                };
              }
              return {};
            },
          },
          {
            prop: "phone",
            name: labels.phone ?? "Phone",
            size: 150,
            cellProperties: (row: Record<string, unknown>) => {
              if (
                row.__errors &&
                (row.__errors as Map<string, string>).has("phone")
              ) {
                return {
                  class: "orbis-cell-error",
                  title: (row.__errors as Map<string, string>).get("phone"),
                };
              }
              return {};
            },
          },
          {
            prop: "national_id",
            name: labels.national_id ?? "National ID",
            size: 150,
            cellProperties: (row: Record<string, unknown>) => {
              if (
                row.__errors &&
                (row.__errors as Map<string, string>).has("national_id")
              ) {
                return {
                  class: "orbis-cell-error",
                  title: (row.__errors as Map<string, string>).get(
                    "national_id",
                  ),
                };
              }
              return {};
            },
          },
          {
            prop: "sex",
            name: labels.sex ?? "Sex",
            size: 90,
            cellProperties: (row: Record<string, unknown>) => {
              if (
                row.__errors &&
                (row.__errors as Map<string, string>).has("sex")
              ) {
                return {
                  class: "orbis-cell-error",
                  title: (row.__errors as Map<string, string>).get("sex"),
                };
              }
              return {};
            },
          },
          {
            prop: "date_of_birth",
            name: labels.dob ?? "Date of Birth",
            size: 140,
            cellProperties: (row: Record<string, unknown>) => {
              if (
                row.__errors &&
                (row.__errors as Map<string, string>).has("date_of_birth")
              ) {
                return {
                  class: "orbis-cell-error",
                  title: (row.__errors as Map<string, string>).get(
                    "date_of_birth",
                  ),
                };
              }
              return {};
            },
          },
        ],
        requiredFields: ["full_name", "email"],
        emptyRow: () => ({
          full_name: "",
          email: "",
          phone: "",
          national_id: "",
          sex: "",
          date_of_birth: "",
          __errors: new Map<string, string>(),
        }),
      };

    case "teachers":
      return {
        columns: [
          indexColumn("#"),
          {
            prop: "full_name",
            name: labels.full_name ?? "Full Name",
            size: 200,
            cellProperties: (row: Record<string, unknown>) => {
              if (
                row.__errors &&
                (row.__errors as Map<string, string>).has("full_name")
              ) {
                return {
                  class: "orbis-cell-error",
                  title: (row.__errors as Map<string, string>).get("full_name"),
                };
              }
              return {};
            },
          },
          {
            prop: "email",
            name: labels.email ?? "Email",
            size: 220,
            cellProperties: (row: Record<string, unknown>) => {
              if (
                row.__errors &&
                (row.__errors as Map<string, string>).has("email")
              ) {
                return {
                  class: "orbis-cell-error",
                  title: (row.__errors as Map<string, string>).get("email"),
                };
              }
              return {};
            },
          },
          {
            prop: "phone",
            name: labels.phone ?? "Phone",
            size: 150,
            cellProperties: (row: Record<string, unknown>) => {
              if (
                row.__errors &&
                (row.__errors as Map<string, string>).has("phone")
              ) {
                return {
                  class: "orbis-cell-error",
                  title: (row.__errors as Map<string, string>).get("phone"),
                };
              }
              return {};
            },
          },
          {
            prop: "national_id",
            name: labels.national_id ?? "National ID",
            size: 150,
            cellProperties: (row: Record<string, unknown>) => {
              if (
                row.__errors &&
                (row.__errors as Map<string, string>).has("national_id")
              ) {
                return {
                  class: "orbis-cell-error",
                  title: (row.__errors as Map<string, string>).get(
                    "national_id",
                  ),
                };
              }
              return {};
            },
          },
          {
            prop: "sex",
            name: labels.sex ?? "Sex",
            size: 90,
            cellProperties: (row: Record<string, unknown>) => {
              if (
                row.__errors &&
                (row.__errors as Map<string, string>).has("sex")
              ) {
                return {
                  class: "orbis-cell-error",
                  title: (row.__errors as Map<string, string>).get("sex"),
                };
              }
              return {};
            },
          },
          {
            prop: "specialization",
            name: labels.specialization ?? "Specialization",
            size: 180,
          },
        ],
        requiredFields: ["full_name", "email"],
        emptyRow: () => ({
          full_name: "",
          email: "",
          phone: "",
          national_id: "",
          sex: "",
          specialization: "",
          __errors: new Map<string, string>(),
        }),
      };

    case "grades":
      return {
        columns: [
          indexColumn("#"),
          {
            prop: "student_email",
            name: labels.student_email ?? "Student Email",
            size: 220,
            cellProperties: (row: Record<string, unknown>) => {
              if (
                row.__errors &&
                (row.__errors as Map<string, string>).has("student_email")
              ) {
                return {
                  class: "orbis-cell-error",
                  title: (row.__errors as Map<string, string>).get(
                    "student_email",
                  ),
                };
              }
              return {};
            },
          },
          {
            prop: "subject_name",
            name: labels.subject ?? "Subject",
            size: 170,
            cellProperties: (row: Record<string, unknown>) => {
              if (
                row.__errors &&
                (row.__errors as Map<string, string>).has("subject_name")
              ) {
                return {
                  class: "orbis-cell-error",
                  title: (row.__errors as Map<string, string>).get(
                    "subject_name",
                  ),
                };
              }
              return {};
            },
          },
          {
            prop: "score",
            name: labels.score ?? "Score",
            size: 100,
            editor: "number",
            editorConfig: { min: 0, max: 999, step: 0.5 },
            cellProperties: (row: Record<string, unknown>) => {
              if (
                row.__errors &&
                (row.__errors as Map<string, string>).has("score")
              ) {
                return {
                  class: "orbis-cell-error",
                  title: (row.__errors as Map<string, string>).get("score"),
                };
              }
              return {};
            },
          },
          {
            prop: "max_score",
            name: labels.max_score ?? "Max Score",
            size: 110,
            editor: "number",
            editorConfig: { min: 1, max: 999, step: 1 },
            cellProperties: (row: Record<string, unknown>) => {
              if (
                row.__errors &&
                (row.__errors as Map<string, string>).has("max_score")
              ) {
                return {
                  class: "orbis-cell-error",
                  title: (row.__errors as Map<string, string>).get("max_score"),
                };
              }
              return {};
            },
          },
          {
            prop: "exam_type",
            name: labels.exam_type ?? "Exam Type",
            size: 130,
          },
          {
            prop: "semester",
            name: labels.semester ?? "Semester",
            size: 130,
            cellProperties: (row: Record<string, unknown>) => {
              if (
                row.__errors &&
                (row.__errors as Map<string, string>).has("semester")
              ) {
                return {
                  class: "orbis-cell-error",
                  title: (row.__errors as Map<string, string>).get("semester"),
                };
              }
              return {};
            },
          },
        ],
        requiredFields: ["student_email", "subject_name", "score", "semester"],
        emptyRow: () => ({
          student_email: "",
          subject_name: "",
          score: "",
          max_score: "100",
          exam_type: "final",
          semester: "",
          __errors: new Map<string, string>(),
        }),
      };
  }
}

/**
 * Get the data property keys (excluding __rowIndex and __errors)
 * for mapping pasted Excel data to columns.
 */
export function getDataProps(type: BulkInputType): string[] {
  switch (type) {
    case "students":
      return [
        "full_name",
        "email",
        "phone",
        "national_id",
        "sex",
        "date_of_birth",
      ];
    case "teachers":
      return [
        "full_name",
        "email",
        "phone",
        "national_id",
        "sex",
        "specialization",
      ];
    case "grades":
      return [
        "student_email",
        "subject_name",
        "score",
        "max_score",
        "exam_type",
        "semester",
      ];
  }
}
