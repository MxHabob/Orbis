<script lang="ts">
  import { GripVertical, MapPin, Pencil, Trash2, Users, AlertTriangle, BookOpen, Clock } from "@lucide/svelte";
  import { Button } from "@orbis/ui-svelte/button";

  interface Props {
    lesson: any;
    timeSlot?: any;
    viewMode?: "class" | "teacher" | "room" | "subject" | "matrix";
    locale?: string;
    getSubjectName?: (id: string) => string;
    getTeacherName?: (id: string) => string;
    getClassName?: (id: string) => string;
    getRoomName?: (id: string) => string;
    subjectColor?: string;
    compact?: boolean;
    hasConflict?: boolean;
    conflictMessage?: string;
    isDraft?: boolean;
    ondragstart?: (event: DragEvent) => void;
    onDelete?: (id: string) => void;
    onEdit?: (lesson: any) => void;
    onClick?: (lesson: any) => void;
  }

  let {
    lesson,
    timeSlot,
    viewMode = "class",
    locale = "ar",
    getSubjectName,
    getTeacherName,
    getClassName,
    getRoomName,
    subjectColor = "var(--primary)",
    compact = false,
    hasConflict = false,
    conflictMessage = "",
    isDraft = false,
    ondragstart,
    onDelete,
    onEdit,
    onClick,
  }: Props = $props();

  const isArabic = $derived(locale === "ar");
  const subjectName = $derived(
    lesson.subject_name ||
      lesson.subjectName ||
      (getSubjectName ? getSubjectName(lesson.subject_id) : "") ||
      lesson.subject_id ||
      (isArabic ? "مادة دراسية" : "Subject")
  );

  const teacherName = $derived(
    lesson.teacher_name ||
      lesson.teacherName ||
      (getTeacherName ? getTeacherName(lesson.teacher_id) : "") ||
      lesson.teacher_id ||
      ""
  );

  const className = $derived(
    lesson.class_name ||
      lesson.className ||
      (getClassName ? getClassName(lesson.class_id) : "") ||
      lesson.class_id ||
      ""
  );

  const roomName = $derived(
    lesson.room_name ||
      lesson.roomName ||
      (getRoomName ? getRoomName(lesson.room_id) : "") ||
      lesson.room ||
      ""
  );

  const isConflict = $derived(Boolean(hasConflict || lesson.has_conflict || lesson.conflict));

  function formatTime(val: string): string {
    if (!val) return "";
    const clean = val.slice(0, 5);
    const [hStr, mStr] = clean.split(":");
    let h = parseInt(hStr, 10);
    const m = mStr || "00";
    if (isNaN(h)) return val;
    const period = h >= 12 ? (isArabic ? "م" : "PM") : (isArabic ? "ص" : "AM");
    h = h % 12 || 12;
    return `${h}:${m} ${period}`;
  }

  const timeLabel = $derived.by(() => {
    const s = lesson.time_slot_start || timeSlot?.start_time;
    const e = lesson.time_slot_end || timeSlot?.end_time;
    return s && e ? `${formatTime(s)} - ${formatTime(e)}` : "";
  });

  function handleCardClick(e: MouseEvent) {
    e.stopPropagation();
    onClick?.(lesson);
  }

  function handleEditClick(e: MouseEvent) {
    e.stopPropagation();
    onEdit?.(lesson);
  }

  function handleDeleteClick(e: MouseEvent) {
    e.stopPropagation();
    onDelete?.(lesson.id);
  }

  function handleDragStart(e: DragEvent) {
    e.stopPropagation();
    if (e.dataTransfer) {
      e.dataTransfer.setData("text/plain", JSON.stringify({ lessonId: lesson.id, lesson }));
      e.dataTransfer.effectAllowed = "move";
    }
    ondragstart?.(e);
  }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<article
  role="button"
  tabindex="0"
  onclick={handleCardClick}
  draggable={!isDraft}
  ondragstart={handleDragStart}
  style={`--subject-theme: ${subjectColor};`}
  class="group relative flex h-full w-full min-w-0 flex-col justify-between overflow-hidden rounded-lg border transition-all duration-150 select-none text-start text-foreground focus-visible:ring-2 focus-visible:ring-ring focus-visible:outline-none {compact
    ? 'p-1.5 min-h-[52px]'
    : 'p-2 min-h-[72px]'} {isConflict
    ? 'border-destructive bg-destructive/10 shadow-sm ring-1 ring-destructive/30'
    : isDraft
      ? 'border-dashed border-primary/50 bg-primary/5'
      : 'border-border/60 bg-card hover:border-primary/40 hover:bg-card/90 hover:shadow-sm'}"
>
  <!-- Accent Edge Bar -->
  <div
    class="absolute inset-s-0 top-0 bottom-0 w-1 rounded-s-lg transition-all group-hover:w-1.5"
    style="background-color: var(--subject-theme);"
  ></div>

  <!-- Header / Subject Title & Conflict Indicator -->
  <div class="flex items-start justify-between gap-1 ps-1.5">
    <div class="min-w-0 flex-1">
      <div class="flex items-center gap-1">
        <span
          class="font-bold tracking-tight truncate leading-tight {compact ? 'text-[11px]' : 'text-xs'}"
          title={subjectName}
        >
          {subjectName}
        </span>
        {#if isDraft}
          <span class="rounded bg-primary/20 px-1 py-0.2 text-[9px] font-semibold text-primary">
            {isArabic ? "مسودة" : "Draft"}
          </span>
        {/if}
      </div>

      {#if lesson.topic}
        <p class="text-[9px] text-muted-foreground truncate leading-none mt-0.5" title={lesson.topic}>
          {lesson.topic}
        </p>
      {/if}
    </div>

    {#if isConflict}
      <span
        class="shrink-0 flex items-center justify-center text-destructive animate-pulse"
        title={conflictMessage || (isArabic ? "يوجد تعارض في هذا الموعد" : "Schedule conflict")}
      >
        <AlertTriangle size={13} strokeWidth={2.5} />
      </span>
    {/if}
  </div>

  <!-- Contextual Metadata Pills -->
  <div class="flex flex-wrap items-center gap-x-2 gap-y-0.5 ps-1.5 text-muted-foreground {compact ? 'text-[9px] mt-1' : 'text-[10px] mt-1.5'}">
    <!-- ViewMode Contextual rendering -->
    {#if viewMode !== "class" && className}
      <span class="inline-flex items-center gap-1 font-medium truncate max-w-[120px]" title={className}>
        <Users size={10} class="shrink-0 text-muted-foreground/70" />
        <span class="truncate">{className}</span>
      </span>
    {/if}

    {#if viewMode !== "teacher" && teacherName}
      <span class="inline-flex items-center gap-1 truncate max-w-[110px]" title={teacherName}>
        <Users size={10} class="shrink-0 text-muted-foreground/70" />
        <span class="truncate">{teacherName}</span>
      </span>
    {/if}

    {#if roomName}
      <span class="inline-flex items-center gap-1 truncate text-foreground/80 font-medium" title={roomName}>
        <MapPin size={10} class="shrink-0 text-primary" />
        <span class="truncate">{roomName}</span>
      </span>
    {/if}
  </div>

  <!-- Footer: Time Slot & Quick Actions -->
  <div class="flex items-center justify-between ps-1.5 pt-1 border-t border-border/40 mt-1">
    <span class="text-[9px] font-medium text-muted-foreground/80 font-mono" dir="ltr">
      {timeLabel || (timeSlot ? `${formatTime(timeSlot.start_time)} - ${formatTime(timeSlot.end_time)}` : "")}
    </span>

    <!-- Hover Micro-Actions -->
    {#if !isDraft && (onEdit || onDelete)}
      <div class="flex items-center gap-0.5 opacity-0 group-hover:opacity-100 transition-opacity">
        {#if onEdit}
          <Button
            variant="ghost"
            size="icon"
            class="h-5 w-5 rounded p-0 text-muted-foreground hover:text-foreground"
            onclick={handleEditClick}
            aria-label={isArabic ? "تعديل" : "Edit"}
          >
            <Pencil size={10} />
          </Button>
        {/if}
        {#if onDelete}
          <Button
            variant="ghost"
            size="icon"
            class="h-5 w-5 rounded p-0 text-destructive/70 hover:text-destructive hover:bg-destructive/10"
            onclick={handleDeleteClick}
            aria-label={isArabic ? "حذف" : "Delete"}
          >
            <Trash2 size={10} />
          </Button>
        {/if}
      </div>
    {/if}
  </div>
</article>
