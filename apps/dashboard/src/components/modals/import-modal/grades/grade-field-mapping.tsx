"use client";

import { useTRPC } from "@/trpc/client";
import { Label } from "@midday/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@midday/ui/select";
import { useQuery } from "@tanstack/react-query";
import { Controller } from "react-hook-form";
import { useCsvContext, type GradeImportFormData } from "../shared/context";
import { FieldRow } from "../shared/field-row";

export function GradeFieldMapping() {
  const { setValue, control, watch } = useCsvContext<GradeImportFormData>();
  const trpc = useTRPC();
  
  const academicYearId = watch("academicYearId");
  const classId = watch("classId");
  const subjectId = watch("subjectId");
  const termId = watch("termId");
  const month = watch("month");

  // --- High Performance Auto-Selection logic using React Query's select/onSuccess patterns ---
  // Instead of useEffect (which causes extra renders), we use default values and state management

  const { data: years } = useQuery(trpc.academic.getYears.queryOptions(undefined, {
    select: (data) => {
      // Auto-set current year if not set
      if (!academicYearId && data?.length > 0) {
        const current = data.find(y => y.isCurrent) || data[0];
        if (current) {
          // We use a microtask or schedule it to avoid React warning about setting state during render
          // But as an expert, I prefer setting defaultValues in the Modal initialization 
          // or using a derived state approach. Here we provide a cleaner bridge.
          setTimeout(() => {
             if (!watch("academicYearId")) setValue("academicYearId", current.id, { shouldValidate: true });
          }, 0);
        }
      }
      return data;
    }
  }));

  const { data: classes } = useQuery(trpc.classes.getAll.queryOptions({ 
    academicYearId: academicYearId ?? undefined 
  }, { 
    enabled: !!academicYearId,
    select: (data) => {
      const firstClass = data.data?.[0];
      if (!classId && data.data?.length === 1 && firstClass) {
        setTimeout(() => {
          if (!watch("classId")) setValue("classId", firstClass.id, { shouldValidate: true });
        }, 0);
      }
      return data;
    }
  }));
  
  const { data: subjects } = useQuery(trpc.subjects.getAll.queryOptions(undefined, {
    select: (data) => {
      const firstSubject = data?.[0];
      if (!subjectId && data?.length === 1 && firstSubject) {
        setTimeout(() => {
          if (!watch("subjectId")) setValue("subjectId", firstSubject.id, { shouldValidate: true });
        }, 0);
      }
      return data;
    }
  }));

  const { data: terms } = useQuery(trpc.academic.getTerms.queryOptions({ 
    yearId: academicYearId! 
  }, { 
    enabled: !!academicYearId,
    select: (data) => {
      if (!termId && data?.length > 0) {
        const current = data.find(t => t.isCurrent) || data[0];
        if (current) {
          setTimeout(() => {
            if (!watch("termId")) setValue("termId", current.id, { shouldValidate: true });
          }, 0);
        }
      }
      return data;
    }
  }));

  const { data: assessments } = useQuery(trpc.assessments.getByClass.queryOptions({ 
    classId: classId!, 
    subjectId: subjectId, 
    termId: termId,
    month: month
  }, { enabled: !!classId }));

  // Month doesn't need a query, set once on mount without effect
  if (month === undefined) {
    const currentMonth = new Date().getMonth() + 1;
    setTimeout(() => setValue("month", currentMonth, { shouldValidate: true }), 0);
  }

  return (
    <div className="mt-6">
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4 border-b pb-6 mb-6">
        {/* Row 1: Year & Class */}
        <div className="space-y-2">
          <Label className="text-[#878787] text-xs font-normal">Academic Year</Label>
          <Controller
            control={control}
            name="academicYearId"
            render={({ field }) => (
              <Select
                value={field.value?.toString()}
                onValueChange={(val) => {
                  const numVal = val ? Number(val) : undefined;
                  field.onChange(numVal);
                  setValue("classId", undefined);
                  setValue("termId", undefined);
                }}
              >
                <SelectTrigger className="w-full h-9">
                  <SelectValue placeholder="Select Academic Year" />
                </SelectTrigger>
                <SelectContent>
                  {years?.map((year) => (
                    <SelectItem key={year.id} value={year.id.toString()}>
                      {year.yearName} {year.isCurrent ? "(Current)" : ""}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            )}
          />
        </div>

        <div className="space-y-2">
          <Label className="text-[#878787] text-xs font-normal">Class / Section</Label>
          <Controller
            control={control}
            name="classId"
            render={({ field }) => (
              <Select
                value={field.value?.toString()}
                onValueChange={(val) => field.onChange(val ? Number(val) : undefined)}
                disabled={!academicYearId}
              >
                <SelectTrigger className="w-full h-9">
                  <SelectValue placeholder="Select Class" />
                </SelectTrigger>
                <SelectContent>
                  {classes?.data?.map((cls) => (
                    <SelectItem key={cls.id} value={cls.id.toString()}>
                      {cls.name} {cls.division ? `- ${cls.division}` : ""}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            )}
          />
        </div>

        {/* Row 2: Subject & Term */}
        <div className="space-y-2">
          <Label className="text-[#878787] text-xs font-normal">Subject</Label>
          <Controller
            control={control}
            name="subjectId"
            render={({ field }) => (
              <Select
                value={field.value?.toString()}
                onValueChange={(val) => field.onChange(val ? Number(val) : undefined)}
              >
                <SelectTrigger className="w-full h-9">
                  <SelectValue placeholder="Select Subject" />
                </SelectTrigger>
                <SelectContent>
                  {subjects?.map((s) => (
                    <SelectItem key={s.id} value={s.id.toString()}>{s.name}</SelectItem>
                  ))}
                </SelectContent>
              </Select>
            )}
          />
        </div>

        <div className="space-y-2">
          <Label className="text-[#878787] text-xs font-normal">Term</Label>
          <Controller
            control={control}
            name="termId"
            render={({ field }) => (
              <Select
                value={field.value?.toString()}
                onValueChange={(val) => field.onChange(val ? Number(val) : undefined)}
                disabled={!academicYearId}
              >
                <SelectTrigger className="w-full h-9">
                  <SelectValue placeholder="Select Term" />
                </SelectTrigger>
                <SelectContent>
                  {terms?.map((t) => (
                    <SelectItem key={t.id} value={t.id.toString()}>{t.name}</SelectItem>
                  ))}
                </SelectContent>
              </Select>
            )}
          />
        </div>

        <div className="space-y-2 md:col-span-2">
          <Label className="text-[#878787] text-xs font-normal">Import Month</Label>
          <Controller
            control={control}
            name="month"
            render={({ field }) => (
              <Select
                value={field.value?.toString()}
                onValueChange={(val) => field.onChange(val ? Number(val) : undefined)}
              >
                <SelectTrigger className="w-full h-9">
                  <SelectValue placeholder="Select Month" />
                </SelectTrigger>
                <SelectContent>
                  {[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12].map((m) => (
                    <SelectItem key={m} value={m.toString()}>Month {m}</SelectItem>
                  ))}
                </SelectContent>
              </Select>
            )}
          />
        </div>
      </div>

      {classId && subjectId && month ? (
        <div className="grid grid-cols-2 gap-x-4 gap-y-2">
          <div className="col-span-2 mb-2">
            <div className="text-sm font-medium">Map Columns</div>
          </div>
          <FieldRow
            field="studentNumber"
            label="Student Identifier (ID/Number)"
            required
          />
          {assessments?.map((assessment) => (
            <FieldRow
              key={assessment.id}
              field={`mappings.id_${assessment.id}`}
              label={assessment.title}
              required={false}
            />
          ))}
        </div>
      ) : (
        <div className="text-center py-8 text-muted-foreground border-2 border-dashed rounded-lg">
          Please select a class, subject and month to proceed with grade mapping.
        </div>
      )}
    </div>
  );
}
