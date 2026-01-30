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
  const { fileColumns, setValue, control, watch } = useCsvContext();
  const type = watch("type") || "students";
  const fields = mappableFields[type as keyof typeof mappableFields];
  const trpc = useTRPC();
  const autoGenerate = watch("autoGenerateStudentNumber");

  // Hooks must be at the top level
  const { data: years } = useQuery(trpc.academic.getYears.queryOptions());
  const academicYearId = watch("academicYearId");
  const { data: classes } = useQuery(trpc.classes.getAll.queryOptions({ 
    academicYearId: academicYearId ?? undefined 
  }, { enabled: !!academicYearId }));


  return (
    <div className="mt-6">
      <div className="grid grid-cols-2 gap-x-4 gap-y-2">
        <div className="text-sm">CSV Data column</div>
        <div className="text-sm">System data column</div>
        {(Object.keys(fields) as (keyof typeof fields)[])
          .filter(field => !(field === "studentNumber" && autoGenerate))
          .map(
          (field) => (
            <FieldRow
              key={field}
              field={field as any}
              type={type as any}
            />
          ),
        )}
      </div>

      {type === "students" && (
        <div className="mt-8 space-y-4 border-t pt-6">
           <div className="flex items-center justify-between py-2 border-b mb-2">
            <div className="space-y-0.5">
              <Label className="text-sm font-medium">Auto-generate student numbers</Label>
              <p className="text-[10px] text-muted-foreground">
                Unique numbers will be created automatically.
              </p>
            </div>
            <Controller
              control={control}
              name="autoGenerateStudentNumber"
              render={({ field: { value, onChange } }) => (
                <Switch
                  checked={value}
                  onCheckedChange={(val) => {
                    onChange(val);
                    if (val) setValue("studentNumber", undefined);
                  }}
                />
              )}
            />
          </div>

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
                    // Reset class when year changes
                    setValue("classId", undefined);
                  }}
                >
                  <SelectTrigger className="w-full">
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
                  <SelectTrigger className="w-full">
                    <SelectValue placeholder={academicYearId ? "Select Class" : "Select Academic Year first"} />
                  </SelectTrigger>
                  <SelectContent>
                    {classes?.map((cls) => (
                      <SelectItem key={cls.id} value={cls.id.toString()}>
                        {cls.name} {cls.division ? `- ${cls.division}` : ""}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              )}
            />
          </div>
        </div>
      )}

      {type === "grades" && (
        <div className="mt-6 space-y-4 border-t pt-6">
           {/* Additional fields for grades if needed */}
        </div>
      )}
    </div>
  );
}


function FieldRow({
  field,
  type,
}: {
  field: string;
  type: keyof typeof mappableFields;
}) {
  const fields = mappableFields[type];
  const { label, required } = (fields as any)[field];
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
