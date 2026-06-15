import { downloadBase64, generateCanvasPdf, parseCsv } from "@orbis/export";
import { Button } from "@orbis/ui/button";
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
} from "@orbis/ui/dropdown-menu";
import { Icons } from "@orbis/ui/icons";
import { useToast } from "@orbis/ui/use-toast";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { Loader2 } from "lucide-react";
import { useRef, useState } from "react";
import { useStudentParams } from "@/hooks/use-student-params";
import { useI18n } from "@/locales/client";
import { useExportStore } from "@/store/export";
import { useTRPC } from "@/ipc/query-adapter";

interface ReportActionsProps {
  classId: string;
  className?: string;
}

export function ReportActions({ classId, className }: ReportActionsProps) {
  const t = useI18n();
  const trpc = useTRPC();
  const queryClient = useQueryClient();
  const { toast } = useToast();
  const { setParams: setStudentParams } = useStudentParams();
  const fileInputRef = useRef<HTMLInputElement>(null);
  const [isExporting, setIsExporting] = useState(false);
  const [isImporting, setIsImporting] = useState(false);

  const importMutation = useMutation(
    trpc.students.import.mutationOptions({
      onSuccess: (_data, variables) => {
        toast({
          title: t("common.success"),
          description: t("students.import_success", {
            count: variables.students.length,
          }),
          variant: "success",
        });
        queryClient.invalidateQueries({
          queryKey: trpc.students.getByClassId.queryKey(),
        });
      },
      onError: (err) => {
        toast({
          title: "Error",
          description: `Failed to import students: ${err.message}`,
          variant: "destructive",
        });
      },
      onSettled: () => setIsImporting(false),
    }),
  );

  const { setExportData, setIsExporting: setGlobalExporting } =
    useExportStore();

  const exportMutation = useMutation(
    trpc.grades.exportClassReport.mutationOptions({
      onSuccess: (data) => {
        setExportData({ runId: data.jobId });
        setGlobalExporting(true);
        toast({
          title: t("common.export_started"),
          description: t("common.export_started_desc"),
          variant: "success",
        });
      },
      onError: (err) => {
        toast({
          title: "Export Failed",
          description: `Failed to export report: ${err.message}`,
          variant: "destructive",
        });
      },
      onSettled: () => setIsExporting(false),
    }),
  );

  const handleExport = (format: "xlsx" | "csv") => {
    setIsExporting(true);
    exportMutation.mutate({
      classId: parseInt(classId),
      type: "students_parents_data",
      format,
    });
  };

  const handlePdfExport = async () => {
    setIsExporting(true);
    try {
      await generateCanvasPdf({
        filename: `${className || "report"}_${new Date().toISOString().split("T")[0]}.pdf`,
        selector: "[data-canvas-content]",
      });

      toast({
        title: t("reports.export_excel_success"),
        description: t("reports.export_excel_success_desc"),
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

  const handleFileChange = async (
    event: React.ChangeEvent<HTMLInputElement>,
  ) => {
    const file = event.target.files?.[0];
    if (!file) return;

    event.target.value = "";
    setIsImporting(true);

    try {
      const { data } = await parseCsv<any>(file);

      const students = data
        .filter((row: any) => row.studentNumber && row.name)
        .map((row: any) => ({
          studentNumber: String(row.studentNumber),
          name: String(row.name),
          email: row.email ? String(row.email) : undefined,
          phone: row.phone ? String(row.phone) : undefined,
          sex: (row.sex?.toLowerCase() === "male"
            ? "male"
            : row.sex?.toLowerCase() === "female"
              ? "female"
              : "unspecified") as "male" | "female" | "unspecified",
        }));

      if (students.length > 0) {
        importMutation.mutate({
          classId: parseInt(classId),
          students,
        });
      } else {
        toast({
          title: "No Data",
          description: "No valid students found in CSV.",
          variant: "default",
        });
        setIsImporting(false);
      }
    } catch (e: any) {
      toast({
        title: "Error",
        description: e.message,
        variant: "destructive",
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
        <Button
          variant="outline"
          size="icon"
          disabled={isExporting || isImporting || importMutation.isPending}
        >
          {isExporting || isImporting || importMutation.isPending ? (
            <Loader2 className="animate-spin" size={17} />
          ) : (
            <Icons.Add size={17} />
          )}
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent sideOffset={10} align="end">
        <DropdownMenuItem
          onClick={() => setStudentParams({ createStudent: true })}
          className="space-x-2"
        >
          <Icons.Accounts size={18} />
          <span>{t("students.create")}</span>
        </DropdownMenuItem>
        <DropdownMenuGroup>
          <DropdownMenuSub>
            <DropdownMenuSubTrigger className="space-x-2 cursor-pointer">
              <Icons.Share size={18} />
              <span>{t("reports.export")}</span>
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
