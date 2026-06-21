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
    onDelete,
    ondragstart,
  }: {
    lesson: any;
    viewMode: "class" | "teacher";
    locale: string;
    getSubjectName: (id: string) => string;
    getTeacherName: (id: string) => string;
    getClassName: (id: string) => string;
    onDelete: (id: string) => void;
    ondragstart?: (e: DragEvent) => void;
  } = $props();

  const isDraft = $derived(!!lesson.isDraft);
  const roomName = $derived(lesson.room_name || lesson.room || null);
</script>

<!-- Timetable Lesson Card component with glassmorphism styling -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div 
  draggable={!isDraft}
  {ondragstart}
  class="relative w-full h-full p-2.5 rounded-lg flex flex-col justify-between items-center text-center select-none group transition-all duration-250 ease-out border
    {isDraft 
      ? 'border-dashed border-purple-500/40 bg-purple-500/[0.04] text-purple-600 dark:text-purple-400 opacity-80 cursor-not-allowed animate-pulse' 
      : 'bg-card/45 border-border/80 shadow-sm cursor-grab active:cursor-grabbing hover:bg-card/75 hover:border-primary/30 hover:-translate-y-[1px] hover:shadow-md'}"
  in:fade={{ duration: 150 }}
>
  <!-- Draft Overlay Indicator -->
  {#if isDraft}
    <span class="absolute top-1.5 start-1.5 bg-purple-500/10 text-purple-600 dark:text-purple-400 text-[8px] font-bold px-1 py-0.2 rounded border border-purple-500/20 scale-90 origin-top-left uppercase tracking-wider font-mono">
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
