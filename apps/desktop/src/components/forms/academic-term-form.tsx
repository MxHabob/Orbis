"use client";

import { createAcademicTermSchema } from "@api/schemas/academic";
import { zodResolver } from "@hookform/resolvers/zod";
import { Button } from "@orbis/ui/button";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@orbis/ui/form";
import { Input } from "@orbis/ui/input";
import { Switch } from "@orbis/ui/switch";
import { useToast } from "@orbis/ui/use-toast";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useEffect } from "react";
import { useForm } from "react-hook-form";
import { useI18n } from "@/locales/client";
import { useTRPC } from "@/trpc/client";

interface AcademicTermFormProps {
  initialData?: any;
  yearId?: number;
  termId?: number;
  onSuccess?: () => void;
}

export function AcademicTermForm({
  initialData,
  yearId,
  termId,
  onSuccess,
}: AcademicTermFormProps) {
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
      isCurrent: false,
    },
  });

  // Fetch all years to find the specific term if it's an edit by ID
  const { data: years } = useQuery({
    ...trpc.academic.getYears.queryOptions(),
    enabled: !!termId && !initialData,
  });

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

  const createMutation = useMutation(
    trpc.academic.createTerm.mutationOptions({
      onSuccess: () => {
        toast({ title: t("promotions.process_success") });
        queryClient.invalidateQueries({
          queryKey: trpc.academic.getYears.queryKey(),
        });
        onSuccess?.();
      },
      onError: (error) => {
        toast({
          title: t("common.error"),
          description: error.message,
          variant: "destructive",
        });
      },
    }),
  );

  const updateMutation = useMutation(
    trpc.academic.updateTerm.mutationOptions({
      onSuccess: () => {
        toast({ title: t("promotions.process_success") });
        queryClient.invalidateQueries({
          queryKey: trpc.academic.getYears.queryKey(),
        });
        onSuccess?.();
      },
      onError: (error) => {
        toast({
          title: t("common.error"),
          description: error.message,
          variant: "destructive",
        });
      },
    }),
  );

  const isPending = createMutation.isPending || updateMutation.isPending;

  const onSubmit = (data: any) => {
    const id = initialData?.id || termId;
    if (id) {
      updateMutation.mutate({
        ...data,
        id,
      });
    } else {
      createMutation.mutate(data);
    }
  };

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
        <FormField
          control={form.control}
          name="name"
          render={({ field }) => (
            <FormItem>
              <FormLabel>{t("reports.half_yearly")}</FormLabel>
              <FormControl>
                <Input
                  placeholder={t("reports.half_yearly") + " 1"}
                  {...field}
                />
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
                  <Input
                    type="date"
                    {...field}
                    value={field.value?.split("T")[0]}
                  />
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
                  <Input
                    type="date"
                    {...field}
                    value={field.value?.split("T")[0]}
                  />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
        </div>

        <Button type="submit" className="w-full" disabled={isPending}>
          {isPending ? t("common.loading") : t("common.save")}
        </Button>
      </form>
    </Form>
  );
}
