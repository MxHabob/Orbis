import { cn } from "@orbis/ui/cn";
import type { VisibilityState } from "@tanstack/react-table";
import { useCallback, useMemo } from "react";

export interface StickyColumnConfig {
  id: string;
  width: number;
}

interface TableColumn {
  id: string;
  getIsVisible: () => boolean;
}

interface TableInterface {
  getAllLeafColumns: () => TableColumn[];
}

interface UseStickyColumnsProps {
  columnVisibility?: VisibilityState;
  table?: TableInterface;
  loading?: boolean;
  stickyColumns?: StickyColumnConfig[];
}

export function useStickyColumns({
  columnVisibility,
  table,
  loading,
  stickyColumns = [],
}: UseStickyColumnsProps) {
  const isVisible = useCallback(
    (id: string) =>
      loading ||
      table
        ?.getAllLeafColumns()
        .find((col) => col.id === id)
        ?.getIsVisible() ||
      (columnVisibility && columnVisibility[id] !== false),
    [loading, table, columnVisibility],
  );

  const stickyColumnIds = useMemo(
    () => new Set(stickyColumns.map((col) => col.id)),
    [stickyColumns],
  );

  const stickyPositions = useMemo(() => {
    const checkVisible = (id: string) =>
      loading ||
      table
        ?.getAllLeafColumns()
        .find((col) => col.id === id)
        ?.getIsVisible() ||
      (columnVisibility && columnVisibility[id] !== false);

    let position = 0;
    const positions: Record<string, number> = {};

    for (const col of stickyColumns) {
      if (checkVisible(col.id)) {
        positions[col.id] = position;
        position += col.width;
      }
    }

    return positions;
  }, [loading, table, columnVisibility, stickyColumns]);

  const getStickyStyle = useCallback(
    (columnId: string) => {
      const position = stickyPositions[columnId];
      return position !== undefined
        ? ({ "--stick-start": `${position}px` } as React.CSSProperties)
        : {};
    },
    [stickyPositions],
  );

  const getStickyClassName = useCallback(
    (columnId: string, baseClassName?: string) => {
      const isSticky = stickyColumnIds.has(columnId);
      return cn(
        baseClassName,
        isSticky && "md:sticky md:inset-inline-start-[var(--stick-start)]",
      );
    },
    [stickyColumnIds],
  );

  return {
    stickyPositions,
    getStickyStyle,
    getStickyClassName,
    isVisible,
  };
}
