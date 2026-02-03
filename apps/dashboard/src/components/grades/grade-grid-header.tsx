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
import { GradesColumnVisibility } from "./grades-column-visibility";
import { GradesSearchFilter } from "./grades-search-filter";

interface GradeGridHeaderProps {
  initialData: {
    classes: any[];
    subjects: any[];
    terms: any[];
    years: any[];
  };
}

const months = [
  { id: 9, name: "September (9)" },
  { id: 10, name: "October (10)" },
  { id: 11, name: "November (11)" },
  { id: 13, name: "Mid-term (النصفي)" },
  { id: 12, name: "December (12)" },
  { id: 1, name: "January (1)" },
  { id: 2, name: "February (2)" },
  { id: 14, name: "Final Exam (النهائي)" },
  { id: 3, name: "March (3)" },
  { id: 4, name: "April (4)" },
  { id: 5, name: "May (5)" },
];

export function GradeGridHeader({ initialData, defaults }: GradeGridHeaderProps & { defaults?: { yearId?: number; termId?: number; month?: number } }) {
  const [selectedClassId, setSelectedClassId] = useQueryState("selectedClassId", parseAsInteger);
  const [selectedSubjectId, setSelectedSubjectId] = useQueryState("selectedSubjectId", parseAsInteger);
  const [termId, setTermId] = useQueryState("termId", parseAsInteger);
  const [month, setMonth] = useQueryState("month", parseAsInteger);
  const [yearId, setYearId] = useQueryState("yearId", parseAsInteger);

  useEffect(() => {
    if (defaults) {
      if (!yearId && defaults.yearId) setYearId(defaults.yearId);
      if (!termId && defaults.termId) setTermId(defaults.termId);
      if (!month && defaults.month) setMonth(defaults.month);
    }
  }, [defaults, yearId, termId, month, setYearId, setTermId, setMonth]);

  return (
    <div className="flex flex-wrap items-center py-2 justify-between">
      <div className="flex gap-6">
        <GradesSearchFilter/>
        <Select
          value={yearId?.toString()}
          onValueChange={(val) => setYearId(parseInt(val))}
        >
          <SelectTrigger className="h-9 bg-background">
            <SelectValue placeholder="Select year" />
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
            <SelectValue placeholder="Select class" />
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
            <SelectValue placeholder="Select subject" />
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
            <SelectValue placeholder="Select term" />
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
            <SelectValue placeholder="Select month" />
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