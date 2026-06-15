"use client";

import { cn } from "@orbis/ui/cn";
import { Skeleton } from "@orbis/ui/skeleton";
import { Tabs, TabsContent } from "@orbis/ui/tabs";
import { Suspense } from "react";
import { useOverviewTab } from "@/hooks/use-overview-tab";
import { usePrefetchMetrics } from "@/hooks/use-prefetch-metrics";
import type { WidgetPreferences } from "@/types/widgets";
import { MetricsView } from "../metrics/metrics-view";
import { TrackerCalendar } from "../tracker-calendar";
import { WidgetsHeader } from "./header";
import { useIsCustomizing, WidgetProvider } from "./widget-provider";
import { WidgetsGrid } from "./widgets-grid";

function WidgetsContent({ weeklyCalendar }: { weeklyCalendar: boolean }) {
  const isCustomizing = useIsCustomizing();
  const { tab, setTab } = useOverviewTab();

  usePrefetchMetrics();

  return (
    <Tabs value={tab} onValueChange={setTab}>
      <div className={cn("flex flex-col mt-6")}>
        <WidgetsHeader />
        <TabsContent value="calendar">
          <WidgetsGrid />
          {!isCustomizing && (
            <Suspense fallback={<Skeleton />}>
              <TrackerCalendar weeklyCalendar={weeklyCalendar} />
            </Suspense>
          )}
        </TabsContent>
        <TabsContent value="overview">
          <MetricsView />
        </TabsContent>
      </div>
    </Tabs>
  );
}

interface WidgetsProps {
  initialPreferences: WidgetPreferences;
  weeklyCalendar: boolean;
}

export function Widgets({ initialPreferences, weeklyCalendar }: WidgetsProps) {
  return (
    <WidgetProvider initialPreferences={initialPreferences}>
      <WidgetsContent weeklyCalendar={weeklyCalendar} />
    </WidgetProvider>
  );
}
