"use client";

import { cn } from "@orbis/ui/cn";
import {
  Bar,
  BarChart,
  CartesianGrid,
  Cell,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts";
import { AnimatedNumber } from "@/components/animated-number";
import { useLongPress } from "@/hooks/use-long-press";
import { useMetricsCustomize } from "@/hooks/use-metrics-customize";
import { useSchoolMetrics } from "@/hooks/use-school-metrics";
import { ShareMetricButton } from "../components/share-metric-button";

interface ClassDistributionCardProps {
  from: string;
  to: string;
  currency?: string;
  locale?: string;
  isCustomizing: boolean;
  wiggleClass?: string;
}

export function ClassDistributionCard({
  from,
  to,
  currency,
  locale,
  isCustomizing,
  wiggleClass,
}: ClassDistributionCardProps) {
  const { data } = useSchoolMetrics();
  const { isCustomizing: metricsIsCustomizing, setIsCustomizing } =
    useMetricsCustomize();
  const classStats = data?.classStats ?? [];

  const longPressHandlers = useLongPress({
    onLongPress: () => setIsCustomizing(true),
    threshold: 500,
    disabled: metricsIsCustomizing,
  });

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
            Class Distribution
          </h3>
          <div className="opacity-0 group-hover:opacity-100 group-has-[*[data-state=open]]:opacity-100 transition-opacity">
            <ShareMetricButton
              type="class_distribution"
              from={from}
              to={to}
              currency={currency}
            />
          </div>
        </div>
        <div className="flex items-baseline gap-2 mb-3 mt-1">
          <p className="text-3xl font-normal">
            <AnimatedNumber
              value={classStats.length}
              maximumFractionDigits={0}
              locale={locale}
            />
          </p>
          <span className="text-sm font-normal text-muted-foreground">
            Active Classes
          </span>
        </div>

        <div className="flex items-center gap-4 mt-4">
          <div className="flex gap-2 items-center">
            <div className="w-2 h-2 bg-primary rounded-full" />
            <span className="text-xs text-muted-foreground">
              Student Population
            </span>
          </div>
        </div>
      </div>

      <div className="h-80 w-full mt-4">
        {classStats.length === 0 ? (
          <div className="flex items-center justify-center h-full text-xs text-muted-foreground border border-dashed">
            No data available
          </div>
        ) : (
          <ResponsiveContainer width="100%" height="100%">
            <BarChart
              data={classStats}
              layout="vertical"
              margin={{ top: 10, right: 30, left: 40, bottom: 0 }}
            >
              <CartesianGrid
                horizontal={false}
                stroke="hsl(var(--border))"
                strokeDasharray="3 3"
              />
              <XAxis type="number" hide />
              <YAxis
                type="category"
                dataKey="className"
                tick={{
                  fontSize: 10,
                  fill: "hsl(var(--muted-foreground))",
                  fontFamily: "inherit",
                }}
                tickLine={false}
                axisLine={false}
                width={80}
              />
              <Tooltip
                cursor={{ fill: "hsl(var(--muted) / 0.5)" }}
                contentStyle={{
                  borderRadius: "0px",
                  border: "1px solid hsl(var(--border))",
                  backgroundColor: "hsl(var(--background))",
                  color: "hsl(var(--foreground))",
                  fontSize: "10px",
                  fontFamily: "inherit",
                }}
                itemStyle={{ color: "hsl(var(--foreground))" }}
              />
              <Bar dataKey="studentCount" radius={[0, 2, 2, 0]} barSize={20}>
                {classStats.map((entry, index) => (
                  <Cell
                    key={`cell-${index}`}
                    fill={`hsl(var(--primary) / ${1 - index * 0.08})`}
                  />
                ))}
              </Bar>
            </BarChart>
          </ResponsiveContainer>
        )}
      </div>
    </div>
  );
}
