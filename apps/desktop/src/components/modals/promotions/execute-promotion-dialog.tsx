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
import { AlertCircle, Loader2 } from "lucide-react";
import { useI18n } from "@/locales/client";

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
          <AlertDialogTitle className="flex items-center gap-2 text-lg font-bold">
            {t("promotions.execute")}
          </AlertDialogTitle>
          <AlertDialogDescription asChild>
            <div className="space-y-3">
              <p className="text-xs text-[#606060]">
                {t("promotions.confirm_execute", { count: studentCount })}
              </p>

              <Alert className="bg-amber-500/20 dark:bg-amber-950/20 border-amber-200 dark:border-amber-800 border-dashed">
                <AlertCircle className="h-4 w-4 text-amber-500" />
                <div className="flex items-center justify-between w-full">
                  <div>
                    <AlertTitle className="text-amber-800 dark:text-amber-400">
                      {t("common.confirm_action")}
                    </AlertTitle>
                    <AlertDescription className="text-amber-700 dark:text-amber-500">
                      {t("promotions.irreversible_action")}
                    </AlertDescription>
                  </div>
                </div>
              </Alert>
            </div>
          </AlertDialogDescription>
        </AlertDialogHeader>

        <AlertDialogFooter className="gap-2">
          <AlertDialogCancel
            className="rounded-none h-8 text-xs"
            disabled={isLoading}
          >
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
