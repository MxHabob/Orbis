"use client";

import { useState, useCallback, useRef, useMemo } from "react";
import { HIJRI_MONTHS } from "@/utils/academic-calendar";
import { useRouter } from "next/navigation";
import { useTRPC } from "@/trpc/client";
import { SheetContent } from "@midday/ui/sheet";
import { useCertificateParams } from "@/hooks/use-certificate-params";
import { useToast } from "@midday/ui/use-toast";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useI18n, useCurrentLocale } from "@/locales/client";

import { Sidebar } from "./editor/sidebar";
import { Toolbar } from "./editor/toolbar";
import { Canvas } from "./editor/canvas";
import { PAPER_SIZES, type EditorElement, type ElementType, type PaperSize } from "./editor/types";

export function CertificateEditor({
  initialData,
  mode = "create",
  isSheet,
}: {
  initialData?: {
    id?: string;
    name?: string;
    size?: PaperSize;
    orientation?: "portrait" | "landscape";
    backgroundImageUrl?: string;
    content?: { elements: EditorElement[] };
  };
  mode?: "create" | "edit";
  isSheet?: boolean;
}) {
  const t = useI18n();
  const locale = useCurrentLocale();
  const { setParams } = useCertificateParams();
  const router = useRouter();
  const trpc = useTRPC();
  const [isSaving, setIsSaving] = useState(false);
  const [isDragging, setIsDragging] = useState(false);
  const { toast } = useToast();
  const queryClient = useQueryClient();

  const canvasRef = useRef<HTMLDivElement>(null);
  const [name, setName] = useState(initialData?.name || "New Template");
  const [paperSize, setPaperSize] = useState<PaperSize>(initialData?.size || "a4");
  const [orientation, setOrientation] = useState<"portrait" | "landscape">(initialData?.orientation || "portrait");
  const [backgroundImage, setBackgroundImage] = useState(initialData?.backgroundImageUrl || "");
  const [elements, setElements] = useState<EditorElement[]>(initialData?.content?.elements || []);
  const [selectedElementId, setSelectedElementId] = useState<string | null>(null);
  const [isPreview, setIsPreview] = useState(false);
  const [selectedType, setSelectedType] = useState<"monthly" | "half-yearly" | "final">("monthly");
  const [selectedMonth, setSelectedMonth] = useState<number>(1);
  const [selectedYear, setSelectedYear] = useState<string>("2023/2024");
  const [previewStudentId, setPreviewStudentId] = useState<string | null>(null);
  
  const { data: students } = useQuery(trpc.students.getAll.queryOptions());

  const selectedPreviewStudent = useMemo(() => 
    students?.find(s => s.userId === previewStudentId) || students?.[0],
    [students, previewStudentId]
  );

  const rankingPeriod = selectedType === "half-yearly" ? "half-yearly" : selectedType === "final" ? "final" : "monthly";
  const rankingMonth = rankingPeriod === "monthly" ? selectedMonth : undefined;
  const previewClassId = (selectedPreviewStudent as any)?.currentClass?.id;
  const previewAcademicYearId = (selectedPreviewStudent as any)?.currentClass?.academicYearId;

  const { data: topStudents } = useQuery(
    trpc.grades.getTopStudents.queryOptions(
      {
        classId: previewClassId || 0,
        academicYearId: previewAcademicYearId || 0,
        period: rankingPeriod,
        month: rankingMonth,
        limit: 10,
      },
      {
        enabled:
          !!previewClassId &&
          !!previewAcademicYearId &&
          (rankingPeriod !== "monthly" || !!rankingMonth),
      }
    )
  );

  const getPreviewData = useCallback((student: any, type: string, month: number, year: string, rankingData?: any[]) => {
    if (!student) return {};
    const stats = (student.stats as any) || {};
    const currentStudentRank = (rankingData || []).find((item: any) => item.studentId === student.userId);

    const typeLabel = type === "half-yearly" ? t("reports.preview.type_half_yearly") : 
                     type === "final" ? t("reports.preview.type_final") : t("reports.preview.type_monthly");
    
    const monthName = month ? (HIJRI_MONTHS.find(m => m.id === month)?.name || t("reports.month_label", { month })) : "";

    return {
      student_name: student.name || "عبدالله محمد الهبوب",
      name: student.name || "عبدالله محمد الهبوب",
      academic_number: student.studentNumber || "20240001",
      national_id: student.secretNumber || "123456789",
      department: stats.department || t("reports.preview.department"),
      section: stats.section || t("reports.preview.section"),
      class_name: student.currentClass?.name || t("reports.preview.class_name"),
      total_grade: stats.total_grade || "0",
      total_grade_text: stats.total_grade_text || "",
      grade_label: stats.grade_label || "",
      final_result: stats.final_result || "",
      rank: currentStudentRank?.rankLabel || stats.rank || "",
      rank_percentage: currentStudentRank?.percentage ?? "",
      rank_total: currentStudentRank?.totalScore ?? "",
      rank_max: currentStudentRank?.maxScore ?? "",
      org_name: t("reports.preview.org_name"),
      admin_name: t("reports.preview.admin_name"),
      vp_name: t("reports.preview.vp_name"),
      issue_date: new Date().toLocaleDateString(locale === 'ar' ? 'ar-YE' : 'en-US'),
      report_type: type === "half-yearly" ? "النصفي" : (type === "final" ? "النهائي" : `شهري - لشهر ${monthName}`),
      month_name: monthName,
      academic_year: year,
    };
  }, [t, locale]);

  const previewData = useMemo(() => 
    getPreviewData(selectedPreviewStudent, selectedType, selectedMonth, selectedYear, topStudents as any[]), 
    [selectedPreviewStudent, selectedType, selectedMonth, selectedYear, topStudents, getPreviewData]
  );

  const upsertTemplate = useMutation(
    trpc.certificateTemplates.upsert.mutationOptions({
      onSuccess: () => {
        // Invalidate all certificate template queries to ensure fresh data everywhere
        queryClient.invalidateQueries({
          queryKey: trpc.certificateTemplates.list.queryKey(),
        });
        queryClient.invalidateQueries({
          queryKey: trpc.certificateTemplates.getById.queryKey(),
        });
        queryClient.invalidateQueries({
          queryKey: trpc.certificateTemplates.summary.queryKey(),
        });
      },
    }),
  );

  const canvasWidth = useMemo(() => {
    const size = (paperSize && PAPER_SIZES[paperSize]) || PAPER_SIZES.a4 || PAPER_SIZES.letter;
    if (!size) return 595; // Absolute fallback for A4 width
    return orientation === "portrait" ? size.width : size.height;
  }, [orientation, paperSize]);
  
  const canvasHeight = useMemo(() => {
    const size = (paperSize && PAPER_SIZES[paperSize]) || PAPER_SIZES.a4 || PAPER_SIZES.letter;
    if (!size) return 842; // Absolute fallback for A4 height
    return orientation === "portrait" ? size.height : size.width;
  }, [orientation, paperSize]);

  const addElement = useCallback((type: ElementType, content: string, variableKey?: string) => {
    const newElement: EditorElement = {
      id: `el-${Date.now()}`,
      type,
      content,
      variableKey,
      x: 50,
      y: 50,
      opacity: 1,
      rotation: 0,
      ...(type === "image" ? { width: 20, height: 20 } : 
         type === "shape" ? { width: 30, height: 0.5, backgroundColor: "#000000" } :
         type === "table" ? { 
            width: 80, 
            height: 20, 
            fontSize: 10,
            columns: [
              { id: "1", header: "Subject", width: 40 },
              { id: "2", header: "Assessment", width: 30 },
              { id: "3", header: "Total", width: 30 }
            ] 
         } : {
        fontSize: 18,
        fontWeight: "bold",
        color: "#000000",
        align: "right",
        fontFamily: "inherit"
      })
    };
    
    setElements(prev => [...prev, newElement]);
    setSelectedElementId(newElement.id);
  }, []);

  const updateElement = useCallback((id: string, updates: Partial<EditorElement>) => {
    setElements(prev => prev.map(el => el.id === id ? { ...el, ...updates } : el));
  }, []);

  const deleteElement = useCallback((id: string) => {
    setElements(prev => prev.filter(el => el.id !== id));
    setSelectedElementId(null);
  }, []);

  const handleDragEnd = useCallback((id: string, info: any, canvasRect: DOMRect, elementRect: DOMRect) => {
    // Get center of element to keep the "center-origin" logic consistent
    const centerX = elementRect.left + elementRect.width / 2;
    const centerY = elementRect.top + elementRect.height / 2;
    
    // Calculate percentage relative to canvas
    const x = ((centerX - canvasRect.left) / canvasRect.width) * 100;
    const y = ((centerY - canvasRect.top) / canvasRect.height) * 100;
    
    updateElement(id, { 
      x: Math.max(0, Math.min(100, x)), 
      y: Math.max(0, Math.min(100, y)) 
    });
  }, [updateElement]);

  const handleFileUpload = async (e: React.ChangeEvent<HTMLInputElement>, elementType: "background" | "image") => {
    const file = e.target.files?.[0];
    if (!file) return;

    const reader = new FileReader();
    reader.onload = (event) => {
      const dataUrl = event.target?.result as string;
      if (elementType === "background") {
        setBackgroundImage(dataUrl);
      } else {
        addElement("image", dataUrl);
      }
    };
    reader.readAsDataURL(file);
  };

  const handleSave = async () => {
    setIsSaving(true);
    try {
      const payload = {
        name,
        backgroundImageUrl: backgroundImage,
        content: { elements },
        size: paperSize,
        orientation,
      };
      
      await upsertTemplate.mutateAsync({
        ...(mode === "edit" ? { id: initialData?.id } : {}),
        ...payload,
      });

      toast({ title: mode === "edit" ? "Template saved successfully" : "Template created successfully", variant: "success" });
      router.refresh();
      
      if (isSheet) {
        setParams(null);
      } else if (mode === "create") {
        router.push("/certificates");
      }
    } catch (e) {
      toast({ title: "Failed to save template", variant: "destructive" });
    } finally {
      setIsSaving(false);
    }
  };

  const handleBack = useCallback(() => {
    if (isSheet) {
      setParams(null);
    } else {
      router.back();
    }
  }, [isSheet, router, setParams]);

  const handleClearBackground = useCallback(() => {
    setBackgroundImage("");
  }, []);

  const EditorContent = (
    <div className="relative h-full flex w-full overflow-hidden">
      <Sidebar 
        name={name}
        setName={setName}
        isSheet={isSheet}
        onBack={handleBack}
        elements={elements}
        selectedElementId={selectedElementId}
        setSelectedElementId={setSelectedElementId}
        paperSize={paperSize}
        setPaperSize={setPaperSize}
        orientation={orientation}
        setOrientation={setOrientation}
        backgroundImage={backgroundImage}
        handleFileUpload={handleFileUpload}
        handleClearBackground={handleClearBackground}
        addElement={addElement}
        updateElement={updateElement}
        deleteElement={deleteElement}
        reorderElements={setElements}
        handleSave={handleSave}
        isSaving={isSaving}
        mode={mode}
      />

      <div className="flex-1 flex flex-col min-w-0 overflow-hidden bg-[#fcfcfc] dark:bg-[#0f0f0f]">
        <Toolbar 
          paperSize={paperSize}
          orientation={orientation}
          isPreview={isPreview}
          setIsPreview={setIsPreview}
          previewStudentId={previewStudentId}
          setPreviewStudentId={setPreviewStudentId}
          students={students}
          selectedType={selectedType}
          setSelectedType={setSelectedType}
          selectedMonth={selectedMonth}
          setSelectedMonth={setSelectedMonth}
          selectedYear={selectedYear}
          setSelectedYear={setSelectedYear}
        />

          <Canvas 
            canvasRef={canvasRef}
            orientation={orientation}
            paperSize={paperSize}
            canvasWidth={canvasWidth}
            canvasHeight={canvasHeight}
            isPreview={isPreview}
            backgroundImage={backgroundImage}
            elements={elements}
            selectedElementId={selectedElementId}
            setSelectedElementId={setSelectedElementId}
            previewData={previewData}
            students={students}
            previewStudentId={previewStudentId}
            updateElement={updateElement}
            onDelete={deleteElement}
            certificateType={selectedType}
            month={selectedMonth}
          />
      </div>
    </div>
  );

  if (isSheet) {
    return (
      <SheetContent
        style={{ maxWidth: 1400 }}
        className="bg-white dark:bg-[#080808] p-0 [&>button]:hidden"
      >
        {EditorContent}
      </SheetContent>
    );
  }

  return EditorContent;
}