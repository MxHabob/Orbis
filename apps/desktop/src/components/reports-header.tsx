"use client";

import { Button } from "@orbis/ui/button";
import { Icons } from "@orbis/ui/icons";
import { useReportExportParams } from "@/hooks/use-report-export-params";
import { AcademicYearFilter } from "./academic-year-filter";
import { ReportColumnVisibility } from "./report-column-visibility";
import { ReportsSearchFilter } from "./reports-search-filter";

export function ReportsHeader() {
  const { setParams } = useReportExportParams();
  return (
    <div className="flex items-center justify-between py-4">
      <div className="flex items-center gap-4">
        <ReportsSearchFilter />
        <AcademicYearFilter />
      </div>

      <div className="hidden sm:flex space-x-2">
        <Button
          variant="outline"
          className="gap-2 shrink-0"
          onClick={() => setParams({ exportReport: true })}
        >
          <Icons.FolderZip size={16} />
        </Button>
        <ReportColumnVisibility />
      </div>
    </div>
  );
}
