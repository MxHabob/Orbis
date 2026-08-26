<script lang="ts">
  import { fade } from "svelte/transition";
  import {
    Plus,
    Clock,
    Pencil,
    Trash2,
    Save,
    X,
  } from "@lucide/svelte";
  import { Button } from "@orbis/ui-svelte/button";

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
    getRoomName,
    getSubjectColor,
    compact = false,
    fitViewport = false,
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
    getRoomName?: (id: string) => string;
    getSubjectColor: (id: string) => string;
    compact?: boolean;
    fitViewport?: boolean;
    class?: string;
    onCellClick: (day: number, slotId: string) => void;
    onDeleteLesson: (id: string) => void;
    onEditLesson?: (lesson: any) => void;
    onMoveLesson: (
      lesson: any,
      targetDay: number,
      targetSlotId: string,
    ) => void;
    onAddTimeSlot?: () => void;
    onEditTimeSlot?: (slot: any) => void;
    onDeleteTimeSlot?: (slot: any) => void;
    onSaveTimeSlot?: (
      slot: any | null,
      data: any,
    ) => void;
  } = $props();

  // ===========================================================================
  // TIME SLOT EDITING
  // ===========================================================================

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

    draftStartTime = String(
      slot.start_time || "",
    ).slice(0, 5);

    draftEndTime = String(
      slot.end_time || "",
    ).slice(0, 5);
  }

  function startSlotAdd() {
    editingSlotId = null;
    isAddingSlot = true;

    const nextNumber =
      timeSlots.reduce(
        (highest, slot) =>
          Math.max(
            highest,
            Number(slot.slot_number) || 0,
          ),
        0,
      ) + 1;

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
    if (
      !draftNameAr.trim() ||
      !draftStartTime ||
      !draftEndTime ||
      !onSaveTimeSlot
    ) {
      return;
    }

    const [
      startHour = 0,
      startMinute = 0,
    ] = draftStartTime
      .split(":")
      .map(Number);

    const [
      endHour = 0,
      endMinute = 0,
    ] = draftEndTime
      .split(":")
      .map(Number);

    onSaveTimeSlot(
      editingSlotId
        ? timeSlots.find(
            (slot) =>
              String(slot.id) ===
              editingSlotId,
          )
        : null,
      {
        name:
          draftName.trim() ||
          draftNameAr.trim(),

        name_ar:
          draftNameAr.trim(),

        start_time:
          draftStartTime,

        end_time:
          draftEndTime,

        duration_minutes:
          endHour * 60 +
          endMinute -
          (startHour * 60 + startMinute),
      },
    );

    cancelSlotEdit();
  }

  // ===========================================================================
  // LESSONS
  // ===========================================================================

  function getLessonsInCell(
    day: number,
    slotId: string,
  ) {
    if (previewDraft) {
      const drafts =
        draftLessons.filter(
          (lesson: any) =>
            Number(lesson.day_of_week) === day &&
            String(lesson.time_slot_id) ===
              String(slotId),
        );

      if (drafts.length > 0) {
        return drafts;
      }
    }

    return lessons.filter(
      (lesson: any) =>
        Number(lesson.day_of_week) === day &&
        String(lesson.time_slot_id) ===
          String(slotId),
    );
  }

  function isSlotAvailable(
    slot: any,
    day: number,
  ): boolean {
    if (!slot.applicable_days) {
      return true;
    }

    const days = String(
      slot.applicable_days,
    )
      .split(",")
      .map((value) =>
        Number(value.trim()),
      )
      .filter((value) =>
        Number.isInteger(value),
      );

    return (
      days.length === 0 ||
      days.includes(day)
    );
  }

  // ===========================================================================
  // TIME SORTING
  // ===========================================================================

  function timeToMinutes(
    value: unknown,
  ): number {
    const match = String(value ?? "").match(
      /^(\d{1,2}):(\d{2})/,
    );

    if (!match) {
      return Number.MAX_SAFE_INTEGER;
    }

    return (
      Number(match[1]) * 60 +
      Number(match[2])
    );
  }

  function getChronologicalTime(
    value: unknown,
  ): number {
    const minutes =
      timeToMinutes(value);

    if (
      minutes !==
      Number.MAX_SAFE_INTEGER
    ) {
      return minutes;
    }

    return Number.MAX_SAFE_INTEGER;
  }

  const orderedTimeSlots = $derived(
    [...timeSlots].sort(
      (first, second) => {
        const startDifference =
          getChronologicalTime(
            first.start_time,
          ) -
          getChronologicalTime(
            second.start_time,
          );

        if (startDifference !== 0) {
          return startDifference;
        }

        return (
          getChronologicalTime(
            first.end_time,
          ) -
          getChronologicalTime(
            second.end_time,
          )
        );
      },
    ),
  );

  // ===========================================================================
  // DRAG & DROP
  // ===========================================================================

  let draggedLesson =
    $state<any>(null);

  let dragOverCell =
    $state<{
      day: number;
      slotId: string;
    } | null>(null);

  function handleDragStart(
    event: DragEvent,
    lesson: any,
  ) {
    if (lesson.isDraft) {
      event.preventDefault();
      return;
    }

    draggedLesson = lesson;

    if (event.dataTransfer) {
      event.dataTransfer.effectAllowed =
        "move";

      event.dataTransfer.setData(
        "text/plain",
        String(lesson.id),
      );
    }
  }

  function handleDragOver(
    event: DragEvent,
    day: number,
    slotId: string,
  ) {
    event.preventDefault();

    if (!draggedLesson) {
      return;
    }

    dragOverCell = {
      day,
      slotId,
    };

    if (event.dataTransfer) {
      event.dataTransfer.dropEffect =
        "move";
    }
  }

  function handleDragLeave() {
    dragOverCell = null;
  }

  function handleDrop(
    event: DragEvent,
    targetDay: number,
    targetSlotId: string,
  ) {
    event.preventDefault();

    dragOverCell = null;

    if (!draggedLesson) {
      return;
    }

    const lesson =
      draggedLesson;

    draggedLesson = null;

    const sameDay =
      Number(
        lesson.day_of_week,
      ) === targetDay;

    const sameSlot =
      String(
        lesson.time_slot_id,
      ) ===
      String(targetSlotId);

    if (sameDay && sameSlot) {
      return;
    }

    onMoveLesson(
      lesson,
      targetDay,
      targetSlotId,
    );
  }
</script>

<!-- ===========================================================================
     SCHEDULE BOARD
     =========================================================================== -->

<div
  class="
    schedule-board
    relative
    flex
    h-full
    min-h-0
    min-w-0
    w-full
    flex-1
    select-none
    flex-col
    border-y
    border-border/60
    bg-card
    scrollbar-thin
    {fitViewport
      ? 'overflow-hidden fit-viewport'
      : 'overflow-auto'}
    {className}
  "
>
  <div
    class="
      schedule-grid
      grid
      w-full
      min-h-0
      flex-1
      gap-px
      bg-border/35
      text-xs
      {fitViewport
        ? 'min-w-0'
        : 'min-w-225'}
    "
    style="
      grid-template-columns:
        {fitViewport
          ? `minmax(64px, 0.55fr) repeat(${weekdays.length}, minmax(0, 1fr))`
          : `minmax(96px, 0.55fr) repeat(${weekdays.length}, minmax(150px, 1fr))`};
      grid-template-rows:
        {fitViewport
          ? `auto repeat(${orderedTimeSlots.length + (isAddingSlot ? 1 : 0)}, minmax(0, 1fr))`
          : 'none'};
    "
    in:fade={{ duration: 150 }}
  >
    <!-- =======================================================================
         HEADER
         ======================================================================= -->

    <div
      class="
        sticky
        top-0
        z-30
        flex
        items-center
        justify-center
        gap-1
        border-b
        border-primary/20
        bg-primary/10
        p-2
        text-center
        text-[9px]
        font-semibold
        text-foreground
      "
    >
      <span>
        {locale === "ar"
          ? "الحصة"
          : "Period"}
      </span>

      {#if onAddTimeSlot}
        <button
          type="button"
          class="
            rounded
            p-1
            text-primary
            transition-colors
          "
          onclick={startSlotAdd}
          aria-label={
            locale === "ar"
              ? "إضافة حصة"
              : "Add period"
          }
          title={
            locale === "ar"
              ? "إضافة حصة"
              : "Add period"
          }
        >
          <Plus size={13} />
        </button>
      {/if}
    </div>

    {#each weekdays as day (day.value)}
      <div
        class="
          sticky
          top-0
          z-20
          flex
          items-center
          justify-center
          border-b
          border-primary/20
          bg-primary/10
          p-2
          text-center
          text-[10px]
          font-semibold
          text-foreground
        "
      >
        {day.label}
      </div>
    {/each}

    <!-- =======================================================================
         ADD PERIOD
         ======================================================================= -->

    {#if isAddingSlot}
      <div
        class="
          sticky
          inset-s-0
          z-10
          flex
          flex-col
          justify-center
          gap-1
          border-b
          border-border
          bg-muted/30
          p-2
        "
      >
        <input
          class="
            h-7
            w-full
            rounded
            border
            border-border
            bg-background
            px-1
            text-[10px]
          "
          bind:value={draftNameAr}
          placeholder="اسم الحصة"
          aria-label="اسم الحصة"
        />

        <div class="flex gap-1">
          <input
            class="
              h-6
              min-w-0
              w-1/2
              rounded
              border
              border-border
              bg-background
              px-1
              text-[9px]
            "
            bind:value={draftStartTime}
            type="time"
            aria-label="وقت البداية"
          />

          <input
            class="
              h-6
              min-w-0
              w-1/2
              rounded
              border
              border-border
              bg-background
              px-1
              text-[9px]
            "
            bind:value={draftEndTime}
            type="time"
            aria-label="وقت النهاية"
          />
        </div>

        <div
          class="
            flex
            justify-center
            gap-1
          "
        >
          <button
            type="button"
            class="
              rounded
              p-1
              text-primary
            "
            onclick={saveSlotEdit}
            aria-label="حفظ"
          >
            <Save size={12} />
          </button>

          <button
            type="button"
            class="
              rounded
              p-1
              text-muted-foreground
            "
            onclick={cancelSlotEdit}
            aria-label="إلغاء"
          >
            <X size={12} />
          </button>
        </div>
      </div>

      <div
        class="border-b border-border/50 bg-muted/20"
        style="
          grid-column:
            span {weekdays.length};
        "
      ></div>
    {/if}

    <!-- =======================================================================
         TIME SLOT ROWS
         ======================================================================= -->

    {#each orderedTimeSlots as slot (slot.id)}
      {@const isBreak =
        slot.is_break === 1 ||
        slot.is_break === true}

       <!-- =====================================================================
         PERIOD COLUMN
         ===================================================================== -->

       {#if !isBreak}
         <div
        class="
          group
          sticky
          inset-s-0
          z-10
          flex
          h-full
          {fitViewport
            ? 'min-h-0'
            : isBreak
              ? 'min-h-6'
              : 'min-h-24'}
          flex-col
          items-center
          justify-center
          gap-1
          border-b
          border-border/50
          bg-muted/45
          {isBreak ? 'p-1' : 'p-1.5'}
          text-center
          font-semibold
        "
        >
        {#if editingSlotId === String(slot.id)}
          <input
            class="
              h-7
              w-full
              rounded
              border
              border-border/50
              bg-background
              px-1
              text-[10px]
            "
            bind:value={draftNameAr}
            aria-label="اسم الحصة"
          />

          <div class="flex w-full gap-1">
            <input
              class="
                h-6
                min-w-0
                w-1/2
                rounded
                border
                border-border
                bg-background
                px-1
                text-[9px]
              "
              bind:value={draftStartTime}
              type="time"
              aria-label="وقت البداية"
            />

            <input
              class="
                h-6
                min-w-0
                w-1/2
                rounded
                border
                border-border
                bg-background
                px-1
                text-[9px]
              "
              bind:value={draftEndTime}
              type="time"
              aria-label="وقت النهاية"
            />
          </div>

          <span class="flex items-center gap-1">
            <button
              type="button"
              class="
                rounded
                p-1
                text-primary
              "
              onclick={saveSlotEdit}
              aria-label="حفظ"
            >
              <Save size={12} />
            </button>

            <button
              type="button"
              class="
                rounded
                p-1
                text-muted-foreground
              "
              onclick={cancelSlotEdit}
              aria-label="إلغاء"
            >
              <X size={12} />
            </button>
          </span>
        {:else}
          {#if !isBreak}
            <span
              class="
                max-w-full
                truncate
                text-[10px]
                font-bold
                tracking-wide
                text-foreground
              "
            >
              {locale === "ar"
                ? (
                    slot.name_ar ||
                    slot.name
                  )
                : (
                    slot.name ||
                    slot.name_ar
                  )}
            </span>
          {/if}

          {#if !isBreak}
            <span
              class="
                flex
                shrink-0
                items-center
                gap-1
                text-[9px]
                font-medium
                text-muted-foreground
              "
            >
              <Clock
                size={10}
                class="opacity-70"
              />

              <span
                dir="ltr"
                class="
                  tabular-nums
                  whitespace-nowrap
                "
              >
                {slot.start_time} -
                {slot.end_time}
              </span>
            </span>
          {/if}
        {/if}

        <!-- PERIOD ACTIONS -->

        {#if
          !isBreak &&
          (onEditTimeSlot ||
            onDeleteTimeSlot) &&
          editingSlotId !==
            String(slot.id)
        }
          <span
            class="
              flex
              w-full
              items-center
              justify-center
              gap-0.5
            "
          >
            {#if onEditTimeSlot}
              <Button
                variant="secondary"
                size="icon-xs"
                class="
                  text-muted-foreground
                  opacity-0
                  pointer-events-none
                  transition-opacity
                  group-hover:opacity-100
                  group-hover:pointer-events-auto
                  focus-visible:opacity-100
                  focus-visible:pointer-events-auto
                "
                onclick={() => {
                  startSlotEdit(slot);
                  onEditTimeSlot?.(
                    slot,
                  );
                }}
                aria-label={
                  locale === "ar"
                    ? "تعديل الفترة"
                    : "Edit period"
                }
                title={
                  locale === "ar"
                    ? "تعديل الفترة"
                    : "Edit period"
                }
              >
                <Pencil size={9} strokeWidth={2.5} />
              </Button>
            {/if}

            {#if onDeleteTimeSlot}
              <Button
                variant="destructive"
                size="icon-xs"
                class="
                  text-muted-foreground
                  opacity-0
                  pointer-events-none
                  transition-opacity
                  group-hover:opacity-100
                  group-hover:pointer-events-auto
                  focus-visible:opacity-100
                  focus-visible:pointer-events-auto
                "
                onclick={() =>
                  onDeleteTimeSlot?.(
                    slot,
                  )}
                aria-label={
                  locale === "ar"
                    ? "حذف الفترة"
                    : "Delete period"
                }
                title={
                  locale === "ar"
                    ? "حذف الفترة"
                    : "Delete period"
                }
              >
                <Trash2 size={9} />
              </Button>
            {/if}
          </span>
        {/if}
        </div>
      {/if}

      <!-- =====================================================================
           BREAK
           ===================================================================== -->

      {#if isBreak}
        <div
          class="
            flex
            group
            h-full
            min-h-6
            items-center
            justify-center
            gap-2
            border-2
            border-dashed
            border-primary/60
            bg-primary/10
            px-2
            text-[10px]
            font-bold
            text-primary
          "
          style="
            grid-column:
              span {weekdays.length + 1};
          "
        >
          {#if editingSlotId === String(slot.id)}
            <input
              class="
                h-6
                w-28
                rounded
                border
                border-border
                bg-background
                px-1
                text-[9px]
                text-foreground
              "
              bind:value={draftNameAr}
              aria-label="اسم الاستراحة"
            />

            <input
              class="
                h-6
                rounded
                border
                border-border
                bg-background
                px-1
                text-[9px]
                text-foreground
              "
              bind:value={draftStartTime}
              type="time"
              aria-label="وقت البداية"
            />

            <input
              class="
                h-6
                rounded
                border
                border-border
                bg-background
                px-1
                text-[9px]
                text-foreground
              "
              bind:value={draftEndTime}
              type="time"
              aria-label="وقت النهاية"
            />

            <button
              type="button"
              class="rounded p-1 text-primary hover:bg-primary/10"
              onclick={saveSlotEdit}
              aria-label="حفظ"
            >
              <Save size={11} />
            </button>

            <button
              type="button"
              class="rounded p-1 text-muted-foreground hover:bg-muted"
              onclick={cancelSlotEdit}
              aria-label="إلغاء"
            >
              <X size={11} />
            </button>
          {:else}
            <span>
              {locale === "ar"
                ? "استراحة"
                : "BREAK"}
            </span>

            <span
              dir="ltr"
              class="
                tabular-nums
                font-mono
                text-[9px]
                font-bold
                opacity-80
              "
            >
              ({slot.start_time} -
              {slot.end_time})
            </span>
          {/if}

          {#if editingSlotId !== String(slot.id) && (onEditTimeSlot || onDeleteTimeSlot)}
            <span
              class="
                flex
                items-center
                gap-0.5
                opacity-0
                pointer-events-none
                transition-opacity
                group-hover:opacity-100
                group-hover:pointer-events-auto
                focus-within:opacity-100
                focus-within:pointer-events-auto
              "
            >
              {#if onEditTimeSlot}
                <Button
                  variant="secondary"
                  size="icon-xs"
                  onclick={() =>
                    startSlotEdit(slot)}
                  aria-label={
                    locale === "ar"
                      ? "تعديل الاستراحة"
                      : "Edit break"
                  }
                  title={
                    locale === "ar"
                      ? "تعديل الاستراحة"
                      : "Edit break"
                  }
                >
                  <Pencil size={9} strokeWidth={2.5} />
                </Button>
              {/if}

              {#if onDeleteTimeSlot}
                <Button
                  variant="destructive"
                  size="icon-xs"
                  onclick={() =>
                    onDeleteTimeSlot?.(
                      slot,
                    )}
                  aria-label={
                    locale === "ar"
                      ? "حذف الاستراحة"
                      : "Delete break"
                  }
                  title={
                    locale === "ar"
                      ? "حذف الاستراحة"
                      : "Delete break"
                  }
                >
                  <Trash2 size={9} />
                </Button>
              {/if}
            </span>
          {/if}
        </div>
      {:else}
        <!-- ===================================================================
             DAY CELLS
             =================================================================== -->

        {#each weekdays as day (day.value)}
          {@const cellLessons =
            getLessonsInCell(
              day.value,
              slot.id,
            )}

          {@const isAvailable =
            isSlotAvailable(
              slot,
              day.value,
            )}

          {@const isDragOver =
            dragOverCell?.day ===
              day.value &&
            dragOverCell?.slotId ===
              slot.id}

          <!-- ================================================================
               CELL

               The cell is full width.

               The lesson card is NOT.
               ================================================================ -->

          <!-- svelte-ignore a11y_no_static_element_interactions -->
          <div
            class="
              timetable-cell
              relative
              flex
              h-full
              {fitViewport ? 'min-h-0' : 'min-h-24'}
              items-center
              justify-center
              border-b
              border-border
              p-1
              text-center
              align-middle
              transition-colors
              duration-150
              {isAvailable
                ? 'bg-card'
                : 'bg-muted/20 schedule-unavailable'}
              {isDragOver
                ? 'bg-primary/8 ring-2 ring-inset ring-primary/40'
                : ''}
            "
            ondragover={(event) =>
              handleDragOver(
                event,
                day.value,
                slot.id,
              )}
            ondragleave={handleDragLeave}
            ondrop={(event) =>
              handleDrop(
                event,
                day.value,
                slot.id,
              )}
          >
            {#if !isAvailable}
              <span
                class="
                  text-[9px]
                  text-muted-foreground/40
                "
              >
                -
              </span>
            {:else if cellLessons.length > 0}
              <!-- ============================================================
                   LESSON STACK

                   IMPORTANT:
                   items-center keeps cards narrow and centered.
                   ============================================================ -->

<div class="flex w-full h-full {fitViewport ? 'min-h-0' : 'min-h-20'} flex-col gap-1">
  {#each cellLessons as lesson (lesson.id)}
    <TimetableCard
      {lesson}
      timeSlot={slot}
      {viewMode}
      {locale}
      {getSubjectName}
      {getTeacherName}
      {getClassName}
      {getRoomName}
      subjectColor={getSubjectColor(lesson.subject_id)}
      compact={fitViewport}
      onDelete={onDeleteLesson}
      onEdit={onEditLesson}
      ondragstart={(e) => handleDragStart(e, lesson)}
    />
  {/each}
</div>
            {:else}
              <!-- ============================================================
                   EMPTY CELL
                   ============================================================ -->

              <button
                type="button"
                onclick={() =>
                  onCellClick(
                    day.value,
                    slot.id,
                  )}
                class="
                  flex
                  h-full
                  min-h-17.5
                  w-full
                  cursor-pointer
                  items-center
                  justify-center
                  rounded-md
                  border
                  border-dashed
                  border-transparent
                  p-2
                  text-transparent
                  hover:border-primary/40
                  hover:bg-primary/5
                  hover:text-primary
                  transition-all
                  duration-200
                "
                disabled={
                  viewMode !==
                    "class" ||
                  !isAvailable
                }
                title={
                  locale === "ar"
                    ? "إضافة حصة جديدة"
                    : "Schedule new lesson"
                }
              >
                <Plus
                  size={16}
                  class="
                  "
                />
              </button>
            {/if}
          </div>
        {/each}
      {/if}
    {/each}
  </div>
</div>

<style>
  /* ==========================================================================
     GRID
     ========================================================================== */

  .schedule-grid > :global(*) {
    min-width: 0;
  }

  /* ==========================================================================
     CELL
     ========================================================================== */

  .timetable-cell {
    min-width: 0;
    overflow: hidden;
    background-color: var(--color-card);
    transition: background-color 150ms ease;
  }

  /*
   * Keep the card centered even when a cell is significantly wider
   * than the reference card.
   */
  .timetable-cell > div {
    min-width: 0;
  }

  /* ==========================================================================
     UNAVAILABLE
     ========================================================================== */

  .schedule-unavailable {
    background-color: var(--color-muted);
    background-image:
      repeating-linear-gradient(
        -45deg,
        transparent,
        transparent 7px,
        color-mix(
          in srgb,
          var(--color-muted-foreground) 8%,
          transparent
        ) 7px,
        color-mix(
          in srgb,
          var(--color-muted-foreground) 8%,
          transparent
        ) 9px
      );
  }

  /* ==========================================================================
     SMALL VIEWPORT
     ========================================================================== */

  @media (max-width: 640px) {
    .schedule-grid {
      min-width: 760px;
    }
  }

  /* ==========================================================================
     REDUCED MOTION
     ========================================================================== */

  @media (prefers-reduced-motion: reduce) {
    .timetable-cell,
    .lesson-card-body {
      transition: none;
    }
  }
</style>