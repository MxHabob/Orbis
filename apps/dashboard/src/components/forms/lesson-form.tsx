"use client";

import { useLessonParams } from "@/hooks/use-lesson-params";
import { useZodForm } from "@/hooks/use-zod-form";
import { useTRPC } from "@/trpc/client";
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
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@orbis/ui/select";
import { SubmitButton } from "@orbis/ui/submit-button";
import { Textarea } from "@orbis/ui/textarea";
import { Switch } from "@orbis/ui/switch";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useEffect } from "react";
import { z } from "zod/v3";

const daysOfWeek = [
  { value: 0, label: "Sunday" },
  { value: 1, label: "Monday" },
  { value: 2, label: "Tuesday" },
  { value: 3, label: "Wednesday" },
  { value: 4, label: "Thursday" },
  { value: 5, label: "Friday" },
  { value: 6, label: "Saturday" },
];

const formSchema = z.object({
  id: z.number().optional(),
  classId: z.number().min(1, "Class is required"),
  subjectId: z.number().min(1, "Subject is required"),
  teacherId: z.string().uuid("Teacher is required"),
  timeSlotId: z.number().min(1, "Time slot is required"),
  dayOfWeek: z.number().min(0).max(6),
  academicYearId: z.number().min(1, "Academic year is required"),
  academicTermId: z.number().optional().nullable(),
  roomId: z.number().optional().nullable(),
  isActive: z.boolean().default(true),
  notes: z.string().optional().nullable(),
});

type LessonFormValues = z.infer<typeof formSchema>;

type Props = {
  data?: any;
};

export function LessonForm({ data }: Props) {
  const trpc = useTRPC();
  const queryClient = useQueryClient();
  const { setParams } = useLessonParams();

  // Fetch options
  const { data: classes } = useQuery(trpc.classes.getAll.queryOptions({}));
  const { data: subjects } = useQuery(trpc.subjects.getAll.queryOptions());
  const { data: teachers } = useQuery(trpc.user.getStaff.queryOptions());
  const { data: academicYears } = useQuery(trpc.academic.getYears.queryOptions());
  // For time slots, we need an academicYearId. We'll use the current one or from form state.
  const { data: timeSlots } = useQuery(
    trpc.lessons.getTimeSlots.queryOptions(
      { yearId: data?.academicYearId || academicYears?.find(y => y.isCurrent)?.id || 0 },
      { enabled: !!academicYears }
    )
  );

  const createMutation = useMutation(
    trpc.lessons.create.mutationOptions({
      onSuccess: () => {
        queryClient.invalidateQueries({ queryKey: trpc.lessons.getClassSchedule.queryKey() });
        queryClient.invalidateQueries({ queryKey: trpc.lessons.getTeacherSchedule.queryKey() });
        setParams({ create: null });
      },
    })
  );

  const updateMutation = useMutation(
    trpc.lessons.update.mutationOptions({
      onSuccess: () => {
        queryClient.invalidateQueries({ queryKey: trpc.lessons.getClassSchedule.queryKey() });
        queryClient.invalidateQueries({ queryKey: trpc.lessons.getTeacherSchedule.queryKey() });
        setParams({ update: null, lessonId: null });
      },
    })
  );

  const defaultValues: Partial<LessonFormValues> = {
    id: data?.id,
    classId: data?.classId,
    subjectId: data?.subjectId,
    teacherId: data?.teacherId,
    timeSlotId: data?.timeSlotId,
    dayOfWeek: data?.dayOfWeek ?? 0,
    academicYearId: data?.academicYearId || academicYears?.find(y => y.isCurrent)?.id,
    academicTermId: data?.academicTermId,
    roomId: data?.roomId,
    isActive: data?.isActive ?? true,
    notes: data?.notes,
  };

  const form = useZodForm(formSchema, {
    defaultValues,
  });

  useEffect(() => {
    if (data) {
      form.reset(defaultValues);
    }
  }, [data]);

  const onSubmit = (formData: LessonFormValues) => {
    if (data?.id) {
      updateMutation.mutate({ ...formData, id: data.id } as any);
    } else {
      const { id, ...createData } = formData;
      createMutation.mutate(createData as any);
    }
  };

  return (
    <Form {...form}>
      <form
        onSubmit={form.handleSubmit(onSubmit)}
        className="flex flex-col space-y-6 h-[calc(100vh-130px)]"
      >
        <div className="flex flex-col space-y-6 overflow-auto pb-20">
          <div className="grid grid-cols-2 gap-4">
            <FormField
              control={form.control}
              name="classId"
              render={({ field }) => (
                <FormItem className="space-y-1">
                  <FormLabel className="text-xs text-[#878787] font-normal">
                    Class
                  </FormLabel>
                  <Select 
                    onValueChange={(val) => field.onChange(Number(val))} 
                    value={field.value?.toString()}
                  >
                    <FormControl>
                      <SelectTrigger>
                        <SelectValue placeholder="Select class" />
                      </SelectTrigger>
                    </FormControl>
                    <SelectContent>
                      {classes?.data?.map((c) => (
                        <SelectItem key={c.id} value={c.id.toString()}>
                          {c.name}
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
              name="subjectId"
              render={({ field }) => (
                <FormItem className="space-y-1">
                  <FormLabel className="text-xs text-[#878787] font-normal">
                    Subject
                  </FormLabel>
                  <Select 
                    onValueChange={(val) => field.onChange(Number(val))} 
                    value={field.value?.toString()}
                  >
                    <FormControl>
                      <SelectTrigger>
                        <SelectValue placeholder="Select subject" />
                      </SelectTrigger>
                    </FormControl>
                    <SelectContent>
                      {subjects?.map((s) => (
                        <SelectItem key={s.id} value={s.id.toString()}>
                          {s.name}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                  <FormMessage />
                </FormItem>
              )}
            />
          </div>

          <FormField
            control={form.control}
            name="teacherId"
            render={({ field }) => (
              <FormItem className="space-y-1">
                <FormLabel className="text-xs text-[#878787] font-normal">
                  Teacher
                </FormLabel>
                <Select 
                  onValueChange={field.onChange} 
                  value={field.value}
                >
                  <FormControl>
                    <SelectTrigger>
                      <SelectValue placeholder="Select teacher" />
                    </SelectTrigger>
                  </FormControl>
                  <SelectContent>
                    {teachers?.map((t) => (
                      <SelectItem key={t.id} value={t.id}>
                        {t.name}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
                <FormMessage />
              </FormItem>
            )}
          />

          <div className="grid grid-cols-2 gap-4">
            <FormField
              control={form.control}
              name="dayOfWeek"
              render={({ field }) => (
                <FormItem className="space-y-1">
                  <FormLabel className="text-xs text-[#878787] font-normal">
                    Day
                  </FormLabel>
                  <Select 
                    onValueChange={(val) => field.onChange(Number(val))} 
                    value={field.value?.toString()}
                  >
                    <FormControl>
                      <SelectTrigger>
                        <SelectValue placeholder="Select day" />
                      </SelectTrigger>
                    </FormControl>
                    <SelectContent>
                      {daysOfWeek.map((day) => (
                        <SelectItem key={day.value} value={day.value.toString()}>
                          {day.label}
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
              name="timeSlotId"
              render={({ field }) => (
                <FormItem className="space-y-1">
                  <FormLabel className="text-xs text-[#878787] font-normal">
                    Period / Time Slot
                  </FormLabel>
                  <Select 
                    onValueChange={(val) => field.onChange(Number(val))} 
                    value={field.value?.toString()}
                  >
                    <FormControl>
                      <SelectTrigger>
                        <SelectValue placeholder="Select period" />
                      </SelectTrigger>
                    </FormControl>
                    <SelectContent>
                      {timeSlots?.map((slot) => (
                        <SelectItem key={slot.id} value={slot.id.toString()}>
                          {slot.slotNumber}. {slot.startTime} - {slot.endTime}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                  <FormMessage />
                </FormItem>
              )}
            />
          </div>

          <FormField
            control={form.control}
            name="notes"
            render={({ field }) => (
              <FormItem className="space-y-1">
                <FormLabel className="text-xs text-[#878787] font-normal">
                  Notes
                </FormLabel>
                <FormControl>
                  <Textarea 
                    {...field} 
                    value={field.value ?? ""} 
                    placeholder="Extra instructions or information..."
                    className="min-h-[100px]"
                  />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />

          <FormField
            control={form.control}
            name="isActive"
            render={({ field }) => (
              <FormItem className="flex flex-row items-center justify-between rounded-lg border p-4">
                <div className="space-y-0.5">
                  <FormLabel className="text-base">Active Status</FormLabel>
                  <FormDescription>
                    Enable or disable this lesson in the schedule.
                  </FormDescription>
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
        </div>

        <div className="flex-1" />
        <div className="fixed bottom-8 w-full sm:max-w-[455px] right-8">
          <SubmitButton
            isSubmitting={createMutation.isPending || updateMutation.isPending}
            className="w-full"
          >
            {data ? "Update Lesson" : "Create Lesson"}
          </SubmitButton>
        </div>
      </form>
    </Form>
  );
}
