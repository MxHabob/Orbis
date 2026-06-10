"use client";

import { useI18n } from "@/locales/client";
import { Button } from "@orbis/ui/button";
import { cn } from "@orbis/ui/cn";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@orbis/ui/dropdown-menu";
import { DotsHorizontalIcon } from "@radix-ui/react-icons";
import type { ColumnDef } from "@tanstack/react-table";
import { ChevronDown, ChevronRight, Calendar } from "lucide-react";
import type { Dispatch, SetStateAction } from "react";
import { Badge } from "@orbis/ui/badge";

export interface AcademicTableMeta {
  deleteYear: (id: number) => void;
  deleteTerm: (id: number) => void;
  onEditYear: (id: number) => void;
  onEditTerm: (id: number) => void;
  onCreateTerm: (yearId: number) => void;
  expandedYears: Set<number>;
  setExpandedYears: Dispatch<SetStateAction<Set<number>>>;
  searchValue?: string;
  setSearchValue?: Dispatch<SetStateAction<string>>;
}

export function flattenAcademicYears(years: any[]): any[] {
  const flattened: any[] = [];

  for (const year of years) {
    flattened.push({
      ...year,
      isTerm: false,
      hasTerms: year.terms && year.terms.length > 0,
    });

    if (year.terms && year.terms.length > 0) {
      for (const term of year.terms) {
        flattened.push({
          ...term,
          isTerm: true,
          parentYearId: year.id,
          hasTerms: false,
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
      const meta = table.options.meta as unknown as AcademicTableMeta;
      const t = useI18n();
      const expandedYears = meta?.expandedYears || new Set();
      const setExpandedYears = meta?.setExpandedYears;

      const isExpanded = expandedYears.has(row.original.id);
      const isTerm = row.original.isTerm;
      const hasTerms = row.original.hasTerms;
      const name = isTerm ? row.original.name : row.original.yearName;

      const toggleExpanded = () => {
        if (!setExpandedYears) return;
        const newExpanded = new Set(expandedYears);
        if (isExpanded) {
          newExpanded.delete(row.original.id);
        } else {
          newExpanded.add(row.original.id);
        }
        setExpandedYears(newExpanded);
      };

      return (
        <div className={cn("flex space-x-2 items-center", isTerm && "ms-10")}>
          {hasTerms && !isTerm && (
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
          {!hasTerms && !isTerm && <div className="w-4" />}
          
          <div className={cn(
             "p-1 rounded-sm",
             isTerm ? "bg-primary/10 text-primary" : "bg-muted text-muted-foreground"
          )}>
            <Calendar className="size-3" />
          </div>

          <div className="flex items-center gap-2">
            <span className="font-medium">{name}</span>
            {!isTerm && row.original.isCurrent && (
              <Badge variant="outline" className="text-[10px] py-0 h-4 bg-primary/10 text-primary border-primary/20">
                {t("education.current")}
              </Badge>
            )}
          </div>
        </div>
      );
    },
  },
  {
    header: "Dates",
    accessorKey: "dates",
    cell: ({ row }) => {
      const startDate = new Date(row.original.startDate).toLocaleDateString();
      const endDate = new Date(row.original.endDate).toLocaleDateString();
      return (
        <span className="text-muted-foreground text-sm">
          {startDate} - {endDate}
        </span>
      );
    },
  },
  {
    id: "actions",
    cell: ({ row, table }) => {
      const meta = table.options.meta as unknown as AcademicTableMeta;
      const isTerm = row.original.isTerm;
      const t = useI18n();

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
              <DropdownMenuItem onClick={() => isTerm ? meta?.onEditTerm?.(row.original.id) : meta?.onEditYear?.(row.original.id)}>
                {t("account.edit")}
              </DropdownMenuItem>
              {!isTerm && (
                <DropdownMenuItem onClick={() => meta?.onCreateTerm?.(row.original.id)}>
                  {t("settings.add_term")}
                </DropdownMenuItem>
              )}
              <DropdownMenuItem
                  className="text-destructive"
                  onClick={() => isTerm ? meta?.deleteTerm?.(row.original.id) : meta?.deleteYear?.(row.original.id)}
                >
                  {t("common.delete")}
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        </div>
      );
    },
  },
];
