"use client";

import { useAction } from "next-safe-action/hooks";
import { upsertGradeAction } from "@/actions/assessments/upsert-grade-action";
import { Input } from "@midday/ui/input";
import { useState, useEffect, useRef, memo } from "react";
import { useDebounce } from "@/hooks/use-debounce";
import { cn } from "@midday/ui/cn";

interface EditableGradeCellProps {
  studentId: string;
  assessmentItemId: number;
  initialValue: string | null;
  maxScore: number;
}

export const EditableGradeCell = memo(function EditableGradeCell({
  studentId,
  assessmentItemId,
  initialValue,
  maxScore,
}: EditableGradeCellProps) {
  const [value, setValue] = useState(initialValue || "");
  const debouncedValue = useDebounce(value, 500);
  const isFirstRender = useRef(true);

  const { execute, status } = useAction(upsertGradeAction);

  useEffect(() => {
    if (isFirstRender.current) {
      isFirstRender.current = false;
      return;
    }

    if (debouncedValue !== initialValue) {
      const score = debouncedValue === "" ? null : parseFloat(debouncedValue);
      execute({
        studentId,
        assessmentItemId,
        score,
      });
    }
  }, [debouncedValue, initialValue, studentId, assessmentItemId, execute]);

  // Update local value if initialValue changes (from external refetch)
  useEffect(() => {
    setValue(initialValue || "");
  }, [initialValue]);

  const currentVal = parseFloat(value);
  const isInvalid = !isNaN(currentVal) && currentVal > maxScore;

  return (
    <div className="relative h-full w-full flex items-center justify-center group/cell">
      <Input
        type="number"
        value={value}
        onChange={(e) => setValue(e.target.value)}
        onFocus={(e) => e.target.select()}
        className={cn(
          "w-full h-full text-center transition-all px-2 border-transparent bg-transparent rounded-none focus-visible:ring-1 focus-visible:ring-primary focus-visible:bg-primary/5 focus-visible:border-primary/20 tabular-nums",
          isInvalid && "text-red-600 font-bold bg-red-50 dark:bg-red-950/20",
          status === "executing" && "opacity-50"
        )}
        placeholder="-"
        step="0.5"
        min="0"
        max={maxScore}
      />
      {status === "executing" && (
        <div className="absolute right-2 top-1/2 -translate-y-1/2 w-1.5 h-1.5 rounded-full bg-primary animate-pulse" />
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