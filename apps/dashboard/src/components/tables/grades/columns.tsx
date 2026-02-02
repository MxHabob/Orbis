"use client";

import { Avatar, AvatarFallback, AvatarImageNext } from "@midday/ui/avatar";
import type { ColumnDef } from "@tanstack/react-table";
import { memo } from "react";
import { EditableGradeCell } from "./editable-grade-cell";

export type GradeStudent = {
  studentId: string;
  name: string | null;
  studentNumber: string | null;
  image: string | null;
};

export type AssessmentItem = {
  id: number;
  title: string;
  maxScore: number;
};

export type Grade = {
  studentId: string;
  assessmentItemId: number;
  score: string | null;
};

export type GradeGridData = {
  students: GradeStudent[];
  items: AssessmentItem[];
  grades: Grade[];
};

export type GradeTableMeta = {
  items: AssessmentItem[];
  getGradeValue: (studentId: string, assessmentItemId: number) => string | null;
  onGradeChange?: (
    studentId: string,
    assessmentItemId: number,
    value: string | null
  ) => void;
};

const StudentNameCell = memo(
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
  }
);

StudentNameCell.displayName = "StudentNameCell";

export const createGradeColumns = (
  items: AssessmentItem[]
): ColumnDef<GradeStudent>[] => {
  const columns: ColumnDef<GradeStudent>[] = [
    {
      id: "studentName",
      accessorKey: "name",
      header: "Student Name",
      size: 250,
      minSize: 200,
      maxSize: 400,
      enableResizing: true,
      meta: {
        sticky: true,
        skeleton: { type: "avatar-text", width: "w-40" },
        headerLabel: "Student Name",
        className:
          "w-[250px] min-w-[200px] sticky left-0 bg-background group-hover:bg-[#F2F1EF] group-hover:dark:bg-[#0f0f0f] z-20 border-r",
      },
      cell: ({ row }) => (
        <StudentNameCell
          name={row.original.name}
          image={row.original.image}
        />
      ),
    },
  ];

  // Add dynamic columns for each assessment item
  items.forEach((item) => {
    columns.push({
      id: `assessment-${item.id}`,
      accessorKey: `assessment-${item.id}`,
      header: () => (
        <div className="flex flex-col items-center gap-0.5">
          <span className="font-bold text-sm tracking-tight">{item.title}</span>
          <span className="text-[10px] uppercase text-muted-foreground font-medium">
            Max: {item.maxScore}
          </span>
        </div>
      ),
      size: 140,
      minSize: 120,
      maxSize: 180,
      enableResizing: true,
      meta: {
        skeleton: { type: "text", width: "w-16" },
        headerLabel: item.title,
        className: "text-center min-w-[140px] border-r",
      },
      cell: ({ row, table }) => {
        const gradeValue = table.options.meta?.getGradeValue?.(
          row.original.studentId,
          item.id
        ) ?? null;
        
        return (
          <EditableGradeCell
            studentId={row.original.studentId}
            assessmentItemId={item.id}
            initialValue={gradeValue}
            maxScore={item.maxScore}
          />
        );
      },
    });
  });

  // Add total column
  columns.push({
    id: "total",
    accessorKey: "total",
    header: "Total",
    size: 120,
    minSize: 100,
    maxSize: 150,
    enableResizing: false,
    enableSorting: false,
    meta: {
      sticky: true,
      skeleton: { type: "text", width: "w-16" },
      headerLabel: "Total",
      className:
        "text-center sticky right-0 bg-primary/5 group-hover:bg-primary/10 z-20 border-l font-bold",
    },
    cell: ({ row, table }) => {
      let total = 0;

      items.forEach((item) => {
        const gradeValue = table.options.meta?.getGradeValue?.(
          row.original.studentId,
          item.id
        );
        if (gradeValue) {
          total += parseFloat(gradeValue);
        }
      });

      return (
        <div className="flex items-center justify-center h-full">
          <span className="text-lg font-black text-primary tabular-nums">
            {total}
          </span>
        </div>
      );
    },
  });

  return columns;
};