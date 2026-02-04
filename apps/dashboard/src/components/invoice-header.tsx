"use client";

import { ReportsSearchFilter } from "./reports-search-filter";
import { InvoiceColumnVisibility } from "./invoice-column-visibility";

export function ReportsHeader() {
  return (
    <div className="flex items-center justify-between py-4">
      <ReportsSearchFilter />

      <div className="hidden sm:flex space-x-2">
        <InvoiceColumnVisibility />
      </div>
    </div>
  );
}
