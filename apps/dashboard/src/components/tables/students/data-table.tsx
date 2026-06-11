"use client";

import { closestCenter, DndContext } from "@dnd-kit/core";
import { Table, TableBody, TableCell, TableRow } from "@orbis/ui/table";
import { useSuspenseInfiniteQuery } from "@tanstack/react-query";
import { getCoreRowModel, useReactTable } from "@tanstack/react-table";
import { useVirtualizer, type VirtualItem } from "@tanstack/react-virtual";
import {
  useCallback,
  useDeferredValue,
  useEffect,
  useMemo,
  useRef,
} from "react";
import { VirtualRow } from "@/components/tables/core";
import { useCustomerFilterParams } from "@/hooks/use-customer-filter-params";
import { useInfiniteScroll } from "@/hooks/use-infinite-scroll";
import { useScrollHeader } from "@/hooks/use-scroll-header";
import { useSortParams } from "@/hooks/use-sort-params";
import { useStickyColumns } from "@/hooks/use-sticky-columns";
import { useStudentParams } from "@/hooks/use-student-params";
import { useTableDnd } from "@/hooks/use-table-dnd";
import { useTableScroll } from "@/hooks/use-table-scroll";
import { useTableSettings } from "@/hooks/use-table-settings";
import { useStudentsStore } from "@/store/students";
import { useTRPC } from "@/trpc/client";
import { STICKY_COLUMNS, SUMMARY_GRID_HEIGHTS } from "@/utils/table-configs";
import type { TableSettings } from "@/utils/table-settings";
import { columns } from "./columns";
import { EmptyState, NoResults } from "./empty-states";
import { DataTableHeader } from "./table-header";

const NON_CLICKABLE_COLUMNS = new Set(["actions"]);

type Props = {
  initialSettings?: Partial<TableSettings>;
  id: string;
};

export function DataTable({ initialSettings, id }: Props) {
  const trpc = useTRPC();
  const { filter, hasFilters } = useCustomerFilterParams();
  const { params } = useSortParams();
  const { deleteStudentId, setParams: setStudentParams } = useStudentParams();
  const parentRef = useRef<HTMLDivElement>(null);
  const { setColumns } = useStudentsStore();

  const deferredSearch = useDeferredValue(filter.q);

  useScrollHeader(parentRef, { extraOffset: SUMMARY_GRID_HEIGHTS.students });

  const {
    columnVisibility,
    setColumnVisibility,
    columnSizing,
    setColumnSizing,
    columnOrder,
    setColumnOrder,
  } = useTableSettings({
    tableId: "students",
    initialSettings,
  });

  const infiniteQueryOptions = trpc.students.getByClassId.infiniteQueryOptions(
    {
      id,
      q: deferredSearch ?? undefined,
      sort: params.sort ?? undefined,
      academicYearId: filter.academicYearId
        ? Number(filter.academicYearId)
        : undefined,
    },
    {
      getNextPageParam: (lastPage) => lastPage.meta.cursor,
    },
  );

  const { data, fetchNextPage, hasNextPage, isFetchingNextPage, refetch } =
    useSuspenseInfiniteQuery(infiniteQueryOptions);

  const handleDeleteStudent = useCallback(
    (id: string) => {
      setStudentParams({ deleteStudentId: id });
    },
    [setStudentParams],
  );

  const tableData = useMemo(() => {
    return data?.pages.flatMap((page) => page.data) ?? [];
  }, [data]);

  const setOpen = useCallback(
    (id?: string) => {
      if (id) {
        setStudentParams({ studentId: id, details: true });
      } else {
        setStudentParams(null);
      }
    },
    [setStudentParams],
  );

  const tableMeta = useMemo(
    () => ({
      deleteStudent: handleDeleteStudent,
    }),
    [handleDeleteStudent],
  );

  const table = useReactTable({
    data: tableData,
    getRowId: (row) => row.userId,
    columns,
    getCoreRowModel: getCoreRowModel(),
    onColumnVisibilityChange: setColumnVisibility,
    enableColumnResizing: true,
    columnResizeMode: "onChange",
    onColumnSizingChange: setColumnSizing,
    onColumnOrderChange: setColumnOrder,
    state: {
      columnVisibility,
      columnSizing,
      columnOrder,
    },
    meta: tableMeta as any,
  });

  // DnD for column reordering
  const { sensors, handleDragEnd } = useTableDnd(table);

  // Sync columns to store for column visibility toggle
  useEffect(() => {
    setColumns(table.getAllLeafColumns());
  }, [table, setColumns, columnVisibility]);

  // Use the reusable sticky columns hook
  const { getStickyStyle, getStickyClassName } = useStickyColumns({
    columnVisibility,
    table,
    stickyColumns: STICKY_COLUMNS.students,
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

  // Trigger infinite load when scrolling near the bottom
  useInfiniteScroll<HTMLDivElement>({
    scrollRef: parentRef,
    rowVirtualizer,
    rowCount: rows.length,
    hasNextPage,
    isFetchingNextPage,
    fetchNextPage,
    threshold: 50,
  });

  if (!tableData.length) {
    return (
      <div
        className="flex items-center justify-center border border-border bg-background"
        style={{ height: "calc(100vh - 420px + var(--header-offset, 0px))" }}
      >
        {hasFilters ? <NoResults /> : <EmptyState />}
      </div>
    );
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
          className="overflow-auto overscroll-contain border-l border-r border-b border-border scrollbar-hide"
          style={{
            height: "calc(100vh - 420px + var(--header-offset, 0px))",
          }}
        >
          <DndContext
            id="students-table-dnd"
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
                        onCellClick={setOpen}
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
                      No results.
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
