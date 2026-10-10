import { save } from "@tauri-apps/plugin-dialog";
import { writeFile } from "@tauri-apps/plugin-fs";
import * as m from "$lib/paraglide/messages";
import { getLocale } from "$lib/paraglide/runtime";
import {
  downloadBlob,
  openDownloadedPath,
  type OpenDownloadedFile,
} from "$lib/shared/utils/download";
import { getBulkEntryColumnHeader, getBulkEntryProfileName } from "../localization";
import type { BulkImportProfile, BulkReferenceData } from "../types";
import { normalizeRegistrationFieldKey } from "$lib/features/students/registration-fields";

export interface GenerateTemplateOptions {
  profile: BulkImportProfile<any, any>;
  refData: BulkReferenceData;
  orgName?: string;
  academicYear?: string;
}

export async function buildTemplateWorkbook(
  options: GenerateTemplateOptions,
): Promise<import("xlsx").WorkBook> {
  const XLSX = await import("xlsx");
  const { profile, refData, orgName, academicYear } = options;
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
  refAoa.push([
    m.bulk_entry_workbook_reference_type(),
    m.bulk_entry_workbook_reference_code(),
    m.bulk_entry_workbook_reference_name(),
    m.bulk_entry_workbook_reference_meta(),
  ]);

  if (refData.classes.length > 0) {
    for (const c of refData.classes) {
      refAoa.push([m.bulk_entry_workbook_class(), c.code || "", c.name || c.nameAr || "", m.bulk_entry_workbook_capacity({ capacity: c.meta?.capacity || 30 })]);
    }
  }

  if (refData.subjects.length > 0) {
    for (const s of refData.subjects) {
      refAoa.push([m.bulk_entry_workbook_subject(), s.code || "", s.name || s.nameAr || "", ""]);
    }
  }

  if (refData.feeTemplates.length > 0) {
    for (const f of refData.feeTemplates) {
      refAoa.push([
        m.bulk_entry_workbook_fee_template(),
        "",
        f.name,
        m.bulk_entry_workbook_fee_details({
          amount: (f.meta as any)?.default_amount || 0,
          requirement: (f.meta as any)?.is_mandatory ? m.bulk_entry_workbook_mandatory() : m.bulk_entry_workbook_optional(),
        }),
      ]);
    }
  }

  if (refData.userGroups.length > 0) {
    for (const g of refData.userGroups) {
      refAoa.push([m.bulk_entry_workbook_user_group(), "", g.name, ""]);
    }
  }

  if (refData.assessmentSchemes.length > 0) {
    for (const a of refData.assessmentSchemes) {
      refAoa.push([m.bulk_entry_workbook_assessment(), "", a.name, m.bulk_entry_workbook_max_score({ score: (a.meta as any)?.max_score || 100 })]);
    }
  }

  const enumReferenceGroups = [
    { fieldId: "gender", values: refData.enums.genders },
    { fieldId: "study_mode", values: refData.enums.studyModes },
    { fieldId: "employee_type", values: refData.enums.employeeTypes },
    { fieldId: "employment_type", values: refData.enums.employmentTypes },
    {
      fieldId: "blood_type",
      values: refData.enums.bloodTypes.map((value) => ({
        value,
        labelAr: value,
        labelEn: value,
      })),
    },
    { fieldId: "guardian_relationship", values: refData.enums.relationships },
  ];

  for (const { fieldId, values } of enumReferenceGroups) {
    const column = columns.find((item) => item.id === fieldId);
    if (!column) continue;
    const fieldLabel = getBulkEntryColumnHeader(
      profile.id,
      fieldId,
      column.headerAr,
    );
    const labelsByValue = new Map(
      values.map((option) => [
        option.value,
        getLocale() === "ar" ? option.labelAr : option.labelEn,
      ]),
    );
    const optionLabels = new Map(
      (column.options ?? []).map((option) => [
        String(option.value),
        option.label,
      ]),
    );
    const allowedValues = new Set([
      ...values.map((option) => option.value),
      ...(column.options ?? []).map((option) => String(option.value)),
    ]);

    for (const value of allowedValues) {
      refAoa.push([
        fieldLabel,
        value,
        optionLabels.get(value) ?? labelsByValue.get(value) ?? value,
        m.bulk_entry_workbook_reference_enum_value(),
      ]);
    }
  }

  for (const column of columns.filter((item) => item.type === "boolean")) {
    const fieldLabel = getBulkEntryColumnHeader(
      profile.id,
      column.id,
      column.headerAr,
    );
    refAoa.push(
      [
        fieldLabel,
        "true",
        m.bulk_entry_workbook_boolean_yes(),
        m.bulk_entry_workbook_reference_enum_value(),
      ],
      [
        fieldLabel,
        "false",
        m.bulk_entry_workbook_boolean_no(),
        m.bulk_entry_workbook_reference_enum_value(),
      ],
    );
  }

  const wsRef = XLSX.utils.aoa_to_sheet(refAoa);
  wsRef["!cols"] = [{ wch: 25 }, { wch: 15 }, { wch: 30 }, { wch: 25 }];
  XLSX.utils.book_append_sheet(wb, wsRef, `_${m.bulk_entry_workbook_reference_sheet()}`);

  // ── Sheet 3: Instructions ──────────────────────────────────────
  const instructionsAoa = [
    [m.bulk_entry_workbook_instructions_title()],
    [""],
    [m.bulk_entry_workbook_organization(), orgName || m.bulk_entry_workbook_current_organization()],
    [m.bulk_entry_workbook_academic_year(), academicYear || m.bulk_entry_workbook_current_year()],
    [m.bulk_entry_workbook_template_type(), getBulkEntryProfileName(profile.id)],
    [m.bulk_entry_workbook_template_version(), profile.templateVersion],
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
): Promise<OpenDownloadedFile | null> {
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
      return openDownloadedPath(savePath);
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
