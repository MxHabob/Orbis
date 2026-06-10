"use client";

import { useOverviewTab } from "@/hooks/use-overview-tab";
import { usePrefetchMetrics } from "@/hooks/use-prefetch-metrics";
import type { AppRouter } from "@orbis/api/trpc/routers/_app";
import { cn } from "@orbis/ui/cn";
import { Skeleton } from "@orbis/ui/skeleton";
import { Tabs, TabsContent } from "@orbis/ui/tabs";
import type { inferRouterOutputs } from "@trpc/server";
import { Suspense } from "react";
import { MetricsView } from "../metrics/metrics-view";
import { WidgetsHeader } from "./header";
import { WidgetProvider, useIsCustomizing } from "./widget-provider";
import { WidgetsGrid } from "./widgets-grid";
import { TrackerCalendar } from "../tracker-calendar";

type RouterOutputs = inferRouterOutputs<AppRouter>;
type WidgetPreferences = RouterOutputs["widgets"]["getWidgetPreferences"];


function WidgetsContent({ weeklyCalendar }: { weeklyCalendar: boolean }) {
  const isCustomizing = useIsCustomizing();
  const { tab, setTab } = useOverviewTab();

  usePrefetchMetrics();


  return (
    <Tabs value={tab} onValueChange={setTab}>
      <div
        className={cn(
          "flex flex-col mt-6",
        )}
      >
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
