"use client";

import { VirtualRow } from "@/components/tables/core";
import { useScrollHeader } from "@/hooks/use-scroll-header";
import { useStickyColumns } from "@/hooks/use-sticky-columns";
import { useTableDnd } from "@/hooks/use-table-dnd";
import { useTableScroll } from "@/hooks/use-table-scroll";
import { useTableSettings } from "@/hooks/use-table-settings";
import { useTRPC } from "@/trpc/client";
import { STICKY_COLUMNS } from "@/utils/table-configs";
import type { TableSettings } from "@/utils/table-settings";
import { DndContext, closestCenter } from "@dnd-kit/core";
import { Table, TableBody, TableCell, TableRow } from "@midday/ui/table";
import { useSuspenseQuery } from "@tanstack/react-query";
import { getCoreRowModel, useReactTable } from "@tanstack/react-table";
import { type VirtualItem, useVirtualizer } from "@tanstack/react-virtual";

import { useCallback, useEffect, useRef } from "react";
import { columns } from "./columns";
import { EmptyState } from "./empty-states";
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

  const trpc = useTRPC();
  const parentRef = useRef<HTMLDivElement>(null);

  // Hide header on scroll
  useScrollHeader(parentRef);

  const { filter } = useCertificateFilterParams();
  const { setParams } = useCertificateParams();
  const { rowSelection, setRowSelection } = useInvoiceStore();

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
    return (
      <div className="relative h-[calc(100vh-200px)] overflow-hidden">
        <EmptyState />
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
              (parentRef as React.MutableRefObject<HTMLDivElement | null>).current = el;
            }
            if (tableScroll.containerRef) {
              (tableScroll.containerRef as React.MutableRefObject<HTMLDivElement | null>).current = el;
            }
          }}
          className="overflow-auto overscroll-none border-l border-r border-b border-border scrollbar-hide"
          style={{
            height: "calc(100vh - 180px + var(--header-offset, 0px))",
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
          {/* Spacer ensures scrolling works when content barely overflows */}
          <div
            style={{ height: "var(--header-offset, 0px)", flexShrink: 0 }}
            aria-hidden
          />
        </div>
      </div>

      <AnimatePresence>
        {showBottomBar && <BottomBar data={tableData} />}
      </AnimatePresence>
    </div>
  );
}
