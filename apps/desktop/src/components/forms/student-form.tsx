"use client";

import type { RouterOutputs } from "@api/trpc/routers/_app";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@orbis/ui/accordion";
import { Button } from "@orbis/ui/button";
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@orbis/ui/form";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@orbis/ui/select";
import { SubmitButton } from "@orbis/ui/submit-button";
import { Textarea } from "@orbis/ui/textarea";
import { useToast } from "@orbis/ui/use-toast";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import React from "react";
import { z } from "zod/v3";
import { AvatarUpload } from "@/components/avatar-upload";
import { useStudentParams } from "@/hooks/use-student-params";
import { useZodForm } from "@/hooks/use-zod-form";
import { useI18n } from "@/locales/client";
import { useTRPC } from "@/trpc/client";
import { invalidateStudentDomain } from "@/utils/query-invalidation";

const formSchema = z.object({
  id: z.string().uuid().optional(),
  image: z.string().optional().nullable(),
  name: z.string().min(2, {
    message: "Name must be at least 2 characters.",
  }),
  email: z
    .string()
    .email({
      message: "Email is not valid.",
    })
    .optional()
    .nullable()
    .or(z.literal("")),
  phone: z.string().optional().nullable(),
  sex: z.enum(["male", "female", "unspecified"]).default("unspecified"),
  studentNumber: z.string().optional().nullable().or(z.literal("")),
  secretNumber: z.string().optional().nullable(),
  currentClassId: z.string().optional().nullable(),
  status: z
    .enum([
      "active",
      "graduated",
      "transferred",
      "suspended",
      "auditor",
      "homeschooled",
    ])
    .default("active"),
  note: z.string().optional().nullable(),
  fatherName: z.string().optional().nullable(),
  fatherPhone: z.string().optional().nullable(),
  motherName: z.string().optional().nullable(),
  motherPhone: z.string().optional().nullable(),
  metadata: z.record(z.any()).optional().nullable(),
});

type Props = {
  data?: any; // We'll refine this if needed
};

export function StudentForm({ data }: Props) {
  const trpc = useTRPC();
  const queryClient = useQueryClient();
  const isEdit = !!data;
  const t = useI18n();

  const { setParams: setStudentParams, classId } = useStudentParams();

  const { data: classes } = useQuery(trpc.classes.getAll.queryOptions());

  const { toast } = useToast();

  const upsertStudentMutation = useMutation(
    trpc.students.upsert.mutationOptions({
      onSuccess: () => {
        void invalidateStudentDomain(queryClient, trpc);

        toast({
          title: t("students.student_added_success"),
          variant: "success",
        });

        // Close the student form
        setStudentParams(null);
      },
      onError: (error) => {
        toast({
          title: t("students.add_error"),
          description: error.message,
          variant: "error",
          duration: 8000,
        });
      },
    }),
  );

  const defaultValues = React.useMemo(
    () => ({
      id: data?.userId,
      image: data?.image ?? null,
      name: data?.name ?? "",
      email: data?.email ?? "",
      phone: data?.phone ?? "",
      sex: (data?.sex as "male" | "female" | "unspecified") ?? "unspecified",
      studentNumber: data?.studentNumber ?? "",
      secretNumber: data?.secretNumber ?? "",
      currentClassId: data?.currentClassId?.toString() ?? classId ?? "",
      status: (data?.status as any) ?? "active",
      note: data?.notes ?? "",
      fatherName: data?.fatherName ?? "",
      fatherPhone: data?.fatherPhone ?? "",
      motherName: data?.motherName ?? "",
      motherPhone: data?.motherPhone ?? "",
      metadata: data?.metadata ?? null,
    }),
    [data, classId],
  );

  const form = useZodForm(formSchema, {
    values: defaultValues,
  });

  const handleSubmit = (values: z.infer<typeof formSchema>) => {
    upsertStudentMutation.mutate({
      userId: values.id,
      name: values.name,
      image: values.image || null,
      email: values.email || null,
      phone: values.phone || null,
      sex: values.sex,
      studentNumber: values.studentNumber || null,
      secretNumber: values.secretNumber || null,
      currentClassId: values.currentClassId
        ? Number(values.currentClassId)
        : null,
      status: values.status,
      notes: values.note || null,
      fatherName: values.fatherName || null,
      fatherPhone: values.fatherPhone || null,
      motherName: values.motherName || null,
      motherPhone: values.motherPhone || null,
      metadata: values.metadata || null,
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
                <AccordionTrigger>
                  {t("students.general_info")}
                </AccordionTrigger>
                <AccordionContent>
                  <div className="space-y-4">
                    <FormField
                      control={form.control}
                      name="name"
                      render={({ field }) => (
                        <FormItem>
                          <div className="flex items-center gap-3">
                            {data?.userId && (
                              <AvatarUpload
                                userId={data.userId}
                                avatarUrl={data?.image}
                                folder="students"
                                size={40}
                                skipUserUpdate
                                onUpload={(url) => {
                                  form.setValue("image", url, {
                                    shouldDirty: true,
                                  });
                                }}
                              />
                            )}
                            <FormControl>
                              <Input
                                {...field}
                                autoFocus
                                placeholder="John Doe"
                                autoComplete="off"
                              />
                            </FormControl>
                          </div>
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
                              {t("students.email")}
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
                              {t("students.phone")}
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
                            {t("students.sex")}
                          </FormLabel>
                          <Select
                            onValueChange={field.onChange}
                            defaultValue={field.value}
                          >
                            <FormControl>
                              <SelectTrigger>
                                <SelectValue
                                  placeholder={t("students.select_sex")}
                                />
                              </SelectTrigger>
                            </FormControl>
                            <SelectContent>
                              <SelectItem value="unspecified">
                                {t("students.unspecified")}
                              </SelectItem>
                              <SelectItem value="male">
                                {t("students.male")}
                              </SelectItem>
                              <SelectItem value="female">
                                {t("students.female")}
                              </SelectItem>
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
                <AccordionTrigger>
                  {t("students.academic_details")}
                </AccordionTrigger>
                <AccordionContent>
                  <div className="space-y-4">
                    <div className="grid grid-cols-2 gap-4">
                      <FormField
                        control={form.control}
                        name="studentNumber"
                        render={({ field }) => (
                          <FormItem>
                            <FormLabel className="text-xs text-[#878787] font-normal">
                              {t("students.student_number")}
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
                              {t("students.secret_number")}
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
                              {t("students.anonymous_grading_desc")}
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
                            {t("students.class")}
                          </FormLabel>
                          <Select
                            onValueChange={field.onChange}
                            defaultValue={field.value ?? ""}
                          >
                            <FormControl>
                              <SelectTrigger>
                                <SelectValue
                                  placeholder={t("students.select_class")}
                                />
                              </SelectTrigger>
                            </FormControl>
                            <SelectContent>
                              {classes?.data?.map((cls) => (
                                <SelectItem
                                  key={cls.id}
                                  value={cls.id.toString()}
                                >
                                  {cls.name}{" "}
                                  {cls.division ? `(${cls.division})` : ""}
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
                            {t("students.status")}
                          </FormLabel>
                          <Select
                            onValueChange={field.onChange}
                            defaultValue={field.value}
                          >
                            <FormControl>
                              <SelectTrigger>
                                <SelectValue
                                  placeholder={t("students.select_status")}
                                />
                              </SelectTrigger>
                            </FormControl>
                            <SelectContent>
                              <SelectItem value="active">
                                {t("students.active")}
                              </SelectItem>
                              <SelectItem value="graduated">
                                {t("students.graduated")}
                              </SelectItem>
                              <SelectItem value="transferred">
                                {t("students.transferred")}
                              </SelectItem>
                              <SelectItem value="suspended">
                                {t("students.suspended")}
                              </SelectItem>
                              <SelectItem value="auditor">
                                {t("students.auditor")}
                              </SelectItem>
                              <SelectItem value="homeschooled">
                                {t("students.homeschooled")}
                              </SelectItem>
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
                            {t("students.note")}
                          </FormLabel>
                          <FormControl>
                            <Textarea
                              {...field}
                              value={field.value ?? ""}
                              className="flex min-h-[80px] resize-none"
                              placeholder={t("students.additional_info")}
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

              <AccordionItem value="additional">
                <AccordionTrigger>
                  {t("students.additional_info")}
                </AccordionTrigger>
                <AccordionContent>
                  <div className="space-y-6">
                    <div className="grid grid-cols-2 gap-4">
                      <FormField
                        control={form.control}
                        name="fatherName"
                        render={({ field }) => (
                          <FormItem>
                            <FormLabel className="text-xs text-[#878787] font-normal">
                              {t("students.father_name")}
                            </FormLabel>
                            <FormControl>
                              <Input
                                {...field}
                                value={field.value ?? ""}
                                placeholder="Father Full Name"
                                autoComplete="off"
                              />
                            </FormControl>
                            <FormMessage />
                          </FormItem>
                        )}
                      />

                      <FormField
                        control={form.control}
                        name="fatherPhone"
                        render={({ field }) => (
                          <FormItem>
                            <FormLabel className="text-xs text-[#878787] font-normal">
                              {t("students.father_phone")}
                            </FormLabel>
                            <FormControl>
                              <Input
                                {...field}
                                value={field.value ?? ""}
                                placeholder="Father Phone"
                                autoComplete="off"
                              />
                            </FormControl>
                            <FormMessage />
                          </FormItem>
                        )}
                      />
                    </div>

                    <div className="grid grid-cols-2 gap-4">
                      <FormField
                        control={form.control}
                        name="motherName"
                        render={({ field }) => (
                          <FormItem>
                            <FormLabel className="text-xs text-[#878787] font-normal">
                              {t("students.mother_name")}
                            </FormLabel>
                            <FormControl>
                              <Input
                                {...field}
                                value={field.value ?? ""}
                                placeholder="Mother Full Name"
                                autoComplete="off"
                              />
                            </FormControl>
                            <FormMessage />
                          </FormItem>
                        )}
                      />

                      <FormField
                        control={form.control}
                        name="motherPhone"
                        render={({ field }) => (
                          <FormItem>
                            <FormLabel className="text-xs text-[#878787] font-normal">
                              {t("students.mother_phone")}
                            </FormLabel>
                            <FormControl>
                              <Input
                                {...field}
                                value={field.value ?? ""}
                                placeholder="Mother Phone"
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
                      name="metadata"
                      render={({ field }) => (
                        <FormItem>
                          <FormLabel className="text-xs text-[#878787] font-normal uppercase tracking-wider">
                            {t("students.metadata")}
                          </FormLabel>
                          <FormControl>
                            <Textarea
                              className="font-mono text-[11px] min-h-[120px] resize-none"
                              placeholder='{ "key": "value" }'
                              value={
                                field.value && typeof field.value === "object"
                                  ? JSON.stringify(field.value, null, 2)
                                  : field.value || ""
                              }
                              onChange={(e) => {
                                const val = e.target.value;
                                try {
                                  if (val.trim() === "") {
                                    field.onChange(null);
                                  } else {
                                    const parsed = JSON.parse(val);
                                    field.onChange(parsed);
                                  }
                                } catch (err) {
                                  field.onChange(val);
                                }
                              }}
                            />
                          </FormControl>
                          <FormDescription className="text-[10px]">
                            {t("students.metadata_desc")}
                          </FormDescription>
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
              {t("common.cancel")}
            </Button>

            <SubmitButton
              isSubmitting={upsertStudentMutation.isPending}
              disabled={
                upsertStudentMutation.isPending || !form.formState.isDirty
              }
            >
              {isEdit ? t("students.update") : t("students.create")}
            </SubmitButton>
          </div>
        </div>
      </form>
    </Form>
  );
}
