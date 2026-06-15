import { TableSkeleton } from "@/components/tables/core";
import { columns } from "./columns";

export function ReportsSkeleton() {
  return (
    <div
      className="overflow-hidden border-l border-r border-b border-border"
      style={{
        height: "calc(100vh - 200px + var(--header-offset, 0px))",
      }}
    >
      <TableSkeleton
        columns={columns}
        rowCount={20}
        stickyColumnIds={["select", "name"]}
        actionsColumnId="actions"
      />
    </div>
  );
}
