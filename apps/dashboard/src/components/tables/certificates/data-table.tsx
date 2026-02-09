"use client";

import { VirtualRow } from "@/components/tables/core";
import { useInfiniteScroll } from "@/hooks/use-infinite-scroll";
import { useScrollHeader } from "@/hooks/use-scroll-header";
import { useStickyColumns } from "@/hooks/use-sticky-columns";
import { useTableDnd } from "@/hooks/use-table-dnd";
import { useTableScroll } from "@/hooks/use-table-scroll";
import { useTableSettings } from "@/hooks/use-table-settings";
import { useTRPC } from "@/trpc/client";
import { STICKY_COLUMNS, SUMMARY_GRID_HEIGHTS } from "@/utils/table-configs";
import type { TableSettings } from "@/utils/table-settings";
import { DndContext, closestCenter } from "@dnd-kit/core";
import { Table, TableBody, TableCell, TableRow } from "@midday/ui/table";
import { useSuspenseQuery } from "@tanstack/react-query";
import { getCoreRowModel, useReactTable } from "@tanstack/react-table";
import { type VirtualItem, useVirtualizer } from "@tanstack/react-virtual";
import { useRouter } from "next/navigation";
import { useCallback, useEffect, useMemo, useRef } from "react";
import { columns } from "./columns";
import { EmptyState, NoResults } from "./empty-states";
import { DataTableHeader } from "./table-header";

import { useInvoiceStore } from "@/store/invoice";
import { BottomBar } from "./bottom-bar";
import { AnimatePresence } from "framer-motion";

import { useCertificateFilterParams } from "@/hooks/use-certificate-filter-params";
import { useCertificateParams } from "@/hooks/use-certificate-params";

const NON_CLICKABLE_COLUMNS = new Set(["select", "actions"]);

type Props = {
  initialSettings?: Partial<TableSettings>;
};

export function DataTable({ initialSettings }: Props) {
  const router = useRouter();
  const trpc = useTRPC();
  const parentRef = useRef<HTMLDivElement>(null);

  const { filter } = useCertificateFilterParams();
  const { setParams } = useCertificateParams();
  const { rowSelection, setRowSelection } = useInvoiceStore();

  // Use unified table settings hook for column state management
  const {
    columnVisibility,
    setColumnVisibility,
    columnSizing,
    setColumnSizing,
    columnOrder,
    setColumnOrder,
  } = useTableSettings({
    tableId: "certificates",
    initialSettings,
  });

  const { data: tableData } = useSuspenseQuery(
    trpc.certificateTemplates.list.queryOptions({
      q: filter.q ?? undefined,
    }),
  );

  const table = useReactTable({
    data: tableData ?? [],
    getRowId: ({ id }) => id,
    columns,
    onRowSelectionChange: setRowSelection,
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
      rowSelection,
    },
  });

  const { sensors, handleDragEnd } = useTableDnd(table);

  const tableScroll = useTableScroll({
    useColumnWidths: true,
    startFromColumn: 2,
  });

  const rows = table.getRowModel().rows;

  const handleCellClick = useCallback(
    (rowId: string) => {
      setParams({ templateId: rowId, type: "edit" });
    },
    [setParams],
  );

  useEffect(() => {
    return () => setRowSelection({});
  }, [setRowSelection]);

  const rowVirtualizer = useVirtualizer({
    count: rows.length,
    getScrollElement: () => parentRef.current,
    estimateSize: () => 57,
    overscan: 10,
  });

  const { getStickyStyle, getStickyClassName } = useStickyColumns({
    columnVisibility,
    table,
    stickyColumns: STICKY_COLUMNS.certificates,
  });

  const showBottomBar = Object.keys(rowSelection).length > 0;

  if (!tableData?.length) {
    return <EmptyState />;
  }

  const virtualItems = rowVirtualizer.getVirtualItems();

  return (
    <div className="relative">
      <div className="w-full">
        <div
          ref={(el) => {
            if (parentRef) {
              (parentRef as React.MutableRefObject<HTMLDivElement | null>).current = el;
            }
            if (tableScroll.containerRef) {
              (tableScroll.containerRef as React.MutableRefObject<HTMLDivElement | null>).current = el;
            }
          }}
          className="overflow-auto overscroll-contain border-l border-r border-b border-border scrollbar-hide"
          style={{
            height: "calc(100vh - 450px)",
          }}
        >
          <DndContext
            id="certificates-table-dnd"
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
                        rowHeight={57}
                        getStickyStyle={getStickyStyle}
                        getStickyClassName={getStickyClassName}
                        nonClickableColumns={NON_CLICKABLE_COLUMNS}
                        onCellClick={handleCellClick}
                        columnSizing={columnSizing}
                        columnOrder={columnOrder}
                        columnVisibility={columnVisibility}
                        isSelected={rowSelection[row.id] ?? false}
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
        </div>
      </div>

      <AnimatePresence>
        {showBottomBar && <BottomBar data={tableData} />}
      </AnimatePresence>
    </div>
  );
}
