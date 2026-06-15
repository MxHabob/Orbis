"use client";

import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@orbis/ui/form";
import { Input } from "@orbis/ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@orbis/ui/select";
import { SubmitButton } from "@orbis/ui/submit-button";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useEffect } from "react";
import { z } from "zod/v3";
import { InputColor } from "@/components/input-color";
import { SelectParentClass } from "@/components/select-parent-class";
import { useClassParams } from "@/hooks/use-class-params";
import { useZodForm } from "@/hooks/use-zod-form";
import { useI18n, useScopedI18n } from "@/locales/client";
import { useTRPC } from "@/ipc/query-adapter";
import { invalidateClassDomain } from "@/utils/query-invalidation";

const classTypes = [
  "classroom",
  "laboratory",
  "library",
  "computer_lab",
  "music_room",
  "exam_hall",
  "workshop",
  "multi-purpose_room",
] as const;

const formSchema = z.object({
  name: z.string().min(1, "name_required"),
  type: z.enum(classTypes).default("classroom"),
  division: z.string().optional().nullable(),
  parentClassId: z.number().optional().nullable(),
  color: z.string().default("#e6e6e6"),
  sortOrder: z.number().default(0),
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
  const tEducation = useScopedI18n("education");

  const classMutation = useMutation(
    trpc.classes.create.mutationOptions({
      onSuccess: () => {
        void invalidateClassDomain(queryClient, trpc);
        setParams(null);
      },
    }),
  );

  const defaultValues = {
    name: data?.name || "",
    type: data?.type || "classroom",
    division: data?.division || "",
    parentClassId: data?.parentClassId || undefined,
    color: data?.color || "#e6e6e6",
    sortOrder: data?.sortOrder || 0,
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
                      placeholder={t("education.class_name_placeholder")}
                      onChange={({ name, color }) => {
                        field.onChange(name);
                        form.setValue("color", color);
                      }}
                      defaultValue={field.value}
                      defaultColor={form.watch("color")}
                    />
                  </FormControl>
                  <FormMessage>
                    {form.formState.errors.name?.message &&
                      tEducation(form.formState.errors.name.message as any)}
                  </FormMessage>
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
                    <Select
                      onValueChange={field.onChange}
                      defaultValue={field.value}
                    >
                      <FormControl>
                        <SelectTrigger>
                          <SelectValue
                            placeholder={t("education.select_type")}
                          />
                        </SelectTrigger>
                      </FormControl>
                      <SelectContent>
                        {classTypes.map((type) => (
                          <SelectItem key={type} value={type}>
                            {tEducation(type)}
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
                        placeholder={t("education.division_placeholder")}
                      />
                    </FormControl>
                  </FormItem>
                )}
              />
            </div>

            <FormField
              control={form.control}
              name="sortOrder"
              render={({ field }) => (
                <FormItem className="space-y-1">
                  <FormLabel className="text-xs text-[#878787] font-normal">
                    {t("education.sort_order")}
                  </FormLabel>
                  <FormControl>
                    <Input
                      type="number"
                      {...field}
                      onChange={(e) => field.onChange(Number(e.target.value))}
                      placeholder={t("education.sort_order_placeholder")}
                    />
                  </FormControl>
                  <FormMessage />
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
