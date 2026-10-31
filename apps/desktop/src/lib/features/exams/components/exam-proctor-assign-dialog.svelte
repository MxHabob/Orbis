<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { createMutation, createQuery } from "@tanstack/svelte-query";
import { FormField } from "@orbis/ui";
import { Button } from "@orbis/ui/button";
import { Switch } from "@orbis/ui/switch";
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@orbis/ui/dialog";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
} from "@orbis/ui/select";
import { toast } from "@orbis/ui";
import { z } from "zod";

import { api, extractErrorMessage } from "$lib/api";
import { formatDate } from "$lib/core/i18n";
import { FormController } from "$lib/shared/forms/form-controller.svelte";

let {
  open = $bindable(false),
  orgId,
  schedules,
  onSuccess,
}: {
  open: boolean;
  orgId: string;
  schedules: Array<{
    id: string;
    exam_date: string;
    start_time: string;
    end_time: string;
  }>;
  onSuccess: () => void;
} = $props();

const assignmentSchema = z.object({
  exam_schedule_id: z
    .string()
    .trim()
    .min(1, m.operations_ui_proctor_select_schedule()),
  room_id: z.string().trim().min(1, m.operations_ui_proctor_select_room()),
  teacher_id: z
    .string()
    .trim()
    .min(1, m.operations_ui_proctor_select_teacher()),
  is_chief: z.boolean(),
});

const form = new FormController({
  initialValues: {
    exam_schedule_id: "",
    room_id: "",
    teacher_id: "",
    is_chief: false,
  },
  schema: assignmentSchema,
});

const roomsQuery = createQuery(() => ({
  queryKey: ["exam-proctor-rooms", orgId],
  queryFn: () => api.listRooms(),
  enabled: open && !!orgId,
  staleTime: 5 * 60 * 1000,
}));

const teachersQuery = createQuery(() => ({
  queryKey: ["exam-proctor-teachers", orgId],
  queryFn: () =>
    api.listEmployees({
      employee_type: "teacher",
      status: "active",
      page_size: 500,
    }),
  enabled: open && !!orgId,
  staleTime: 5 * 60 * 1000,
}));

const assignMutation = createMutation(() => ({
  mutationFn: (payload: z.infer<typeof assignmentSchema>) =>
    api.assignProctor(payload),
}));

const rooms = $derived(roomsQuery.data ?? []);
const examHalls = $derived(rooms.filter((room) => room.is_exam_hall));
const teachers = $derived(teachersQuery.data?.items ?? []);
const selectedSchedule = $derived(
  schedules.find((schedule) => schedule.id === form.values.exam_schedule_id),
);
const isLoadingOptions = $derived(
  roomsQuery.isLoading || teachersQuery.isLoading,
);
const isSaving = $derived(form.isSubmitting || assignMutation.isPending);

async function handleSubmit() {
  const submitted = await form.submit(async (payload) => {
    await assignMutation.mutateAsync(payload);
    toast.success(m.operations_ui_proctor_assign_success());
    form.reset();
    open = false;
    onSuccess();
  });

  if (!submitted && form.formError) {
    console.error("Failed to assign exam proctor:", form.formError);
  }
}
</script>

<Dialog bind:open>
  <DialogContent class="max-h-[90vh] max-w-md overflow-y-auto text-start">
    <DialogHeader>
      <DialogTitle>{m.operations_ui_assign_proctors()}</DialogTitle>
    </DialogHeader>

    <form class="flex flex-col gap-4" onsubmit={(event) => { event.preventDefault(); handleSubmit(); }}>
      {#if schedules.length === 0}
        <p class="text-sm text-muted-foreground">{m.academics_exams_no_exams_scheduled()}</p>
      {:else}
        <FormField
          id="proctor-schedule"
          label={m.operations_ui_proctor_select_schedule()}
          error={form.errors.exam_schedule_id?.[0]}
          required
        >
          <Select
            type="single"
            value={form.values.exam_schedule_id}
            onValueChange={(value) => form.setField("exam_schedule_id", value ?? "")}
            disabled={isLoadingOptions || isSaving}
          >
            <SelectTrigger id="proctor-schedule" class="h-9 w-full text-sm">
              {selectedSchedule
                ? `${formatDate(selectedSchedule.exam_date, "date") || selectedSchedule.exam_date} · ${selectedSchedule.start_time}–${selectedSchedule.end_time}`
                : m.operations_ui_proctor_select_schedule()}
            </SelectTrigger>
            <SelectContent>
              {#each schedules as schedule (schedule.id)}
                <SelectItem value={schedule.id}>
                  {formatDate(schedule.exam_date, "date") || schedule.exam_date}
                  · {schedule.start_time}–{schedule.end_time}
                </SelectItem>
              {/each}
            </SelectContent>
          </Select>
        </FormField>

        <FormField
          id="proctor-room"
          label={m.operations_ui_proctor_select_room()}
          error={form.errors.room_id?.[0]}
          required
        >
          <Select
            type="single"
            value={form.values.room_id}
            onValueChange={(value) => form.setField("room_id", value ?? "")}
            disabled={isLoadingOptions || isSaving || examHalls.length === 0}
          >
            <SelectTrigger id="proctor-room" class="h-9 w-full text-sm">
              {examHalls.find((room) => room.id === form.values.room_id)?.name_ar
                || examHalls.find((room) => room.id === form.values.room_id)?.name
                || m.operations_ui_proctor_select_room()}
            </SelectTrigger>
            <SelectContent>
              {#each examHalls as room (room.id)}
                <SelectItem value={room.id}>{room.name_ar || room.name}</SelectItem>
              {/each}
            </SelectContent>
          </Select>
        </FormField>

        <FormField
          id="proctor-teacher"
          label={m.operations_ui_proctor_select_teacher()}
          error={form.errors.teacher_id?.[0]}
          required
        >
          <Select
            type="single"
            value={form.values.teacher_id}
            onValueChange={(value) => form.setField("teacher_id", value ?? "")}
            disabled={isLoadingOptions || isSaving || teachers.length === 0}
          >
            <SelectTrigger id="proctor-teacher" class="h-9 w-full text-sm">
              {teachers.find((teacher) => teacher.id === form.values.teacher_id)?.full_name_ar
                || teachers.find((teacher) => teacher.id === form.values.teacher_id)?.full_name
                || m.operations_ui_proctor_select_teacher()}
            </SelectTrigger>
            <SelectContent>
              {#each teachers as teacher (teacher.id)}
                <SelectItem value={teacher.id}>
                  {teacher.full_name_ar || teacher.full_name || teacher.display_name || teacher.employee_number}
                </SelectItem>
              {/each}
            </SelectContent>
          </Select>
        </FormField>

        <label class="flex items-center justify-between gap-4 rounded-lg border border-border/60 px-3 py-3 text-sm">
          <span>{m.operations_ui_proctor_chief()}</span>
          <Switch
            checked={form.values.is_chief}
            onCheckedChange={(checked) => form.setField("is_chief", checked)}
            disabled={isSaving}
          />
        </label>
      {/if}

      {#if roomsQuery.error || teachersQuery.error}
        <p class="text-sm text-destructive" role="alert">
          {extractErrorMessage(roomsQuery.error || teachersQuery.error)}
        </p>
      {:else if !isLoadingOptions && schedules.length > 0 && examHalls.length === 0}
        <p class="text-sm text-muted-foreground">{m.operations_ui_proctor_no_exam_halls()}</p>
      {:else if !isLoadingOptions && schedules.length > 0 && teachers.length === 0}
        <p class="text-sm text-muted-foreground">{m.operations_ui_proctor_no_teachers()}</p>
      {/if}

      {#if form.formError}
        <p class="text-sm text-destructive" role="alert">{form.formError}</p>
      {/if}

      <DialogFooter class="border-t border-border/60 pt-4">
        <Button
          type="button"
          variant="outline"
          onclick={() => { form.reset(); open = false; }}
        >
          {m.common_action_cancel()}
        </Button>
        <Button
          type="submit"
          variant="default"
          disabled={isSaving || isLoadingOptions || schedules.length === 0 || examHalls.length === 0 || teachers.length === 0}
        >
          {isSaving ? m.operations_ui_proctor_assigning() : m.operations_ui_assign_proctors()}
        </Button>
      </DialogFooter>
    </form>
  </DialogContent>
</Dialog>
