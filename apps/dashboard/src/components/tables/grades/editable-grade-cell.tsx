import { Input } from "@midday/ui/input";
import { useState, useEffect, memo } from "react";
import { cn } from "@midday/ui/cn";
import type { Table } from "@tanstack/react-table";

interface EditableGradeCellProps {
  studentId: string;
  assessmentItemId: number;
  initialValue: string | null;
  maxScore: number;
  table: Table<any>;
}

export const EditableGradeCell = memo(function EditableGradeCell({
  studentId,
  assessmentItemId,
  initialValue,
  maxScore,
  table,
}: EditableGradeCellProps) {
  const [value, setValue] = useState(initialValue || "");

  const onGradeChange = (table.options.meta as any)?.onGradeChange;
  const originalGrades = (table.options.meta as any)?.grades as any[];
  
  const originalGrade = originalGrades?.find(
    (g: any) => g.studentId === studentId && g.assessmentItemId === assessmentItemId
  )?.score ?? null;

  const isPending = initialValue !== originalGrade;

  // Update local value if initialValue changes (from external refetch or sync)
  useEffect(() => {
    setValue(initialValue || "");
  }, [initialValue]);

  const handleBlur = () => {
    if (value !== initialValue) {
      const score = value === "" ? null : parseFloat(value);
      onGradeChange?.(studentId, assessmentItemId, score);
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Enter") {
      e.currentTarget.blur();
    }
  };

  const currentVal = parseFloat(value);
  const isInvalid = !isNaN(currentVal) && currentVal > maxScore;

  return (
    <div className="relative h-full w-full flex items-center justify-center group/cell">
      <Input
        type="number"
        value={value}
        onChange={(e) => setValue(e.target.value)}
        onBlur={handleBlur}
        onKeyDown={handleKeyDown}
        onFocus={(e) => e.target.select()}
        className={cn(
          "w-full h-full text-center transition-all px-2 border-transparent bg-transparent rounded-none focus-visible:ring-1 focus-visible:ring-primary focus-visible:bg-primary/5 focus-visible:border-primary/20 tabular-nums",
          isInvalid && "text-red-600 font-bold bg-red-50 dark:bg-red-950/20",
          isPending && "bg-amber-50/50 dark:bg-amber-950/10 italic font-medium"
        )}
        placeholder="-"
        step="0.5"
        min="0"
        max={maxScore}
      />
      {isPending && (
        <div className="absolute right-1 top-1 w-1.5 h-1.5 rounded-full bg-amber-500" title="Unsaved change" />
      )}
      {isInvalid && (
        <div
          className="absolute top-0 right-0 w-0 h-0 border-t-[6px] border-t-red-500 border-l-[6px] border-l-transparent"
          title={`Exceeds maximum score of ${maxScore}`}
        />
      )}
    </div>
  );
});
