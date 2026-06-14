"use client";

import {
  horizontalListSortingStrategy,
  SortableContext,
} from "@dnd-kit/sortable";
import { Button } from "@orbis/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@orbis/ui/dropdown-menu";
import { TableHead, TableHeader, TableRow } from "@orbis/ui/table";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@orbis/ui/tooltip";
import type { Header, Table } from "@tanstack/react-table";
import { Eraser, MoreHorizontal, Trash2, Zap } from "lucide-react";
import { useMemo } from "react";
import { HorizontalPagination } from "@/components/horizontal-pagination";
import type { TableScrollState } from "@/components/tables/core";
import { DraggableHeader } from "@/components/tables/draggable-header";
import { ResizeHandle } from "@/components/tables/resize-handle";
import { useStickyColumns } from "@/hooks/use-sticky-columns";
import { useI18n } from "@/locales/client";
import { STICKY_COLUMNS } from "@/utils/table-configs";
import type { AssessmentItem } from "./columns";

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

  const t = useI18n();

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
                  additionalClasses =
                    "justify-center md:sticky md:inset-inline-end-0 bg-primary/5 font-bold !border-inline-start !border-border";
                } else if (isStudentNameColumn) {
                  additionalClasses =
                    "md:sticky md:inset-inline-start-0 bg-background !border-inline-end !border-border";
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
                    {renderHeaderContent(header, columnId, t, tableScroll)}
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
                  <div className="flex items-center flex-1 min-w-0 overflow-hidden justify-center relative group/header-cell">
                    {renderHeaderContent(header, columnId, t, tableScroll)}

                    {/* Bulk Actions Menu (2026 UX: Proactive tools available on hover) */}
                    {columnId.startsWith("assessment-") && (
                      <div className="absolute right-0 opacity-0 group-hover/header-cell:opacity-100 transition-opacity">
                        <AssessmentBulkActions header={header} table={table} />
                      </div>
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
  t: (key: any, options?: any) => string,
  tableScroll?: TableScrollState,
) {
  // Total column
  if (columnId === "total") {
    return (
      <span className="text-primary w-full text-center font-bold">
        {t("grades.total")}
      </span>
    );
  }

  // Student name column - special case with horizontal pagination
  if (columnId === "studentName") {
    return (
      <div className="flex items-center justify-between w-full overflow-hidden">
        <div className="min-w-0 overflow-hidden">
          <span className="font-semibold">{t("grades.student_name")}</span>
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

function AssessmentBulkActions<TData>({
  header,
  table,
}: {
  header: Header<TData, unknown>;
  table: Table<TData>;
}) {
  const t = useI18n();
  const meta = table.options.meta as any;
  const columnId = header.column.id;
  const assessmentId = parseInt(columnId.replace("assessment-", ""));

  // Find assessment item details from meta
  const assessmentItem = meta?.items?.find(
    (i: AssessmentItem) => i.id === assessmentId,
  );
  if (!assessmentItem) return null;

  const maxScore = Number(assessmentItem.maxScore);

  return (
    <TooltipProvider>
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button variant="ghost" size="icon" className="h-6 w-6">
            <MoreHorizontal className="h-3 w-3" />
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent
          align="end"
          className="p-1 flex items-center gap-1 min-w-0"
        >
          <Tooltip>
            <TooltipTrigger asChild>
              <div
                className="p-2 hover:bg-accent rounded-md cursor-pointer transition-colors"
                onClick={() => meta?.onBulkFill?.(assessmentId, maxScore)}
              >
                <Zap className="h-4 w-4 text-amber-500" />
              </div>
            </TooltipTrigger>
            <TooltipContent side="top">
              {t("grades.fill_max", { maxScore })}
            </TooltipContent>
          </Tooltip>

          <Tooltip>
            <TooltipTrigger asChild>
              <div
                className="p-2 hover:bg-accent rounded-md cursor-pointer transition-colors"
                onClick={() => meta?.onBulkFill?.(assessmentId, 0)}
              >
                <Eraser className="h-4 w-4 text-muted-foreground" />
              </div>
            </TooltipTrigger>
            <TooltipContent side="top">{t("grades.fill_zero")}</TooltipContent>
          </Tooltip>

          <div className="w-[1px] h-4 bg-border mx-0.5" />

          <Tooltip>
            <TooltipTrigger asChild>
              <div
                className="p-2 hover:bg-destructive/10 hover:text-destructive rounded-md cursor-pointer transition-colors text-destructive"
                onClick={() => meta?.onClearColumn?.(assessmentId)}
              >
                <Trash2 className="h-4 w-4" />
              </div>
            </TooltipTrigger>
            <TooltipContent side="top">
              {t("grades.clear_column")}
            </TooltipContent>
          </Tooltip>
        </DropdownMenuContent>
      </DropdownMenu>
    </TooltipProvider>
  );
}
