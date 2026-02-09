"use client";

import type { TableScrollState } from "@/components/tables/core";
import { DraggableHeader } from "@/components/tables/draggable-header";
import { ResizeHandle } from "@/components/tables/resize-handle";
import { useStickyColumns } from "@/hooks/use-sticky-columns";
import {
  NON_REORDERABLE_COLUMNS,
  STICKY_COLUMNS,
} from "@/utils/table-configs";
import {
  SortableContext,
  horizontalListSortingStrategy,
} from "@dnd-kit/sortable";
import { Checkbox } from "@midday/ui/checkbox";
import { TableHead, TableHeader, TableRow } from "@midday/ui/table";
import type { Header, Table } from "@tanstack/react-table";
import { useMemo } from "react";

interface Props<TData> {
  table?: Table<TData>;
  loading?: boolean;
  tableScroll?: TableScrollState;
}

export function DataTableHeader<TData>({
  table,
  loading,
  tableScroll,
}: Props<TData>) {
  const { getStickyStyle, getStickyClassName, isVisible } = useStickyColumns({
    table,
    loading,
    stickyColumns: STICKY_COLUMNS.certificates,
  });

  const stickyColumnIds = useMemo(
    () => new Set(STICKY_COLUMNS.certificates.map((col) => col.id)),
    [],
  );

  const sortableColumnIds = useMemo(() => {
    if (!table) return [];
    return table
      .getAllLeafColumns()
      .filter((col) => !NON_REORDERABLE_COLUMNS.certificates.has(col.id))
      .map((col) => col.id);
  }, [table]);

  if (!table) return null;

  const headerGroups = table.getHeaderGroups();

  return (
    <TableHeader className="border-0 block sticky top-0 z-20 bg-background w-full">
      {headerGroups.map((headerGroup) => (
        <TableRow
          key={headerGroup.id}
          className="h-[45px] hover:bg-transparent flex items-center !border-b-0 min-w-full"
        >
          <SortableContext
            items={sortableColumnIds}
            strategy={horizontalListSortingStrategy}
          >
            {headerGroup.headers.map((header, headerIndex, headers) => {
              const columnId = header.column.id;
              
              if (!isVisible(columnId)) return null;

              const isSticky = stickyColumnIds.has(columnId);
              const canReorder = !NON_REORDERABLE_COLUMNS.certificates.has(columnId);

              const headerStyle = {
                width: header.getSize(),
                minWidth: isSticky
                  ? header.getSize()
                  : header.column.columnDef.minSize,
                maxWidth: isSticky ? header.getSize() : undefined,
                ...getStickyStyle(columnId),
              };

              if (!canReorder) {
                const stickyClass = getStickyClassName(
                  columnId,
                  "group/header relative h-full px-4 border-t border-border flex items-center",
                );
                const isActionsColumn = columnId === "actions";
                const finalClassName = isActionsColumn
                  ? "group/header relative h-full px-4 !border-t !border-l !border-border flex items-center justify-center md:sticky md:right-0 bg-background z-10"
                  : `${stickyClass} bg-background z-10`;

                return (
                  <TableHead
                    key={header.id}
                    className={finalClassName}
                    style={headerStyle}
                  >
                    {renderHeaderContent(header, columnId, table)}
                    {header.column.getCanResize() && (
                      <ResizeHandle header={header} />
                    )}
                  </TableHead>
                );
              }

              return (
                <DraggableHeader
                  key={header.id}
                  id={columnId}
                  style={headerStyle}
                >
                  <div className="flex items-center flex-1 min-w-0 overflow-hidden">
                    {renderHeaderContent(header, columnId, table)}
                  </div>
                  {header.column.getCanResize() && (
                    <ResizeHandle header={header} />
                  )}
                </DraggableHeader>
              );
            })}
          </SortableContext>
        </TableRow>
      ))}
    </TableHeader>
  );
}

function renderHeaderContent<TData>(
  header: Header<TData, unknown>,
  columnId: string,
  table: Table<TData>,
) {
  if (columnId === "select") {
    return (
      <Checkbox
        checked={
          table.getIsAllPageRowsSelected() ||
          (table.getIsSomePageRowsSelected() && "indeterminate")
        }
        onCheckedChange={(value) => table.toggleAllPageRowsSelected(!!value)}
      />
    );
  }

  if (columnId === "actions") {
    return (
      <span className="text-muted-foreground w-full text-center"></span>
    );
  }

  const meta = header.column.columnDef.meta as { headerLabel?: string } | undefined;
  const label = meta?.headerLabel || columnId;

  return <span className="truncate text-[11px] font-medium uppercase text-muted-foreground">{label}</span>;
}
