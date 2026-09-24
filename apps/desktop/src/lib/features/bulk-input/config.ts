/**
 * @module bulk-input-config
 * Column configurations and metadata for each bulk input type.
 * Dynamically switches columns based on selected data type.
 */

import { indexColumn, type OrbisColumnDef } from "@orbis/svelte-grid";

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
            id: "full_name",
            prop: "full_name",
            header: labels.full_name ?? "Full Name",
            name: labels.full_name ?? "Full Name",
            width: 200,
            size: 200,
            editor: "text",
          },
          {
            id: "email",
            prop: "email",
            header: labels.email ?? "Email",
            name: labels.email ?? "Email",
            width: 220,
            size: 220,
            editor: "text",
          },
          {
            id: "phone",
            prop: "phone",
            header: labels.phone ?? "Phone",
            name: labels.phone ?? "Phone",
            width: 150,
            size: 150,
            editor: "text",
          },
          {
            id: "national_id",
            prop: "national_id",
            header: labels.national_id ?? "National ID",
            name: labels.national_id ?? "National ID",
            width: 150,
            size: 150,
            editor: "text",
          },
          {
            id: "sex",
            prop: "sex",
            header: labels.sex ?? "Sex",
            name: labels.sex ?? "Sex",
            width: 90,
            size: 90,
            editor: "combo",
            options: [
              { value: "male", label: "Male" },
              { value: "female", label: "Female" },
            ],
            align: "center",
          },
          {
            id: "date_of_birth",
            prop: "date_of_birth",
            header: labels.dob ?? "Date of Birth",
            name: labels.dob ?? "Date of Birth",
            width: 140,
            size: 140,
            type: "date",
            editor: "datepicker",
            align: "center",
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
            id: "full_name",
            prop: "full_name",
            header: labels.full_name ?? "Full Name",
            name: labels.full_name ?? "Full Name",
            width: 200,
            size: 200,
            editor: "text",
          },
          {
            id: "email",
            prop: "email",
            header: labels.email ?? "Email",
            name: labels.email ?? "Email",
            width: 220,
            size: 220,
            editor: "text",
          },
          {
            id: "phone",
            prop: "phone",
            header: labels.phone ?? "Phone",
            name: labels.phone ?? "Phone",
            width: 150,
            size: 150,
            editor: "text",
          },
          {
            id: "national_id",
            prop: "national_id",
            header: labels.national_id ?? "National ID",
            name: labels.national_id ?? "National ID",
            width: 150,
            size: 150,
            editor: "text",
          },
          {
            id: "sex",
            prop: "sex",
            header: labels.sex ?? "Sex",
            name: labels.sex ?? "Sex",
            width: 90,
            size: 90,
            editor: "combo",
            options: [
              { value: "male", label: "Male" },
              { value: "female", label: "Female" },
            ],
            align: "center",
          },
          {
            id: "specialization",
            prop: "specialization",
            header: labels.specialization ?? "Specialization",
            name: labels.specialization ?? "Specialization",
            width: 180,
            size: 180,
            editor: "text",
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
            id: "student_email",
            prop: "student_email",
            header: labels.student_email ?? "Student Email",
            name: labels.student_email ?? "Student Email",
            width: 220,
            size: 220,
            editor: "text",
          },
          {
            id: "subject_name",
            prop: "subject_name",
            header: labels.subject ?? "Subject",
            name: labels.subject ?? "Subject",
            width: 170,
            size: 170,
            editor: "text",
          },
          {
            id: "score",
            prop: "score",
            header: labels.score ?? "Score",
            name: labels.score ?? "Score",
            width: 100,
            size: 100,
            type: "number",
            editor: "number",
            align: "end",
          },
          {
            id: "max_score",
            prop: "max_score",
            header: labels.max_score ?? "Max Score",
            name: labels.max_score ?? "Max Score",
            width: 110,
            size: 110,
            type: "number",
            editor: "number",
            align: "end",
          },
          {
            id: "exam_type",
            prop: "exam_type",
            header: labels.exam_type ?? "Exam Type",
            name: labels.exam_type ?? "Exam Type",
            width: 130,
            size: 130,
            editor: "text",
          },
          {
            id: "semester",
            prop: "semester",
            header: labels.semester ?? "Semester",
            name: labels.semester ?? "Semester",
            width: 130,
            size: 130,
            editor: "text",
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
