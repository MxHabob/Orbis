"use client";

import { useI18n } from "@/locales/client";
import { Button } from "@midday/ui/button";
import { RotateCcw, PenSquare } from "lucide-react";
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "@midday/ui/tooltip";
import type { PromotionHistory } from "./columns";

interface ActionsMenuProps {
  record: PromotionHistory;
  onRollback: (studentIds: string[]) => void;
  onOverride: (history: PromotionHistory) => void;
  isRollingBack?: boolean;
}

export function ActionsMenu({
  record,
  onRollback,
  onOverride,
  isRollingBack,
}: ActionsMenuProps) {
  const t = useI18n();

  return (
    <div className="flex items-center gap-1">
      <TooltipProvider>
        <Tooltip>
          <TooltipTrigger asChild>
            <Button
              variant="ghost"
              size="icon"
              onClick={(e) => {
                e.stopPropagation();
                onOverride(record);
              }}
              className="size-7 text-[#606060] hover:text-primary rounded-none hover:bg-primary/5"
            >
              <PenSquare size={12} />
            </Button>
          </TooltipTrigger>
          <TooltipContent side="top" className="rounded-none text-[10px] bg-background border-border text-foreground">
            {t("promotions.manual_decision")}
          </TooltipContent>
        </Tooltip>
      </TooltipProvider>

      <TooltipProvider>
        <Tooltip>
          <TooltipTrigger asChild>
            <Button
              variant="ghost"
              size="icon"
              onClick={(e) => {
                e.stopPropagation();
                onRollback([record.studentId]);
              }}
              disabled={isRollingBack}
              className="size-7 text-[#606060] hover:text-destructive rounded-none hover:bg-destructive/5"
            >
              <RotateCcw size={12} />
            </Button>
          </TooltipTrigger>
          <TooltipContent side="top" className="rounded-none text-[10px] bg-background border-border text-foreground">
            {t("promotions.rollback")}
          </TooltipContent>
        </Tooltip>
      </TooltipProvider>
    </div>
  );
}
