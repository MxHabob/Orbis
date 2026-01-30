"use client";

import { useStudentParams } from "@/hooks/use-student-params";
import type { RouterOutputs } from "@api/trpc/routers/_app";
import { Avatar, AvatarFallback, AvatarImageNext } from "@midday/ui/avatar";
import { Badge } from "@midday/ui/badge";
import { Button } from "@midday/ui/button";
import { Icons } from "@midday/ui/icons";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@midday/ui/dropdown-menu";
import type { ColumnDef } from "@tanstack/react-table";
import { format } from "date-fns";
import { memo, useCallback } from "react";

export type Student = RouterOutputs["students"]["getByClassId"]["data"][number];

const NameCell = memo(
  ({
    name,
    image,
  }: {
    name: string | null;
    image: string | null;
  }) => {
    const displayName = name ?? "-";

    return (
      <div className="flex items-center space-x-2">
        <Avatar className="size-5">
          {image && (
            <AvatarImageNext
              src={image}
              alt={`${displayName} avatar`}
              width={20}
              height={20}
              quality={100}
            />
          )}
          <AvatarFallback className="text-[9px] font-medium">
            {displayName[0]}
          </AvatarFallback>
        </Avatar>
        <span className="truncate">{displayName}</span>
      </div>
    );
  },
);

NameCell.displayName = "NameCell";

const ActionsCell = memo(
  ({
    studentId,
    onDelete,
  }: {
    studentId: string;
    onDelete?: (id: string) => void;
  }) => {
    const { setParams } = useStudentParams();

    const handleEdit = useCallback(() => {
      setParams({ studentId: studentId });
    }, [studentId, setParams]);

    const handleDelete = useCallback(() => {
      onDelete?.(studentId);
    }, [studentId, onDelete]);

    return (
      <div className="flex items-center justify-center w-full">
        <DropdownMenu>
          <DropdownMenuTrigger asChild className="relative">
            <Button variant="ghost" className="h-8 w-8 p-0">
              <Icons.MoreHoriz className="h-4 w-4" />
            </Button>
          </DropdownMenuTrigger>

          <DropdownMenuContent align="end">
            <DropdownMenuItem onClick={handleEdit}>
              Edit student
            </DropdownMenuItem>

            <DropdownMenuItem onClick={handleDelete} className="text-[#FF3638]">
              Delete
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      </div>
    );
  },
);

ActionsCell.displayName = "ActionsCell";

export const columns: ColumnDef<Student>[] = [
  {
    id: "name",
    accessorKey: "name",
    header: "Name",
    size: 250,
    minSize: 200,
    maxSize: 400,
    enableResizing: true,
    meta: {
      sticky: true,
      skeleton: { type: "avatar-text", width: "w-32" },
      headerLabel: "Name",
      className:
        "w-[250px] min-w-[200px] md:sticky md:left-0 bg-background group-hover:bg-[#F2F1EF] group-hover:dark:bg-[#0f0f0f] z-20",
    },
    cell: ({ row }) => (
      <NameCell 
        name={row.original.name} 
        image={row.original.image} 
      />
    ),
  },
  {
    id: "studentNumber",
    accessorKey: "studentNumber",
    header: "Student ID",
    size: 150,
    minSize: 100,
    maxSize: 200,
    enableResizing: true,
    meta: {
      skeleton: { type: "text", width: "w-24" },
      headerLabel: "Student ID",
      className: "w-[150px] min-w-[100px]",
    },
    cell: ({ row }) => (
      <Badge variant="outline" className="font-mono">
        {row.original.studentNumber}
      </Badge>
    ),
  },
  {
    id: "class",
    accessorKey: "currentClass.name",
    header: "Class",
    size: 150,
    minSize: 100,
    maxSize: 200,
    enableResizing: true,
    meta: {
      skeleton: { type: "text", width: "w-20" },
      headerLabel: "Class",
      className: "w-[150px] min-w-[100px]",
    },
    cell: ({ row }) => row.original.currentClass?.name ?? "-",
  },
  {
    id: "status",
    accessorKey: "status",
    header: "Status",
    size: 120,
    minSize: 100,
    maxSize: 150,
    enableResizing: true,
    meta: {
      skeleton: { type: "text", width: "w-16" },
      headerLabel: "Status",
      className: "w-[120px] min-w-[100px]",
    },
    cell: ({ row }) => {
      const status = row.original.status;
      return (
        <Badge 
          variant={status === "active" ? "success" : "secondary"}
          className="capitalize"
        >
          {status}
        </Badge>
      );
    },
  },
  {
    id: "sex",
    accessorKey: "sex",
    header: "Sex",
    size: 100,
    minSize: 80,
    maxSize: 120,
    enableResizing: true,
    meta: {
      skeleton: { type: "text", width: "w-12" },
      headerLabel: "Sex",
      className: "w-[100px] min-w-[80px]",
    },
    cell: ({ row }) => <span className="capitalize">{row.original.sex}</span>,
  },
  {
    id: "enrollmentDate",
    accessorKey: "enrollmentDate",
    header: "Enrolled At",
    size: 150,
    minSize: 120,
    maxSize: 200,
    enableResizing: true,
    meta: {
      skeleton: { type: "text", width: "w-24" },
      headerLabel: "Enrolled At",
      className: "w-[150px] min-w-[120px]",
    },
    cell: ({ row }) => {
      const date = row.original.enrollmentDate;
      if (!date) return "-";
      return format(new Date(date), "MMM d, yyyy");
    },
  },
  {
    id: "actions",
    header: "Actions",
    size: 100,
    minSize: 100,
    maxSize: 100,
    enableResizing: false,
    enableSorting: false,
    enableHiding: false,
    meta: {
      sticky: true,
      skeleton: { type: "icon" },
      headerLabel: "Actions",
      className:
        "text-right sticky right-0 bg-background group-hover:bg-[#F2F1EF] group-hover:dark:bg-[#0f0f0f] z-30 justify-center !border-l !border-border",
    },
    cell: ({ row, table }) => (
      <ActionsCell
        studentId={row.original.userId}
        onDelete={table.options.meta?.deleteCustomer}
      />
    ),
  },
];
