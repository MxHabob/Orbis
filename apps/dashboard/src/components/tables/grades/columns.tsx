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
  maxScore: number | string;
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
  grades: Grade[];
  getGradeValue: (studentId: string, assessmentItemId: number) => string | null;
  onGradeChange?: (
    studentId: string,
    assessmentItemId: number,
    value: number | null
  ) => void;
  onBulkFill?: (assessmentItemId: number, value: number) => void;
  onClearColumn?: (assessmentItemId: number) => void;
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

const roundGrade = (value: number) => Math.round(value * 2) / 2;

export const createGradeColumns = (
  items: AssessmentItem[] = [],
  month?: number
): ColumnDef<GradeStudent>[] => {
  const isSummaryMode = month === 4 || month === 8;
  const summaryMonths = month === 4 ? [1, 2, 3] : [5, 6, 7];
  const examMonth = month;

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
    // 1. Column for the 3-month average (scaled to 20)
    columns.push({
      id: "monthly-avg",

      header: () => (
        <div className="flex flex-col items-center gap-0.5 px-2">
          <span className="font-bold text-xs md:text-sm tracking-tight whitespace-nowrap">
            متوسط الشهور
          </span>
          <span className="text-[9px] uppercase text-muted-foreground font-medium">
            الحد الأقصى: 20
          </span>
        </div>
      ),
      size: 130,
      minSize: 110,
      meta: {
        headerLabel: "المتوسط (20)",
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
            totalMax += Number(item.maxScore);
          });

        // Scale to 20: (Score / Max) * 20
        const scaled = totalMax > 0 ? roundGrade((totalScore / totalMax) * 20) : 0;
        return (
          <span className="font-bold text-blue-600 dark:text-blue-400">
            {scaled}
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
              {month === 4 ? "الاختبار النصفي" : "الاختبار النهائي"}
            </span>
            <span className="text-[9px] uppercase text-muted-foreground font-medium">
              الحد الأقصى: 30
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
              maxScore={Number(examItem.maxScore)}
              table={table}
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
              monthlyMax += Number(item.maxScore);
            });
          const monthlyScaled =
            monthlyMax > 0 ? roundGrade((monthlyTotal / monthlyMax) * 20) : 0;

          let examScore = 0;
          let examMax = Number(examItem.maxScore);
          const score = table.options.meta?.getGradeValue?.(
            row.original.studentId,
            examItem.id
          );
          if (score) examScore = parseFloat(score);
          const examScaled = examMax > 0 ? roundGrade((examScore / examMax) * 30) : 0;

          const totalValue = roundGrade(monthlyScaled + examScaled);
          const totalMax = 50;
          const isPass = totalValue >= 25; // 50% of 50 is 25

          return (
      <div className="flex items-center justify-center h-full">
              <span
                className={`text-sm font-bold px-2 py-0.5 rounded-full ${
                  isPass
                    ? "text-green-600 bg-green-50 dark:bg-green-950/20"
                    : "text-red-600 bg-red-50 dark:bg-red-950/20"
                }`}
              >
                {isPass ? "ناجح" : "راسب"}
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
            monthlyMax += Number(item.maxScore);
          });
        const monthlyScaled =
          monthlyMax > 0 ? roundGrade((monthlyTotal / monthlyMax) * 20) : 0;

        let examScaled = 0;
        if (examItem) {
          const score = table.options.meta?.getGradeValue?.(
            row.original.studentId,
            examItem.id
          );
          if (score) {
            const examScore = parseFloat(score);
            const examMax = Number(examItem.maxScore);
            examScaled = examMax > 0 ? roundGrade((examScore / examMax) * 30) : 0;
          }
        }
        const result = roundGrade(monthlyScaled + examScaled);

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
              maxScore={Number(item.maxScore)}
              table={table}
            />
          );
        },
      });
    });

    // Total Column for Regular Mode
    columns.push({
      id: "total",
      header: () => (
        <div className="flex flex-col items-center gap-0.5 px-2">
          <span className="font-bold text-xs md:text-sm tracking-tight whitespace-nowrap">
            المحصلة
          </span>
          <span className="text-[9px] uppercase text-muted-foreground font-medium">
            الحد الأقصى: 100
          </span>
        </div>
      ),
      size: 120,
      minSize: 100,
      maxSize: 150,
      enableResizing: false,
      enableSorting: false,
      meta: {
        sticky: true,
        skeleton: { type: "text", width: "w-16" },
        headerLabel: "Total (100)",
        className:
          "text-center bg-primary/5 group-hover:bg-primary/10 z-20 border-l",
      },
      cell: ({ row, table }) => {
        let total = 0;
        let maxTotal = 0;
        items.forEach((item) => {
          const gradeValue = table.options.meta?.getGradeValue?.(
            row.original.studentId,
            item.id
          );
          if (gradeValue) total += parseFloat(gradeValue);
          maxTotal += Number(item.maxScore);
        });

        const scaled = maxTotal > 0 ? roundGrade((total / maxTotal) * 100) : 0;

        return (
          <div className="flex items-center justify-center h-full">
            <span className="text-sm font-bold text-primary tabular-nums">
              {scaled}
            </span>
          </div>
        );
      },
    });
  }

  return columns;
};
