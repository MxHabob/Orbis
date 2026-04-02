"use client";

import { Checkbox } from "@midday/ui/checkbox";
import { Badge } from "@midday/ui/badge";
import { Icons } from "@midday/ui/icons";
import type { ColumnDef } from "@tanstack/react-table";
import { formatDate } from "date-fns";
import { ActionsMenu } from "./actions-menu";
import type { CertificateTemplate } from "./types";

export const columns: ColumnDef<CertificateTemplate>[] = [
  {
    id: "select",
    size: 50,
    minSize: 50,
    maxSize: 50,
    enableResizing: false,
    enableHiding: false,
    enableSorting: false,
    meta: {
      sticky: true,
      skeleton: { type: "checkbox" },
      className:
        "w-[50px] min-w-[50px] md:sticky md:inset-inline-start-0 bg-background group-hover:bg-[#F2F1EF] group-hover:dark:bg-[#0f0f0f] z-20 justify-center !border-inline-end !border-border p-0",
    },
    cell: ({ row }) => (
      <Checkbox
        checked={row.getIsSelected()}
        onCheckedChange={(checked) => {
          if (checked === "indeterminate") {
            row.toggleSelected();
          } else {
            row.toggleSelected(checked);
          }
        }}
      />
    ),
  },
  {
    id: "name",
    accessorKey: "name",
    size: 300,
    minSize: 200,
    maxSize: 500,
    enableResizing: true,
    meta: {
      sticky: true,
      skeleton: { type: "text", width: "w-48" },
      sortField: "name",
      className:
        "w-[300px] min-w-[200px] md:sticky md:inset-inline-start-[50px] bg-background group-hover:bg-[#F2F1EF] group-hover:dark:bg-[#0f0f0f] z-20 !border-inline-end !border-border",
    },
    cell: ({ row }) => {
      const name = row.original.name;
      const isDefault = row.original.isDefault;
      
      return (
        <div className="flex items-center gap-2 min-w-0">
          <div className="h-8 w-8 rounded border bg-muted/50 flex items-center justify-center shrink-0">
             <Icons.Files className="w-4 h-4 text-muted-foreground" />
          </div>
          <div className="flex flex-col min-w-0">
            <span className="truncate font-medium">{name}</span>
            {isDefault && (
              <Badge variant="secondary" className="w-fit text-[9px] font-bold uppercase py-0 px-1.5 h-4 mt-1">
                Default
              </Badge>
            )}
          </div>
        </div>
      );
    },
  },
  {
    id: "size",
    accessorKey: "size",
    size: 100,
    minSize: 80,
    maxSize: 150,
    enableResizing: true,
    meta: {
      skeleton: { type: "text", width: "w-12" },
      className: "w-[100px] min-w-[80px]",
    },
    cell: ({ row }) => {
      return <span className="uppercase text-xs font-medium">{row.original.size}</span>;
    },
  },
  {
    id: "orientation",
    accessorKey: "orientation",
    size: 120,
    minSize: 100,
    maxSize: 180,
    enableResizing: true,
    meta: {
      skeleton: { type: "text", width: "w-16" },
      className: "w-[120px] min-w-[100px]",
    },
    cell: ({ row }) => {
      const orientation = row.original.orientation;
      return <span className="capitalize text-xs">{orientation}</span>;
    },
  },
  {
    id: "updatedAt",
    accessorKey: "updatedAt",
    size: 180,
    minSize: 150,
    maxSize: 220,
    enableResizing: true,
    meta: {
      skeleton: { type: "text", width: "w-24" },
      className: "w-[180px] min-w-[150px]",
    },
    cell: ({ row }) => {
      return (
        <span className="text-xs text-muted-foreground">
          {formatDate(new Date(row.original.updatedAt), "PPP")}
        </span>
      );
    },
  },
  {
    id: "actions",
    header: "",
    size: 80,
    minSize: 80,
    maxSize: 80,
    enableResizing: false,
    enableHiding: false,
    meta: {
      sticky: true,
      skeleton: { type: "icon" },
      className:
        "w-[80px] min-w-[80px] md:sticky md:inset-inline-end-0 bg-background group-hover:bg-[#F2F1EF] group-hover:dark:bg-[#0f0f0f] z-30 justify-center !border-inline-start !border-border",
    },
    cell: ({ row }) => {
      return <ActionsMenu row={row.original} />;
    },
  },
];
