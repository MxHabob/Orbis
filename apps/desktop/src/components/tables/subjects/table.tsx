import { cn } from "@orbis/ui/cn";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@orbis/ui/table";
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
import { useSubjectParams } from "@/hooks/use-subject-params";
import { useTRPC } from "@/ipc/query-adapter";
import { columns, type SubjectsTableMeta } from "./columns";
import { Header } from "./header";

export function DataTable() {
  const trpc = useTRPC();
  const queryClient = useQueryClient();
  const { setParams } = useSubjectParams();

  const { data } = useSuspenseQuery(trpc.subjects.getAll.queryOptions());

  // Get search filter value
  const [searchValue, setSearchValue] = React.useState<string>("");

  const filteredData = React.useMemo(() => {
    if (!searchValue) return data;
    const searchLower = searchValue.toLowerCase();
    return data.filter(
      (subject) =>
        subject.name.toLowerCase().includes(searchLower) ||
        subject.code?.toLowerCase().includes(searchLower),
    );
  }, [data, searchValue]);

  const tableMeta: SubjectsTableMeta = {
    deleteSubject: (id: number) => {
      setParams({ deleteSubjectId: id.toString() });
    },
    onEditSubject: (id: number) => {
      setParams({ subjectId: id.toString() });
    },
    searchValue,
    setSearchValue,
  };

  const table = useReactTable({
    data: filteredData,
    getRowId: ({ id }) => id,
    columns,
    getCoreRowModel: getCoreRowModel(),
    meta: tableMeta,
  });

  return (
    <div className="w-full">
      <Header table={table} />

      <Table>
        <TableHeader>
          {table.getHeaderGroups().map((headerGroup) => (
            <TableRow key={headerGroup.id}>
              {headerGroup.headers.map((header) => {
                return (
                  <TableHead key={header.id}>
                    {header.isPlaceholder
                      ? null
                      : flexRender(
                          header.column.columnDef.header,
                          header.getContext(),
                        )}
                  </TableHead>
                );
              })}
            </TableRow>
          ))}
        </TableHeader>

        <TableBody>
          {table.getRowModel().rows.map((row) => (
            <TableRow
              className="hover:bg-muted/50 cursor-pointer"
              key={row.id}
              onClick={() =>
                setParams({ subjectId: row.original.id.toString() })
              }
            >
              {row.getVisibleCells().map((cell, index) => (
                <TableCell
                  key={cell.id}
                  className={cn(index === 3 && "w-[50px]")}
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
