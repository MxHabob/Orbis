"use client";

import { Button } from "@midday/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuPortal,
  DropdownMenuSub,
  DropdownMenuSubContent,
  DropdownMenuSubTrigger,
  DropdownMenuTrigger,
} from "@midday/ui/dropdown-menu";
import { Icons } from "@midday/ui/icons";
import { parseAsBoolean, parseAsString, parseAsInteger, useQueryStates } from "nuqs";
import { useGradeFilterParams } from "@/hooks/use-grade-filter-params";
import { useTRPC } from "@/trpc/client";
import { useToast } from "@midday/ui/use-toast";
import { useRef, useState } from "react";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { Loader2 } from "lucide-react";
import { exportToCsv, parseCsv, generateCanvasPdf, downloadBase64 } from "@midday/export";
import { useI18n } from "@/locales/client";
import { invalidateGradesDomain } from "@/utils/query-invalidation";
import { useExportStore } from "@/store/export";

export function GradesActions() {
  const [_, setParams] = useQueryStates({
    step: parseAsString,
    type: parseAsString,
    import_classId: parseAsString,
    import_yearId: parseAsString,
    import_subjectId: parseAsString,
    import_termId: parseAsString,
    import_month: parseAsInteger,
    import_assessmentId: parseAsString,
    initialize_classId: parseAsInteger,
    initialize_subjectId: parseAsInteger,
    initialize_termId: parseAsInteger,
    initialize_month: parseAsInteger,
    hide: parseAsBoolean,
  });
  
  const t = useI18n();

  const { filter } = useGradeFilterParams();
  const trpc = useTRPC();
  const { setExportData, setIsExporting: setGlobalExporting } = useExportStore();
  const queryClient = useQueryClient(); 
  const { toast } = useToast();
  const fileInputRef = useRef<HTMLInputElement>(null);
  const [isExporting, setIsExporting] = useState(false);
  const [isImporting, setIsImporting] = useState(false);
  const gradeGridInput =
    filter.selectedClassId &&
    filter.selectedSubjectId &&
    filter.termId &&
    filter.month &&
    filter.yearId
      ? {
          classId: filter.selectedClassId,
          subjectId: filter.selectedSubjectId,
          termId: filter.termId,
          month: filter.month,
          academicYearId: filter.yearId,
        }
      : null;

  const batchUpsertMutation = useMutation(
    trpc.grades.batchUpsert.mutationOptions({
      onSuccess: () => {
        toast({
          title: t("common.success"),
          description: t("grades.import_success"),
          variant: "success",
        });
        void invalidateGradesDomain(queryClient, trpc, gradeGridInput);
      },
      onError: (err) => {
        toast({
          title: "Error",
          description: `Failed to import grades: ${err.message}`,
          variant: "destructive",
        });
      },
    })
  );

  const exportMutation = useMutation(
    trpc.grades.export.mutationOptions({
      onSuccess: (data) => {
        setExportData({ runId: data.jobId });
        setGlobalExporting(true);
      },
      onError: (err) => {
        toast({
          title: t("common.error"),
          description: `Failed to export grades: ${err.message}`,
          variant: "destructive",
        });
      },
      onSettled: () => setIsExporting(false),
    })
  );

  const handleExport = async (format: "xlsx" | "csv") => {
    const { selectedClassId, selectedSubjectId, termId, month, yearId } =
      filter;

    if (
      !selectedClassId ||
      !selectedSubjectId ||
      !termId ||
      !month ||
      !yearId
    ) {
      toast({
        title: "Error",
        description: "Please select all filters before exporting.",
        variant: "destructive",
      });
      return;
    }

    setIsExporting(true);
    exportMutation.mutate({
      classId: selectedClassId,
      subjectId: selectedSubjectId,
      termId,
      month,
      academicYearId: yearId,
      format,
    });
  };

  const handlePdfExport = async () => {
    setIsExporting(true);
    try {
      await generateCanvasPdf({
        filename: `grades_report_${new Date().toISOString().split("T")[0]}.pdf`,
        selector: "[data-canvas-content]" 
      });

      toast({
        title: "Export Successful",
        description: "Grades have been exported to PDF.",
        variant: "success",
      });
    } catch (error: any) {
      toast({
        title: "Export Failed",
        description: "Failed to generate PDF.",
        variant: "destructive",
      });
    } finally {
      setIsExporting(false);
    }
  };

  const handleImportClick = () => {
     fileInputRef.current?.click();
  };

  const handleFileChange = async (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (!file) return;

    // Reset input so same file can be selected again if needed
    event.target.value = "";

    const { selectedClassId, selectedSubjectId, termId, month, yearId } =
      filter;

    if (
      !selectedClassId ||
      !selectedSubjectId ||
      !termId ||
      !month ||
      !yearId
    ) {
      toast({
        title: "Error",
        description: "Please ensure all filters are selected to match the context.",
        variant: "destructive", 
      });
      return;
    }

    setIsImporting(true);

    try {
      // 1. Fetch current context data to current student IDs and Assessment IDs
      const currentData = await queryClient.fetchQuery(
        trpc.grades.getGridData.queryOptions({
          classId: selectedClassId,
          subjectId: selectedSubjectId,
          termId,
          month,
          academicYearId: yearId,
        })
      );

      const { data, meta } = await parseCsv<any>(file);

      const gradesToUpsert: {
        studentId: string;
        assessmentItemId: number;
        score: number;
      }[] = [];

      // Identify assessment columns from headers
      const assessmentColumns: { name: string; id: number }[] = [];
      meta.fields?.forEach((field) => {
          const match = field.match(/.*\(ID:\s*(\d+)\)/);
          if (match && match[1]) {
            assessmentColumns.push({
                name: field,
                id: parseInt(match[1], 10)
            });
          }
      });

      if (assessmentColumns.length === 0) {
          toast({
            title: "Invalid Format",
            description: "Could not find any assessment columns with ID format e.g. 'Quiz (ID: 123)'",
            variant: "destructive"
          });
          setIsImporting(false);
          return;
      }

      // Process rows
      data.forEach((row) => {
          const studentNumber = row["Student Number"];
          if (!studentNumber) return;

          // Find student ID
          const student = currentData.students.find(s => s.studentNumber === studentNumber);
          if (!student) return; // Skip unknown students

          assessmentColumns.forEach(col => {
            const scoreVal = row[col.name];
            if (scoreVal !== undefined && scoreVal !== null && scoreVal !== "") {
                const scoreNum = parseFloat(scoreVal);
                if (!isNaN(scoreNum)) {
                  gradesToUpsert.push({
                    studentId: student.studentId,
                    assessmentItemId: col.id,
                    score: scoreNum
                  });
                }
            }
          });
      });

      if (gradesToUpsert.length > 0) {
          batchUpsertMutation.mutate({ grades: gradesToUpsert }, {
            onSettled: () => setIsImporting(false)
          });
      } else {
          toast({
            title: "No Data",
            description: "No valid grades found to import.",
            variant: "default" 
          });
          setIsImporting(false);
      }
    } catch (e: any) {
        toast({
           title: "Error",
           description: e.message,
           variant: "destructive"
        });
        setIsImporting(false);
    }
  };

  return (
    <DropdownMenu>
      <input
        type="file"
        ref={fileInputRef}
        className="hidden"
        accept=".csv"
        onChange={handleFileChange}
      />
      <DropdownMenuTrigger asChild>
        <Button variant="outline" size="icon" disabled={isExporting || isImporting || batchUpsertMutation.isPending}>
          {isExporting || isImporting || batchUpsertMutation.isPending ? <Loader2 className="animate-spin" size={17} /> : <Icons.Add size={17} />}
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent sideOffset={10} align="end">
        <DropdownMenuItem
          onClick={() => setParams({ 
            step: "import", 
            type: "grades",
            import_classId: filter.selectedClassId?.toString() ?? null,
            import_yearId: filter.yearId?.toString() ?? null,
            import_subjectId: filter.selectedSubjectId?.toString() ?? null,
            import_termId: filter.termId?.toString() ?? null,
            import_month: filter.month ?? null,
            import_assessmentId: null // Reset this as it might be old
          })}
          className="space-x-2"
        >
          <Icons.Import size={18} />
          <span>Import Grades</span>
        </DropdownMenuItem>

        <DropdownMenuItem
          onClick={() => setParams({ 
            step: "initialize_assessments", 
            initialize_classId: filter.selectedClassId,
            initialize_subjectId: filter.selectedSubjectId,
            initialize_termId: filter.termId,
            initialize_month: filter.month,
          })}
          className="space-x-2 text-primary"
        >
          <Icons.Add size={18} />
          <span>{t("grades.initialize_assessments")}</span>
        </DropdownMenuItem>
        
        {/* Helper for receipts if needed, keeping generic */}
        <DropdownMenuItem
          onClick={() =>
            document.getElementById("upload-transaction-files")?.click()
          }
          className="space-x-2"
        >
          <Icons.ReceiptLong size={18} />
          <span>Upload receipts</span>
        </DropdownMenuItem>

        <DropdownMenuGroup>
          <DropdownMenuSub>
            <DropdownMenuSubTrigger className="space-x-2 cursor-pointer">
              <Icons.Share size={18} />
              <span>Export</span>
            </DropdownMenuSubTrigger>
            <DropdownMenuPortal>
              <DropdownMenuSubContent sideOffset={14} alignOffset={-4}>
                <DropdownMenuItem 
                  className="space-x-2 cursor-pointer"
                  onClick={() => handleExport("xlsx")}
                >
                  <Icons.FolderZip size={18} />
                  <span>Excel (XLSX)</span>
                </DropdownMenuItem>
                <DropdownMenuItem 
                  className="space-x-2 cursor-pointer"
                  onClick={handlePdfExport}
                >
                  <Icons.Pdf size={18} />
                  <span>PDF</span>
                </DropdownMenuItem>
              </DropdownMenuSubContent>
            </DropdownMenuPortal>
          </DropdownMenuSub>
        </DropdownMenuGroup>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
