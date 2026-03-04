"use client";

import { usePromotions } from "@/hooks/use-promotions";
import { PromotionsHeader } from "./promotions-header";
import { PromotionsTabs } from "./promotions-tabs";
import { RollbackConfirmDialog } from "../modals/promotions/rollback-confirm-dialog";
import { OverridePromotionModal } from "../modals/promotions/override-modal";

// ─── Types ───────────────────────────────────────────────────────────────────

interface PromotionsClientProps {
  initialData: {
    classes: Array<{ id: number; name: string; division?: string }>;
    years: Array<{ id: number; yearName: string }>;
  };
  currentYearId?: number;
}

// ─── Component ───────────────────────────────────────────────────────────────

export function PromotionsClient({
  initialData,
  currentYearId,
}: PromotionsClientProps) {
  const {
    // Tab
    activeTab,
    setActiveTab,

    // Params
    promotionClassId,

    // Queries
    previewData,
    isPreviewLoading,
    historyData,
    isHistoryLoading,
    statsData,
    isStatsLoading,
    classesData,

    // Mutations
    isExecuting,
    isRollingBack,
    canExecute,

    // Handlers
    handleExecute,
    handleRollbackClick,
    confirmRollback,
    handleRefresh,
    openOverrideModal,
    closeOverrideModal,

    // Modal state
    overrideRecord,
    rollbackConfirmOpen,
    setRollbackConfirmOpen,
    pendingRollbackIds,
  } = usePromotions();

  return (
    <div className="flex flex-col h-full bg-background">
      {/* ── Header with filters & actions ── */}
      <PromotionsHeader
        initialData={initialData}
        currentYearId={currentYearId}
        onExecute={handleExecute}
        isExecuting={isExecuting}
        canExecute={canExecute}
        onRefresh={handleRefresh}
        isRefreshing={isPreviewLoading}
      />

      {/* ── Tabbed content area ── */}
      <div className="flex-1 overflow-auto p-6 pt-0">
        <PromotionsTabs
          activeTab={activeTab}
          onTabChange={setActiveTab}
          previewData={previewData}
          isPreviewLoading={isPreviewLoading}
          historyData={historyData}
          isHistoryLoading={isHistoryLoading}
          onRollback={handleRollbackClick}
          onOverride={openOverrideModal}
          isRollingBack={isRollingBack}
          statsData={statsData}
          isStatsLoading={isStatsLoading}
        />
      </div>

      {/* ── Modals ── */}
      <OverridePromotionModal
        open={!!overrideRecord}
        onOpenChange={(open) => !open && closeOverrideModal()}
        record={overrideRecord as any}
        classes={(classesData as any)?.data ?? []}
      />

      <RollbackConfirmDialog
        open={rollbackConfirmOpen}
        onOpenChange={setRollbackConfirmOpen}
        studentCount={pendingRollbackIds.length}
        onConfirm={confirmRollback}
      />
    </div>
  );
}
