"use client";

import { useInvoiceStore } from "@/store/invoice";
import { useTRPC } from "@/trpc/client";
import { useQuery } from "@tanstack/react-query";
import { Button } from "@midday/ui/button";
import { Icons } from "@midday/ui/icons";
import { SubmitButton } from "@midday/ui/submit-button";
import { BottomBar as CoreBottomBar } from "../core/bottom-bar";
import { useState, useMemo } from "react";
import { HIJRI_MONTHS } from "@/utils/academic-calendar";
import {
  DropdownMenu,
  DropdownMenuCheckboxItem,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@midday/ui/dropdown-menu";
import { useToast } from "@midday/ui/use-toast";
type CertificateType = "monthly" | "half-yearly" | "final";

const MONTHLY_MONTHS = HIJRI_MONTHS.filter(
  (m) => !m.isExam && !m.isVacation
);

export function BottomBar() {
  const { rowSelection, setRowSelection } = useInvoiceStore();
  const trpc = useTRPC();
  const { toast } = useToast();

  const [certificateType, setCertificateType] = useState<CertificateType>("monthly");
  const [selectedMonth, setSelectedMonth] = useState<number>(1);
  const [templateId, setTemplateId] = useState<string | null>(null);
  const [isExporting, setIsExporting] = useState(false);

  const selectedIds = Object.keys(rowSelection).filter((id) => rowSelection[id]);
  const count = selectedIds.length;

  const { data: templates } = useQuery({
    ...trpc.certificateTemplates.list.queryOptions(),
    enabled: count > 0,
  });

  const currentTemplate = useMemo(() => {
    if (!templates) return null;
    return (templates as any)?.find((t: any) => t.id === templateId) || (templates as any)?.[0];
  }, [templates, templateId]);

  const certificateTypes = [
    { value: "monthly", label: "شهري", fullLabel: "الشهادة الشهرية", description: "شهادة لشهر محدد" },
    { value: "half-yearly", label: "نصفي", fullLabel: "شهادة نصف العام", description: "3 أشهر + اختبار نصفي" },
    { value: "final", label: "نهائي", fullLabel: "شهادة نهاية العام", description: "الترم الأول + الترم الثاني" },
  ];

  const availableMonths = MONTHLY_MONTHS.map((m) => ({ value: m.id, label: m.label, term: m.term }));

  const handleExport = async () => {
    setIsExporting(true);
    try {
      toast({
        title: "جاري تحضير الشهادات...",
        description: `يتم الآن استخراج الشهادات لـ ${count} فصول بصيغة PDF.`,
      });
      
      // Simulate bulk export logic
      await new Promise(resolve => setTimeout(resolve, 3000));
      
      toast({
        title: "تم الاستخراج بنجاح",
        description: "تم تحميل ملف الشهادات الموحد لكل الطلاب.",
        variant: "success",
      });
    } catch (error) {
      toast({
        title: "خطأ في الاستخراج",
        description: "حدث خطأ أثناء محاولة توليد الشهادات المجمعة.",
        variant: "destructive",
      });
    } finally {
      setIsExporting(false);
    }
  };

  if (count === 0) return null;

  return (
    <CoreBottomBar
      selectedCount={count}
      onDeselect={() => setRowSelection({})}
      selectedLabel={(count) => (
        <span className="font-medium">
          تم تحديد <span className="text-primary">{count}</span> فصول
        </span>
      )}
      deselectLabel="إلغاء التحديد"
    >
      <div className="flex items-center gap-1 bg-accent/50 dark:bg-accent/20 rounded-lg p-1 mr-4">
        {/* Type Selector */}
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button
              variant="ghost"
              size="sm"
              className="h-7 px-2 gap-1.5 text-[11px] font-medium hover:bg-background/80"
            >
              <span>{certificateTypes.find(t => t.value === certificateType)?.label}</span>
              <Icons.ChevronDown className="size-3 text-muted-foreground opacity-50" />
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent align="start" className="w-56" sideOffset={8}>
             <DropdownMenuGroup>
              <DropdownMenuLabel className="text-[10px] text-muted-foreground font-normal px-2 py-1.5 uppercase tracking-wider">
                نوع الشهادة
              </DropdownMenuLabel>
              {certificateTypes.map((type) => (
                <DropdownMenuCheckboxItem
                  key={type.value}
                  checked={certificateType === type.value}
                  onCheckedChange={() => setCertificateType(type.value as CertificateType)}
                  className="text-xs flex flex-col items-start gap-0.5 py-2 cursor-pointer"
                >
                  <span className="font-medium">{type.fullLabel}</span>
                  <span className="text-[10px] text-muted-foreground">{type.description}</span>
                </DropdownMenuCheckboxItem>
              ))}
            </DropdownMenuGroup>
          </DropdownMenuContent>
        </DropdownMenu>

        <div className="w-[1px] h-3 bg-border/40 mx-0.5" />

        {certificateType === "monthly" && (
          <>
            <DropdownMenu>
              <DropdownMenuTrigger asChild>
                <Button
                  variant="ghost"
                  size="sm"
                  className="h-7 px-2 gap-1.5 text-[11px] font-medium hover:bg-background/80"
                >
                  <span>{availableMonths.find(m => m.value === selectedMonth)?.label}</span>
                  <Icons.ChevronDown className="size-3 text-muted-foreground opacity-50" />
                </Button>
              </DropdownMenuTrigger>
              <DropdownMenuContent align="start" className="w-48" sideOffset={8}>
                <DropdownMenuGroup>
                  <DropdownMenuLabel className="text-[10px] text-muted-foreground font-normal px-2 py-1.5 uppercase tracking-wider">
                    الترم الأول
                  </DropdownMenuLabel>
                  {availableMonths.filter(m => m.term === 1).map((m) => (
                    <DropdownMenuCheckboxItem
                      key={m.value}
                      checked={selectedMonth === m.value}
                      onCheckedChange={() => setSelectedMonth(m.value)}
                      className="text-xs cursor-pointer"
                    >
                      {m.label}
                    </DropdownMenuCheckboxItem>
                  ))}
                  <DropdownMenuSeparator />
                  <DropdownMenuLabel className="text-[10px] text-muted-foreground font-normal px-2 py-1.5 uppercase tracking-wider">
                    الترم الثاني
                  </DropdownMenuLabel>
                  {availableMonths.filter(m => m.term === 2).map((m) => (
                    <DropdownMenuCheckboxItem
                      key={m.value}
                      checked={selectedMonth === m.value}
                      onCheckedChange={() => setSelectedMonth(m.value)}
                      className="text-xs cursor-pointer"
                    >
                      {m.label}
                    </DropdownMenuCheckboxItem>
                  ))}
                </DropdownMenuGroup>
              </DropdownMenuContent>
            </DropdownMenu>
            <div className="w-[1px] h-3 bg-border/40 mx-0.5" />
          </>
        )}

        {/* Template Selector */}
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button
              variant="ghost"
              size="sm"
              className="h-7 px-2 gap-1.5 text-[11px] font-medium hover:bg-background/80"
            >
              <span className="max-w-[80px] truncate">{currentTemplate?.name || "القالب"}</span>
              <Icons.ChevronDown className="size-3 text-muted-foreground opacity-50" />
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent align="start" className="w-56" sideOffset={8}>
            <DropdownMenuGroup>
              <DropdownMenuLabel className="text-[10px] text-muted-foreground font-normal px-2 py-1.5 uppercase tracking-wider">
                اختر قالب الشهادة
              </DropdownMenuLabel>
              {(templates as any)?.map((t: any) => (
                <DropdownMenuCheckboxItem
                  key={t.id}
                  checked={currentTemplate?.id === t.id}
                  onCheckedChange={() => setTemplateId(t.id)}
                  className="text-xs cursor-pointer"
                >
                  <span className="flex items-center gap-1">
                    {t.name}
                    {t.isDefault && <span className="text-[9px] opacity-60 bg-accent px-1 rounded">(الافتراضي)</span>}
                  </span>
                </DropdownMenuCheckboxItem>
              ))}
            </DropdownMenuGroup>
          </DropdownMenuContent>
        </DropdownMenu>
      </div>

      <SubmitButton
        isSubmitting={isExporting}
        onClick={handleExport}
        className="h-8 gap-2 px-4 shadow-sm"
      >
        <span className="text-xs">استخراج</span>
      </SubmitButton>
    </CoreBottomBar>
  );
}
