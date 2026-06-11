"use client";

import type { ColumnSizingState, VisibilityState } from "@tanstack/react-table";
import { TableSkeleton } from "@/components/tables/core";
import { type AssessmentItem, createGradeColumns } from "./columns";

interface GradesSkeletonProps {
  items?: AssessmentItem[];
  columnVisibility?: VisibilityState;
  columnSizing?: ColumnSizingState;
  columnOrder?: string[];
  isEmpty?: boolean;
}

export function GradesSkeleton({
  items = [],
  columnVisibility = {},
  columnSizing = {},
  columnOrder = [],
  isEmpty = false,
}: GradesSkeletonProps) {
  // Create columns based on assessment items
  const columns = createGradeColumns(items);

  return (
    <div
      className="overflow-hidden border-l border-r border-b border-border"
      style={{
        height: "calc(100vh - 260px + var(--header-offset, 0px))",
      }}
    >
      <TableSkeleton
        columns={columns}
        rowCount={15}
        columnVisibility={columnVisibility}
        columnSizing={columnSizing}
        columnOrder={columnOrder}
        stickyColumnIds={["studentName"]}
        actionsColumnId="total"
        isEmpty={isEmpty}
      />
    </div>
  );
}
