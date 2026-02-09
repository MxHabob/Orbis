
"use client";

import { useSchoolMetrics } from "@/hooks/use-school-metrics";
import { useLongPress } from "@/hooks/use-long-press";
import { useMetricsCustomize } from "@/hooks/use-metrics-customize";
import { cn } from "@midday/ui/cn";
import { Cell, Pie, PieChart, ResponsiveContainer, Tooltip, Legend } from "recharts";
import { ShareMetricButton } from "../components/share-metric-button";

interface GenderDistributionCardProps {
  from: string;
  to: string;
  currency?: string;
  locale?: string;
  isCustomizing: boolean;
  wiggleClass?: string;
}

const COLORS = ['#0088FE', '#00C49F', '#FFBB28', '#FF8042'];

export function GenderDistributionCard({ 
  from,
  to,
  currency,
  locale,
  isCustomizing, 
  wiggleClass 
}: GenderDistributionCardProps) {
  const { data } = useSchoolMetrics();
  const { isCustomizing: metricsIsCustomizing, setIsCustomizing } = useMetricsCustomize();
  const genderStats = data?.genderStats ?? [];

  const longPressHandlers = useLongPress({
    onLongPress: () => setIsCustomizing(true),
    threshold: 500,
    disabled: metricsIsCustomizing,
  });

  const chartData = genderStats.map((item, index) => ({
    name: item.sex === 'male' ? 'Male' : item.sex === 'female' ? 'Female' : 'Unspecified',
    value: item.count,
    fill: COLORS[index % COLORS.length]
  }));

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
          <h3 className="text-sm font-normal text-muted-foreground">Gender Distribution</h3>
          <div className="opacity-0 group-hover:opacity-100 group-has-[*[data-state=open]]:opacity-100 transition-opacity">
            <ShareMetricButton
              type="gender_distribution"
              from={from}
              to={to}
              currency={currency}
            />
          </div>
        </div>
        <p className="text-3xl font-normal mb-3">
          {chartData.reduce((acc, curr) => acc + curr.value, 0)}
        </p>
        <p className="text-xs text-muted-foreground">Total records analyzed</p>
      </div>
      
      <div className="h-80 w-full flex items-center justify-center">
        {chartData.length === 0 ? (
           <div className="flex items-center justify-center h-full text-xs text-muted-foreground">
             No data available
           </div>
        ) : (
          <ResponsiveContainer width="100%" height="100%">
            <PieChart>
              <Pie
                data={chartData}
                cx="50%"
                cy="50%"
                innerRadius={80}
                outerRadius={120}
                paddingAngle={5}
                dataKey="value"
                stroke="none"
              >
                {chartData.map((entry, index) => (
                  <Cell key={`cell-${index}`} fill={entry.fill} />
                ))}
              </Pie>
              <Tooltip 
                contentStyle={{ 
                  borderRadius: '0px', 
                  border: '1px solid #1d1d1d',
                  backgroundColor: '#0c0c0c',
                  color: '#fff',
                  fontSize: '10px'
                }}
                itemStyle={{ color: '#fff' }}
              />
              <Legend verticalAlign="bottom" height={36} iconType="circle" />
            </PieChart>
          </ResponsiveContainer>
        )}
      </div>
    </div>
  );
}
