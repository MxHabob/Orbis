import type { OrbisColumnDef } from "@orbis/svelte-grid";
import { api, type BulkRegisterStudentCommandItem } from "$lib/api";
import { orgContext } from "$lib/stores/org-context.svelte";
import { effectiveContext } from "$lib/stores/effective-context.svelte";
import { parseRegistrationFieldPolicies, normalizeRegistrationFieldKey } from "$lib/features/students/registration-fields";
import { splitPersonName } from "$lib/shared/utils/person-name";
import { createBulkValidationContext, findDuplicateIndex, type BulkValidationContext } from "../engine/duplicate-index";
import {
  ensureStudentRegistrationArtifacts,
  provisionStudentLoginAccount,
} from "$lib/features/students/registration-effects";
import * as m from "$lib/paraglide/messages";
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
  full_name_ar: string;
  full_name_en?: string;
  national_id?: string;
  passport_number?: string;
  gender: "male" | "female";
  birth_date: string;
  place_of_birth?: string;
  nationality?: string;
  blood_type?: string;
  medical_notes?: string;
  special_needs_notes?: string;
  address?: string;
  phone?: string;
  email?: string;
  student_number?: string;
  class_name: string;
  study_mode?: string;
  guardian_name?: string;
  guardian_phone?: string;
  guardian_relationship?: string;
  guardian_national_id?: string;
  create_login_account?: boolean | string;
  user_group_names?: string;
  [key: string]: unknown;
}

export const studentColumnMetadata: ColumnDefinitionMetadata[] = [
  {
    id: "full_name_ar",
    headerAr: "الاسم الكامل (عربي)*",
    headerEn: "Full Name (Arabic)*",
    required: true,
    type: "text",
    descriptionAr: "الاسم الكامل للطالب باللغة العربية، كما سيظهر في سجله",
    example: "أحمد محمد علي السالمي",
  },
  {
    id: "full_name_en",
    headerAr: "الاسم الكامل (إنجليزي)",
    headerEn: "Full Name (English)",
    required: false,
    type: "text",
    descriptionAr: "الاسم الكامل بالإنجليزية كما في الوثائق الرسمية",
    example: "Ahmed Mohammed Ali Al-Salmi",
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
      { value: "male", label: m.bulk_entry_enum_gender_male() },
      { value: "female", label: m.bulk_entry_enum_gender_female() },
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
    id: "place_of_birth",
    headerAr: "مكان الميلاد",
    headerEn: "Place of Birth",
    required: false,
    type: "text",
    example: "صنعاء",
  },
  {
    id: "nationality",
    headerAr: "الجنسية",
    headerEn: "Nationality",
    required: false,
    type: "text",
    example: "يمني",
  },
  {
    id: "passport_number",
    headerAr: "رقم جواز السفر",
    headerEn: "Passport Number",
    required: false,
    type: "text",
    example: "P12345678",
  },
  {
    id: "blood_type",
    headerAr: "فصيلة الدم",
    headerEn: "Blood Type",
    required: false,
    type: "enum",
    allowedValues: ["A+", "A-", "B+", "B-", "AB+", "AB-", "O+", "O-"],
    example: "O+",
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
    headerAliases: ["المستوى الدراسي / الشعبة"],
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
    id: "address",
    headerAr: "العنوان",
    headerEn: "Address",
    required: false,
    type: "text",
    example: "صنعاء، اليمن",
  },
  {
    id: "study_mode",
    headerAr: "نمط الدراسة",
    headerEn: "Study Mode",
    required: false,
    type: "enum",
    allowedValues: ["regular", "part_time", "distance_learning", "homeschooled", "auditor", "visiting", "exchange", "special_needs"],
    example: "regular",
  },
  {
    id: "medical_notes",
    headerAr: "ملاحظات طبية",
    headerEn: "Medical Notes",
    required: false,
    type: "text",
    example: "",
  },
  {
    id: "special_needs_notes",
    headerAr: "الاحتياجات الخاصة",
    headerEn: "Special Needs",
    required: false,
    type: "text",
    example: "",
  },
  {
    id: "create_login_account",
    headerAr: "إنشاء حساب دخول للنظام",
    headerEn: "Create Login Account",
    required: false,
    type: "boolean",
    example: "لا",
  },
  {
    id: "user_group_names",
    headerAr: "مجموعات المستخدمين",
    headerEn: "User Groups",
    required: false,
    type: "reference",
    referenceSource: "userGroups",
    descriptionAr: "أدخل رمز المجموعة أو اسمها، وافصل بين المجموعات بالرمز |. يتطلب تفعيل إنشاء حساب الدخول.",
    descriptionEn: "Enter group codes or names separated by |. Requires login account creation.",
    example: "",
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
      { value: "father", label: m.bulk_entry_enum_relationship_father() },
      { value: "mother", label: m.bulk_entry_enum_relationship_mother() },
      { value: "guardian", label: m.bulk_entry_enum_relationship_guardian() },
    ],
    example: "father",
  },
];

export const studentGridColumns: OrbisColumnDef[] = [
  { id: "__idx", header: m.bulk_entry_grid_row_number(), width: 50, pin: "left", readonly: true },
  { id: "status", header: m.bulk_entry_grid_status(), width: 110, pin: "left", type: "badge", readonly: true },
  { id: "full_name_ar", header: "الاسم الكامل (عربي)*", width: 210 },
  { id: "full_name_en", header: "الاسم الكامل (إنجليزي)", width: 210 },
  { id: "national_id", header: m.bulk_entry_student_header_national_id(), width: 130 },
  { id: "gender", header: m.bulk_entry_header_gender(), width: 90, type: "enum", options: [{ value: "male", label: m.bulk_entry_enum_gender_male() }, { value: "female", label: m.bulk_entry_enum_gender_female() }] },
  { id: "birth_date", header: m.bulk_entry_student_header_birth_date(), width: 120, type: "date" },
  { id: "place_of_birth", header: "مكان الميلاد", width: 130 },
  { id: "nationality", header: "الجنسية", width: 110 },
  { id: "passport_number", header: "رقم جواز السفر", width: 130 },
  { id: "blood_type", header: "فصيلة الدم", width: 90, type: "enum", options: ["A+", "A-", "B+", "B-", "AB+", "AB-", "O+", "O-"].map((value) => ({ value, label: value })) },
  { id: "class_name", header: m.bulk_entry_student_header_class_name(), width: 150 },
  { id: "student_number", header: m.bulk_entry_student_header_student_number(), width: 120 },
  { id: "study_mode", header: "نمط الدراسة", width: 130, type: "enum", options: [
    { value: "regular", label: m.operations_ui_study_mode_regular() },
    { value: "part_time", label: m.operations_ui_study_mode_part_time() },
    { value: "distance_learning", label: m.operations_ui_study_mode_distance() },
    { value: "homeschooled", label: m.operations_ui_study_mode_homeschooled() },
    { value: "auditor", label: m.operations_ui_study_mode_auditor() },
    { value: "visiting", label: m.operations_ui_study_mode_visiting() },
    { value: "exchange", label: m.operations_ui_study_mode_exchange() },
    { value: "special_needs", label: m.operations_ui_study_mode_special_needs() },
  ] },
  { id: "guardian_name", header: m.bulk_entry_student_header_guardian_name(), width: 150 },
  { id: "guardian_phone", header: m.bulk_entry_student_header_guardian_phone(), width: 130 },
  { id: "guardian_relationship", header: m.bulk_entry_student_header_guardian_relationship(), width: 100 },
  { id: "phone", header: m.bulk_entry_header_phone(), width: 120 },
  { id: "email", header: m.bulk_entry_header_email(), width: 160 },
  { id: "address", header: "العنوان", width: 180 },
  { id: "medical_notes", header: "ملاحظات طبية", width: 180 },
  { id: "special_needs_notes", header: "الاحتياجات الخاصة", width: 180 },
  { id: "create_login_account", header: "إنشاء حساب دخول للنظام", width: 170, type: "boolean" },
  { id: "user_group_names", header: "مجموعات المستخدمين", width: 180 },
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
  templateVersion: "ORBIS-TPL-STU-2026.4",
  columnMetadata: studentColumnMetadata,
  gridColumns: studentGridColumns,

  async buildReferenceData(_orgId: string): Promise<BulkReferenceData> {
    const [classesRes, feeTemplatesRes, groupsRes, org] = await Promise.all([
      api.listClasses(),
      api.listFeeTemplates(),
      api.listGroups(),
      api.getCurrentOrg(),
    ]);

    const classes = (classesRes || []).map((c: any) => ({
      id: c.id,
      code: c.code || c.id,
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
      code: g.id,
      name: g.name || "",
    }));

    return {
      classes,
      teachers: [],
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
          { value: "homeschooled", labelAr: "تعليم منزلي", labelEn: "Homeschooled" },
          { value: "auditor", labelAr: "مستمع", labelEn: "Auditor" },
          { value: "visiting", labelAr: "زائر", labelEn: "Visiting" },
          { value: "exchange", labelAr: "تبادل", labelEn: "Exchange" },
          { value: "special_needs", labelAr: "احتياجات خاصة", labelEn: "Special Needs" },
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
      registrationFields: parseRegistrationFieldPolicies(org?.settings),
      existingClassSubjectAssignments: [],
    };
  },

  validateCell(colId: string, value: unknown, row: StudentRowData, refData: BulkReferenceData): BulkCellIssue | null {
    const strVal = String(value ?? "").trim();
    const policy = refData.registrationFields?.[normalizeRegistrationFieldKey(colId)];
    if (
      colId !== "full_name_ar" &&
      colId !== "class_name" &&
      policy?.active === false
    ) {
      return null;
    }

    // Required fields check
    const required = colId === "full_name_ar" || colId === "class_name" ||
      (policy?.required ?? ["national_id", "gender", "birth_date"].includes(colId));
    if (required) {
      if (!strVal) {
        return {
          severity: "BLOCKING_ERROR",
          columnId: colId,
          message: m.bulk_entry_validation_required(),
        };
      }
    }

    if (colId === "national_id" && strVal) {
      if (strVal.length < 5) {
        return {
          severity: "BLOCKING_ERROR",
          columnId: colId,
          message: m.bulk_entry_student_validation_national_id_length(),
        };
      }
    }

    if (colId === "birth_date" && strVal) {
      const parsed = Date.parse(strVal);
      if (isNaN(parsed)) {
        return {
          severity: "BLOCKING_ERROR",
          columnId: colId,
          message: m.bulk_entry_validation_date_format(),
        };
      }
      const birth = new Date(strVal);
      const now = new Date();
      if (birth >= now) {
        return {
          severity: "BLOCKING_ERROR",
          columnId: colId,
          message: m.bulk_entry_student_validation_birth_date_past(),
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
          message: m.bulk_entry_student_validation_class_not_found({ name: strVal }),
        };
      }
    }

    if (colId === "user_group_names" && strVal) {
      if (
        !(
          row.create_login_account === true ||
          ["true", "1", "نعم", "yes"].includes(
            String(row.create_login_account ?? "").trim().toLowerCase(),
          )
        )
      ) {
        return {
          severity: "BLOCKING_ERROR",
          columnId: colId,
          message: m.bulk_entry_group_requires_login(),
        };
      }
      const tokens = strVal.split(/[|;]/).map((token) => token.trim()).filter(Boolean);
      const unmatched = tokens.find((token) =>
        !resolveReferenceItem(refData.userGroups, token),
      );
      if (unmatched) {
        return {
          severity: "BLOCKING_ERROR",
          columnId: colId,
          message: m.bulk_entry_group_not_found({ group: unmatched }),
        };
      }
    }

    if (colId === "guardian_phone" && strVal) {
      if (!row.guardian_name?.trim()) {
        return {
          severity: "WARNING",
          columnId: colId,
          message: m.bulk_entry_student_validation_guardian_name(),
        };
      }
    }

    return null;
  },

  validateRow(
    row: StudentRowData,
    allRows: StudentRowData[],
    rowIndex: number,
    refData: BulkReferenceData,
    context?: BulkValidationContext,
  ): BulkCellIssue[] {
    const issues: BulkCellIssue[] = [];

    // Check duplicate national_id across the entire batch (Level 4)
    if (
      row.national_id?.trim() &&
      refData.registrationFields?.national_id?.active !== false &&
      refData.registrationFields?.national_id?.preventDuplicate !== false
    ) {
      const nid = row.national_id.trim();
      const duplicateIndex = context
        ? findDuplicateIndex(context, "national_id", nid, rowIndex) ?? -1
        : allRows.findIndex((r, idx) => idx !== rowIndex && r.national_id?.trim() === nid);
      if (duplicateIndex !== -1) {
        issues.push({
          severity: "BLOCKING_ERROR",
          columnId: "national_id",
          message: m.bulk_entry_validation_duplicate_national_id({ row: duplicateIndex + 1 }),
        });
      }
    }

    // Check duplicate student_number if specified
    if (row.student_number?.trim()) {
      const sn = row.student_number.trim();
      const duplicateIndex = context
        ? findDuplicateIndex(context, "student_number", sn, rowIndex) ?? -1
        : allRows.findIndex((r, idx) => idx !== rowIndex && r.student_number?.trim() === sn);
      if (duplicateIndex !== -1) {
        issues.push({
          severity: "BLOCKING_ERROR",
          columnId: "student_number",
          message: m.bulk_entry_student_validation_duplicate_student_number({ row: duplicateIndex + 1 }),
        });
      }
    }

    return issues;
  },

  async generatePreview(rows: StudentRowData[], refData: BulkReferenceData): Promise<BulkPreviewSummary<StudentPreviewDetails>> {
    let validCount = 0;
    let errorCount = 0;
    let warningCount = 0;
    let userAccountsToProvision = 0;
    const classPlacements: Record<string, number> = {};
    const parentPhones = new Set<string>();
    const validationContext = createBulkValidationContext(
      "students",
      rows as Record<string, unknown>[],
    );

    const mandatoryTemplates = refData.feeTemplates.filter((t) => (t.meta as any)?.is_mandatory);
    const mandatoryTotalPerStudent = mandatoryTemplates.reduce(
      (sum, t) => sum + (Number((t.meta as any)?.default_amount) || 0),
      0,
    );

    for (const [rowIndex, row] of rows.entries()) {
      const enabledColumns = studentColumnMetadata.filter((column) =>
        column.id === "full_name_ar" || column.id === "class_name" ||
        refData.registrationFields?.[normalizeRegistrationFieldKey(column.id)]?.active !== false,
      );
      const hasErrors = enabledColumns.some((column) =>
        this.validateCell(column.id, row[column.id], row, refData)?.severity === "BLOCKING_ERROR",
      ) || this.validateRow(row, rows, rowIndex, refData, validationContext).some(
        (issue) => issue.severity === "BLOCKING_ERROR",
      );
      if (hasErrors) {
        errorCount++;
      } else {
        validCount++;
        const cls = row.class_name || "Unassigned";
        classPlacements[cls] = (classPlacements[cls] || 0) + 1;
        if (row.guardian_phone?.trim()) {
          parentPhones.add(row.guardian_phone.trim());
        }
        if (
          row.create_login_account === true ||
          ["true", "1", "نعم", "yes"].includes(
            String(row.create_login_account ?? "").trim().toLowerCase(),
          )
        ) {
          userAccountsToProvision++;
        }
      }
    }

    const details: StudentPreviewDetails = {
      studentsToCreate: validCount,
      studentsToUpdate: 0,
      personsToCreate: validCount,
      personsToLink: 0,
      userAccountsToProvision,
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

  async commitChunk(rows: StudentRowData[], _orgId: string, refData: BulkReferenceData) {
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

      const fullNameAr = row.full_name_ar.trim();
      const namePartsAr = splitPersonName(fullNameAr);
      const fullNameEn = row.full_name_en?.trim() || fullNameAr;
      const namePartsEn = splitPersonName(fullNameEn);

      const rawGender = String(row.gender || "").toLowerCase().trim();
      const genderStr = rawGender === "female" || rawGender === "أنثى" ? "female" : rawGender === "male" || rawGender === "ذكر" ? "male" : undefined;
      const fieldActive = (id: string) =>
        refData.registrationFields?.[normalizeRegistrationFieldKey(id)]?.active !== false;

      items.push({
        row_index: row.rowNumber || i + 1,
        registration: {
          student_number: row.student_number?.trim() || "",
          first_name: namePartsEn.firstName,
          middle_name: namePartsEn.middleName,
          last_name: namePartsEn.lastName,
          first_name_ar: namePartsAr.firstName,
          middle_name_ar: namePartsAr.middleName,
          last_name_ar: namePartsAr.lastName,
          display_name: fullNameAr,
          national_id: fieldActive("national_id") ? row.national_id?.trim() || undefined : undefined,
          passport_number: fieldActive("passport_number") ? row.passport_number?.trim() || undefined : undefined,
          gender: fieldActive("gender") ? genderStr : undefined,
          birth_date: fieldActive("birth_date") ? row.birth_date?.trim() || undefined : undefined,
          place_of_birth: fieldActive("place_of_birth") ? row.place_of_birth?.trim() || undefined : undefined,
          nationality: fieldActive("nationality") ? row.nationality?.trim() || undefined : undefined,
          blood_type: fieldActive("blood_type") ? row.blood_type?.trim() || undefined : undefined,
          medical_notes: fieldActive("medical_notes") ? row.medical_notes?.trim() || undefined : undefined,
          special_needs_notes: fieldActive("special_needs_notes") ? row.special_needs_notes?.trim() || undefined : undefined,
          address: fieldActive("address") ? row.address?.trim() || undefined : undefined,
          phone: fieldActive("phone") ? row.phone?.trim() || undefined : undefined,
          email: fieldActive("email") ? row.email?.trim() || undefined : undefined,
          initial_period_id: effectiveContext.currentPeriodId || undefined,
          initial_class_id: matchedClass?.id || undefined,
          study_mode: row.study_mode?.trim() || "regular",
        },
        guardian_name: row.guardian_name?.trim() || undefined,
        guardian_phone: row.guardian_phone?.trim() || undefined,
        guardian_relationship: row.guardian_relationship?.trim() || undefined,
      });
    }

    try {
      const bulkRes = await api.bulkRegisterStudents(items, { atomic: false });
      const rowErrors: Array<{ rowNumber: number; reason: string }> = [];
      const warnings: Array<{ rowNumber: number; reason: string }> = [];

      for (const item of bulkRes.items) {
        if (!item.success && item.error) {
          rowErrors.push({
            rowNumber: item.row_index,
            reason: m.bulk_entry_row_error({ row: item.row_index, reason: item.error }),
          });
        }
      }

      const succeededItems = bulkRes.items.filter((item) => item.success && item.student_id);
      let accounts: Awaited<ReturnType<typeof api.listAccounts>> | undefined;
      let accountListError: string | undefined;
      if (succeededItems.length > 0) {
        try {
          accounts = await api.listAccounts();
        } catch (error) {
          accountListError = error instanceof Error ? error.message : String(error);
        }
      }
      for (let offset = 0; offset < succeededItems.length; offset += 10) {
        const batch = succeededItems.slice(offset, offset + 10);
        const artifactResults = await Promise.all(
          batch.map(async (item) => {
            const row = rows.find((candidate) => candidate.rowNumber === item.row_index);
            const studentId = item.student_id;
            if (!studentId) return { rowNumber: item.row_index, errors: [] };
            const name = row?.full_name_ar?.trim() || row?.full_name_en?.trim() || item.student_number || "";
            const artifacts = await ensureStudentRegistrationArtifacts(studentId, name, {
              accounts,
              accountListError,
            });
            const errors = [...artifacts.warnings];
            if (artifacts.folderError) {
              errors.push(
                m.students_archive_folder_failed({ error: artifacts.folderError }),
              );
            }
            if (
              row?.create_login_account === true ||
              ["true", "1", "نعم", "yes"].includes(
                String(row?.create_login_account ?? "").trim().toLowerCase(),
              )
            ) {
              try {
                await provisionStudentLoginAccount({
                  studentId,
                  fullNameAr: row?.full_name_ar?.trim() || name,
                  fullNameEn: row?.full_name_en?.trim(),
                  email: row?.email,
                  phone: row?.phone,
                  groupIds: resolveGroupIds(row?.user_group_names, refData),
                  preferredUsername: row?.email || row?.phone || row?.student_number || undefined,
                });
              } catch (error) {
                errors.push(error instanceof Error ? error.message : String(error));
              }
            }
            return { rowNumber: item.row_index, errors };
          }),
        );
        warnings.push(
          ...artifactResults.flatMap(({ rowNumber, errors: artifactErrors }) =>
            artifactErrors.map((reason) => ({ rowNumber, reason })),
          ),
        );
      }

      return {
        success: true,
        rowErrors: rowErrors.length > 0 ? rowErrors : undefined,
        warnings: warnings.length > 0 ? warnings : undefined,
      };
    } catch (err: any) {
      return {
        success: false,
        errors: [err.message || String(err)],
      };
    }
  },
};

function resolveReferenceItem(
  items: BulkReferenceData["userGroups"],
  value: string,
) {
  const needle = value.trim().toLocaleLowerCase();
  const matches = items.filter((item) =>
    [item.id, item.code, item.name, item.nameAr]
      .filter((candidate): candidate is string => Boolean(candidate))
      .some((candidate) => candidate.trim().toLocaleLowerCase() === needle),
  );
  return matches.length === 1 ? matches[0] : undefined;
}

function resolveGroupIds(value: string | undefined, refData: BulkReferenceData) {
  if (!value) return [];
  return [...new Set(
    value
      .split(/[|;]/)
      .map((token) => resolveReferenceItem(refData.userGroups, token)?.id)
      .filter((id): id is string => Boolean(id)),
  )];
}
