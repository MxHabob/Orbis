"use client";

import { TableSkeleton } from "@/components/tables/core";
import { columns } from "./columns";

export function ReportsSkeleton() {
  return (
    <TableSkeleton
      columns={columns}
      rowCount={25}
      stickyColumnIds={["select", "name"]}
      actionsColumnId="actions"
    />
  );
}
