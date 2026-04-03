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
          <AlertDialogTitle className="flex items-center gap-2 text-sm font-bold uppercase tracking-tight">
            <TrendingUp className="size-4" />
            {t("promotions.execute")}
          </AlertDialogTitle>
          <AlertDialogDescription asChild>
            <div className="space-y-3">
              <p className="text-xs text-[#606060]">
                {t("promotions.confirm_execute", { count: studentCount })}
              </p>
              
              <div className="my-6 px-3 py-3 bg-amber-50 border border-amber-200 dark:bg-amber-900/10 dark:border-amber-800/30">
                <div className="flex items-start gap-2">
                  <AlertTriangle className="size-4 text-amber-600 dark:text-amber-400 flex-shrink-0 mt-0.5" />
                  <div className="text-[11px]">
                    <p className="font-medium text-amber-700 dark:text-amber-300 mb-1">
                      {t("common.confirm_action")}
                    </p>
                    <p className="text-amber-600/80 dark:text-amber-400/60">
                       سيتم نقل هؤلاء الطلاب إلى العام الدراسي الجديد بناءً على القرارات المقترحة. تأكد من صحة البيانات قبل التأكيد.
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
