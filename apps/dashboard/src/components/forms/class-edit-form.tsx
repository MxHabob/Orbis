"use client";

import { SelectParentClass } from "@/components/select-parent-class";
import { useClassParams } from "@/hooks/use-class-params";
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
import { InputColor } from "@/components/input-color";

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
  id: z.number(),
  name: z.string().min(1, "Name is required"),
  type: z.enum(classTypes).default('classroom'),
  division: z.string().optional().nullable(),
  parentClassId: z.number().optional().nullable(),
  color: z.string().default("#e6e6e6"),
  sortOrder: z.number().default(0),
});

type ClassEditFormValues = z.infer<typeof formSchema>;

type Props = {
  data?: any;
};

export function ClassEditForm({ data }: Props) {
  const trpc = useTRPC();
  const queryClient = useQueryClient();
  const { setParams } = useClassParams();



  const defaultValues = {
    id: data?.id,
    name: data?.name || "",
    type: data?.type || "classroom",
    division: data?.division || "",
    parentClassId: data?.parentClassId || undefined,
    color: data?.color || "#e6e6e6",
    sortOrder: data?.sortOrder || 0,
  };

  const form = useZodForm(formSchema, {
    values: defaultValues,
  });

  const updateClassMutation = useMutation(
    trpc.classes.update.mutationOptions({
      onSuccess: () => {
        queryClient.invalidateQueries({
          queryKey: trpc.classes.getAll.queryKey(),
        });
        queryClient.invalidateQueries({
          queryKey: trpc.classes.getById.queryKey({ id: data.id }),
        });
        queryClient.invalidateQueries({ queryKey: trpc.grades.getGridData.queryKey() });
        queryClient.invalidateQueries({ queryKey: trpc.assessments.getByClass.queryKey() });
        queryClient.invalidateQueries({ queryKey: trpc.students.getAll.queryKey() });
        setParams(null);
      },
    }),
  );

  const onSubmit = (formData: ClassEditFormValues) => {
    updateClassMutation.mutate({
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
                    Class Name
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
              render={({ field }) => {
                const hasChildren = data?.children && data.children.length > 0;
                
                return (
                  <FormItem className="space-y-1">
                    <FormLabel className="text-xs text-[#878787] font-normal">
                      Parent Class (Optional)
                    </FormLabel>
                    <FormControl>
                      {hasChildren ? (
                        <div className="flex items-center space-x-2 p-3 py-2 border border-border bg-muted/50">
                          <span className="text-sm text-muted-foreground">
                            Cannot change parent - this class has children
                          </span>
                        </div>
                      ) : (
                        <SelectParentClass
                          parentId={field.value}
                          onChange={(parent) => {
                            field.onChange(parent?.id ?? undefined);
                          }}
                          excludeIds={data?.id ? [data.id] : []}
                        />
                      )}
                    </FormControl>
                  </FormItem>
                );
              }}
            />

            <div className="grid grid-cols-2 gap-4">
               <FormField
                control={form.control}
                name="type"
                render={({ field }) => (
                  <FormItem className="space-y-1">
                    <FormLabel className="text-xs text-[#878787] font-normal">
                      Type
                    </FormLabel>
                    <Select onValueChange={field.onChange} defaultValue={field.value}>
                      <FormControl>
                        <SelectTrigger>
                          <SelectValue placeholder="Select type" />
                        </SelectTrigger>
                      </FormControl>
                      <SelectContent>
                        {classTypes.map((type) => (
                          <SelectItem key={type} value={type}>
                            {type.charAt(0).toUpperCase() + type.slice(1).replace('_', ' ')}
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
                      Division / Section
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
              name="sortOrder"
              render={({ field }) => (
                <FormItem className="space-y-1">
                  <FormLabel className="text-xs text-[#878787] font-normal">
                    Sort Order
                  </FormLabel>
                  <FormControl>
                    <Input
                      type="number"
                      {...field}
                      onChange={(e) => field.onChange(Number(e.target.value))}
                      placeholder="e.g. 1"
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
            isSubmitting={updateClassMutation.isPending}
            className="w-full"
          >
            Update Class
          </SubmitButton>
        </div>
      </form>
    </Form>
  );
}
