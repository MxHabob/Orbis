
"use client";

import { AnimatedNumber } from "@/components/animated-number";
import { SchoolMetricChart, type SchoolMetricData } from "@/components/charts/school-metric-chart";
import { useSchoolMetrics } from "@/hooks/use-school-metrics";
import { useLongPress } from "@/hooks/use-long-press";
import { useMetricsCustomize } from "@/hooks/use-metrics-customize";
import { cn } from "@orbis/ui/cn";
import { format, subMonths } from "date-fns";
import { useMemo } from "react";
import { ShareMetricButton } from "../components/share-metric-button";

interface OverallPerformanceCardProps {
  from: string;
  to: string;
  currency?: string;
  locale?: string;
  isCustomizing: boolean;
  wiggleClass?: string;
}

export function OverallPerformanceCard({ 
  from,
  to,
  currency,
  locale,
  isCustomizing, 
  wiggleClass 
}: OverallPerformanceCardProps) {
  const { data } = useSchoolMetrics();
  const { isCustomizing: metricsIsCustomizing, setIsCustomizing } = useMetricsCustomize();
  const performance = (data as any)?.performance ?? 0;

  const longPressHandlers = useLongPress({
    onLongPress: () => setIsCustomizing(true),
    threshold: 500,
    disabled: metricsIsCustomizing,
  });

  // Generate mock history
  const chartData = useMemo(() => {
    if (!performance) return [];
    const months = 12;
    const history: SchoolMetricData[] = [];
    for (let i = months - 1; i >= 0; i--) {
      const date = subMonths(new Date(), i);
      const randomVar = (Math.random() - 0.5) * 4;
      let value = performance + randomVar;
      value = Math.min(100, Math.max(0, value));
      history.push({
        month: format(date, "MMM"),
        value: i === 0 ? performance : value,
        average: 0
      });
    }
    const total = history.reduce((sum, item) => sum + item.value, 0);
    const average = total / months;
    history.forEach(item => item.average = average);
    return history;
  }, [performance]);

  return (
    <div
      className={cn(
        "border bg-background border-border p-6 flex flex-col h-full relative group min-h-[460px]",
        !metricsIsCustomizing && "cursor-pointer",
        wiggleClass
      )}
      {...longPressHandlers}
    >
      <div className="mb-4 min-h-[140px]">
        <div className="flex items-start justify-between h-7">
          <h3 className="text-sm font-normal text-muted-foreground">Overall Performance</h3>
          <div className="opacity-0 group-hover:opacity-100 group-has-[*[data-state=open]]:opacity-100 transition-opacity">
            <ShareMetricButton
              type="overall_performance"
              from={from}
              to={to}
              currency={currency}
            />
          </div>
        </div>
        <div className="flex items-end gap-1 mb-3">
            <p className="text-3xl font-normal">
              <AnimatedNumber value={performance} maximumFractionDigits={1} locale={locale} />
            </p>
            <span className="text-xl pb-1 text-muted-foreground">GPA</span>
        </div>
        <div className="flex items-center gap-4 mt-2">
           <div className="flex gap-2 items-center">
             <div className="w-2 h-2 bg-primary rounded-full" />
             <span className="text-xs text-muted-foreground">Avg Grade</span>
          </div>
           <div className="flex gap-2 items-center">
            <div
              className="w-4 h-0.5"
              style={{ borderTop: "2px dashed hsl(var(--muted-foreground))" }}
            />
            <span className="text-xs text-muted-foreground">School Average</span>
          </div>
        </div>
      </div>
      
       <div className="h-80 w-full">
         <SchoolMetricChart 
            data={chartData} 
            height={320} 
            currency={currency}
            locale={locale}
            valueFormatter={(val) => val.toFixed(1)}
         />
      </div>
    </div>
  );
}
