import * as m from "$lib/paraglide/messages";
import { getLocale } from "$lib/paraglide/runtime";
import type { BulkImportProfile, BulkProfileId } from "./types";
import { normalizeRegistrationFieldKey } from "$lib/features/students/registration-fields";

const localizedHeader = (ar: string, en: string) => () => (getLocale() === "ar" ? ar : en);

const profileNames: Record<BulkProfileId, () => string> = {
  students: m.bulk_entry_profile_students,
  employees: m.bulk_entry_profile_employees,
  grades: m.bulk_entry_profile_grades,
  "teacher-subjects": m.bulk_entry_profile_teacher_subjects,
};

const columnHeaders: Record<BulkProfileId, Record<string, () => string>> = {
  students: {
    full_name_ar: m.bulk_entry_student_header_first_name_ar,
    full_name_en: m.bulk_entry_student_header_first_name,
    place_of_birth: localizedHeader("مكان الميلاد", "Place of Birth"),
    nationality: localizedHeader("الجنسية", "Nationality"),
    passport_number: localizedHeader("رقم جواز السفر", "Passport Number"),
    blood_type: localizedHeader("فصيلة الدم", "Blood Type"),
    study_mode: localizedHeader("نمط الدراسة", "Study Mode"),
    address: localizedHeader("العنوان", "Address"),
    medical_notes: localizedHeader("ملاحظات طبية", "Medical Notes"),
    special_needs_notes: localizedHeader("الاحتياجات الخاصة", "Special Needs"),
    create_login_account: m.bulk_entry_employee_header_provision_user,
    user_group_names: m.bulk_entry_user_groups_header,
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
  },
  employees: {
    full_name_ar: m.bulk_entry_employee_header_first_name_ar,
    full_name_en: localizedHeader("الاسم الكامل (إنجليزي)", "Full Name (English)"),
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
  "teacher-subjects": {
    class_code: localizedHeader("رمز الصف أو معرفه", "Class Code or ID"),
    academic_period_code: localizedHeader(
      "رمز الفترة الأكاديمية أو معرفها",
      "Academic Period Code or ID",
    ),
    subject_code: localizedHeader("رمز المادة أو معرفها", "Subject Code or ID"),
    teacher_code: localizedHeader(
      "الرقم الوظيفي للمعلم أو معرفه",
      "Teacher Employee Number or ID",
    ),
    weekly_hours: localizedHeader("الحصص الأسبوعية", "Weekly Hours"),
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
  required?: boolean,
): string {
  const header = columnHeaders[profileId][columnId]?.() ?? fallback;
  if (required === undefined) return header;
  return `${header.replace(/\*$/, "")}${required ? "*" : ""}`;
}

export function getBulkEntryGridColumns<T>(
  profile: BulkImportProfile<T, any>,
  registrationFields?: Record<string, { active: boolean; required: boolean }>,
): BulkImportProfile<T, any>["gridColumns"] {
  return profile.gridColumns
    .filter((column) => {
      if (profile.id !== "students" || !column.id || !registrationFields) return true;
      if (column.id === "full_name_ar" || column.id === "class_name") return true;
      return registrationFields[normalizeRegistrationFieldKey(column.id)]?.active !== false;
    })
    .map((column) => {
    const policy = column.id ? registrationFields?.[normalizeRegistrationFieldKey(column.id)] : undefined;
    const header =
      (column.id
        ? getBulkEntryColumnHeader(profile.id, column.id, column.header, policy?.required)
        : undefined) ??
      (column.id ? gridHeaders[column.id]?.() : undefined) ??
      column.header;
    const options = column.options?.map((option) => ({
      ...option,
      label: gridEnumLabels[`${column.id}:${String(option.value)}`]?.() ?? option.label,
    }));
    return { ...column, header, ...(options ? { options } : {}) };
  });
}
