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
          <AlertDialogTitle className="flex items-center gap-2 text-sm font-bold">
            {title || t("common.confirm_action")}
          </AlertDialogTitle>
          <AlertDialogDescription className="text-sm text-muted-foreground pt-2">
            {description}
          </AlertDialogDescription>
        </AlertDialogHeader>

        <div className="flex flex-col items-center justify-center py-10 px-6 border border-dashed rounded-xl bg-muted/2 text-muted-foreground animate-in fade-in duration-500">
          <div className="space-y-1">
            <h4 className="text-sm font-semibold text-red-900 dark:text-red-400">
              {t("common.destructive_action")}
            </h4>
            <p className="text-xs text-red-800/80 dark:text-red-400/60 leading-relaxed">
              {t("common.irreversible_action")}
            </p>
          </div>
        </div>

        <AlertDialogFooter className="gap-2">
          <AlertDialogCancel asChild>
            <Button
              variant="outline"
              className="rounded-none h-8 text-xs bg-primary text-primary-foreground hover:bg-primary/90"
              disabled={isPending}
            >
              {cancelLabel || t("common.cancel")}
            </Button>
          </AlertDialogCancel>
          <AlertDialogAction asChild>
            <Button
              variant="destructive"
              className="rounded-none h-8 text-xs bg-primary text-primary-foreground hover:bg-primary/90"
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
