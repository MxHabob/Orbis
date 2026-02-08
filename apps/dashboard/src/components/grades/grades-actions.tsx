"use client";

import { useGradesParams } from "@/hooks/use-grades-params";
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
import { parseAsBoolean, parseAsString, useQueryStates } from "nuqs";
import { useGradeFilterParams } from "@/hooks/use-grade-filter-params";
import { useTRPC } from "@/trpc/client";
import { useToast } from "@midday/ui/use-toast";
import { useRef, useState } from "react";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { Loader2 } from "lucide-react";
import { exportToCsv, parseCsv, generateCanvasPdf, downloadBase64 } from "@midday/export";

export function GradesActions() {
  const [_, setParams] = useQueryStates({
    step: parseAsString,
    hide: parseAsBoolean,
  });

  const { setParams: setGradesParams } = useGradesParams();
  const { filter } = useGradeFilterParams();
  const trpc = useTRPC();
  const queryClient = useQueryClient(); 
  const { toast } = useToast();
  const fileInputRef = useRef<HTMLInputElement>(null);
  const [isExporting, setIsExporting] = useState(false);
  const [isImporting, setIsImporting] = useState(false);

  const batchUpsertMutation = useMutation(
    trpc.grades.batchUpsert.mutationOptions({
      onSuccess: () => {
        toast({
          title: "Success",
          description: "Grades imported successfully",
          variant: "success",
        });
        queryClient.invalidateQueries({
          queryKey: trpc.grades.getGridData.queryKey(),
        });
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
        downloadBase64(data.data, data.filename, data.mimeType);
        toast({
          title: "Export Successful",
          description: "Grades have been exported successfully.",
          variant: "success",
        });
      },
      onError: (err) => {
        toast({
          title: "Export Failed",
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
          onClick={() => setParams({ step: "connect" })}
          className="space-x-2"
        >
          <Icons.Accounts size={18} />
          <span>Connect accounts</span>
        </DropdownMenuItem>
        <DropdownMenuItem
          onClick={() => setParams({ step: "import", hide: true })}
          className="space-x-2"
        >
          <Icons.Import size={18} />
          <span>Backfill</span>
        </DropdownMenuItem>
        <DropdownMenuItem
          onClick={handleImportClick}
          className="space-x-2"
        >
          <Icons.CreateTransaction size={18} />
          <span>Upload grades (CSV)</span>
        </DropdownMenuItem>
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
