"use client";

import type { RouterOutputs } from "@api/trpc/routers/_app";
import { Button } from "@midday/ui/button";
import { cn } from "@midday/ui/cn";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@midday/ui/dropdown-menu";

import { DotsHorizontalIcon } from "@radix-ui/react-icons";
import type { ColumnDef } from "@tanstack/react-table";
import type { Dispatch, SetStateAction } from "react";

export interface SubjectsTableMeta {
  deleteSubject: (id: number) => void;
  onEditSubject: (id: number) => void;
  searchValue?: string;
  setSearchValue?: Dispatch<SetStateAction<string>>;
}

export type Subject = RouterOutputs["subjects"]["getAll"]

export const columns: ColumnDef<any>[] = [
  {
    header: "Name",
    accessorKey: "name",
    cell: ({ row }) => {
      return (
        <div className="flex space-x-2 items-center">
          <div
            className="size-3 rounded-sm border"
            style={{ backgroundColor: row.original?.color || "#e6e6e6" }}
          />
          <span className="font-medium">
            {row.getValue("name")}
          </span>
        </div>
      );
    },
  },
  {
    header: "Code",
    accessorKey: "code",
    cell: ({ row }) => (
      <span className="text-muted-foreground">{row.getValue("code") || "-"}</span>
    ),
  },
  {
    id: "actions",
    cell: ({ row, table }) => {
      const meta = table.options.meta as unknown as SubjectsTableMeta;

      return (
        <div className="text-right">
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button
                variant="ghost"
                className="h-8 w-8 p-0"
                onClick={(e) => e.stopPropagation()}
              >
                <DotsHorizontalIcon className="h-4 w-4" />
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent
              align="end"
              onClick={(e) => e.stopPropagation()}
            >
              <DropdownMenuItem onClick={() => meta?.onEditSubject?.(row.original.id)}>
                Edit
              </DropdownMenuItem>
              <DropdownMenuItem
                className="text-destructive"
                onClick={() => meta?.deleteSubject?.(row.original.id)}
              >
                Delete
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        </div>
      );
    },
  },
];
