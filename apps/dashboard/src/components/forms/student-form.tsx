"use client";

import { useStudentParams } from "@/hooks/use-student-params";
import { useZodForm } from "@/hooks/use-zod-form";
import { useTRPC } from "@/trpc/client";
import type { RouterOutputs } from "@api/trpc/routers/_app";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@midday/ui/accordion";
import { Button } from "@midday/ui/button";
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@midday/ui/form";
import { Input } from "@midday/ui/input";
import { Label } from "@midday/ui/label";
import { SubmitButton } from "@midday/ui/submit-button";
import { Textarea } from "@midday/ui/textarea";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@midday/ui/select";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { z } from "zod/v3";

const formSchema = z.object({
  id: z.string().uuid().optional(),
  name: z.string().min(2, {
    message: "Name must be at least 2 characters.",
  }),
  email: z.string().email({
    message: "Email is not valid.",
  }).optional().nullable().or(z.literal("")),
  phone: z.string().optional().nullable(),
  sex: z.enum(['male', 'female', 'unspecified']).default('unspecified'),
  studentNumber: z.string().optional().nullable().or(z.literal("")),
  secretNumber: z.string().optional().nullable(),
  currentClassId: z.string().optional().nullable(),
  status: z.enum(['active', 'graduated', 'transferred', 'suspended']).default('active'),
  note: z.string().optional().nullable(),
});

type Props = {
  data?: any; // We'll refine this if needed
};

export function StudentForm({ data }: Props) {
  const trpc = useTRPC();
  const queryClient = useQueryClient();
  const isEdit = !!data;

  const { setParams: setStudentParams, classId } = useStudentParams();

  const { data: classes } = useQuery(trpc.classes.getAll.queryOptions());

  const upsertStudentMutation = useMutation(
    trpc.students.upsert.mutationOptions({
      onSuccess: () => {
        queryClient.invalidateQueries({
          queryKey: trpc.students.getAll.queryKey(),
        });

        // Close the student form
        setStudentParams(null);
      },
    }),
  );

  const form = useZodForm(formSchema, {
    defaultValues: {
      id: data?.userId,
      name: data?.name ?? "",
      email: data?.email ?? "",
      phone: data?.phone ?? "",
      sex: data?.sex ?? "unspecified",
      studentNumber: data?.studentNumber ?? "",
      secretNumber: data?.secretNumber ?? "",
      currentClassId: data?.currentClassId?.toString() ?? classId ?? "",
      status: data?.status ?? "active",
      note: data?.stats?.note ?? "",
    },
  });

  const handleSubmit = (values: z.infer<typeof formSchema>) => {
    // Auto-generate numbers if empty
    const generatedStudentNumber = values.studentNumber || `ST-${Math.floor(100000 + Math.random() * 900000)}`;
    const generatedSecretNumber = values.secretNumber || `SEC-${Math.random().toString(36).substring(2, 8).toUpperCase()}`;

    upsertStudentMutation.mutate({
      userId: values.id,
      name: values.name,
      email: values.email || null,
      phone: values.phone || null,
      sex: values.sex,
      studentNumber: generatedStudentNumber,
      secretNumber: generatedSecretNumber,
      currentClassId: values.currentClassId ? Number(values.currentClassId) : null,
      status: values.status,
      stats: values.note ? { note: values.note } : null,
    });
  };

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(handleSubmit)}>
        <div className="h-[calc(100vh-180px)] scrollbar-hide overflow-auto">
          <div>
            <Accordion
              type="multiple"
              defaultValue={["general", "academic"]}
              className="space-y-6"
            >
              <AccordionItem value="general">
                <AccordionTrigger>General Information</AccordionTrigger>
                <AccordionContent>
                  <div className="space-y-4">
                    <FormField
                      control={form.control}
                      name="name"
                      render={({ field }) => (
                        <FormItem>
                          <FormLabel className="text-xs text-[#878787] font-normal">
                            Full Name
                          </FormLabel>
                          <FormControl>
                            <Input
                              {...field}
                              autoFocus
                              placeholder="John Doe"
                              autoComplete="off"
                            />
                          </FormControl>
                          <FormMessage />
                        </FormItem>
                      )}
                    />

                    <div className="grid grid-cols-2 gap-4">
                      <FormField
                        control={form.control}
                        name="email"
                        render={({ field }) => (
                          <FormItem>
                            <FormLabel className="text-xs text-[#878787] font-normal">
                              Email
                            </FormLabel>
                            <FormControl>
                              <Input
                                {...field}
                                value={field.value ?? ""}
                                placeholder="john@example.com"
                                type="email"
                                autoComplete="off"
                              />
                            </FormControl>
                            <FormMessage />
                          </FormItem>
                        )}
                      />

                      <FormField
                        control={form.control}
                        name="phone"
                        render={({ field }) => (
                          <FormItem>
                            <FormLabel className="text-xs text-[#878787] font-normal">
                              Phone
                            </FormLabel>
                            <FormControl>
                              <Input
                                {...field}
                                value={field.value ?? ""}
                                placeholder="+1 (555) 000-0000"
                                type="tel"
                                autoComplete="off"
                              />
                            </FormControl>
                            <FormMessage />
                          </FormItem>
                        )}
                      />
                    </div>

                    <FormField
                      control={form.control}
                      name="sex"
                      render={({ field }) => (
                        <FormItem>
                          <FormLabel className="text-xs text-[#878787] font-normal">
                            Sex
                          </FormLabel>
                          <Select onValueChange={field.onChange} defaultValue={field.value}>
                            <FormControl>
                              <SelectTrigger>
                                <SelectValue placeholder="Select sex" />
                              </SelectTrigger>
                            </FormControl>
                            <SelectContent>
                              <SelectItem value="unspecified">Unspecified</SelectItem>
                              <SelectItem value="male">Male</SelectItem>
                              <SelectItem value="female">Female</SelectItem>
                            </SelectContent>
                          </Select>
                          <FormMessage />
                        </FormItem>
                      )}
                    />
                  </div>
                </AccordionContent>
              </AccordionItem>

              <AccordionItem value="academic">
                <AccordionTrigger>Academic Details</AccordionTrigger>
                <AccordionContent>
                  <div className="space-y-4">
                    <div className="grid grid-cols-2 gap-4">
                      <FormField
                        control={form.control}
                        name="studentNumber"
                        render={({ field }) => (
                          <FormItem>
                            <FormLabel className="text-xs text-[#878787] font-normal">
                              Student Number
                            </FormLabel>
                            <FormControl>
                              <Input
                                {...field}
                                value={field.value ?? ""}
                                placeholder="ST12345"
                                autoComplete="off"
                              />
                            </FormControl>
                            <FormMessage />
                          </FormItem>
                        )}
                      />

                      <FormField
                        control={form.control}
                        name="secretNumber"
                        render={({ field }) => (
                          <FormItem>
                            <FormLabel className="text-xs text-[#878787] font-normal">
                              Secret Number
                            </FormLabel>
                            <FormControl>
                              <Input
                                {...field}
                                value={field.value ?? ""}
                                placeholder="SEC-001"
                                autoComplete="off"
                              />
                            </FormControl>
                            <FormDescription>
                              Used for anonymous grading.
                            </FormDescription>
                            <FormMessage />
                          </FormItem>
                        )}
                      />
                    </div>

                    <FormField
                      control={form.control}
                      name="currentClassId"
                      render={({ field }) => (
                        <FormItem>
                          <FormLabel className="text-xs text-[#878787] font-normal">
                            Class
                          </FormLabel>
                          <Select onValueChange={field.onChange} defaultValue={field.value ?? ""}>
                            <FormControl>
                              <SelectTrigger>
                                <SelectValue placeholder="Select a class" />
                              </SelectTrigger>
                            </FormControl>
                            <SelectContent>
                              {classes?.map((cls) => (
                                <SelectItem key={cls.id} value={cls.id.toString()}>
                                  {cls.name} {cls.division ? `(${cls.division})` : ""}
                                </SelectItem>
                              ))}
                            </SelectContent>
                          </Select>
                          <FormMessage />
                        </FormItem>
                      )}
                    />

                    <FormField
                      control={form.control}
                      name="status"
                      render={({ field }) => (
                        <FormItem>
                          <FormLabel className="text-xs text-[#878787] font-normal">
                            Status
                          </FormLabel>
                          <Select onValueChange={field.onChange} defaultValue={field.value}>
                            <FormControl>
                              <SelectTrigger>
                                <SelectValue placeholder="Select status" />
                              </SelectTrigger>
                            </FormControl>
                            <SelectContent>
                              <SelectItem value="active">Active</SelectItem>
                              <SelectItem value="graduated">Graduated</SelectItem>
                              <SelectItem value="transferred">Transferred</SelectItem>
                              <SelectItem value="suspended">Suspended</SelectItem>
                            </SelectContent>
                          </Select>
                          <FormMessage />
                        </FormItem>
                      )}
                    />

                    <FormField
                      control={form.control}
                      name="note"
                      render={({ field }) => (
                        <FormItem>
                          <FormLabel className="text-xs text-[#878787] font-normal">
                            Note
                          </FormLabel>
                          <FormControl>
                            <Textarea
                              {...field}
                              value={field.value ?? ""}
                              className="flex min-h-[80px] resize-none"
                              placeholder="Additional information..."
                              autoComplete="off"
                            />
                          </FormControl>
                          <FormMessage />
                        </FormItem>
                      )}
                    />
                  </div>
                </AccordionContent>
              </AccordionItem>
            </Accordion>
          </div>
        </div>

        <div className="absolute bottom-0 left-0 right-0 p-4">
          <div className="flex justify-end mt-auto space-x-4">
            <Button
              variant="outline"
              onClick={() => setStudentParams(null)}
              type="button"
            >
              Cancel
            </Button>

            <SubmitButton
              isSubmitting={upsertStudentMutation.isPending}
              disabled={
                upsertStudentMutation.isPending || !form.formState.isDirty
              }
            >
              {isEdit ? "Update" : "Create"}
            </SubmitButton>
          </div>
        </div>
      </form>
    </Form>
  );
}
