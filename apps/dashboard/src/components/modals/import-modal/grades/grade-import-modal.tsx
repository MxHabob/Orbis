"use client";

import { useZodForm } from "@/hooks/use-zod-form";
import { useTRPC } from "@/trpc/client";
import { AnimatedSizeContainer } from "@midday/ui/animated-size-container";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from "@midday/ui/dialog";
import { SubmitButton } from "@midday/ui/submit-button";
import { useToast } from "@midday/ui/use-toast";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { parseAsBoolean, parseAsString, parseAsInteger, useQueryStates } from "nuqs";
import { useEffect, useState } from "react";
import { parseFile } from "@midday/import";
import { ImportCsvContext, importSchema } from "../shared/context";
import { SelectFile } from "../shared/select-file";
import { GradeFieldMapping } from "./grade-field-mapping";

const pages = ["select-file", "confirm-import"] as const;

export function GradeImportModal() {
  const trpc = useTRPC();
  const queryClient = useQueryClient();
  const [isImporting, setIsImporting] = useState(false);
  const [fileColumns, setFileColumns] = useState<string[] | null>(null);
  const [firstRows, setFirstRows] = useState<Record<string, string>[] | null>(null);
  const [pageNumber, setPageNumber] = useState<number>(0);
  const page = pages[pageNumber];
  const { toast } = useToast();

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

  const importGradesMutation = useMutation(
    trpc.assessments.importGrades.mutationOptions({})
  );

  const {
    control,
    watch,
    setValue,
    handleSubmit,
    reset,
    formState: { isValid },
  } = useZodForm(importSchema, {
    defaultValues: {
      type: "grades",
      classId: params.import_classId ? parseInt(params.import_classId) : undefined,
      academicYearId: params.import_yearId ? parseInt(params.import_yearId) : undefined,
      subjectId: params.import_subjectId ? parseInt(params.import_subjectId) : undefined,
      termId: params.import_termId ? parseInt(params.import_termId) : undefined,
      month: params.import_month ?? undefined,
    },
  });

  const file = watch("file");

  const onclose = () => {
    setIsImporting(false);
    setFileColumns(null);
    setFirstRows(null);
    setPageNumber(0);
    reset();
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

  useEffect(() => {
    if (file && fileColumns && firstRows && pageNumber === 0) {
      setPageNumber(1);
    }
  }, [file, fileColumns, firstRows, pageNumber]);

  return (
    <Dialog open={isOpen} onOpenChange={onclose}>
      <DialogContent className="overflow-visible">
        <div className="p-4 pb-0 max-h-[calc(100svh-10vw)] overflow-y-auto overflow-x-visible">
          <DialogHeader>
            <DialogTitle>
              {page === "select-file" ? "Choose grades import file" : "Map Grade Columns"}
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
                }}
              >
                <div className="overflow-visible">
                  <form
                    className="flex flex-col gap-y-4"
                    onSubmit={handleSubmit(async (data) => {
                      setIsImporting(true);
                      try {
                        const { data: rows } = await parseFile<Record<string, string>>(data.file, {
                          header: true,
                          skipEmptyLines: true,
                        });

                        const mappings = (data.mappings as Record<string, string>) || {};
                        const importPromises: Promise<any>[] = [];

                        for (const [assessmentIdStr, columnName] of Object.entries(mappings)) {
                          if (!columnName || columnName === "None") continue;
                          const assessmentItemId = Number(assessmentIdStr);
                          
                          const grades = rows
                            .map((row) => {
                              const studentNumber = row[data.studentNumber!];
                              if (!studentNumber) return null;
                              const scoreVal = row[columnName as string];
                              if (scoreVal === undefined || scoreVal === null || scoreVal.trim() === '') return null;
                              let score = parseFloat(scoreVal);
                              if (isNaN(score)) return null;

                              return { studentNumber, score, isAbsent: false };
                            })
                            .filter((g): g is NonNullable<typeof g> => g !== null);

                          if (grades.length > 0) {
                            importPromises.push(
                              importGradesMutation.mutateAsync({
                                assessmentItemId,
                                grades,
                                month: data.month,
                              } as any)
                            );
                          }
                        }

                        if (importPromises.length === 0) {
                          toast({ title: "No valid grade data found", variant: "warning" });
                          return;
                        }

                        await Promise.all(importPromises);
                        toast({ title: "Grades imported successfully", variant: "success" });
                        queryClient.invalidateQueries({ queryKey: trpc.assessments.getGrades.queryKey() });
                        onclose();
                      } catch (err: any) {
                        toast({ title: "Error during import", description: err.message, variant: "error" });
                      } finally {
                        setIsImporting(false);
                      }
                    })}
                  >
                    {page === "select-file" && <SelectFile />}
                    {page === "confirm-import" && (
                      <>
                        <GradeFieldMapping />
                        <SubmitButton isSubmitting={isImporting} disabled={!isValid} className="mt-4">
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
      </DialogContent>
    </Dialog>
  );
}
