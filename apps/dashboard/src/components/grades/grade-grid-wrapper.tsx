"use client";

import { useTRPC } from "@/trpc/client";
import { useEffect, useMemo, useState } from "react";
import { GradeDataTable } from "@/components/tables/grades/data-table";
import { GradesSkeleton } from "@/components/tables/grades/skeleton";
import { InitializeAssessmentsButton } from "./initialize-assessments-button";
import { AlertCircle, Save, Loader2 } from "lucide-react";
import { Alert, AlertDescription, AlertTitle } from "@midday/ui/alert";
import { useGradeFilterParams } from "@/hooks/use-grade-filter-params";
import { Button } from "@midday/ui/button";
import { useToast } from "@midday/ui/use-toast";
import { useMutation, useQuery } from "@tanstack/react-query";
import { GradesSaveBar } from "./grades-save-bar";

export function GradeGridWrapper({ initialSettings }: { initialSettings: any }) {
  const { filter } = useGradeFilterParams();
  const { selectedClassId: classId, selectedSubjectId: subjectId, termId, month, yearId, q } = filter;
  const trpc = useTRPC();
  const { toast } = useToast();
  
  const [pendingChanges, setPendingChanges] = useState<Record<string, number | null>>({});

  const { data, isLoading, error, refetch } = useQuery(
    trpc.grades.getGridData.queryOptions(
      { 
        classId: classId!, 
        subjectId: subjectId!, 
        termId: termId!, 
        month: month!, 
        academicYearId: yearId! 
      },
      {
        enabled: !!(classId && subjectId && termId && month && yearId),
      }
    )
  );

  const batchUpsertMutation = useMutation(
    trpc.grades.batchUpsert.mutationOptions({
      onSuccess: () => {
        toast({
          title: "Success",
          description: "Grades saved successfully",
          variant: "success",
        });
        setPendingChanges({});
        refetch();
      },
      onError: (err) => {
        toast({
          title: "Error",
          description: `Failed to save grades: ${err.message}`,
          variant: "destructive",
        });
      },
    })
  );

  const handleGradeChange = (studentId: string, assessmentItemId: number, value: number | null) => {
    const key = `${studentId}:${assessmentItemId}`;
    setPendingChanges(prev => ({
      ...prev,
      [key]: value
    }));
  };

  const handleBulkFill = (assessmentItemId: number, value: number) => {
    if (!data) return;
    const newChanges = { ...pendingChanges };
    data.students.forEach(student => {
      const key = `${student.studentId}:${assessmentItemId}`;
      newChanges[key] = value;
    });
    setPendingChanges(newChanges);
    toast({
      title: "Bulk Fill Applied",
      description: `Set score ${value} for all students in this assessment.`,
    });
  };

  const handleClearColumn = (assessmentItemId: number) => {
    if (!data) return;
    const newChanges = { ...pendingChanges };
    data.students.forEach(student => {
      const key = `${student.studentId}:${assessmentItemId}`;
      newChanges[key] = null;
    });
    setPendingChanges(newChanges);
    toast({
      title: "Column Cleared",
      description: "Cleared all pending scores for this assessment.",
    });
  };

  const handleSaveAll = () => {
    const invalidGrades: string[] = [];
    const gradesToSave = Object.entries(pendingChanges).map(([key, score]) => {
      const parts = key.split(":");
      const studentId = parts[0];
      const assessmentItemId = parseInt(parts[1]!);
      
      const item = data?.items.find(i => i.id === assessmentItemId);
      if (item && score !== null && score > Number(item.maxScore)) {
        invalidGrades.push(`${item.title}: ${score} > ${item.maxScore}`);
      }

      return {
        studentId: studentId!,
        assessmentItemId,
        score,
      };
    });

    if (invalidGrades.length > 0) {
      toast({
        title: "Invalid Grades",
        description: "Some grades exceed the maximum score. Please correct them before saving.",
        variant: "destructive",
      });
      return;
    }

    if (gradesToSave.length === 0) return;

    batchUpsertMutation.mutate({ grades: gradesToSave });
  };

  const hasChanges = Object.keys(pendingChanges).length > 0;

  const hasErrors = useMemo(() => {
    return Object.entries(pendingChanges).some(([key, score]) => {
      const assessmentItemId = parseInt(key.split(":")[1]!);
      const item = data?.items.find((i) => i.id === assessmentItemId);
      const isExceeding = item && score !== null && score > Number(item.maxScore);
      const isInvalidNumber = score !== null && isNaN(score);
      return isExceeding || isInvalidNumber;
    });
  }, [pendingChanges, data?.items]);

  useEffect(() => {
    // Clear pending changes when filters change
    setPendingChanges({});
  }, [classId, subjectId, termId, month, yearId]);

  const filteredData = useMemo(() => {
    if (!data) return null;

    if (!q) return data;

    const lowerQ = q.toLowerCase();
    const filteredStudents = data.students.filter(
      (student) =>
        student.name?.toLowerCase().includes(lowerQ) ||
        student.studentNumber?.toLowerCase().includes(lowerQ)
    );

    return {
      ...data,
      students: filteredStudents,
    };
  }, [data, q]);

  if (!classId || !subjectId || !termId || !month || !yearId) {
    return (
      <div className="flex flex-col items-center justify-center p-20 border-2 border-dashed rounded-2xl bg-muted/5 text-muted-foreground">
        <div className="h-16 w-16 rounded-full bg-muted/10 flex items-center justify-center mb-4 text-3xl">🔍</div>
        <p className="font-medium text-lg">Waiting for selection</p>
        <p className="text-sm">Please select class, subject, and term from the filters above to display the grade sheet.</p>
      </div>
    );
  }

  if (isLoading) {
    return <GradesSkeleton />;
  }

  if (error) {
    return (
      <Alert variant="destructive" className="max-w-2xl mx-auto">
        <AlertCircle className="h-4 w-4" />
        <AlertTitle>Error</AlertTitle>
        <AlertDescription>{error.message}</AlertDescription>
      </Alert>
    );
  }

  if (filteredData) {
    const { items, students } = filteredData;

    if (students.length === 0 && !q) {
      return (
        <div className="flex flex-col items-center justify-center p-20 border border-dashed rounded-xl bg-muted/5 text-muted-foreground">
          <p className="text-lg font-semibold">No students enrolled in this class</p>
          <p className="text-sm">Make sure students are enrolled in this class for the selected academic year first.</p>
        </div>
      );
    }

    if (students.length === 0 && q) {
       return (
        <div className="flex flex-col items-center justify-center p-20 border border-dashed rounded-xl bg-muted/5 text-muted-foreground">
          <p className="text-lg font-semibold">No students found matching "{q}"</p>
        </div>
      );
    }

    if (items.length === 0) {
      return (
        <InitializeAssessmentsButton
          classId={classId}
          subjectId={subjectId}
          termId={termId}
          month={month}
          onSuccess={() => refetch()}
        />
      );
    }

    return (
      <div className="flex flex-col gap-4">
        <GradeDataTable 
          initialSettings={initialSettings} 
          data={filteredData as any} 
          pendingChanges={pendingChanges}
          onGradeChange={handleGradeChange}
          onBulkFill={handleBulkFill}
          onClearColumn={handleClearColumn}
        />
        <GradesSaveBar
          count={Object.keys(pendingChanges).length}
          onSave={handleSaveAll}
          onCancel={() => setPendingChanges({})}
          isPending={batchUpsertMutation.isPending}
          hasErrors={hasErrors}
        />
      </div>
    );
  }

  return null;
}
