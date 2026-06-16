import { useMemo, useCallback } from "react";
import { getCoreRowModel, useReactTable } from "@tanstack/react-table";
import { ExcelGrid } from "@orbis/excel-grid";
import { useTableSettings } from "@orbis/excel-grid";
import { useI18n } from "@/locales/client";
import { STICKY_COLUMNS } from "@/utils/table-configs";
import type { TableSettings } from "@/utils/table-settings";
import {
  createGradeColumns,
  type GradeGridData,
  type GradeTableMeta,
} from "./columns";
import { EmptyState } from "./empty-states";

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
        (g: any) =>
          g.studentId === studentId && g.assessmentItemId === assessmentItemId,
      );
      return grade?.score ?? null;
    },
    [grades, pendingChanges],
  );

  const columns = useMemo(
    () => createGradeColumns(items, month ?? undefined, t),
    [items, month, t],
  );

  // Derived State - calculate order during render, avoiding useEffect
  const effectiveColumnOrder = useMemo(() => {
    const currentColumnIds = columns.map((c) => c.id as string);

    // If no saved order, use the default order from createGradeColumns
    if (columnOrder.length === 0) return currentColumnIds;

    // Filter out columns that no longer exist (e.g. after switching months)
    const activeOrder = columnOrder.filter(
      (id) => currentColumnIds.includes(id) && id !== "total",
    );

    // Find new columns that aren't in the saved order yet
    const newColumns = currentColumnIds.filter(
      (id) => !columnOrder.includes(id) && id !== "total",
    );

    // Final construction: Preserved Order + New Columns + Total at the very end
    return [...activeOrder, ...newColumns, "total"];
  }, [columns, columnOrder]);

  const tableMeta = useMemo(
    () =>
      ({
        items,
        grades,
        getGradeValue,
        onGradeChange,
        onBulkFill,
        onClearColumn,
      }) as GradeTableMeta,
    [items, grades, getGradeValue, onGradeChange, onBulkFill, onClearColumn],
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
      const nextOrder =
        typeof updater === "function" ? updater(columnOrder) : updater;
      if (nextOrder.includes("total")) {
        setColumnOrder([...nextOrder.filter((id) => id !== "total"), "total"]);
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

  const handlePaste = useCallback((pastedText: string, startRowIndex: number, startColIndex: number) => {
    if (!onGradeChange) return;

    const rows = pastedText.split(/\r?\n/).map(row => row.split("\t"));
    const visibleColumns = table.getVisibleLeafColumns();

    rows.forEach((rowCells, rOffset) => {
      const targetRowIndex = startRowIndex + rOffset;
      const student = students[targetRowIndex];
      if (!student) return;

      rowCells.forEach((value, cOffset) => {
        const targetColIndex = startColIndex + cOffset;
        const column = visibleColumns[targetColIndex];
        if (!column || !column.id?.startsWith("assessment-")) return;

        const itemId = parseInt(column.id.split("-")[1]!);
        const cleanVal = value.trim();
        const score = cleanVal === "" ? null : parseFloat(cleanVal);
        if (score === null || !isNaN(score)) {
          onGradeChange(student.studentId, itemId, score);
        }
      });
    });
  }, [students, onGradeChange, table]);

  if (!students.length) {
    return <EmptyState />;
  }

  return (
    <ExcelGrid
      table={table}
      height="calc(100vh - 260px)"
      stickyColumns={STICKY_COLUMNS.grades}
      onPaste={handlePaste}
      t={t}
    />
  );
}

