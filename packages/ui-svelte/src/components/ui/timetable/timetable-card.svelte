<script lang="ts">
  import { fade } from "svelte/transition";
  import { GripVertical, MapPin, Pencil, Trash2, Users } from "@lucide/svelte";
  import { Button } from "@orbis/ui-svelte/button";

  let {
    lesson,
    timeSlot,
    viewMode = "class",
    locale = "ar",
    getSubjectName,
    getTeacherName,
    getClassName,
    getRoomName,
    subjectColor = "var(--color-primary)",
    compact = false,
    ondragstart,
    onDelete,
    onEdit,
  }: {
    lesson: any;
    timeSlot?: any;
    viewMode: "whole" | "class" | "teacher" | "subject" | "room";
    locale: string;
    getSubjectName: (id: string) => string;
    getTeacherName: (id: string) => string;
    getClassName: (id: string) => string;
    getRoomName?: (id: string) => string;
    subjectColor?: string;
    compact?: boolean;
    ondragstart?: (event: DragEvent) => void;
    onDelete?: (id: string) => void;
    onEdit?: (lesson: any) => void;
  } = $props();

  const isDraft = $derived(Boolean(lesson.isDraft));
  const hasConflict = $derived(Boolean(lesson.conflict || lesson.conflict_type || lesson.has_conflict));
  const statusLabel = $derived(lesson.status || lesson.lesson_status || "");
  const subjectName = $derived(lesson.subject_name || lesson.subjectName || getSubjectName(lesson.subject_id) || "");
  const teacherName = $derived(lesson.teacher_name || lesson.teacherName || getTeacherName(lesson.teacher_id) || "");
  const className = $derived(lesson.class_name || lesson.className || getClassName(lesson.class_id) || "");
  const roomName = $derived(lesson.room_name || lesson.roomName || getRoomName?.(lesson.room_id) || lesson.room || "");

  function formatTime(value: string): string {
    if (!value) return "";
    const match = value.match(/^(\d{1,2})(?::(\d{2}))?/);
    if (!match) return value;
    let hour = Number(match[1]);
    const minute = match[2];
    const period = hour >= 12 ? "PM" : "AM";
    hour = hour % 12 || 12;
    return minute && minute !== "00" ? `${hour}:${minute} ${period}` : `${hour} ${period}`;
  }

  const timeLabel = $derived.by(() => {
    const start = lesson.time_slot_start || timeSlot?.start_time || "";
    const end = lesson.time_slot_end || timeSlot?.end_time || "";
    return start && end ? `${formatTime(start)} - ${formatTime(end)}` : "";
  });

  function preventCardDrag(event: DragEvent) {
    event.preventDefault();
  }

  function startLessonDrag(event: DragEvent) {
    event.stopPropagation();
    if (event.dataTransfer) event.dataTransfer.effectAllowed = "move";
    ondragstart?.(event);
  }

  function handleEdit(event: MouseEvent) {
    event.stopPropagation();
    onEdit?.(lesson);
  }

  function handleDelete(event: MouseEvent) {
    event.stopPropagation();
    onDelete?.(lesson.id);
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<article
  dir="auto"
  ondragstart={preventCardDrag}
  style={`--subject-color: ${subjectColor};`}
  class="group relative flex h-full min-w-0 w-full flex-col overflow-hidden rounded-md border border-primary-foreground/20 bg-(--subject-color) text-primary-foreground select-none {compact ? 'min-h-0' : 'min-h-20'} {hasConflict ? 'border-destructive' : ''} {isDraft ? 'border-dashed' : ''}"
  in:fade={{ duration: 120 }}
  aria-label={subjectName}
>
  <div class="relative flex min-h-0 flex-1 flex-col px-2 pb-1.5 pt-2 text-start {compact ? 'overflow-visible' : 'overflow-hidden'}">
    {#if !isDraft}
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <span
        draggable="true"
        ondragstart={startLessonDrag}
        class="absolute inset-e-1 top-1 z-20 flex h-5 w-5 cursor-grab items-center justify-center rounded-sm text-primary-foreground/75 active:cursor-grabbing"
        role="button"
        tabindex="0"
        aria-label={locale === "ar" ? "سحب الحصة" : "Drag lesson"}
        title={locale === "ar" ? "اسحب من هنا لنقل الحصة" : "Drag here to move lesson"}
      >
        <GripVertical size={13} strokeWidth={2.5} />
      </span>
    {/if}

    <div class="min-w-0 truncate pe-5 text-start text-[11px] font-bold leading-tight" title={subjectName}>
      {subjectName}
    </div>

    <div class="mt-1 flex min-h-0 flex-1 flex-col gap-0.5 text-[9px] leading-tight text-primary-foreground/80 {compact ? 'overflow-visible' : 'overflow-hidden'}">
      {#if viewMode === "class" && teacherName}
        <span class="flex min-w-0 items-center gap-1" title={teacherName}>
          <Users size={9} strokeWidth={2} class="shrink-0 text-primary-foreground/70" />
          <span class="truncate">{teacherName}</span>
        </span>
      {:else if className}
        <span class="flex min-w-0 items-center gap-1" title={className}>
          <Users size={9} strokeWidth={2} class="shrink-0 text-primary-foreground/70" />
          <span class="truncate">{className}</span>
        </span>
      {/if}

      {#if roomName}
        <span class="flex min-w-0 items-center gap-1" title={roomName}>
          <MapPin size={9} strokeWidth={2} class="shrink-0 text-primary-foreground/70" />
          <span class="truncate">{roomName}</span>
        </span>
      {/if}

      {#if hasConflict || statusLabel}
        <span class="truncate text-[8px] text-primary-foreground">
          {hasConflict ? (locale === "ar" ? "تعارض" : "Conflict") : statusLabel}
        </span>
      {/if}
    </div>

    {#if (onDelete || onEdit) && !isDraft}
      <div class="absolute inset-s-1 bottom-1 z-30 flex items-center gap-1 opacity-0 pointer-events-none transition-opacity group-hover:pointer-events-auto group-hover:opacity-100 focus-within:pointer-events-auto focus-within:opacity-100" dir="rtl">
        {#if onEdit}
          <Button variant="secondary" size="icon-xs" onclick={handleEdit} title={locale === "ar" ? "تعديل الحصة" : "Edit lesson"}>
            <Pencil size={9} strokeWidth={2.5}  />
          </Button>
        {/if}
        {#if onDelete}
          <Button variant="destructive" size="icon-xs" onclick={handleDelete} title={locale === "ar" ? "حذف الحصة" : "Delete lesson"}>
            <Trash2 size={9} />
          </Button>
        {/if}
      </div>
    {/if}
  </div>

  {#if timeLabel}
    <div class="flex {compact ? 'h-6 min-h-6' : 'h-7 min-h-7'} items-center justify-center overflow-hidden border-t border-primary-foreground/20 bg-primary-foreground/15 px-1.5 text-[10px] font-bold leading-none text-primary-foreground whitespace-nowrap" dir="ltr" aria-label={timeLabel}>
      <span class="max-w-full truncate">{timeLabel}</span>
    </div>
  {/if}
</article>
