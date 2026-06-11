"use client";

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
import { AlertCircle, Loader2 } from "lucide-react";
import { useI18n } from "@/locales/client";

type Props = {
  isOpen: boolean;
  onOpenChange: (open: boolean) => void;
  studentCount: number;
  className: string;
  onConfirm: () => void;
  isImporting: boolean;
};

export function StudentImportConfirm({
  isOpen,
  onOpenChange,
  studentCount,
  className,
  onConfirm,
  isImporting,
}: Props) {
  const t = useI18n();

  return (
    <AlertDialog open={isOpen} onOpenChange={onOpenChange}>
      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogTitle className="flex items-center gap-2 text-sm p-0">
            <AlertCircle className="size-4 text-[#878787]" />
            {t("students.confirm_import_title")}
          </AlertDialogTitle>
          <AlertDialogDescription asChild>
            <div className="space-y-3">
              <p>
                {t("students.confirm_import_description", {
                  count: studentCount,
                  className: className,
                })}
              </p>
              <div className="my-6 px-3 py-3 bg-amber-50 border border-amber-200 dark:bg-amber-900/10 dark:border-amber-800/30">
                <div className="flex items-start gap-2">
                  <Loader2 className="size-4 text-amber-600 dark:text-amber-400 flex-shrink-0 mt-0.5 animate-pulse" />
                  <div className="text-sm">
                    <p className="font-medium text-amber-700 dark:text-amber-300 mb-1">
                      {studentCount} {t("grades.student_name")}
                    </p>
                  </div>
                </div>
              </div>
              <p className="text-sm text-muted-foreground">
                Click confirm to start the import process.
              </p>
            </div>
          </AlertDialogDescription>
        </AlertDialogHeader>

        <AlertDialogFooter>
          <AlertDialogCancel disabled={isImporting}>
            {t("common.cancel")}
          </AlertDialogCancel>
          <AlertDialogAction
            onClick={(e) => {
              e.preventDefault();
              onConfirm();
            }}
            disabled={isImporting}
            className="bg-primary text-primary-foreground hover:bg-primary/90"
          >
            {isImporting ? (
              <div className="flex items-center gap-2">
                <Loader2 className="size-4 animate-spin" />
                {t("students.importing")}
              </div>
            ) : (
              t("students.confirm_import_button")
            )}
          </AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  );
}
