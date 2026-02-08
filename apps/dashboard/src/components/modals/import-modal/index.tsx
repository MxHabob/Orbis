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
import { parseAsBoolean, parseAsString, useQueryStates } from "nuqs";

import { useEffect, useState } from "react";
import { ImportCsvContext, importSchema } from "./context";
import { FieldMapping } from "./field-mapping";
import { SelectFile } from "./select-file";
import { parseCsv } from "@midday/export";

const pages = ["select-file", "confirm-import"] as const;

export function ImportModal() {
  const trpc = useTRPC();
  const queryClient = useQueryClient();
  const [isImporting, setIsImporting] = useState(false);
  const [fileColumns, setFileColumns] = useState<string[] | null>(null);
  const [firstRows, setFirstRows] = useState<Record<string, string>[] | null>(
    null,
  );

  const [pageNumber, setPageNumber] = useState<number>(0);
  const page = pages[pageNumber];

  const { toast } = useToast();

  const [params, setParams] = useQueryStates({
    step: parseAsString,
    type: parseAsString.withDefault("students"),
    classId: parseAsString,
    academicYearId: parseAsString,
    subjectId: parseAsString,
    assessmentItemId: parseAsString,
    hide: parseAsBoolean.withDefault(false),
  });

  const isOpen = params.step === "import";

  // Mutations
  const importStudentsMutation = useMutation(
    trpc.students.import.mutationOptions({
      onSuccess: () => {
        setIsImporting(false);
        toast({ title: "Students imported successfully", variant: "success" });
        onclose();
        queryClient.invalidateQueries({ queryKey: trpc.students.getAll.queryKey() });
      },
      onError: (err) => {
        setIsImporting(false);
        toast({ title: "Error importing students", description: err.message, variant: "error" });
      }
    })
  );

  const importGradesMutation = useMutation(
    trpc.assessments.importGrades.mutationOptions({
      onSuccess: () => {
        setIsImporting(false);
        toast({ title: "Grades imported successfully", variant: "success" });
        onclose();
        queryClient.invalidateQueries({ queryKey: trpc.assessments.getGrades.queryKey() });

      },
      onError: (err) => {
        setIsImporting(false);
        toast({ title: "Error importing grades", description: err.message, variant: "error" });
      }
    })
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
      type: (params.type as any) || "students",
      classId: params.classId ? parseInt(params.classId) : undefined,
      academicYearId: params.academicYearId ? parseInt(params.academicYearId) : undefined,
      subjectId: params.subjectId ? parseInt(params.subjectId) : undefined,
      assessmentItemId: params.assessmentItemId ? parseInt(params.assessmentItemId) : undefined,
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
      classId: null,
      academicYearId: null,
      subjectId: null,
      assessmentItemId: null,
      hide: null,
    });
  };

  // Go to second page if file looks good
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
            <div className="flex space-x-4 items-center mb-4">
              <DialogTitle className="m-0 p-0 w-full">
                {page === "select-file" && "Choose import file"}
                {page === "confirm-import" && "Confirm and import data"}
              </DialogTitle>
            </div>
            <DialogDescription>
              {page === "select-file" &&
                `Upload a CSV file containing ${params.type === "students" ? "student" : "grade"} data.`}
              {page === "confirm-import" &&
                "We've automatically mapped the columns, please review the data to ensure correctness."}
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

                      // Parse the complete file
                      // Parse the complete file
                      try {
                        const { data: rows } = await parseCsv<Record<string, string>>(data.file, {
                          header: true,
                          skipEmptyLines: true,
                        });

                        if (data.type === "students") {
                          const students = rows
                            .map((row, index) => {
                              let studentNumber = data.autoGenerateStudentNumber 
                                ? `STU-${Date.now()}-${index}`
                                : row[data.studentNumber!];
                              
                              const name = row[data.name!];
                              if (!studentNumber || !name) return null;

                              return {
                                studentNumber,
                                name,
                                phone: (data.phone && row[data.phone]) ? row[data.phone] : undefined,
                                email: (data.email && row[data.email]) ? row[data.email] : undefined,
                                sex: (data.sex && row[data.sex]) ? (row[data.sex]!.toLowerCase() as any) : 'unspecified',
                              };

                            })
                            .filter((s): s is NonNullable<typeof s> => s !== null);

                          importStudentsMutation.mutate({
                            classId: data.classId!,
                            academicYearId: data.academicYearId,
                            students,
                          });
                        } else if (data.type === "grades") {
                          const grades = rows
                            .map((row) => {
                              const studentNumber = row[data.studentNumber!];
                              if (!studentNumber) return null;

                              return {
                                studentNumber,
                                score: (data.score && row[data.score]) ? parseFloat(row[data.score]!) : null,
                                isAbsent: (data.isAbsent && row[data.isAbsent]) ? row[data.isAbsent]?.toLowerCase() === 'true' || row[data.isAbsent] === '1' : false,
                                comments: (data.comments && row[data.comments]) ? row[data.comments] : undefined,
                              };

                            })
                            .filter((g): g is NonNullable<typeof g> => g !== null);

                          importGradesMutation.mutate({
                            assessmentItemId: data.assessmentItemId!,
                            grades,
                          });


                        } else {
                          // Fallback for legacy transaction import if needed
                          toast({ title: "Transaction import not implemented in this version", variant: "error" });
                          setIsImporting(false);
                        }
                      } catch (err: any) {
                        setIsImporting(false);
                        toast({ title: "Error parsing CSV", description: err.message, variant: "error" });
                      }
                    })}

                  >
                    {page === "select-file" && <SelectFile />}
                    {page === "confirm-import" && (
                      <>
                        <FieldMapping />

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
      </DialogContent>
    </Dialog>
  );
}
