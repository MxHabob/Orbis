"use client";

import { useCallback } from "react";
import { useTRPC } from "@/trpc/client";
import { useToast } from "@midday/ui/use-toast";
import { useExportStore } from "@/store/export";
import { useMutation } from "@tanstack/react-query";

/**
 * Modern Hook for Bulk Certificate Export
 * 
 * Switches from local client-side processing to Server-Side Background Jobs.
 */

interface ExportConfig {
  classIds: string[];
  templateId?: string;
  certificateType: string;
  selectedMonth: number;
}

export function useBulkCertificateExport() {
  const trpc = useTRPC();
  const { toast } = useToast();
  const { setExportData, setIsExporting } = useExportStore();

  const startExportMutation = useMutation(
    trpc.certificateTemplates.startExport.mutationOptions({
      onSuccess: (data) => {
        // Trigger the global Background Job monitor (ExportStatus.tsx)
        setExportData({
          runId: data.id,
          exportType: "file",
        });
        setIsExporting(true);
      },
      onError: (error) => {
        toast({
          title: "فشل بدء التصدير",
          description: error instanceof Error ? error.message : "حدث خطأ غير متوقع",
          variant: "error",
        });
      },
    })
  );

  const exportCertificates = useCallback(
    async (config: ExportConfig) => {
      // Trigger the mutation to start background job
      startExportMutation.mutate({
        classIds: config.classIds,
        templateId: config.templateId,
        certificateType: config.certificateType,
        selectedMonth: config.selectedMonth,
      });

      return { success: true };
    },
    [startExportMutation]
  );

  return { 
    exportCertificates, 
    isLoading: startExportMutation.isPending 
  };
}
