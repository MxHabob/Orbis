import { Alert, AlertDescription, AlertTitle } from "@orbis/ui/alert";
import { Button } from "@orbis/ui/button";
import { useToast } from "@orbis/ui/use-toast";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { AlertCircle, Loader2, Plus, Settings2, Sparkles } from "lucide-react";
import { parseAsInteger, parseAsString, useQueryStates } from "nuqs";
import { useEffect, useMemo, useState } from "react";
import { GradeDataTable } from "@/components/tables/grades/data-table";
import { GradesSkeleton } from "@/components/tables/grades/skeleton";
import { useInitializeAssessments } from "@/hooks/api/use-enrollments";
import { useGradeFilterParams } from "@/hooks/use-grade-filter-params";
import { useI18n } from "@/locales/client";
import { useTRPC } from "@/ipc/query-adapter";
import { invalidateGradesDomain } from "@/utils/query-invalidation";
import { GradesSaveBar } from "./grades-save-bar";

export function GradeGridWrapper({
  initialSettings,
}: {
  initialSettings: any;
}) {
  const { filter } = useGradeFilterParams();
  const {
    selectedClassId: classId,
    selectedSubjectId: subjectId,
    termId,
    month,
    yearId,
    q,
  } = filter;
  const trpc = useTRPC();
  const { toast } = useToast();
  const t = useI18n();
  const queryClient = useQueryClient();
  const [_, setModalParams] = useQueryStates({
    step: parseAsString,
    initialize_classId: parseAsInteger,
    initialize_subjectId: parseAsInteger,
    initialize_termId: parseAsInteger,
    initialize_month: parseAsInteger,
  });

  const {
    mutate: executeInitialize,
    isPending: initializePending,
  } = useInitializeAssessments();

  const initializeStatus = initializePending ? "executing" : "idle";

  const runInitialize = () => {
    if (!classId || !subjectId || !termId || !month) return;
    executeInitialize(
      { classId, subjectId, termId, month },
      {
        onSuccess: () => {
          toast({
            title: t("grades.export_success"),
            description: "Standard items created.",
            variant: "success",
          });
          queryClient.invalidateQueries({
            queryKey: trpc.grades.getGridData.queryKey({
              classId: classId!,
              subjectId: subjectId!,
              termId: termId!,
              month: month!,
              academicYearId: yearId!,
            }),
          });
        },
        onError: () => {
          toast({
            title: "Error",
            description: "Failed to create standard items.",
            variant: "destructive",
          });
        },
      },
    );
  };

  const [pendingChanges, setPendingChanges] = useState<
    Record<string, number | null>
  >({});

  const gridInput =
    classId && subjectId && termId && month && yearId
      ? {
          classId,
          subjectId,
          termId,
          month,
          academicYearId: yearId,
        }
      : null;

  const gridQueryKey = gridInput
    ? trpc.grades.getGridData.queryKey(gridInput)
    : trpc.grades.getGridData.queryKey();

  const { data, isLoading, error } = useQuery(
    trpc.grades.getGridData.queryOptions(
      {
        classId: classId!,
        subjectId: subjectId!,
        termId: termId!,
        month: month!,
        academicYearId: yearId!,
      },
      {
        enabled: !!(classId && subjectId && termId && month && yearId),
      },
    ),
  );

  const batchUpsertMutation = useMutation(
    trpc.grades.batchUpsert.mutationOptions({
      onMutate: async (variables) => {
        if (!gridInput) return { previousData: undefined };

        await queryClient.cancelQueries({ queryKey: gridQueryKey });
        const previousData = queryClient.getQueryData<any>(gridQueryKey);

        queryClient.setQueryData<any>(gridQueryKey, (current: any) => {
          if (!current) return current;

          const nextGrades = [...(current.grades ?? [])];

          for (const grade of variables.grades) {
            const index = nextGrades.findIndex(
              (entry) =>
                entry.studentId === grade.studentId &&
                entry.assessmentItemId === grade.assessmentItemId,
            );

            if (grade.score === null) {
              if (index !== -1) nextGrades.splice(index, 1);
              continue;
            }

            if (index !== -1) {
              nextGrades[index] = {
                ...nextGrades[index],
                score: grade.score,
              };
            } else {
              nextGrades.push({
                studentId: grade.studentId,
                assessmentItemId: grade.assessmentItemId,
                score: grade.score,
              });
            }
          }

          return {
            ...current,
            grades: nextGrades,
          };
        });

        return { previousData };
      },
      onSuccess: () => {
        toast({
          title: "Success",
          description: "Grades saved successfully",
          variant: "success",
        });
        setPendingChanges({});
      },
      onError: (err, _variables, context) => {
        if (context?.previousData && gridInput) {
          queryClient.setQueryData(gridQueryKey, context.previousData);
        }
        toast({
          title: "Error",
          description: `Failed to save grades: ${err.message}`,
          variant: "destructive",
        });
      },
      onSettled: async () => {
        await invalidateGradesDomain(queryClient, trpc, gridInput);
      },
    }),
  );

  const handleGradeChange = (
    studentId: string,
    assessmentItemId: number,
    value: number | null,
  ) => {
    const key = `${studentId}:${assessmentItemId}`;
    setPendingChanges((prev) => ({
      ...prev,
      [key]: value,
    }));
  };

  const handleBulkFill = (assessmentItemId: number, value: number) => {
    if (!data) return;
    const newChanges = { ...pendingChanges };
    data.students.forEach((student) => {
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
    data.students.forEach((student) => {
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

      const item = data?.items.find((i) => i.id === assessmentItemId);
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
        description:
          "Some grades exceed the maximum score. Please correct them before saving.",
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
      const isExceeding =
        item && score !== null && score > Number(item.maxScore);
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
        student.studentNumber?.toLowerCase().includes(lowerQ),
    );

    return {
      ...data,
      students: filteredStudents,
    };
  }, [data, q]);

  if (!classId || !subjectId || !termId || !month || !yearId) {
    return (
      <div className="flex flex-col items-center justify-center py-10 px-6 border border-dashed rounded-xl bg-muted/2 text-muted-foreground animate-in fade-in duration-500">
        <div className="h-16 w-16 rounded-full bg-muted/10 flex items-center justify-center mb-6 text-3xl">
          🔍
        </div>
        <h2 className="font-medium text-lg text-foreground mb-2">
          Waiting for selection
        </h2>
        <p className="text-sm text-[#606060] text-center max-w-md">
          Please select class, subject, and term from the filters above to
          display the grade sheet.
        </p>
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
        <div className="flex flex-col items-center justify-center py-10 px-6 border border-dashed rounded-xl bg-muted/5 text-muted-foreground animate-in fade-in duration-500">
          <div className="h-16 w-16 rounded-full bg-muted/10 flex items-center justify-center mb-6 text-3xl">
            👥
          </div>
          <h2 className="font-medium text-lg text-foreground mb-2">
            No students enrolled in this class
          </h2>
          <p className="text-sm text-[#606060] max-w-sm text-center">
            Make sure students are enrolled in this class for the selected
            academic year first.
          </p>
        </div>
      );
    }

    if (students.length === 0 && q) {
      return (
        <div className="flex flex-col items-center justify-center py-10 px-6 border border-dashed rounded-xl bg-muted/5 text-muted-foreground animate-in fade-in duration-300">
          <div className="h-12 w-12 rounded-full bg-muted/10 flex items-center justify-center mb-4 text-2xl">
            🔍
          </div>
          <h2 className="font-medium text-lg text-foreground mb-2">
            No students found matching "{q}"
          </h2>
          <p className="text-sm text-[#606060]">
            Try adjusting your search term
          </p>
        </div>
      );
    }

    const currentMonthItems = items.filter((i) => i.month === month);

    if (items.length === 0) {
      return (
        <div className="flex flex-col items-center justify-center py-10 px-6 border border-dashed rounded-xl bg-muted/2 text-muted-foreground animate-in fade-in duration-500">
          <div className="h-16 w-16 rounded-full bg-muted/10 flex items-center justify-center mb-6">
            <Settings2 className="h-8 w-8 text-foreground/60" />
          </div>
          <h2 className="font-medium text-lg text-foreground mb-2">
            {t("grades.no_items")}
          </h2>
          <p className="text-sm text-[#606060] max-w-sm text-center mb-8">
            {t("grades.initialize_assessments_description")}
          </p>
          <div className="flex gap-3">
            <Button
              variant="outline"
              className="px-6"
              onClick={() =>
                setModalParams({
                  step: "initialize_assessments",
                  initialize_classId: classId,
                  initialize_subjectId: subjectId,
                  initialize_termId: termId,
                  initialize_month: month,
                })
              }
            >
              {t("grades.add_item")}
            </Button>
            <Button
              className="px-6 gap-2"
              disabled={initializeStatus === "executing"}
              onClick={runInitialize}
            >
              {initializeStatus === "executing" ? (
                <Loader2 className="h-4 w-4 animate-spin" />
              ) : (
                <Sparkles className="h-4 w-4" />
              )}
              {t("grades.create_default_items")}
            </Button>
          </div>
          <p className="mt-10 text-xs text-[#606060] flex items-center gap-1.5 opacity-80">
            <Plus className="h-3.5 w-3.5" />
            {t("grades.standard_items_help")}
          </p>
        </div>
      );
    }

    return (
      <div className="flex flex-col gap-4">
        {currentMonthItems.length === 0 && (
          <Alert className="bg-amber-50 dark:bg-amber-950/20 border-amber-200 dark:border-amber-800 border-dashed">
            <AlertCircle className="h-4 w-4 text-amber-600 dark:text-amber-500" />
            <div className="flex items-center justify-between w-full">
              <div>
                <AlertTitle className="text-amber-800 dark:text-amber-400">
                  {t("grades.initialize_assessments")}
                </AlertTitle>
                <AlertDescription className="text-amber-700 dark:text-amber-500">
                  {t("grades.initialize_assessments_description")}
                </AlertDescription>
              </div>
              <Button
                variant="outline"
                size="sm"
                disabled={initializeStatus === "executing"}
                onClick={() =>
                  executeInitialize({
                    classId: classId!,
                    subjectId: subjectId!,
                    termId: termId!,
                    month: month!,
                  })
                }
                className="bg-white dark:bg-black border-amber-200 dark:border-amber-800 hover:bg-amber-100 dark:hover:bg-amber-900/40 text-amber-900 dark:text-amber-200"
              >
                {initializeStatus === "executing" ? (
                  <Loader2 className="h-4 w-4 animate-spin mr-2" />
                ) : null}
                {t("grades.create_default_items")}
              </Button>
            </div>
          </Alert>
        )}
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
