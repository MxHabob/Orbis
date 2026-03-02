"use client";

import { useI18n } from "@/locales/client";
import { usePromotionsParams } from "@/hooks/use-promotions-params";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@midday/ui/select";
import { Button } from "@midday/ui/button";
import { Icons } from "@midday/ui/icons";
import { useEffect } from "react";
import { cn } from "@midday/ui/cn";

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
  isRefreshing
}: PromotionsHeaderProps) {
  const t = useI18n();
  const { promotionClassId, sourceYearId, targetYearId, setParams } = usePromotionsParams();

  // Initialization
  useEffect(() => {
    if (!sourceYearId && currentYearId) {
       setParams({ sourceYearId: currentYearId });
    }
    
    if (sourceYearId && !targetYearId) {
      const sourceYearIndex = initialData.years.findIndex((y: any) => y.id === sourceYearId);
      if (sourceYearIndex !== -1 && sourceYearIndex < initialData.years.length - 1) {
        setParams({ targetYearId: initialData.years[sourceYearIndex + 1].id });
      }
    }
  }, [sourceYearId, currentYearId, targetYearId, setParams, initialData.years]);

  return (
    <div className="flex flex-col gap-4 py-4">
      <div className="flex flex-wrap items-end gap-4">
        <div className="flex flex-col gap-1.5">
          <label className="text-[10px] uppercase tracking-wider text-muted-foreground font-bold px-1">
            {t("promotions.current_year")}
          </label>
          <Select
            value={sourceYearId?.toString() || ""}
            onValueChange={(val) => setParams({ sourceYearId: parseInt(val) })}
          >
            <SelectTrigger className="h-9 bg-background min-w-[140px] text-xs">
              <SelectValue placeholder={t("promotions.current_year")} />
            </SelectTrigger>
            <SelectContent>
              {initialData.years.map((y: any) => (
                <SelectItem key={y.id} value={y.id.toString()} className="text-xs">
                  {y.yearName}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>

        <div className="flex flex-col gap-1.5">
           <label className="text-[10px] uppercase tracking-wider text-muted-foreground font-bold px-1">
            {t("promotions.target_year")}
          </label>
          <Select
            value={targetYearId?.toString() || ""}
            onValueChange={(val) => setParams({ targetYearId: parseInt(val) })}
          >
            <SelectTrigger className="h-9 bg-background min-w-[140px] text-xs">
              <SelectValue placeholder={t("promotions.target_year")} />
            </SelectTrigger>
            <SelectContent>
              {initialData.years.map((y: any) => (
                <SelectItem key={y.id} value={y.id.toString()} className="text-xs">
                  {y.yearName}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>

        <div className="flex flex-col gap-1.5">
           <label className="text-[10px] uppercase tracking-wider text-muted-foreground font-bold px-1">
            {t("students.class")}
          </label>
          <Select
            value={promotionClassId?.toString() || ""}
            onValueChange={(val) => setParams({ promotionClassId: parseInt(val) })}
          >
            <SelectTrigger className="h-9 bg-background min-w-[180px] text-xs">
              <SelectValue placeholder={t("students.class")} />
            </SelectTrigger>
            <SelectContent>
              {initialData.classes.map((c: any) => (
                <SelectItem key={c.id} value={c.id.toString()} className="text-xs">
                  {c.name} {c.division ? `- ${c.division}` : ""}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>

        <div className="flex items-center gap-2 ms-auto">
          <Button 
            variant="outline" 
            size="icon" 
            className="h-9 w-9"
            onClick={onRefresh}
            disabled={isRefreshing || !promotionClassId}
          >
            <Icons.RefreshOutline className={cn("h-4 w-4", isRefreshing && "animate-spin")} />
          </Button>

          <Button 
            onClick={onExecute}
            disabled={isExecuting || !canExecute}
            className="h-9 px-4 text-xs font-semibold"
          >
            {isExecuting ? <Icons.RefreshOutline className="w-3.5 h-3.5 mr-2 animate-spin" /> : <Icons.TrendingUp size={14} className="mr-2" />}
            {t("promotions.execute")}
          </Button>
        </div>
      </div>
    </div>
  );
}
