import { Button } from "@orbis/ui/button";
import { Icons } from "@orbis/ui/icons";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import {
  Select as BasicSelect,
  SelectContent as BasicSelectContent,
  SelectItem as BasicSelectItem,
  SelectTrigger as BasicSelectTrigger,
  SelectValue as BasicSelectValue,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@orbis/ui/select";
import { Switch } from "@orbis/ui/switch";
import { useQuery } from "@tanstack/react-query";
import { Controller, useFieldArray } from "react-hook-form";
import { useI18n } from "@/locales/client";
import { useTRPC } from "@/ipc/query-adapter";
import { type StudentImportFormData, useCsvContext } from "../shared/context";
import { FieldRow } from "../shared/field-row";

export function StudentFieldMapping() {
  const { setValue, control, watch, fileColumns, formState } =
    useCsvContext<StudentImportFormData>();
  const trpc = useTRPC();
  const t = useI18n();

  const { fields, append, remove } = useFieldArray({
    control,
    name: "metadataMappings",
  });
  const autoGenerate = watch("autoGenerateStudentNumber");

  const academicYearId = watch("academicYearId");
  const classId = watch("classId");

  // --- High Performance Auto-Selection logic using React Query's select pattern ---

  const { data: years } = useQuery(
    trpc.academic.getYears.queryOptions(undefined, {
      select: (data) => {
        if (!academicYearId && data.length > 0) {
          const current = data.find((y) => y.isCurrent) || data[0];
          if (current) {
            setTimeout(() => {
              if (!watch("academicYearId"))
                setValue("academicYearId", current.id);
            }, 0);
          }
        }
        return data;
      },
    }),
  );

  const { data: classes } = useQuery(
    trpc.classes.getAll.queryOptions(
      {
        academicYearId: academicYearId ?? undefined,
      },
      {
        enabled: !!academicYearId,
        select: (data) => {
          const firstClass = data.data?.[0];
          if (!classId && data.data?.length === 1 && firstClass) {
            setTimeout(() => {
              if (!watch("classId")) setValue("classId", firstClass.id);
            }, 0);
          }
          return data;
        },
      },
    ),
  );

  return (
    <div className="mt-6">
      <div className="grid grid-cols-2 gap-x-4 gap-y-2 mb-4">
        <div className="text-sm font-medium">{t("import.csv_column")}</div>
        <div className="text-sm font-medium">{t("import.system_column")}</div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-4 border-b pb-6 mb-6">
        <div className="space-y-2">
          <Label className="text-[#878787] text-xs font-normal">
            {t("import.academic_year")}
          </Label>
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
                  <SelectValue placeholder={t("import.select_year")} />
                </SelectTrigger>
                <SelectContent>
                  {years?.map((year) => (
                    <SelectItem key={year.id} value={year.id.toString()}>
                      {year.yearName}{" "}
                      {year.isCurrent ? `(${t("education.current")})` : ""}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            )}
          />
        </div>

        <div className="space-y-2">
          <Label className="text-[#878787] text-xs font-normal">
            {t("import.class_section")}
          </Label>
          <Controller
            control={control}
            name="classId"
            render={({ field }) => (
              <Select
                value={field.value?.toString()}
                onValueChange={(val) =>
                  field.onChange(val ? Number(val) : undefined)
                }
                disabled={!academicYearId}
              >
                <SelectTrigger className="w-full h-9">
                  <SelectValue
                    placeholder={
                      academicYearId
                        ? t("import.select_class")
                        : t("import.select_year_first")
                    }
                  />
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
            <Label className="text-sm font-normal">
              {t("import.auto_generate")}
            </Label>
          </div>
        </div>

        {!autoGenerate && (
          <FieldRow field="studentNumber" type="students" required />
        )}
        <FieldRow field="name" type="students" required />
        <FieldRow field="phone" type="students" />
        <FieldRow field="email" type="students" />
        <FieldRow field="sex" type="students" />

        {/* Parent Information Section */}
        <div className="col-span-2 pt-4 border-t mt-2">
          <Label className="text-[#878787] text-xs font-normal uppercase tracking-wider">
            {t("import.parent_info")}
          </Label>
        </div>
        <FieldRow field="fatherName" type="students" />
        <FieldRow field="fatherPhone" type="students" />
        <FieldRow field="motherName" type="students" />
        <FieldRow field="motherPhone" type="students" />

        {/* Custom Student Meta Section - Exactly like Parent Info */}
        <div className="col-span-2 pt-6 border-t mt-4 flex items-center justify-between">
          <Label className="text-[#878787] text-xs font-normal uppercase tracking-wider">
            {t("import.custom_meta")}
          </Label>
          <Button
            type="button"
            variant="ghost"
            size="sm"
            className="h-7 text-[10px] uppercase font-bold text-primary hover:bg-primary/5 px-2 gap-1"
            onClick={() => append({ key: "", column: "" })}
          >
            <Icons.Add className="size-3" />
          </Button>
        </div>

        {fields.map((field, index) => {
          const errors = (formState.errors.metadataMappings as any)?.[index];

          return (
            <div
              key={field.id}
              className="col-span-2 grid grid-cols-2 gap-x-4 items-start group"
            >
              <div className="relative flex min-w-0 items-center gap-2">
                <Controller
                  control={control}
                  name={`metadataMappings.${index}.column`}
                  render={({ field: columnField }) => (
                    <div className="grow space-y-1">
                      <BasicSelect
                        value={columnField.value}
                        onValueChange={columnField.onChange}
                      >
                        <BasicSelectTrigger
                          className={`h-9 text-xs bg-background border-border ${errors?.column ? "border-red-400" : ""}`}
                        >
                          <BasicSelectValue
                            placeholder={t("import.map_from_column")}
                          />
                        </BasicSelectTrigger>
                        <BasicSelectContent>
                          {fileColumns
                            ?.filter((c) => c != null && c !== "")
                            .map((col, idx) => (
                              <BasicSelectItem
                                key={`${col}-${idx}`}
                                value={col}
                              >
                                {col}
                              </BasicSelectItem>
                            ))}
                        </BasicSelectContent>
                      </BasicSelect>
                      {errors?.column && (
                        <p className="text-[9px] text-red-500 font-medium ml-1 leading-none">
                          {errors.column.message}
                        </p>
                      )}
                    </div>
                  )}
                />
                <div className="flex items-center justify-end shrink-0">
                  <Icons.ArrowRightAlt className="size-4 text-[#878787]" />
                </div>
              </div>

              <div className="flex items-center gap-2">
                <Controller
                  control={control}
                  name={`metadataMappings.${index}.key`}
                  render={({ field: keyField }) => (
                    <div className="grow space-y-1">
                      <div className="flex h-9 w-full items-center border border-border bg-transparent px-0 text-sm">
                        <Input
                          {...keyField}
                          placeholder={t("import.attribute_name")}
                          className={`h-full border-0 focus-visible:ring-0 text-xs shadow-none px-3 font-normal text-foreground placeholder:text-muted-foreground/60 ${errors?.key ? "text-red-500" : ""}`}
                        />
                      </div>
                      {errors?.key && (
                        <p className="text-[9px] text-red-500 font-medium ml-1 leading-none">
                          {errors.key.message}
                        </p>
                      )}
                    </div>
                  )}
                />
                <Button
                  type="button"
                  variant="ghost"
                  size="icon"
                  className="h-8 w-8 text-muted-foreground hover:text-red-500 hover:bg-red-50/50 rounded-lg opacity-0 group-hover:opacity-100 transition-opacity shrink-0"
                  onClick={() => remove(index)}
                >
                  <Icons.Delete className="size-3.5" />
                </Button>
              </div>
            </div>
          );
        })}

        {fields.length === 0 && (
          <div
            className="col-span-2 flex items-center justify-center p-4 border border-dashed border-border/60 rounded-lg bg-muted/5 opacity-80 cursor-pointer hover:bg-muted/10 transition-colors mt-2"
            onClick={() => append({ key: "", column: "" })}
          >
            <p className="text-[10px] text-muted-foreground font-medium uppercase tracking-tight">
              {t("import.click_to_add")}
            </p>
          </div>
        )}
      </div>
    </div>
  );
}
