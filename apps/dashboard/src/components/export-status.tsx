"use client";

import { useJobStatus } from "@/hooks/use-job-status";
import { useSuccessSound } from "@/hooks/use-success-sound";
import { useExportStore } from "@/store/export";
import { downloadFile } from "@/lib/download";
import { Button } from "@midday/ui/button";
import { useToast } from "@midday/ui/use-toast";
import { useCallback, useEffect, useRef, useState } from "react";

type ExportResult = {
  url: string; // Direct S3 URL from uploadFile result
  filePath: string;
  fullPath: string;
  fileName: string;
  totalItems: number;
};

/**
 * Download button that uses the direct S3 URL from the job result.
 * No need for signed URL since the file is in the exports bucket.
 */
function DownloadButton({
  fullPath,
  fileName,
  onDownload,
}: {
  fullPath?: string;
  fileName: string;
  onDownload: () => void;
}) {
  const handleClick = useCallback(() => {
    if (!fullPath) return;
    
    // Use API proxy to handle authentication and avoid S3 AccessDenied
    const downloadUrl = `${process.env.NEXT_PUBLIC_API_URL}/files/proxy?filePath=vault/${fullPath}`;
    const url = new URL(downloadUrl, window.location.origin);
    url.searchParams.set("download", "true");
    
    downloadFile(url.toString(), fileName);
    onDownload();
  }, [fullPath, fileName, onDownload]);

  return (
    <Button size="sm" onClick={handleClick} disabled={!fullPath}>
      تحميل الملف
    </Button>
  );
}

/**
 * Global Export Status Monitor (2026)
 *
 * Tracks background PDF generation jobs and provides the download link.
 * Shows granular progress with phase descriptions.
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
    result && typeof result === "object" && "url" in result
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

  /**
   * Get a human-readable description based on progress and job status.
   */
  const getProgressDescription = (progressValue: number | undefined, jobStatus: string | undefined): string => {
    if (jobStatus === "waiting") return "في الانتظار (سيتم البدء قريباً)...";
    if (progressValue === undefined) return "جاري التحضير...";
    
    const val = progressValue;
    if (val < 5) return "جاري تحضير البيانات...";
    if (val < 10) return "جاري إنشاء جلسة المحرك...";
    if (val < 15) return "جاري تشغيل محرك التصدير...";
    if (val < 85) return "جاري توليد الشهادات (قد يستغرق وقتاً)...";
    if (val < 90) return "جاري دمج ملفات PDF...";
    if (val < 95) return "جاري رفع الملف النهائي...";
    return "جاري الإنهاء...";
  };

  // Handle failure
  useEffect(() => {
    if (status === "failed" || queryError) {
      if (toastId) dismiss(toastId);

      toast({
        duration: 6000,
        variant: "error",
        title: "فشل استخراج الشهادات",
        description:
          "حدث خطأ غير متوقع في محرك التصدير. يرجى المحاولة مرة أخرى.",
      });

      setToastId(null);
      setExportData(undefined);
      setIsExporting(false);
    }
  }, [
    status,
    queryError,
    toast,
    setExportData,
    setIsExporting,
    toastId,
    dismiss,
  ]);

  // Create toast when export starts
  useEffect(() => {
    if (exportData?.runId && !toastId && status !== "completed") {
      const { id } = toast({
        title: "جاري تجهيز الشهادات (في الخلفية)",
        variant: "progress",
        description:
          "يمكنك إغلاق هذه الرسالة ومواصلة العمل، سنخطرك عند الاكتمال.",
        duration: Number.POSITIVE_INFINITY,
        progress: 0,
      });

      setToastId(id);
      lastProgressRef.current = 0;
      completionHandledRef.current = false;
    }
  }, [exportData?.runId, toastId, toast, status]);

  // Update progress with phase description
  useEffect(() => {
    if (!toastId || !status) return;

    const currentProgress =
      status === "completed"
        ? 100
        : progress !== undefined
          ? Number(progress)
          : undefined;

    if (
      (currentProgress !== undefined && currentProgress !== lastProgressRef.current) ||
      status === "waiting"
    ) {
      lastProgressRef.current = currentProgress;
      update(toastId, {
        id: toastId,
        progress: currentProgress,
        description: getProgressDescription(currentProgress, status),
      });
    }
  }, [toastId, status, progress, update]);

  // Handle completion
  useEffect(() => {
    if (
      status === "completed" &&
      toastId &&
      !completionHandledRef.current
    ) {
      completionHandledRef.current = true;
      playSuccessSound();

      if (exportResult) {
        update(toastId, {
          id: toastId,
          title: "اكتمل استخراج الشهادات ✅",
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

      // Clear state after a short delay
      setTimeout(() => {
        setExportData(undefined);
        setIsExporting(false);
        completionHandledRef.current = false;
      }, 100);
    }
  }, [
    status,
    exportResult,
    toastId,
    update,
    setExportData,
    setIsExporting,
    handleOnDownload,
    playSuccessSound,
  ]);

  return null;
}
