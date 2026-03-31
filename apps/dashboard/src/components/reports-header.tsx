"use client";

import { ReportsSearchFilter } from "./reports-search-filter";
import { ReportColumnVisibility } from "./report-column-visibility";
import { AcademicYearFilter } from "./academic-year-filter";

export function ReportsHeader() {
  return (
    <div className="flex items-center justify-between py-4">
      <div className="flex items-center gap-4">
        <ReportsSearchFilter />
        <AcademicYearFilter />
      </div>

      <div className="hidden sm:flex space-x-2">
        <ReportColumnVisibility />
      </div>
    </div>
  );
}
