<script lang="ts">
import { fade } from "svelte/transition";
import { Plus, Clock, Pencil, Trash2, Save, X } from "@lucide/svelte";
import TimetableCard from "./timetable-card.svelte";

let {
  timeSlots = [],
  weekdays = [],
  lessons = [],
  draftLessons = [],
  previewDraft = false,
  viewMode = "class",
  locale = "ar",
  getSubjectName,
  getTeacherName,
  getClassName,
  getSubjectColor,
  class: className = "",
  onCellClick,
  onDeleteLesson,
  onEditLesson,
  onMoveLesson,
  onAddTimeSlot,
  onEditTimeSlot,
  onDeleteTimeSlot,
  onSaveTimeSlot,
}: {
  timeSlots: any[];
  weekdays: any[];
  lessons: any[];
  draftLessons: any[];
  previewDraft: boolean;
  viewMode: "whole" | "class" | "teacher" | "subject" | "room";
  locale: string;
  getSubjectName: (id: string) => string;
  getTeacherName: (id: string) => string;
  getClassName: (id: string) => string;
  getSubjectColor: (id: string) => string;
  class?: string;
  onCellClick: (day: number, slotId: string) => void;
  onDeleteLesson: (id: string) => void;
  onEditLesson?: (lesson: any) => void;
  onMoveLesson: (lesson: any, targetDay: number, targetSlotId: string) => void;
  onAddTimeSlot?: () => void;
  onEditTimeSlot?: (slot: any) => void;
  onDeleteTimeSlot?: (slot: any) => void;
  onSaveTimeSlot?: (slot: any | null, data: any) => void;
} = $props();

let editingSlotId = $state<string | null>(null);
let isAddingSlot = $state(false);
let draftNameAr = $state("");
let draftName = $state("");
let draftStartTime = $state("");
let draftEndTime = $state("");

function startSlotEdit(slot: any) {
  editingSlotId = String(slot.id);
  isAddingSlot = false;
  draftNameAr = slot.name_ar || "";
  draftName = slot.name || "";
  draftStartTime = String(slot.start_time || "").slice(0, 5);
  draftEndTime = String(slot.end_time || "").slice(0, 5);
}

function startSlotAdd() {
  editingSlotId = null;
  isAddingSlot = true;
  const nextNumber = timeSlots.reduce((highest, slot) => Math.max(highest, Number(slot.slot_number) || 0), 0) + 1;
  draftNameAr = `الحصة ${nextNumber}`;
  draftName = `Period ${nextNumber}`;
  draftStartTime = "";
  draftEndTime = "";
}

function cancelSlotEdit() {
  editingSlotId = null;
  isAddingSlot = false;
}

function saveSlotEdit() {
  if (!draftNameAr.trim() || !draftStartTime || !draftEndTime || !onSaveTimeSlot) return;
  onSaveTimeSlot(
    editingSlotId ? timeSlots.find((slot) => String(slot.id) === editingSlotId) : null,
    {
      name: draftName.trim() || draftNameAr.trim(),
      name_ar: draftNameAr.trim(),
      start_time: draftStartTime,
      end_time: draftEndTime,
      duration_minutes: (() => {
        const [startHour, startMinute] = draftStartTime.split(":").map(Number);
        const [endHour, endMinute] = draftEndTime.split(":").map(Number);
        return endHour * 60 + endMinute - (startHour * 60 + startMinute);
      })(),
    },
  );
  cancelSlotEdit();
}

// Retrieve lessons (real or draft) inside a grid cell
function getLessonsInCell(day: number, slotId: string) {
  if (previewDraft) {
    const drafts = draftLessons.filter(
      (l: any) =>
        Number(l.day_of_week) === day &&
        String(l.time_slot_id) === String(slotId),
    );
    if (drafts.length > 0) return drafts;
  }
  return lessons.filter(
    (l: any) =>
      Number(l.day_of_week) === day &&
      String(l.time_slot_id) === String(slotId),
  );
}

function isSlotAvailable(slot: any, day: number): boolean {
  if (!slot.applicable_days) return true;
  const days = String(slot.applicable_days)
    .split(",")
    .map((value) => Number(value.trim()))
    .filter((value) => Number.isInteger(value));
  return days.length === 0 || days.includes(day);
}

function timeToMinutes(value: unknown): number {
  const match = String(value ?? "").match(/^(\d{1,2}):(\d{2})/);
  if (!match) return Number.MAX_SAFE_INTEGER;
  return Number(match[1]) * 60 + Number(match[2]);
}

function getChronologicalTime(value: unknown): number {
  const minutes = timeToMinutes(value);
  if (minutes !== Number.MAX_SAFE_INTEGER) return minutes;
  return Number.MAX_SAFE_INTEGER;
}

const orderedTimeSlots = $derived(
  [...timeSlots].sort((first, second) => {
    const startDifference = getChronologicalTime(first.start_time) - getChronologicalTime(second.start_time);
    if (startDifference !== 0) return startDifference;
    return getChronologicalTime(first.end_time) - getChronologicalTime(second.end_time);
  }),
);

// Drag and Drop handlers
let draggedLesson = $state<any>(null);
let dragOverCell = $state<{ day: number; slotId: string } | null>(null);

function handleDragStart(e: DragEvent, lesson: any) {
  if (lesson.isDraft) {
    e.preventDefault();
    return;
  }
  draggedLesson = lesson;
  if (e.dataTransfer) {
    e.dataTransfer.effectAllowed = "move";
    e.dataTransfer.setData("text/plain", lesson.id);
  }
}

function handleDragOver(e: DragEvent, day: number, slotId: string) {
  e.preventDefault();
  if (draggedLesson) {
    dragOverCell = { day, slotId };
  }
}

function handleDragLeave() {
  dragOverCell = null;
}

function handleDrop(e: DragEvent, targetDay: number, targetSlotId: string) {
  e.preventDefault();
  dragOverCell = null;
  if (!draggedLesson) return;
  const lesson = draggedLesson;
  draggedLesson = null;

  if (
    Number(lesson.day_of_week) === targetDay &&
    String(lesson.time_slot_id) === String(targetSlotId)
  ) {
    return;
  }

  onMoveLesson(lesson, targetDay, targetSlotId);
}
</script>

<!-- Reusable responsive timetable board -->
<div class="schedule-board flex-1 min-h-0 min-w-0 w-full h-full border-y border-border bg-card flex flex-col overflow-auto select-none relative scrollbar-thin {className}">
  
  <div 
    class="schedule-grid grid gap-px text-xs w-full min-w-180 bg-border flex-1"
    style="grid-template-columns: minmax(84px, 0.72fr) repeat({weekdays.length}, minmax(132px, 1fr));"
    in:fade={{ duration: 150 }}
  >
    <!-- 1. Header Row (Sticky Top) -->
    <div class="bg-muted/40 text-muted-foreground p-2 md:p-2.5 font-semibold text-center flex items-center justify-center gap-1 border-b border-border text-[9px] sticky top-0 inset-s-0 z-30">
      <span>{locale === "ar" ? "الحصة" : "Period"}</span>
      {#if onAddTimeSlot}
          <button type="button" class="rounded p-1 text-muted-foreground hover:bg-primary/10 hover:text-primary" onclick={startSlotAdd} aria-label={locale === "ar" ? "إضافة حصة" : "Add period"} title={locale === "ar" ? "إضافة حصة" : "Add period"}>
          <Plus size={13} />
        </button>
      {/if}
    </div>
    {#each weekdays as day (day.value)}
      <div class="bg-muted/40 text-foreground p-2 md:p-2.5 font-semibold text-center flex items-center justify-center border-b border-border text-[10px] sticky top-0 z-20">
        {day.label}
      </div>
    {/each}

    <!-- 2. Rows (Periods & Breaks) -->
    {#if isAddingSlot}
      <div class="bg-muted/30 border-border p-2 flex flex-col gap-1 justify-center border-b sticky inset-s-0 z-10">
        <input class="h-7 w-full rounded border border-border bg-background px-1 text-[10px]" bind:value={draftNameAr} placeholder="اسم الحصة" aria-label="اسم الحصة" />
        <div class="flex gap-1">
          <input class="h-6 min-w-0 w-1/2 rounded border border-border bg-background px-1 text-[9px]" bind:value={draftStartTime} type="time" aria-label="وقت البداية" />
          <input class="h-6 min-w-0 w-1/2 rounded border border-border bg-background px-1 text-[9px]" bind:value={draftEndTime} type="time" aria-label="وقت النهاية" />
        </div>
        <div class="flex justify-center gap-1">
          <button type="button" class="rounded p-1 text-primary hover:bg-primary/10" onclick={saveSlotEdit} aria-label="حفظ"><Save size={12} /></button>
          <button type="button" class="rounded p-1 text-muted-foreground hover:text-destructive" onclick={cancelSlotEdit} aria-label="إلغاء"><X size={12} /></button>
        </div>
      </div>
      <div class="bg-muted/20 border-b border-border" style="grid-column: span {weekdays.length};"></div>
    {/if}

    {#each orderedTimeSlots as slot (slot.id)}
      {@const isBreak = slot.is_break === 1 || slot.is_break === true}

      <!-- Row Period Info Column (Sticky Start for Horizontal Scroll) -->
      <div class="bg-muted/25 border-border p-1.5 font-semibold text-center flex flex-col justify-center items-center gap-1 {isBreak ? 'min-h-10' : 'min-h-24'} h-full border-b sticky inset-s-0 z-10">
        {#if editingSlotId === String(slot.id)}
          <input class="h-7 w-full rounded border border-border bg-background px-1 text-[10px]" bind:value={draftNameAr} aria-label="اسم الحصة" />
          <div class="flex w-full gap-1">
            <input class="h-6 min-w-0 w-1/2 rounded border border-border bg-background px-1 text-[9px]" bind:value={draftStartTime} type="time" aria-label="وقت البداية" />
            <input class="h-6 min-w-0 w-1/2 rounded border border-border bg-background px-1 text-[9px]" bind:value={draftEndTime} type="time" aria-label="وقت النهاية" />
          </div>
          <span class="flex items-center gap-1">
            <button type="button" class="rounded p-1 text-primary hover:bg-primary/10" onclick={saveSlotEdit} aria-label="حفظ"><Save size={12} /></button>
            <button type="button" class="rounded p-1 text-muted-foreground hover:text-destructive" onclick={cancelSlotEdit} aria-label="إلغاء"><X size={12} /></button>
          </span>
        {:else}
          {#if !isBreak}
            <span class="text-[10px] tracking-wide font-bold text-foreground truncate max-w-31.25">
              {locale === "ar" ? (slot.name_ar || slot.name) : (slot.name || slot.name_ar)}
            </span>
          {/if}
          <span class="text-muted-foreground text-[9px] font-mono font-medium flex items-center gap-1 shrink-0">
            <Clock size={10} class="opacity-70" />
            <span dir="ltr" class="tabular-nums">{slot.start_time} - {slot.end_time}</span>
          </span>
        {/if}
        {#if (onEditTimeSlot || onDeleteTimeSlot) && editingSlotId !== String(slot.id)}
          <span class="flex items-center gap-0.5">
            {#if onEditTimeSlot}
              <button type="button" class="rounded p-1 text-muted-foreground hover:bg-primary/10 hover:text-primary" onclick={() => { startSlotEdit(slot); onEditTimeSlot?.(slot); }} aria-label={locale === "ar" ? "تعديل الفترة" : "Edit period"} title={locale === "ar" ? "تعديل الفترة" : "Edit period"}>
                <Pencil size={11} />
              </button>
            {/if}
            {#if onDeleteTimeSlot}
              <button type="button" class="rounded p-1 text-muted-foreground hover:bg-destructive/10 hover:text-destructive" onclick={() => onDeleteTimeSlot?.(slot)} aria-label={locale === "ar" ? "حذف الفترة" : "Delete period"} title={locale === "ar" ? "حذف الفترة" : "Delete period"}>
                <Trash2 size={11} />
              </button>
            {/if}
          </span>
        {/if}
      </div>

      <!-- Rows Data Columns -->
      {#if isBreak}
        <!-- Break row spans across all weekdays -->
        <div
          class="bg-muted/50 text-muted-foreground border-b border-border font-bold text-[10px] flex items-center justify-center gap-2 min-h-10 h-full"
          style="grid-column: span {weekdays.length};"
        >
          <span>{locale === "ar" ? "استراحة" : "BREAK"}</span>
          <span dir="ltr" class="text-[9px] font-mono font-bold opacity-80 tabular-nums">({slot.start_time} - {slot.end_time})</span>
        </div>
      {:else}
        {#each weekdays as day (day.value)}
          {@const cellLessons = getLessonsInCell(day.value, slot.id)}
          {@const isAvailable = isSlotAvailable(slot, day.value)}
          {@const isDragOver = dragOverCell?.day === day.value && dragOverCell?.slotId === slot.id}
          
          <!-- Cell Container -->
          <!-- svelte-ignore a11y_no_static_element_interactions -->
          <div 
            class="{isAvailable ? 'bg-card hover:bg-muted/30' : 'bg-muted/20 schedule-unavailable'} border-border p-1.5 text-center align-middle min-h-24 h-full relative flex items-center justify-center transition-colors duration-150 border-b
              {isDragOver ? 'bg-primary/8 ring-2 ring-primary/40 ring-inset border-dashed' : ''}"
            ondragover={(e) => handleDragOver(e, day.value, slot.id)}
            ondragleave={handleDragLeave}
            ondrop={(e) => handleDrop(e, day.value, slot.id)}
          >
            {#if !isAvailable}
              <span class="text-[9px] text-muted-foreground/40">-</span>
            {:else}
              {#if cellLessons.length > 0}
                <div class="flex w-full h-full min-h-20 flex-col gap-1">
                  {#each cellLessons as lesson (lesson.id)}
                    <TimetableCard
                      {lesson}
                      {viewMode}
                      {locale}
                      {getSubjectName}
                      {getTeacherName}
                      {getClassName}
                      subjectColor={getSubjectColor(lesson.subject_id)}
                      onDelete={onDeleteLesson}
                      onEdit={onEditLesson}
                      ondragstart={(e) => handleDragStart(e, lesson)}
                    />
                  {/each}
                </div>
              {:else}
                <!-- Empty Cell Add Trigger (Class Mode only) -->
                <button
                  type="button"
                  onclick={() => onCellClick(day.value, slot.id)}
                  class="w-full h-full border border-dashed border-transparent hover:border-primary/30 hover:bg-primary/3 rounded-md flex items-center justify-center text-muted-foreground/0 hover:text-primary transition-all duration-200 cursor-pointer p-2 min-h-17.5"
                  disabled={viewMode !== "class" || !isAvailable}
                  title={locale === "ar" ? "إضافة حصة جديدة" : "Schedule new lesson"}
                >
                  <Plus size={16} class="transition-transform duration-200 hover:scale-110" />
                </button>
              {/if}
            {/if}
          </div>
        {/each}
      {/if}
    {/each}
  </div>
</div>

<style>
  .schedule-grid > :global(*) {
    min-width: 0;
  }

  .schedule-unavailable {
    background-image: repeating-linear-gradient(-45deg, transparent, transparent 7px, color-mix(in srgb, var(--color-muted-foreground) 8%, transparent) 7px, color-mix(in srgb, var(--color-muted-foreground) 8%, transparent) 9px);
  }

  @media (max-width: 640px) {
    .schedule-grid {
      min-width: 680px;
    }
  }
</style>
