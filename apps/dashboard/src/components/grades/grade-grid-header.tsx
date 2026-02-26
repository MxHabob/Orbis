"use client";

import { useQueryState, parseAsInteger } from "nuqs";
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
  const [selectedClassId, setSelectedClassId] = useQueryState("selectedClassId", parseAsInteger);
  const [selectedSubjectId, setSelectedSubjectId] = useQueryState("selectedSubjectId", parseAsInteger);
  const [termId, setTermId] = useQueryState("termId", parseAsInteger);
  const [month, setMonth] = useQueryState("month", parseAsInteger);
  const [yearId, setYearId] = useQueryState("yearId", parseAsInteger);
  const t = useI18n();

  useEffect(() => {
    if (defaults) {
      if (!yearId && defaults.yearId) setYearId(defaults.yearId);
      if (!termId && defaults.termId) setTermId(defaults.termId);
      if (!month && defaults.month) setMonth(defaults.month);
    }
  }, [defaults, yearId, termId, month, setYearId, setTermId, setMonth]);

  return (
    <div className="flex flex-wrap items-center py-4 justify-between">
      <div className="flex gap-6">
        <GradesSearchFilter/>
        <Select
          value={yearId?.toString()}
          onValueChange={(val) => setYearId(parseInt(val))}
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
          value={selectedClassId?.toString()}
          onValueChange={(val) => setSelectedClassId(parseInt(val))}
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
          value={selectedSubjectId?.toString()}
          onValueChange={(val) => setSelectedSubjectId(parseInt(val))}
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
          value={termId?.toString()}
          onValueChange={(val) => setTermId(parseInt(val))}
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
          value={month?.toString()}
          onValueChange={(val) => setMonth(parseInt(val))}
        >
          <SelectTrigger className="h-9 bg-background">
            <SelectValue placeholder={t("grades.select_month")} />
          </SelectTrigger>
          <SelectContent>
            {months.map((m) => (
              <SelectItem key={m.id} value={m.id.toString()}>
                {m.name}
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