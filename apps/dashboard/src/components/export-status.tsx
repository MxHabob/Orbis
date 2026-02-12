"use client";

import { useFileUrl } from "@/hooks/use-file-url";
import { useJobStatus } from "@/hooks/use-job-status";
import { useSuccessSound } from "@/hooks/use-success-sound";
import { downloadFile } from "@/lib/download";
import { useExportStore } from "@/store/export";
import { Button } from "@midday/ui/button";
import { useToast } from "@midday/ui/use-toast";
import { useCallback, useEffect, useRef, useState } from "react";

type ExportResult = {
  filePath: string;
  fullPath: string;
  fileName: string;
  totalItems: number;
};

function DownloadButton({
  fullPath,
  fileName,
  onDownload,
}: {
  fullPath: string;
  fileName: string;
  onDownload: () => void;
}) {
  const { url: authenticatedUrl } = useFileUrl({
    type: "download",
    filePath: fullPath,
    filename: fileName,
  });

  return (
    <Button
      size="sm"
      onClick={() => {
        if (authenticatedUrl && fileName) {
          downloadFile(authenticatedUrl, fileName);
        }
        onDownload();
      }}
      disabled={!authenticatedUrl}
    >
      تحميل الملف
    </Button>
  );
}

/**
 * Global Export Status Monitor
 * 
 * Tracks the background Puppeteer PDF generation and provides the download link.
 */
export function ExportStatus() {
  const { toast, dismiss, update } = useToast();
  const { play: playSuccessSound } = useSuccessSound();
  const [toastId, setToastId] = useState<string | null>(null);
  const { exportData, setExportData, setIsExporting } = useExportStore();
  
  const { status, progress, result, isLoading, queryError } = useJobStatus({
    jobId: exportData?.runId,
    enabled: !!exportData?.runId,
  });

  // Type guard for export result
  const exportResult: ExportResult | undefined =
    result && typeof result === "object" && "fullPath" in result
      ? (result as ExportResult)
      : undefined;

  const handleOnDownload = useCallback(() => {
    if (toastId) {
      dismiss(toastId);
      setToastId(null);
    }
  }, [toastId, dismiss]);

  const lastProgressRef = useRef<number | undefined>(undefined);
  const completionHandledRef = useRef(false);

  // Handle failure
  useEffect(() => {
    if (status === "failed" || queryError) {
      if (toastId) dismiss(toastId);

      toast({
        duration: 4000,
        variant: "error",
        title: "فشل استخراج الشهادات",
        description: "حدث خطأ غير متوقع في محرك التصدير الخلفي.",
      });

      setToastId(null);
      setExportData(undefined);
      setIsExporting(false);
    }
  }, [status, queryError, toast, setExportData, setIsExporting, toastId, dismiss]);

  // Create toast when export starts
  useEffect(() => {
    if (exportData?.runId && !toastId && status !== "completed") {
      const { id } = toast({
        title: "جاري تجهيز الشهادات (في الخلفية)",
        variant: "progress",
        description: "يمكنك إغلاق هذه الرسالة ومواصلة العمل، سنخطرك عند الاكتمال.",
        duration: Number.POSITIVE_INFINITY,
        progress: 0,
      });

      setToastId(id);
      lastProgressRef.current = 0;
      completionHandledRef.current = false;
    }
  }, [exportData?.runId, toastId, toast, status]);

  // Update progress
  useEffect(() => {
    if (!toastId || !status) return;

    const currentProgress = status === "completed" ? 100 : progress !== undefined ? Number(progress) : undefined;

    if (
      currentProgress !== undefined &&
      currentProgress !== lastProgressRef.current &&
      (status === "active" || status === "waiting" || status === "completed")
    ) {
      lastProgressRef.current = currentProgress;
      update(toastId, {
        id: toastId,
        progress: currentProgress,
      });
    }
  }, [toastId, status, progress, update]);

  // Handle completion
  useEffect(() => {
    if (status === "completed" && toastId && !completionHandledRef.current) {
      completionHandledRef.current = true;
      playSuccessSound();

      if (exportResult) {
        update(toastId, {
          id: toastId,
          title: "اكتمل استخراج الشهادات",
          description: `تم استخراج ${exportResult.totalItems} شهادة بنجاح.`,
          variant: "success",
          footer: (
            <div className="mt-4 flex gap-3">
              <DownloadButton
                fullPath={exportResult.fullPath}
                fileName={exportResult.fileName}
                onDownload={handleOnDownload}
              />
            </div>
          ),
        });
      } else {
        update(toastId, {
          id: toastId,
          title: "اكتمل التصدير",
          description: "تم تجهيز الملف بنجاح.",
          variant: "success",
        });
      }

      // Automatically clear after a while if not dismissed manually
      setTimeout(() => {
        setExportData(undefined);
        setIsExporting(false);
        completionHandledRef.current = false;
      }, 100);
    }
  }, [status, exportResult, toastId, update, setExportData, setIsExporting, handleOnDownload, playSuccessSound]);

  return null;
}
