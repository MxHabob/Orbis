"use client";

import { useJobStatus } from "@/hooks/use-job-status";
import { useSuccessSound } from "@/hooks/use-success-sound";
import { useExportStore } from "@/store/export";
import { useToast } from "@midday/ui/use-toast";
import { useEffect, useRef, useState } from "react";

/**
 * Global Export Status Monitor
 * 
 * Handles both Background Jobs (via jobId) and Client-Side Exports (via store status/progress).
 */
export function ExportStatus() {
  const { toast, dismiss, update } = useToast();
  const { play: playSuccessSound } = useSuccessSound();
  const [toastId, setToastId] = useState<string | null>(null);
  const { exportData, setExportData, setIsExporting, progress: clientProgress, status: clientStatus, setStatus: setClientStatus } = useExportStore();
  
  const { status: jobStatus, progress: jobProgress, queryError } = useJobStatus({
    jobId: exportData?.runId,
    enabled: !!exportData?.runId,
  });

  // Track last progress to avoid unnecessary updates
  const lastProgressRef = useRef<number | undefined>(undefined);
  const completionHandledRef = useRef(false);

  // 1. Handle Export Start (Create Toast)
  useEffect(() => {
    const isExportingBackground = !!exportData?.runId;
    const isExportingLocal = clientStatus === "processing";

    if ((isExportingBackground || isExportingLocal) && !toastId) {
      const { id } = toast({
        title: isExportingLocal ? "جاري استخراج الشهادات..." : "جاري التصدير...",
        variant: "progress",
        description: "يرجى عدم إغلاق المتصفح حتى الاكتمال",
        duration: Number.POSITIVE_INFINITY,
        progress: 0,
      });

      setToastId(id);
      lastProgressRef.current = 0;
      completionHandledRef.current = false;
    }
  }, [exportData?.runId, clientStatus, toastId, toast]);

  // 2. Handle Progress Updates
  useEffect(() => {
    if (!toastId) return;

    let currentProgress: number | undefined;
    let currentStatus: string | undefined;

    // Direct client-side progress takes precedence if clientStatus is processing
    if (clientStatus === "processing") {
      currentProgress = clientProgress;
      currentStatus = "active";
    } else if (exportData?.runId) {
      currentProgress = jobStatus === "completed" ? 100 : jobProgress;
      currentStatus = jobStatus;
    }

    if (
      currentProgress !== undefined &&
      currentProgress !== lastProgressRef.current &&
      (currentStatus === "active" || currentStatus === "waiting" || currentStatus === "completed")
    ) {
      lastProgressRef.current = currentProgress;
      update(toastId, {
        id: toastId,
        progress: currentProgress,
      });
    }
  }, [toastId, jobStatus, jobProgress, clientProgress, clientStatus, update, exportData?.runId]);

  // 3. Handle Completion
  useEffect(() => {
    const isCompleted = jobStatus === "completed" || clientStatus === "completed";
    
    if (isCompleted && toastId && !completionHandledRef.current) {
      completionHandledRef.current = true;
      playSuccessSound();

      update(toastId, {
        id: toastId,
        title: "اكتمل التصدير",
        description: "تم تحميل الملف بنجاح.",
        variant: "success",
      });

      // Clear after showing completion
      setTimeout(() => {
        setToastId(null);
        setExportData(undefined);
        setIsExporting(false);
        setClientStatus("idle");
        completionHandledRef.current = false;
      }, 3000);
    }
  }, [jobStatus, clientStatus, toastId, update, setExportData, setIsExporting, setClientStatus, playSuccessSound]);

  // 4. Handle Failure
  useEffect(() => {
    const isFailed = jobStatus === "failed" || clientStatus === "failed" || queryError;

    if (isFailed && toastId) {
      dismiss(toastId);
      
      toast({
        duration: 4000,
        variant: "error",
        title: "فشل التصدير",
        description: "حدث خطأ أثناء التصدير. يرجى المحاولة مرة أخرى.",
      });

      setToastId(null);
      setExportData(undefined);
      setIsExporting(false);
      setClientStatus("idle");
    }
  }, [jobStatus, clientStatus, queryError, toast, setExportData, setIsExporting, setClientStatus, toastId, dismiss]);

  return null;
}
