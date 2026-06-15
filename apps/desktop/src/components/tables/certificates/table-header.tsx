import {
  horizontalListSortingStrategy,
  SortableContext,
} from "@dnd-kit/sortable";
import { Button } from "@orbis/ui/button";
import { Checkbox } from "@orbis/ui/checkbox";
import { TableHead, TableHeader, TableRow } from "@orbis/ui/table";
import type { Header, Table } from "@tanstack/react-table";
import { ArrowDown, ArrowUp } from "lucide-react";
import { useMemo } from "react";
import { HorizontalPagination } from "@/components/horizontal-pagination";
import type { TableScrollState } from "@/components/tables/core";
import { DraggableHeader } from "@/components/tables/draggable-header";
import { ResizeHandle } from "@/components/tables/resize-handle";
import { useSortQuery } from "@/hooks/use-sort-query";
import { useStickyColumns } from "@/hooks/use-sticky-columns";
import { useI18n } from "@/locales/client";
import {
  NON_REORDERABLE_COLUMNS,
  SORT_FIELD_MAPS,
  STICKY_COLUMNS,
} from "@/utils/table-configs";

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
  const { sortColumn, sortValue, createSortQuery } = useSortQuery();
  const t = useI18n();

  // Use the reusable sticky columns hook
  const { getStickyStyle, getStickyClassName, isVisible } = useStickyColumns({
    table,
    loading,
    stickyColumns: STICKY_COLUMNS.certificates,
  });

  // Get sortable column IDs (excluding sticky columns)
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
              const meta = header.column.columnDef.meta as
                | { sticky?: boolean; className?: string }
                | undefined;
              const isSticky = meta?.sticky;
              const canReorder =
                !NON_REORDERABLE_COLUMNS.certificates.has(columnId);

              if (!isVisible(columnId)) return null;

              // Check if this is the last column before actions (should flex to fill space)
              const isLastBeforeActions =
                headerIndex === headers.length - 2 &&
                headers[headers.length - 1]?.column.id === "actions";

              const headerStyle = {
                width: header.getSize(),
                minWidth: isSticky
                  ? header.getSize()
                  : header.column.columnDef.minSize,
                maxWidth: isSticky ? header.getSize() : undefined,
                ...getStickyStyle(columnId),
                // Only apply flex: 1 to non-sticky columns
                ...(isLastBeforeActions &&
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
                const isActionsColumn = columnId === "actions";
                const isSelectColumn = columnId === "select";
                const finalClassName = isActionsColumn
                  ? "group/header relative h-full px-4 !border-t !border-inline-start !border-border flex items-center justify-center md:sticky md:inset-inline-end-0 bg-background z-10"
                  : isSelectColumn
                    ? `group/header relative h-full p-0 !border-t !border-inline-end !border-border flex items-center justify-center md:sticky md:inset-inline-start-0 bg-background z-10`
                    : `${stickyClass} bg-background z-10`;

                return (
                  <TableHead
                    key={header.id}
                    className={finalClassName}
                    style={headerStyle}
                  >
                    {renderHeaderContent(
                      header,
                      columnId,
                      sortColumn,
                      sortValue,
                      createSortQuery,
                      table,
                      t,
                      tableScroll,
                    )}
                    <ResizeHandle header={header} />
                  </TableHead>
                );
              }

              // Draggable columns
              return (
                <DraggableHeader
                  key={header.id}
                  id={columnId}
                  className={getStickyClassName(
                    columnId,
                    "group/header relative h-full px-4 border-t border-border flex items-center",
                  )}
                  style={headerStyle}
                >
                  {renderHeaderContent(
                    header,
                    columnId,
                    sortColumn,
                    sortValue,
                    createSortQuery,
                    table,
                    t,
                    tableScroll,
                  )}
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

/**
 * Renders the content inside a header cell
 */
function renderHeaderContent<TData>(
  header: Header<TData, unknown>,
  columnId: string,
  sortColumn: string | undefined,
  sortValue: string | undefined,
  createSortQuery: (name: string) => void,
  table: Table<TData>,
  t: any,
  tableScroll?: TableScrollState,
) {
  const sortField = SORT_FIELD_MAPS.certificates[columnId];

  // Select column - checkbox
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

  // Actions column - static text
  if (columnId === "actions") {
    return (
      <span className="text-muted-foreground w-full text-center">
        {t("common.actions")}
      </span>
    );
  }

  // Name column - special case with horizontal pagination (like description in transactions)
  if (columnId === "name") {
    return (
      <div className="flex items-center justify-between w-full overflow-hidden">
        <div className="min-w-0 overflow-hidden">
          <SortButton
            label={t("common.template_name")}
            sortField="name"
            currentSortColumn={sortColumn}
            currentSortValue={sortValue}
            onSort={createSortQuery}
          />
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

  // Default sortable header
  if (sortField) {
    const headerLabel = getHeaderLabel(columnId, t);
    return (
      <div className="w-full overflow-hidden">
        <SortButton
          label={headerLabel}
          sortField={sortField}
          currentSortColumn={sortColumn}
          currentSortValue={sortValue}
          onSort={createSortQuery}
        />
      </div>
    );
  }

  // Fallback - just render the header text
  return (
    <span className="truncate">{header.column.columnDef.header as string}</span>
  );
}

function SortButton({
  label,
  sortField,
  currentSortColumn,
  currentSortValue,
  onSort,
}: {
  label: string;
  sortField: string;
  currentSortColumn?: string;
  currentSortValue?: string;
  onSort: (field: string) => void;
}) {
  return (
    <Button
      className="p-0 hover:bg-transparent space-x-2 min-w-0 max-w-full"
      variant="ghost"
      onClick={(e) => {
        e.stopPropagation(); // Prevent drag when clicking sort
        onSort(sortField);
      }}
    >
      <span className="truncate">{label}</span>
      {sortField === currentSortColumn && currentSortValue === "asc" && (
        <ArrowDown size={16} />
      )}
      {sortField === currentSortColumn && currentSortValue === "desc" && (
        <ArrowUp size={16} />
      )}
    </Button>
  );
}

function getHeaderLabel(columnId: string, t: any): string {
  const labels: Record<string, string> = {
    name: t("common.template_name"),
    size: t("common.size"),
    orientation: t("common.orientation"),
    updatedAt: t("common.updated_at"),
    actions: t("common.actions"),
  };
  return labels[columnId] || columnId;
}
