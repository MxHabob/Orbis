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
  Pencil,
  Eye,
  User,
  Calendar,
  GraduationCap,
  ChevronLeft,
  ChevronRight,
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
    <div className="h-14 flex items-center justify-between px-4 bg-background/95 backdrop-blur-sm shrink-0">
      <div className="flex items-center gap-3 overflow-x-auto no-scrollbar py-2">
        {/* Paper info badge */}
        <div className="flex items-center gap-1.5 text-[10px] text-muted-foreground whitespace-nowrap bg-accent/50 px-2.5 py-1 rounded-lg border border-border/30">
          <Maximize className="w-3 h-3" />
          <span className="font-medium">{paperSize.toUpperCase()}</span>
          <span className="text-muted-foreground/50">·</span>
          <span>{orientation === 'portrait' ? t("reports.editor.portrait") : t("reports.editor.landscape")}</span>
        </div>

        {/* Preview-only controls */}
        {isPreview && (
          <div className="flex items-center gap-1.5 pr-3 ml-1" dir="ltr">
            {/* Student pagination controls */}
            <div className="flex items-center gap-0.5 bg-accent/40 rounded-lg p-0.5 border border-border/20">
              <Button 
                variant="ghost" 
                size="icon" 
                className="h-6 w-6 rounded-md" 
                onClick={() => {
                  const currentIndex = students?.findIndex((s: any) => s.userId === previewStudentId);
                  if (currentIndex > 0) setPreviewStudentId(students[currentIndex - 1].userId);
                }}
                disabled={!students || students.findIndex((s: any) => s.userId === (previewStudentId || students?.[0]?.userId)) <= 0}
              >
                <ChevronLeft className="w-3 h-3" />
              </Button>

              <Select dir="ltr" value={previewStudentId || students?.[0]?.userId || ""} onValueChange={setPreviewStudentId}>
                <SelectTrigger className="h-6 w-[140px] text-[10px] border-0 shadow-none focus:ring-0 bg-transparent gap-1">
                  <User className="w-3 h-3 text-muted-foreground shrink-0" />
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

              <Button 
                variant="ghost" 
                size="icon" 
                className="h-6 w-6 rounded-md" 
                onClick={() => {
                  const currentIndex = students?.findIndex((s: any) => s.userId === previewStudentId);
                  if (currentIndex < students.length - 1) setPreviewStudentId(students[currentIndex + 1].userId);
                }}
                disabled={!students || students.findIndex((s: any) => s.userId === (previewStudentId || students?.[0]?.userId)) >= students.length - 1}
              >
                <ChevronRight className="w-3 h-3" />
              </Button>
            </div>

            {/* Type selector */}
            <Select dir="ltr" value={selectedType} onValueChange={setSelectedType}>
              <SelectTrigger className="h-7 w-[100px] text-[10px] bg-accent/40 shadow-none focus:ring-0 rounded-lg gap-1">
                <GraduationCap className="w-3 h-3 text-muted-foreground shrink-0" />
                <SelectValue placeholder={t("reports.type")} />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="monthly" className="text-xs">{t("reports.monthly")}</SelectItem>
                <SelectItem value="half-yearly" className="text-xs">{t("reports.half_yearly")}</SelectItem>
                <SelectItem value="final" className="text-xs">{t("reports.final")}</SelectItem>
              </SelectContent>
            </Select>

            {/* Month selector */}
            {selectedType === "monthly" && (
              <Select dir="ltr" value={selectedMonth.toString()} onValueChange={(v) => setSelectedMonth(parseInt(v))}>
                <SelectTrigger className="h-7 w-[100px] text-[10px] bg-accent/40 shadow-none focus:ring-0 rounded-lg gap-1">
                  <Calendar className="w-3 h-3 text-muted-foreground shrink-0" />
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

            {/* Year selector */}
            <Select dir="ltr" value={selectedYear} onValueChange={setSelectedYear}>
              <SelectTrigger className="h-7 w-[100px] text-[10px] bg-accent/40 shadow-none focus:ring-0 rounded-lg gap-1">
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

      {/* Mode Toggle */}
      <div className="flex items-center gap-1 bg-accent/50 p-0.5 rounded-xl">
        <Button 
          variant="ghost"
          size="sm"
          onClick={() => setIsPreview(false)}
          className={`h-7 text-xs rounded-lg gap-1.5 px-3 transition-all ${!isPreview ? 'bg-background shadow-sm text-foreground' : 'text-muted-foreground hover:text-foreground'}`}
        >
          <Pencil className="w-3 h-3" />
          {t("common.edit")}
        </Button>
        <Button 
          variant="ghost"
          size="sm"
          onClick={() => setIsPreview(true)}
          className={`h-7 text-xs rounded-lg gap-1.5 px-3 transition-all ${isPreview ? 'bg-background shadow-sm text-foreground' : 'text-muted-foreground hover:text-foreground'}`}
        >
          <Eye className="w-3 h-3" />
          {t("common.preview")}
        </Button>
      </div>
    </div>
  );
});

Toolbar.displayName = "Toolbar";
