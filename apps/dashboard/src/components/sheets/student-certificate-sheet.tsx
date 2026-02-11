"use client";

import { useStudentParams } from "@/hooks/use-student-params";
import { Sheet, SheetContent } from "@midday/ui/sheet";
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
    if (!studentData) return;
    setIsExporting(true);
    try {
      const typeLabel = certificateType === "half-yearly" 
        ? "نصفي" 
        : certificateType === "final" 
          ? "نهائي" 
          : "شهري";
      await generateCanvasPdf({
         filename: `شهادة_${studentData.name}_${typeLabel}.pdf`,
        selector: "[data-canvas-content]"
      });
      toast({ title: "تم تصدير الشهادة بنجاح", variant: "success" });
    } catch (error) {
      toast({ title: "فشل تصدير الشهادة", variant: "destructive" });
    } finally {
      setIsExporting(false);
    }
  };

  const studentDataValues = React.useMemo(() => {
    if (!studentData) return undefined;
    const stats = (studentData.stats as any) || {};
    return {
      studentId: studentData.userId,
      student_name: studentData.name || "",
      academic_number: studentData.studentNumber || "",
      national_id: studentData.secretNumber || "",
      department: stats.department || "",
      section: stats.section || "",
      class_name: studentData.currentClass?.name || "",
      total_grade: stats.total_grade || "0",
      total_grade_text: stats.total_grade_text || "",
      grade_label: stats.grade_label || "",
      final_result: stats.final_result || "",
      rank: stats.rank || "1",
      org_name: "Sanaa Model School",
      admin_name: "School Principal",
      vp_name: "Registrar",
      issue_date: new Date().toLocaleDateString('en-US'),
    } as Record<string, string | number>;
  }, [studentData]);

  const certificateTypes = [
    { value: "monthly", label: "الشهادة الشهرية", description: "شهر واحد" },
    { value: "half-yearly", label: "شهادة نصف العام", description: "3 شهور + اختبار نصفي" },
    { value: "final", label: "شهادة نهاية العام", description: "الترم الأول + الترم الثاني" },
  ];

  // For monthly type: show only months that accept grade entry (not exam months)
  const availableMonths = MONTHLY_MONTHS.map((m) => ({ value: m.id, label: m.label, term: m.term }));

  const selectedMonth = month || 1;
  const selectedType = certificateType || "monthly";

  // Determine if month selector should be visible
  const showMonthSelector = selectedType === "monthly";

  return (
    <Sheet open={isOpen} onOpenChange={(open) => !open && handleClose()}>
      <SheetContent 
        style={{ maxWidth: 740 }} 
        className="p-0 bg-white dark:bg-[#080808] transition-[max-width] duration-300 ease-in-out [&>button]:hidden border-none"
      >
        <div className="relative h-full flex flex-col">
          <ScrollArea className="flex-1 p-6" hideScrollbar>
            <div className="p-8 pb-32 min-h-full flex flex-col items-center justify-center bg-[#fcfcfc] dark:bg-[#0f0f0f]">
              {(isStudentLoading || !template) ? (
                <div className="flex flex-col items-center gap-4 py-20">
                  <Loader2 className="w-8 h-8 animate-spin text-primary" />
                  <p className="text-sm text-muted-foreground font-medium">جاري تحضير الشهادة...</p>
                </div>
              ) : (
                <div className="w-full flex flex-col items-center">
                  {(() => {
                    const canvasWidth = template?.orientation === 'landscape' ? 1123 : 794;
                    const canvasHeight = template?.orientation === 'landscape' ? 794 : 1123;
                    const targetWidth = 600; // Fitting safely within the sheet padding
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

          <div className="absolute bottom-4 w-full border-t border-border pt-4 px-6 bg-white dark:bg-[#080808] z-10">
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
                        نوع الشهادة
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
                          {availableMonths.find(m => m.value === selectedMonth)?.label || `الشهر ${selectedMonth}`}
                        </span>
                        <Icons.ChevronDown className="size-4 text-[#666]" />
                      </button>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent align="start" sideOffset={10} className="w-56">
                      <DropdownMenuGroup>
                        <DropdownMenuLabel className="text-[10px] text-muted-foreground font-normal px-2 py-1.5 uppercase">
                          الترم الأول
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
                          الترم الثاني
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
                        {currentTemplate?.name || "اختر القالب"}
                      </span>
                      <Icons.ChevronDown className="size-4 text-[#666]" />
                    </button>
                  </DropdownMenuTrigger>
                  <DropdownMenuContent align="start" sideOffset={10} className="w-56">
                    <DropdownMenuGroup>
                      <DropdownMenuLabel className="text-[10px] text-muted-foreground font-normal px-2 py-1.5 uppercase">
                        القوالب
                      </DropdownMenuLabel>
                      {(templates as any)?.map((t: any) => (
                        <DropdownMenuCheckboxItem
                          key={t.id}
                          checked={currentTemplate?.id === t.id}
                          onCheckedChange={() => setParams({ templateId: t.id })}
                          className={cn(
                            "text-xs",
                            currentTemplate?.id === t.id
                              ? "dark:bg-[#131313] text-primary"
                              : "text-[#666]"
                          )}
                        >
                          <span className="flex items-center gap-1">
                            {t.name}
                            {t.isDefault && (
                              <span className="text-muted-foreground text-[10px]">
                                (افتراضي)
                              </span>
                            )}
                          </span>
                        </DropdownMenuCheckboxItem>
                      ))}
                      {(!templates || (templates as any)?.length === 0) && (
                        <div className="px-2 py-1.5 text-xs text-muted-foreground">
                          لا توجد قوالب متاحة
                        </div>
                      )}
                    </DropdownMenuGroup>
                  </DropdownMenuContent>
                </DropdownMenu>
              </div>

              <div className="flex gap-2">
                <Button variant="outline" size="icon" onClick={() => window.print()} className="h-9 w-9">
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
                    <Icons.Pdf className="w-4 h-4 ml-2" />
                  )}
                  تصدير PDF
                </Button>
                
                <Button 
                  variant="outline" 
                  size="icon" 
                  onClick={handleClose}
                  className="h-9 w-9"
                >
                  <Icons.X className="size-4" />
                </Button>
              </div>
            </div>
          </div>
        </div>
      </SheetContent>
    </Sheet>
  );
}
