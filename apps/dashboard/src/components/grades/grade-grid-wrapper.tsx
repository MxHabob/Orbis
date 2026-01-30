"use client";

import { useAction } from "next-safe-action/hooks";
import { getGradeGridDataAction } from "@/actions/assessments/get-grade-grid-data-action";
import { useEffect } from "react";
import { useQueryState, parseAsInteger } from "nuqs";
import { GradeDataTable } from "@/components/tables/grades/data-table";
import { GradesSkeleton } from "@/components/tables/grades/skeleton";
import { InitializeAssessmentsButton } from "./initialize-assessments-button";
import { AlertCircle } from "lucide-react";
import { Alert, AlertDescription, AlertTitle } from "@midday/ui/alert";

export function GradeGridWrapper({ initialSettings }: { initialSettings: any }) {
  const [classId] = useQueryState("classId", parseAsInteger);
  const [subjectId] = useQueryState("subjectId", parseAsInteger);
  const [termId] = useQueryState("termId", parseAsInteger);
  const [month] = useQueryState("month", parseAsInteger);
  const [yearId] = useQueryState("yearId", parseAsInteger);

  const { execute, result, status } = useAction(getGradeGridDataAction);

  useEffect(() => {
    if (classId && subjectId && termId && month && yearId) {
      execute({ classId, subjectId, termId, month, academicYearId: yearId });
    }
  }, [classId, subjectId, termId, month, yearId, execute]);

  if (!classId || !subjectId || !termId || !month || !yearId) {
    return (
      <div className="flex flex-col items-center justify-center p-20 border-2 border-dashed rounded-2xl bg-muted/5 text-muted-foreground">
        <div className="h-16 w-16 rounded-full bg-muted/10 flex items-center justify-center mb-4 text-3xl">🔍</div>
        <p className="font-medium text-lg">Waiting for selection</p>
        <p className="text-sm">Please select class, subject, and term from the filters above to display the grade sheet.</p>
      </div>
    );
  }

  if (status === "executing") {
    return <GradesSkeleton />;
  }

  // Handle server errors
  if (result.serverError) {
    return (
      <Alert variant="destructive" className="max-w-2xl mx-auto">
        <AlertCircle className="h-4 w-4" />
        <AlertTitle>Server Error</AlertTitle>
        <AlertDescription>{result.serverError}</AlertDescription>
      </Alert>
    );
  }

  // Handle validation errors (e.g. schema mismatch)
  if (result.validationErrors) {
    return (
      <Alert variant="destructive" className="max-w-2xl mx-auto">
        <AlertCircle className="h-4 w-4" />
        <AlertTitle>Validation Error</AlertTitle>
        <AlertDescription>
          <pre className="text-xs mt-2">
            {JSON.stringify(result.validationErrors, null, 2)}
          </pre>
        </AlertDescription>
      </Alert>
    );
  }

  if (result.data) {
    const { items, students } = result.data;

    // First check: Are there any students?
    if (students.length === 0) {
      return (
        <div className="flex flex-col items-center justify-center p-20 border border-dashed rounded-xl bg-muted/5 text-muted-foreground">
          <p className="text-lg font-semibold">No students enrolled in this class</p>
          <p className="text-sm">Make sure students are enrolled in this class for the selected academic year first.</p>
        </div>
      );
    }

    // Second check: Are there assessment items?
    if (items.length === 0) {
      return (
        <InitializeAssessmentsButton
          classId={classId}
          subjectId={subjectId}
          termId={termId}
          month={month}
        />
      );
    }

    // Pass the data to the new data table component
    return <GradeDataTable initialSettings={initialSettings} data={result.data as any} />;
  }

  // If we are here and not executing/not having data, it might mean nothing found or initial state
  return null;
}
