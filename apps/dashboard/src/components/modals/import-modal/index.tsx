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
import { ImportCsvContext, importSchema } from "./context";
import { FieldMapping } from "./field-mapping";
import { SelectFile } from "./select-file";
import { parseFile } from "@midday/import";

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
    import_classId: parseAsString,
    import_yearId: parseAsString,
    import_subjectId: parseAsString,
    import_termId: parseAsString,
    import_month: parseAsInteger,
    import_assessmentId: parseAsString,
    hide: parseAsBoolean.withDefault(false),
  });

  const isOpen = params.step === "import";

  // Mutations
  const importStudentsMutation = useMutation(
    trpc.students.import.mutationOptions({})
  );

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
      type: (params.type as any) || "students",
      classId: params.import_classId ? parseInt(params.import_classId) : undefined,
      academicYearId: params.import_yearId ? parseInt(params.import_yearId) : undefined,
      subjectId: params.import_subjectId ? parseInt(params.import_subjectId) : undefined,
      termId: params.import_termId ? parseInt(params.import_termId) : undefined,
      month: params.import_month ?? undefined,
      autoGenerateStudentNumber: false,
      assessmentItemId: params.import_assessmentId ? parseInt(params.import_assessmentId) : undefined,
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
      import_assessmentId: null,
      hide: null,
    });
  };

  // Improved key-based reset for 2026 performance standards
  const formKey = `${isOpen}-${params.type}-${params.import_classId}-${params.import_month}`;

  useEffect(() => {
    if (file && fileColumns && firstRows && pageNumber === 0) {
      setPageNumber(1);
    }
  }, [file, fileColumns, firstRows, pageNumber]);

  return (
    <Dialog open={isOpen} onOpenChange={onclose}>
      <DialogContent key={formKey} className="overflow-visible">
        <div className="p-4 pb-0 max-h-[calc(100svh-10vw)] overflow-y-auto overflow-x-visible">
            <DialogHeader>
              <div className="flex space-x-4 items-center mb-4">
                <DialogTitle className="m-0 p-0 w-full">
                  {page === "select-file" && "Choose import file"}
                  {page === "confirm-import" && (params.type === "grades" ? "Map Grade Columns" : "Confirm and import data")}
                </DialogTitle>
              </div>
              <DialogDescription>
                {page === "select-file" &&
                  `Upload a CSV or Excel file containing ${params.type === "students" ? "student" : "grade"} data.`}
                {page === "confirm-import" &&
                  (params.type === "grades" 
                    ? `Mapping grades for Month ${params.import_month}. Match your file columns to the assessments below.`
                    : "We've automatically mapped the columns, please review the data to ensure correctness.")}
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
                          // Parse the complete file
                          const { data: rows } = await parseFile<Record<string, string>>(data.file, {
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

                            await importStudentsMutation.mutateAsync({
                              classId: data.classId!,
                              academicYearId: data.academicYearId,
                              students,
                            });
                            
                            toast({ title: "Students imported successfully", variant: "success" });
                            queryClient.invalidateQueries({ queryKey: trpc.students.getAll.queryKey() });

                          } else if (data.type === "grades") {
                            // Professional Grade Import: Iterate over mappings
                            const mappings = data.mappings || {};
                            const importPromises: Promise<any>[] = [];

                            // Iterate over each mapped assessment item
                            for (const [assessmentIdStr, columnName] of Object.entries(mappings)) {
                              if (!columnName || columnName === "None") continue;

                              const assessmentItemId = Number(assessmentIdStr);
                              
                              const grades = rows
                                .map((row) => {
                                  
                                  const studentNumber = row[data.studentNumber!];
                                  if (!studentNumber) return null;

                                  const scoreVal = row[columnName];
                                  // Skip if no score
                                  if (scoreVal === undefined || scoreVal === null || scoreVal.trim() === '') return null;

                                  let score = parseFloat(scoreVal);
                                  if (isNaN(score)) return null;

                                  return {
                                    studentNumber,
                                    score,
                                    isAbsent: false, 
                                    comments: undefined,
                                  };
                                })
                                .filter((g): g is NonNullable<typeof g> => g !== null);

                                if (grades.length > 0) {
                                  importPromises.push(
                                    importGradesMutation.mutateAsync({
                                      assessmentItemId,
                                      grades,
                                      month: data.month, // Professional requirement: Monthly upload
                                    } as any) // Assuming API takes month
                                  );
                                }
                            }

                            if (importPromises.length === 0) {
                              toast({ title: "No valid grade data found to import", variant: "warning" });
                              setIsImporting(false);
                              return;
                            }

                            await Promise.all(importPromises);
                            
                            toast({ title: "Grades imported successfully", variant: "success" });
                            queryClient.invalidateQueries({ queryKey: trpc.assessments.getGrades.queryKey() });
                            // Also invalidate assessment items to show updated stats? 
                            // queryClient.invalidateQueries({ queryKey: trpc.assessments.getByClass.queryKey() });


                          } else {
                            toast({ title: "Transaction import not implemented in this version", variant: "error" });
                          }
                          
                          // Success cleanup
                          setIsImporting(false);
                          onclose();

                        } catch (err: any) {
                          console.error("Import error:", err);
                          setIsImporting(false);
                          toast({ title: "Error during import", description: err.message || "An unexpected error occurred", variant: "error" });
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
