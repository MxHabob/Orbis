"use client";

import { useI18n } from "@/locales/client";
import type { RouterOutputs } from "@api/trpc/routers/_app";
import { Button } from "@midday/ui/button";
import { cn } from "@midday/ui/cn";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@midday/ui/dropdown-menu";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@midday/ui/tooltip";

import { DotsHorizontalIcon } from "@radix-ui/react-icons";
import type { ColumnDef } from "@tanstack/react-table";
import type { Dispatch, SetStateAction } from "react";

export interface SubjectsTableMeta {
  deleteSubject: (id: number) => void;
  onEditSubject: (id: number) => void;
  expandedSubjects: Set<number>;
  setExpandedSubjects: Dispatch<SetStateAction<Set<number>>>;
  searchValue?: string;
  setSearchValue?: Dispatch<SetStateAction<string>>;
}

export type Subject = RouterOutputs["subjects"]["getAll"]

// Check if a category should show a tooltip
function shouldShowSubjectTooltip(subject: any): boolean {
  // Show tooltip if category has a user-defined description
  if (subject.description) {
    return true;
  }
  // Show tooltip for system categories (they have localized descriptions)
  if (subject.system) {
    return true;
  }
  // Don't show tooltip for user-created categories without descriptions
  return false;
}

// Component to display category description from localization
function SubjectTooltip({ subject }: { subject: any }) {
  const t = useI18n();

  // Priority 1: User-defined description
  if (subject.description) {
    return <span>{subject.description}</span>;
  }

  // Priority 2: System description from localization (only for system categories)
  if (subject.system) {
    try {
      return (
        // @ts-expect-error - slug is not nullable
        <span>{t(`transaction_categories.${subject.slug}`)}</span>
      );
    } catch {
      return <span>Category description not available</span>;
    }
  }

  return null;
}

export const columns: ColumnDef<any>[] = [
  {
    header: "Name",
    accessorKey: "name",
    cell: ({ row, table }) => {
      const meta = table.options.meta as unknown as SubjectsTableMeta;
      console.log("data", row.original);
      return (
        <div className={cn("flex space-x-2 items-center ")}>
          <div
            className="size-3"
            style={{ backgroundColor: row.original.color ? `#${row.original.color}` : undefined }}
          />
          {shouldShowSubjectTooltip(row.original) ? (
            <TooltipProvider delayDuration={0}>
              <Tooltip>
                <TooltipTrigger asChild>
                  <span
                    className={cn(
                      "cursor-pointer",
                    )}
                  >
                    {row.getValue("name")}
                  </span>
                </TooltipTrigger>
                <TooltipContent
                  className="px-3 py-1.5 text-xs"
                  side="right"
                  sideOffset={10}
                >
                  <SubjectTooltip subject={row.original} />
                </TooltipContent>
              </Tooltip>
            </TooltipProvider>
          ) : (
            <span
              className={cn(
                "cursor-pointer",
              )}
            >
              {row.getValue("name")}
            </span>
          )}

          {row.original.name && (
            <div className="pl-2">
              <span className="border border-border rounded-full py-1 px-2 text-[10px] text-[#878787] font-mono">
                System
              </span>
            </div>
          )}
        </div>
      );
    },
  },
  {
    header: "Code",
    accessorKey: "code",
    cell: ({ row }) => row.getValue("code") || "-",
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

              {!row.original.system && (
                <DropdownMenuItem
                  onClick={() => meta?.deleteSubject?.(row.original.id)}
                >
                  Remove
                </DropdownMenuItem>
              )}
            </DropdownMenuContent>
          </DropdownMenu>
        </div>
      );
    },
  },
];
