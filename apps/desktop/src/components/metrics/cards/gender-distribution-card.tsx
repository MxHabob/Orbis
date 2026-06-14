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

interface GenderDistributionCardProps {
  from: string;
  to: string;
  currency?: string;
  locale?: string;
  isCustomizing: boolean;
  wiggleClass?: string;
}

const COLORS = [
  "hsl(var(--primary))",
  "hsl(var(--primary) / 0.6)",
  "hsl(var(--primary) / 0.3)",
];

export function GenderDistributionCard({
  from,
  to,
  currency,
  locale,
  isCustomizing,
  wiggleClass,
}: GenderDistributionCardProps) {
  const { data } = useSchoolMetrics();
  const { isCustomizing: metricsIsCustomizing, setIsCustomizing } =
    useMetricsCustomize();
  const genderStats = data?.genderStats ?? [];

  const longPressHandlers = useLongPress({
    onLongPress: () => setIsCustomizing(true),
    threshold: 500,
    disabled: metricsIsCustomizing,
  });

  const chartData = genderStats.map((item) => ({
    name:
      item.sex === "male" ? "Male" : item.sex === "female" ? "Female" : "Other",
    count: item.count,
  }));

  const total = chartData.reduce((acc, curr) => acc + curr.count, 0);

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
            Gender Distribution
          </h3>
          <div className="opacity-0 group-hover:opacity-100 group-has-[*[data-state=open]]:opacity-100 transition-opacity">
            <ShareMetricButton
              type="gender_distribution"
              from={from}
              to={to}
              currency={currency}
            />
          </div>
        </div>
        <div className="flex items-baseline gap-2 mb-3 mt-1">
          <p className="text-3xl font-normal">
            <AnimatedNumber
              value={total}
              maximumFractionDigits={0}
              locale={locale}
            />
          </p>
          <span className="text-sm font-normal text-muted-foreground">
            Total Students
          </span>
        </div>

        <div className="flex items-center gap-4 mt-4">
          {chartData.map((item, index) => (
            <div key={item.name} className="flex gap-2 items-center">
              <div
                className="w-2 h-2 rounded-full"
                style={{ backgroundColor: COLORS[index % COLORS.length] }}
              />
              <span className="text-xs text-muted-foreground">{item.name}</span>
            </div>
          ))}
        </div>
      </div>

      <div className="h-80 w-full mt-4">
        {chartData.length === 0 ? (
          <div className="flex items-center justify-center h-full text-xs text-muted-foreground border border-dashed w-full">
            No data available
          </div>
        ) : (
          <ResponsiveContainer width="100%" height="100%">
            <BarChart
              data={chartData}
              margin={{ top: 10, right: 10, left: -25, bottom: 0 }}
            >
              <CartesianGrid
                vertical={false}
                stroke="hsl(var(--border))"
                strokeDasharray="3 3"
              />
              <XAxis
                dataKey="name"
                tick={{
                  fontSize: 10,
                  fill: "hsl(var(--muted-foreground))",
                  fontFamily: "inherit",
                }}
                tickLine={false}
                axisLine={false}
                dy={10}
              />
              <YAxis
                tick={{
                  fontSize: 10,
                  fill: "hsl(var(--muted-foreground))",
                  fontFamily: "inherit",
                }}
                tickLine={false}
                axisLine={false}
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
              <Bar dataKey="count" radius={[2, 2, 0, 0]} barSize={40}>
                {chartData.map((entry, index) => (
                  <Cell
                    key={`cell-${index}`}
                    fill={COLORS[index % COLORS.length]}
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
