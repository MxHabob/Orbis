import type { OrbisColumnDef } from "@orbis/svelte-grid";
import { api, type BulkRegisterEmployeeCommandItem } from "$lib/api";
import { splitPersonName } from "$lib/shared/utils/person-name";
import * as m from "$lib/paraglide/messages";
import type {
  BulkImportProfile,
  BulkReferenceData,
  ColumnDefinitionMetadata,
  BulkCellIssue,
  BulkChunkCommitResult,
  BulkPreviewSummary,
  EmployeePreviewDetails,
} from "../types";

export interface EmployeeRowData {
  rowNumber?: number;
  full_name_ar: string;
  full_name_en?: string;
  national_id?: string;
  gender: "male" | "female";
  birth_date?: string;
  phone?: string;
  email?: string;
  employee_number?: string;
  employee_type: string;
  job_title?: string;
  department?: string;
  hire_date?: string;
  employment_type?: string;
  basic_salary?: number;
  provision_user?: boolean | string;
  user_group_name?: string;
  [key: string]: unknown;
}

export const employeeColumnMetadata: ColumnDefinitionMetadata[] = [
  {
    id: "full_name_ar",
    headerAr: "الاسم الكامل (عربي)*",
    headerEn: "Full Name (Arabic)*",
    required: true,
    type: "text",
    example: "محمود صالح العمري",
  },
  {
    id: "full_name_en",
    headerAr: "الاسم الكامل (إنجليزي)",
    headerEn: "Full Name (English)",
    required: false,
    type: "text",
    example: "Mahmoud Saleh Al-Omari",
  },
  {
    id: "national_id",
    headerAr: "الرقم الوطني / الهوية*",
    headerEn: "National ID*",
    required: true,
    type: "text",
    example: "1087654321",
  },
  {
    id: "gender",
    headerAr: "الجنس*",
    headerEn: "Gender*",
    required: true,
    type: "enum",
    allowedValues: ["male", "female", "ذكر", "أنثى"],
    options: [
      { value: "male", label: m.bulk_entry_enum_gender_male() },
      { value: "female", label: m.bulk_entry_enum_gender_female() },
    ],
    example: "male",
  },
  {
    id: "employee_type",
    headerAr: "النوع الوظيفي*",
    headerEn: "Employee Type*",
    required: true,
    type: "enum",
    allowedValues: [
      "teacher",
      "admin",
      "principal",
      "accountant",
      "receptionist",
      "supervisor",
      "worker",
      "معلم",
      "إداري",
      "مدير",
      "محاسب",
      "مشرف",
    ],
    options: [
      { value: "teacher", label: m.bulk_entry_enum_employee_teacher() },
      { value: "admin", label: m.bulk_entry_enum_employee_admin() },
      { value: "principal", label: m.bulk_entry_enum_employee_principal() },
      { value: "accountant", label: m.bulk_entry_enum_employee_accountant() },
      { value: "supervisor", label: m.bulk_entry_enum_employee_supervisor() },
      { value: "worker", label: m.bulk_entry_enum_employee_worker() },
    ],
    example: "teacher",
  },
  {
    id: "job_title",
    headerAr: "المسمى الوظيفي",
    headerEn: "Job Title",
    required: false,
    type: "text",
    example: "معلم رياضيات أول",
  },
  {
    id: "department",
    headerAr: "القسم / الإدارة",
    headerEn: "Department",
    required: false,
    type: "text",
    example: "قسم العلوم والرياضيات",
  },
  {
    id: "hire_date",
    headerAr: "تاريخ التعيين*",
    headerEn: "Hire Date*",
    required: true,
    type: "date",
    format: "YYYY-MM-DD",
    example: "2024-09-01",
  },
  {
    id: "employment_type",
    headerAr: "نوع العقد",
    headerEn: "Employment Type",
    required: false,
    type: "enum",
    allowedValues: ["full_time", "part_time", "contractor", "دوام كامل", "دوام جزئي", "عقد محدد"],
    options: [
      { value: "full_time", label: m.bulk_entry_enum_employment_full_time() },
      { value: "part_time", label: m.bulk_entry_enum_employment_part_time() },
      { value: "contractor", label: m.bulk_entry_enum_employment_contractor() },
    ],
    example: "full_time",
  },
  {
    id: "phone",
    headerAr: "رقم الهاتف",
    headerEn: "Phone",
    required: false,
    type: "text",
    example: "771122334",
  },
  {
    id: "email",
    headerAr: "البريد الإلكتروني",
    headerEn: "Email",
    required: false,
    type: "text",
    example: "mahmoud.omari@orbis.edu",
  },
  {
    id: "provision_user",
    headerAr: "إنشاء حساب دخول للنظام",
    headerEn: "Provision User Account",
    required: false,
    type: "boolean",
    example: "نعم",
  },
  {
    id: "user_group_name",
    headerAr: "مجموعة الصلاحيات",
    headerEn: "User Group",
    required: false,
    type: "reference",
    referenceSource: "userGroups",
    example: "المعلمين",
  },
];

export const employeeGridColumns: OrbisColumnDef[] = [
  { id: "__idx", header: m.bulk_entry_grid_row_number(), width: 50, pin: "left", readonly: true },
  { id: "status", header: m.bulk_entry_grid_status(), width: 110, pin: "left", type: "badge", readonly: true },
  { id: "full_name_ar", header: m.bulk_entry_employee_header_first_name_ar(), width: 200 },
  { id: "full_name_en", header: "Full Name (English)", width: 200 },
  { id: "national_id", header: m.bulk_entry_employee_header_national_id(), width: 130 },
  { id: "gender", header: m.bulk_entry_header_gender(), width: 90, type: "enum", options: [{ value: "male", label: m.bulk_entry_enum_gender_male() }, { value: "female", label: m.bulk_entry_enum_gender_female() }] },
  { id: "employee_type", header: m.bulk_entry_employee_header_employee_type(), width: 140 },
  { id: "job_title", header: m.bulk_entry_employee_header_job_title(), width: 150 },
  { id: "department", header: m.bulk_entry_employee_header_department(), width: 140 },
  { id: "hire_date", header: m.bulk_entry_employee_header_hire_date(), width: 120, type: "date" },
  { id: "phone", header: m.bulk_entry_header_phone(), width: 120 },
  { id: "email", header: m.bulk_entry_header_email(), width: 160 },
  { id: "provision_user", header: m.bulk_entry_employee_header_provision_user(), width: 100, type: "boolean" },
  { id: "user_group_name", header: m.bulk_entry_employee_header_user_group(), width: 140 },
];

export const employeeImportProfile: BulkImportProfile<EmployeeRowData, EmployeePreviewDetails> = {
  id: "employees",
  displayName: {
    ar: "استيراد وقيد موظفي وكادر المدرسة",
    en: "Staff & Employee Bulk Registration",
  },
  description: {
    ar: "إدخال بيانات الموظفين والمعلمين، وتعيين المجموعات وتوليد بيانات الدخول الآمنة آلياً",
    en: "Bulk register staff, teachers, assign user groups, and automatically provision CSPRNG credentials",
  },
  templateVersion: "ORBIS-TPL-EMP-2026.2",
  columnMetadata: employeeColumnMetadata,
  gridColumns: employeeGridColumns,

  async buildReferenceData(_orgId: string): Promise<BulkReferenceData> {
    const groupsRes = await api.listGroups().catch(() => []);
    const userGroups = (groupsRes || []).map((g: any) => ({
      id: g.id,
      name: g.name || "",
    }));

    return {
      classes: [],
      sections: [],
      levels: [],
      subjects: [],
      feeTemplates: [],
      discountTemplates: [],
      userGroups,
      assessmentSchemes: [],
      academicPeriods: [],
      enums: {
        genders: [
          { value: "male", labelAr: "ذكر", labelEn: "Male" },
          { value: "female", labelAr: "أنثى", labelEn: "Female" },
        ],
        studyModes: [],
        employeeTypes: [
          { value: "teacher", labelAr: "معلم", labelEn: "Teacher" },
          { value: "admin", labelAr: "إداري", labelEn: "Administrator" },
          { value: "principal", labelAr: "مدير", labelEn: "Principal" },
          { value: "accountant", labelAr: "محاسب", labelEn: "Accountant" },
          { value: "supervisor", labelAr: "مشرف", labelEn: "Supervisor" },
          { value: "worker", labelAr: "خدمات مساندة", labelEn: "Worker" },
        ],
        employmentTypes: [
          { value: "full_time", labelAr: "دوام كامل", labelEn: "Full-time" },
          { value: "part_time", labelAr: "دوام جزئي", labelEn: "Part-time" },
          { value: "contractor", labelAr: "عقد محدد", labelEn: "Contractor" },
        ],
        bloodTypes: ["A+", "A-", "B+", "B-", "AB+", "AB-", "O+", "O-"],
        relationships: [],
      },
    };
  },

  validateCell(colId: string, value: unknown, _row: EmployeeRowData, _refData: BulkReferenceData): BulkCellIssue | null {
    const strVal = String(value ?? "").trim();

    if (["full_name_ar", "national_id", "employee_type", "hire_date"].includes(colId)) {
      if (!strVal) {
        return {
          severity: "BLOCKING_ERROR",
          columnId: colId,
          message: m.bulk_entry_employee_validation_required(),
        };
      }
    }

    if (colId === "hire_date" && strVal) {
      const parsed = Date.parse(strVal);
      if (isNaN(parsed)) {
        return {
          severity: "BLOCKING_ERROR",
          columnId: colId,
          message: m.bulk_entry_employee_validation_hire_date_format(),
        };
      }
    }

    return null;
  },

  validateRow(row: EmployeeRowData, allRows: EmployeeRowData[], rowIndex: number, _refData: BulkReferenceData): BulkCellIssue[] {
    const issues: BulkCellIssue[] = [];

    if (row.national_id?.trim()) {
      const nid = row.national_id.trim();
      const duplicateIndex = allRows.findIndex((r, idx) => idx !== rowIndex && r.national_id?.trim() === nid);
      if (duplicateIndex !== -1) {
        issues.push({
          severity: "BLOCKING_ERROR",
          columnId: "national_id",
          message: m.bulk_entry_employee_validation_duplicate_national_id({ row: duplicateIndex + 1 }),
        });
      }
    }

    return issues;
  },

  async generatePreview(rows: EmployeeRowData[], _refData: BulkReferenceData): Promise<BulkPreviewSummary<EmployeePreviewDetails>> {
    let validCount = 0;
    let errorCount = 0;
    let provisionCount = 0;
    const depts: Record<string, number> = {};
    const groups: Record<string, number> = {};

    for (const row of rows) {
      if (!row.full_name_ar?.trim() || !row.national_id?.trim() || !row.employee_type || !row.hire_date) {
        errorCount++;
      } else {
        validCount++;
        const d = row.department || "عام (General)";
        depts[d] = (depts[d] || 0) + 1;

        const isProvision =
          row.provision_user === true ||
          ["true", "1", "نعم", "yes"].includes(String(row.provision_user ?? "").trim().toLowerCase());
        if (isProvision) provisionCount++;

        const g = row.user_group_name || "الافتراضية";
        groups[g] = (groups[g] || 0) + 1;
      }
    }

    return {
      totalRows: rows.length,
      validRowsCount: validCount,
      errorRowsCount: errorCount,
      warningRowsCount: 0,
      details: {
        employeesToCreate: validCount,
        personsToCreate: validCount,
        personsToLink: 0,
        userAccountsToProvision: provisionCount,
        departmentsSummary: depts,
        userGroupsSummary: groups,
      },
    };
  },

  async commitChunk(
    rows: EmployeeRowData[],
    _orgId: string,
    refData: BulkReferenceData,
  ): Promise<BulkChunkCommitResult> {
    const items: BulkRegisterEmployeeCommandItem[] = [];

    for (let i = 0; i < rows.length; i++) {
      const row = rows[i]!;
      const fullNameAr = row.full_name_ar.trim();
      const namePartsAr = splitPersonName(fullNameAr, "موظف");
      const fullNameEn = row.full_name_en?.trim() || fullNameAr;
      const namePartsEn = splitPersonName(fullNameEn, "Employee");

      let typeStr = "teacher";
      const rawType = String(row.employee_type || "").toLowerCase().trim();
      if (rawType.includes("admin") || rawType.includes("إداري")) typeStr = "admin";
      if (rawType.includes("principal") || rawType.includes("مدير")) typeStr = "principal";
      if (rawType.includes("accountant") || rawType.includes("محاسب")) typeStr = "accountant";
      if (rawType.includes("supervisor") || rawType.includes("مشرف")) typeStr = "supervisor";
      if (rawType.includes("worker") || rawType.includes("خدمات")) typeStr = "worker";

      const isProvision =
        row.provision_user === true ||
        ["true", "1", "نعم", "yes"].includes(
          String(row.provision_user ?? "").trim().toLowerCase(),
        );

      let matchedGroupIds: string[] = [];
      if (row.user_group_name?.trim()) {
        const g = refData.userGroups.find((grp) => grp.name.toLowerCase() === row.user_group_name?.toLowerCase());
        if (g?.id) matchedGroupIds.push(g.id);
      }

      items.push({
        row_index: row.rowNumber || i + 1,
        registration: {
          first_name: namePartsEn.firstName,
          middle_name: namePartsEn.middleName,
          last_name: namePartsEn.lastName,
          first_name_ar: namePartsAr.firstName,
          middle_name_ar: namePartsAr.middleName,
          last_name_ar: namePartsAr.lastName,
          display_name: fullNameAr,
          national_id: row.national_id?.trim() || undefined,
          gender: row.gender || "male",
          phone: row.phone?.trim() || undefined,
          email: row.email?.trim() || undefined,
          employee_type: typeStr,
          job_title: row.job_title?.trim() || undefined,
          department: row.department?.trim() || undefined,
          hire_date: row.hire_date?.trim() || new Date().toISOString().slice(0, 10),
          employment_type: (row.employment_type as any) || "full_time",
          account: isProvision
            ? {
                enabled: true,
                preferred_username: row.email?.trim() || row.employee_number?.trim() || undefined,
                role: typeStr === "teacher" ? "teacher" : "staff",
                group_ids: matchedGroupIds,
                delivery_channel: row.email?.trim() ? "email" : row.phone?.trim() ? "sms" : "manual",
              }
            : undefined,
        },
      });
    }

    try {
      const bulkRes = await api.bulkRegisterEmployees(items, { atomic: false });
      const rowErrors: Array<{ rowNumber: number; reason: string }> = [];

      for (const it of bulkRes.items) {
        if (!it.success && it.error) {
          rowErrors.push({
            rowNumber: it.row_index,
            reason: m.bulk_entry_row_error({ row: it.row_index, reason: it.error }),
          });
        }
      }

      return {
        success: true,
        rowErrors: rowErrors.length > 0 ? rowErrors : undefined,
      };
    } catch (err: any) {
      return {
        success: false,
        errors: [err.message || String(err)],
      };
    }
  },
};
