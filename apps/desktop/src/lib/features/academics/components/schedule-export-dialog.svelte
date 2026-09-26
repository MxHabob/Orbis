<script lang="ts">
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@orbis/ui/dialog";
import { Button } from "@orbis/ui/button";
import { formatDate } from "$lib/core/i18n";
import { Label } from "@orbis/ui/label";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import { AdjustmentsHorizontal, DocumentPage, Download, Grid, Keyframes, Pin, User, Users } from "@orbis/icons";
import * as m from "$lib/paraglide/messages";
import { l } from "$lib/core/i18n";

interface Props {
  open: boolean;
  periodName: string;
  viewMode: "class" | "teacher" | "room" | "subject" | "matrix";
  selectedClassId: string;
  selectedTeacherId: string;
  selectedRoomId: string;
  classes: any[];
  teachers: any[];
  rooms: any[];
  timeSlots: any[];
  weekdays: any[];
  lessons: any[];
  getSubjectName: (id: string) => string;
  getTeacherName: (id: string) => string;
  getClassName: (id: string) => string;
  getRoomName: (id: string) => string;
}

let {
  open = $bindable(false),
  periodName = "",
  viewMode = "class",
  selectedClassId = "",
  selectedTeacherId = "",
  selectedRoomId = "",
  classes = [],
  teachers = [],
  rooms = [],
  timeSlots = [],
  weekdays = [],
  lessons = [],
  getSubjectName,
  getTeacherName,
  getClassName,
  getRoomName,
}: Props = $props();

let exportType = $state<"class" | "teacher" | "room" | "master">("class");
let exportTargetId = $state("");

$effect(() => {
  if (open) {
    if (viewMode === "teacher") {
      exportType = "teacher";
      exportTargetId = selectedTeacherId;
    } else if (viewMode === "room") {
      exportType = "room";
      exportTargetId = selectedRoomId;
    } else {
      exportType = "class";
      exportTargetId = selectedClassId;
    }
  }
});

const exportTitle = $derived.by(() => {
  if (exportType === "class") {
    const cls = classes.find((c) => String(c.id) === exportTargetId);
    return cls ? l(cls.name_ar, cls.name) : m.academics_schedule_period_title();
  }
  if (exportType === "teacher") {
    const teach = teachers.find((t) => String(t.id) === exportTargetId);
    return teach
      ? l(teach.name_ar, teach.full_name)
      : m.academics_schedule_teacher_title();
  }
  if (exportType === "room") {
    const rm = rooms.find((r) => String(r.id) === exportTargetId);
    return rm ? l(rm.name_ar, rm.name) : m.academics_schedule_room_title();
  }
  return m.academics_schedule_master_title();
});

const targetLessons = $derived.by(() => {
  if (exportType === "class") {
    return lessons.filter((l) => String(l.class_id) === exportTargetId);
  }
  if (exportType === "teacher") {
    return lessons.filter((l) => String(l.teacher_id) === exportTargetId);
  }
  if (exportType === "room") {
    return lessons.filter((l) => String(l.room_id) === exportTargetId);
  }
  return lessons;
});

function getLessonInSlot(day: number, slotId: string) {
  return targetLessons.find(
    (l) => Number(l.day_of_week) === day && String(l.time_slot_id) === slotId,
  );
}

function handlePrint() {
  window.print();
}
</script>

<Dialog bind:open>
  <DialogContent class="w-full sm:max-w-3xl overflow-y-auto max-h-[90vh] flex flex-col p-6 text-start">
    <DialogHeader>
      <DialogTitle class="flex items-center gap-2">
        <AdjustmentsHorizontal size={18} class="text-primary" />
        <span>{m.academics_schedule_export_dialog_title()}</span>
      </DialogTitle>
    </DialogHeader>

    <div class="mt-4 flex flex-col gap-4">
      <!-- Target & Format Selection Controls -->
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-3 p-3 rounded-xl border border-border/60 bg-muted/20">
        <div class="space-y-1.5">
          <Label class="text-xs font-semibold">{m.academics_schedule_export_type()}</Label>
          <Select type="single" value={exportType} onValueChange={(val: any) => (exportType = val)}>
            <SelectTrigger class="h-9 text-xs">
              <span>
                {exportType === "class"
                  ? m.academics_schedule_type_class()
                  : exportType === "teacher"
                    ? m.academics_schedule_type_teacher()
                    : exportType === "room"
                      ? m.academics_schedule_type_room()
                      : m.academics_schedule_type_master()}
              </span>
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="class" class="text-xs">{m.academics_schedule_type_class()}</SelectItem>
              <SelectItem value="teacher" class="text-xs">{m.academics_schedule_type_teacher()}</SelectItem>
              <SelectItem value="room" class="text-xs">{m.academics_schedule_type_room()}</SelectItem>
              <SelectItem value="master" class="text-xs">{m.academics_schedule_type_master()}</SelectItem>
            </SelectContent>
          </Select>
        </div>

        {#if exportType === "class"}
          <div class="space-y-1.5">
            <Label class="text-xs font-semibold">{m.academics_schedule_target_class()}</Label>
            <Select type="single" value={exportTargetId} onValueChange={(val) => (exportTargetId = val ?? "")}>
              <SelectTrigger class="h-9 text-xs">
                <span>{classes.find((c) => String(c.id) === exportTargetId) ? l(classes.find((c) => String(c.id) === exportTargetId).name_ar, classes.find((c) => String(c.id) === exportTargetId).name) : m.common_lesson_select_class()}</span>
              </SelectTrigger>
              <SelectContent class="max-h-56">
                {#each classes as c (c.id)}
                  <SelectItem value={String(c.id)} class="text-xs">{l(c.name_ar, c.name)}</SelectItem>
                {/each}
              </SelectContent>
            </Select>
          </div>
        {:else if exportType === "teacher"}
          <div class="space-y-1.5">
            <Label class="text-xs font-semibold">{m.academics_schedule_target_teacher()}</Label>
            <Select type="single" value={exportTargetId} onValueChange={(val) => (exportTargetId = val ?? "")}>
              <SelectTrigger class="h-9 text-xs">
                <span>{teachers.find((t) => String(t.id) === exportTargetId) ? l(teachers.find((t) => String(t.id) === exportTargetId).name_ar, teachers.find((t) => String(t.id) === exportTargetId).full_name) : m.common_lesson_select_teacher()}</span>
              </SelectTrigger>
              <SelectContent class="max-h-56">
                {#each teachers as t (t.id)}
                  <SelectItem value={String(t.id)} class="text-xs">{l(t.name_ar, t.full_name)}</SelectItem>
                {/each}
              </SelectContent>
            </Select>
          </div>
        {:else if exportType === "room"}
          <div class="space-y-1.5">
            <Label class="text-xs font-semibold">{m.academics_schedule_target_room()}</Label>
            <Select type="single" value={exportTargetId} onValueChange={(val) => (exportTargetId = val ?? "")}>
              <SelectTrigger class="h-9 text-xs">
                <span>{rooms.find((r) => String(r.id) === exportTargetId) ? l(rooms.find((r) => String(r.id) === exportTargetId).name_ar, rooms.find((r) => String(r.id) === exportTargetId).name) : m.academics_studio_select_room()}</span>
              </SelectTrigger>
              <SelectContent class="max-h-56">
                {#each rooms as r (r.id)}
                  <SelectItem value={String(r.id)} class="text-xs">{l(r.name_ar, r.name)}</SelectItem>
                {/each}
              </SelectContent>
            </Select>
          </div>
        {/if}
      </div>

      <!-- Printable Sheet Preview Canvas -->
      <div class="border border-border/80 rounded-xl bg-card p-4 shadow-sm overflow-x-auto text-foreground" id="printable-timetable">
        <!-- Header -->
        <div class="flex items-center justify-between border-b border-border/60 pb-3 mb-3">
          <div>
            <h3 class="text-base font-bold text-foreground">{exportTitle}</h3>
            <p class="text-xs text-muted-foreground">{periodName}</p>
          </div>
          <div class="text-end">
            <span class="text-xs font-semibold text-primary">ORBIS SCHEDULING</span>
            <p class="text-[10px] text-muted-foreground">{formatDate(new Date())}</p>
          </div>
        </div>

        <!-- Matrix Grid for Print -->
        <table class="w-full border-collapse text-xs">
          <thead>
            <tr class="bg-muted/40 border-b border-border">
              <th class="p-2 border-e border-border text-start w-24 font-bold">{m.academics_schedule_period_column()}</th>
              {#each weekdays as day}
                <th class="p-2 border-e border-border text-start last:border-e-0 font-bold">{day.label}</th>
              {/each}
            </tr>
          </thead>
          <tbody>
            {#each timeSlots as slot}
              {@const isBreak = Boolean(slot.is_break)}
              <tr class="border-b border-border/60 {isBreak ? 'bg-amber-500/5' : ''}">
                <td class="p-2 border-e border-border font-medium bg-muted/10">
                  <div class="font-bold text-[11px]">{l(slot.name_ar, slot.name) || `P${slot.slot_number}`}</div>
                  <div class="text-[9px] text-muted-foreground font-mono">{slot.start_time?.slice(0, 5)} - {slot.end_time?.slice(0, 5)}</div>
                </td>
                {#if isBreak}
                  <td colspan={weekdays.length} class="p-2 text-center text-xs font-bold text-amber-600 dark:text-amber-400">
                    {l(slot.name_ar, slot.name) || m.common_onboarding_break_recess()}
                  </td>
                {:else}
                  {#each weekdays as day}
                    {@const dayValue = day.value ?? day.day}
                    {@const l = getLessonInSlot(dayValue, String(slot.id))}
                    <td class="p-2 border-e border-border last:border-e-0 align-top">
                      {#if l}
                        <div class="font-bold text-[11px] text-foreground">{getSubjectName(l.subject_id)}</div>
                        {#if exportType !== "teacher" && l.teacher_id}
                          <div class="text-[10px] text-muted-foreground">{getTeacherName(l.teacher_id)}</div>
                        {/if}
                        {#if exportType !== "class" && l.class_id}
                          <div class="text-[10px] text-muted-foreground">{getClassName(l.class_id)}</div>
                        {/if}
                        {#if exportType !== "room" && l.room_id}
                          <div class="text-[9px] text-primary font-medium">{getRoomName(l.room_id)}</div>
                        {/if}
                      {:else}
                        <span class="text-muted-foreground/30 text-[10px]">-</span>
                      {/if}
                    </td>
                  {/each}
                {/if}
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </div>

    <DialogFooter class="mt-6 flex items-center justify-between gap-2 border-t border-border/40 pt-4">
      <Button variant="outline" size="sm" class="text-xs" onclick={() => (open = false)}>
        {m.common_action_cancel()}
      </Button>

      <Button size="sm" class="gap-1.5 text-xs font-bold shadow-xs" onclick={handlePrint}>
        <AdjustmentsHorizontal size={14} />
        <span>{m.academics_schedule_print_now()}</span>
      </Button>
    </DialogFooter>
  </DialogContent>
</Dialog>

<style>
  @media print {
    :global(body *) {
      visibility: hidden;
    }
    :global(#printable-timetable),
    :global(#printable-timetable *) {
      visibility: visible;
    }
    :global(#printable-timetable) {
      position: absolute;
      inset-inline-start: 0;
      top: 0;
      width: 100%;
    }
  }
</style>
