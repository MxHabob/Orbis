"use client";

import { useQueryState, parseAsInteger } from "nuqs";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@midday/ui/select";
import { Label } from "@midday/ui/label";

interface GradeGridHeaderProps {
  initialData: {
    classes: any[];
    subjects: any[];
    terms: any[];
    years: any[];
  };
}

const months = [
    { id: 9, name: "September" },
    { id: 10, name: "October" },
    { id: 11, name: "November" },
    { id: 12, name: "December" },
    { id: 1, name: "January" },
    { id: 2, name: "February" },
    { id: 3, name: "March" },
    { id: 4, name: "April" },
    { id: 5, name: "May" },
];

export function GradeGridHeader({ initialData }: GradeGridHeaderProps) {
  const [classId, setClassId] = useQueryState("classId", parseAsInteger);
  const [subjectId, setSubjectId] = useQueryState("subjectId", parseAsInteger);
  const [termId, setTermId] = useQueryState("termId", parseAsInteger);
  const [month, setMonth] = useQueryState("month", parseAsInteger);
  const [yearId, setYearId] = useQueryState("yearId", parseAsInteger);

  return (
    <div className="flex flex-wrap items-center gap-6 p-6 border rounded-xl bg-background/50 backdrop-blur-sm shadow-sm">
      <div className="flex flex-col gap-2 min-w-[140px]">
        <Label className="text-[10px] uppercase tracking-wider text-muted-foreground font-bold">Academic Year</Label>
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
      </div>

      <div className="flex flex-col gap-2 min-w-[160px]">
        <Label className="text-[10px] uppercase tracking-wider text-muted-foreground font-bold">Class</Label>
        <Select
          value={classId?.toString()}
          onValueChange={(val) => setClassId(parseInt(val))}
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
      </div>

      <div className="flex flex-col gap-2 min-w-[160px]">
        <Label className="text-[10px] uppercase tracking-wider text-muted-foreground font-bold">Subject</Label>
        <Select
          value={subjectId?.toString()}
          onValueChange={(val) => setSubjectId(parseInt(val))}
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
      </div>

      <div className="flex flex-col gap-2 min-w-[140px]">
        <Label className="text-[10px] uppercase tracking-wider text-muted-foreground font-bold">Term</Label>
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
      </div>

      <div className="flex flex-col gap-2 min-w-[130px]">
        <Label className="text-[10px] uppercase tracking-wider text-muted-foreground font-bold">Month</Label>
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
    </div>
  );
}
