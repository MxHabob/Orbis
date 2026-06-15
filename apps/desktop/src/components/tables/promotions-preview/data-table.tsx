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
import { columns, type PromotionPreview } from "./columns";
import { PreviewEmptyState } from "./empty-states";
import { PreviewTableSkeleton } from "./skeleton";

interface PromotionsPreviewTableProps {
  data: PromotionPreview[];
  isLoading: boolean;
}

export function PromotionsPreviewDataTable({
  data,
  isLoading,
}: PromotionsPreviewTableProps) {
  const t = useI18n();

  const tableColumns = React.useMemo(() => columns(t), [t]);

  const table = useReactTable({
    data,
    columns: tableColumns,
    getCoreRowModel: getCoreRowModel(),
  });

  if (isLoading) {
    return <PreviewTableSkeleton />;
  }

  if (data.length === 0) {
    return <PreviewEmptyState />;
  }

  return (
    <div className="w-full bg-background border-x border-border">
      <Table>
        <TableHeader>
          {table.getHeaderGroups().map((headerGroup) => (
            <TableRow
              key={headerGroup.id}
              className="hover:bg-transparent bg-muted/30 border-b h-10 border-border"
            >
              {headerGroup.headers.map((header) => (
                <TableHead
                  key={header.id}
                  className={cn(
                    "text-[10px] uppercase font-bold tracking-tight py-2 h-auto text-[#606060]",
                    (header.id === "finalScore" ||
                      header.id === "suggestedStatus") &&
                      "text-center",
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
              className="h-10 hover:bg-muted/30 transition-colors group border-b border-border last:border-0"
            >
              {row.getVisibleCells().map((cell) => (
                <TableCell
                  key={cell.id}
                  className={cn(
                    (cell.column.id === "finalScore" ||
                      cell.column.id === "suggestedStatus") &&
                      "text-center",
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
  );
}
