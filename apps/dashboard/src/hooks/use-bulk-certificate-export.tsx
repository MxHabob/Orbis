"use client";

import { useToast } from "@orbis/ui/use-toast";
import { useMutation } from "@tanstack/react-query";
import { useCallback } from "react";
import { useExportStore } from "@/store/export";
import { useTRPC } from "@/trpc/client";

/**
 * Hook for Bulk Certificate Export (2026 - Production Grade)
 *
 * Uses Server-Side Background Jobs via Puppeteer for
 * pixel-perfect template rendering with batch processing.
 *
 * Flow:
 * 1. Client triggers tRPC mutation → BullMQ job created
 * 2. ExportStatus component polls job progress
 * 3. Worker processes batches of students via Puppeteer
 * 4. PDF uploaded to S3 → download link provided
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
        setExportData({
          runId: data.id,
          exportType: "file",
        });
        setIsExporting(true);

        toast({
          title: "تم بدء التصدير",
          description: "سيتم تجهيز الشهادات في الخلفية. يمكنك متابعة عملك.",
          variant: "default",
          duration: 3000,
        });
      },
      onError: (error) => {
        toast({
          title: "فشل بدء التصدير",
          description:
            error instanceof Error ? error.message : "حدث خطأ غير متوقع",
          variant: "error",
        });
        setIsExporting(false);
      },
    }),
  );

  const exportCertificates = useCallback(
    async (config: ExportConfig) => {
      // Client-side validation
      if (!config.classIds || config.classIds.length === 0) {
        toast({
          title: "خطأ",
          description: "يجب تحديد فصل واحد على الأقل",
          variant: "error",
        });
        return { success: false };
      }

      try {
        setIsExporting(true);
        await startExportMutation.mutateAsync({
          classIds: config.classIds,
          templateId: config.templateId,
          certificateType: config.certificateType,
          selectedMonth: config.selectedMonth,
        });

        return { success: true };
      } catch (error) {
        setIsExporting(false);
        return { success: false };
      }
    },
    [startExportMutation, setIsExporting, toast],
  );

  return {
    exportCertificates,
    isLoading: startExportMutation.isPending,
  };
}
