"use client";

import { Icons } from "@midday/ui/icons";
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
import { Controller, useWatch } from "react-hook-form";
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
          <div className="flex items-center gap-1.5">
            <span>{label}</span>
            {!required && <span className="text-[10px] text-[#878787] uppercase">(Optional)</span>}
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
