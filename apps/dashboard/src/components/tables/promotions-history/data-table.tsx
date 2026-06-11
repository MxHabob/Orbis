"use client";

import { cn } from "@orbis/ui/cn";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@orbis/ui/table";
import {
  flexRender,
  getCoreRowModel,
  useReactTable,
} from "@tanstack/react-table";
import React from "react";
import { useI18n } from "@/locales/client";
import { columns, type PromotionHistory } from "./columns";
import { HistoryEmptyState } from "./empty-states";
import { HistoryTableSkeleton } from "./skeleton";
import { HistoryTableHeader } from "./table-header";

interface PromotionsHistoryTableProps {
  data: PromotionHistory[];
  isLoading: boolean;
  onRollback: (studentIds: string[]) => void;
  onOverride: (history: PromotionHistory) => void;
  isRollingBack?: boolean;
}

export function PromotionsHistoryDataTable({
  data,
  isLoading,
  onRollback,
  onOverride,
  isRollingBack,
}: PromotionsHistoryTableProps) {
  const t = useI18n();

  const tableColumns = React.useMemo(
    () => columns(t, onRollback, onOverride, isRollingBack),
    [t, onRollback, onOverride, isRollingBack],
  );

  const table = useReactTable({
    data,
    columns: tableColumns,
    getCoreRowModel: getCoreRowModel(),
  });

  if (isLoading) {
    return <HistoryTableSkeleton />;
  }

  if (data.length === 0) {
    return <HistoryEmptyState />;
  }

  const currentYearName = data[0]?.fromYearName;
  const targetYearName = data[0]?.toYearName;

  return (
    <div className="w-full bg-background">
      <HistoryTableHeader
        currentYear={currentYearName}
        targetYear={targetYearName}
        count={data.length}
        onRollbackAll={() => onRollback(data.map((d) => d.studentId))}
        isRollingBack={isRollingBack}
      />

      <div className="border-x border-b border-border">
        <Table>
          <TableHeader>
            {table.getHeaderGroups().map((headerGroup) => (
              <TableRow
                key={headerGroup.id}
                className="hover:bg-transparent bg-muted/30 border-b h-10 border-border"
              >
                {headerGroup.headers.map((header, index) => (
                  <TableHead
                    key={header.id}
                    className={cn(
                      "text-[10px] uppercase font-bold tracking-tight py-2 h-auto text-[#606060]",
                      header.id === "status" && "text-center",
                      index === headerGroup.headers.length - 1 && "text-right",
                    )}
                  >
                    {flexRender(
                      header.column.columnDef.header,
                      header.getContext(),
                    )}
                  </TableHead>
                ))}
              </TableRow>
            ))}
          </TableHeader>
          <TableBody>
            {table.getRowModel().rows.map((row) => (
              <TableRow
                key={row.id}
                className="h-10 hover:bg-muted/30 transition-colors group border-b last:border-0 text-foreground border-border"
              >
                {row.getVisibleCells().map((cell, index) => (
                  <TableCell
                    key={cell.id}
                    className={cn(
                      cell.column.id === "status" && "text-center",
                      index === row.getVisibleCells().length - 1 &&
                        "text-right",
                    )}
                  >
                    {flexRender(cell.column.columnDef.cell, cell.getContext())}
                  </TableCell>
                ))}
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </div>
    </div>
  );
}
