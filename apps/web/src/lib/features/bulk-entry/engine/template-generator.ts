import * as m from "$lib/paraglide/messages";
import { getLocale } from "$lib/paraglide/runtime";
import {
  getBulkEntryColumnHeader,
  getBulkEntryProfileName,
} from "../localization";
import {
  getActiveImportColumns,
  isImportColumnRequired,
} from "./template-columns";
import type {
  BulkImportProfile,
  BulkReferenceData,
  BulkProfileId,
  ReferenceItem,
} from "../types";
import { extractSchemeComponents } from "../../exams/grade-helpers";

export interface GenerateTemplateOptions {
  profile: BulkImportProfile<any, any>;
  refData: BulkReferenceData;
  orgName?: string;
  academicCalendar?: string;
}

const referenceSources = {
  students: ["classes", "userGroups"],
  employees: ["userGroups"],
  "teacher-subjects": ["classes", "academicPeriods", "subjects", "teachers"],
  grades: ["academicPeriods", "assessmentSchemes"],
} as const satisfies Record<
  BulkProfileId,
  readonly (keyof BulkReferenceData)[]
>;

const referenceLabels = {
  classes: m.bulk_entry_workbook_class,
  userGroups: m.bulk_entry_workbook_user_group,
  academicPeriods: m.bulk_entry_workbook_reference_academic_periods,
  subjects: m.bulk_entry_workbook_subject,
  teachers: m.bulk_entry_workbook_teachers,
  assessmentSchemes: m.bulk_entry_workbook_assessment,
};

export async function buildTemplateWorkbook(
  options: GenerateTemplateOptions,
): Promise<import("xlsx").WorkBook> {
  const XLSX = await import("xlsx");
  const { profile, refData, orgName, academicCalendar } = options;
  const arabic = getLocale() === "ar";
  const workbook = XLSX.utils.book_new();
  workbook.Workbook = { Views: [{ RTL: arabic }] };
  const columns = getActiveImportColumns(profile, refData.registrationFields);
  const headers = columns.map((column) =>
    getBulkEntryColumnHeader(
      profile.id,
      column.id,
      arabic ? column.headerAr : column.headerEn,
      isImportColumnRequired(profile.id, column, refData.registrationFields),
    ),
  );

  const dataSheet = XLSX.utils.aoa_to_sheet([
    headers,
    ...Array.from({ length: 100 }, () =>
      columns.map((column) =>
        column.type === "decimal" ||
        column.type === "number" ||
        column.type === "integer"
          ? ""
          : { t: "s", v: "", z: "@" },
      ),
    ),
  ]);
  dataSheet["!cols"] = headers.map((header) => ({
    wch: Math.min(36, Math.max(18, header.length + 6)),
  }));
  dataSheet["!rows"] = [{ hpt: 27 }];
  dataSheet["!autofilter"] = { ref: dataSheet["!ref"]! };
  columns.forEach((column, index) => {
    const note =
      (arabic ? column.descriptionAr : column.descriptionEn) || column.format;
    if (note)
      dataSheet[XLSX.utils.encode_cell({ r: 0, c: index })].c = [
        { a: "Orbis", t: note },
      ];
  });
  XLSX.utils.book_append_sheet(
    workbook,
    dataSheet,
    m.bulk_entry_workbook_data_sheet(),
  );

  const code = m.bulk_entry_workbook_reference_code();
  const name = m.bulk_entry_workbook_reference_name();
  const referenceRows: string[][] = [];
  const addSection = (label: string, fields: string[], rows: string[][]) => {
    if (rows.length) referenceRows.push([label], fields, ...rows, []);
  };
  const itemName = (item: ReferenceItem) =>
    (arabic && item.nameAr) || item.name;
  for (const key of referenceSources[profile.id]) {
    if (key === "assessmentSchemes") {
      addSection(
        referenceLabels[key](),
        [
          code,
          name,
          referenceLabels[key](),
          m.bulk_entry_workbook_reference_meta(),
        ],
        refData[key].flatMap((item) =>
          extractSchemeComponents(item.meta ?? {}).map((component) => [
            component.code,
            (arabic && component.name_ar) || component.name,
            itemName(item),
            String(component.max_score),
          ]),
        ),
      );
    } else {
      addSection(
        referenceLabels[key](),
        [code, name, "ID"],
        refData[key].map((item) => [
          item.code ?? item.id,
          itemName(item),
          item.id,
        ]),
      );
    }
  }

  const enums = refData.enums;
  const enumOptions = {
    gender: enums.genders,
    study_mode: enums.studyModes,
    employee_type: enums.employeeTypes,
    employment_type: enums.employmentTypes,
    guardian_relationship: enums.relationships,
  };
  columns.forEach((column, index) => {
    const values = enumOptions[column.id as keyof typeof enumOptions] ?? [];
    const pairs = new Map(
      values.map((option) => [
        option.value,
        arabic ? option.labelAr : option.labelEn,
      ]),
    );
    for (const option of column.options ?? []) {
      if (!pairs.has(String(option.value)))
        pairs.set(String(option.value), option.label);
    }
    for (const value of column.allowedValues ?? []) {
      if (!pairs.has(value)) pairs.set(value, value);
    }
    if (column.id === "blood_type")
      for (const value of enums.bloodTypes) pairs.set(value, value);
    if (column.type === "boolean") {
      pairs.set("true", m.bulk_entry_workbook_boolean_yes());
      pairs.set("false", m.bulk_entry_workbook_boolean_no());
    }
    addSection(headers[index]!, [code, name], [...pairs]);
  });
  if (referenceRows.length) {
    const referenceSheet = XLSX.utils.aoa_to_sheet(referenceRows);
    referenceSheet["!cols"] = [
      { wch: 32 },
      { wch: 36 },
      { wch: 38 },
      { wch: 22 },
    ];
    XLSX.utils.book_append_sheet(
      workbook,
      referenceSheet,
      `_${m.bulk_entry_workbook_reference_sheet()}`,
    );
  }

  const guide = XLSX.utils.aoa_to_sheet([
    [m.bulk_entry_workbook_instructions_title()],
    [
      m.bulk_entry_workbook_template_type(),
      getBulkEntryProfileName(profile.id),
    ],
    [
      m.bulk_entry_workbook_organization(),
      orgName || m.bulk_entry_workbook_current_organization(),
    ],
    [
      m.bulk_entry_workbook_academic_year(),
      academicCalendar || m.bulk_entry_workbook_current_year(),
    ],
    [m.bulk_entry_workbook_template_version(), profile.templateVersion],
    [],
    [m.bulk_entry_workbook_input_rule()],
    [m.bulk_entry_workbook_rule_required_fields()],
    [m.bulk_entry_workbook_rule_date_format()],
    ...(referenceRows.length ? [[m.bulk_entry_workbook_reference_rule()]] : []),
    ...(profile.id === "teacher-subjects"
      ? [[m.bulk_entry_workbook_rule_teacher_subject_assignment()]]
      : []),
    [],
    [
      m.bulk_entry_workbook_field(),
      m.bulk_entry_workbook_example(),
      m.bulk_entry_workbook_reference_meta(),
    ],
    ...columns.map((column, index) => [
      headers[index],
      column.example ?? "",
      (arabic ? column.descriptionAr : column.descriptionEn) ||
        column.format ||
        "",
    ]),
  ]);
  guide["!cols"] = [{ wch: 44 }, { wch: 36 }, { wch: 72 }];
  XLSX.utils.book_append_sheet(
    workbook,
    guide,
    `_${m.bulk_entry_workbook_instructions_sheet()}`,
  );
  return workbook;
}

export async function downloadExcelTemplate(options: GenerateTemplateOptions) {
  const XLSX = await import("xlsx");
  const { downloadSpreadsheet } = await import("./workbook-download");
  const workbook = await buildTemplateWorkbook(options);
  const bytes = XLSX.write(workbook, {
    bookType: "xlsx",
    type: "array",
  }) as ArrayBuffer;
  return downloadSpreadsheet(
    bytes,
    `Orbis-${options.profile.id}-template.xlsx`,
    "xlsx",
  );
}
