<script lang="ts">
import { fade } from "svelte/transition";
import { Trash2 } from "@lucide/svelte";

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
}: {
  lesson: any;
  viewMode: "class" | "teacher";
  locale: string;
  getSubjectName: (id: string) => string;
  getTeacherName: (id: string) => string;
  getClassName: (id: string) => string;
  subjectColor?: string;
  ondragstart?: (e: DragEvent) => void;
  onDelete?: (id: string) => void;
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
  style="--subject-color: {subjectColor};"
  class="relative overflow-hidden w-full h-full p-2.5 flex flex-row items-stretch justify-between gap-2.5 select-none transition-all duration-150 rounded-xl border
    {isDraft 
      ? 'bg-muted/40 border-muted-foreground/30 border-dashed opacity-80 text-muted-foreground animate-pulse-draft' 
      : 'bg-card border-border text-foreground cursor-grab active:cursor-grabbing hover:bg-accent/5 hover:border-border/80'}"
  in:fade={{ duration: 150 }}
>
  
  <!-- Flex layout places children according to language direction -->
  {#if locale === "ar"}
    <!-- RTL Layout: Text on Right, Color Square on Left -->
    <div class="flex-grow flex flex-col justify-center items-start text-start gap-0.5 min-w-0">
      <span class="font-bold text-[11px] md:text-[11.5px] leading-tight text-foreground tracking-wide line-clamp-2">
        {subjectName}
      </span>
      {#if timeStr}
        <span class="text-[9px] md:text-[9.5px] font-semibold text-muted-foreground opacity-80 leading-none font-mono">
          {timeStr}
        </span>
      {/if}
      <span class="text-[8px] md:text-[8.5px] text-muted-foreground opacity-60 line-clamp-1 mt-0.5">
        {viewMode === "class" ? getTeacherName(lesson.teacher_id) : getClassName(lesson.class_id)}
        {#if lesson.room_name || lesson.room}
          • {lesson.room_name || lesson.room}
        {/if}
      </span>
    </div>
    {#if onDelete && !isDraft}
      <div class="flex items-center shrink-0 self-center px-1">
        <button 
          type="button" 
          onclick={(e) => { e.stopPropagation(); onDelete(lesson.id); }}
          class="text-muted-foreground/60 hover:text-destructive p-1 rounded hover:bg-muted/60 transition-all cursor-pointer flex items-center justify-center"
          title={locale === "ar" ? "حذف الحصة" : "Delete lesson"}
        >
          <Trash2 size={12} />
        </button>
      </div>
    {/if}
    <!-- Color block styled as a button-like square -->
    <div 
      class="w-3.5 h-3.5 rounded shadow-[0_1.5px_3px_rgba(0,0,0,0.1)] border border-black/10 dark:border-white/10 shrink-0 self-center" 
      style="background-color: var(--subject-color); opacity: {isDraft ? 0.5 : 1};"
    ></div>
  {:else}
    <!-- LTR Layout: Color Square on Left, Text on Right -->
    <!-- Color block styled as a button-like square -->
    <div 
      class="w-3.5 h-3.5 rounded shadow-[0_1.5px_3px_rgba(0,0,0,0.1)] border border-black/10 dark:border-white/10 shrink-0 self-center" 
      style="background-color: var(--subject-color); opacity: {isDraft ? 0.5 : 1};"
    ></div>
    {#if onDelete && !isDraft}
      <div class="flex items-center shrink-0 self-center px-1">
        <button 
          type="button" 
          onclick={(e) => { e.stopPropagation(); onDelete(lesson.id); }}
          class="text-muted-foreground/60 hover:text-destructive p-1 rounded hover:bg-muted/60 transition-all cursor-pointer flex items-center justify-center"
          title={locale === "ar" ? "حذف الحصة" : "Delete lesson"}
        >
          <Trash2 size={12} />
        </button>
      </div>
    {/if}
    <div class="flex-grow flex flex-col justify-center items-start text-start gap-0.5 min-w-0">
      <span class="font-bold text-[11px] md:text-[11.5px] leading-tight text-foreground tracking-wide line-clamp-2">
        {subjectName}
      </span>
      {#if timeStr}
        <span class="text-[9px] md:text-[9.5px] font-semibold text-muted-foreground opacity-80 leading-none font-mono">
          {timeStr}
        </span>
      {/if}
      <span class="text-[8px] md:text-[8.5px] text-muted-foreground opacity-60 line-clamp-1 mt-0.5">
        {viewMode === "class" ? getTeacherName(lesson.teacher_id) : getClassName(lesson.class_id)}
        {#if lesson.room_name || lesson.room}
          • {lesson.room_name || lesson.room}
        {/if}
      </span>
    </div>
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
