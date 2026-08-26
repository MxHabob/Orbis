<script lang="ts">
  import {
    Dialog,
    DialogContent,
    DialogHeader,
    DialogTitle,
    DialogFooter,
  } from "@orbis/ui-svelte/dialog";
  import { Button } from "@orbis/ui-svelte/button";
  import { Label } from "@orbis/ui-svelte/label";
  import {
    Select,
    SelectTrigger,
    SelectContent,
    SelectItem,
  } from "@orbis/ui-svelte/select";
  import { Printer, FileText, Download, Users, User, MapPin, Layers } from "@lucide/svelte";
  import { getLocale } from "$lib/paraglide/runtime";

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

  const isArabic = $derived(getLocale() === "ar");

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
      return cls ? (isArabic ? cls.name_ar || cls.name : cls.name || cls.name_ar) : (isArabic ? "جدول الفصل" : "Class Schedule");
    }
    if (exportType === "teacher") {
      const teach = teachers.find((t) => String(t.id) === exportTargetId);
      return teach ? (isArabic ? teach.name_ar || teach.full_name : teach.full_name) : (isArabic ? "جدول المعلم" : "Teacher Schedule");
    }
    if (exportType === "room") {
      const rm = rooms.find((r) => String(r.id) === exportTargetId);
      return rm ? (isArabic ? rm.name_ar || rm.name : rm.name || rm.name_ar) : (isArabic ? "جدول القاعة" : "Room Schedule");
    }
    return isArabic ? "الجدول المدرسي العام" : "Master Schedule";
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
      (l) => Number(l.day_of_week) === day && String(l.time_slot_id) === slotId
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
        <Printer size={18} class="text-primary" />
        <span>{isArabic ? "تصدير وطباعة الجدول الدراسي" : "Export & Print Timetable"}</span>
      </DialogTitle>
    </DialogHeader>

    <div class="mt-4 flex flex-col gap-4">
      <!-- Target & Format Selection Controls -->
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-3 p-3 rounded-xl border border-border/60 bg-muted/20">
        <div class="space-y-1.5">
          <Label class="text-xs font-semibold">{isArabic ? "نوع الجدول للطباعة" : "Schedule Type"}</Label>
          <Select type="single" value={exportType} onValueChange={(val: any) => (exportType = val)}>
            <SelectTrigger class="h-9 text-xs">
              <span>
                {exportType === "class"
                  ? (isArabic ? "جدول فصل دراسي" : "Class Timetable")
                  : exportType === "teacher"
                    ? (isArabic ? "جدول معلم" : "Teacher Schedule")
                    : exportType === "room"
                      ? (isArabic ? "جدول قاعة دراسية" : "Room Schedule")
                      : (isArabic ? "الجدول العام للمدرسة" : "Master Schedule")}
              </span>
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="class" class="text-xs">{isArabic ? "جدول فصل دراسي" : "Class Timetable"}</SelectItem>
              <SelectItem value="teacher" class="text-xs">{isArabic ? "جدول معلم" : "Teacher Schedule"}</SelectItem>
              <SelectItem value="room" class="text-xs">{isArabic ? "جدول قاعة دراسية" : "Room Schedule"}</SelectItem>
              <SelectItem value="master" class="text-xs">{isArabic ? "الجدول العام للمدرسة" : "Master Schedule"}</SelectItem>
            </SelectContent>
          </Select>
        </div>

        {#if exportType === "class"}
          <div class="space-y-1.5">
            <Label class="text-xs font-semibold">{isArabic ? "حدد الفصل" : "Target Class"}</Label>
            <Select type="single" value={exportTargetId} onValueChange={(val) => (exportTargetId = val ?? "")}>
              <SelectTrigger class="h-9 text-xs">
                <span>{classes.find((c) => String(c.id) === exportTargetId)?.name_ar || classes.find((c) => String(c.id) === exportTargetId)?.name || (isArabic ? "اختر الفصل..." : "Select class...")}</span>
              </SelectTrigger>
              <SelectContent class="max-h-56">
                {#each classes as c (c.id)}
                  <SelectItem value={String(c.id)} class="text-xs">{c.name_ar || c.name}</SelectItem>
                {/each}
              </SelectContent>
            </Select>
          </div>
        {:else if exportType === "teacher"}
          <div class="space-y-1.5">
            <Label class="text-xs font-semibold">{isArabic ? "حدد المعلم" : "Target Teacher"}</Label>
            <Select type="single" value={exportTargetId} onValueChange={(val) => (exportTargetId = val ?? "")}>
              <SelectTrigger class="h-9 text-xs">
                <span>{teachers.find((t) => String(t.id) === exportTargetId)?.name_ar || teachers.find((t) => String(t.id) === exportTargetId)?.full_name || (isArabic ? "اختر المعلم..." : "Select teacher...")}</span>
              </SelectTrigger>
              <SelectContent class="max-h-56">
                {#each teachers as t (t.id)}
                  <SelectItem value={String(t.id)} class="text-xs">{t.name_ar || t.full_name}</SelectItem>
                {/each}
              </SelectContent>
            </Select>
          </div>
        {:else if exportType === "room"}
          <div class="space-y-1.5">
            <Label class="text-xs font-semibold">{isArabic ? "حدد القاعة" : "Target Room"}</Label>
            <Select type="single" value={exportTargetId} onValueChange={(val) => (exportTargetId = val ?? "")}>
              <SelectTrigger class="h-9 text-xs">
                <span>{rooms.find((r) => String(r.id) === exportTargetId)?.name_ar || rooms.find((r) => String(r.id) === exportTargetId)?.name || (isArabic ? "اختر القاعة..." : "Select room...")}</span>
              </SelectTrigger>
              <SelectContent class="max-h-56">
                {#each rooms as r (r.id)}
                  <SelectItem value={String(r.id)} class="text-xs">{r.name_ar || r.name}</SelectItem>
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
            <p class="text-[10px] text-muted-foreground">{new Date().toLocaleDateString()}</p>
          </div>
        </div>

        <!-- Matrix Table for Print -->
        <table class="w-full border-collapse text-xs">
          <thead>
            <tr class="bg-muted/40 border-b border-border">
              <th class="p-2 border-e border-border text-start w-24 font-bold">{isArabic ? "الحصة" : "Period"}</th>
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
                  <div class="font-bold text-[11px]">{slot.name_ar || slot.name || `P${slot.slot_number}`}</div>
                  <div class="text-[9px] text-muted-foreground font-mono" dir="ltr">{slot.start_time?.slice(0, 5)} - {slot.end_time?.slice(0, 5)}</div>
                </td>
                {#if isBreak}
                  <td colspan={weekdays.length} class="p-2 text-center text-xs font-bold text-amber-600 dark:text-amber-400">
                    {slot.name_ar || slot.name || (isArabic ? "استراحة" : "Break")}
                  </td>
                {:else}
                  {#each weekdays as day}
                    {@const l = getLessonInSlot(day.value, String(slot.id))}
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
        {isArabic ? "إلغاء" : "Cancel"}
      </Button>

      <Button size="sm" class="gap-1.5 text-xs font-bold shadow-xs" onclick={handlePrint}>
        <Printer size={14} />
        <span>{isArabic ? "طباعة الجدول الآن" : "Print Schedule"}</span>
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
