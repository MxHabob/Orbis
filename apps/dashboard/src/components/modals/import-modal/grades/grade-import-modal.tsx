"use client";

import { parseFile } from "@orbis/import";
import { AnimatedSizeContainer } from "@orbis/ui/animated-size-container";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from "@orbis/ui/dialog";
import { SubmitButton } from "@orbis/ui/submit-button";
import { useToast } from "@orbis/ui/use-toast";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import {
  parseAsBoolean,
  parseAsInteger,
  parseAsString,
  useQueryStates,
} from "nuqs";
import { useState } from "react";
import { useZodForm } from "@/hooks/use-zod-form";
import { useTRPC } from "@/trpc/client";
import { invalidateGradesDomain } from "@/utils/query-invalidation";
import {
  type GradeImportFormData,
  gradeImportSchema,
  ImportCsvContext,
} from "../shared/context";
import { SelectFile } from "../shared/select-file";
import { GradeFieldMapping } from "./grade-field-mapping";

const pages = ["select-file", "confirm-import"] as const;

interface GradeImportFormContentProps {
  params: {
    import_classId: string | null;
    import_yearId: string | null;
    import_subjectId: string | null;
    import_termId: string | null;
    import_month: number | null;
  };
  onclose: () => void;
}

function GradeImportFormContent({
  params,
  onclose,
}: GradeImportFormContentProps) {
  const trpc = useTRPC();
  const queryClient = useQueryClient();
  const importGradesMutation = useMutation(
    trpc.assessments.importGrades.mutationOptions({}),
  );
  const [isImporting, setIsImporting] = useState(false);
  const gradeGridInput =
    params.import_classId &&
    params.import_subjectId &&
    params.import_termId &&
    params.import_month &&
    params.import_yearId
      ? {
          classId: Number(params.import_classId),
          subjectId: Number(params.import_subjectId),
          termId: Number(params.import_termId),
          month: params.import_month,
          academicYearId: Number(params.import_yearId),
        }
      : null;
  const [fileColumns, setFileColumns] = useState<string[] | null>(null);
  const [firstRows, setFirstRows] = useState<Record<string, string>[] | null>(
    null,
  );
  const [pageNumber, setPageNumber] = useState<number>(0);
  const page = pages[pageNumber];
  const { toast } = useToast();

  const { control, watch, setValue, handleSubmit, reset, formState } =
    useZodForm(gradeImportSchema, {
      mode: "onChange",
      defaultValues: {
        type: "grades",
        classId: params.import_classId
          ? Number(params.import_classId)
          : undefined,
        academicYearId: params.import_yearId
          ? Number(params.import_yearId)
          : undefined,
        subjectId: params.import_subjectId
          ? Number(params.import_subjectId)
          : undefined,
        termId: params.import_termId ? Number(params.import_termId) : undefined,
        month: params.import_month || new Date().getMonth() + 1,
        mappings: {},
      } as GradeImportFormData,
    });

  const { isValid } = formState;

  const renderErrors = (errors: any, prefix = ""): string[] => {
    return Object.keys(errors).flatMap((key) => {
      const error = errors[key];
      if (error?.message) {
        return [`${prefix}${key}: ${error.message}`];
      }
      if (typeof error === "object" && error !== null) {
        return renderErrors(error, `${prefix}${key}.`);
      }
      return [`${prefix}${key}: Required`];
    });
  };

  return (
    <div className="p-4 pb-0 max-h-[calc(100svh-10vw)] overflow-y-auto overflow-x-visible">
      <DialogHeader>
        <DialogTitle>
          {page === "select-file"
            ? "Choose grades import file"
            : "Map Grade Columns"}
        </DialogTitle>
        <DialogDescription>
          {page === "select-file"
            ? "Upload a CSV or Excel file containing student grades."
            : `Mapping grades for Month ${watch("month") || params.import_month || "selected"}. Match your file columns to the assessments.`}
        </DialogDescription>
      </DialogHeader>

      <div className="relative overflow-visible">
        <AnimatedSizeContainer height>
          <ImportCsvContext.Provider
            value={{
              fileColumns,
              setFileColumns,
              firstRows,
              setFirstRows,
              control,
              watch,
              setValue,
              formState,
              onFileProcessed: () => setPageNumber(1),
            }}
          >
            <div className="overflow-visible">
              <form
                className="flex flex-col gap-y-4"
                onSubmit={handleSubmit(async (data) => {
                  setIsImporting(true);
                  try {
                    const { data: rows } = await parseFile<
                      Record<string, string>
                    >(data.file, {
                      header: true,
                      skipEmptyLines: true,
                    });

                    const rawMappings =
                      (data.mappings as Record<string, string>) || {};
                    const importPromises: Promise<any>[] = [];

                    for (const [key, columnName] of Object.entries(
                      rawMappings,
                    )) {
                      if (!columnName || columnName === "None") continue;
                      const assessmentItemId = Number(key.replace("id_", ""));

                      const grades = rows
                        .map((row) => {
                          const identifier = row[data.studentNumber!];
                          if (!identifier) return null;
                          const scoreVal = row[columnName as string];
                          if (
                            scoreVal === undefined ||
                            scoreVal === null ||
                            scoreVal.trim() === ""
                          )
                            return null;
                          const score = parseFloat(scoreVal);
                          if (isNaN(score)) return null;

                          return { identifier, score, isAbsent: false };
                        })
                        .filter((g): g is NonNullable<typeof g> => g !== null);

                      if (grades.length > 0) {
                        importPromises.push(
                          importGradesMutation.mutateAsync({
                            assessmentItemId,
                            grades,
                            month: data.month,
                            identifierType: data.identifierType,
                          } as any),
                        );
                      }
                    }

                    if (importPromises.length === 0) {
                      toast({
                        title: "No valid grade data found",
                        variant: "warning",
                      });
                      return;
                    }

                    await Promise.all(importPromises);
                    toast({
                      title: "Grades imported successfully",
                      variant: "success",
                    });
                    void invalidateGradesDomain(
                      queryClient,
                      trpc,
                      gradeGridInput,
                    );
                    queryClient.invalidateQueries({
                      queryKey: trpc.assessments.getGrades.queryKey(),
                    });
                    onclose();
                  } catch (err: any) {
                    toast({
                      title: "Error during import",
                      description: err.message,
                      variant: "error",
                    });
                  } finally {
                    setIsImporting(false);
                  }
                })}
              >
                {page === "select-file" && <SelectFile />}
                {page === "confirm-import" && (
                  <>
                    <GradeFieldMapping />
                    {Object.keys(formState.errors).length > 0 && (
                      <div className="text-xs text-red-500 mt-2">
                        Please fill all required fields:{" "}
                        {renderErrors(formState.errors).join(", ")}
                      </div>
                    )}
                    <SubmitButton
                      isSubmitting={isImporting}
                      disabled={!isValid}
                      className="mt-4"
                    >
                      Confirm Import
                    </SubmitButton>
                    <button
                      type="button"
                      className="text-sm mb-4 text-[#878787]"
                      onClick={() => {
                        setPageNumber(0);
                        reset();
                        setFileColumns(null);
                        setFirstRows(null);
                      }}
                    >
                      Choose another file
                    </button>
                  </>
                )}
              </form>
            </div>
          </ImportCsvContext.Provider>
        </AnimatedSizeContainer>
      </div>
    </div>
  );
}

export function GradeImportModal() {
  const [params, setParams] = useQueryStates({
    step: parseAsString,
    type: parseAsString,
    import_classId: parseAsString,
    import_yearId: parseAsString,
    import_subjectId: parseAsString,
    import_termId: parseAsString,
    import_month: parseAsInteger,
    hide: parseAsBoolean.withDefault(false),
  });

  const isOpen = params.step === "import" && params.type === "grades";

  const onclose = () => {
    setParams({
      step: null,
      type: null,
      import_classId: null,
      import_yearId: null,
      import_subjectId: null,
      import_termId: null,
      import_month: null,
    });
  };

  return (
    <Dialog open={isOpen} onOpenChange={onclose}>
      <DialogContent className="overflow-visible">
        {isOpen && (
          <GradeImportFormContent
            params={params}
            onclose={onclose}
            key={`grades-import-${params.import_classId}-${params.import_subjectId}-${params.import_month}`}
          />
        )}
      </DialogContent>
    </Dialog>
  );
}
