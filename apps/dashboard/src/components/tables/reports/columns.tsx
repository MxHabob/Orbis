"use client";

import { Avatar, AvatarFallback } from "@midday/ui/avatar";
import { Checkbox } from "@midday/ui/checkbox";
import { cn } from "@midday/ui/cn";
import type { RouterOutputs } from "@api/trpc/routers/_app";
import type { ColumnDef } from "@tanstack/react-table";
import { ActionsMenu } from "./actions-menu";

export type ClassReport = NonNullable<
  RouterOutputs["classes"]["getAll"]["data"]
>[number];

export const columns: ColumnDef<ClassReport>[] = [
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
        "w-[50px] min-w-[50px] md:sticky md:left-0 bg-background group-hover:bg-[#F2F1EF] group-hover:dark:bg-[#0f0f0f] z-20",
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
    header: "Class Name",
    accessorKey: "name",
    size: 250,
    minSize: 180,
    maxSize: 400,
    enableResizing: true,
    meta: {
      sticky: true,
      skeleton: { type: "avatar-text", width: "w-32" },
      headerLabel: "Class Name",
      sortField: "name",
      className:
        "w-[250px] min-w-[180px] md:sticky md:left-[50px] bg-background group-hover:bg-[#F2F1EF] group-hover:dark:bg-[#0f0f0f] z-20",
    },
    cell: ({ row }) => {
      const name = row.original.name;
      
      return (
        <div className="flex items-center space-x-2 min-w-0">
          <Avatar className="size-5 flex-shrink-0">
            <AvatarFallback className="text-[9px] font-medium">
              {name?.[0]?.toUpperCase()}
            </AvatarFallback>
          </Avatar>
          <span className="truncate font-medium">{name}</span>
        </div>
      );
    },
  },
  {
    id: "type",
    header: "Type",
    accessorKey: "type",
    size: 150,
    minSize: 120,
    maxSize: 200,
    enableResizing: true,
    meta: {
      skeleton: { type: "text", width: "w-24" },
      headerLabel: "Type",
      className: "w-[150px] min-w-[120px]",
    },
    cell: ({ row }) => {
      const type = row.original.type;
      return <span className="capitalize">{type.replace("_", " ")}</span>;
    },
  },
  {
    id: "division",
    header: "Division",
    accessorKey: "division",
    size: 120,
    minSize: 100,
    maxSize: 180,
    enableResizing: true,
    meta: {
      skeleton: { type: "text", width: "w-16" },
      headerLabel: "Division",
      className: "w-[120px] min-w-[100px]",
    },
    cell: ({ row }) => {
      return <span>{row.original.division || "-"}</span>;
    },
  },
  {
    id: "studentCount",
    header: "Students",
    accessorKey: "_count.students",
    size: 100,
    minSize: 80,
    maxSize: 150,
    enableResizing: true,
    meta: {
      skeleton: { type: "text", width: "w-12" },
      headerLabel: "Students",
      className: "w-[100px] min-w-[80px]",
    },
    cell: ({ row }) => {
      const count = row.original._count?.students ?? 0;
      return (
        <span className="truncate">
          {count}
        </span>
      );
    },
  },
  {
    id: "scheduleCount",
    header: "Schedule",
    accessorKey: "_count.schedule",
    size: 100,
    minSize: 80,
    maxSize: 150,
    enableResizing: true,
    meta: {
      skeleton: { type: "text", width: "w-12" },
      headerLabel: "Schedule",
      className: "w-[100px] min-w-[80px]",
    },
    cell: ({ row }) => {
      const count = row.original._count?.schedule ?? 0;
      return (
        <span className="truncate">
          {count}
        </span>
      );
    },
  },
  {
    id: "parentClass",
    header: "Parent Class",
    accessorKey: "parentClass.name",
    size: 180,
    minSize: 140,
    maxSize: 280,
    enableResizing: true,
    meta: {
      skeleton: { type: "text", width: "w-24" },
      headerLabel: "Parent Class",
      className: "w-[180px] min-w-[140px]",
    },
    cell: ({ row }) => {
      const parentClass = row.original.parentClass?.name;
      return (
        <span className="truncate text-muted-foreground">
          {parentClass || "-"}
        </span>
      );
    },
  },
  {
    id: "academicYear",
    header: "Academic Year",
    accessorKey: "yearName",
    size: 150,
    minSize: 120,
    maxSize: 200,
    enableResizing: true,
    meta: {
      skeleton: { type: "text", width: "w-24" },
      headerLabel: "Academic Year",
      className: "w-[150px] min-w-[120px]",
    },
    cell: ({ row }) => {
      return <span>{row.original.yearName || "-"}</span>;
    },
  },
  {
    id: "actions",
    header: "Actions",
    size: 100,
    minSize: 80,
    maxSize: 100,
    enableResizing: false,
    enableHiding: false,
    meta: {
      sticky: true,
      skeleton: { type: "icon" },
      headerLabel: "Actions",
      className:
        "w-[100px] min-w-[80px] md:sticky md:right-0 bg-background group-hover:bg-[#F2F1EF] group-hover:dark:bg-[#0f0f0f] z-30 justify-center !border-l !border-border",
    },
    cell: ({ row }) => {
      return <ActionsMenu row={row.original} />;
    },
  },
];
