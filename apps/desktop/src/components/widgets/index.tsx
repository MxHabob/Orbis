import { cn } from "@orbis/ui/cn";
import { Skeleton } from "@orbis/ui/skeleton";
import { Tabs, TabsContent } from "@orbis/ui/tabs";
import { useOverviewTab } from "@/hooks/use-overview-tab";
import { usePrefetchMetrics } from "@/hooks/use-prefetch-metrics";
import type { WidgetPreferences } from "@/types/widgets";
import { MetricsView } from "../metrics/metrics-view";
import { WidgetsHeader } from "./header";
import { useIsCustomizing, WidgetProvider } from "./widget-provider";
import { WidgetsGrid } from "./widgets-grid";

function WidgetsContent() {
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
            <p className="text-sm text-muted-foreground py-8 text-center">
              Calendar view is not available in the desktop app yet.
            </p>
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

export function Widgets({ initialPreferences }: WidgetsProps) {
  return (
    <WidgetProvider initialPreferences={initialPreferences}>
      <WidgetsContent />
    </WidgetProvider>
  );
}
