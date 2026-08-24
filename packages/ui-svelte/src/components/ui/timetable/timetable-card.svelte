<script lang="ts">
import { fade } from "svelte/transition";
import { Pencil, Trash2 } from "@lucide/svelte";

let {
  lesson,
  viewMode = "class",
  locale = "ar",
  getSubjectName,
  getTeacherName,
  getClassName,
  subjectColor = "#a855f7",
  ondragstart,
  onDelete,
  onEdit,
}: {
  lesson: any;
  viewMode: "whole" | "class" | "teacher" | "subject" | "room";
  locale: string;
  getSubjectName: (id: string) => string;
  getTeacherName: (id: string) => string;
  getClassName: (id: string) => string;
  subjectColor?: string;
  ondragstart?: (e: DragEvent) => void;
  onDelete?: (id: string) => void;
  onEdit?: (lesson: any) => void;
} = $props();

const isDraft = $derived(!!lesson.isDraft);
const subjectName = $derived(getSubjectName(lesson.subject_id));

// Format times nicely
const timeStr = $derived(
  lesson.time_slot_start && lesson.time_slot_end
    ? `${lesson.time_slot_start} - ${lesson.time_slot_end}`
    : "",
);
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  draggable={!isDraft}
  {ondragstart}
  dir={locale === "ar" ? "rtl" : "ltr"}
  style="--subject-color: {subjectColor};"
  class="relative overflow-hidden w-full h-full min-h-20 p-2 flex flex-col items-stretch justify-center gap-1 select-none transition-all duration-150 rounded-md border-s-4
    {isDraft 
      ? 'bg-muted/40 border-muted-foreground/30 border-dashed opacity-80 text-muted-foreground animate-pulse-draft' 
      : 'bg-card border-border text-foreground cursor-grab active:cursor-grabbing hover:bg-accent/5'}"
  style:border-inline-start-color="{isDraft ? 'var(--color-muted-foreground)' : 'var(--subject-color)'}"
  style:background-color={!isDraft ? `color-mix(in srgb, ${subjectColor} 14%, var(--color-card))` : undefined}
  in:fade={{ duration: 150 }}
>
  {#if onDelete && !isDraft}
    <button
      type="button"
      onclick={(e) => { e.stopPropagation(); onDelete(lesson.id); }}
      class="absolute inset-e-1 top-1 text-muted-foreground/50 hover:text-destructive p-0.5 rounded transition-colors cursor-pointer"
      title={locale === "ar" ? "حذف الحصة" : "Delete lesson"}
    >
      <Trash2 size={11} />
    </button>
  {/if}
  {#if onEdit && !isDraft}
    <button
      type="button"
      onclick={(e) => { e.stopPropagation(); onEdit(lesson); }}
      class="absolute inset-e-6 top-1 text-muted-foreground/50 hover:text-primary p-0.5 rounded transition-colors cursor-pointer"
      title={locale === "ar" ? "تعديل الحصة" : "Edit lesson"}
    >
      <Pencil size={11} />
    </button>
  {/if}

  <span class="font-bold text-[10px] leading-tight text-start line-clamp-2 pe-3">
    {subjectName}
  </span>
  <span class="text-[8px] leading-tight text-start text-muted-foreground line-clamp-1">
    {viewMode === "class" ? getTeacherName(lesson.teacher_id) : getClassName(lesson.class_id)}
    {#if lesson.room_name || lesson.room}
      · {lesson.room_name || lesson.room}
    {/if}
  </span>
  {#if timeStr}
    <span dir="ltr" class="text-[8px] font-mono text-start text-muted-foreground/80 leading-none tabular-nums">
      {timeStr}
    </span>
  {/if}

</div>

<style>
  .animate-pulse-draft {
    animation: draft-pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
  }

  @keyframes draft-pulse {
    0%, 100% { opacity: 0.8; }
    50% { opacity: 0.4; }
  }
</style>
