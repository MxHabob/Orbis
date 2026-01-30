"use client";

import { TableSkeleton } from "@/components/tables/core";
import type { ColumnSizingState, VisibilityState } from "@tanstack/react-table";
import { createGradeColumns, type AssessmentItem } from "./columns";

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
  );
}
