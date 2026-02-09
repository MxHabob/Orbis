
"use client";

import { useSchoolMetrics } from "@/hooks/use-school-metrics";
import { useLongPress } from "@/hooks/use-long-press";
import { useMetricsCustomize } from "@/hooks/use-metrics-customize";
import { cn } from "@midday/ui/cn";
import { Bar, BarChart, CartesianGrid, Cell, ResponsiveContainer, Tooltip, XAxis, YAxis } from "recharts";
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
  wiggleClass 
}: ClassDistributionCardProps) {
  const { data } = useSchoolMetrics();
  const { isCustomizing: metricsIsCustomizing, setIsCustomizing } = useMetricsCustomize();
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
        wiggleClass
      )}
      {...longPressHandlers}
    >
      <div className="mb-4 min-h-[140px]">
        <div className="flex items-start justify-between h-7">
          <h3 className="text-sm font-normal text-muted-foreground">Class Distribution</h3>
          <div className="opacity-0 group-hover:opacity-100 group-has-[*[data-state=open]]:opacity-100 transition-opacity">
            <ShareMetricButton
              type="class_distribution"
              from={from}
              to={to}
              currency={currency}
            />
          </div>
        </div>
        <p className="text-3xl font-normal mb-3">
          {classStats.length}
          <span className="text-sm font-normal text-muted-foreground ml-2">Active Classes</span>
        </p>
        <p className="text-xs text-muted-foreground">Student distribution across sections</p>
      </div>
      
      <div className="h-80 w-full">
         {classStats.length === 0 ? (
           <div className="flex items-center justify-center h-full text-xs text-muted-foreground">
             No data available
           </div>
        ) : (
          <ResponsiveContainer width="100%" height="100%">
            <BarChart data={classStats} margin={{ top: 20, right: 20, left: -20, bottom: 0 }}>
              <CartesianGrid strokeDasharray="3 3" vertical={false} stroke="hsl(var(--border))" />
              <XAxis 
                dataKey="className" 
                tick={{ fontSize: 10, fill: 'hsl(var(--muted-foreground))' }} 
                tickLine={false}
                axisLine={false}
                interval={0}
                dy={10}
              />
              <YAxis 
                tick={{ fontSize: 10, fill: 'hsl(var(--muted-foreground))' }} 
                tickLine={false}
                axisLine={false}
              />
              <Tooltip 
                cursor={{ fill: 'transparent' }}
                contentStyle={{ 
                  borderRadius: '0px', 
                  border: '1px solid #1d1d1d',
                  backgroundColor: '#0c0c0c',
                  color: '#fff',
                  fontSize: '10px'
                }}
                itemStyle={{ color: '#fff' }}
              />
              <Bar dataKey="studentCount" fill="hsl(var(--primary))" radius={[4, 4, 0, 0]} barSize={40}>
                 {classStats.map((entry, index) => (
                    <Cell key={`cell-${index}`} fill={index % 2 === 0 ? "hsl(var(--primary))" : "hsl(var(--primary) / 0.7)"} />
                 ))}
              </Bar>
            </BarChart>
          </ResponsiveContainer>
        )}
      </div>
    </div>
  );
}
