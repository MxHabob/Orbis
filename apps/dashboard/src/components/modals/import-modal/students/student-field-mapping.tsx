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
import { Switch } from "@midday/ui/switch";
import { useCsvContext } from "../shared/context";
import { FieldRow } from "../shared/field-row";

export function StudentFieldMapping() {
  const { setValue, control, watch } = useCsvContext();
  const trpc = useTRPC();
  const autoGenerate = watch("autoGenerateStudentNumber");

  const academicYearId = watch("academicYearId");
  const classId = watch("classId");

  // --- High Performance Auto-Selection logic using React Query's select pattern ---
  
  const { data: years } = useQuery(trpc.academic.getYears.queryOptions(undefined, {
    select: (data) => {
      if (!academicYearId && data.length > 0) {
        const current = data.find(y => y.isCurrent) || data[0];
        if (current) {
          setTimeout(() => {
            if (!watch("academicYearId")) setValue("academicYearId", current.id);
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
      if (!classId && data.data?.length === 1) {
        setTimeout(() => {
          if (!watch("classId")) setValue("classId", data.data[0].id);
        }, 0);
      }
      return data;
    }
  }));

  return (
    <div className="mt-6">
      <div className="grid grid-cols-2 gap-x-4 gap-y-2 mb-4">
        <div className="text-sm font-medium">CSV Data column</div>
        <div className="text-sm font-medium">System data column</div>
      </div>

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
      </div>

      <div className="grid grid-cols-2 gap-x-4 gap-y-2">
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
      </div>
    </div>
  );
}
