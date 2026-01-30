"use client";

import { useAction } from "next-safe-action/hooks";
import { initializeMonthlyAssessmentsAction } from "@/actions/assessments/initialize-monthly-assessments-action";
import { Button } from "@midday/ui/button";
import { Plus, Loader2 } from "lucide-react";
import { toast } from "@midday/ui/use-toast";

interface InitializeAssessmentsButtonProps {
  classId: number;
  subjectId: number;
  termId: number;
  month: number;
}

export function InitializeAssessmentsButton({
  classId,
  subjectId,
  termId,
  month,
}: InitializeAssessmentsButtonProps) {
  const { execute, status } = useAction(initializeMonthlyAssessmentsAction, {
    onSuccess: () => {
      toast({
        title: "Success!",
        description: "Assessment items created successfully for this month.",
      });
    },
    onError: () => {
      toast({
        title: "Error",
        description: "An error occurred while creating assessment items.",
        variant: "destructive",
      });
    },
  });

  return (
    <div className="flex flex-col items-center justify-center p-12 border-2 border-dashed rounded-xl bg-muted/20 gap-4">
      <div className="text-center">
        <h3 className="text-lg font-semibold">No records found for this month</h3>
        <p className="text-sm text-muted-foreground mt-1">
          You must create assessment items (attendance, participation, homework, exam) before entering grades.
        </p>
      </div>
      <Button
        size="lg"
        onClick={() => execute({ classId, subjectId, termId, month })}
        disabled={status === "executing"}
        className="gap-2"
      >
        {status === "executing" ? (
          <Loader2 className="h-5 w-5 animate-spin" />
        ) : (
          <Plus className="h-5 w-5" />
        )}
        Create Default Assessment Items
      </Button>
    </div>
  );
}
