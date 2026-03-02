"use client";

import { useI18n } from "@/locales/client";
import { useTRPC } from "@/trpc/client";
import { useMutation, useQueryClient, useQuery } from "@tanstack/react-query";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { createAcademicTermSchema } from "@api/schemas/academic";
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from "@midday/ui/form";
import { Input } from "@midday/ui/input";
import { Button } from "@midday/ui/button";
import { useToast } from "@midday/ui/use-toast";
import { useEffect } from "react";

interface AcademicTermFormProps {
  initialData?: any;
  yearId?: number;
  termId?: number;
  onSuccess?: () => void;
}

export function AcademicTermForm({ initialData, yearId, termId, onSuccess }: AcademicTermFormProps) {
  const t = useI18n();
  const trpc = useTRPC();
  const { toast } = useToast();
  const queryClient = useQueryClient();

  // If termId is provided but no initialData, we might want to fetch (though usually years getYears includes terms)
  // For simplicity, we'll assume the form is used either for create (yearId) or edit (initialData/termId)

  const form = useForm({
    resolver: zodResolver(createAcademicTermSchema),
    defaultValues: initialData || {
      name: "",
      academicYearId: yearId,
      startDate: new Date().toISOString(),
      endDate: new Date().toISOString(),
    },
  });

  // Fetch all years to find the specific term if it's an edit by ID
  const { data: years } = useQuery(trpc.academic.getYears.queryOptions({ enabled: !!termId && !initialData }));

  useEffect(() => {
    if (termId && years) {
      for (const y of years) {
        const term = y.terms?.find((t: any) => t.id === termId);
        if (term) {
          form.reset({
            id: term.id,
            name: term.name,
            academicYearId: term.academicYearId,
            startDate: term.startDate,
            endDate: term.endDate,
          } as any);
          break;
        }
      }
    }
  }, [termId, years, form]);

  const isEdit = !!(initialData?.id || termId);

  const mutation = useMutation(
    isEdit 
      ? trpc.academic.updateTerm.mutationOptions({
          onSuccess: () => {
            toast({ title: t("process_success") });
            queryClient.invalidateQueries({ queryKey: trpc.academic.getYears.queryKey() });
            onSuccess?.();
          }
        })
      : trpc.academic.createTerm.mutationOptions({
          onSuccess: () => {
            toast({ title: t("process_success") });
            queryClient.invalidateQueries({ queryKey: trpc.academic.getYears.queryKey() });
            onSuccess?.();
          }
        })
  );

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit((data: any) => mutation.mutate(data))} className="space-y-4">
        <FormField
          control={form.control}
          name="name"
          render={({ field }) => (
            <FormItem>
              <FormLabel>{t("reports.half_yearly")}</FormLabel>
              <FormControl>
                <Input placeholder={t("reports.half_yearly") + " 1"} {...field} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />

        <div className="grid grid-cols-2 gap-4">
          <FormField
            control={form.control}
            name="startDate"
            render={({ field }) => (
              <FormItem>
                <FormLabel>{t("common.start_date")}</FormLabel>
                <FormControl>
                  <Input type="date" {...field} value={field.value?.split('T')[0]} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name="endDate"
            render={({ field }) => (
              <FormItem>
                <FormLabel>{t("common.end_date")}</FormLabel>
                <FormControl>
                  <Input type="date" {...field} value={field.value?.split('T')[0]} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
        </div>

        <Button type="submit" className="w-full" disabled={mutation.isPending}>
          {mutation.isPending ? t("common.loading") : t("common.save")}
        </Button>
      </form>
    </Form>
  );
}
