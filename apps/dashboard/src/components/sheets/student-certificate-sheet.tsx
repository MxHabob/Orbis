"use client";

import { useStudentParams } from "@/hooks/use-student-params";
import { Sheet, SheetContent } from "@midday/ui/sheet";
import { useI18n, useCurrentLocale } from "@/locales/client";
import React, { useState } from "react";
import { useTRPC } from "@/trpc/client";
import { HIJRI_MONTHS } from "@/utils/academic-calendar";
import { useQuery } from "@tanstack/react-query";
import { CertificateViewer } from "../certificates/certificate-viewer";
import { Button } from "@midday/ui/button";
import {
  DropdownMenu,
  DropdownMenuCheckboxItem,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@midday/ui/dropdown-menu";
import { Icons } from "@midday/ui/icons";
import { Loader2, Printer } from "lucide-react";
import { ScrollArea } from "@midday/ui/scroll-area";
import { useToast } from "@midday/ui/use-toast";
import { generateCanvasPdf } from "@midday/export";
import { cn } from "@midday/ui/cn";

// ==========================================
// Month Filtering Logic:
//   monthly → show only months 1,2,3 (term1) or 5,6,7 (term2) — not exam months
//   half-yearly → no month selector needed (auto: months 1-4)
//   final → no month selector needed (auto: months 1-8)
// ==========================================

// Only allow selecting months that are for grade entry (not exam months)
const MONTHLY_MONTHS = HIJRI_MONTHS.filter(
  (m) => !m.isExam && !m.isVacation
);

// Paper dimensions for rendering and print CSS
const PAPER_DIMS = {
  a4: { width: 595, height: 842, mmWidth: 210, mmHeight: 297 },
  a5: { width: 420, height: 595, mmWidth: 148, mmHeight: 210 },
  letter: { width: 612, height: 792, mmWidth: 216, mmHeight: 279 },
} as const;

type PaperSizeKey = keyof typeof PAPER_DIMS;

export function StudentCertificateSheet() {
  const trpc = useTRPC();
  const { toast } = useToast();
  const { 
    studentId, 
    certificate, 
    certificateType, 
    templateId,
    month,
    setParams 
  } = useStudentParams();
  const t = useI18n();
  const locale = useCurrentLocale();

  const selectedMonth = month || 1;
  const selectedType = certificateType || "monthly";

  const [isExporting, setIsExporting] = useState(false);

  const isOpen = Boolean(studentId && certificate);

  const { data: studentData, isLoading: isStudentLoading } = useQuery({
    ...trpc.students.getById.queryOptions(
      { id: studentId! },
    ),
    enabled: isOpen,
  });

  const { data: templates } = useQuery({
    ...trpc.certificateTemplates.list.queryOptions(),
    enabled: isOpen,
  });

  const currentTemplate = (templates as any)?.find((t: any) => t.id === templateId) || (templates as any)?.[0];
  
  const { data: template } = useQuery({
    ...trpc.certificateTemplates.getById.queryOptions(
      { id: currentTemplate?.id || "" },
    ),
    enabled: isOpen && !!currentTemplate?.id,
  });

  const rankingPeriod = selectedType === "half-yearly" ? "half-yearly" : selectedType === "final" ? "final" : "monthly";
  const rankingMonth = rankingPeriod === "monthly" ? selectedMonth : undefined;
  const studentRecord = studentData as any;

  const { data: topStudents } = useQuery({
    ...trpc.grades.getTopStudents.queryOptions(
      {
        classId: studentRecord?.currentClass?.id || 0,
        academicYearId: studentRecord?.currentClass?.academicYearId || 0,
        period: rankingPeriod,
        month: rankingMonth,
        limit: 10,
      },
      {
        enabled:
          isOpen &&
          !!studentRecord?.currentClass?.id &&
          !!studentRecord?.currentClass?.academicYearId &&
          (rankingPeriod !== "monthly" || !!rankingMonth),
      },
    ),
  });

  const handleClose = () => {
    setParams({ 
      studentId: null, 
      certificate: null, 
      certificateType: null,
      templateId: null,
      month: null 
    });
  };

  const handleDownload = async () => {
    if (!studentData || !template) return;
    setIsExporting(true);
    try {
      const typeLabel = certificateType === "half-yearly" 
        ? t("reports.half_yearly")
        : certificateType === "final" 
          ? t("reports.final")
          : t("reports.monthly");
      // Professional PDF generation using higher scale and specific capture area
      // Let the off-screen container render fully (fonts, images, etc.)
      await new Promise((resolve) => setTimeout(resolve, 500));

      // University Grade PDF generation: capture from a hidden, non-scaled container
      await generateCanvasPdf({
        filename: `${t("reports.certificate")}_${studentData.name}_${typeLabel}.pdf`,
        selector: "#perfect-pdf-capture-root [data-canvas-content]",
        paperSize: (template as any).size || "a4",
        orientation: (template as any).orientation || "portrait",
        scale: 4, // Maximum resolution
      });
      toast({ title: t("reports.export_success"), variant: "success" });
    } catch (error) {
      console.error("PDF Export Error:", error);
      toast({ title: t("reports.export_failed"), variant: "destructive" });
    } finally {
      setIsExporting(false);
    }
  };

  const studentDataValues = React.useMemo(() => {
    if (!studentRecord) return undefined;
    const stats = (studentRecord.stats as any) || {};
    const currentStudentRank = (topStudents || []).find((item: any) => item.studentId === studentRecord.userId);
    return {
      studentId: studentRecord.userId,
      student_name: studentRecord.name || "",
      seat_number: studentRecord.studentNumber || "",
      secret_number: studentRecord.secretNumber || "",
      student_id: studentRecord.academicId || studentRecord.studentNumber || "",
      department: stats.department || "",
      section: stats.section || "",
      class_name: studentRecord.currentClass?.name || "",
      total_grade: stats.total_grade || "0",
      total_grade_text: stats.total_grade_text || "",
      grade_label: stats.grade_label || "",
      final_result: stats.final_result || "",
      rank: currentStudentRank?.rankLabel || stats.rank || "",
      certificate_type: selectedType === "half-yearly" ? "النصفي" : (selectedType === "final" ? "النهائي" : "الشهري"),
      org_name: "مدرسة صنعاء النموذجية",
      admin_name: stats.admin_name || t("reports.preview.admin_name"),
      vp_name: stats.vp_name || t("reports.preview.vp_name"),
      issue_date: new Date().toLocaleDateString(locale === 'ar' ? 'ar-YE' : 'en-US', { year: 'numeric', month: 'long', day: 'numeric' }),
      report_type: selectedType === "half-yearly" ? "النصفي" : (selectedType === "final" ? "النهائي" : `الشهري - لشهر ${selectedMonth ? (HIJRI_MONTHS.find(m => m.id === selectedMonth)?.name || t("reports.month_label", { month: selectedMonth })) : ""}`),
      school_year: studentRecord.currentClass?.yearName || "2024/2025",
    } as Record<string, string | number>;
  }, [studentRecord, topStudents, selectedType, selectedMonth, t, locale]);

  const certificateTypes = [
    { value: "monthly", label: t("reports.monthly_full"), description: t("reports.one_month") },
    { value: "half-yearly", label: t("reports.half_yearly_full"), description: t("reports.half_yearly_description") },
    { value: "final", label: t("reports.final_full"), description: t("reports.final_description") },
  ];

  // For monthly type: show only months that accept grade entry (not exam months)
  const availableMonths = MONTHLY_MONTHS.map((m) => ({ value: m.id, label: m.label, term: m.term }));

  // Determine if month selector should be visible
  const showMonthSelector = selectedType === "monthly";

  // Get paper dimensions for the current template
  const templateSize = ((template as any)?.size || 'a4') as PaperSizeKey;
  const templateOrientation = ((template as any)?.orientation || 'portrait') as string;
  const paperConfig = PAPER_DIMS[templateSize] || PAPER_DIMS.a4;
  const printWidthMm = templateOrientation === 'landscape' ? paperConfig.mmHeight : paperConfig.mmWidth;
  const printHeightMm = templateOrientation === 'landscape' ? paperConfig.mmWidth : paperConfig.mmHeight;

  // Print handler with dynamic page size
  const handlePrint = React.useCallback(() => {
    window.print();
  }, []);

  const handleShareToX = () => {
    if (!studentData) return;
    
    const baseUrl = typeof window !== 'undefined' ? window.location.origin : '';
    // Construction of a public-ish sharing URL if applicable, or just a dashboard link
    const sharingUrl = `${baseUrl}/${locale}/export/certificate?studentId=${studentId}&templateId=${currentTemplate?.id}&type=${selectedType}&month=${selectedMonth}`;
    
    const monthName = availableMonths.find(m => m.value === selectedMonth)?.label || "";
    
    const text = selectedType === "final" 
      ? t("reports.share_text_final", { name: studentData.name })
      : t("reports.share_text_monthly", { name: studentData.name, month: monthName });

    const xUrl = `https://x.com/intent/tweet?text=${encodeURIComponent(text)}&url=${encodeURIComponent(sharingUrl)}`;
    window.open(xUrl, '_blank');
  };

  return (
    <Sheet open={isOpen} onOpenChange={(open) => !open && handleClose()}>
      <SheetContent 
        style={{ maxWidth: 740 }} 
        className="p-0 bg-white dark:bg-[#080808] transition-[max-width] duration-300 ease-in-out [&>button]:hidden border-none"
      >
        <div className="relative h-full flex flex-col print-content">
          {/* University Grade Off-screen PDF Capture Container - ZERO SCALE issues */}
          <div 
            className="fixed left-0 top-0 pointer-events-none opacity-0 z-[-1000]"
            aria-hidden="true"
             id="perfect-pdf-capture-root"
          >
            {studentDataValues && template && (
                <CertificateViewer 
                  template={template as any} 
                  data={studentDataValues}
                  certificateType={selectedType}
                  month={selectedMonth}
                  preview={false}
                  className="shadow-none border-none m-0 p-0"
                />
            )}
          </div>

          <ScrollArea className="flex-1 p-6" hideScrollbar>
            <div className="p-8 pb-32 min-h-full flex flex-col items-center justify-center bg-[#fcfcfc] dark:bg-[#0f0f0f]">
              {(isStudentLoading || !template) ? (
                <div className="flex flex-col items-center gap-4 py-20">
                  <Loader2 className="w-8 h-8 animate-spin text-primary" />
                  <p className="text-sm text-muted-foreground font-medium">{t("reports.preparing")}</p>
                </div>
              ) : (
                <div className="w-full flex flex-col items-center" id="certificate-capture-area">
                  {(() => {
                    const canvasWidth = templateOrientation === 'landscape' ? paperConfig.height : paperConfig.width;
                    const canvasHeight = templateOrientation === 'landscape' ? paperConfig.width : paperConfig.height;
                    const targetWidth = 600;
                    const scale = Math.min(1, targetWidth / canvasWidth);
                    
                    return (
                      <div 
                        style={{ 
                          width: `${canvasWidth * scale}px`, 
                          height: `${canvasHeight * scale}px`,
                        }}
                        className="relative mb-10"
                      >
                        <div 
                          className="shadow-2xl transition-transform origin-top"
                          style={{
                            transform: `scale(${scale})`,
                            width: `${canvasWidth}px`,
                            height: `${canvasHeight}px`,
                            position: 'absolute',
                            left: '50%',
                            marginLeft: `-${canvasWidth / 2}px`
                          }}
                        >
                          <CertificateViewer 
                            template={template as any} 
                            data={studentDataValues}
                            certificateType={selectedType}
                            month={selectedMonth}
                          />
                        </div>
                      </div>
                    );
                  })()}
                </div>
              )}
            </div>
          </ScrollArea>

          <div 
            className="absolute bottom-4 w-full border-t border-border pt-4 px-6 bg-white dark:bg-[#080808] z-10 no-print"
          >
            <div className="flex justify-between items-center">
              <div className="flex gap-2">
                {/* Certificate Type Selector */}
                <DropdownMenu>
                  <DropdownMenuTrigger asChild>
                    <button
                      type="button"
                      className="h-9 px-3 flex items-center justify-center gap-2 border border-border hover:bg-accent transition-colors text-xs rounded-md"
                    >
                      <span className="max-w-[140px] truncate font-medium">
                        {certificateTypes.find(t => t.value === selectedType)?.label}
                      </span>
                      <Icons.ChevronDown className="size-4 text-[#666]" />
                    </button>
                  </DropdownMenuTrigger>
                  <DropdownMenuContent align="start" sideOffset={10} className="w-64">
                    <DropdownMenuGroup>
                      <DropdownMenuLabel className="text-[10px] text-muted-foreground font-normal px-2 py-1.5 uppercase">
                        {t("reports.type")}
                      </DropdownMenuLabel>
                      {certificateTypes.map((type) => (
                        <DropdownMenuCheckboxItem
                          key={type.value}
                          checked={selectedType === type.value}
                          onCheckedChange={() => setParams({ certificateType: type.value as any })}
                          className={cn(
                            "text-xs flex flex-col items-start gap-0.5 py-2",
                            selectedType === type.value
                              ? "dark:bg-[#131313] text-primary"
                              : "text-[#666]"
                          )}
                        >
                          <span className="font-medium">{type.label}</span>
                          <span className="text-[10px] text-muted-foreground">{type.description}</span>
                        </DropdownMenuCheckboxItem>
                      ))}
                    </DropdownMenuGroup>
                  </DropdownMenuContent>
                </DropdownMenu>

                {/* Month Selector - Only for monthly certificates */}
                {showMonthSelector && (
                  <DropdownMenu>
                    <DropdownMenuTrigger asChild>
                      <button
                        type="button"
                        className="h-9 px-3 flex items-center justify-center gap-2 border border-border hover:bg-accent transition-colors text-xs rounded-md"
                      >
                        <span className="max-w-[120px] truncate">
                          {availableMonths.find(m => m.value === selectedMonth)?.label || t("reports.month_label", { month: selectedMonth })}
                        </span>
                        <Icons.ChevronDown className="size-4 text-[#666]" />
                      </button>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent align="start" sideOffset={10} className="w-56">
                       <DropdownMenuGroup>
                        <DropdownMenuLabel className="text-[10px] text-muted-foreground font-normal px-2 py-1.5 uppercase">
                          {t("reports.first_term")}
                        </DropdownMenuLabel>
                        {availableMonths.filter(m => m.term === 1).map((m) => (
                          <DropdownMenuCheckboxItem
                            key={m.value}
                            checked={selectedMonth === m.value}
                            onCheckedChange={() => setParams({ month: m.value })}
                            className={cn(
                              "text-xs",
                              selectedMonth === m.value
                                ? "dark:bg-[#131313] text-primary"
                                : "text-[#666]"
                            )}
                          >
                            {m.label}
                          </DropdownMenuCheckboxItem>
                        ))}
                         <DropdownMenuSeparator />
                        <DropdownMenuLabel className="text-[10px] text-muted-foreground font-normal px-2 py-1.5 uppercase">
                          {t("reports.second_term")}
                        </DropdownMenuLabel>
                        {availableMonths.filter(m => m.term === 2).map((m) => (
                          <DropdownMenuCheckboxItem
                            key={m.value}
                            checked={selectedMonth === m.value}
                            onCheckedChange={() => setParams({ month: m.value })}
                            className={cn(
                              "text-xs",
                              selectedMonth === m.value
                                ? "dark:bg-[#131313] text-primary"
                                : "text-[#666]"
                            )}
                          >
                            {m.label}
                          </DropdownMenuCheckboxItem>
                        ))}
                      </DropdownMenuGroup>
                    </DropdownMenuContent>
                  </DropdownMenu>
                )}

                {/* Template Selector */}
                <DropdownMenu>
                  <DropdownMenuTrigger asChild>
                    <button
                      type="button"
                      className="h-9 px-3 flex items-center justify-center gap-2 border border-border hover:bg-accent transition-colors text-xs rounded-md"
                     >
                      <span className="max-w-[120px] truncate font-medium">
                        {currentTemplate?.name || t("reports.select_template_prompt")}
                      </span>
                      <Icons.ChevronDown className="size-4 text-[#666]" />
                    </button>
                  </DropdownMenuTrigger>
                  <DropdownMenuContent align="start" sideOffset={10} className="w-56">
                     <DropdownMenuGroup>
                      <DropdownMenuLabel className="text-[10px] text-muted-foreground font-normal px-2 py-1.5 uppercase">
                        {t("reports.templates")}
                      </DropdownMenuLabel>
                      {(templates as any)?.map((template: any) => (
                        <DropdownMenuCheckboxItem
                          key={template.id}
                          checked={currentTemplate?.id === template.id}
                          onCheckedChange={() => setParams({ templateId: template.id })}
                          className={cn(
                            "text-xs",
                            currentTemplate?.id === template.id
                              ? "dark:bg-[#131313] text-primary"
                              : "text-[#666]"
                          )}
                        >
                          <span className="flex items-center gap-1">
                             {template.name}
                            {template.isDefault && (
                              <span className="text-muted-foreground text-[10px]">
                                ({t("reports.default")})
                              </span>
                            )}
                          </span>
                        </DropdownMenuCheckboxItem>
                      ))}
                       {(!templates || (templates as any)?.length === 0) && (
                        <div className="px-2 py-1.5 text-xs text-muted-foreground">
                          {t("reports.no_templates")}
                        </div>
                      )}
                    </DropdownMenuGroup>
                  </DropdownMenuContent>
                </DropdownMenu>
              </div>

              <div className="flex gap-2">
                <Button variant="outline" size="icon" onClick={handlePrint} className="h-9 w-9">
                  <Printer className="size-4" />
                </Button>

                <Button 
                  onClick={handleDownload} 
                  disabled={isExporting}
                  className="h-9 text-xs"
                >
                  {isExporting ? (
                    <Loader2 className="w-4 h-4 ml-2 animate-spin" />
                  ) : (
                    <Icons.Pdf className="w-4 h-4 ml-2 rtl:mr-2 rtl:ml-0" />
                  )}
                  {t("reports.export_pdf")}
                </Button>
                
                <Button 
                  variant="outline" 
                  size="icon" 
                  onClick={handleClose}
                  className="h-9 w-9"
                >
                  <Icons.Close className="size-4" />
                </Button>
              </div>
            </div>
          </div>
        </div>
      </SheetContent>
    </Sheet>
  );
}
