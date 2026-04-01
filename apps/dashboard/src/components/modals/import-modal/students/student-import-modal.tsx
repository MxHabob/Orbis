"use client";

import { useZodForm } from "@/hooks/use-zod-form";
import { useI18n } from "@/locales/client";
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
import { invalidateStudentDomain } from "@/utils/query-invalidation";

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
  const t = useI18n();

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
                  onFileProcessed: () => setPageNumber(1),
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

                        const importResult = await importStudentsMutation.mutateAsync({
                          classId: data.classId!,
                          academicYearId: data.academicYearId,
                          students,
                        });
                        
                        // Show detailed result toast
                        const { summary, skippedStudents, failedStudents } = importResult;

                        if (summary.skipped === 0 && summary.failed === 0) {
                          toast({
                            title: t("students.import_success", { count: summary.added }),
                            variant: "success",
                            duration: 5000,
                          });
                        } else if (summary.added === 0 && summary.total > 0) {
                          toast({
                            title: t("students.import_none_added"),
                            description: (
                              <div className="mt-1 space-y-1.5">
                                <p className="text-xs text-[#878787]">
                                  {t("students.import_all_exist", { count: summary.total })}
                                </p>
                                {skippedStudents.length > 0 && (
                                  <div className="space-y-1 pr-1">
                                    {skippedStudents.slice(0, 3).map((s, i) => (
                                      <div key={i} className="flex items-start gap-1.5 text-xs text-[#878787]">
                                        <span className="shrink-0 mt-0.5">•</span>
                                        <span>
                                          <span className="text-foreground font-medium">{s.name}</span>
                                          {" — "}{s.reason}
                                        </span>
                                      </div>
                                    ))}
                                    {skippedStudents.length > 3 && (
                                      <p className="text-[10px] text-[#666]">
                                        {t("students.import_and_others", { count: skippedStudents.length - 3 })}
                                      </p>
                                    )}
                                  </div>
                                )}
                              </div>
                            ),
                            variant: "error",
                            duration: 12000,
                          });
                        } else {
                          toast({
                            title: t("students.import_partial", { added: summary.added, total: summary.total }),
                            description: (
                              <div className="mt-1 space-y-1.5">
                                <div className="flex gap-3 text-xs text-[#878787]">
                                  {summary.skipped > 0 && (
                                    <span>{t("students.import_skipped_count", { count: summary.skipped })}</span>
                                  )}
                                  {summary.failed > 0 && (
                                    <span>{t("students.import_failed_count", { count: summary.failed })}</span>
                                  )}
                                </div>
                                {skippedStudents.length > 0 && (
                                  <div className="space-y-1 pr-1">
                                    {skippedStudents.slice(0, 3).map((s, i) => (
                                      <div key={i} className="flex items-start gap-1.5 text-xs text-[#878787]">
                                        <span className="shrink-0 mt-0.5">•</span>
                                        <span>
                                          <span className="text-foreground font-medium">{s.name}</span>
                                          {" — "}{s.reason}
                                        </span>
                                      </div>
                                    ))}
                                    {skippedStudents.length > 3 && (
                                      <p className="text-[10px] text-[#666]">
                                        {t("students.import_and_others", { count: skippedStudents.length - 3 })}
                                      </p>
                                    )}
                                  </div>
                                )}
                                {failedStudents.length > 0 && (
                                  <div className="space-y-1 pr-1 border-t border-border pt-1.5 mt-1.5">
                                    {failedStudents.slice(0, 2).map((s, i) => (
                                      <div key={i} className="flex items-start gap-1.5 text-xs text-[#FF3638]">
                                        <span className="shrink-0 mt-0.5">✕</span>
                                        <span>
                                          <span className="font-medium">{s.name}</span>
                                          {" — "}{s.error}
                                        </span>
                                      </div>
                                    ))}
                                  </div>
                                )}
                              </div>
                            ),
                            variant: "warning",
                            duration: 15000,
                          });
                        }

                        void invalidateStudentDomain(queryClient, trpc);
                        onclose();
                      } catch (err: any) {
                        toast({ 
                          title: t("students.import_error"), 
                          description: err.message, 
                          variant: "error", 
                          duration: 8000,
                        });
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
