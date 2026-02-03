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
  month?: number;
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
  items: AssessmentItem[] = [],
  month?: number
): ColumnDef<GradeStudent>[] => {
  const isSummaryMode = month === 13 || month === 14;

  const columns: ColumnDef<GradeStudent>[] = [
    {
      id: "studentName",
      accessorKey: "name",
      header: "Student Name",
      size: 200,
      minSize: 150,
      maxSize: 300,
      enableResizing: true,
      meta: {
        sticky: true,
        skeleton: { type: "avatar-text", width: "w-40" },
        headerLabel: "Student Name",
        className:
          "w-[200px] min-w-[150px] sticky left-0 bg-background group-hover:bg-[#F2F1EF] group-hover:dark:bg-[#0f0f0f] z-20 border-r",
      },
      cell: ({ row }) => (
        <StudentNameCell
          name={row.original.name}
          image={row.original.image}
        />
      ),
    },
  ];

  if (isSummaryMode) {
    // Summary Mode Columns: Average (20) + Exam (30) = Total (50)
    const summaryMonths = month === 13 ? [9, 10, 11] : [12, 1, 2];
    const examMonth = month;

    // 1. Column for the 3-month average (scaled to 20)
    columns.push({
      id: "monthly-avg",
      header: () => (
        <div className="flex flex-col items-center gap-0.5 px-2">
          <span className="font-bold text-xs md:text-sm tracking-tight whitespace-nowrap">
            Monthly Avg
          </span>
          <span className="text-[9px] uppercase text-muted-foreground font-medium">
            Max: 20
          </span>
        </div>
      ),
      size: 130,
      minSize: 110,
      meta: {
        headerLabel: "Average (20)",
        className: "text-center border-r bg-muted/30",
      },
      cell: ({ row, table }) => {
        let totalScore = 0;
        let totalMax = 0;

        // Sum up all items from the 3 months
        items
          .filter((i) => summaryMonths.includes((i as any).month))
          .forEach((item) => {
            const score = table.options.meta?.getGradeValue?.(
              row.original.studentId,
              item.id
            );
            if (score) totalScore += parseFloat(score);
            totalMax += item.maxScore;
          });

        // Scale to 20: (Score / Max) * 20
        const scaled = totalMax > 0 ? (totalScore / totalMax) * 20 : 0;
        return (
          <span className="font-bold text-blue-600 dark:text-blue-400">
            {scaled.toFixed(2)}
          </span>
        );
      },
    });

    // 2. Column for the Exam (30)
    const examItem = items.find((i) => (i as any).month === examMonth);
    if (examItem) {
      columns.push({
        id: `assessment-${examItem.id}`,
        accessorKey: `assessment-${examItem.id}`,
        header: () => (
          <div className="flex flex-col items-center gap-0.5 px-2">
            <span className="font-bold text-xs md:text-sm tracking-tight whitespace-nowrap">
              {month === 13 ? "Mid-term Exam" : "Final Exam"}
            </span>
            <span className="text-[9px] uppercase text-muted-foreground font-medium">
              Max: 30
            </span>
          </div>
        ),
        size: 130,
        minSize: 110,
        meta: { headerLabel: "Exam (30)", className: "text-center border-r" },
        cell: ({ row, table }) => {
          const gradeValue =
            table.options.meta?.getGradeValue?.(
              row.original.studentId,
              examItem.id
            ) ?? null;
          return (
            <EditableGradeCell
              studentId={row.original.studentId}
              assessmentItemId={examItem.id}
              initialValue={gradeValue}
              maxScore={examItem.maxScore}
            />
          );
        },
      });

      // 3. Status Column
      columns.push({
        id: "status",
        header: "Status",
        size: 100,
        minSize: 80,
        meta: {
          headerLabel: "Status",
          sticky: true,
          className:
            "text-center border-l bg-primary/5 group-hover:bg-primary/10 z-20",
        },
        cell: ({ row, table }) => {
          let monthlyTotal = 0;
          let monthlyMax = 0;
          items
            .filter((i) => summaryMonths.includes((i as any).month))
            .forEach((item) => {
              const score = table.options.meta?.getGradeValue?.(
                row.original.studentId,
                item.id
              );
              if (score) monthlyTotal += parseFloat(score);
              monthlyMax += item.maxScore;
            });
          const monthlyScaled =
            monthlyMax > 0 ? (monthlyTotal / monthlyMax) * 20 : 0;

          let examScore = 0;
          const score = table.options.meta?.getGradeValue?.(
            row.original.studentId,
            examItem.id
          );
          if (score) examScore = parseFloat(score);

          const totalValue = monthlyScaled + examScore;
          const isPass = totalValue >= 25; // Assuming 50% of 50 is pass

          return (
      <div className="flex items-center justify-center h-full">
              <span
                className={`text-sm font-bold ${
                  isPass
                    ? "text-green-600 font-bold bg-green-50 dark:bg-green-950/20"
                    : "text-red-600 font-bold bg-red-50 dark:bg-red-950/20"
                }`}
              >
                {isPass ? "Pass" : "Fail"}
              </span>
            </div>
          );
        },
      });
    }

    // 4. Total Column for Summary Mode
    columns.push({
      id: "total",
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
          "text-center bg-primary/5 group-hover:bg-primary/10 z-20 border-l",
      },
      cell: ({ row, table }) => {
        let monthlyTotal = 0;
        let monthlyMax = 0;
        items
          .filter((i) => summaryMonths.includes((i as any).month))
          .forEach((item) => {
            const score = table.options.meta?.getGradeValue?.(
              row.original.studentId,
              item.id
            );
            if (score) monthlyTotal += parseFloat(score);
            monthlyMax += item.maxScore;
          });
        const monthlyScaled =
          monthlyMax > 0 ? (monthlyTotal / monthlyMax) * 20 : 0;

        let examScore = 0;
        if (examItem) {
          const score = table.options.meta?.getGradeValue?.(
            row.original.studentId,
            examItem.id
          );
          if (score) examScore = parseFloat(score);
        }
        const result = (monthlyScaled + examScore).toFixed(2);

        return (
          <div className="flex items-center justify-center h-full">
            <span className="text-sm text-primary tabular-nums">{result}</span>
          </div>
        );
      },
    });
  } else {
    // Regular Month Mode: Show all assessments for the month
    items.forEach((item) => {
      columns.push({
        id: `assessment-${item.id}`,
        accessorKey: `assessment-${item.id}`,
        header: () => (
          <div className="flex flex-col items-center gap-0.5 px-2">
            <span className="font-bold text-xs md:text-sm tracking-tight whitespace-nowrap">
              {item.title}
            </span>
            <span className="text-[9px] uppercase text-muted-foreground font-medium">
              Max: {item.maxScore}
            </span>
          </div>
        ),
        size: 120,
        minSize: 100,
        meta: {
          headerLabel: item.title,
          className: "text-center border-r",
        },
        cell: ({ row, table }) => {
          const gradeValue =
            table.options.meta?.getGradeValue?.(
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

    // Total Column for Regular Mode
    columns.push({
      id: "total",
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
          "text-center bg-primary/5 group-hover:bg-primary/10 z-20 border-l",
      },
      cell: ({ row, table }) => {
        let total = 0;
        items.forEach((item) => {
          const gradeValue = table.options.meta?.getGradeValue?.(
            row.original.studentId,
            item.id
          );
          if (gradeValue) total += parseFloat(gradeValue);
        });

        return (
          <div className="flex items-center justify-center h-full">
            <span className="text-sm text-primary tabular-nums">{total}</span>
          </div>
        );
      },
    });
  }

  return columns;
};
