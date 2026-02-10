"use client";

import { TableSkeleton } from "@/components/tables/core";
import type { ColumnSizingState, VisibilityState } from "@tanstack/react-table";
import { columns } from "./columns";

interface CustomersSkeletonProps {
  columnVisibility?: VisibilityState;
  columnSizing?: ColumnSizingState;
  columnOrder?: string[];
  isEmpty?: boolean;
}

export function CustomersSkeleton({
  columnVisibility = {},
  columnSizing = {},
  columnOrder = [],
  isEmpty = false,
}: CustomersSkeletonProps) {
  return (
    <div
      className="overflow-hidden border-l border-r border-b border-border"
      style={{
        height: "calc(100vh - 380px + var(--header-offset, 0px))",
      }}
    >
      <TableSkeleton
        columns={columns}
        rowCount={25}
        columnVisibility={columnVisibility}
        columnSizing={columnSizing}
        columnOrder={columnOrder}
        stickyColumnIds={["name"]}
        actionsColumnId="actions"
        isEmpty={isEmpty}
      />
    </div>
  );
}
