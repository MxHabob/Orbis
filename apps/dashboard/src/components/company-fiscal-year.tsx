"use client";

import { useCurrentAcademicYearQuery, useUpdateAcademicYearMutation } from "@/hooks/use-academic-year";
import { useZodForm } from "@/hooks/use-zod-form";
import { useI18n } from "@/locales/client";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@orbis/ui/card";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormMessage,
} from "@orbis/ui/form";
import { SubmitButton } from "@orbis/ui/submit-button";
import { z } from "zod";
import { Input } from "@orbis/ui/input";

const formSchema = z.object({
  yearName: z.string().min(1).max(20),
  startDate: z.coerce.date(),
  endDate: z.coerce.date().optional().nullable(),
});

export function SchoolAcademicYear() {
  const { data } = useCurrentAcademicYearQuery();
  const updateAcademicYearMutation = useUpdateAcademicYearMutation();
  const t = useI18n();

  const form = useZodForm(formSchema, {
    defaultValues: {
      yearName: data?.yearName ?? "",
      startDate: data?.startDate ? new Date(data.startDate) : new Date(),
      endDate: data?.endDate ? new Date(data.endDate) : null,
    },
  });

  const onSubmit = form.handleSubmit((values) => {
    if (data?.id) {
      updateAcademicYearMutation.mutate({
        id: data.id,
        ...values,
      });
    }
  });

  return (
    <Form {...form}>
      <form onSubmit={onSubmit}>
        <Card>
          <CardHeader>
            <CardTitle>{t("settings.academic_year_title")}</CardTitle>
            <CardDescription>
              {t("settings.academic_year_description")}
            </CardDescription>
          </CardHeader>

          <CardContent className="space-y-4">
            <FormField
              control={form.control}
              name="yearName"
              render={({ field }) => (
                <FormItem className="max-w-[300px]">
                  <FormControl>
                    <Input {...field} placeholder={t("settings.academic_year_placeholder")} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />

            <div className="flex gap-4">
              <FormField
                control={form.control}
                name="startDate"
                render={({ field }) => (
                  <FormItem className="flex-1">
                    <FormControl>
                      <Input 
                        type="date" 
                        {...field} 
                        value={field.value instanceof Date ? field.value.toISOString().split('T')[0] : field.value}
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
                  <FormItem className="flex-1">
                    <FormControl>
                      <Input 
                        type="date" 
                        {...field} 
                        value={field.value instanceof Date ? field.value.toISOString().split('T')[0] : (field.value ?? "")}
                      />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />
            </div>
          </CardContent>

          <CardFooter className="flex justify-end">
            <SubmitButton
              disabled={updateAcademicYearMutation.isPending || !form.formState.isDirty}
              isSubmitting={updateAcademicYearMutation.isPending}
            >
              {t("common.save")}
            </SubmitButton>
          </CardFooter>
        </Card>
      </form>
    </Form>
  );
}
