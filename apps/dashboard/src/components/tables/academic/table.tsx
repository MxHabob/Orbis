"use client";

import { useAcademicParams } from "@/hooks/use-academic-params";
import { useTRPC } from "@/trpc/client";
import { cn } from "@midday/ui/cn";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@midday/ui/table";
import {
  useMutation,
  useQueryClient,
  useSuspenseQuery,
} from "@tanstack/react-query";
import {
  flexRender,
  getCoreRowModel,
  useReactTable,
} from "@tanstack/react-table";
import React from "react";
import {
  type AcademicTableMeta,
  columns,
  flattenAcademicYears,
} from "./columns";
import { Header } from "./header";
import { useI18n } from "@/locales/client";

export function DataTable() {
  const [expandedYears, setExpandedYears] = React.useState<Set<number>>(new Set());
  const trpc = useTRPC();
  const t = useI18n();
  const queryClient = useQueryClient();
  const { setParams } = useAcademicParams();

  const { data } = useSuspenseQuery(
    trpc.academic.getYears.queryOptions()
  );

  const deleteYearMutation = useMutation(
    trpc.academic.deleteYear.mutationOptions({
      onSuccess: () => {
        queryClient.invalidateQueries({ queryKey: trpc.academic.getYears.queryKey() });
      },
    })
  );

  const deleteTermMutation = useMutation(
    trpc.academic.deleteTerm.mutationOptions({
      onSuccess: () => {
        queryClient.invalidateQueries({ queryKey: trpc.academic.getYears.queryKey() });
      },
    })
  );

  const flattenedData = React.useMemo(() => {
    return flattenAcademicYears(data as any ?? []);
  }, [data]);

  const [searchValue, setSearchValue] = React.useState<string>("");

  const filteredData = React.useMemo(() => {
    let result = flattenedData;

    if (searchValue) {
      const searchLower = searchValue.toLowerCase();
      result = result.filter(item => {
        const name = (item.isTerm ? item.name : item.yearName)?.toLowerCase();
        return name?.includes(searchLower);
      });
    } else {
      result = result.filter(item => {
        if (!item.isTerm) return true;
        return expandedYears.has(item.parentYearId);
      });
    }

    return result;
  }, [flattenedData, expandedYears, searchValue]);

  const tableMeta: AcademicTableMeta = {
    deleteYear: (id: number) => {
      if (confirm(t("common.delete") + "?")) deleteYearMutation.mutate({ id });
    },
    deleteTerm: (id: number) => {
      if (confirm(t("common.delete") + "?")) deleteTermMutation.mutate({ id });
    },
    onEditYear: (id: number) => {
      setParams({ yearId: id.toString() });
    },
    onEditTerm: (id: number) => {
      setParams({ termId: id.toString() });
    },
    onCreateTerm: (yearId: number) => {
       setParams({ createTerm: true, yearId: yearId.toString() });
    },
    expandedYears,
    setExpandedYears,
    searchValue,
    setSearchValue,
  };

  const table = useReactTable({
    data: filteredData,
    getRowId: (row) => `${row.isTerm ? 'term' : 'year'}-${row.id}`,
    columns,
    getCoreRowModel: getCoreRowModel(),
    manualFiltering: true,
    meta: tableMeta,
  });

  return (
    <div className="w-full">
      <Header table={table} />

      <Table>
        <TableHeader>
          {table.getHeaderGroups().map((headerGroup) => (
            <TableRow key={headerGroup.id}>
              {headerGroup.headers.map((header) => (
                <TableHead key={header.id}>
                  {header.isPlaceholder
                    ? null
                    : flexRender(
                        header.column.columnDef.header,
                        header.getContext(),
                      )}
                </TableHead>
              ))}
            </TableRow>
          ))}
        </TableHeader>

        <TableBody>
          {table.getRowModel().rows.map((row) => (
            <TableRow
              className="hover:bg-muted/50 cursor-pointer"
              key={row.id}
              onClick={() => row.original.isTerm ? tableMeta.onEditTerm(row.original.id) : tableMeta.onEditYear(row.original.id)}
            >
              {row.getVisibleCells().map((cell, index) => (
                <TableCell
                  key={cell.id}
                  className={cn(index === 2 && "w-[50px]")}
                >
                  {flexRender(cell.column.columnDef.cell, cell.getContext())}
                </TableCell>
              ))}
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </div>
  );
}
