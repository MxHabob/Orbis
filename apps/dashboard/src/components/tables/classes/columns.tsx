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
import { ChevronDown, ChevronRight } from "lucide-react";
import type { Dispatch, SetStateAction } from "react";

export interface ClassesTableMeta {
  deleteClass: (id: number) => void;
  onEditClass: (id: number) => void;
  expandedClasses: Set<number>;
  setExpandedClasses: Dispatch<SetStateAction<Set<number>>>;
  searchValue?: string;
  setSearchValue?: Dispatch<SetStateAction<string>>;
}

export type Class = RouterOutputs["classes"]["getAll"];

// Flatten classes to include both parents and children with hierarchy info
export function flattenClasses(classes: any[]): any[] {
  const flattened: any[] = [];

  for (const cls of classes) {
    // Add parent class
    flattened.push({
      ...cls,
      isChild: false,
      hasChildren: cls.children && cls.children.length > 0,
    });

    // Add children if they exist
    if (cls.children && cls.children.length > 0) {
      for (const child of cls.children) {
        flattened.push({
          ...child,
          isChild: true,
          parentId: cls.id,
          hasChildren: false,
        });
      }
    }
  }

  return flattened;
}

export const columns: ColumnDef<any>[] = [
  {
    header: "Name",
    accessorKey: "name",
    cell: ({ row, table }) => {
      // Get expanded state from table meta
      const meta = table.options.meta as unknown as ClassesTableMeta;
      const expandedClasses = meta?.expandedClasses || new Set();
      const setExpandedClasses = meta?.setExpandedClasses;

      const isExpanded = expandedClasses.has(row.original.id);
      const hasChildren = row.original.hasChildren;
      const isChild = row.original.isChild;

      const toggleExpanded = () => {
        if (!setExpandedClasses) return;

        const newExpanded = new Set(expandedClasses);
        if (isExpanded) {
          newExpanded.delete(row.original.id);
        } else {
          newExpanded.add(row.original.id);
        }
        setExpandedClasses(newExpanded);
      };

      return (
        <div className={cn("flex space-x-2 items-center", isChild && "ml-10")}>
          {hasChildren && !isChild && (
            <Button
              variant="ghost"
              size="sm"
              className="h-4 w-4 p-0 hover:bg-transparent"
              onClick={(e) => {
                e.stopPropagation();
                toggleExpanded();
              }}
            >
              {isExpanded ? (
                <ChevronDown className="h-3 w-3" />
              ) : (
                <ChevronRight className="h-3 w-3" />
              )}
            </Button>
          )}
          {!hasChildren && !isChild && <div className="w-4" />}
          <div
            className="size-3 rounded-sm border"
            style={{ backgroundColor: row.original.color || "#e6e6e6" }}
          />
          <span
            className={cn(
              "font-medium",
              hasChildren && !isChild ? "cursor-pointer" : "cursor-default",
            )}
            onClick={
              hasChildren && !isChild
                ? (e) => {
                    e.stopPropagation();
                    toggleExpanded();
                  }
                : undefined
            }
          >
            {row.getValue("name")}
          </span>
        </div>
      );
    },
  },
  {
    header: "Division",
    accessorKey: "division",
    cell: ({ row }) => (
      <span className="text-muted-foreground">{row.getValue("division") || "-"}</span>
    ),
  },
  {
    header: "Type",
    accessorKey: "type",
    cell: ({ row }) => {
      const type = row.getValue("type") as string | null;
      return (
        <span className="capitalize text-muted-foreground">{type?.replace(/_/g, ' ') || "-"}</span>
      );
    },
  },
  {
    header: "Year",
    accessorKey: "yearName",
    cell: ({ row }) => (
       <span className="text-muted-foreground">{row.original.yearName || "-"}</span>
    ),
  },
  {
    id: "actions",
    cell: ({ row, table }) => {
      const meta = table.options.meta as unknown as ClassesTableMeta;

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
              <DropdownMenuItem onClick={() => meta?.onEditClass?.(row.original.id)}>
                Edit
              </DropdownMenuItem>
              <DropdownMenuItem
                  className="text-destructive"
                  onClick={() => meta?.deleteClass?.(row.original.id)}
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
