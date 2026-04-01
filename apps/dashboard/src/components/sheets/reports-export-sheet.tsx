"use client";

import { useReportExportParams } from "@/hooks/use-report-export-params";
import { Button } from "@midday/ui/button";
import {
  Sheet,
  SheetContent,
  SheetDescription,
  SheetHeader,
  SheetTitle,
} from "@midday/ui/sheet";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@midday/ui/select";
import { useExportStore } from "@/store/export";
import { useTRPC } from "@/trpc/client";
import { useState } from "react";
import { useToast } from "@midday/ui/use-toast";
import { Loader2 } from "lucide-react";
import React from "react";
import { useQuery, useMutation } from "@tanstack/react-query";
import { useI18n } from "@/locales/client";
import { downloadBase64 } from "@midday/export";
import { HIJRI_MONTHS } from "@/utils/academic-calendar";

export function ReportsExportSheet() {
  const t = useI18n();
  const { exportReport, setParams } = useReportExportParams();
  const trpc = useTRPC();
  const { toast } = useToast();
  const { setExportData, setIsExporting } = useExportStore();
  
  const [classId, setClassId] = useState<string>("");
  const [reportType, setReportType] = useState<"monthly" | "half-yearly" | "final" | "attendance_sheet">("monthly");
  const [month, setMonth] = useState<string>("");

  const isOpen = Boolean(exportReport);

  const { data: classesResponse } = useQuery(
    trpc.classes.getAll.queryOptions({})
  );
  
  const exportMutation = useMutation(
    trpc.grades.exportClassReport.mutationOptions({
      onSuccess: (data) => {
        setExportData({ runId: data.jobId });
        setIsExporting(true);
        setParams({ exportReport: null });
      },
      onError: (err) => {
        toast({
          title: t("reports.export_excel_failed"),
          description: `${t("reports.export_excel_failed_desc")}${err.message}`,
          variant: "destructive",
        });
        setParams({ exportReport: null });
      },
    })
  );

  const handleExport = () => {
    if (!classId) return;

    exportMutation.mutate({
      classId: parseInt(classId),
      type: reportType,
      month: reportType === "monthly" ? parseInt(month) : undefined,
      monthText: reportType === "monthly" ? HIJRI_MONTHS.find(m => m.id === parseInt(month))?.label : undefined,
      format: "xlsx",
    });
  };

  return (
    <Sheet open={isOpen} onOpenChange={(open) => setParams({ exportReport: open ? true : null })}>
      <SheetContent className="flex flex-col" side="right" dir="auto">
        <SheetHeader className="text-start">
          <SheetTitle className="text-start">{t("reports.export_reports_title")}</SheetTitle>
          <SheetDescription className="text-start">
            {t("reports.export_reports_description")}
          </SheetDescription>
        </SheetHeader>
        
        <div className="flex flex-col gap-6 py-6 flex-1">
          <div className="flex flex-col gap-2">
            <label className="text-sm font-medium">{t("education.class_name")}</label>
            <Select value={classId} onValueChange={setClassId} dir="auto">
              <SelectTrigger>
                <SelectValue placeholder={t("grades.select_class")} />
              </SelectTrigger>
              <SelectContent dir="auto">
                {classesResponse?.data?.map((c: any) => (
                  <SelectItem key={c.id} value={c.id.toString()}>
                    {c.name}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>

          <div className="flex flex-col gap-2">
            <label className="text-sm font-medium">{t("reports.report_type")}</label>
            <Select value={reportType} onValueChange={(val: any) => setReportType(val)} dir="auto">
              <SelectTrigger>
                <SelectValue placeholder={t("reports.type")} />
              </SelectTrigger>
              <SelectContent dir="auto">
                <SelectItem value="monthly">{t("reports.monthly")}</SelectItem>
                <SelectItem value="half-yearly">{t("reports.half_yearly")}</SelectItem>
                <SelectItem value="final">{t("reports.final")}</SelectItem>
                <SelectItem value="attendance_sheet">{t("reports.attendance_sheet")}</SelectItem>
              </SelectContent>
            </Select>
          </div>

          {reportType === "monthly" && (
            <div className="flex flex-col gap-2">
              <label className="text-sm font-medium">{t("grades.select_month")}</label>
              <Select value={month} onValueChange={setMonth} dir="auto">
                <SelectTrigger>
                  <SelectValue placeholder={t("grades.select_month")} />
                </SelectTrigger>
                <SelectContent dir="auto">
                  {HIJRI_MONTHS.filter((m) => !m.isExam && !m.isVacation).map((m) => (
                    <SelectItem key={m.id} value={m.id.toString()}>
                      {m.label}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
          )}
        </div>

        <div className="mt-4 flex gap-2">
          <Button 
            onClick={handleExport} 
            disabled={!classId || (reportType === "monthly" && !month) || exportMutation.isPending}
            className="w-full mt-auto"
          >
            {exportMutation.isPending ? (
              <Loader2 className="mx-2 h-4 w-4 animate-spin" />
            ) : null}
            {t("reports.extract_report")}
          </Button>
        </div>
      </SheetContent>
    </Sheet>
  );
}
