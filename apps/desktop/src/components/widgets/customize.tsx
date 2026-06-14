"use client";

import { Button } from "@orbis/ui/button";
import { Icons } from "@orbis/ui/icons";
import { useLocation } from "react-router-dom";
import { useMetricsCustomize } from "@/hooks/use-metrics-customize";
import { useOverviewTab } from "@/hooks/use-overview-tab";
import { useIsCustomizing, useWidgetActions } from "./widget-provider";

export function Customize() {
  const { pathname } = useLocation();
  const { isMetricsTab } = useOverviewTab();
  const widgetsIsCustomizing = useIsCustomizing();
  const { setIsCustomizing: setWidgetsCustomizing } = useWidgetActions();
  const {
    isCustomizing: metricsIsCustomizing,
    setIsCustomizing: setMetricsCustomizing,
  } = useMetricsCustomize();

  const isOnRootPath = pathname === "/" || pathname === "";

  if (!isOnRootPath) {
    return null;
  }

  const isCustomizing =
    isMetricsTab || !isMetricsTab // Since we merged metrics into overview, overview now has metrics
      ? metricsIsCustomizing
      : widgetsIsCustomizing;

  const handleToggle = () => {
    if (isMetricsTab || true) {
      // Always toggle metrics for now as it's the main view
      setMetricsCustomizing(!metricsIsCustomizing);
    } else {
      setWidgetsCustomizing(!widgetsIsCustomizing);
    }
  };

  return (
    <Button
      variant="outline"
      size="icon"
      className="h-9 w-9"
      onClick={handleToggle}
    >
      {isCustomizing ? (
        <Icons.Check size={16} />
      ) : (
        <Icons.DashboardCustomize size={16} />
      )}
    </Button>
  );
}
