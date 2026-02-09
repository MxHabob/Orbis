
"use client";

import { useLocalStorage } from "@/hooks/use-local-storage";
import { useMetricsCustomize } from "@/hooks/use-metrics-customize";
import { useMetricsFilter } from "@/hooks/use-metrics-filter";
import { useMemo, useRef } from "react";
import { useOnClickOutside } from "usehooks-ts";
import { AttendanceRateCard } from "./cards/attendance-rate-card";
import { ClassDistributionCard } from "./cards/class-distribution-card";
import { GenderDistributionCard } from "./cards/gender-distribution-card";
import { OverallPerformanceCard } from "./cards/overall-performance-card";
import { StudentCountCard } from "./cards/student-count-card";
import { TeacherCountCard } from "./cards/teacher-count-card";
import { MetricsGrid } from "./components/metrics-grid";
import { SortableChartCard } from "./components/sortable-chart-card";
import { type ChartId, DEFAULT_CHART_ORDER } from "./utils/chart-types";
import { useParams } from "next/navigation";

export function MetricsView() {
  const params = useParams();
  const locale = params.locale as string;
  // We keep filter hooks to avoid breaking context or if we want to add date filtering later
  const { from, to, currency, revenueType } = useMetricsFilter();
  const { isCustomizing, setIsCustomizing } = useMetricsCustomize();
  const [chartOrder, setChartOrder] = useLocalStorage<ChartId[]>(
    "metrics-chart-order-school", // Changed key to reset order for new charts
    DEFAULT_CHART_ORDER,
  );
  const gridRef = useRef<HTMLDivElement>(null!);


  // Ensure all charts are in the order (handle new charts being added)
  const orderedCharts = useMemo(() => {
    const orderSet = new Set(chartOrder);
    // Filter out any old chart types that might be in local storage but not in our new set
    const validCharts = chartOrder.filter(id => DEFAULT_CHART_ORDER.includes(id));
    const missing = DEFAULT_CHART_ORDER.filter((id) => !validCharts.includes(id));
    return [...validCharts, ...missing];
  }, [chartOrder]);

  useOnClickOutside(gridRef, (event) => {
    if (isCustomizing) {
      const target = event.target as Element;
      if (!target.closest("[data-no-close]")) {
        setIsCustomizing(false);
      }
    }
  });

  // Get wiggle class for customize mode
  const getWiggleClass = (index: number) => {
    if (!isCustomizing) return "";
    const wiggleIndex = (index % 7) + 1;
    return `wiggle-${wiggleIndex}`;
  };

  // Chart component mapping
  const renderChart = (chartId: ChartId, index: number) => {
    const wiggleClass = getWiggleClass(index);
    const commonProps = {
      isCustomizing,
      wiggleClass,
      from,
      to,
      currency,
      locale,
    };

    const chartContent = (() => {
      switch (chartId) {
        case "student-count":
          return <StudentCountCard {...commonProps} />;
        case "teacher-count":
          return <TeacherCountCard {...commonProps} />;
        case "attendance-rate":
          return <AttendanceRateCard {...commonProps} />;
        case "gender-distribution":
          return <GenderDistributionCard {...commonProps} />;
        case "class-distribution":
          return <ClassDistributionCard {...commonProps} />;
        case "overall-performance":
          return <OverallPerformanceCard {...commonProps} />;
        default:
          return null;
      }
    })();

    if (isCustomizing) {
      return (
        <SortableChartCard
          key={chartId}
          id={chartId}
          className={isCustomizing ? "cursor-grab active:cursor-grabbing" : ""}
          customizeMode={isCustomizing}
          wiggleClass={wiggleClass}
        >
          {chartContent}
        </SortableChartCard>
      );
    }

    return <div key={chartId} className="h-full">{chartContent}</div>;
  };

  return (
    <div className="flex flex-col gap-6" ref={gridRef}>
      <MetricsGrid
        orderedCharts={orderedCharts}
        isCustomizing={isCustomizing}
        onChartOrderChange={setChartOrder}
        renderChart={renderChart}
        getWiggleClass={getWiggleClass}
      />
    </div>
  );
}
