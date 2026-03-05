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
import { parseFile } from "@midday/import";
import { ImportCsvContext, studentImportSchema, type StudentImportFormData } from "../shared/context";
import { SelectFile } from "../shared/select-file";
import { StudentFieldMapping } from "./student-field-mapping";

const pages = ["select-file", "confirm-import"] as const;

export function StudentImportModal() {
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
    hide: parseAsBoolean.withDefault(false),
  });

  const isOpen = params.step === "import" && params.type === "students";

  const importStudentsMutation = useMutation(
    trpc.students.import.mutationOptions({})
  );

  const {
    control,
    watch,
    setValue,
    handleSubmit,
    reset,
    formState,
  } = useZodForm(studentImportSchema, {
    mode: "onChange",
    defaultValues: {
      type: "students",
      classId: params.import_classId ? parseInt(params.import_classId) : undefined,
      academicYearId: params.import_yearId ? parseInt(params.import_yearId) : undefined,
      autoGenerateStudentNumber: false,
    } as StudentImportFormData,
  });

  const { isValid } = formState;

  const file = watch("file");

  const onclose = () => {
    setIsImporting(false);
    setFileColumns(null);
    setFirstRows(null);
    setPageNumber(0);
    reset();
    setParams({ step: null, type: null, import_classId: null, import_yearId: null });
  };

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
    <Dialog open={isOpen} onOpenChange={onclose}>
      <DialogContent className="overflow-visible">
        <div className="p-4 pb-0 max-h-[calc(100svh-10vw)] overflow-y-auto overflow-x-visible">
          <DialogHeader>
            <DialogTitle>
              {page === "select-file" ? "Choose student import file" : "Confirm and import students"}
            </DialogTitle>
            <DialogDescription>
              {page === "select-file" 
                ? "Upload a CSV or Excel file containing student data."
                : "We've automatically mapped the columns, please review the data to ensure correctness."}
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

                        const students = rows
                          .map((row) => {
                            const studentNumber = data.autoGenerateStudentNumber 
                              ? undefined
                              : row[data.studentNumber!];
                            
                            const name = row[data.name!];
                            if (!data.autoGenerateStudentNumber && !studentNumber) return null;
                            if (!name) return null;

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
                        <StudentFieldMapping />
                        {Object.keys(formState.errors).length > 0 && (
                          <div className="text-xs text-red-500 mt-2">
                            Please fill all required fields: {renderErrors(formState.errors).join(', ')}
                          </div>
                        )}
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
