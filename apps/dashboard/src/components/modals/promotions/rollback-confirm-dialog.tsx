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

interface RollbackConfirmDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  studentCount: number;
  onConfirm: () => void;
}

export function RollbackConfirmDialog({
  open,
  onOpenChange,
  studentCount,
  onConfirm,
}: RollbackConfirmDialogProps) {
  const t = useI18n();

  return (
    <AlertDialog open={open} onOpenChange={onOpenChange}>
      <AlertDialogContent className="rounded-none border-border shadow-none">
        <AlertDialogHeader>
          <AlertDialogTitle className="text-sm font-bold uppercase tracking-tight">
            {t("promotions.rollback")}
          </AlertDialogTitle>
          <AlertDialogDescription className="text-xs text-[#606060]">
            {studentCount > 1
              ? t("promotions.confirm_rollback_bulk", { count: studentCount })
              : t("promotions.confirm_rollback")}
          </AlertDialogDescription>
        </AlertDialogHeader>
        <AlertDialogFooter className="gap-2">
          <AlertDialogCancel className="rounded-none h-8 text-xs">
            {t("common.cancel")}
          </AlertDialogCancel>
          <AlertDialogAction
            className="rounded-none h-8 text-xs bg-destructive text-destructive-foreground hover:bg-destructive/90"
            onClick={onConfirm}
          >
            {t("common.confirm")}
          </AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  );
}
