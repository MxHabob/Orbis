"use client";

import { HorizontalPagination } from "@/components/horizontal-pagination";
import type { TableScrollState } from "@/components/tables/core";
import { DraggableHeader } from "@/components/tables/draggable-header";
import { ResizeHandle } from "@/components/tables/resize-handle";
import { useStickyColumns } from "@/hooks/use-sticky-columns";
import {
  SortableContext,
  horizontalListSortingStrategy,
} from "@dnd-kit/sortable";
import { TableHead, TableHeader, TableRow } from "@midday/ui/table";
import type { Header, Table } from "@tanstack/react-table";
import { useMemo } from "react";
import { STICKY_COLUMNS } from "@/utils/table-configs";

interface Props<TData> {
  table?: Table<TData>;
  loading?: boolean;
  tableScroll?: TableScrollState;
}

const NON_REORDERABLE_COLUMNS = new Set(["studentName", "total"]);

export function DataTableHeader<TData>({
  table,
  loading,
  tableScroll,
}: Props<TData>) {
  // Use the reusable sticky columns hook
  const { getStickyStyle, getStickyClassName, isVisible } = useStickyColumns({
    table,
    loading,
    stickyColumns: STICKY_COLUMNS.grades,
  });

  // Get sortable column IDs (excluding sticky columns)
  const sortableColumnIds = useMemo(() => {
    if (!table) return [];
    return table
      .getAllLeafColumns()
      .filter((col) => !NON_REORDERABLE_COLUMNS.has(col.id))
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
              const meta = header.column.columnDef.meta as
                | { sticky?: boolean; className?: string }
                | undefined;
              const isSticky = meta?.sticky;
              const canReorder = !NON_REORDERABLE_COLUMNS.has(columnId);

              if (!isVisible(columnId)) return null;

              // Check if this is the last column before total (should flex to fill space)
              const isLastBeforeTotal =
                headerIndex === headers.length - 2 &&
                headers[headers.length - 1]?.column.id === "total";

              const headerStyle = {
                width: header.getSize(),
                minWidth: isSticky
                  ? header.getSize()
                  : header.column.columnDef.minSize,
                maxWidth: isSticky ? header.getSize() : undefined,
                ...getStickyStyle(columnId),
                // Only apply flex: 1 to non-sticky columns
                ...(isLastBeforeTotal &&
                  !isSticky && {
                    flex: 1,
                  }),
              };

              // Sticky columns use regular TableHead (not draggable)
              if (!canReorder) {
                const stickyClass = getStickyClassName(
                  columnId,
                  "group/header relative h-full px-4 border-t border-border flex items-center",
                );
                const isTotalColumn = columnId === "total";
                const isStudentNameColumn = columnId === "studentName";
                
                let additionalClasses = "";
                if (isTotalColumn) {
                  additionalClasses = "justify-center md:sticky md:right-0 bg-primary/5 font-bold !border-l";
                } else if (isStudentNameColumn) {
                  additionalClasses = "md:sticky md:left-0 bg-background";
                } else {
                  additionalClasses = "bg-background";
                }

                const finalClassName = `${stickyClass} z-10 ${additionalClasses}`;

                return (
                  <TableHead
                    key={header.id}
                    className={finalClassName}
                    style={headerStyle}
                  >
                    {renderHeaderContent(
                      header,
                      columnId,
                      tableScroll,
                    )}
                    <ResizeHandle header={header} />
                  </TableHead>
                );
              }

              // Draggable columns (assessment items)
              return (
                <DraggableHeader
                  key={header.id}
                  id={columnId}
                  style={headerStyle}
                >
                  <div className="flex items-center flex-1 min-w-0 overflow-hidden justify-center">
                    {renderHeaderContent(
                      header,
                      columnId,
                      tableScroll,
                    )}
                  </div>
                  <ResizeHandle header={header} />
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
  tableScroll?: TableScrollState,
) {
  // Total column
  if (columnId === "total") {
    return (
      <span className="text-primary w-full text-center font-bold">Total</span>
    );
  }

  // Student name column - special case with horizontal pagination
  if (columnId === "studentName") {
    return (
      <div className="flex items-center justify-between w-full overflow-hidden">
        <div className="min-w-0 overflow-hidden">
          <span className="font-semibold">Student Name</span>
        </div>
        {tableScroll?.isScrollable && (
          <HorizontalPagination
            canScrollLeft={tableScroll.canScrollLeft}
            canScrollRight={tableScroll.canScrollRight}
            onScrollLeft={tableScroll.scrollLeft}
            onScrollRight={tableScroll.scrollRight}
            className="hidden md:flex flex-shrink-0"
          />
        )}
      </div>
    );
  }

  // Assessment item columns - render the header function result
  if (typeof header.column.columnDef.header === "function") {
    return header.column.columnDef.header(header.getContext());
  }

  // Fallback - just render the header text
  return (
    <span className="truncate">{header.column.columnDef.header as string}</span>
  );
}