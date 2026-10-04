import * as m from "$lib/paraglide/messages";
import type { BulkImportProfile, BulkProfileId } from "./types";

const profileNames: Record<BulkProfileId, () => string> = {
  students: m.bulk_entry_profile_students,
  employees: m.bulk_entry_profile_employees,
  grades: m.bulk_entry_profile_grades,
};

const columnHeaders: Record<BulkProfileId, Record<string, () => string>> = {
  students: {
    first_name_ar: m.bulk_entry_student_header_first_name_ar,
    middle_name_ar: m.bulk_entry_student_header_middle_name_ar,
    last_name_ar: m.bulk_entry_student_header_last_name_ar,
    first_name: m.bulk_entry_student_header_first_name,
    last_name: m.bulk_entry_student_header_last_name,
    national_id: m.bulk_entry_student_header_national_id,
    gender: m.bulk_entry_header_gender,
    birth_date: m.bulk_entry_student_header_birth_date,
    student_number: m.bulk_entry_student_header_student_number,
    class_name: m.bulk_entry_student_header_class_name,
    phone: m.bulk_entry_header_phone,
    email: m.bulk_entry_header_email,
    guardian_name: m.bulk_entry_student_header_guardian_name,
    guardian_phone: m.bulk_entry_student_header_guardian_phone,
    guardian_relationship: m.bulk_entry_student_header_guardian_relationship,
    fee_template_name: m.bulk_entry_student_header_fee_template,
  },
  employees: {
    first_name_ar: m.bulk_entry_employee_header_first_name_ar,
    middle_name_ar: m.bulk_entry_employee_header_middle_name_ar,
    last_name_ar: m.bulk_entry_employee_header_last_name_ar,
    national_id: m.bulk_entry_employee_header_national_id,
    gender: m.bulk_entry_header_gender,
    employee_type: m.bulk_entry_employee_header_employee_type,
    job_title: m.bulk_entry_employee_header_job_title,
    department: m.bulk_entry_employee_header_department,
    hire_date: m.bulk_entry_employee_header_hire_date,
    employment_type: m.bulk_entry_employee_header_employment_type,
    phone: m.bulk_entry_header_phone,
    email: m.bulk_entry_header_email,
    provision_user: m.bulk_entry_employee_header_provision_user,
    user_group_name: m.bulk_entry_employee_header_user_group,
  },
  grades: {
    student_number: m.bulk_entry_grade_header_student_number,
    student_name: m.bulk_entry_grade_header_student_name,
    subject_name: m.bulk_entry_grade_header_subject_name,
    assessment_type: m.bulk_entry_grade_header_assessment_type,
    score: m.bulk_entry_grade_header_score,
    max_score: m.bulk_entry_grade_header_max_score,
    is_excused: m.bulk_entry_grade_header_is_excused,
    notes: m.bulk_entry_grade_header_notes,
  },
};

const gridHeaders: Record<string, () => string> = {
  __idx: m.bulk_entry_grid_row_number,
  status: m.bulk_entry_grid_status,
};

const gridEnumLabels: Record<string, () => string> = {
  "gender:male": m.bulk_entry_enum_gender_male,
  "gender:female": m.bulk_entry_enum_gender_female,
};

export function getBulkEntryProfileName(profileId: BulkProfileId): string {
  return profileNames[profileId]();
}

export function getBulkEntryColumnHeader(
  profileId: BulkProfileId,
  columnId: string,
  fallback: string,
): string {
  return columnHeaders[profileId][columnId]?.() ?? fallback;
}

export function getBulkEntryGridColumns<T>(
  profile: BulkImportProfile<T, any>,
): BulkImportProfile<T, any>["gridColumns"] {
  return profile.gridColumns.map((column) => {
    const header =
      columnHeaders[profile.id][column.id]?.() ??
      gridHeaders[column.id]?.() ??
      column.header;
    const options = column.options?.map((option) => ({
      ...option,
      label: gridEnumLabels[`${column.id}:${String(option.value)}`]?.() ?? option.label,
    }));
    return { ...column, header, ...(options ? { options } : {}) };
  });
}
