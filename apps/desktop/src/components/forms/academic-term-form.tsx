import { createAcademicTermSchema } from "@/schemas/academic";
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
  useAcademicYears,
  useCreateAcademicTerm,
  useUpdateAcademicTerm,
} from "@/hooks/api/use-academics";

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
  const { toast } = useToast();
  const createMutation = useCreateAcademicTerm();
  const updateMutation = useUpdateAcademicTerm();

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
  const { data: years } = useAcademicYears();

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

  const isPending = createMutation.isPending || updateMutation.isPending;

  const onSubmit = (data: Record<string, unknown>) => {
    const id = initialData?.id || termId;
    if (id) {
      updateMutation.mutate(
        { cycleId: String(id), request: { ...data, id } },
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
