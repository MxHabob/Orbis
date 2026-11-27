<script lang="ts">
import { CheckSquare, InfoCircle, Pencil, Pin, Trash, Users } from "@orbis/icons";
import { Button } from "@orbis/ui/button";
import { formatTimeRange, resolveSubjectColor } from "./utils.js";

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
  onAttendance?: (lesson: any) => void;
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
  onAttendance,
}: Props = $props();

const isArabic = $derived(locale === "ar");

const subjectName = $derived(
  lesson.subject_name ||
    lesson.subjectName ||
    (getSubjectName ? getSubjectName(lesson.subject_id) : "") ||
    lesson.subject_id ||
    (isArabic ? "مادة دراسية" : "Subject"),
);

const teacherName = $derived(
  lesson.teacher_name ||
    lesson.teacherName ||
    (getTeacherName ? getTeacherName(lesson.teacher_id) : "") ||
    lesson.teacher_id ||
    "",
);

const className = $derived(
  lesson.class_name ||
    lesson.className ||
    (getClassName ? getClassName(lesson.class_id) : "") ||
    lesson.class_id ||
    "",
);

const roomName = $derived(
  lesson.room_name ||
    lesson.roomName ||
    (getRoomName ? getRoomName(lesson.room_id) : "") ||
    lesson.room ||
    "",
);

const primaryTitle = $derived(
  viewMode === "subject" && className ? className : subjectName,
);

const resolvedColor = $derived(
  resolveSubjectColor(lesson.subject_id, subjectColor),
);

const isConflict = $derived(
  Boolean(hasConflict || lesson.has_conflict || lesson.conflict),
);

const timeLabel = $derived(
  formatTimeRange(
    lesson.time_slot_start || timeSlot?.start_time,
    lesson.time_slot_end || timeSlot?.end_time,
    isArabic,
  ),
);

function handleCardClick(e: MouseEvent) {
  e.stopPropagation();
  onClick?.(lesson);
}

function handleCardKeydown(e: KeyboardEvent) {
  if (e.key === "Enter" || e.key === " ") {
    e.preventDefault();
    onClick?.(lesson);
  }
}

function handleEditClick(e: MouseEvent) {
  e.stopPropagation();
  onEdit?.(lesson);
}

function handleDeleteClick(e: MouseEvent) {
  e.stopPropagation();
  onDelete?.(lesson.id);
}

function handleAttendanceClick(e: MouseEvent) {
  e.stopPropagation();
  onAttendance?.(lesson);
}

function handleDragStart(e: DragEvent) {
  e.stopPropagation();
  if (e.dataTransfer) {
    e.dataTransfer.setData(
      "text/plain",
      JSON.stringify({ lessonId: lesson.id, lesson }),
    );
    e.dataTransfer.effectAllowed = "move";
  }
  ondragstart?.(e);
}
</script>

<!-- svelte-ignore a11y_no_noninteractive_element_to_interactive_role -->
<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<article
  role="button"
  tabindex="0"
  onclick={handleCardClick}
  onkeydown={handleCardKeydown}
  draggable={!isDraft}
  ondragstart={handleDragStart}
  style={`--subject-theme: ${resolvedColor};`}
  class="session-card group relative flex h-full w-full min-w-0 cursor-pointer flex-col justify-between overflow-hidden rounded-md border text-start text-foreground transition-colors select-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:outline-none {compact
    ? 'min-h-14'
    : 'min-h-18'} {isConflict
    ? 'border-destructive/80 bg-destructive/10'
    : isDraft
      ? 'border-dashed border-primary/50 bg-primary/8'
      : 'session-card--tinted'}"
>
  <!-- Crisp 3px subject accent bar -->
  <div
    class="absolute inset-s-0 top-0 bottom-0 w-[3px]"
    style="background-color: {isConflict ? 'var(--destructive)' : 'var(--subject-theme)'};"
  ></div>

  <!-- Main Content -->
  <div class="flex min-h-0 flex-1 flex-col justify-between {compact ? 'px-2 pt-1.5 pb-1 ps-2.5' : 'px-2.5 pt-2 pb-1.5 ps-3'}">
    <!-- Header -->
    <div class="flex items-start justify-between gap-1.5">
      <div class="min-w-0 flex-1">
        <div class="flex items-center gap-1.5">
          <span
            class="truncate font-semibold leading-snug {compact
              ? 'text-[11px]'
              : 'text-xs'}"
            title={primaryTitle}
          >
            {primaryTitle}
          </span>
          {#if isDraft}
            <span
              class="shrink-0 rounded-sm bg-primary/15 px-1 py-0.5 text-[10px] font-medium leading-none text-primary"
            >
              {isArabic ? "مسودة" : "Draft"}
            </span>
          {/if}
        </div>

        {#if lesson.topic}
          <p
            class="mt-0.5 truncate text-[11px] leading-tight text-muted-foreground"
            title={lesson.topic}
          >
            {lesson.topic}
          </p>
        {/if}
      </div>

      {#if isConflict}
        <span
          class="flex shrink-0 items-center justify-center text-destructive"
          title={conflictMessage ||
            (isArabic ? "يوجد تعارض في هذا الموعد" : "Schedule conflict")}
        >
          <InfoCircle size={13} strokeWidth={2.25} />
        </span>
      {/if}
    </div>

    <!-- Contextual Metadata -->
    <div
      class="flex flex-wrap items-center gap-x-2 gap-y-0.5 text-muted-foreground {compact
        ? 'mt-1 text-[10px]'
        : 'mt-1.5 text-[11px]'}"
    >
      {#if viewMode !== "class" && viewMode !== "subject" && className}
        <span
          class="inline-flex max-w-[130px] items-center gap-1 truncate font-medium text-foreground/85"
          title={className}
        >
          <Users size={11} class="shrink-0 text-muted-foreground" />
          <span class="truncate">{className}</span>
        </span>
      {/if}

      {#if viewMode !== "teacher" && teacherName}
        <span
          class="inline-flex max-w-[130px] items-center gap-1 truncate"
          title={teacherName}
        >
          <span class="truncate">{teacherName}</span>
        </span>
      {/if}

      {#if viewMode !== "room" && roomName}
        <span
          class="inline-flex items-center gap-0.5 truncate font-medium text-foreground/85"
          title={roomName}
        >
          <Pin size={10} class="shrink-0" style="color: var(--subject-theme);" />
          <span class="truncate">{roomName}</span>
        </span>
      {/if}
    </div>
  </div>

  <!-- Footer: Subtle Tinted Time Band & Quick Actions -->
  {#if !compact || (!isDraft && (onAttendance || onEdit || onDelete))}
    <div
      class="session-card__footer flex items-center justify-between gap-1 border-t px-2 py-1 ps-2.5"
    >
      <span
        class="truncate font-mono text-[10px] font-medium text-foreground/75 tabular-nums"
      >
        {timeLabel}
      </span>

      {#if !isDraft && (onAttendance || onEdit || onDelete)}
        <div
          class="touch-actions flex shrink-0 items-center gap-0.5 opacity-0 transition-opacity group-hover:opacity-100 group-focus-within:opacity-100"
        >
          {#if onAttendance}
            <Button
              variant="ghost"
              size="icon"
              class="h-5 w-5 rounded-sm p-0 text-muted-foreground hover:text-primary"
              onclick={handleAttendanceClick}
              aria-label={isArabic ? "تسجيل الحضور" : "Record attendance"}
              title={isArabic ? "تسجيل الحضور" : "Record attendance"}
            >
              <CheckSquare size={11} />
            </Button>
          {/if}
          {#if onEdit}
            <Button
              variant="ghost"
              size="icon"
              class="h-5 w-5 rounded-sm p-0 text-muted-foreground hover:text-foreground"
              onclick={handleEditClick}
              aria-label={isArabic ? "تعديل" : "Edit"}
              title={isArabic ? "تعديل" : "Edit"}
            >
              <Pencil size={10} />
            </Button>
          {/if}
          {#if onDelete}
            <Button
              variant="ghost"
              size="icon"
              class="h-5 w-5 rounded-sm p-0 text-destructive/75 hover:bg-destructive/10 hover:text-destructive"
              onclick={handleDeleteClick}
              aria-label={isArabic ? "حذف" : "Delete"}
              title={isArabic ? "حذف" : "Delete"}
            >
              <Trash size={10} />
            </Button>
          {/if}
        </div>
      {/if}
    </div>
  {/if}
</article>

<style>
  .session-card--tinted {
    border-color: color-mix(in srgb, var(--subject-theme) 38%, var(--border));
    background-color: color-mix(in srgb, var(--subject-theme) 13%, var(--card));
  }

  .session-card--tinted:hover {
    border-color: color-mix(in srgb, var(--subject-theme) 58%, var(--border));
    background-color: color-mix(in srgb, var(--subject-theme) 20%, var(--card));
  }

  .session-card__footer {
    border-color: color-mix(in srgb, var(--subject-theme) 22%, var(--border));
    background-color: color-mix(in srgb, var(--subject-theme) 8%, transparent);
  }

  @media (hover: none) {
    .touch-actions {
      opacity: 1;
    }
  }
</style>
