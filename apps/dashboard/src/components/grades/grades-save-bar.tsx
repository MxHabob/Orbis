"use client";

import { Portal } from "@/components/portal";
import { useI18n } from "@/locales/client";
import { Button } from "@orbis/ui/button";
import { AnimatePresence, motion } from "framer-motion";
import { Loader2, Save } from "lucide-react";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from "@orbis/ui/alert-dialog";
import { cn } from "@orbis/ui/cn";

interface GradesSaveBarProps {
  count: number;
  onSave: () => void;
  onCancel: () => void;
  isPending: boolean;
  hasErrors?: boolean;
}

export function GradesSaveBar({
  count,
  onSave,
  onCancel,
  isPending,
  hasErrors,
}: GradesSaveBarProps) {
  const isOpen = count > 0;
  const t = useI18n();

  return (
    <Portal>
      <AnimatePresence>
        {isOpen && (
          <motion.div
            className="h-12 fixed bottom-6 left-0 right-0 pointer-events-none flex justify-center z-50"
            initial={{ y: 100 }}
            animate={{ y: 0 }}
            exit={{ y: 100 }}
            transition={{ duration: 0.2, ease: "easeOut" }}
          >
            <div className="relative pointer-events-auto min-w-[500px] h-12">
              {/* Blur layer */}
              <motion.div
                className="absolute inset-0 backdrop-filter backdrop-blur-lg bg-[rgba(247,247,247,0.85)] dark:bg-[rgba(19,19,19,0.7)] rounded-none border border-border"
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                exit={{ opacity: 0 }}
                transition={{ duration: 0.15 }}
              />
              <div className="relative h-12 justify-between items-center flex pl-4 pr-2">
                <div className="flex items-center gap-2 px-4">
                  <span 
                    className={cn(
                      "text-sm font-medium transition-colors duration-200",
                      hasErrors ? "text-destructive font-bold" : "text-foreground"
                    )}
                  >
                    {t("grades.unsaved_changes", { count })}
                    {hasErrors && ` ${t("grades.invalid")}`}
                  </span>
                </div>

                <div className="flex items-center space-x-2 gap-2">
                  <AlertDialog>
                    <AlertDialogTrigger asChild>
                      <Button
                        variant="ghost"
                        size="sm"
                        className="text-muted-foreground h-8 hover:bg-transparent"
                        disabled={isPending}
                      >
                        {t("grades.discard")}
                      </Button>
                    </AlertDialogTrigger>
                    <AlertDialogContent>
                      <AlertDialogHeader>
                        <AlertDialogTitle>{t("grades.discard_title")}</AlertDialogTitle>
                        <AlertDialogDescription>
                          {t("grades.discard_description", { count })}
                        </AlertDialogDescription>
                      </AlertDialogHeader>
                      <AlertDialogFooter>
                        <AlertDialogCancel>{t("grades.keep_editing")}</AlertDialogCancel>
                        <AlertDialogAction
                          onClick={onCancel}
                          className="bg-destructive text-destructive-foreground hover:bg-destructive/90"
                        >
                          {t("grades.discard_changes")}
                        </AlertDialogAction>
                      </AlertDialogFooter>
                    </AlertDialogContent>
                  </AlertDialog>

                  <div className="h-4 w-[1px] bg-border mx-1" />

                  <Button
                    size="sm"
                    onClick={onSave}
                    disabled={isPending || hasErrors}
                    className="gap-2 h-8 min-w-[120px]"
                    variant={hasErrors ? "secondary" : "default"}
                  >
                    {isPending ? (
                      <Loader2 className="h-4 w-4 animate-spin" />
                    ) : (
                      <Save size={16} />
                    )}
                    <span>{t("grades.save_all")}</span>
                  </Button>
                </div>
              </div>
            </div>
          </motion.div>
        )}
      </AnimatePresence>
    </Portal>
  );
}
