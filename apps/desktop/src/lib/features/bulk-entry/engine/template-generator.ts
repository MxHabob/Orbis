import { save } from "@tauri-apps/plugin-dialog";
import { writeFile } from "@tauri-apps/plugin-fs";
import type { BulkImportProfile, BulkReferenceData } from "../types";

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

  // ── Sheet 1: Data Entry Sheet ──────────────────────────────────
  const headers = profile.columnMetadata.map((col) => col.headerAr);
  const sampleRow = profile.columnMetadata.map((col) => col.example || "");

  const dataAoa = [headers, sampleRow];
  const wsData = XLSX.utils.aoa_to_sheet(dataAoa);

  // Set column widths based on metadata
  wsData["!cols"] = profile.columnMetadata.map((col) => ({
    wch: Math.max(col.headerAr.length * 2, 16),
  }));

  XLSX.utils.book_append_sheet(wb, wsData, "البيانات (Data)");

  // ── Sheet 2: Reference Data Sheet ──────────────────────────────
  const refAoa: string[][] = [];
  refAoa.push(["النوع (Type)", "الرمز (Code)", "الاسم (Name)", "معلومات إضافية (Meta)"]);

  if (refData.classes.length > 0) {
    for (const c of refData.classes) {
      refAoa.push(["صف (Class)", c.code || "", c.name || c.nameAr || "", `السعة: ${c.meta?.capacity || 30}`]);
    }
  }

  if (refData.subjects.length > 0) {
    for (const s of refData.subjects) {
      refAoa.push(["مادة (Subject)", s.code || "", s.name || s.nameAr || "", ""]);
    }
  }

  if (refData.feeTemplates.length > 0) {
    for (const f of refData.feeTemplates) {
      refAoa.push([
        "قالب رسوم (Fee Template)",
        "",
        f.name,
        `المبلغ: ${(f.meta as any)?.default_amount || 0} (${(f.meta as any)?.is_mandatory ? "إلزامي" : "اختياري"})`,
      ]);
    }
  }

  if (refData.userGroups.length > 0) {
    for (const g of refData.userGroups) {
      refAoa.push(["مجموعة صلاحيات (User Group)", "", g.name, ""]);
    }
  }

  if (refData.assessmentSchemes.length > 0) {
    for (const a of refData.assessmentSchemes) {
      refAoa.push(["لائحة تقييم (Assessment)", "", a.name, `الدرجة العظمى: ${(a.meta as any)?.max_score || 100}`]);
    }
  }

  const wsRef = XLSX.utils.aoa_to_sheet(refAoa);
  wsRef["!cols"] = [{ wch: 25 }, { wch: 15 }, { wch: 30 }, { wch: 25 }];
  XLSX.utils.book_append_sheet(wb, wsRef, "_البيانات_المرجعية");

  // ── Sheet 3: Instructions ──────────────────────────────────────
  const instructionsAoa = [
    ["نظام Orbis لإدارة التعليم — إرشادات تعبئة ملف الاستيراد"],
    [""],
    ["المؤسسة التعليمية:", orgName || "المؤسسة الحالية"],
    ["العام الأكاديمي:", academicYear || "العام الحالي"],
    ["نوع القالب:", profile.displayName.ar],
    ["إصدار القالب:", profile.templateVersion],
    [""],
    ["قواعد عامة هامة:"],
    ["1. الحقول التي تنتهي بعلامة (*) هي حقول إلزامية لا يمكن تركها فارغة."],
    ["2. صيغة التواريخ المعتمدة في النظام هي: YYYY-MM-DD (مثال: 2015-04-12)."],
    ["3. الأرقام الوطنية وأرقام الهواتف يجب أن تكون فريدة وغير مكررة داخل الملف."],
    ["4. يمكنك مراجعة ورقة '_البيانات_المرجعية' لمعرفة أسماء الصفوف والمواد والقوالب المعتمدة."],
    ["5. بعد تعبئة البيانات، احفظ الملف ثم ارفعه عبر شاشة 'الاستيراد الجماعي' في Orbis لمراجعته في OrbisGrid."],
  ];

  const wsInstructions = XLSX.utils.aoa_to_sheet(instructionsAoa);
  wsInstructions["!cols"] = [{ wch: 80 }];
  XLSX.utils.book_append_sheet(wb, wsInstructions, "التعليمات (Instructions)");

  return wb;
}

export async function downloadExcelTemplate(options: GenerateTemplateOptions): Promise<void> {
  const XLSX = await import("xlsx");
  const wb = await buildTemplateWorkbook(options);
  const rawBytes = XLSX.write(wb, { bookType: "xlsx", type: "array" });
  const filename = `Orbis_Template_${options.profile.id}_${Date.now()}.xlsx`;

  try {
    // Try native Tauri save dialog
    const savePath = await save({
      defaultPath: filename,
      filters: [{ name: "Excel Spreadsheet", extensions: ["xlsx"] }],
    });

    if (savePath) {
      await writeFile(savePath, new Uint8Array(rawBytes));
      return;
    }
  } catch (err) {
    console.warn("Native Tauri save dialog not available, falling back to browser download:", err);
  }

  // Browser Fallback
  const blob = new Blob([rawBytes], {
    type: "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
  });
  const url = URL.createObjectURL(blob);
  const link = document.createElement("a");
  link.href = url;
  link.download = filename;
  document.body.appendChild(link);
  link.click();
  setTimeout(() => {
    URL.revokeObjectURL(url);
    link.remove();
  }, 100);
}
