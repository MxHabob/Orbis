"use client";

import { ReportsSearchFilter } from "./reports-search-filter";
import { ReportColumnVisibility } from "./report-column-visibility";
import { AcademicYearFilter } from "./academic-year-filter";
import { useReportExportParams } from "@/hooks/use-report-export-params";
import { Button } from "@midday/ui/button";
import { Icons } from "@midday/ui/icons";

export function ReportsHeader() {
  const { setParams } = useReportExportParams();
  return (
    <div className="flex items-center justify-between py-4">
      <div className="flex items-center gap-4">
        <ReportsSearchFilter />
        <AcademicYearFilter />
      </div>

      <div className="hidden sm:flex space-x-2">
        <Button variant="outline" className="gap-2 shrink-0" onClick={() => setParams({ exportReport: true })}>
           <Icons.FolderZip size={16} />
        </Button>
        <ReportColumnVisibility />
      </div>
    </div>
  );
}
