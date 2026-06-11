"use client";

import { Button } from "@orbis/ui/button";
import { toast } from "@orbis/ui/use-toast";
import { Loader2, Plus } from "lucide-react";
import { useAction } from "next-safe-action/hooks";
import { initializeAssessmentsAction } from "@/actions/assessments/initialize-assessments";

interface InitializeAssessmentsButtonProps {
  classId: number;
  subjectId: number;
  termId: number;
  month: number;
  onSuccess?: () => void;
}

export function InitializeAssessmentsButton({
  classId,
  subjectId,
  termId,
  month,
  onSuccess,
}: InitializeAssessmentsButtonProps) {
  const { execute, status } = useAction(initializeAssessmentsAction, {
    onSuccess: () => {
      toast({
        title: "Success!",
        description: "Assessment items created successfully for this month.",
      });
      onSuccess?.();
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
        <h3 className="text-lg font-semibold">
          No records found for this month
        </h3>
        <p className="text-sm text-muted-foreground mt-1">
          You must create assessment items (attendance, participation, homework,
          exam) before entering grades.
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
