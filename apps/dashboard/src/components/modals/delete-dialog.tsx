"use client";

import { useI18n } from "@/locales/client";
import { Alert, AlertDescription, AlertTitle } from "@orbis/ui/alert";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from "@orbis/ui/alert-dialog";
import { Button } from "@orbis/ui/button";
import { AlertCircle, AlertTriangle, Loader2, Trash2 } from "lucide-react";
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
          <AlertDialogTitle className="flex items-center gap-2 text-lg font-bold">
            {title || t("common.confirm_action")}
          </AlertDialogTitle>
          <AlertDialogDescription className="text-sm text-muted-foreground pt-2">
            {description}
          </AlertDialogDescription>
        </AlertDialogHeader>

          <Alert className="bg-amber-500/20 dark:bg-amber-950/20 border-amber-200 dark:border-amber-800 border-dashed">
            <AlertCircle className="h-4 w-4 text-amber-500" />
            <div className="flex items-center justify-between w-full">
              <div>
                <AlertTitle className="text-amber-800 dark:text-amber-400">
                   {t("common.destructive_action")}
                </AlertTitle>
                <AlertDescription className="text-amber-700 dark:text-amber-500">
                  {t("common.irreversible_action")}
                </AlertDescription>
              </div>
            </div>
          </Alert>

        <AlertDialogFooter className="gap-2">
          <AlertDialogCancel asChild>
            <Button
              variant="outline"
              className="rounded-none h-8"
              disabled={isPending}
            >
              {cancelLabel || t("common.cancel")}
            </Button>
          </AlertDialogCancel>
          <AlertDialogAction asChild>
            <Button
              variant="destructive"
              className="rounded-none h-8"
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
