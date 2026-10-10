import type { OrbisColumnDef } from "@orbis/svelte-grid";
import { api } from "$lib/api";
import { getLocale } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";
import type {
  BulkCellIssue,
  BulkImportProfile,
  BulkPreviewSummary,
  BulkReferenceData,
  ColumnDefinitionMetadata,
  ReferenceItem,
} from "../types";

export interface TeacherSubjectRowData {
  rowNumber?: number;
  class_code: string;
  academic_period_code: string;
  subject_code: string;
  teacher_code: string;
  weekly_hours?: number | string;
  [key: string]: unknown;
}

const requiredReferenceFields = [
  "class_code",
  "academic_period_code",
  "subject_code",
  "teacher_code",
] as const;

export const teacherSubjectColumnMetadata: ColumnDefinitionMetadata[] = [
  {
    id: "class_code",
    headerAr: "رمز الصف أو معرفه*",
    headerEn: "Class Code or ID*",
    required: true,
    type: "reference",
    referenceSource: "classes",
    descriptionAr: "استخدم الرمز/المعرف المطابق في ورقة البيانات المرجعية.",
    descriptionEn: "Use the matching code or ID from the Reference Data sheet.",
    example: "",
  },
  {
    id: "academic_period_code",
    headerAr: "رمز الفترة الأكاديمية أو معرفها*",
    headerEn: "Academic Period Code or ID*",
    required: true,
    type: "reference",
    referenceSource: "academicPeriods",
    descriptionAr: "يجب أن تكون الفترة هي الفترة المرتبطة بالصف.",
    descriptionEn: "The period must be the one associated with the selected class.",
    example: "",
  },
  {
    id: "subject_code",
    headerAr: "رمز المادة أو معرفها*",
    headerEn: "Subject Code or ID*",
    required: true,
    type: "reference",
    referenceSource: "subjects",
    descriptionAr: "استخدم رمز المادة أو معرفها من ورقة البيانات المرجعية.",
    descriptionEn: "Use the subject code or ID from the Reference Data sheet.",
    example: "",
  },
  {
    id: "teacher_code",
    headerAr: "الرقم الوظيفي للمعلم أو معرفه*",
    headerEn: "Teacher Employee Number or ID*",
    required: true,
    type: "reference",
    referenceSource: "teachers",
    descriptionAr: "يجب أن يكون المعلم موظفاً نشطاً مصنفاً كمعلم في النظام.",
    descriptionEn: "The teacher must be an active employee classified as a teacher.",
    example: "",
  },
  {
    id: "weekly_hours",
    headerAr: "الحصص الأسبوعية",
    headerEn: "Weekly Hours",
    required: false,
    type: "integer",
    descriptionAr: "أدخل عدداً صحيحاً موجباً، أو اتركه فارغاً إذا لم يُحدد.",
    descriptionEn: "Enter a positive whole number, or leave blank if unspecified.",
    example: "",
  },
];

export const teacherSubjectGridColumns: OrbisColumnDef[] = [
  { id: "__idx", header: m.bulk_entry_grid_row_number(), width: 50, pin: "left", readonly: true },
  { id: "status", header: m.bulk_entry_grid_status(), width: 110, pin: "left", type: "badge", readonly: true },
  { id: "class_code", header: teacherSubjectColumnMetadata[0]!.headerAr, width: 210 },
  { id: "academic_period_code", header: teacherSubjectColumnMetadata[1]!.headerAr, width: 230 },
  { id: "subject_code", header: teacherSubjectColumnMetadata[2]!.headerAr, width: 210 },
  { id: "teacher_code", header: teacherSubjectColumnMetadata[3]!.headerAr, width: 240 },
  { id: "weekly_hours", header: teacherSubjectColumnMetadata[4]!.headerAr, width: 130, type: "integer" },
];

export const teacherSubjectImportProfile: BulkImportProfile<
  TeacherSubjectRowData,
  { assignmentsToCreate: number }
> = {
  id: "teacher-subjects",
  displayName: {
    ar: "إسناد المعلمين والمواد",
    en: "Teacher & Subject Assignments",
  },
  description: {
    ar: "إسناد المواد للمعلمين ضمن الصفوف والفترات الأكاديمية، مع منع التكرار والتحقق من تطابق الفترة",
    en: "Assign teachers to subjects in classes and academic periods with duplicate and period-mismatch checks",
  },
  templateVersion: "ORBIS-TPL-TEACHER-SUBJECT-2026.1",
  columnMetadata: teacherSubjectColumnMetadata,
  gridColumns: teacherSubjectGridColumns,

  async buildReferenceData(): Promise<BulkReferenceData> {
    const [classesResponse, subjectsResponse, periodsResponse, firstEmployeePage] =
      await Promise.all([
        api.listClasses(),
        api.listSubjects(),
        api.listAcademicPeriods(),
        api.listEmployees({ status: "active", page: 1, page_size: 100 }),
      ]);

    const employeePages = await Promise.all(
      Array.from(
        { length: Math.max(0, firstEmployeePage.total_pages - 1) },
        (_, index) =>
          api.listEmployees({
            status: "active",
            page: index + 2,
            page_size: firstEmployeePage.page_size || 100,
          }),
      ),
    );
    const activeEmployees = [
      ...firstEmployeePage.items,
      ...employeePages.flatMap((page) => page.items),
    ];
    const teachers = activeEmployees.filter(
      (employee) =>
        employee.status === "active" &&
        (employee.employee_type.toLowerCase() === "teacher" ||
          employee.roles.some((role) => role.toLowerCase() === "teacher")),
    );

    const classes = classesResponse
      .filter((item) => item.is_active)
      .map((item) => {
        const period = periodsResponse.find((candidate) => candidate.id === item.period_id);
        const periodName =
          getLocale() === "ar"
            ? period?.name_ar || period?.name || ""
            : period?.name || period?.name_ar || "";
        const details = [
          item.level,
          item.division,
          periodName,
        ].filter(Boolean).join(" · ");
        return {
          id: item.id,
          code: item.id,
          name: item.name,
          nameAr: item.name_ar || item.name,
          meta: {
            periodId: item.period_id,
            level: item.level,
            division: item.division || "",
            details,
          },
        };
      });
    const subjects = subjectsResponse
      .filter((item) => item.is_active)
      .map((item) => ({
        id: item.id,
        code: item.code || item.id,
        name: item.name,
        nameAr: item.name_ar || item.name,
      }));
    const academicPeriods = periodsResponse.map((item) => ({
      id: item.id,
      code: item.code || item.id,
      name: item.name,
      nameAr: item.name_ar || item.name,
    }));
    const teacherReferences = teachers.map((item) => ({
      id: item.id,
      code: item.employee_number || item.id,
      name: item.full_name || item.employee_number,
      nameAr: item.full_name_ar || item.full_name || item.employee_number,
      meta: { employeeNumber: item.employee_number },
    }));

    const classPeriods = [
      ...new Map(
        classes.map((item) => [
          `${item.id}:${String(item.meta?.periodId)}`,
          { classId: item.id, periodId: String(item.meta?.periodId) },
        ]),
      ).values(),
    ];
    const existingClassSubjectAssignments: BulkReferenceData["existingClassSubjectAssignments"] =
      [];
    for (let offset = 0; offset < classPeriods.length; offset += 10) {
      const batch = classPeriods.slice(offset, offset + 10);
      const assignments = await Promise.all(
        batch.map(({ classId, periodId }) =>
          api.listClassSubjects(classId, periodId),
        ),
      );
      for (const assignment of assignments.flat()) {
        if (assignment.is_active) {
          existingClassSubjectAssignments.push({
            classId: assignment.class_id,
            subjectId: assignment.subject_id,
            periodId: assignment.period_id,
            teacherId: assignment.primary_teacher_id,
          });
        }
      }
    }

    return {
      classes,
      teachers: teacherReferences,
      sections: [],
      levels: [],
      subjects,
      feeTemplates: [],
      discountTemplates: [],
      userGroups: [],
      assessmentSchemes: [],
      academicPeriods,
      enums: {
        genders: [],
        studyModes: [],
        employeeTypes: [],
        employmentTypes: [],
        bloodTypes: [],
        relationships: [],
      },
      existingClassSubjectAssignments,
    };
  },

  validateCell(
    colId: string,
    value: unknown,
    _row: TeacherSubjectRowData,
    refData: BulkReferenceData,
  ): BulkCellIssue | null {
    const text = String(value ?? "").trim();
    if (requiredReferenceFields.includes(colId as (typeof requiredReferenceFields)[number]) && !text) {
      return issue(colId, message("هذا الحقل مطلوب.", "This field is required."));
    }

    const refSource = referenceSourceForField(colId);
    if (text && refSource && !resolveUnique(refData[refSource], text)) {
      return issue(
        colId,
        message(
          "القيمة غير موجودة أو غير فريدة في البيانات المرجعية.",
          "This value is missing or ambiguous in the Reference Data sheet.",
        ),
      );
    }

    if (colId === "weekly_hours" && text) {
      const hours = Number(text);
      if (!Number.isInteger(hours) || hours < 1) {
        return issue(
          colId,
          message("أدخل عدداً صحيحاً موجباً للحصص الأسبوعية.", "Enter a positive whole number for weekly hours."),
        );
      }
    }
    return null;
  },

  validateRow(
    row: TeacherSubjectRowData,
    allRows: TeacherSubjectRowData[],
    rowIndex: number,
    refData: BulkReferenceData,
  ): BulkCellIssue[] {
    const issues: BulkCellIssue[] = [];
    const classItem = resolveUnique(refData.classes, row.class_code);
    const periodItem = resolveUnique(refData.academicPeriods, row.academic_period_code);
    const subjectItem = resolveUnique(refData.subjects, row.subject_code);

    if (classItem && periodItem && classItem.meta?.periodId !== periodItem.id) {
      issues.push(
        issue(
          "academic_period_code",
          message(
            "الفترة الأكاديمية المحددة لا تطابق الفترة المرتبطة بالصف.",
            "The selected academic period does not match the class period.",
          ),
        ),
      );
    }

    if (classItem && periodItem && subjectItem) {
      const key = assignmentKey(classItem.id, subjectItem.id, periodItem.id);
      const duplicateRow = allRows.findIndex((other, index) => {
        if (index === rowIndex) return false;
        const otherClass = resolveUnique(refData.classes, other.class_code);
        const otherPeriod = resolveUnique(refData.academicPeriods, other.academic_period_code);
        const otherSubject = resolveUnique(refData.subjects, other.subject_code);
        return (
          otherClass &&
          otherPeriod &&
          otherSubject &&
          assignmentKey(otherClass.id, otherSubject.id, otherPeriod.id) === key
        );
      });
      if (duplicateRow !== -1) {
        issues.push(
          issue(
            "subject_code",
            message(
              `هذا الإسناد مكرر في الصف ${duplicateRow + 1} من الملف.`,
              `This assignment is duplicated in file row ${allRows[duplicateRow]?.rowNumber ?? duplicateRow + 1}.`,
            ),
          ),
        );
      } else if (
        refData.existingClassSubjectAssignments.some(
          (item) => assignmentKey(item.classId, item.subjectId, item.periodId) === key,
        )
      ) {
        issues.push(
          issue(
            "subject_code",
            message(
              "المادة مسندة مسبقاً لهذا الصف والفترة. لا يستبدل هذا القالب إسناداً قائماً.",
              "This subject is already assigned to this class and period. Existing assignments are not overwritten.",
            ),
          ),
        );
      }
    }

    return issues;
  },

  async generatePreview(
    rows: TeacherSubjectRowData[],
    refData: BulkReferenceData,
  ): Promise<BulkPreviewSummary<{ assignmentsToCreate: number }>> {
    let validRowsCount = 0;
    let errorRowsCount = 0;
    for (let index = 0; index < rows.length; index++) {
      const row = rows[index]!;
      const invalidCell = teacherSubjectColumnMetadata.some(
        (column) =>
          this.validateCell(column.id, row[column.id], row, refData)?.severity ===
          "BLOCKING_ERROR",
      );
      const invalidRow = this.validateRow(row, rows, index, refData).length > 0;
      if (invalidCell || invalidRow) errorRowsCount++;
      else validRowsCount++;
    }
    return {
      totalRows: rows.length,
      validRowsCount,
      errorRowsCount,
      warningRowsCount: 0,
      details: { assignmentsToCreate: validRowsCount },
    };
  },

  async commitChunk(rows, _orgId, refData) {
    const rowErrors: Array<{ rowNumber: number; reason: string }> = [];
    for (let index = 0; index < rows.length; index++) {
      const row = rows[index]!;
      const classItem = resolveUnique(refData.classes, row.class_code);
      const periodItem = resolveUnique(refData.academicPeriods, row.academic_period_code);
      const subjectItem = resolveUnique(refData.subjects, row.subject_code);
      const teacherItem = resolveUnique(refData.teachers, row.teacher_code);
      const rowNumber = row.rowNumber || index + 1;

      if (!classItem || !periodItem || !subjectItem || !teacherItem) {
        rowErrors.push({
          rowNumber,
          reason: message(
            "تعذر مطابقة أحد المعرّفات المرجعية. حدّث البيانات المرجعية وأعد المحاولة.",
            "A reference could not be resolved. Refresh reference data and retry.",
          ),
        });
        continue;
      }

      if (classItem.meta?.periodId !== periodItem.id) {
        rowErrors.push({
          rowNumber,
          reason: message(
            "الفترة الأكاديمية المحددة لا تطابق الفترة المرتبطة بالصف.",
            "The selected academic period does not match the class period.",
          ),
        });
        continue;
      }

      const key = assignmentKey(classItem.id, subjectItem.id, periodItem.id);
      if (
        refData.existingClassSubjectAssignments.some(
          (item) => assignmentKey(item.classId, item.subjectId, item.periodId) === key,
        )
      ) {
        rowErrors.push({
          rowNumber,
          reason: message(
            "يوجد إسناد لهذه المادة والصف والفترة بالفعل.",
            "An assignment already exists for this subject, class, and period.",
          ),
        });
        continue;
      }

      try {
        const weeklyHoursText = String(row.weekly_hours ?? "").trim();
        await api.assignClassSubject({
          class_id: classItem.id,
          subject_id: subjectItem.id,
          period_id: periodItem.id,
          primary_teacher_id: teacherItem.id,
          weekly_hours: weeklyHoursText ? Number(weeklyHoursText) : undefined,
        });
        refData.existingClassSubjectAssignments.push({
          classId: classItem.id,
          subjectId: subjectItem.id,
          periodId: periodItem.id,
          teacherId: teacherItem.id,
        });
      } catch (error) {
        rowErrors.push({
          rowNumber,
          reason: error instanceof Error ? error.message : String(error),
        });
      }
    }

    return { success: true, rowErrors: rowErrors.length ? rowErrors : undefined };
  },
};

function resolveUnique(items: ReferenceItem[], value: unknown) {
  const needle = String(value ?? "").trim().toLocaleLowerCase();
  if (!needle) return undefined;
  const matches = items.filter((item) =>
    [item.id, item.code, item.name, item.nameAr]
      .filter((candidate): candidate is string => Boolean(candidate))
      .some((candidate) => candidate.trim().toLocaleLowerCase() === needle),
  );
  return matches.length === 1 ? matches[0] : undefined;
}

function referenceSourceForField(
  field: string,
): "classes" | "academicPeriods" | "subjects" | "teachers" | undefined {
  switch (field) {
    case "class_code":
      return "classes";
    case "academic_period_code":
      return "academicPeriods";
    case "subject_code":
      return "subjects";
    case "teacher_code":
      return "teachers";
    default:
      return undefined;
  }
}

function assignmentKey(classId: string, subjectId: string, periodId: string) {
  return `${classId}:${subjectId}:${periodId}`;
}

function issue(columnId: string, text: string): BulkCellIssue {
  return { severity: "BLOCKING_ERROR", columnId, message: text };
}

function message(ar: string, en: string) {
  return getLocale() === "ar" ? ar : en;
}
