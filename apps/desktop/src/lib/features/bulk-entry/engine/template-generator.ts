import { save } from "@tauri-apps/plugin-dialog";
import { writeFile } from "@tauri-apps/plugin-fs";
import * as m from "$lib/paraglide/messages";
import { getLocale } from "$lib/paraglide/runtime";
import {
  downloadBlob,
  revealDownloadedPath,
  type RevealDownloadedLocation,
} from "$lib/shared/utils/download";
import { getBulkEntryColumnHeader, getBulkEntryProfileName } from "../localization";
import type { BulkImportProfile, BulkReferenceData } from "../types";
import { normalizeRegistrationFieldKey } from "$lib/features/students/registration-fields";

export interface GenerateTemplateOptions {
  profile: BulkImportProfile<any, any>;
  refData: BulkReferenceData;
  orgName?: string;
  academicCalendar?: string;
}

export async function buildTemplateWorkbook(
  options: GenerateTemplateOptions,
): Promise<import("xlsx").WorkBook> {
  const XLSX = await import("xlsx");
  const { profile, refData, orgName, academicCalendar } = options;
  const wb = XLSX.utils.book_new();
  const columns = profile.columnMetadata.filter((column) => {
    if (profile.id !== "students" || !refData.registrationFields) return true;
    if (column.id === "full_name_ar" || column.id === "class_name") return true;
    return refData.registrationFields[normalizeRegistrationFieldKey(column.id)]?.active !== false;
  });

  // ── Sheet 1: Data Entry Sheet ──────────────────────────────────
  const headers = columns.map((col) => {
    const policy = refData.registrationFields?.[normalizeRegistrationFieldKey(col.id)];
    return getBulkEntryColumnHeader(
      profile.id,
      col.id,
      col.headerAr,
      profile.id === "students" ? policy?.required ?? col.required : undefined,
    );
  });
  const sampleRow = columns.map((col) => col.example || "");

  const dataAoa = [headers, sampleRow];
  const wsData = XLSX.utils.aoa_to_sheet(dataAoa);

  // Set column widths based on metadata
  wsData["!cols"] = columns.map((col) => ({
    wch: Math.max(col.headerAr.length * 2, 16),
  }));

  XLSX.utils.book_append_sheet(wb, wsData, m.bulk_entry_workbook_data_sheet());

  // ── Sheet 2: Reference Data Sheet ──────────────────────────────
  const refAoa: string[][] = [];
  const referenceMerges: Array<{
    s: { r: number; c: number };
    e: { r: number; c: number };
  }> = [];

  function appendReferenceSection(
    title: string,
    headers: string[],
    rows: string[][],
  ) {
    if (rows.length === 0) return;
    if (refAoa.length > 0) refAoa.push(["", "", "", ""]);

    const titleRow = refAoa.length;
    refAoa.push([title, "", "", ""]);
    referenceMerges.push({
      s: { r: titleRow, c: 0 },
      e: { r: titleRow, c: 3 },
    });
    refAoa.push(headers);
    refAoa.push(...rows);
  }

  const codeHeader = m.bulk_entry_workbook_reference_code();
  const nameHeader = m.bulk_entry_workbook_reference_name();
  const detailsHeader = m.bulk_entry_workbook_reference_meta();

  appendReferenceSection(
    m.bulk_entry_workbook_class(),
    [codeHeader, nameHeader, detailsHeader],
    refData.classes.map((item) => [
      item.code || item.id,
      item.name || item.nameAr || "",
      String(
        item.meta?.details ??
          m.bulk_entry_workbook_capacity({
            capacity: Number(item.meta?.capacity) || 30,
          }),
      ),
    ]),
  );
  appendReferenceSection(
    m.bulk_entry_workbook_subject(),
    [codeHeader, nameHeader],
    refData.subjects.map((item) => [item.code || "", item.name || item.nameAr || ""]),
  );
  appendReferenceSection(
    m.bulk_entry_workbook_reference_sections(),
    [codeHeader, nameHeader],
    refData.sections.map((item) => [item.code || "", item.name || item.nameAr || ""]),
  );
  appendReferenceSection(
    m.bulk_entry_workbook_reference_levels(),
    [codeHeader, nameHeader],
    refData.levels.map((item) => [item.code || "", item.name || item.nameAr || ""]),
  );
  appendReferenceSection(
    m.bulk_entry_workbook_fee_template(),
    [nameHeader, detailsHeader],
    refData.feeTemplates.map((item) => {
      const meta = item.meta ?? {};
      return [
        item.name || item.nameAr || "",
        m.bulk_entry_workbook_fee_details({
          amount: Number(meta.default_amount) || 0,
          requirement: meta.is_mandatory
            ? m.bulk_entry_workbook_mandatory()
            : m.bulk_entry_workbook_optional(),
        }),
      ];
    }),
  );
  appendReferenceSection(
    m.bulk_entry_workbook_reference_discount_templates(),
    [codeHeader, nameHeader],
    refData.discountTemplates.map((item) => [
      item.code || "",
      item.name || item.nameAr || "",
    ]),
  );
  appendReferenceSection(
    m.bulk_entry_workbook_user_group(),
    [codeHeader, nameHeader],
    refData.userGroups.map((item) => [
      item.code || item.id,
      item.name || item.nameAr || "",
    ]),
  );
  appendReferenceSection(
    m.bulk_entry_workbook_reference_academic_periods(),
    [codeHeader, nameHeader],
    refData.academicPeriods.map((item) => [
      item.code || "",
      item.name || item.nameAr || "",
    ]),
  );
  appendReferenceSection(
    m.bulk_entry_workbook_assessment(),
    [nameHeader, detailsHeader],
    refData.assessmentSchemes.map((item) => [
      item.name || item.nameAr || "",
      m.bulk_entry_workbook_max_score({
        score: Number(item.meta?.max_score) || 100,
      }),
    ]),
  );

  const enumValuesByField = new Map<
    string,
    Array<{ value: string; label: string }>
  >([
    [
      "gender",
      refData.enums.genders.map((item) => ({
        value: item.value,
        label: getLocale() === "ar" ? item.labelAr : item.labelEn,
      })),
    ],
    [
      "study_mode",
      refData.enums.studyModes.map((item) => ({
        value: item.value,
        label: getLocale() === "ar" ? item.labelAr : item.labelEn,
      })),
    ],
    [
      "employee_type",
      refData.enums.employeeTypes.map((item) => ({
        value: item.value,
        label: getLocale() === "ar" ? item.labelAr : item.labelEn,
      })),
    ],
    [
      "employment_type",
      refData.enums.employmentTypes.map((item) => ({
        value: item.value,
        label: getLocale() === "ar" ? item.labelAr : item.labelEn,
      })),
    ],
    [
      "blood_type",
      refData.enums.bloodTypes.map((value) => ({ value, label: value })),
    ],
    [
      "guardian_relationship",
      refData.enums.relationships.map((item) => ({
        value: item.value,
        label: getLocale() === "ar" ? item.labelAr : item.labelEn,
      })),
    ],
  ]);

  for (const column of columns.filter((item) => item.type === "enum")) {
    const labels = new Map(
      (enumValuesByField.get(column.id) ?? []).map((item) => [
        item.value,
        item.label,
      ]),
    );
    for (const option of column.options ?? []) {
      labels.set(String(option.value), option.label);
    }
    if (labels.size === 0 && column.allowedValues) {
      for (const value of column.allowedValues) labels.set(value, value);
    }

    const fieldLabel = getBulkEntryColumnHeader(
      profile.id,
      column.id,
      column.headerAr,
    );
    appendReferenceSection(
      fieldLabel,
      [codeHeader, nameHeader],
      [...labels].map(([value, label]) => [value, label]),
    );
  }

  for (const column of columns.filter((item) => item.type === "boolean")) {
    const fieldLabel = getBulkEntryColumnHeader(
      profile.id,
      column.id,
      column.headerAr,
    );
    appendReferenceSection(fieldLabel, [codeHeader, nameHeader], [
      ["true", m.bulk_entry_workbook_boolean_yes()],
      ["false", m.bulk_entry_workbook_boolean_no()],
    ]);
  }

  appendReferenceSection(
    m.bulk_entry_workbook_teachers(),
    [codeHeader, nameHeader],
    refData.teachers.map((item) => [
      item.code || item.id,
      item.name || item.nameAr || "",
    ]),
  );

  const wsRef = XLSX.utils.aoa_to_sheet(refAoa);
  wsRef["!cols"] = [{ wch: 28 }, { wch: 32 }, { wch: 36 }, { wch: 32 }];
  wsRef["!merges"] = referenceMerges;
  XLSX.utils.book_append_sheet(wb, wsRef, `_${m.bulk_entry_workbook_reference_sheet()}`);

  // ── Sheet 3: Instructions ──────────────────────────────────────
  const instructionsAoa = [
    [m.bulk_entry_workbook_instructions_title()],
    [""],
    [m.bulk_entry_workbook_organization(), orgName || m.bulk_entry_workbook_current_organization()],
    [m.bulk_entry_workbook_academic_calendar(), academicCalendar || m.bulk_entry_workbook_current_year()],
    [m.bulk_entry_workbook_template_type(), getBulkEntryProfileName(profile.id)],
    [m.bulk_entry_workbook_template_version(), profile.templateVersion],
    ...(profile.id === "teacher-subjects"
      ? [[m.bulk_entry_workbook_rule_teacher_subject_assignment()]]
      : []),
    [""],
    [m.bulk_entry_workbook_general_rules()],
    [m.bulk_entry_workbook_rule_required_fields()],
    [m.bulk_entry_workbook_rule_date_format()],
    [m.bulk_entry_workbook_rule_unique_ids()],
    [m.bulk_entry_workbook_rule_reference_sheet()],
    [m.bulk_entry_workbook_rule_upload()],
  ];

  const wsInstructions = XLSX.utils.aoa_to_sheet(instructionsAoa);
  wsInstructions["!cols"] = [{ wch: 80 }];
  XLSX.utils.book_append_sheet(wb, wsInstructions, m.bulk_entry_workbook_instructions_sheet());

  return wb;
}

export async function downloadExcelTemplate(
  options: GenerateTemplateOptions,
): Promise<RevealDownloadedLocation | null> {
  const XLSX = await import("xlsx");
  const wb = await buildTemplateWorkbook(options);
  const rawBytes = XLSX.write(wb, { bookType: "xlsx", type: "array" });
  const filename = `Orbis_Template_${options.profile.id}_${Date.now()}.xlsx`;

  try {
    // Try native Tauri save dialog
    const savePath = await save({
      defaultPath: filename,
      filters: [{ name: m.bulk_entry_file_filter_excel(), extensions: ["xlsx"] }],
    });

    if (savePath) {
      await writeFile(savePath, new Uint8Array(rawBytes));
      return revealDownloadedPath(savePath);
    }
  } catch (err) {
    console.warn("Native Tauri save dialog not available, falling back to browser download:", err);
  }

  // Browser Fallback
  const blob = new Blob([rawBytes], {
    type: "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
  });
  return downloadBlob(blob, filename);
}
