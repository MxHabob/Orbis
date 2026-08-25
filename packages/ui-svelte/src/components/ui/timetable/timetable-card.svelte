<script lang="ts">
import { fade } from "svelte/transition";
import { Pencil, Trash2, Users, MapPin } from "@lucide/svelte";

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
  ondragstart?: (e: DragEvent) => void;
  onDelete?: (id: string) => void;
  onEdit?: (lesson: any) => void;
} = $props();

const isDraft = $derived(!!lesson.isDraft);
const subjectName = $derived(getSubjectName(lesson.subject_id));
const hasConflict = $derived(Boolean(lesson.conflict || lesson.conflict_type || lesson.has_conflict));
const statusLabel = $derived(lesson.status || lesson.lesson_status || "");
const roomName = $derived(
  lesson.room_name || getRoomName?.(lesson.room_id) || lesson.room || "",
);

// Format times nicely
const timeStr = $derived(
  lesson.time_slot_start && lesson.time_slot_end
    ? `${lesson.time_slot_start} - ${lesson.time_slot_end}`
    : timeSlot?.start_time && timeSlot?.end_time
      ? `${timeSlot.start_time} - ${timeSlot.end_time}`
    : "",
);
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  draggable={!isDraft}
  {ondragstart}
  dir="auto"
  style="--subject-color: {subjectColor};"
  class="lesson-card relative self-center overflow-hidden w-full h-full min-h-20 flex flex-col items-stretch select-none transition-all duration-150 rounded-lg
    {isDraft 
      ? 'bg-muted/40 text-muted-foreground animate-pulse-draft' 
      : hasConflict
        ? 'bg-destructive/10 text-foreground cursor-grab active:cursor-grabbing'
        : 'text-foreground cursor-grab active:cursor-grabbing'}"
  in:fade={{ duration: 150 }}
>
  <div
    class="lesson-card-body relative flex min-h-0 flex-1 flex-col justify-start gap-1 p-2 text-start"
    style:background-color={!isDraft && !hasConflict ? `color-mix(in srgb, ${subjectColor} 72%, var(--color-card))` : undefined}
  >
    {#if onDelete && !isDraft}
      <button
        type="button"
        onclick={(e) => { e.stopPropagation(); onDelete(lesson.id); }}
        class="absolute inset-s-1 top-1 text-foreground/60 hover:text-destructive p-0.5 rounded transition-colors cursor-pointer"
        title={locale === "ar" ? "حذف الحصة" : "Delete lesson"}
      >
        <Trash2 size={11} />
      </button>
    {/if}
    {#if onEdit && !isDraft}
      <button
        type="button"
        onclick={(e) => { e.stopPropagation(); onEdit(lesson); }}
        class="absolute inset-s-6 top-1 text-foreground/60 hover:text-primary p-0.5 rounded transition-colors cursor-pointer"
        title={locale === "ar" ? "تعديل الحصة" : "Edit lesson"}
      >
        <Pencil size={11} />
      </button>
    {/if}

    <span class="lesson-subject ps-7 font-bold text-[11px] leading-tight text-end line-clamp-2">
      {subjectName}
    </span>
    {#if hasConflict || statusLabel}
      <span class="lesson-status text-[8px] font-semibold leading-none {hasConflict ? 'text-destructive' : 'text-foreground/70'}">
        {hasConflict ? "Conflict" : statusLabel}
      </span>
    {/if}
    <div class="lesson-meta flex min-w-0 flex-col gap-0.5 text-[9px] leading-tight text-foreground/75">
      <span class="flex min-w-0 items-center gap-1 truncate">
        <Users size={9} class="shrink-0 opacity-70" />
        <span class="truncate">{viewMode === "class" ? getTeacherName(lesson.teacher_id) : getClassName(lesson.class_id)}</span>
      </span>
      {#if roomName}
        <span class="lesson-room flex min-w-0 items-center gap-1 truncate">
          <MapPin size={9} class="shrink-0 opacity-70" />
          <span class="truncate">{roomName}</span>
        </span>
      {/if}
    </div>
  </div>
  {#if timeStr}
    <div
      class="lesson-time shrink-0 px-2 py-1.5 text-center text-[9px] font-mono font-bold leading-none text-foreground/80 tabular-nums"
      style:background-color={!isDraft && !hasConflict ? `color-mix(in srgb, ${subjectColor} 18%, var(--color-card))` : undefined}
      dir="ltr"
    >
      {timeStr}
    </div>
  {/if}

</div>

<style>
  .lesson-card {
    container-type: inline-size;
    max-width: none;
  }

  @container (max-width: 112px) {
    .lesson-card-body {
      padding: 0.35rem;
      gap: 0.25rem;
    }

    .lesson-meta {
      font-size: 7px;
    }

    .lesson-room {
      display: none;
    }

    .lesson-time {
      padding-inline: 0.25rem;
      font-size: 7px;
    }
  }

  @container (min-width: 180px) {
    .lesson-card-body {
      padding: 0.65rem;
      gap: 0.35rem;
    }

    .lesson-meta {
      font-size: 9px;
    }

    .lesson-time {
      padding-block: 0.35rem;
      font-size: 9px;
    }
  }

  .animate-pulse-draft {
    animation: draft-pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
  }

  @keyframes draft-pulse {
    0%, 100% { opacity: 0.8; }
    50% { opacity: 0.4; }
  }
</style>
