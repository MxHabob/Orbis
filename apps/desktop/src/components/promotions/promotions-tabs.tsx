"use client";

import { Tabs, TabsContent, TabsList, TabsTrigger } from "@orbis/ui/tabs";
import type { PromotionTab } from "@/hooks/use-promotions";
import { useI18n } from "@/locales/client";
import { PromotionsHistoryDataTable } from "../tables/promotions-history/data-table";
import { PromotionsPreviewDataTable } from "../tables/promotions-preview/data-table";
import { PromotionsStatistics } from "./promotions-statistics";

// ─── Constants ───────────────────────────────────────────────────────────────

const TAB_TRIGGER_CLASS =
  "rounded-none h-full px-6 text-[10px] font-bold tracking-tight data-[state=active]:bg-background data-[state=active]:border-x data-[state=active]:border-t data-[state=active]:border-b-0 data-[state=active]:-mb-[1px] border-transparent";

// ─── Types ───────────────────────────────────────────────────────────────────

interface PromotionsTabsProps {
  activeTab: PromotionTab;
  onTabChange: (tab: PromotionTab) => void;
  // Preview
  previewData: unknown[];
  isPreviewLoading: boolean;
  // History
  historyData: unknown[];
  isHistoryLoading: boolean;
  onRollback: (studentIds: string[]) => void;
  onOverride: (record: unknown) => void;
  isRollingBack: boolean;
  // Statistics
  statsData: unknown;
  isStatsLoading: boolean;
}

// ─── Component ───────────────────────────────────────────────────────────────

export function PromotionsTabs({
  activeTab,
  onTabChange,
  previewData,
  isPreviewLoading,
  historyData,
  isHistoryLoading,
  onRollback,
  onOverride,
  isRollingBack,
  statsData,
  isStatsLoading,
}: PromotionsTabsProps) {
  const t = useI18n();

  return (
    <Tabs
      value={activeTab}
      onValueChange={(val) => onTabChange(val as PromotionTab)}
      className="w-full"
    >
      <TabsList className="bg-muted/50 w-full justify-start h-9 p-0 rounded-none border-b border-border mb-6">
        <TabsTrigger value="preview" className={TAB_TRIGGER_CLASS}>
          {t("promotions.preview")}
        </TabsTrigger>
        <TabsTrigger value="history" className={TAB_TRIGGER_CLASS}>
          {t("promotions.history")}
        </TabsTrigger>
        <TabsTrigger value="statistics" className={TAB_TRIGGER_CLASS}>
          {t("promotions.statistics")}
        </TabsTrigger>
      </TabsList>

      <TabsContent value="preview" className="mt-0 focus-visible:outline-none">
        <PromotionsPreviewDataTable
          data={previewData as any[]}
          isLoading={isPreviewLoading}
        />
      </TabsContent>

      <TabsContent value="history" className="mt-0 focus-visible:outline-none">
        <PromotionsHistoryDataTable
          data={historyData as any[]}
          isLoading={isHistoryLoading}
          onRollback={onRollback}
          onOverride={onOverride as any}
          isRollingBack={isRollingBack}
        />
      </TabsContent>

      <TabsContent
        value="statistics"
        className="mt-0 focus-visible:outline-none"
      >
        <PromotionsStatistics
          data={statsData as any}
          isLoading={isStatsLoading}
        />
      </TabsContent>
    </Tabs>
  );
}
