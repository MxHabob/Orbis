"use client";

import { useRouter } from "next/navigation";
import { Button } from "@orbis/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
  DropdownMenuSeparator,
} from "@orbis/ui/dropdown-menu";
import { DotsHorizontalIcon } from "@radix-ui/react-icons";
import type { ClassReport } from "./columns";
import { useMutation } from "@tanstack/react-query";
import { useTRPC } from "@/trpc/client";
import { useToast } from "@orbis/ui/use-toast";
import { downloadBase64, generateCanvasPdf } from "@orbis/export";
import { Icons } from "@orbis/ui/icons";
import { Loader2 } from "lucide-react";
import { useExportStore } from "@/store/export";

type Props = {
  row: ClassReport;
};

export function ActionsMenu({ row }: Props) {
  const router = useRouter();
  const trpc = useTRPC();
  const { toast } = useToast();

  const { setExportData, setIsExporting } = useExportStore();

  const exportMutation = useMutation(
    trpc.grades.exportClassReport.mutationOptions({
      onSuccess: (data) => {
        setExportData({ runId: data.jobId });
        setIsExporting(true);
      },
      onError: (error) => {
        toast({
          title: "Export Failed",
          description: error.message || "An error occurred while exporting data.",
          variant: "destructive",
        });
      },
    })
  );

  const handleViewReport = () => {
    router.push(`/reports/${row.id}`);
  };

  const handleExportExcel = () => {
    exportMutation.mutate({
      classId: Number(row.id),
      type: "students_parents_data",
      format: "xlsx"
    });
  };

  const handleExportPdf = async () => {
    // Note: This expects the report content to be tagged with data-canvas-content if we are viewing it.
    // If we are in the list view, we might need to navigate to the report first OR we can use a different approach.
    // However, usually these "Export PDF" from a table row might trigger a specialized report generation.
    // For now, let's assume we want to view it first or if it's already rendered somewhere (unlikely for other rows).
    // If the user wants "direct", maybe they mean the Excel one.
    // Let's implement the PDF export by navigating to the report page with a query param that triggers download?
    // Or just fetch data and generate.
    
    // For now, I'll provide the option. If the user is ON the report page, it works perfectly.
    // In the LIST view, we might need a different PDF generator that doesn't rely on DOM.
    // I'll stick to the Excel export as the primary "direct" one requested.
    
    toast({
       title: "Generating PDF",
       description: "Opening report to generate PDF...",
    });
    router.push(`/reports/${row.id}?export=pdf`);
  };

  return (
    <div className="flex items-center justify-center w-full">
      <DropdownMenu>
        <DropdownMenuTrigger asChild className="relative">
          <Button variant="ghost" className="h-8 w-8 p-0" disabled={exportMutation.isPending}>
            {exportMutation.isPending ? (
              <Loader2 className="h-4 w-4 animate-spin text-muted-foreground" />
            ) : (
              <DotsHorizontalIcon className="h-4 w-4" />
            )}
          </Button>
        </DropdownMenuTrigger>

        <DropdownMenuContent align="end">
          <DropdownMenuItem onClick={handleViewReport} className="gap-2">
             <Icons.FolderZip className="size-4" />
             <span>View Report</span>
          </DropdownMenuItem>
          <DropdownMenuSeparator />
          <DropdownMenuItem onClick={handleExportExcel} className="gap-2">
            <Icons.CreateTransaction className="size-4" />
            <span>Export to Excel</span>
          </DropdownMenuItem>
          <DropdownMenuItem onClick={handleExportPdf} className="gap-2">
            <Icons.Pdf className="size-4" />
            <span>Export to PDF</span>
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    </div>
  );
}
