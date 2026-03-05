"use client";

import { memo } from "react";
import { Button } from "@midday/ui/button";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@midday/ui/select";
import {
  Maximize,
  Type,
  Eye,
  Table as TableIcon,
} from "lucide-react";

import { useI18n } from "@/locales/client";

import { HIJRI_MONTHS } from "@/utils/academic-calendar";
import type { PaperSize } from "./types";

interface ToolbarProps {
  paperSize: PaperSize;
  orientation: "portrait" | "landscape";
  isPreview: boolean;
  setIsPreview: (isPreview: boolean) => void;
  previewStudentId: string | null;
  setPreviewStudentId: (id: string) => void;
  students: any;
  selectedType: string;
  setSelectedType: (type: any) => void;
  selectedMonth: number;
  setSelectedMonth: (month: number) => void;
  selectedYear: string;
  setSelectedYear: (year: string) => void;
}

export const Toolbar = memo(({
  paperSize,
  orientation,
  isPreview,
  setIsPreview,
  previewStudentId,
  setPreviewStudentId,
  students,
  selectedType,
  setSelectedType,
  selectedMonth,
  setSelectedMonth,
  selectedYear,
  setSelectedYear,
}: ToolbarProps) => {
  const t = useI18n();
  return (
    <div className="h-16 border-b flex items-center justify-between px-6 bg-background shrink-0">
      <div className="flex items-center gap-4 overflow-x-auto no-scrollbar py-2">
        <div className="flex items-center gap-2 text-xs text-muted-foreground whitespace-nowrap">
          <Maximize className="w-3 h-3" />
          {paperSize.toUpperCase()} · {orientation === 'portrait' ? t("reports.editor.portrait") : t("reports.editor.landscape")}
        </div>

        {isPreview && (
          <div className="flex items-center gap-2 border-r pr-4 ml-2" dir="ltr">
            <Select dir="ltr" value={previewStudentId || students?.[0]?.userId || ""} onValueChange={setPreviewStudentId}>
              <SelectTrigger className="h-7 w-[140px] text-[10px] bg-accent/50 border-0 shadow-none focus:ring-0">
                <SelectValue placeholder={t("reports.preview.student_name")} />
              </SelectTrigger>
              <SelectContent className="max-h-[400px]">
                {students?.map((s: any) => (
                  <SelectItem key={s.userId} value={s.userId} className="text-xs">
                    {s.name || t("common.no_data")}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>

            <Select dir="ltr" value={selectedType} onValueChange={setSelectedType}>
              <SelectTrigger className="h-7 w-[100px] text-[10px] bg-accent/50 border-0 shadow-none focus:ring-0">
                <SelectValue placeholder={t("reports.type")} />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="monthly" className="text-xs">{t("reports.monthly")}</SelectItem>
                <SelectItem value="half-yearly" className="text-xs">{t("reports.half_yearly")}</SelectItem>
                <SelectItem value="final" className="text-xs">{t("reports.final")}</SelectItem>
              </SelectContent>
            </Select>

            {selectedType === "monthly" && (
              <Select dir="ltr" value={selectedMonth.toString()} onValueChange={(v) => setSelectedMonth(parseInt(v))}>
                <SelectTrigger className="h-7 w-[100px] text-[10px] bg-accent/50 border-0 shadow-none focus:ring-0">
                  <SelectValue placeholder={t("grades.select_month")} />
                </SelectTrigger>
                <SelectContent className="max-h-[300px]">
                  {HIJRI_MONTHS.map((m) => (
                    <SelectItem key={m.id} value={m.id.toString()} className="text-xs">
                      {m.name} ({m.id})
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            )}

            <Select dir="ltr" value={selectedYear} onValueChange={setSelectedYear}>
              <SelectTrigger className="h-7 w-[100px] text-[10px] bg-accent/50 border-0 shadow-none focus:ring-0">
                <SelectValue placeholder={t("reports.academic_year")} />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="2023/2024" className="text-xs">2023/2024</SelectItem>
                <SelectItem value="2024/2025" className="text-xs">2024/2025</SelectItem>
                <SelectItem value="2025/2026" className="text-xs">2025/2026</SelectItem>
              </SelectContent>
            </Select>
          </div>
        )}
      </div>

      <div className="flex items-center gap-2">
        <Button 
          variant={!isPreview ? "secondary" : "ghost"} 
          size="sm"
          onClick={() => setIsPreview(false)}
        >
          <Type className="w-4 h-4 mr-2" />
          {t("common.edit")}
        </Button>
        <Button 
          variant={isPreview ? "secondary" : "ghost"} 
          size="sm"
          onClick={() => setIsPreview(true)}
        >
          <Eye className="w-4 h-4 mr-2" />
          {t("common.preview")}
        </Button>
      </div>
    </div>
  );
});

Toolbar.displayName = "Toolbar";
