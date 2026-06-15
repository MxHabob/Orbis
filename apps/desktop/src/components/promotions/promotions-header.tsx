import { Button } from "@orbis/ui/button";
import { cn } from "@orbis/ui/cn";
import { Icons } from "@orbis/ui/icons";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@orbis/ui/select";
import { useEffect } from "react";
import { usePromotionsParams } from "@/hooks/use-promotions-params";
import { useI18n } from "@/locales/client";

interface PromotionsHeaderProps {
  initialData: {
    classes: any[];
    years: any[];
  };
  currentYearId?: number;
  onExecute: () => void;
  isExecuting: boolean;
  canExecute: boolean;
  onRefresh: () => void;
  isRefreshing: boolean;
}

export function PromotionsHeader({
  initialData,
  currentYearId,
  onExecute,
  isExecuting,
  canExecute,
  onRefresh,
  isRefreshing,
}: PromotionsHeaderProps) {
  const t = useI18n();
  const { promotionClassId, sourceYearId, targetYearId, setParams } =
    usePromotionsParams();

  // Initialization
  useEffect(() => {
    if (!sourceYearId && currentYearId) {
      setParams({ sourceYearId: currentYearId });
    }

    if (sourceYearId && !targetYearId) {
      const sourceYearIndex = initialData.years.findIndex(
        (y: any) => y.id === sourceYearId,
      );
      if (
        sourceYearIndex !== -1 &&
        sourceYearIndex < initialData.years.length - 1
      ) {
        setParams({ targetYearId: initialData.years[sourceYearIndex + 1].id });
      }
    }
  }, [sourceYearId, currentYearId, targetYearId, setParams, initialData.years]);

  return (
    <div className="flex flex-col gap-4 py-4 border-b">
      <div className="flex flex-wrap items-end gap-6">
        <div className="flex flex-col gap-2">
          <label className="text-[11px] text-[#606060] font-medium">
            {t("promotions.current_year")}
          </label>
          <Select
            value={sourceYearId?.toString() || ""}
            onValueChange={(val) => setParams({ sourceYearId: parseInt(val) })}
          >
            <SelectTrigger className="h-8 bg-transparent min-w-[140px] text-xs border-border rounded-none shadow-none">
              <SelectValue placeholder={t("promotions.current_year")} />
            </SelectTrigger>
            <SelectContent>
              {initialData.years.map((y: any) => (
                <SelectItem
                  key={y.id}
                  value={y.id.toString()}
                  className="text-xs"
                >
                  {y.yearName}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>

        <div className="flex flex-col gap-2">
          <label className="text-[11px] text-[#606060] font-medium">
            {t("promotions.target_year")}
          </label>
          <Select
            value={targetYearId?.toString() || ""}
            onValueChange={(val) => setParams({ targetYearId: parseInt(val) })}
          >
            <SelectTrigger className="h-8 bg-transparent min-w-[140px] text-xs border-border rounded-none shadow-none">
              <SelectValue placeholder={t("promotions.target_year")} />
            </SelectTrigger>
            <SelectContent>
              {initialData.years.map((y: any) => (
                <SelectItem
                  key={y.id}
                  value={y.id.toString()}
                  className="text-xs"
                >
                  {y.yearName}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>

        <div className="flex flex-col gap-2">
          <label className="text-[11px] text-[#606060] font-medium">
            {t("students.class")}
          </label>
          <Select
            value={promotionClassId?.toString() || ""}
            onValueChange={(val) =>
              setParams({ promotionClassId: parseInt(val) })
            }
          >
            <SelectTrigger className="h-8 bg-transparent min-w-[180px] text-xs border-border rounded-none shadow-none">
              <SelectValue placeholder={t("students.class")} />
            </SelectTrigger>
            <SelectContent>
              {initialData.classes.map((c: any) => (
                <SelectItem
                  key={c.id}
                  value={c.id.toString()}
                  className="text-xs"
                >
                  {c.name} {c.division ? `- ${c.division}` : ""}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>

        <div className="flex items-center gap-3 ms-auto mb-0.5">
          <Button
            variant="ghost"
            size="icon"
            className="h-8 w-8 hover:bg-accent rounded-none"
            onClick={onRefresh}
            disabled={isRefreshing || !promotionClassId}
          >
            <Icons.RefreshOutline
              className={cn(
                "h-4 w-4 text-[#606060]",
                isRefreshing && "animate-spin",
              )}
            />
          </Button>

          <Button
            onClick={onExecute}
            disabled={isExecuting || !canExecute}
            variant="default"
            className="h-8 px-4 text-xs rounded-none"
          >
            {isExecuting ? (
              <Icons.RefreshOutline className="w-3.5 h-3.5 mr-2 animate-spin" />
            ) : (
              <Icons.TrendingUp size={14} className="mr-2" />
            )}
            {t("promotions.execute")}
          </Button>
        </div>
      </div>
    </div>
  );
}
