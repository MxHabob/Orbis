import * as m from "$lib/paraglide/messages";
/**
 * @module student-columns
 * Column definitions for bulk student import grid.
 * Organized in groups: Essential, Enrollment, Location, Guardian, Medical, Financial.
 */

import type { OrbisColumnDef } from "@orbis/svelte-grid";
import { indexColumn } from "$lib/tables";

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

/** Essential columns always visible */
export const ESSENTIAL_PROPS: StudentDataProp[] = [
  "full_name",
  "name_ar",
  "email",
  "phone",
  "national_id",
  "sex",
  "date_of_birth",
  "class_id",
];

/** Extended columns shown on demand */
export const EXTENDED_PROPS: StudentDataProp[] = [
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
];

// Error cell helper — DRY approach instead of repeating per column
function errorCellProps(prop: string) {
  return (row: Record<string, unknown>) => {
    const errors = row.__errors as Map<string, string> | undefined;
    if (errors?.has(prop)) {
      return { class: "orbis-cell-error", title: errors.get(prop) };
    }
    return {};
  };
}

/**
 * Build column definitions from labels and visible props.
 */
export function buildStudentColumns(
  labels: Record<string, string>,
  visibleExtended: Set<string>,
  classOptions?: Array<{ id: string; name: string }>,
): OrbisColumnDef[] {
  const cols: OrbisColumnDef[] = [indexColumn("#")];

  // ── Essential Columns ──────────────────────────────────────────
  cols.push({
    prop: "full_name",
    name: labels.full_name ?? "Full Name",
    size: 190,
    editor: "text",
    cellProperties: errorCellProps("full_name"),
  });

  cols.push({
    prop: "name_ar",
    name: labels.name_ar ?? "Name (Arabic)",
    size: 170,
    editor: "text",
    cellProperties: errorCellProps("name_ar"),
  });

  cols.push({
    prop: "email",
    name: labels.email ?? "Email",
    size: 210,
    editor: "text",
    cellProperties: errorCellProps("email"),
  });

  cols.push({
    prop: "phone",
    name: labels.phone ?? "Phone",
    size: 140,
    editor: "text",
    cellProperties: errorCellProps("phone"),
  });

  cols.push({
    prop: "national_id",
    name: labels.national_id ?? "National ID",
    size: 140,
    editor: "text",
    cellProperties: errorCellProps("national_id"),
  });

  cols.push({
    prop: "sex",
    name: labels.sex ?? "Sex",
    size: 85,
    editor: "text",
    cellProperties: errorCellProps("sex"),
  });

  cols.push({
    prop: "date_of_birth",
    name: labels.dob ?? "Date of Birth",
    size: 130,
    editor: "datepicker",
    cellProperties: errorCellProps("date_of_birth"),
  });

  cols.push({
    prop: "class_id",
    name: labels.class ?? "Class",
    size: 160,
    editor: classOptions && classOptions.length > 0 ? "combo" : "text",
    options:
      classOptions?.map((c) => ({ value: c.id, label: c.name })) ||
      [],
    cellProperties: errorCellProps("class_id"),
  });

  // ── Extended Columns (shown on toggle) ─────────────────────────
  if (visibleExtended.has("enrollment_status")) {
    cols.push({
      prop: "enrollment_status",
      name: labels.enrollment_status ?? "Status",
      size: 110,
      editor: "combo",
      options: [
        { value: "active", label: m.common_phase_active() },
        {
          value: "withdrawn",
          label: m.common_status_withdrawn(),
        },
      ],
    });
  }

  if (visibleExtended.has("nationality")) {
    cols.push({
      prop: "nationality",
      name: labels.nationality ?? "Nationality",
      size: 130,
      editor: "text",
    });
  }

  if (visibleExtended.has("address")) {
    cols.push({
      prop: "address",
      name: labels.address ?? "Address",
      size: 180,
      editor: "text",
    });
  }

  if (visibleExtended.has("city")) {
    cols.push({
      prop: "city",
      name: labels.city ?? "City",
      size: 110,
      editor: "text",
    });
  }

  if (visibleExtended.has("country")) {
    cols.push({
      prop: "country",
      name: labels.country ?? "Country",
      size: 110,
      editor: "text",
    });
  }

  if (visibleExtended.has("guardian_name")) {
    cols.push({
      prop: "guardian_name",
      name: labels.guardian_name ?? "Guardian Name",
      size: 170,
      editor: "text",
      cellProperties: errorCellProps("guardian_name"),
    });
  }

  if (visibleExtended.has("guardian_phone")) {
    cols.push({
      prop: "guardian_phone",
      name: labels.guardian_phone ?? "Guardian Phone",
      size: 140,
      editor: "text",
    });
  }

  if (visibleExtended.has("guardian_relationship")) {
    cols.push({
      prop: "guardian_relationship",
      name: labels.guardian_rel ?? "Relationship",
      size: 120,
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
    });
  }

  if (visibleExtended.has("blood_type")) {
    cols.push({
      prop: "blood_type",
      name: labels.blood_type ?? "Blood Type",
      size: 100,
      editor: "combo",
      options: ["A+", "A-", "B+", "B-", "AB+", "AB-", "O+", "O-"].map((bt) => ({
        value: bt,
        label: bt,
      })),
    });
  }

  if (visibleExtended.has("emergency_contact")) {
    cols.push({
      prop: "emergency_contact",
      name: labels.emergency ?? "Emergency Contact",
      size: 150,
      editor: "text",
    });
  }

  if (visibleExtended.has("allergies")) {
    cols.push({
      prop: "allergies",
      name: labels.allergies ?? "Allergies",
      size: 150,
      editor: "text",
    });
  }

  if (visibleExtended.has("medical_conditions")) {
    cols.push({
      prop: "medical_conditions",
      name: labels.medical ?? "Medical Conditions",
      size: 170,
      editor: "text",
    });
  }

  return cols;
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
