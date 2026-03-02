"use client";

import { useI18n } from "@/locales/client";
import { useTRPC } from "@/trpc/client";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { createAcademicYearSchema } from "@api/schemas/academic";
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from "@midday/ui/form";
import { Input } from "@midday/ui/input";
import { Button } from "@midday/ui/button";
import { useToast } from "@midday/ui/use-toast";
import { Switch } from "@midday/ui/switch";

interface AcademicYearFormProps {
  initialData?: any;
  onSuccess?: () => void;
}

export function AcademicYearForm({ initialData, onSuccess }: AcademicYearFormProps) {
  const t = useI18n();
  const trpc = useTRPC();
  const { toast } = useToast();
  const queryClient = useQueryClient();

  const form = useForm({
    resolver: zodResolver(createAcademicYearSchema),
    defaultValues: initialData || {
      yearName: "",
      startDate: new Date().toISOString(),
      endDate: new Date(new Date().setFullYear(new Date().getFullYear() + 1)).toISOString(),
      isCurrent: false,
    },
  });

  const mutation = useMutation({
    mutationFn: (data: any) => {
      if (initialData?.id) {
        return trpc.academic.updateYear.mutate({
          ...data,
          id: initialData.id,
        });
      }
      return trpc.academic.createYear.mutate(data);
    },
    onSuccess: () => {
      toast({ title: t("promotions.process_success") });
      queryClient.invalidateQueries({
        queryKey: trpc.academic.getYears.queryKey(),
      });
      onSuccess?.();
    },
  });

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit((data: any) => mutation.mutate(data))} className="space-y-4">
        <FormField
          control={form.control}
          name="yearName"
          render={({ field }) => (
            <FormItem>
              <FormLabel>{t("education.academic_year")}</FormLabel>
              <FormControl>
                <Input placeholder="2025-2026" {...field} />
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

        <FormField
          control={form.control}
          name="isCurrent"
          render={({ field }) => (
            <FormItem className="flex flex-row items-center justify-between rounded-lg border p-3">
              <div className="space-y-0.5">
                <FormLabel>{t("education.current")}</FormLabel>
              </div>
              <FormControl>
                <Switch
                  checked={field.value}
                  onCheckedChange={field.onChange}
                />
              </FormControl>
            </FormItem>
          )}
        />

        <Button type="submit" className="w-full" disabled={mutation.isPending}>
          {mutation.isPending ? t("common.loading") : t("common.save")}
        </Button>
      </form>
    </Form>
  );
}
