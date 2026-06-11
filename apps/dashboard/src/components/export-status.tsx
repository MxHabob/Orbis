"use client";

import { Button } from "@orbis/ui/button";
import { useToast } from "@orbis/ui/use-toast";
import { useCallback, useEffect, useRef, useState } from "react";
import { useJobStatus } from "@/hooks/use-job-status";
import { useSuccessSound } from "@/hooks/use-success-sound";
import { downloadFile } from "@/lib/download";
import { useI18n } from "@/locales/client";
import { useExportStore } from "@/store/export";

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
  const t = useI18n();
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
      {t("reports.export_progress.download")}
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
  const t = useI18n();
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
  const getProgressDescription = (
    progressValue: number | undefined,
    jobStatus: string | undefined,
  ): string => {
    if (jobStatus === "waiting") return t("reports.export_progress.waiting");
    if (progressValue === undefined)
      return t("reports.export_progress.preparing");

    const val = progressValue;
    if (val < 5) return t("reports.export_progress.preparing_data");
    if (val < 15) return t("reports.export_progress.starting_engine");
    if (val < 85) return t("reports.export_progress.generating");
    if (val < 95) return t("reports.export_progress.uploading");
    return t("reports.export_progress.finalizing");
  };

  // Handle failure
  useEffect(() => {
    if (status === "failed" || queryError) {
      if (toastId) dismiss(toastId);

      toast({
        duration: 6000,
        variant: "error",
        title: t("reports.export_progress.failed"),
        description: t("reports.export_progress.failed_desc"),
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
        title: t("reports.export_progress.queued_title"),
        variant: "progress",
        description: t("reports.export_progress.background_notice"),
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
      (currentProgress !== undefined &&
        currentProgress !== lastProgressRef.current) ||
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
    if (status === "completed" && toastId && !completionHandledRef.current) {
      completionHandledRef.current = true;
      playSuccessSound();

      if (exportResult) {
        update(toastId, {
          id: toastId,
          title: t("reports.export_progress.completed"),
          description: exportResult.fileName,
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
          title: t("reports.export_progress.completed"),
          description: t("reports.export_progress.completed_desc"),
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
