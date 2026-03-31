"use client";

import { useZodForm } from "@/hooks/use-zod-form";
import { useTRPC } from "@/trpc/client";
import { Button } from "@midday/ui/button";
import { Form, FormControl, FormField, FormItem, FormLabel } from "@midday/ui/form";
import { Input } from "@midday/ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@midday/ui/select";
import { SubmitButton } from "@midday/ui/submit-button";
import { useToast } from "@midday/ui/use-toast";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useFieldArray } from "react-hook-form";
import { z } from "zod/v3";
import { X, Sparkles, Loader2 } from "lucide-react";
import { useI18n } from "@/locales/client";
import { useState, useMemo } from "react";
import { useAction } from "next-safe-action/hooks";
import { initializeAssessmentsAction } from "@/actions/assessments/initialize-assessments";

const formSchema = z.object({
  assessments: z.array(
    z.object({
      id: z.number().optional(),
      title: z.string().min(1, "Title is required"),
      type: z.enum(['exam', 'quiz', 'assignment', 'participation', 'oral', 'attendance', 'behavior', 'homework']),
      maxScore: z.string().min(1, "Score is required"),
      weight: z.string().default("1"),
    }),
  ),
});

type AssessmentsFormProps = {
  classId: number;
  subjectId: number;
  termId: number;
  month: number;
  onSuccess?: () => void;
};

export function AssessmentsForm({ classId, subjectId, termId, month, onSuccess }: AssessmentsFormProps) {
  const trpc = useTRPC();
  const queryClient = useQueryClient();
  const { toast } = useToast();
  const t = useI18n();

  // Load existing data
  const { data: initialData, isLoading, refetch } = useQuery({
    ...trpc.assessments.getByClass.queryOptions({
      classId,
      subjectId,
      termId,
      month,
    }),
    staleTime: 0,
    gcTime: 0,
    refetchOnMount: true,
    refetchOnWindowFocus: false,
    refetchOnReconnect: false,
  });

  const formValues = useMemo(() => {
    if (!initialData) return undefined;
    
    return {
      assessments: initialData.map((item) => ({
        id: item.id,
        title: item.title,
        type: item.type as any,
        maxScore: item.maxScore ? String(item.maxScore) : "10",
        weight: item.weight ? String(item.weight) : "1",
      }))
    };
  }, [initialData]);

  const form = useZodForm(formSchema, {
    defaultValues: {
      assessments: [],
    },
    values: formValues,
  });

  const { fields, append, remove } = useFieldArray({
    name: "assessments",
    control: form.control,
  });

  const [isSaving, setIsSaving] = useState(false);

  const createMutation = useMutation(trpc.assessments.create.mutationOptions());
  const updateMutation = useMutation(trpc.assessments.update.mutationOptions());
  const deleteMutation = useMutation(trpc.assessments.delete.mutationOptions());

  const { execute: executeInitialize, status: initializeStatus } = useAction(initializeAssessmentsAction, {
    onSuccess: () => {
      toast({ title: "Success!", description: "Standard items created.", variant: "success" });
      refetch();
      queryClient.invalidateQueries({ queryKey: trpc.grades.getGridData.queryKey() });
      queryClient.invalidateQueries({ queryKey: trpc.assessments.getByClass.queryKey() });
    },
  });

  const onSubmit = form.handleSubmit(async (data) => {
    setIsSaving(true);
    try {
      const currentIds = data.assessments.map(a => a.id).filter(Boolean) as number[];
      const toDelete = (initialData || []).filter(a => !currentIds.includes(a.id));

      const promises: Promise<any>[] = [];

      toDelete.forEach(a => {
        promises.push(deleteMutation.mutateAsync({ id: a.id }));
      });

      data.assessments.forEach(a => {
        if (a.id) {
          promises.push(
            updateMutation.mutateAsync({
              id: a.id,
              title: a.title,
              maxScore: a.maxScore,
              type: a.type,
              weight: a.weight,
            })
          );
        } else {
          promises.push(
            createMutation.mutateAsync({
              classId,
              subjectId,
              termId,
              month,
              title: a.title,
              maxScore: a.maxScore,
              type: a.type,
              weight: a.weight,
            } as any)
          );
        }
      });

      await Promise.all(promises);

      toast({
        title: "Success",
        description: "Assessments saved successfully.",
        variant: "success",
      });

      queryClient.invalidateQueries({ queryKey: trpc.grades.getGridData.queryKey() });
      queryClient.invalidateQueries({ queryKey: trpc.assessments.getByClass.queryKey() });
      onSuccess?.();
    } catch (err) {
      toast({
        title: "Error",
        description: "Failed to save assessments.",
        variant: "destructive",
      });
    } finally {
      setIsSaving(false);
    }
  });

  if (isLoading) {
    return (
      <div className="space-y-5 animate-pulse">
        {[1, 2, 3].map((i) => (
          <div key={i} className="h-20 bg-muted/20 rounded-lg w-full" />
        ))}
      </div>
    );
  }

  return (
    <Form {...form}>
      <form onSubmit={onSubmit}>
        <div className="space-y-5">
          {fields.map((field, index) => (
            <AssessmentRow
              key={field.id}
              index={index}
              control={form.control}
              t={t}
              onRemove={fields.length > 1 ? () => remove(index) : undefined}
            />
          ))}
        </div>

        <div className="flex items-center gap-4 mt-4">
          <Button
            variant="outline"
            type="button"
            className="border-none bg-[#F2F1EF] text-[11px] dark:bg-[#1D1D1D]"
            onClick={() =>
              append({
                title: "",
                maxScore: "10",
                type: "quiz",
                weight: "1",
              })
            }
          >
            Add more
          </Button>
          
          <Button 
             variant="ghost"
             type="button"
             className="text-[11px] text-muted-foreground p-0 h-auto font-normal hover:bg-transparent"
             onClick={() => executeInitialize({ classId, subjectId, termId, month })}
             disabled={initializeStatus === "executing"}
          >
             {initializeStatus === "executing" ? <Loader2 className="h-3 w-3 mr-2 animate-spin" /> : <Sparkles className="h-3 w-3 mr-2" />}
             {t("grades.create_default_items")}
          </Button>
        </div>

        <div className="border-t-[1px] pt-4 mt-8 items-center justify-between">
          <div>
            {Object.values(form.formState.errors).length > 0 && (
              <span className="text-sm text-destructive">
                Please complete all required fields above.
              </span>
            )}
          </div>

          <div className="flex items-center justify-between">
            <Button
              variant="ghost"
              type="button"
              className="p-0 hover:bg-transparent font-normal"
              onClick={onSuccess}
            >
              Cancel
            </Button>

            <SubmitButton
              type="submit"
              isSubmitting={isSaving}
              disabled={isSaving}
            >
              {t("grades.save_all")}
            </SubmitButton>
          </div>
        </div>

      </form>
    </Form>
  );
}

function AssessmentRow({
  index,
  control,
  t,
  onRemove,
}: {
  index: number;
  control: any;
  t: any;
  onRemove?: () => void;
}) {
  return (
    <div className="relative rounded-lg p-4 transition-all duration-200">
      {onRemove && (
        <button
          type="button"
          onClick={onRemove}
          className="absolute -right-2 -top-2 rounded-full bg-background border border-border p-1 text-muted-foreground hover:text-destructive hover:border-destructive transition-colors z-10"
        >
          <X className="size-3" />
        </button>
      )}

      <div className="grid grid-cols-12 gap-3">
        <FormField
          control={control}
          name={`assessments.${index}.title`}
          render={({ field }) => (
            <FormItem className="col-span-4">
              <FormLabel className="text-xs font-medium text-muted-foreground">
                {t("grades.title")}
              </FormLabel>
              <FormControl>
                <Input
                  placeholder="e.g. Monthly Quiz"
                  autoComplete="off"
                  {...field}
                />
              </FormControl>
            </FormItem>
          )}
        />

        <FormField
          control={control}
          name={`assessments.${index}.type`}
          render={({ field }) => (
            <FormItem className="col-span-5">
              <FormLabel className="text-xs font-medium text-muted-foreground">
                {t("grades.type")}
              </FormLabel>
              <Select
                onValueChange={field.onChange}
                defaultValue={field.value}
              >
                <FormControl>
                  <SelectTrigger>
                    <SelectValue placeholder="Type" />
                  </SelectTrigger>
                </FormControl>
                <SelectContent>
                  {['participation', 'attendance', 'homework', 'oral', 'behavior', 'quiz', 'assignment', 'exam'].map(type => (
                    <SelectItem key={type} value={type}>{t(`grades.${type}`)}</SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </FormItem>
          )}
        />

        <FormField
          control={control}
          name={`assessments.${index}.maxScore`}
          render={({ field }) => (
            <FormItem className="col-span-3">
              <FormLabel className="text-xs font-medium text-muted-foreground">
                {t("grades.max_score")}
              </FormLabel>
              <FormControl>
                <Input
                  type="number"
                  placeholder="10"
                  max="100"
                  min="1"
                  {...field}
                />
              </FormControl>
            </FormItem>
          )}
        />
      </div>
    </div>
  );
}
