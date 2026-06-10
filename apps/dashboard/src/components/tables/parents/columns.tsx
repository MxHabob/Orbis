import { useScopedI18n } from "@/locales/client";
import type { RouterOutputs } from "@api/trpc/routers/_app";
import { Avatar, AvatarFallback } from "@orbis/ui/avatar";
import { Button } from "@orbis/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@orbis/ui/dropdown-menu";
import type { ColumnDef, FilterFn, Row } from "@tanstack/react-table";
import { MoreHorizontal } from "lucide-react";

type ParentUser = RouterOutputs["user"]["getParents"][number];

const userFilterFn: FilterFn<ParentUser> = (
  row: Row<ParentUser>,
  _: string,
  filterValue: string,
) => {
  const name = row.original.name?.toLowerCase();
  const email = row.original.email?.toLowerCase();
  const filter = filterValue.toLowerCase();
  return (name?.includes(filter) || email?.includes(filter)) ?? false;
};

export const columns: ColumnDef<ParentUser>[] = [
  {
    id: "user",
    accessorKey: "email",
    filterFn: userFilterFn,
    cell: ({ row }) => {
      return (
        <div className="flex items-center space-x-4">
          <Avatar className="rounded-full w-8 h-8">
            <AvatarFallback>
              <span className="text-xs">
                {row.original.email?.slice(0, 1)?.toUpperCase() ?? "P"}
              </span>
            </AvatarFallback>
          </Avatar>
          <div className="flex flex-col">
            <span className="font-medium text-sm">{row.original.name || "Parent User"}</span>
            <span className="text-sm text-[#606060]">{row.original.email}</span>
          </div>
        </div>
      );
    },
  },
  {
    id: "actions",
    cell: ({ row, table }) => {
      const tRoles = useScopedI18n("roles");
      const meta = table.options.meta;

      return (
        <div className="flex justify-end">
          <div className="flex space-x-2 items-center">
            <span className="text-[#606060]">
              {tRoles((row.original.role as any) || "member")}
            </span>

            <DropdownMenu>
              <DropdownMenuTrigger asChild>
                <Button variant="ghost" className="h-8 w-8 p-0">
                  <span className="sr-only">Open menu</span>
                  <MoreHorizontal className="h-4 w-4" />
                </Button>
              </DropdownMenuTrigger>
              <DropdownMenuContent align="end">
                <DropdownMenuItem
                  className="text-destructive"
                  onSelect={() =>
                    meta?.setParams?.({ deleteStaffId: row.original.id })
                  }
                >
                  Remove
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>
          </div>
        </div>
      );
    },
    meta: {
      className: "text-right",
    },
  },
];
