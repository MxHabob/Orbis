import * as React from "react";
import { useMemo, useRef, useCallback } from "react";
import {
  Table,
  TableBody,
  TableCell,
  TableRow,
  TableHead,
  TableHeader,
} from "@orbis/ui/table";
import { Button } from "@orbis/ui/button";
import { cn } from "@orbis/ui/cn";
import { Icons } from "@orbis/ui/icons";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@orbis/ui/tooltip";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuTrigger,
} from "@orbis/ui/dropdown-menu";
import {
  flexRender,
  type Header,
  type Table as ReactTable,
  type Cell,
  type Row,
} from "@tanstack/react-table";
import { useVirtualizer, type VirtualItem } from "@tanstack/react-virtual";
import { closestCenter, DndContext } from "@dnd-kit/core";
import {
  horizontalListSortingStrategy,
  SortableContext,
  useSortable,
} from "@dnd-kit/sortable";
import { CSS } from "@dnd-kit/utilities";
import {
  GripVertical,
  MoreHorizontal,
  Zap,
  Eraser,
  Trash2,
} from "lucide-react";

import {
  useStickyColumns,
  type StickyColumnConfig,
} from "./hooks/use-sticky-columns";
import { useTableDnd } from "./hooks/use-table-dnd";
import { useTableScroll } from "./hooks/use-table-scroll";

// -------------------------------------------------------------
// Subcomponents
// -------------------------------------------------------------

interface DraggableHeaderProps {
  id: string;
  children: React.ReactNode;
  className?: string;
  style?: React.CSSProperties;
  disabled?: boolean;
}

function DraggableHeader({
  id,
  children,
  className,
  style,
  disabled = false,
}: DraggableHeaderProps) {
  const {
    attributes,
    listeners,
    setNodeRef,
    transform,
    transition,
    isDragging,
  } = useSortable({
    id,
    disabled,
  });

  const dragStyle: React.CSSProperties = {
    transform: CSS.Translate.toString(transform),
    transition,
    ...style,
  };

  return (
    <TableHead
      ref={setNodeRef}
      className={cn(
        "group/header relative h-full px-4 border-t border-border flex items-center select-none",
        "shadow-none outline-none ring-0 focus:shadow-none focus:outline-none focus:ring-0 hover:shadow-none",
        isDragging && "border border-border bg-background z-50",
        className,
      )}
      style={dragStyle}
    >
      <div className="flex-1 min-w-0 overflow-hidden">{children}</div>
      {!disabled && (
        <GripVertical
          size={14}
          className="ml-1 text-muted-foreground opacity-0 group-hover/header:opacity-100 flex-shrink-0 cursor-grab active:cursor-grabbing"
          {...attributes}
          {...listeners}
        />
      )}
    </TableHead>
  );
}

interface ResizeHandleProps<TData> {
  header: Header<TData, unknown>;
  className?: string;
}

function ResizeHandle<TData>({ header, className }: ResizeHandleProps<TData>) {
  if (!header.column.getCanResize()) {
    return null;
  }

  return (
    <div
      onDoubleClick={() => header.column.resetSize()}
      onMouseDown={(e) => {
        e.stopPropagation();
        header.getResizeHandler()(e);
      }}
      onTouchStart={(e) => {
        e.stopPropagation();
        header.getResizeHandler()(e);
      }}
      onPointerDown={(e) => e.stopPropagation()}
      className={cn(
        "absolute right-0 top-0 h-full w-1 cursor-col-resize select-none touch-none",
        "bg-transparent",
        className,
      )}
      style={{
        transform: "translateX(50%)",
      }}
    />
  );
}

interface HorizontalPaginationProps {
  canScrollLeft: boolean;
  canScrollRight: boolean;
  onScrollLeft: () => void;
  onScrollRight: () => void;
  className?: string;
  t?: (key: string) => string;
}

function HorizontalPagination({
  canScrollLeft,
  canScrollRight,
  onScrollLeft,
  onScrollRight,
  className,
}: HorizontalPaginationProps) {
  return (
    <div className={cn("flex items-center space-x-2", className)}>
      <Button
        variant="outline"
        size="sm"
        disabled={!canScrollLeft}
        className="size-6 p-0"
        onClick={onScrollLeft}
      >
        <Icons.ArrowBack
          className={cn("size-3.5", canScrollLeft && "text-primary")}
        />
      </Button>
      <Button
        variant="outline"
        size="sm"
        disabled={!canScrollRight}
        className="size-6 p-0"
        onClick={onScrollRight}
      >
        <Icons.ArrowForward
          className={cn("size-3.5", canScrollRight && "text-primary")}
        />
      </Button>
    </div>
  );
}

interface DataTableHeaderProps<TData> {
  table: ReactTable<TData>;
  loading?: boolean;
  tableScroll?: ReturnType<typeof useTableScroll>;
  getStickyStyle: (columnId: string) => React.CSSProperties;
  getStickyClassName: (columnId: string, baseClassName?: string) => string;
  isVisible: (columnId: string) => boolean;
  t?: (key: string, options?: any) => string;
  nonReorderableColumns?: Set<string>;
}

function DataTableHeader<TData>({
  table,
  loading,
  tableScroll,
  getStickyStyle,
  getStickyClassName,
  isVisible,
  t = (k) => k,
  nonReorderableColumns = new Set(["studentName", "total"]),
}: DataTableHeaderProps<TData>) {
  const sortableColumnIds = useMemo(() => {
    return table
      .getAllLeafColumns()
      .filter((col) => !nonReorderableColumns.has(col.id))
      .map((col) => col.id);
  }, [table, nonReorderableColumns]);

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
              const canReorder = !nonReorderableColumns.has(columnId);

              if (!isVisible(columnId)) return null;

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
                ...(isLastBeforeTotal &&
                  !isSticky && {
                    flex: 1,
                  }),
              };

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

              return (
                <DraggableHeader
                  key={header.id}
                  id={columnId}
                  style={headerStyle}
                >
                  <div className="flex items-center flex-1 min-w-0 overflow-hidden justify-center relative group/header-cell">
                    {renderHeaderContent(header, columnId, t, tableScroll)}

                    {columnId.startsWith("assessment-") && (
                      <div className="absolute right-0 opacity-0 group-hover/header-cell:opacity-100 transition-opacity">
                        <AssessmentBulkActions
                          header={header}
                          table={table}
                          t={t}
                        />
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
  t: (key: string, options?: any) => string,
  tableScroll?: ReturnType<typeof useTableScroll>,
) {
  if (columnId === "total") {
    return (
      <span className="text-primary w-full text-center font-bold">
        {t("grades.total") || "Total"}
      </span>
    );
  }

  if (columnId === "studentName") {
    return (
      <div className="flex items-center justify-between w-full overflow-hidden">
        <div className="min-w-0 overflow-hidden">
          <span className="font-semibold">
            {t("grades.student_name") || "Student Name"}
          </span>
        </div>
        {tableScroll?.isScrollable && (
          <HorizontalPagination
            canScrollLeft={tableScroll.canScrollLeft}
            canScrollRight={tableScroll.canScrollRight}
            onScrollLeft={tableScroll.scrollLeft}
            onScrollRight={tableScroll.scrollRight}
            className="hidden md:flex flex-shrink-0"
            t={t}
          />
        )}
      </div>
    );
  }

  if (typeof header.column.columnDef.header === "function") {
    return header.column.columnDef.header(header.getContext());
  }

  return (
    <span className="truncate">{header.column.columnDef.header as string}</span>
  );
}

function AssessmentBulkActions<TData>({
  header,
  table,
  t,
}: {
  header: Header<TData, unknown>;
  table: ReactTable<TData>;
  t: (key: string, options?: any) => string;
}) {
  const meta = table.options.meta as any;
  const columnId = header.column.id;
  const assessmentId = parseInt(columnId.replace("assessment-", ""));

  const assessmentItem = meta?.items?.find((i: any) => i.id === assessmentId);
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
              {t("grades.fill_max", { maxScore }) || `Fill Max (${maxScore})`}
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
            <TooltipContent side="top">
              {t("grades.fill_zero") || "Fill 0"}
            </TooltipContent>
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
              {t("grades.clear_column") || "Clear Column"}
            </TooltipContent>
          </Tooltip>
        </DropdownMenuContent>
      </DropdownMenu>
    </TooltipProvider>
  );
}

// -------------------------------------------------------------
// VirtualRow Component
// -------------------------------------------------------------

interface VirtualRowProps<TData> {
  row: Row<TData>;
  virtualStart: number;
  rowHeight: number;
  getStickyStyle: (columnId: string) => React.CSSProperties;
  getStickyClassName: (columnId: string, baseClassName?: string) => string;
  nonClickableColumns?: Set<string>;
  columnSizing?: ColumnSizingState;
  columnOrder?: ColumnOrderState;
  columnVisibility?: VisibilityState;
}

function VirtualRowInner<TData>({
  row,
  virtualStart,
  rowHeight,
  getStickyStyle,
  getStickyClassName,
  nonClickableColumns = new Set(["select", "actions"]),
}: VirtualRowProps<TData>) {
  const cells = row.getVisibleCells();
  const lastCellId = cells[cells.length - 1]?.column.id ?? "";
  const isLastColumnSticky = lastCellId === "actions" || lastCellId === "total";

  return (
    <TableRow
      data-index={row.index}
      className={cn(
        "group cursor-pointer select-text",
        "hover:bg-[#F2F1EF] hover:dark:bg-[#0f0f0f]",
        "flex items-center border-0",
        "absolute top-0 inset-inline-start-0 w-full min-w-full",
      )}
      style={{
        height: rowHeight,
        transform: `translateY(${virtualStart}px)`,
        contain: "layout style paint",
      }}
    >
      {cells.map((cell: Cell<TData, unknown>, cellIndex: number) => {
        const columnId = cell.column.id;
        const meta = cell.column.columnDef.meta as any;
        const isSticky = meta?.sticky ?? false;
        const isCentered = columnId === "actions" || columnId === "select";
        const isLastBeforeSticky =
          cellIndex === cells.length - 2 && isLastColumnSticky;

        const cellStyle: React.CSSProperties = {
          width: cell.column.getSize(),
          ...getStickyStyle(columnId),
          ...(isLastBeforeSticky && !isSticky && { flex: 1 }),
        };

        const cellClassName = getStickyClassName(columnId, meta?.className);

        return (
          <TableCell
            key={cell.id}
            className={cn(
              "h-full flex items-center border-b border-border",
              cellClassName,
              isCentered && "justify-center",
            )}
            style={cellStyle}
          >
            <div
              className={cn(
                "overflow-hidden truncate",
                isCentered
                  ? "flex items-center justify-center h-full w-full"
                  : "w-full",
              )}
            >
              {flexRender(cell.column.columnDef.cell, cell.getContext())}
            </div>
          </TableCell>
        );
      })}
    </TableRow>
  );
}

function arePropsEqual<TData>(
  prevProps: VirtualRowProps<TData>,
  nextProps: VirtualRowProps<TData>,
): boolean {
  return (
    prevProps.row.id === nextProps.row.id &&
    prevProps.virtualStart === nextProps.virtualStart &&
    prevProps.rowHeight === nextProps.rowHeight &&
    prevProps.columnSizing === nextProps.columnSizing &&
    prevProps.columnOrder === nextProps.columnOrder &&
    prevProps.columnVisibility === nextProps.columnVisibility &&
    prevProps.row.original === nextProps.row.original
  );
}

const VirtualRow = React.memo(VirtualRowInner, arePropsEqual) as <TData>(
  props: VirtualRowProps<TData>,
) => React.ReactNode;

// -------------------------------------------------------------
// ExcelGrid Component (Main Wrapper)
// -------------------------------------------------------------

export interface ExcelGridProps<TData> {
  table: ReactTable<TData>;
  height?: string;
  stickyColumns?: StickyColumnConfig[];
  nonReorderableColumns?: Set<string>;

  // Custom Translations Callback
  t?: (key: string, options?: any) => string;

  // Excel Specific Events
  onPaste?: (
    pastedText: string,
    startRowIndex: number,
    startColIndex: number,
  ) => void;
}

export function ExcelGrid<TData>({
  table,
  height = "calc(100vh - 260px)",
  stickyColumns = [],
  nonReorderableColumns = new Set(["studentName", "total"]),
  t = (k) => k,
  onPaste,
}: ExcelGridProps<TData>) {
  const parentRef = useRef<HTMLDivElement>(null);

  const { columnVisibility, columnSizing, columnOrder } = table.getState();

  // Use the sticky columns hook
  const { getStickyStyle, getStickyClassName, isVisible } = useStickyColumns({
    columnVisibility,
    table,
    stickyColumns,
  });

  // Use the table scroll hook
  const tableScroll = useTableScroll({
    useColumnWidths: true,
    startFromColumn: 1, // Skip name column
  });

  // DnD for column reordering
  const { sensors, handleDragEnd } = useTableDnd(table);

  const rows = table.getRowModel().rows;

  // Row virtualizer for performance
  const rowVirtualizer = useVirtualizer({
    count: rows.length,
    getScrollElement: () => parentRef.current,
    estimateSize: () => 45,
    overscan: 10,
  });

  const virtualItems = rowVirtualizer.getVirtualItems();

  // 1. Keyboard Arrow Navigation and Enter Key Delegation
  const handleKeyDown = useCallback(
    (e: React.KeyboardEvent<HTMLDivElement>) => {
      const target = e.target as HTMLElement;
      if (target.tagName !== "INPUT") return;

      const { key } = e;
      const row = parseInt(target.getAttribute("data-row") || "-1");
      const col = parseInt(target.getAttribute("data-col") || "-1");
      if (row === -1 || col === -1) return;

      const focusInput = (r: number, c: number) => {
        const nextInput = parentRef.current?.querySelector(
          `input[data-row="${r}"][data-col="${c}"]`,
        ) as HTMLInputElement;
        if (nextInput) {
          nextInput.focus();
          nextInput.select();
        }
      };

      if (key === "Enter" || key === "ArrowDown") {
        e.preventDefault();
        focusInput(row + 1, col);
      } else if (key === "ArrowUp") {
        e.preventDefault();
        focusInput(row - 1, col);
      } else if (key === "ArrowRight") {
        const input = target as HTMLInputElement;
        if (input.selectionEnd === input.value.length) {
          focusInput(row, col + 1);
        }
      } else if (key === "ArrowLeft") {
        const input = target as HTMLInputElement;
        if (input.selectionStart === 0) {
          focusInput(row, col - 1);
        }
      }
    },
    [],
  );

  // 2. Clipboard Paste Handler Delegation
  const handlePasteEvent = useCallback(
    (e: React.ClipboardEvent<HTMLDivElement>) => {
      const target = e.target as HTMLElement;
      if (target.tagName !== "INPUT") return;

      const row = parseInt(target.getAttribute("data-row") || "-1");
      const col = parseInt(target.getAttribute("data-col") || "-1");
      if (row === -1 || col === -1) return;

      e.preventDefault();
      const pastedText = e.clipboardData.getData("text");

      if (onPaste) {
        onPaste(pastedText, row, col);
      }
    },
    [onPaste],
  );

  return (
    <div
      className="relative"
      onKeyDown={handleKeyDown}
      onPaste={handlePasteEvent}
    >
      <div className="w-full">
        <div
          ref={(el) => {
            if (parentRef) {
              (
                parentRef as React.MutableRefObject<HTMLDivElement | null>
              ).current = el;
            }
            if (tableScroll.containerRef) {
              (
                tableScroll.containerRef as React.MutableRefObject<HTMLDivElement | null>
              ).current = el;
            }
          }}
          className="overflow-auto overscroll-none border-l border-r border-b border-border scrollbar-hide"
          style={{ height }}
        >
          <DndContext
            id="excel-grid-dnd"
            sensors={sensors}
            collisionDetection={closestCenter}
            onDragEnd={handleDragEnd}
          >
            <Table className="w-full min-w-full">
              <DataTableHeader
                table={table}
                tableScroll={tableScroll}
                getStickyStyle={getStickyStyle}
                getStickyClassName={getStickyClassName}
                isVisible={isVisible}
                t={t}
                nonReorderableColumns={nonReorderableColumns}
              />

              <TableBody
                className="border-l-0 border-r-0 block"
                style={{
                  height: `${rowVirtualizer.getTotalSize()}px`,
                  position: "relative",
                }}
              >
                {virtualItems.length > 0 ? (
                  virtualItems.map((virtualRow: VirtualItem) => {
                    const row = rows[virtualRow.index];
                    if (!row) return null;

                    return (
                      <VirtualRow
                        key={row.id}
                        row={row}
                        virtualStart={virtualRow.start}
                        rowHeight={45}
                        getStickyStyle={getStickyStyle}
                        getStickyClassName={getStickyClassName}
                        columnSizing={columnSizing}
                        columnOrder={columnOrder}
                        columnVisibility={columnVisibility}
                      />
                    );
                  })
                ) : (
                  <TableRow>
                    <TableCell
                      colSpan={table.getAllLeafColumns().length}
                      className="h-24 text-center"
                    >
                      {t("vault.no_results_title") || "No Results"}
                    </TableCell>
                  </TableRow>
                )}
              </TableBody>
            </Table>
          </DndContext>
        </div>
      </div>
    </div>
  );
}
