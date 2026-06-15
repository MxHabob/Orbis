import { Button } from "@orbis/ui/button";
import { Checkbox } from "@orbis/ui/checkbox";
import { Icons } from "@orbis/ui/icons";
import { Popover, PopoverContent, PopoverTrigger } from "@orbis/ui/popover";
import { useI18n } from "@/locales/client";
import { useReportStore } from "@/store/report";

export function ReportColumnVisibility() {
  const { columns } = useReportStore();
  const t = useI18n();

  return (
    <Popover>
      <PopoverTrigger asChild>
        <Button variant="outline" size="icon">
          <Icons.Tune size={18} />
        </Button>
      </PopoverTrigger>

      <PopoverContent className="w-[200px] p-0" align="end" sideOffset={8}>
        <div className="flex flex-col p-4 space-y-2 max-h-[450px] overflow-auto">
          {columns
            .filter(
              (column) =>
                column.columnDef.enableHiding !== false &&
                column.id !== "actions",
            )
            .map((column) => {
              let label = column.id;
              if (typeof column.columnDef.header === "string") {
                label = column.columnDef.header;
              } else if (column.id === "name") {
                label = t("education.class_name");
              } else if (column.id === "description") {
                label = t("common.description");
              } else if (column.id === "studentCount") {
                label = t("common.students");
              } else if (column.id === "scheduleCount") {
                label = t("common.schedule");
              } else if (column.id === "parentClass") {
                label = t("education.parent_class");
              } else if (column.id === "type") {
                label = t("education.type");
              } else if (column.id === "division") {
                label = t("education.division");
              } else if ((column.columnDef.meta as any)?.headerLabel) {
                label = (column.columnDef.meta as any).headerLabel;
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
