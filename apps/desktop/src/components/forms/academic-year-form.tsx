import { createAcademicYearSchema } from "@/schemas/academic";
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
import { useEffect } from "react";
import { useForm } from "react-hook-form";
import { useI18n } from "@/locales/client";
import {
  useCreateAcademicYear,
  useUpdateAcademicYear,
} from "@/hooks/api/use-academics";

interface AcademicYearFormProps {
  initialData?: any;
  onSuccess?: () => void;
}

export function AcademicYearForm({
  initialData,
  onSuccess,
}: AcademicYearFormProps) {
  const t = useI18n();
  const { toast } = useToast();
  const createMutation = useCreateAcademicYear();
  const updateMutation = useUpdateAcademicYear();

  const form = useForm({
    resolver: zodResolver(createAcademicYearSchema),
    defaultValues: initialData || {
      yearName: "",
      startDate: new Date().toISOString(),
      endDate: new Date(
        new Date().setFullYear(new Date().getFullYear() + 1),
      ).toISOString(),
      isCurrent: false,
    },
  });

  useEffect(() => {
    if (initialData) {
      form.reset(initialData);
    }
  }, [initialData, form]);

  const isPending = createMutation.isPending || updateMutation.isPending;

  const onSubmit = (data: Record<string, unknown>) => {
    if (initialData?.id) {
      updateMutation.mutate(
        { cycleId: String(initialData.id), request: { ...data, id: initialData.id } },
        {
          onSuccess: () => {
            toast({ title: t("promotions.process_success") });
            onSuccess?.();
          },
          onError: (error) => {
            toast({
              title: t("common.error"),
              description: error.message,
              variant: "destructive",
            });
          },
        },
      );
    } else {
      createMutation.mutate(data, {
        onSuccess: () => {
          toast({ title: t("promotions.process_success") });
          onSuccess?.();
        },
        onError: (error) => {
          toast({
            title: t("common.error"),
            description: error.message,
            variant: "destructive",
          });
        },
      });
    }
  };

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
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

        <Button type="submit" className="w-full" disabled={isPending}>
          {isPending ? t("common.loading") : t("common.save")}
        </Button>
      </form>
    </Form>
  );
}
