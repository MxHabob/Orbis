import type { ColumnSizingState, VisibilityState } from "@tanstack/react-table";
import { TableSkeleton } from "@/components/tables/core";
import { columns } from "./columns";

interface StudentsSkeletonProps {
  columnVisibility?: VisibilityState;
  columnSizing?: ColumnSizingState;
  columnOrder?: string[];
  isEmpty?: boolean;
}

export function StudentsSkeleton({
  columnVisibility = {},
  columnSizing = {},
  columnOrder = [],
  isEmpty = false,
}: StudentsSkeletonProps) {
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
