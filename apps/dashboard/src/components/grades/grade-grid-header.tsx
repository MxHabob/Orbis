"use client";

import { useEffect } from "react";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@midday/ui/select";
import { GradesActions } from "./grades-actions";
import { GradesSearchFilter } from "./grades-search-filter";
import { useI18n } from "@/locales/client";
import { useGradingStore } from "@/store/use-grading-store";
import { useGradeFilterParams } from "@/hooks/use-grade-filter-params";

interface GradeGridHeaderProps {
  initialData: {
    classes: any[];
    subjects: any[];
    terms: any[];
    years: any[];
  };
}

import { HIJRI_MONTHS } from "@/utils/academic-calendar";

const months = HIJRI_MONTHS.map(m => ({ id: m.id, name: m.label }));


export function GradeGridHeader({ initialData, defaults }: GradeGridHeaderProps & { defaults?: { yearId?: number; termId?: number; month?: number } }) {
  const { filter, setFilter } = useGradeFilterParams();
  const { selectedClassId, selectedSubjectId, termId, month, yearId } = filter;

  const t = useI18n();
  const { 
    yearId: stYear, 
    termId: stTerm, 
    month: stMonth, 
    classId: stClass, 
    subjectId: stSubject, 
    setFilters 
  } = useGradingStore();

  // Initialization: If URL is empty, load from Zustand (One-time sync)
  useEffect(() => {
    const updates: any = {};
    if (yearId === null && stYear) updates.yearId = stYear;
    if (termId === null && stTerm) updates.termId = stTerm;
    if (month === null && stMonth) updates.month = stMonth;
    if (selectedClassId === null && stClass) updates.selectedClassId = stClass;
    if (selectedSubjectId === null && stSubject) updates.selectedSubjectId = stSubject;
    
    // If everything is empty, use provided defaults
    if (yearId === null && !stYear && defaults?.yearId && updates.yearId === undefined) {
      updates.yearId = defaults.yearId;
    }

    if (Object.keys(updates).length > 0) {
      setFilter(updates);
    }
  }, []);

  // Sync Store with URL changes (Declarative & Centralized)
  const updateFilter = (updates: any) => {
    setFilters(updates);
  };

  return (
    <div className="flex flex-wrap items-center py-4 justify-between">
      <div className="flex gap-6">
        <GradesSearchFilter/>
        <Select
          value={yearId?.toString() || ""}
          onValueChange={(val) => {
            const id = parseInt(val);
            setFilter({ yearId: id });
            updateFilter({ yearId: id });
          }}
        >
          <SelectTrigger className="h-9 bg-background">
            <SelectValue placeholder={t("grades.select_year")} />
          </SelectTrigger>
          <SelectContent>
            {initialData.years.map((y) => (
              <SelectItem key={y.id} value={y.id.toString()}>
                {y.yearName}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>

        <Select
          value={selectedClassId?.toString() || ""}
          onValueChange={(val) => {
            const id = parseInt(val);
            setFilter({ selectedClassId: id });
            updateFilter({ classId: id });
          }}
        >
          <SelectTrigger className="h-9 bg-background">
            <SelectValue placeholder={t("grades.select_class")} />
          </SelectTrigger>
          <SelectContent>
            {initialData.classes.map((c) => (
              <SelectItem key={c.id} value={c.id.toString()}>
                {c.name} {c.division ? `- ${c.division}` : ""}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>

        <Select
          value={selectedSubjectId?.toString() || ""}
          onValueChange={(val) => {
            const id = parseInt(val);
            setFilter({ selectedSubjectId: id });
            updateFilter({ subjectId: id });
          }}
        >
          <SelectTrigger className="h-9 bg-background">
            <SelectValue placeholder={t("grades.select_subject")} />
          </SelectTrigger>
          <SelectContent>
            {initialData.subjects.map((s) => (
              <SelectItem key={s.id} value={s.id.toString()}>
                {s.name}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>

        <Select
          value={termId?.toString() || ""}
          onValueChange={(val) => {
            const newTermId = parseInt(val);
            
            // Auto-select first month of the new term
            const activeTerm = initialData.terms.find(t => t.id === newTermId);
            const newMonth = activeTerm?.name.includes("الاول") ? 1 : 5;
            
            setFilter({ termId: newTermId, month: newMonth });
            updateFilter({ termId: newTermId, month: newMonth });
          }}
        >
          <SelectTrigger className="h-9 bg-background">
            <SelectValue placeholder={t("grades.select_term")} />
          </SelectTrigger>
          <SelectContent>
            {initialData.terms
              .filter(t => !yearId || t.academicYearId === yearId)
              .map((t) => (
                <SelectItem key={t.id} value={t.id.toString()}>
                  {t.name}
                </SelectItem>
              ))}
          </SelectContent>
        </Select>

        <Select
          value={month?.toString() || ""}
          onValueChange={(val) => {
            const id = parseInt(val);
            setFilter({ month: id });
            updateFilter({ month: id });
          }}
        >
          <SelectTrigger className="h-9 bg-background">
            <SelectValue placeholder={t("grades.select_month")} />
          </SelectTrigger>
          <SelectContent>
            {months
              .filter(m => {
                if (!termId) return true;
                const activeTerm = initialData.terms.find(t => t.id === termId);
                const isFirstTerm = activeTerm?.name.includes("الاول");
                
                const monthData = HIJRI_MONTHS.find(hm => hm.id === m.id);
                return isFirstTerm ? monthData?.term === 1 : monthData?.term === 2;
              })
              .map((m) => (
                <SelectItem key={m.id} value={m.id.toString()}>
                  <div className="flex items-center justify-between w-full gap-2">
                    <span>{m.name}</span>
                    {HIJRI_MONTHS.find(hm => hm.id === m.id)?.isExam && (
                      <span className="text-[10px] bg-primary/10 text-primary px-1.5 py-0.5 rounded-full font-medium">
                        {m.id === 4 ? t("grades.midterm_exam") : t("grades.final_exam")}
                      </span>
                    )}
                  </div>
                </SelectItem>
              ))}
          </SelectContent>
        </Select>
      </div>
      <div className="flex gap-2">
        <GradesActions/>
      </div>
    </div>
  );
}