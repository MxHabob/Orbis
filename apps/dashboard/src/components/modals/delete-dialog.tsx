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
import { Button } from "@midday/ui/button";
import { AlertTriangle, Loader2, Trash2 } from "lucide-react";
import React from "react";

interface DeleteDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onConfirm: () => void;
  isPending?: boolean;
  title?: string;
  description?: string;
  confirmLabel?: string;
  cancelLabel?: string;
}

export function DeleteDialog({
  open,
  onOpenChange,
  onConfirm,
  isPending,
  title,
  description,
  confirmLabel,
  cancelLabel,
}: DeleteDialogProps) {
  const t = useI18n();

  return (
    <AlertDialog open={open} onOpenChange={onOpenChange}>
      <AlertDialogContent className="rounded-none border-border shadow-none max-w-md">
        <AlertDialogHeader>
          <AlertDialogTitle className="flex items-center gap-2 text-sm font-bold uppercase tracking-tight">
            <Trash2 className="size-4" />
            {title || t("common.confirm_action")}
          </AlertDialogTitle>
          <AlertDialogDescription className="text-sm text-muted-foreground pt-2">
            {description}
          </AlertDialogDescription>
        </AlertDialogHeader>

        <div className="bg-amber-50/50 dark:bg-amber-950/20 border border-amber-200 dark:border-amber-900/50 p-4 my-2 flex gap-3 items-start animate-in fade-in slide-in-from-top-2 duration-500">
          <AlertTriangle className="size-5 text-amber-600 dark:text-amber-500 mt-0.5 shrink-0" />
          <div className="space-y-1">
            <h4 className="text-sm font-semibold text-amber-900 dark:text-amber-400">
              {t("common.destructive_action")}
            </h4>
            <p className="text-xs text-amber-800/80 dark:text-amber-400/60 leading-relaxed">
              {t("common.irreversible_action")}
            </p>
          </div>
        </div>

        <AlertDialogFooter className="flex-row gap-2 mt-2">
          <AlertDialogCancel asChild>
            <Button
              variant="outline"
              className="flex-1 rounded-none border-border hover:bg-muted font-medium transition-all"
              disabled={isPending}
            >
              {cancelLabel || t("common.cancel")}
            </Button>
          </AlertDialogCancel>
          <AlertDialogAction asChild>
            <Button
              variant="destructive"
              className="flex-1 rounded-none font-semibold transition-all hover:opacity-90 active:scale-[0.98]"
              disabled={isPending}
              onClick={(e) => {
                e.preventDefault();
                onConfirm();
              }}
            >
              {isPending ? (
                <>
                  <Loader2 className="size-4 mr-2 animate-spin" />
                  {t("common.deleting")}
                </>
              ) : (
                confirmLabel || t("common.delete")
              )}
            </Button>
          </AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  );
}
