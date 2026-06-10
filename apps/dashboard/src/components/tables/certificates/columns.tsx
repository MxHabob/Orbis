"use client";

import { Checkbox } from "@orbis/ui/checkbox";
import { Badge } from "@orbis/ui/badge";
import { Icons } from "@orbis/ui/icons";
import type { ColumnDef } from "@tanstack/react-table";
import { formatDate } from "date-fns";
import { ActionsMenu } from "./actions-menu";
import type { CertificateTemplate } from "./types";
import { cn } from "@orbis/ui/cn";

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
        "w-[50px] min-w-[50px] md:sticky md:inset-inline-start-0 bg-background group-hover:bg-accent/40 z-20 justify-center !border-inline-end !border-border p-0",
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
    size: 350,
    minSize: 200,
    maxSize: 500,
    enableResizing: true,
    meta: {
      sticky: true,
      skeleton: { type: "text", width: "w-48" },
      sortField: "name",
      className:
        "w-[350px] min-w-[200px] md:sticky md:inset-inline-start-[50px] bg-background group-hover:bg-accent/40 z-20 !border-inline-end !border-border",
    },
    cell: ({ row }) => {
      const name = row.original.name;
      const isDefault = row.original.isDefault;
      const thumbnailUrl = row.original.thumbnailUrl;
      
      return (
        <div className="flex items-center gap-3 min-w-0">
          <div className="h-10 w-10 rounded border bg-accent/20 flex items-center justify-center shrink-0 overflow-hidden">
             {thumbnailUrl ? (
                <img src={thumbnailUrl} alt="" className="w-full h-full object-cover" />
             ) : (
                <Icons.Files className="w-4 h-4 text-muted-foreground opacity-50" />
             )}
          </div>
          <div className="flex flex-col min-w-0">
            <span className="truncate font-bold text-sm">{name}</span>
            <div className="flex items-center gap-1.5 mt-0.5">
               {isDefault && (
                 <Badge variant="secondary" className="text-[8px] font-bold uppercase py-0 px-1.5 h-3.5 bg-blue-500/10 text-blue-500 border-none">
                   Default
                 </Badge>
               )}
               <span className="text-[10px] text-muted-foreground uppercase font-mono">{row.original.category}</span>
            </div>
          </div>
        </div>
      );
    },
  },
  {
    id: "category",
    accessorKey: "category",
    size: 150,
    minSize: 100,
    maxSize: 200,
    enableResizing: true,
    meta: {
      skeleton: { type: "text", width: "w-20" },
      className: "w-[150px] min-w-[100px]",
    },
    cell: ({ row }) => {
      return (
         <div className="flex items-center gap-2">
            <div className={cn(
               "w-1.5 h-1.5 rounded-full",
               row.original.isArchived ? "bg-gray-400" : "bg-green-500"
            )} />
            <span className="text-[11px] font-bold uppercase tracking-tight text-muted-foreground">
               {row.original.isArchived ? "Archived" : "Live"}
            </span>
         </div>
      );
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
        <span className="text-xs text-muted-foreground font-medium">
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
        "w-[80px] min-w-[80px] md:sticky md:inset-inline-end-0 bg-background group-hover:bg-accent/40 z-30 justify-center !border-inline-start !border-border",
    },
    cell: ({ row }) => {
      return <ActionsMenu row={row.original} />;
    },
  },
];
