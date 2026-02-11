"use client";

import { useTRPC } from "@/trpc/client";
import { Icons } from "@midday/ui/icons";
import { Label } from "@midday/ui/label";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
} from "@midday/ui/select";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@midday/ui/tooltip";
import { useQuery } from "@tanstack/react-query";
import { Controller, useWatch } from "react-hook-form";
import { mappableFields, useCsvContext } from "./context";
import { Switch } from "@midday/ui/switch";

export function FieldMapping() {
  const { setValue, control, watch } = useCsvContext();
  const type = watch("type") || "students";
  const trpc = useTRPC();
  const autoGenerate = watch("autoGenerateStudentNumber");

  // Common Context
  const { data: years } = useQuery(trpc.academic.getYears.queryOptions());
  const academicYearId = watch("academicYearId");
  
  // Classes Context (Required for both)
  const { data: classes } = useQuery(trpc.classes.getAll.queryOptions({ 
    academicYearId: academicYearId ?? undefined 
  }, { enabled: !!academicYearId }));

  // Grades Specific Context
  const { data: subjects } = useQuery(trpc.subjects.getAll.queryOptions(undefined,{
    enabled: type === "grades"
  }));
  
  const { data: terms } = useQuery(trpc.academic.getTerms.queryOptions({ 
    yearId: academicYearId! 
  }, { enabled: type === "grades" && !!academicYearId }));

  const classId = watch("classId");
  const subjectId = watch("subjectId");
  const termId = watch("termId");
  const month = watch("month");

  const { data: assessments } = useQuery(trpc.assessments.getByClass.queryOptions({ 
    classId: classId!, 
    subjectId: subjectId, 
    termId: termId,
    month: month
  }, { enabled: type === "grades" && !!classId }));

  return (
    <div className="mt-6">
      <div className="grid grid-cols-2 gap-x-4 gap-y-2 mb-4">
        <div className="text-sm font-medium">CSV Data column</div>
        <div className="text-sm font-medium">System data column</div>
      </div>

      {/* 1. Shared Context Selectors (Year & Class) */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4 border-b pb-6 mb-6">
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
                  <SelectValue placeholder={academicYearId ? "Select Class" : "Select Year first"} />
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

        {type === "grades" && (
          <>
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
                      <SelectValue placeholder={academicYearId ? "Select Term" : "Select Year first"} />
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

            <div className="space-y-2">
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
          </>
        )}
      </div>

      {/* 2. Mapping Rows */}
      <div className="grid grid-cols-2 gap-x-4 gap-y-2">
        {type === "students" && (
          <>
            <div className="col-span-2 mb-4">
              <div className="flex items-center space-x-2">
                <Controller
                  control={control}
                  name="autoGenerateStudentNumber"
                  render={({ field }) => (
                    <Switch
                      checked={field.value}
                      onCheckedChange={field.onChange}
                    />
                  )}
                />
                <Label className="text-sm font-normal">Auto-generate student numbers</Label>
              </div>
            </div>

            {!autoGenerate && (
              <FieldRow
                field="studentNumber"
                type="students"
                required
              />
            )}
            <FieldRow field="name" type="students" required />
            <FieldRow field="phone" type="students" />
            <FieldRow field="email" type="students" />
            <FieldRow field="sex" type="students" />
          </>
        )}

        {type === "grades" && classId && month && (
          <>
            <FieldRow
              field="studentNumber"
              label="Student Number (Identifier)"
              required
            />
            {assessments?.map((assessment) => (
              <FieldRow
                key={assessment.id}
                field={`mappings.${assessment.id}`}
                label={assessment.title}
                required={false}
              />
            ))}
          </>
        )}
      </div>

      {type === "grades" && (!classId || !month) && (
        <div className="text-center py-8 text-muted-foreground border-2 border-dashed rounded-lg">
          Please select a class and month to proceed with grade mapping.
        </div>
      )}
    </div>
  );
}


function FieldRow({
  field,
  type,
  label: explicitLabel,
  required: explicitRequired,
}: {
  field: string;
  type?: keyof typeof mappableFields;
  label?: string;
  required?: boolean;
}) {
  const fields = type ? mappableFields[type] : null;
  const fieldConfig = fields ? (fields as any)[field] : null;
  
  const label = explicitLabel ?? fieldConfig?.label ?? field;
  const required = explicitRequired ?? fieldConfig?.required ?? false;

  const { control, fileColumns, firstRows } = useCsvContext();

  // Use useWatch for better reactivity when values change
  const value = useWatch({ control, name: field as any });

  const firstRow = firstRows?.at(0);
  const description = value && firstRow ? firstRow[value as string] : undefined;

  return (
    <>
      <div className="relative flex min-w-0 items-center gap-2">
        <Controller
          control={control}
          name={field as any}
          rules={{ required }}
          render={({ field: controllerField }) => {
            return (
              <Select
                value={(controllerField?.value as string) ?? undefined}
                onValueChange={controllerField.onChange}
              >
                <SelectTrigger className="w-full relative">
                  <SelectValue placeholder={`Select ${label}`} />
                </SelectTrigger>
                <SelectContent>
                  <SelectGroup>
                    <SelectLabel>{label}</SelectLabel>
                    {[
                      // Filter out empty columns
                      ...(fileColumns?.filter((column) => column !== "") || []),
                      ...(controllerField.value && !required ? ["None"] : []),
                    ]?.map((column) => {
                      return (
                        <SelectItem key={column} value={column}>
                          {column}
                        </SelectItem>
                      );
                    })}
                  </SelectGroup>
                </SelectContent>
              </Select>
            );
          }}
        />

        <div className="flex items-center justify-end">
          <Icons.ArrowRightAlt className="size-4 text-[#878787]" />
        </div>
      </div>

      <span className="flex h-9 w-full items-center justify-between whitespace-nowrap border border-border bg-transparent px-3 py-2 text-sm">
        <div className="grow whitespace-nowrap text-sm font-normal text-muted-foreground justify-between flex">
          <span>{label}</span>

          {description?.trim() && (
            <TooltipProvider delayDuration={50}>
              <Tooltip>
                <TooltipTrigger asChild>
                  <button type="button" className="cursor-help">
                    <Icons.Info />
                  </button>
                </TooltipTrigger>
                <TooltipContent className="p-2 text-xs">
                  {description}
                </TooltipContent>
              </Tooltip>
            </TooltipProvider>
          )}
        </div>
      </span>
    </>
  );
}
