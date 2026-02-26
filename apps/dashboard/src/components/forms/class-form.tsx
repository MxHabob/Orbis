"use client";

import { InputColor } from "@/components/input-color";
import { SelectParentClass } from "@/components/select-parent-class";
import { useClassParams } from "@/hooks/use-class-params";
import { useI18n } from "@/locales/client";
import { useZodForm } from "@/hooks/use-zod-form";
import { useTRPC } from "@/trpc/client";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@midday/ui/form";
import { Input } from "@midday/ui/input";
import { SubmitButton } from "@midday/ui/submit-button";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@midday/ui/select";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useEffect } from "react";
import { z } from "zod/v3";

const classTypes = [
  'classroom',
  'laboratory',
  'library',
  'computer_lab',
  'music_room',
  'exam_hall',
  'workshop',
  'multi-purpose_room'
] as const;

const formSchema = z.object({
  name: z.string().min(1, "Name is required"),
  type: z.enum(classTypes).default('classroom'),
  division: z.string().optional().nullable(),
  parentClassId: z.number().optional().nullable(),
  academicYearId: z.number().optional().nullable(),
  color: z.string().default("#e6e6e6"),
});

type ClassFormValues = z.infer<typeof formSchema>;

type Props = {
  data?: any;
};

export function ClassForm({ data }: Props) {
  const trpc = useTRPC();
  const queryClient = useQueryClient();
  const { setParams } = useClassParams();
  const t = useI18n();

  // Fetch academic years for the dropdown
  const { data: academicYears } = useQuery(trpc.academic.getYears.queryOptions());

  const classMutation = useMutation(
    trpc.classes.create.mutationOptions({
      onSuccess: () => {
        queryClient.invalidateQueries({
          queryKey: trpc.classes.getAll.queryKey(),
        });
        setParams(null);
      },
    }),
  );

  const defaultValues = {
    name: data?.name || "",
    type: data?.type || "classroom",
    division: data?.division || "",
    parentClassId: data?.parentClassId || undefined,
    academicYearId: data?.academicYearId || undefined,
    color: data?.color || "#e6e6e6",
  };

  const form = useZodForm(formSchema, {
    defaultValues,
  });

  useEffect(() => {
    if (data) {
      form.reset(defaultValues);
    }
  }, [data, form]);

  const onSubmit = (formData: ClassFormValues) => {
    classMutation.mutate({
        ...formData,
        division: formData.division || null,
        parentClassId: formData.parentClassId || null,
        academicYearId: formData.academicYearId || null,
    });
  };

  return (
    <Form {...form}>
      <form
        onSubmit={form.handleSubmit(onSubmit)}
        className="flex flex-col space-y-6 h-[calc(100vh-130px)]"
      >
        <div className="flex flex-col space-y-6 overflow-auto">
          <div className="flex flex-col space-y-4">
            <FormField
              control={form.control}
              name="name"
              render={({ field }) => (
                <FormItem className="space-y-1">
                  <FormLabel className="text-xs text-[#878787] font-normal">
                    {t("education.class_name")}
                  </FormLabel>
                  <FormControl>
                    <InputColor
                      autoFocus
                      placeholder="e.g. Grade 10"
                      onChange={({ name, color }) => {
                        field.onChange(name);
                        form.setValue("color", color);
                      }}
                      defaultValue={field.value}
                      defaultColor={form.watch("color")}
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />

            <FormField
              control={form.control}
              name="parentClassId"
              render={({ field }) => (
                <FormItem className="space-y-1">
                  <FormLabel className="text-xs text-[#878787] font-normal">
                    {t("education.parent_class")}
                  </FormLabel>
                  <FormControl>
                    <SelectParentClass
                      parentId={field.value}
                      onChange={(parent) => {
                        field.onChange(parent?.id ?? undefined);
                      }}
                      excludeIds={data?.id ? [data.id] : []}
                    />
                  </FormControl>
                </FormItem>
              )}
            />

            <div className="grid grid-cols-2 gap-4">
               <FormField
                control={form.control}
                name="type"
                render={({ field }) => (
                  <FormItem className="space-y-1">
                    <FormLabel className="text-xs text-[#878787] font-normal">
                      {t("education.type")}
                    </FormLabel>
                    <Select onValueChange={field.onChange} defaultValue={field.value}>
                      <FormControl>
                        <SelectTrigger>
                          <SelectValue placeholder={t("education.select_type")} />
                        </SelectTrigger>
                      </FormControl>
                      <SelectContent>
                        {classTypes.map((type) => (
                          <SelectItem key={type} value={type}>
                            {t(`education.${type}` as any)}
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                  </FormItem>
                )}
              />

              <FormField
                control={form.control}
                name="division"
                render={({ field }) => (
                  <FormItem className="space-y-1">
                    <FormLabel className="text-xs text-[#878787] font-normal">
                      {t("education.division")}
                    </FormLabel>
                    <FormControl>
                      <Input
                        {...field}
                        value={field.value ?? ""}
                        placeholder="e.g. A"
                      />
                    </FormControl>
                  </FormItem>
                )}
              />
            </div>

            <FormField
              control={form.control}
              name="academicYearId"
              render={({ field }) => (
                <FormItem className="space-y-1">
                  <FormLabel className="text-xs text-[#878787] font-normal">
                    {t("education.academic_year")}
                  </FormLabel>
                  <Select 
                    onValueChange={(val) => field.onChange(val ? Number(val) : undefined)} 
                    defaultValue={field.value?.toString()}
                  >
                    <FormControl>
                      <SelectTrigger>
                        <SelectValue placeholder={t("education.select_year")} />
                      </SelectTrigger>
                    </FormControl>
                    <SelectContent>
                      {academicYears?.map((year) => (
                        <SelectItem key={year.id} value={year.id.toString()}>
                          {year.yearName} {year.isCurrent ? `(${t("education.current")})` : ""}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                </FormItem>
              )}
            />
          </div>
        </div>

        <div className="flex-1" />
        <div className="pt-6 border-t mt-auto text-right rtl:text-left">
          <SubmitButton
            isSubmitting={classMutation.isPending}
            className="w-full"
          >
            {data ? t("education.update_class") : t("education.create_class")}
          </SubmitButton>
        </div>
      </form>
    </Form>
  );
}
