import { Icons } from "@orbis/ui/icons";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
} from "@orbis/ui/select";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@orbis/ui/tooltip";
import { Controller, useWatch } from "react-hook-form";
import { useI18n } from "@/locales/client";
import { mappableFields, useCsvContext } from "./context";

export function FieldRow({
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
  const t = useI18n();
  const fields = type ? mappableFields[type] : null;
  const fieldConfig = fields ? (fields as any)[field] : null;

  // Mapping field names to translation keys
  const getLabel = (f: string) => {
    if (explicitLabel) return explicitLabel;

    const keyMap: Record<string, any> = {
      studentNumber: t("students.student_number"),
      name: t("students.full_name"),
      phone: t("students.phone"),
      email: t("students.email"),
      sex: t("students.sex"),
      fatherName: t("students.father_name"),
      fatherPhone: t("students.father_phone"),
      motherName: t("students.mother_name"),
      motherPhone: t("students.mother_phone"),
    };

    return keyMap[f] ?? fieldConfig?.label ?? f;
  };

  const label = getLabel(field);
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
                  <SelectValue placeholder={`${t("import.map_from_column")}`} />
                </SelectTrigger>
                <SelectContent>
                  <SelectGroup>
                    <SelectLabel>{label}</SelectLabel>
                    {[
                      ...(fileColumns?.filter(
                        (column) => column != null && column !== "",
                      ) || []),
                      ...(controllerField.value && !required ? ["None"] : []),
                    ]?.map((column, index) => {
                      return (
                        <SelectItem key={`${column}-${index}`} value={column}>
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
          <div className="flex items-center gap-1.5">
            <span>{label}</span>
            {!required && (
              <span className="text-[10px] text-[#878787] uppercase">
                ({t("common.other")})
              </span>
            )}
          </div>

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
