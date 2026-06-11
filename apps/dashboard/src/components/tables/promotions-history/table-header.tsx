"use client";

import { Button } from "@orbis/ui/button";
import { ArrowRight, RotateCcw, School } from "lucide-react";
import { useI18n } from "@/locales/client";

interface HistoryTableHeaderProps {
  currentYear?: string | null;
  targetYear?: string | null;
  count: number;
  onRollbackAll: () => void;
  isRollingBack?: boolean;
}

export function HistoryTableHeader({
  currentYear,
  targetYear,
  count,
  onRollbackAll,
  isRollingBack,
}: HistoryTableHeaderProps) {
  const t = useI18n();

  return (
    <div className="flex items-center justify-between p-2 bg-muted/20 border-x border-t">
      <div className="flex items-center gap-3 text-[10px] text-[#606060]">
        <span className="flex items-center gap-1 font-medium">
          <School size={12} /> {currentYear || "-"}
        </span>
        <ArrowRight size={10} />
        <span className="font-bold text-foreground">{targetYear || "-"}</span>
        <span className="text-muted-foreground ml-2">
          ({count} {t("roles.student")})
        </span>
      </div>
      <Button
        variant="outline"
        size="sm"
        onClick={onRollbackAll}
        disabled={isRollingBack}
        className="h-7 rounded-none text-[10px] border-destructive/20 text-destructive hover:bg-destructive hover:text-destructive-foreground transition-all gap-1.5"
      >
        <RotateCcw size={12} />
        تراجع جماعي
      </Button>
    </div>
  );
}
