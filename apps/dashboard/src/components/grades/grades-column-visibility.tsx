"use client";

import { useGradesStore } from "@/store/grades";
import { Button } from "@midday/ui/button";
import { Checkbox } from "@midday/ui/checkbox";
import { Icons } from "@midday/ui/icons";
import { Popover, PopoverContent, PopoverTrigger } from "@midday/ui/popover";
import { useI18n } from "@/locales/client";

export function GradesColumnVisibility() {
  const { columns } = useGradesStore();
  const t = useI18n();

  return (
    <Popover>
      <PopoverTrigger asChild>
        <Button variant="outline" size="icon">
          <Icons.Tune size={18} />
        </Button>
      </PopoverTrigger>

      <PopoverContent className="w-[200px] p-0" align="end" sideOffset={8}>
        <div className="flex flex-col p-4 space-y-2 max-h-[352px] overflow-auto">
          {columns
            .filter(
              (column) =>
                column.columnDef.enableHiding !== false &&
                column.id !== "status",
            )
            .map((column) => {
              let label = column.id;
              const metaLabel = (column.columnDef.meta as any)?.headerLabel;
              
              if (metaLabel) {
                if (metaLabel.startsWith("grades.") || metaLabel.startsWith("common.") || metaLabel.startsWith("education.")) {
                   // @ts-ignore
                  label = t(metaLabel);
                } else {
                  label = metaLabel;
                }
              } else if (typeof column.columnDef.header === "string") {
                label = column.columnDef.header;
              }

              return (
                <div key={column.id} className="flex items-center space-x-2">
                  <Checkbox
                    id={column.id}
                    checked={column.getIsVisible()}
                    onCheckedChange={(checked) =>
                      column.toggleVisibility(checked === true)
                    }
                  />
                  <label
                    htmlFor={column.id}
                    className="text-sm peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
                  >
                    {label}
                  </label>
                </div>
              );
            })}
        </div>
      </PopoverContent>
    </Popover>
  );
}
