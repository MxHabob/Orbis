import * as m from "$lib/paraglide/messages";
/**
 * @module student-columns
 * Column definitions for bulk student import grid.
 * Displays all data properties supported by the backend without hidden extended subsets.
 */

import { indexColumn, type OrbisColumnDef } from "@orbis/svelte-grid";

/** All data property keys in column order (for paste mapping) */
export const STUDENT_DATA_PROPS = [
  "full_name",
  "name_ar",
  "email",
  "phone",
  "national_id",
  "sex",
  "date_of_birth",
  "class_id",
  "enrollment_status",
  "nationality",
  "address",
  "city",
  "country",
  "guardian_name",
  "guardian_phone",
  "guardian_relationship",
  "blood_type",
  "emergency_contact",
  "allergies",
  "medical_conditions",
] as const;

export type StudentDataProp = (typeof STUDENT_DATA_PROPS)[number];

/**
 * Build column definitions for all backend-supported student properties.
 */
export function buildStudentColumns(
  labels: Record<string, string>,
  _visibleExtended?: Set<string>,
  classOptions?: Array<{ id: string; name: string }>,
): OrbisColumnDef[] {
  return [
    indexColumn("#"),
    {
      id: "full_name",
      prop: "full_name",
      header: labels.full_name ?? "Full Name",
      name: labels.full_name ?? "Full Name",
      width: 190,
      size: 190,
      editor: "text",
      pin: "left",
    },
    {
      id: "name_ar",
      prop: "name_ar",
      header: labels.name_ar ?? "Name (Arabic)",
      name: labels.name_ar ?? "Name (Arabic)",
      width: 170,
      size: 170,
      editor: "text",
    },
    {
      id: "email",
      prop: "email",
      header: labels.email ?? "Email",
      name: labels.email ?? "Email",
      width: 210,
      size: 210,
      editor: "text",
    },
    {
      id: "phone",
      prop: "phone",
      header: labels.phone ?? "Phone",
      name: labels.phone ?? "Phone",
      width: 140,
      size: 140,
      editor: "text",
    },
    {
      id: "national_id",
      prop: "national_id",
      header: labels.national_id ?? "National ID",
      name: labels.national_id ?? "National ID",
      width: 140,
      size: 140,
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
        { value: "male", label: m.common_form_male() },
        { value: "female", label: m.common_form_female() },
      ],
      align: "center",
    },
    {
      id: "date_of_birth",
      prop: "date_of_birth",
      header: labels.dob ?? "Date of Birth",
      name: labels.dob ?? "Date of Birth",
      width: 130,
      size: 130,
      type: "date",
      editor: "datepicker",
      align: "center",
    },
    {
      id: "class_id",
      prop: "class_id",
      header: labels.class ?? "Class",
      name: labels.class ?? "Class",
      width: 160,
      size: 160,
      editor: classOptions && classOptions.length > 0 ? "combo" : "text",
      options:
        classOptions?.map((c) => ({ value: c.name, label: c.name })) ||
        [],
    },
    {
      id: "enrollment_status",
      prop: "enrollment_status",
      header: labels.enrollment_status ?? "Status",
      name: labels.enrollment_status ?? "Status",
      width: 120,
      size: 120,
      type: "badge",
      editor: "combo",
      options: [
        { value: "active", label: m.common_phase_active() },
        { value: "withdrawn", label: m.common_status_withdrawn() },
      ],
      align: "center",
    },
    {
      id: "nationality",
      prop: "nationality",
      header: labels.nationality ?? "Nationality",
      name: labels.nationality ?? "Nationality",
      width: 130,
      size: 130,
      editor: "text",
    },
    {
      id: "address",
      prop: "address",
      header: labels.address ?? "Address",
      name: labels.address ?? "Address",
      width: 180,
      size: 180,
      editor: "text",
    },
    {
      id: "city",
      prop: "city",
      header: labels.city ?? "City",
      name: labels.city ?? "City",
      width: 110,
      size: 110,
      editor: "text",
    },
    {
      id: "country",
      prop: "country",
      header: labels.country ?? "Country",
      name: labels.country ?? "Country",
      width: 110,
      size: 110,
      editor: "text",
    },
    {
      id: "guardian_name",
      prop: "guardian_name",
      header: labels.guardian_name ?? "Guardian Name",
      name: labels.guardian_name ?? "Guardian Name",
      width: 170,
      size: 170,
      editor: "text",
    },
    {
      id: "guardian_phone",
      prop: "guardian_phone",
      header: labels.guardian_phone ?? "Guardian Phone",
      name: labels.guardian_phone ?? "Guardian Phone",
      width: 140,
      size: 140,
      editor: "text",
    },
    {
      id: "guardian_relationship",
      prop: "guardian_relationship",
      header: labels.guardian_rel ?? "Relationship",
      name: labels.guardian_rel ?? "Relationship",
      width: 130,
      size: 130,
      editor: "combo",
      options: [
        {
          value: "father",
          label: m.common_guardians_relationship_father?.() || "Father",
        },
        {
          value: "mother",
          label: m.common_guardians_relationship_mother?.() || "Mother",
        },
        {
          value: "other",
          label: m.common_guardians_relationship_other?.() || "Other",
        },
      ],
      align: "center",
    },
    {
      id: "blood_type",
      prop: "blood_type",
      header: labels.blood_type ?? "Blood Type",
      name: labels.blood_type ?? "Blood Type",
      width: 100,
      size: 100,
      editor: "combo",
      options: ["A+", "A-", "B+", "B-", "AB+", "AB-", "O+", "O-"].map((bt) => ({
        value: bt,
        label: bt,
      })),
      align: "center",
    },
    {
      id: "emergency_contact",
      prop: "emergency_contact",
      header: labels.emergency ?? "Emergency Contact",
      name: labels.emergency ?? "Emergency Contact",
      width: 150,
      size: 150,
      editor: "text",
    },
    {
      id: "allergies",
      prop: "allergies",
      header: labels.allergies ?? "Allergies",
      name: labels.allergies ?? "Allergies",
      width: 150,
      size: 150,
      editor: "text",
    },
    {
      id: "medical_conditions",
      prop: "medical_conditions",
      header: labels.medical ?? "Medical Conditions",
      name: labels.medical ?? "Medical Conditions",
      width: 170,
      size: 170,
      editor: "text",
    },
  ];
}

/**
 * Create an empty student row with all properties initialized.
 */
export function createEmptyStudentRow(): Record<string, unknown> {
  const row: Record<string, unknown> = {};
  for (const prop of STUDENT_DATA_PROPS) {
    row[prop] = "";
  }
  row.__errors = new Map<string, string>();
  return row;
}
