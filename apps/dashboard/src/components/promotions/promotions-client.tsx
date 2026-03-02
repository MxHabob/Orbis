"use client";

import { useI18n } from "@/locales/client";
import { useTRPC } from "@/trpc/client";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { usePromotionsParams } from "@/hooks/use-promotions-params";
import { useToast } from "@midday/ui/use-toast";
import { PromotionsHeader } from "./promotions-header";
import { PromotionsPreviewTable } from "./promotions-preview-table";

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

  const handleExecute = () => {
    const data = previewQuery.data as any;
    if (!promotionClassId || !sourceYearId || !targetYearId || !data?.students) return;

    if (confirm(t("promotions.confirm_execute", { count: data.students.length }))) {
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
    }
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
    </div>
  );
}
