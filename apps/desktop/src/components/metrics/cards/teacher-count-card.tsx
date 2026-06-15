import { cn } from "@orbis/ui/cn";
import { format, subMonths } from "date-fns";
import { useMemo } from "react";
import { AnimatedNumber } from "@/components/animated-number";
import {
  SchoolMetricChart,
  type SchoolMetricData,
} from "@/components/charts/school-metric-chart";
import { useLongPress } from "@/hooks/use-long-press";
import { useMetricsCustomize } from "@/hooks/use-metrics-customize";
import { useSchoolMetrics } from "@/hooks/use-school-metrics";
import { ShareMetricButton } from "../components/share-metric-button";

interface TeacherCountCardProps {
  from: string;
  to: string;
  currency?: string;
  locale?: string;
  isCustomizing: boolean;
  wiggleClass?: string;
}

export function TeacherCountCard({
  from,
  to,
  currency,
  locale,
  isCustomizing,
  wiggleClass,
}: TeacherCountCardProps) {
  const { data } = useSchoolMetrics();
  const { isCustomizing: metricsIsCustomizing, setIsCustomizing } =
    useMetricsCustomize();
  const count = data?.teacherCount ?? 0;

  const longPressHandlers = useLongPress({
    onLongPress: () => setIsCustomizing(true),
    threshold: 500,
    disabled: metricsIsCustomizing,
  });

  // Generate mock history
  const chartData = useMemo(() => {
    if (!count) return [];
    const months = 12;
    const history: SchoolMetricData[] = [];
    const baseCount = Math.max(1, Math.round(count * 0.9));
    for (let i = months - 1; i >= 0; i--) {
      const date = subMonths(new Date(), i);
      const progress = (months - i) / months;
      const value = Math.round(baseCount + (count - baseCount) * progress);
      history.push({
        month: format(date, "MMM"),
        value: i === 0 ? count : value,
        average: 0,
      });
    }
    const total = history.reduce((sum, item) => sum + item.value, 0);
    const average = Math.round(total / months);
    history.forEach((item) => (item.average = average));
    return history;
  }, [count]);

  return (
    <div
      className={cn(
        "border bg-background border-border p-6 flex flex-col h-full relative group min-h-[460px]",
        !metricsIsCustomizing && "cursor-pointer",
        wiggleClass,
      )}
      {...longPressHandlers}
    >
      <div className="mb-4 min-h-[140px]">
        <div className="flex items-start justify-between h-7">
          <h3 className="text-sm font-normal text-muted-foreground">
            Total Teachers
          </h3>
          <div className="opacity-0 group-hover:opacity-100 group-has-[*[data-state=open]]:opacity-100 transition-opacity">
            <ShareMetricButton
              type="teacher_count"
              from={from}
              to={to}
              currency={currency}
            />
          </div>
        </div>
        <p className="text-3xl font-normal mb-3">
          <AnimatedNumber
            value={count}
            maximumFractionDigits={0}
            locale={locale}
          />
        </p>
        <div className="flex items-center gap-4 mt-2">
          <div className="flex gap-2 items-center">
            <div className="w-2 h-2 bg-primary rounded-full" />
            <span className="text-xs text-muted-foreground">
              Staff Employment
            </span>
          </div>
          <div className="flex gap-2 items-center">
            <div
              className="w-4 h-0.5"
              style={{ borderTop: "2px dashed hsl(var(--muted-foreground))" }}
            />
            <span className="text-xs text-muted-foreground">Average</span>
          </div>
        </div>
      </div>

      <div className="h-80 w-full">
        <SchoolMetricChart
          data={chartData}
          height={320}
          currency={currency}
          locale={locale}
        />
      </div>
    </div>
  );
}
