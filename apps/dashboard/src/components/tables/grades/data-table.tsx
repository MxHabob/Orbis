"use client";

import { VirtualRow } from "@/components/tables/core";
import { useScrollHeader } from "@/hooks/use-scroll-header";
import { useStickyColumns } from "@/hooks/use-sticky-columns";
import { useTableScroll } from "@/hooks/use-table-scroll";
import { useTableDnd } from "@/hooks/use-table-dnd";
import { useI18n } from "@/locales/client";
import { useTableSettings } from "@/hooks/use-table-settings";
import { STICKY_COLUMNS, SUMMARY_GRID_HEIGHTS } from "@/utils/table-configs";
import type { TableSettings } from "@/utils/table-settings";
import { DndContext, closestCenter } from "@dnd-kit/core";
import { Table, TableBody, TableCell, TableRow } from "@orbis/ui/table";
import { getCoreRowModel, useReactTable } from "@tanstack/react-table";
import { type VirtualItem, useVirtualizer } from "@tanstack/react-virtual";
import { useMemo, useRef } from "react";
import {
  createGradeColumns,
  type GradeGridData,
  type GradeTableMeta,
} from "./columns";
import { EmptyState } from "./empty-states";
import { DataTableHeader } from "./table-header";

const NON_CLICKABLE_COLUMNS = new Set(["total"]);

type Props = {
  initialSettings?: Partial<TableSettings>;
  initialTab?: string;
  onGradeChange?: (
    studentId: string,
    assessmentItemId: number,
    value: number | null,
  ) => void;
  onBulkFill?: (assessmentItemId: number, value: number) => void;
  onClearColumn?: (assessmentItemId: number) => void;
  data: GradeGridData;
  pendingChanges?: Record<string, number | null>;
};

export function GradeDataTable({
  initialSettings,
  onGradeChange,
  onBulkFill,
  onClearColumn,
  data,
  pendingChanges,
}: Props) {
  const { students, items, grades, month } = data;
  const t = useI18n();

  const parentRef = useRef<HTMLDivElement>(null);

  useScrollHeader(parentRef, { extraOffset: SUMMARY_GRID_HEIGHTS.grades });

  const {
    columnVisibility,
    setColumnVisibility,
    columnSizing,
    setColumnSizing,
    columnOrder,
    setColumnOrder,
  } = useTableSettings({
    tableId: "grades",
    initialSettings,
  });

  const getGradeValue = useMemo(
    () => (studentId: string, assessmentItemId: number) => {
      const key = `${studentId}:${assessmentItemId}`;
      if (pendingChanges && key in pendingChanges) {
        return pendingChanges[key]?.toString() ?? null;
      }

      const grade = grades.find(
        (g: any) => g.studentId === studentId && g.assessmentItemId === assessmentItemId
      );
      return grade?.score ?? null;
    },
    [grades, pendingChanges]
  );

  const columns = useMemo(() => createGradeColumns(items, month ?? undefined, t), [items, month, t]);

  // Modern React 2026: Derived State - calculate order during render, avoiding useEffect
  const effectiveColumnOrder = useMemo(() => {
    const currentColumnIds = columns.map((c) => c.id as string);
    
    // If no saved order, use the default order from createGradeColumns
    if (columnOrder.length === 0) return currentColumnIds;

    // Filter out columns that no longer exist (e.g. after switching months)
    const activeOrder = columnOrder.filter(
      (id) => currentColumnIds.includes(id) && id !== "total"
    );

    // Find new columns that aren't in the saved order yet
    const newColumns = currentColumnIds.filter(
      (id) => !columnOrder.includes(id) && id !== "total"
    );

    // Final construction: Preserved Order + New Columns + Total at the very end
    return [...activeOrder, ...newColumns, "total"];
  }, [columns, columnOrder]);

  const tableMeta = useMemo(
    () => ({
      items,
      grades,
      getGradeValue,
      onGradeChange,
      onBulkFill,
      onClearColumn,
    } as GradeTableMeta),
    [items, grades, getGradeValue, onGradeChange, onBulkFill, onClearColumn]
  );

  const table = useReactTable({
    data: students,
    getRowId: (row) => row.studentId,
    columns,
    getCoreRowModel: getCoreRowModel(),
    onColumnVisibilityChange: setColumnVisibility,
    enableColumnResizing: true,
    columnResizeMode: "onChange",
    onColumnSizingChange: setColumnSizing,
    onColumnOrderChange: (updater) => {
      const nextOrder = typeof updater === "function" ? updater(columnOrder) : updater;
      if (nextOrder.includes("total")) {
        setColumnOrder([...nextOrder.filter(id => id !== "total"), "total"]);
      } else {
        setColumnOrder(nextOrder);
      }
    },
    state: {
      columnVisibility,
      columnSizing,
      columnOrder: effectiveColumnOrder,
    },
    meta: tableMeta,
  });

  // DnD for column reordering
  const { sensors, handleDragEnd } = useTableDnd(table);

  // Use the reusable sticky columns hook
  const { getStickyStyle, getStickyClassName } = useStickyColumns({
    columnVisibility,
    table,
    stickyColumns: STICKY_COLUMNS.grades,
  });

  // Use the reusable table scroll hook
  const tableScroll = useTableScroll({
    useColumnWidths: true,
    startFromColumn: 1, // Skip sticky name column
  });

  const rows = table.getRowModel().rows;

  // Row virtualizer for performance
  const rowVirtualizer = useVirtualizer({
    count: rows.length,
    getScrollElement: () => parentRef.current,
    estimateSize: () => 45,
    overscan: 10,
  });

  if (!students.length) {
    return <EmptyState />;
  }

  const virtualItems = rowVirtualizer.getVirtualItems();

  return (
    <div className="relative">
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
          style={{
            height: "calc(100vh - 260px + var(--header-offset, 0px))",
          }}
        >
          <DndContext
            id="grades-table-dnd"
            sensors={sensors}
            collisionDetection={closestCenter}
            onDragEnd={handleDragEnd}
          >
            <Table className="w-full min-w-full">
              <DataTableHeader table={table} tableScroll={tableScroll} />

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
                        nonClickableColumns={NON_CLICKABLE_COLUMNS}
                        columnSizing={columnSizing}
                        columnOrder={columnOrder}
                        columnVisibility={columnVisibility}
                      />
                    );
                  })
                ) : (
                  <TableRow>
                    <TableCell
                      colSpan={columns.length}
                      className="h-24 text-center"
                    >
                      {t("vault.no_results_title")}
                    </TableCell>
                  </TableRow>
                )}
              </TableBody>
            </Table>
          </DndContext>
          {/* Spacer ensures scrolling works when content barely overflows */}
          <div
            style={{ height: "var(--header-offset, 0px)", flexShrink: 0 }}
            aria-hidden
          />
        </div>
      </div>
    </div>
  );
}