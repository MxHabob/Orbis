<script lang="ts">
  import { User, MapPin, X } from "lucide-svelte";
  import { fade } from "svelte/transition";

  let {
    lesson,
    viewMode = "class",
    locale = "ar",
    getSubjectName,
    getTeacherName,
    getClassName,
    subjectColor = "#9ca3af",
    onDelete,
    ondragstart,
  }: {
    lesson: any;
    viewMode: "class" | "teacher";
    locale: string;
    getSubjectName: (id: string) => string;
    getTeacherName: (id: string) => string;
    getClassName: (id: string) => string;
    subjectColor?: string;
    onDelete: (id: string) => void;
    ondragstart?: (e: DragEvent) => void;
  } = $props();

  const isDraft = $derived(!!lesson.isDraft);
  const roomName = $derived(lesson.room_name || lesson.room || null);
</script>

<!-- Timetable Lesson Card component with dynamic 3D tactile styling -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div 
  draggable={!isDraft}
  {ondragstart}
  style="--subject-color: {subjectColor};"
  class="relative overflow-hidden w-full h-full pt-2.5 pb-2.5 pe-2.5 ps-4.5 flex flex-col justify-between items-center text-center select-none group transition-all duration-200
    {isDraft ? 'timetable-card-draft' : 'timetable-card-tactile cursor-grab active:cursor-grabbing'}"
  in:fade={{ duration: 150 }}
>
  <!-- Dynamic 3D colored vertical bar on the start edge -->
  <div class="absolute top-0 bottom-0 start-0 w-1.5 bg-[var(--subject-color)] shadow-[inset_-1px_0_1.5px_rgba(0,0,0,0.15)] opacity-95"></div>

  <!-- Draft Overlay Indicator -->
  {#if isDraft}
    <span class="absolute top-1.5 start-3 bg-purple-500/10 text-purple-600 dark:text-purple-400 text-[8px] font-bold px-1 py-0.2 rounded border border-purple-500/20 scale-90 origin-top-left uppercase tracking-wider font-mono">
      {locale === "ar" ? "مسودة" : "Draft"}
    </span>
  {/if}

  <!-- Delete Quick-Action Button -->
  {#if !isDraft}
    <button 
      type="button"
      onclick={() => onDelete(lesson.id)}
      class="absolute top-1.5 end-1.5 opacity-0 group-hover:opacity-100 h-4.5 w-4.5 text-muted-foreground hover:text-destructive hover:bg-destructive/10 rounded flex items-center justify-center transition-all duration-150 cursor-pointer border border-transparent hover:border-destructive/10"
      title={locale === "ar" ? "حذف الحصة" : "Delete lesson"}
    >
      <X size={10} />
    </button>
  {/if}

  <!-- Content Details -->
  <div class="flex flex-col items-center gap-1 w-full mt-2.5">
    <!-- Subject Name -->
    <span class="font-bold text-foreground text-[10px] leading-tight line-clamp-1 tracking-wide">
      {getSubjectName(lesson.subject_id)}
    </span>
    
    <!-- User/Class Label -->
    <span class="text-[9px] text-muted-foreground leading-tight flex items-center justify-center gap-0.5 w-full">
      <User size={8} class="text-muted-foreground/75" />
      <span class="line-clamp-1 font-medium">
        {viewMode === "class" ? getTeacherName(lesson.teacher_id) : getClassName(lesson.class_id)}
      </span>
    </span>
  </div>

  <!-- Room Number / Location Badge -->
  {#if roomName}
    <span class="text-[8px] font-mono font-semibold text-muted-foreground bg-muted/60 border border-border/80 px-1.5 py-0.5 rounded flex items-center gap-0.5 mt-1.5 shrink-0 max-w-full">
      <MapPin size={7} class="text-muted-foreground/75" />
      <span class="line-clamp-1">{roomName}</span>
    </span>
  {/if}
</div>

<style>
  .timetable-card-tactile {
    position: relative;
    border-radius: 0.5rem; /* rounded-lg */
    border: 1px solid var(--border-color);
    background-color: var(--bg-color);
    --shadow-color: color-mix(in srgb, var(--subject-color) 75%, black 25%);
    --bg-color: color-mix(in srgb, var(--subject-color) 6%, var(--card) 94%);
    --border-color: color-mix(in srgb, var(--subject-color) 25%, var(--border) 75%);
    --glow-color: color-mix(in srgb, var(--subject-color) 15%, transparent);

    /* 3D button effect */
    transform: translateY(-1.5px);
    box-shadow:
      0 3px 0 0 var(--shadow-color),
      0 1px 3px 0 rgba(0, 0, 0, 0.08);
  }

  .timetable-card-tactile:hover {
    transform: translateY(-3.5px);
    background-color: color-mix(in srgb, var(--subject-color) 9%, var(--card) 91%);
    border-color: color-mix(in srgb, var(--subject-color) 40%, var(--border) 60%);
    box-shadow:
      0 5px 0 0 var(--shadow-color),
      0 6px 12px -2px var(--glow-color),
      0 4px 6px -1px rgba(0, 0, 0, 0.06);
  }

  .timetable-card-tactile:active {
    transform: translateY(1.5px);
    box-shadow:
      0 0 0 0 var(--shadow-color),
      0 1px 2px 0 rgba(0, 0, 0, 0.05);
  }

  /* Dark mode overrides */
  :global(.dark) .timetable-card-tactile {
    --shadow-color: color-mix(in srgb, var(--subject-color) 40%, black 60%);
    --bg-color: color-mix(in srgb, var(--subject-color) 9%, var(--card) 91%);
    --border-color: color-mix(in srgb, var(--subject-color) 30%, var(--border) 70%);
    --glow-color: color-mix(in srgb, var(--subject-color) 25%, transparent);
    
    box-shadow:
      0 3px 0 0 var(--shadow-color),
      0 1px 4px 0 rgba(0, 0, 0, 0.3);
  }

  :global(.dark) .timetable-card-tactile:hover {
    background-color: color-mix(in srgb, var(--subject-color) 12%, var(--card) 88%);
    border-color: color-mix(in srgb, var(--subject-color) 45%, var(--border) 55%);
    box-shadow:
      0 5px 0 0 var(--shadow-color),
      0 8px 16px -2px var(--glow-color),
      0 4px 8px -2px rgba(0, 0, 0, 0.2);
  }

  :global(.dark) .timetable-card-tactile:active {
    box-shadow:
      0 0 0 0 var(--shadow-color),
      0 1px 2px 0 rgba(0, 0, 0, 0.2);
  }

  /* Draft layout rules */
  .timetable-card-draft {
    position: relative;
    border-radius: 0.5rem;
    border: 1px dashed var(--border-color);
    background-color: var(--bg-color);
    opacity: 0.85;
    cursor: not-allowed;
    animation: draft-pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
    --bg-color: color-mix(in srgb, var(--subject-color) 4%, transparent);
    --border-color: color-mix(in srgb, var(--subject-color) 40%, transparent);
  }

  @keyframes draft-pulse {
    0%, 100% { opacity: 0.85; }
    50% { opacity: 0.45; }
  }

  /* Dark mode overrides for draft */
  :global(.dark) .timetable-card-draft {
    --bg-color: color-mix(in srgb, var(--subject-color) 8%, transparent);
    --border-color: color-mix(in srgb, var(--subject-color) 50%, transparent);
  }
</style>
