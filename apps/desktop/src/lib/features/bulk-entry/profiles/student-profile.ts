import type { OrbisColumnDef } from "@orbis/svelte-grid";
import { api, type BulkRegisterStudentCommandItem } from "$lib/api";
import { orgContext } from "$lib/stores/org-context.svelte";
import type {
  BulkImportProfile,
  BulkReferenceData,
  ColumnDefinitionMetadata,
  BulkCellIssue,
  BulkPreviewSummary,
  StudentPreviewDetails,
} from "../types";

export interface StudentRowData {
  rowNumber?: number;
  first_name: string;
  middle_name?: string;
  last_name: string;
  first_name_ar?: string;
  middle_name_ar?: string;
  last_name_ar?: string;
  national_id?: string;
  passport_number?: string;
  gender: "male" | "female";
  birth_date: string;
  phone?: string;
  email?: string;
  student_number?: string;
  class_name: string;
  study_mode?: string;
  guardian_name?: string;
  guardian_phone?: string;
  guardian_relationship?: string;
  guardian_national_id?: string;
  fee_template_name?: string;
  [key: string]: unknown;
}

export const studentColumnMetadata: ColumnDefinitionMetadata[] = [
  {
    id: "first_name_ar",
    headerAr: "الاسم الأول (عربي)*",
    headerEn: "First Name (Arabic)*",
    required: true,
    type: "text",
    descriptionAr: "الاسم الأول للطالب باللغة العربية",
    example: "أحمد",
  },
  {
    id: "middle_name_ar",
    headerAr: "اسم الأب (عربي)",
    headerEn: "Middle Name (Arabic)",
    required: false,
    type: "text",
    descriptionAr: "اسم الأب والجد باللغة العربية",
    example: "محمد علي",
  },
  {
    id: "last_name_ar",
    headerAr: "اسم العائلة (عربي)*",
    headerEn: "Last Name (Arabic)*",
    required: true,
    type: "text",
    descriptionAr: "اللقب أو اسم العائلة باللغة العربية",
    example: "السالمي",
  },
  {
    id: "first_name",
    headerAr: "الاسم الأول (إنجليزي)",
    headerEn: "First Name (English)",
    required: false,
    type: "text",
    descriptionAr: "الاسم الأول بالإنجليزية",
    example: "Ahmed",
  },
  {
    id: "last_name",
    headerAr: "اسم العائلة (إنجليزي)",
    headerEn: "Last Name (English)",
    required: false,
    type: "text",
    descriptionAr: "اسم العائلة بالإنجليزية",
    example: "Al-Salmi",
  },
  {
    id: "national_id",
    headerAr: "الرقم الوطني / الهوية*",
    headerEn: "National ID / Iqama*",
    required: true,
    type: "text",
    descriptionAr: "رقم الهوية الوطنية أو الإقامة (يجب أن يكون فريداً)",
    example: "1098765432",
  },
  {
    id: "gender",
    headerAr: "الجنس*",
    headerEn: "Gender*",
    required: true,
    type: "enum",
    allowedValues: ["male", "female", "ذكر", "أنثى"],
    options: [
      { value: "male", label: "ذكر (Male)" },
      { value: "female", label: "أنثى (Female)" },
    ],
    example: "male",
  },
  {
    id: "birth_date",
    headerAr: "تاريخ الميلاد*",
    headerEn: "Birth Date*",
    required: true,
    type: "date",
    format: "YYYY-MM-DD",
    descriptionAr: "تاريخ ميلاد الطالب بصيغة سنة-شهر-يوم",
    example: "2015-04-12",
  },
  {
    id: "student_number",
    headerAr: "رقم القيد / رقم الطالب",
    headerEn: "Student Number",
    required: false,
    type: "text",
    descriptionAr: "اتركه فارغاً للتوليد التلقائي حسب النمط المعتمد للمؤسسة",
    example: "STU-2026-001",
  },
  {
    id: "class_name",
    headerAr: "الصف / الشعبة*",
    headerEn: "Class / Section*",
    required: true,
    type: "reference",
    referenceSource: "classes",
    descriptionAr: "اسم الصف أو رمزه المطابق لصفوف المدرسة",
    example: "الصف السابع - أ",
  },
  {
    id: "phone",
    headerAr: "رقم هاتف الطالب",
    headerEn: "Student Phone",
    required: false,
    type: "text",
    example: "771234567",
  },
  {
    id: "email",
    headerAr: "البريد الإلكتروني",
    headerEn: "Email",
    required: false,
    type: "text",
    example: "ahmed@example.com",
  },
  {
    id: "guardian_name",
    headerAr: "اسم ولي الأمر",
    headerEn: "Guardian Name",
    required: false,
    type: "text",
    descriptionAr: "الاسم الكامل لولي أمر الطالب",
    example: "محمد علي السالمي",
  },
  {
    id: "guardian_phone",
    headerAr: "هاتف ولي الأمر",
    headerEn: "Guardian Phone",
    required: false,
    type: "text",
    descriptionAr: "رقم هاتف ولي الأمر للتواصل والدخول للنظام",
    example: "777888999",
  },
  {
    id: "guardian_relationship",
    headerAr: "صلة القرابة",
    headerEn: "Relationship",
    required: false,
    type: "enum",
    allowedValues: ["father", "mother", "guardian", "أب", "أم", "ولي أمر"],
    options: [
      { value: "father", label: "أب (Father)" },
      { value: "mother", label: "أم (Mother)" },
      { value: "guardian", label: "ولي أمر (Guardian)" },
    ],
    example: "father",
  },
  {
    id: "fee_template_name",
    headerAr: "قالب الرسوم الدراسية",
    headerEn: "Fee Template",
    required: false,
    type: "reference",
    referenceSource: "feeTemplates",
    descriptionAr: "قالب الرسوم (الرسوم الإلزامية تُسند تلقائياً في جميع الأحوال)",
    example: "رسوم المرحلة الإعدادية 2026",
  },
];

export const studentGridColumns: OrbisColumnDef[] = [
  { id: "__idx", header: "#", width: 50, pin: "left", readonly: true },
  { id: "status", header: "الحالة / Status", width: 110, pin: "left", type: "badge", readonly: true },
  { id: "first_name_ar", header: "الاسم الأول (عربي)*", width: 140 },
  { id: "middle_name_ar", header: "اسم الأب", width: 130 },
  { id: "last_name_ar", header: "اسم العائلة*", width: 140 },
  { id: "national_id", header: "الرقم الوطني*", width: 130 },
  { id: "gender", header: "الجنس*", width: 90, type: "enum", options: [{ value: "male", label: "ذكر" }, { value: "female", label: "أنثى" }] },
  { id: "birth_date", header: "تاريخ الميلاد*", width: 120, type: "date" },
  { id: "class_name", header: "الصف / الشعبة*", width: 150 },
  { id: "student_number", header: "رقم الطالب", width: 120 },
  { id: "guardian_name", header: "ولي الأمر", width: 150 },
  { id: "guardian_phone", header: "هاتف ولي الأمر", width: 130 },
  { id: "guardian_relationship", header: "الصلة", width: 100 },
  { id: "phone", header: "هاتف الطالب", width: 120 },
  { id: "email", header: "البريد الإلكتروني", width: 160 },
  { id: "fee_template_name", header: "قالب الرسوم", width: 160 },
];

export const studentImportProfile: BulkImportProfile<StudentRowData, StudentPreviewDetails> = {
  id: "students",
  displayName: {
    ar: "استيراد وقيد الطلاب الجدد",
    en: "Student Admissions & Registration Import",
  },
  description: {
    ar: "إدخال وقيد سجلات الطلاب، وربط أولياء الأمور، وإسناد الصفوف، والالتزامات المالية آلياً",
    en: "Bulk register student profiles, link guardians, assign classes, and create tuition fee obligations",
  },
  templateVersion: "ORBIS-TPL-STU-2026.1",
  columnMetadata: studentColumnMetadata,
  gridColumns: studentGridColumns,

  async buildReferenceData(orgId: string): Promise<BulkReferenceData> {
    const [classesRes, feeTemplatesRes, groupsRes] = await Promise.all([
      api.listClasses().catch(() => []),
      api.listFeeTemplates().catch(() => []),
      api.listGroups().catch(() => []),
    ]);

    const classes = (classesRes || []).map((c: any) => ({
      id: c.id,
      code: c.code || "",
      name: c.name || "",
      nameAr: c.name_ar || c.name || "",
      meta: { capacity: c.capacity ?? 30 },
    }));

    const feeTemplates = (feeTemplatesRes || []).map((f: any) => ({
      id: f.id,
      name: f.name || "",
      meta: { default_amount: f.default_amount, is_mandatory: f.is_mandatory },
    }));

    const userGroups = (groupsRes || []).map((g: any) => ({
      id: g.id,
      name: g.name || "",
    }));

    return {
      classes,
      sections: [],
      levels: [],
      subjects: [],
      feeTemplates,
      discountTemplates: [],
      userGroups,
      assessmentSchemes: [],
      academicPeriods: [],
      enums: {
        genders: [
          { value: "male", labelAr: "ذكر", labelEn: "Male" },
          { value: "female", labelAr: "أنثى", labelEn: "Female" },
        ],
        studyModes: [
          { value: "regular", labelAr: "منتظم", labelEn: "Regular" },
          { value: "part_time", labelAr: "دوام جزئي", labelEn: "Part-time" },
          { value: "distance_learning", labelAr: "تعليم عن بعد", labelEn: "Distance Learning" },
        ],
        employeeTypes: [],
        employmentTypes: [],
        bloodTypes: ["A+", "A-", "B+", "B-", "AB+", "AB-", "O+", "O-"],
        relationships: [
          { value: "father", labelAr: "أب", labelEn: "Father" },
          { value: "mother", labelAr: "أم", labelEn: "Mother" },
          { value: "guardian", labelAr: "ولي أمر", labelEn: "Guardian" },
        ],
      },
    };
  },

  validateCell(colId: string, value: unknown, row: StudentRowData, refData: BulkReferenceData): BulkCellIssue | null {
    const strVal = String(value ?? "").trim();

    // Required fields check
    if (["first_name_ar", "last_name_ar", "national_id", "gender", "birth_date", "class_name"].includes(colId)) {
      if (!strVal) {
        return {
          severity: "BLOCKING_ERROR",
          columnId: colId,
          message: "هذا الحقل إلزامي ولا يمكن تركه فارغاً",
        };
      }
    }

    if (colId === "national_id" && strVal) {
      if (strVal.length < 5) {
        return {
          severity: "BLOCKING_ERROR",
          columnId: colId,
          message: "رقم الهوية غير صالح (يجب ألا يقل عن 5 خانات)",
        };
      }
    }

    if (colId === "birth_date" && strVal) {
      const parsed = Date.parse(strVal);
      if (isNaN(parsed)) {
        return {
          severity: "BLOCKING_ERROR",
          columnId: colId,
          message: "صيغة التاريخ غير صالحة، استخدم YYYY-MM-DD",
        };
      }
      const birth = new Date(strVal);
      const now = new Date();
      if (birth >= now) {
        return {
          severity: "BLOCKING_ERROR",
          columnId: colId,
          message: "تاريخ الميلاد يجب أن يكون في الماضي",
        };
      }
    }

    if (colId === "class_name" && strVal) {
      const match = refData.classes.find(
        (c) =>
          c.id.toLowerCase() === strVal.toLowerCase() ||
          c.code?.toLowerCase() === strVal.toLowerCase() ||
          c.name.toLowerCase() === strVal.toLowerCase() ||
          c.nameAr?.toLowerCase() === strVal.toLowerCase(),
      );
      if (!match) {
        return {
          severity: "BLOCKING_ERROR",
          columnId: colId,
          message: `الصف "${strVal}" غير مسجل في النظام. تحقق من قائمة الصفوف المعتمدة`,
        };
      }
    }

    if (colId === "guardian_phone" && strVal) {
      if (!row.guardian_name?.trim()) {
        return {
          severity: "WARNING",
          columnId: colId,
          message: "يُفضل كتابة اسم ولي الأمر عند إدخال رقم هاتفه",
        };
      }
    }

    return null;
  },

  validateRow(row: StudentRowData, allRows: StudentRowData[], rowIndex: number, refData: BulkReferenceData): BulkCellIssue[] {
    const issues: BulkCellIssue[] = [];

    // Check duplicate national_id across the entire batch (Level 4)
    if (row.national_id?.trim()) {
      const nid = row.national_id.trim();
      const duplicateIndex = allRows.findIndex((r, idx) => idx !== rowIndex && r.national_id?.trim() === nid);
      if (duplicateIndex !== -1) {
        issues.push({
          severity: "BLOCKING_ERROR",
          columnId: "national_id",
          message: `الرقم الوطني مكرر في الملف (يتطابق مع الصف رقم ${duplicateIndex + 1})`,
        });
      }
    }

    // Check duplicate student_number if specified
    if (row.student_number?.trim()) {
      const sn = row.student_number.trim();
      const duplicateIndex = allRows.findIndex((r, idx) => idx !== rowIndex && r.student_number?.trim() === sn);
      if (duplicateIndex !== -1) {
        issues.push({
          severity: "BLOCKING_ERROR",
          columnId: "student_number",
          message: `رقم الطالب مكرر في الملف (يتطابق مع الصف رقم ${duplicateIndex + 1})`,
        });
      }
    }

    return issues;
  },

  async generatePreview(rows: StudentRowData[], refData: BulkReferenceData): Promise<BulkPreviewSummary<StudentPreviewDetails>> {
    let validCount = 0;
    let errorCount = 0;
    let warningCount = 0;
    const classPlacements: Record<string, number> = {};
    const parentPhones = new Set<string>();

    const mandatoryTemplates = refData.feeTemplates.filter((t) => (t.meta as any)?.is_mandatory);
    const mandatoryTotalPerStudent = mandatoryTemplates.reduce(
      (sum, t) => sum + (Number((t.meta as any)?.default_amount) || 0),
      0,
    );

    for (const row of rows) {
      const hasErrors = !row.first_name_ar || !row.last_name_ar || !row.national_id || !row.class_name;
      if (hasErrors) {
        errorCount++;
      } else {
        validCount++;
        const cls = row.class_name || "Unassigned";
        classPlacements[cls] = (classPlacements[cls] || 0) + 1;
        if (row.guardian_phone?.trim()) {
          parentPhones.add(row.guardian_phone.trim());
        }
      }
    }

    const details: StudentPreviewDetails = {
      studentsToCreate: validCount,
      studentsToUpdate: 0,
      personsToCreate: validCount,
      personsToLink: 0,
      guardiansToLink: parentPhones.size,
      newParentAccounts: parentPhones.size,
      mandatoryFeesAssignedCount: validCount * mandatoryTemplates.length,
      totalEstimatedMandatoryFees: validCount * mandatoryTotalPerStudent,
      currency: orgContext.baseCurrency || "YER",
      capacityOverridesRequired: [],
      classPlacementsSummary: classPlacements,
    };

    return {
      totalRows: rows.length,
      validRowsCount: validCount,
      errorRowsCount: errorCount,
      warningRowsCount: warningCount,
      details,
    };
  },

  async commitChunk(rows: StudentRowData[], orgId: string, refData: BulkReferenceData): Promise<{ success: boolean; errors?: string[] }> {
    const items: BulkRegisterStudentCommandItem[] = [];

    for (let i = 0; i < rows.length; i++) {
      const row = rows[i]!;
      const matchedClass = refData.classes.find(
        (c) =>
          c.id.toLowerCase() === row.class_name.toLowerCase() ||
          c.code?.toLowerCase() === row.class_name.toLowerCase() ||
          c.name.toLowerCase() === row.class_name.toLowerCase() ||
          c.nameAr?.toLowerCase() === row.class_name.toLowerCase(),
      );

      const firstNameAr = row.first_name_ar?.trim() || "طالب";
      const lastNameAr = row.last_name_ar?.trim() || "جديد";
      const middleNameAr = row.middle_name_ar?.trim();

      const firstNameEn = row.first_name?.trim() || firstNameAr;
      const lastNameEn = row.last_name?.trim() || lastNameAr;
      const middleNameEn = row.middle_name?.trim();

      let genderStr = "male";
      const rawGender = String(row.gender || "").toLowerCase().trim();
      if (rawGender === "female" || rawGender === "أنثى") genderStr = "female";

      items.push({
        row_index: row.rowNumber || i + 1,
        registration: {
          student_number: row.student_number?.trim() || "",
          first_name: firstNameEn,
          middle_name: middleNameEn,
          last_name: lastNameEn,
          first_name_ar: firstNameAr,
          middle_name_ar: middleNameAr,
          last_name_ar: lastNameAr,
          display_name: `${firstNameAr} ${lastNameAr}`,
          national_id: row.national_id?.trim() || undefined,
          passport_number: row.passport_number?.trim() || undefined,
          gender: genderStr,
          birth_date: row.birth_date?.trim() || undefined,
          phone: row.phone?.trim() || undefined,
          email: row.email?.trim() || undefined,
          initial_period_id: orgId,
          initial_class_id: matchedClass?.id || undefined,
          study_mode: (row.study_mode as any) || "regular",
        },
        guardian_name: row.guardian_name?.trim() || undefined,
        guardian_phone: row.guardian_phone?.trim() || undefined,
        guardian_relationship: row.guardian_relationship?.trim() || undefined,
      });
    }

    try {
      const bulkRes = await api.bulkRegisterStudents(items);
      const errors: string[] = [];

      for (const it of bulkRes.items) {
        if (!it.success && it.error) {
          errors.push(`صف ${it.row_index}: ${it.error}`);
        }
      }

      return {
        success: bulkRes.failed === 0,
        errors: errors.length > 0 ? errors : undefined,
      };
    } catch (err: any) {
      return {
        success: false,
        errors: [err.message || String(err)],
      };
    }
  },
};
