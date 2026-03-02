"use client";

import { useI18n } from "@/locales/client";
import { useTRPC } from "@/trpc/client";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { usePromotionsParams } from "@/hooks/use-promotions-params";
import { useToast } from "@midday/ui/use-toast";
import { PromotionsHeader } from "./promotions-header";
import { PromotionsPreviewTable } from "./promotions-preview-table";
import { useState } from "react";
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
import { Loader2, TrendingUp } from "lucide-react";

interface PromotionsClientProps {
  initialData: {
    classes: any[];
    years: any[];
  };
  currentYearId?: number;
}

export function PromotionsClient({ initialData, currentYearId }: PromotionsClientProps) {
  const t = useI18n();
  const trpc = useTRPC();
  const { toast } = useToast();
  const queryClient = useQueryClient();
  const { promotionClassId, sourceYearId, targetYearId } = usePromotionsParams();
  const [isConfirmOpen, setIsConfirmOpen] = useState(false);

  const previewQuery = useQuery(
    trpc.promotions.preview.queryOptions(
      {
        academicYearId: sourceYearId!,
        classId: promotionClassId!,
      },
      {
        enabled: !!sourceYearId && !!promotionClassId,
      }
    )
  );

  const executeMutation = useMutation(
    trpc.promotions.execute.mutationOptions({
      onSuccess: () => {
        toast({
          title: t("promotions.process_success"),
          variant: "default",
        });
        previewQuery.refetch();
        // Also invalidate other queries that might be affected
        queryClient.invalidateQueries({ queryKey: trpc.students.getAll.queryKey() });
        queryClient.invalidateQueries({ queryKey: trpc.classes.getAll.queryKey() });
      },
      onError: (err) => {
        toast({
          title: t("promotions.process_error"),
          description: err.message,
          variant: "destructive",
        });
      },
    })
  );

  const onConfirmExecute = () => {
    const data = previewQuery.data as any;
    if (!promotionClassId || !sourceYearId || !targetYearId || !data?.students) return;

    executeMutation.mutate({
      fromAcademicYearId: sourceYearId,
      toAcademicYearId: targetYearId,
      classId: promotionClassId,
      decisions: data.students
        .filter((p: any) => p.suggestedStatus === "promoted" || p.suggestedStatus === "graduated")
        .map((p: any) => ({
          studentId: p.studentId,
          status: p.suggestedStatus as any,
          toClassId: p.targetClassId || undefined,
          notes: "Bulk Promotion",
        }))
    });
    
    setIsConfirmOpen(false);
  };

  const handleExecute = () => {
    const data = previewQuery.data as any;
    if (!promotionClassId || !sourceYearId || !targetYearId || !data?.students) return;

    setIsConfirmOpen(true);
  };

  const previewData = previewQuery.data as any;

  return (
    <div className="flex flex-col gap-6">
      <div className="flex flex-col gap-4">
        <PromotionsHeader 
          initialData={initialData}
          currentYearId={currentYearId}
          onExecute={handleExecute}
          canExecute={!!previewData?.students?.length}
          isExecuting={executeMutation.isPending}
          onRefresh={() => previewQuery.refetch()}
          isRefreshing={previewQuery.isFetching}
        />
        
        <PromotionsPreviewTable 
          data={previewData?.students?.map((s: any) => ({
            ...s,
            targetClassName: s.passed && previewData.nextClass ? previewData.nextClass.name : null
          })) || []} 
          isLoading={previewQuery.isLoading} 
        />
      </div>

      <AlertDialog open={isConfirmOpen} onOpenChange={setIsConfirmOpen}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle className="flex items-center gap-2 text-sm">
              <TrendingUp className="size-4" />
              {t("promotions.execute")}
            </AlertDialogTitle>
            <AlertDialogDescription asChild>
              <div className="space-y-3">
                <p>{t("promotions.confirm_execute", { count: previewData?.students?.length || 0 })}</p>
                <p className="text-sm text-muted-foreground mt-2">
                  {t("vault.delete_undo_warning")}
                </p>
              </div>
            </AlertDialogDescription>
          </AlertDialogHeader>

          <AlertDialogFooter>
            <AlertDialogCancel disabled={executeMutation.isPending}>
              {t("common.cancel")}
            </AlertDialogCancel>
            <AlertDialogAction 
              onClick={onConfirmExecute} 
              disabled={executeMutation.isPending}
            >
              {executeMutation.isPending ? (
                <div className="flex items-center gap-2">
                  <Loader2 className="size-4 animate-spin" />
                  {t("common.loading")}
                </div>
              ) : (
                <div className="flex items-center gap-2">
                  <TrendingUp className="size-4" />
                  {t("promotions.execute")}
                </div>
              )}
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </div>
  );
}
