"use client";

import { InputColor } from "@/components/input-color";
import { useSubjectParams } from "@/hooks/use-subject-params";
import { useZodForm } from "@/hooks/use-zod-form";
import { useTRPC } from "@/trpc/client";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@orbis/ui/form";
import { Input } from "@orbis/ui/input";
import { SubmitButton } from "@orbis/ui/submit-button";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useEffect } from "react";
import { z } from "zod/v3";
import { invalidateSubjectDomain } from "@/utils/query-invalidation";

const formSchema = z.object({
  id: z.number(),
  name: z.string().min(1, "Name is required"),
  code: z.string().optional().nullable(),
  color: z.string().default("#FFF3E0"),
  sortOrder: z.coerce.number().int().default(0),
});

type SubjectEditFormValues = z.infer<typeof formSchema>;

type Props = {
  data?: any;
};

export function SubjectEditForm({ data }: Props) {
  const trpc = useTRPC();
  const queryClient = useQueryClient();
  const { setParams } = useSubjectParams();

  const defaultValues = {
    id: data?.id,
    name: data?.name || "",
    code: data?.code || "",
    color: data?.color || "#FFF3E0",
    sortOrder: data?.sortOrder ?? 0,
  };

  const form = useZodForm(formSchema, {
    values: defaultValues,
  });

  const updateSubjectMutation = useMutation(
    trpc.subjects.update.mutationOptions({
      onSuccess: () => {
        void invalidateSubjectDomain(queryClient, trpc, data?.id);
        setParams(null);
      },
    }),
  );

  const onSubmit = (formData: SubjectEditFormValues) => {
    updateSubjectMutation.mutate({
      ...formData,
      code: formData.code || null,
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
                    Subject Name
                  </FormLabel>
                  <FormControl>
                    <InputColor
                      autoFocus
                      placeholder="e.g. Mathematics"
                      onChange={({ name, color }) => {
                        field.onChange(name);
                        form.setValue("color", color);
                      }}
                      defaultValue={field.value}
                      defaultColor={form.watch("color")}
                    />
                  </FormControl>
                </FormItem>
              )}
            />

            <FormField
              control={form.control}
              name="code"
              render={({ field }) => (
                <FormItem className="space-y-1">
                  <FormLabel className="text-xs text-[#878787] font-normal">
                    Subject Code (Optional)
                  </FormLabel>
                  <FormControl>
                    <Input
                      {...field}
                      value={field.value ?? ""}
                      placeholder="e.g. MATH101"
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />

            <FormField
              control={form.control}
              name="sortOrder"
              render={({ field }) => (
                <FormItem className="space-y-1">
                  <FormLabel className="text-xs text-[#878787] font-normal">
                    Sort Order
                  </FormLabel>
                  <FormControl>
                    <Input
                      {...field}
                      type="number"
                      placeholder="0"
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
          </div>
        </div>

        <div className="flex-1" />
        <div className="pt-6 border-t mt-auto">
          <SubmitButton
            isSubmitting={updateSubjectMutation.isPending}
            className="w-full"
          >
            Update Subject
          </SubmitButton>
        </div>
      </form>
    </Form>
  );
}
