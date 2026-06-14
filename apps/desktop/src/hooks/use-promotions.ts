"use client";

import { useToast } from "@orbis/ui/use-toast";
import {
  useMutation,
  useQuery,
  useQueryClient,
  useSuspenseQuery,
} from "@tanstack/react-query";
import { useCallback, useState } from "react";
import { usePromotionsParams } from "@/hooks/use-promotions-params";
import { useI18n } from "@/locales/client";
import { useTRPC } from "@/trpc/client";

// ─── Types ───────────────────────────────────────────────────────────────────

export type PromotionTab = "preview" | "history" | "statistics";

export interface PromotionDecision {
  studentId: string;
  status: "promoted" | "retained" | "graduated" | "conditional";
  toClassId?: number;
}

export interface PreviewStudent {
  studentId: string;
  suggestedStatus: string;
  toClassId?: number;
}

// ─── Hook ────────────────────────────────────────────────────────────────────

export function usePromotions() {
  const t = useI18n();
  const trpc = useTRPC();
  const { toast } = useToast();
  const queryClient = useQueryClient();
  const { promotionClassId, sourceYearId, targetYearId, setParams } =
    usePromotionsParams();

  const [activeTab, setActiveTab] = useState<PromotionTab>("preview");
  const [rollbackConfirmOpen, setRollbackConfirmOpen] = useState(false);
  const [executeConfirmOpen, setExecuteConfirmOpen] = useState(false);
  const [pendingRollbackIds, setPendingRollbackIds] = useState<string[]>([]);
  const [overrideRecord, setOverrideRecord] = useState<unknown>(null);

  // ── Queries ────────────────────────────────────────────────────────────

  const previewQueryOptions = trpc.promotions.preview.queryOptions({
    classId: promotionClassId ?? 0,
    academicYearId: sourceYearId ?? 0,
  });

  const previewQuery = useQuery({
    ...previewQueryOptions,
    enabled: !!promotionClassId && !!sourceYearId && activeTab === "preview",
  });

  const historyQueryOptions = trpc.promotions.historyRecords.queryOptions({
    classId: promotionClassId ?? undefined,
    academicYearId: sourceYearId ?? undefined,
  });

  const historyQuery = useQuery({
    ...historyQueryOptions,
    enabled: activeTab === "history",
  });

  const statsQueryOptions = trpc.promotions.statistics.queryOptions({
    academicYearId: sourceYearId ?? 0,
    classId: promotionClassId ?? undefined,
  });

  const statsQuery = useQuery({
    ...statsQueryOptions,
    enabled: !!sourceYearId && activeTab === "statistics",
  });

  const classesQuery = useSuspenseQuery(trpc.classes.getAll.queryOptions());

  // ── Mutations ──────────────────────────────────────────────────────────

  const executeMutation = useMutation(
    trpc.promotions.execute.mutationOptions({
      onSuccess: () => {
        toast({ title: t("promotions.process_success") });
        queryClient.invalidateQueries({
          queryKey: historyQueryOptions.queryKey,
        });
        queryClient.invalidateQueries({ queryKey: statsQueryOptions.queryKey });
        setExecuteConfirmOpen(false);
        setActiveTab("history");
      },
      onError: (err) => {
        toast({
          title: t("common.error"),
          description: err.message,
          variant: "destructive",
        });
      },
    }),
  );

  const rollbackMutation = useMutation(
    trpc.promotions.rollback.mutationOptions({
      onSuccess: () => {
        toast({ title: t("promotions.process_success") });
        queryClient.invalidateQueries({
          queryKey: previewQueryOptions.queryKey,
        });
        queryClient.invalidateQueries({
          queryKey: historyQueryOptions.queryKey,
        });
        queryClient.invalidateQueries({ queryKey: statsQueryOptions.queryKey });
      },
      onError: (err) => {
        toast({
          title: t("common.error"),
          description: err.message,
          variant: "destructive",
        });
      },
    }),
  );

  // ── Handlers ───────────────────────────────────────────────────────────

  const handleExecute = useCallback(() => {
    if (
      !promotionClassId ||
      !sourceYearId ||
      !targetYearId ||
      !previewQuery.data
    )
      return;

    const decisionsCount = previewQuery.data?.students?.filter(
      (s) => s.suggestedStatus !== "already_promoted",
    ).length;

    if (!decisionsCount || decisionsCount === 0) {
      toast({
        title: t("common.error"),
        description: t("promotions.no_students_to_promote"),
        variant: "destructive",
      });
      return;
    }

    setExecuteConfirmOpen(true);
  }, [
    promotionClassId,
    sourceYearId,
    targetYearId,
    previewQuery.data,
    toast,
    t,
  ]);

  const confirmExecute = useCallback(() => {
    if (
      !promotionClassId ||
      !sourceYearId ||
      !targetYearId ||
      !previewQuery.data
    )
      return;

    const decisions: PromotionDecision[] =
      previewQuery.data?.students
        ?.filter((s) => s.suggestedStatus !== "already_promoted")
        .map((student) => ({
          studentId: student.studentId,
          status: student.suggestedStatus as PromotionDecision["status"],
          toClassId:
            student.suggestedStatus === "promoted"
              ? previewQuery.data?.nextClass?.id
              : undefined,
        })) ?? [];

    executeMutation.mutate({
      classId: promotionClassId,
      fromAcademicYearId: sourceYearId,
      toAcademicYearId: targetYearId,
      decisions,
    });
  }, [
    promotionClassId,
    sourceYearId,
    targetYearId,
    previewQuery.data,
    executeMutation,
  ]);

  const handleRollbackClick = useCallback((studentIds: string[]) => {
    setPendingRollbackIds(studentIds);
    setRollbackConfirmOpen(true);
  }, []);

  const confirmRollback = useCallback(() => {
    if (!promotionClassId || !sourceYearId) return;

    rollbackMutation.mutate({
      studentIds: pendingRollbackIds,
      classId: promotionClassId,
      academicYearId: sourceYearId,
    });
    setRollbackConfirmOpen(false);
  }, [promotionClassId, sourceYearId, pendingRollbackIds, rollbackMutation]);

  const handleRefresh = useCallback(() => {
    previewQuery.refetch();
    queryClient.invalidateQueries({ queryKey: historyQueryOptions.queryKey });
  }, [previewQuery, queryClient, historyQueryOptions.queryKey]);

  const openOverrideModal = useCallback(
    (record: unknown) => setOverrideRecord(record),
    [],
  );

  const closeOverrideModal = useCallback(() => setOverrideRecord(null), []);

  // ── Derived state ──────────────────────────────────────────────────────

  const canExecute =
    !!previewQuery.data?.students?.length && !!promotionClassId;

  return {
    // Tab
    activeTab,
    setActiveTab,

    // Params
    promotionClassId,
    sourceYearId,
    setParams,

    // Queries
    previewData:
      previewQuery.data?.students?.map((student) => ({
        ...student,
        targetClassId: previewQuery.data?.nextClass?.id ?? null,
        targetClassName: previewQuery.data?.nextClass?.name ?? null,
      })) ?? [],
    isPreviewLoading: previewQuery.isLoading as boolean,
    historyData: (historyQuery.data as unknown[]) ?? [],
    isHistoryLoading: historyQuery.isLoading as boolean,
    statsData: statsQuery.data,
    isStatsLoading: statsQuery.isLoading as boolean,
    classesData: classesQuery.data,

    // Mutations
    isExecuting: executeMutation.isPending,
    isRollingBack: rollbackMutation.isPending,
    canExecute,

    // Handlers
    handleExecute,
    confirmExecute,
    handleRollbackClick,
    confirmRollback,
    handleRefresh,
    openOverrideModal,
    closeOverrideModal,

    // Modal state
    overrideRecord,
    rollbackConfirmOpen,
    setRollbackConfirmOpen,
    executeConfirmOpen,
    setExecuteConfirmOpen,
    pendingRollbackIds,
    studentsCount:
      previewQuery.data?.students?.filter(
        (s) => s.suggestedStatus !== "already_promoted",
      ).length ?? 0,
  } as const;
}
