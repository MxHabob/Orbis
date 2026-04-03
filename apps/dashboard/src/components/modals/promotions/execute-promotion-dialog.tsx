"use client";

import { useI18n } from "@/locales/client";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from "@midday/ui/alert-dialog";
import { AlertTriangle, TrendingUp, Loader2 } from "lucide-react";

interface ExecutePromotionDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  studentCount: number;
  onConfirm: () => void;
  isLoading?: boolean;
}

export function ExecutePromotionDialog({
  open,
  onOpenChange,
  studentCount,
  onConfirm,
  isLoading,
}: ExecutePromotionDialogProps) {
  const t = useI18n();

  return (
    <AlertDialog open={open} onOpenChange={onOpenChange}>
      <AlertDialogContent className="rounded-none border-border shadow-none max-w-md">
        <AlertDialogHeader>
          <AlertDialogTitle className="flex items-center gap-2 text-sm font-bold">
            {t("promotions.execute")}
          </AlertDialogTitle>
          <AlertDialogDescription asChild>
            <div className="space-y-3">
              <p className="text-xs text-[#606060]">
                {t("promotions.confirm_execute", { count: studentCount })}
              </p>
              
              <div className="flex flex-col items-center justify-center py-10 px-6 border border-dashed rounded-xl bg-muted/2 text-muted-foreground animate-in fade-in duration-500">
                <div className="flex items-start gap-2">
                  <div className="text-[11px]">
                    <p className="font-medium text-red-700 dark:text-red-300 mb-1">
                      {t("common.confirm_action")}
                    </p>
                    <p className="text-red-600/80 dark:text-red-400/60">
                      {t("promotions.irreversible_action")}
                    </p>
                  </div>
                </div>
              </div>
            </div>
          </AlertDialogDescription>
        </AlertDialogHeader>

        <AlertDialogFooter className="gap-2">
          <AlertDialogCancel className="rounded-none h-8 text-xs" disabled={isLoading}>
            {t("common.cancel")}
          </AlertDialogCancel>
          <AlertDialogAction
            className="rounded-none h-8 text-xs bg-primary text-primary-foreground hover:bg-primary/90"
            onClick={(e) => {
              e.preventDefault();
              onConfirm();
            }}
            disabled={isLoading}
          >
            {isLoading ? (
              <div className="flex items-center gap-2">
                <Loader2 className="size-3 animate-spin" />
                {t("common.loading")}
              </div>
            ) : (
              t("common.confirm")
            )}
          </AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  );
}
