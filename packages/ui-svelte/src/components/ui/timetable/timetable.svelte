<script lang="ts">
import { fade } from "svelte/transition";
import { Plus } from "lucide-svelte";
import TimetableCard from "./timetable-card.svelte";
import { onMount } from "svelte";

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
  onCellClick,
  onDeleteLesson,
  onMoveLesson,
  // Add default properties for calendar layout
  weekStart = new Date("2026-02-23T00:00:00"),
}: {
  timeSlots: any[];
  weekdays: any[];
  lessons: any[];
  draftLessons: any[];
  previewDraft: boolean;
  viewMode: "class" | "teacher";
  locale: string;
  getSubjectName: (id: string) => string;
  getTeacherName: (id: string) => string;
  getClassName: (id: string) => string;
  getSubjectColor: (id: string) => string;
  onCellClick: (day: number, slotId: string) => void;
  onDeleteLesson: (id: string) => void;
  onMoveLesson: (lesson: any, targetDay: number, targetSlotId: string) => void;
  weekStart?: Date;
} = $props();

// Local Drag & Drop State
let draggedLesson = $state<any>(null);
let dragOverCell = $state<{ day: number; hour: number } | null>(null);

// Hours to display in the grid: 08:00 to 18:00
const startHour = 8;
const endHour = 18;
const hourHeight = 84; // height of an hour in pixels
const startMin = startHour * 60;

// Real-time indicator state
let currentTime = $state(new Date());
onMount(() => {
  const timer = setInterval(() => {
    currentTime = new Date();
  }, 30000); // update every 30s
  return () => clearInterval(timer);
});

// Time conversion helpers
function timeToMinutes(timeStr: string) {
  if (!timeStr) return 0;
  const [h, m] = timeStr.split(":").map(Number);
  return h * 60 + m;
}

function getLessonTimeDetails(l: any) {
  if (l.start_time && l.end_time) {
    return { start: l.start_time, end: l.end_time };
  }
  const slot = timeSlots.find((s) => String(s.id) === String(l.time_slot_id));
  if (slot) {
    return { start: slot.start_time, end: slot.end_time };
  }
  return { start: "08:00", end: "09:00" };
}

// Combine real and draft lessons
const allLessons = $derived.by(() => {
  const all = [...lessons];
  if (previewDraft) {
    all.push(...draftLessons);
  }
  return all;
});

// Calculate position and overlaps of cards for a column
function getPositionedLessonsForDay(dayValue: number) {
  const dayLessons = allLessons.filter(
    (l) => Number(l.day_of_week) === dayValue,
  );

  const positioned: {
    lesson: any;
    top: number;
    height: number;
    width: number;
    left: number;
    zIndex: number;
    color: string;
  }[] = [];

  // Sort by start time
  const sorted = [...dayLessons].sort((a, b) => {
    const timesA = getLessonTimeDetails(a);
    const timesB = getLessonTimeDetails(b);
    return timeToMinutes(timesA.start) - timeToMinutes(timesB.start);
  });

  const columns: (typeof positioned)[][] = [];

  for (const l of sorted) {
    const times = getLessonTimeDetails(l);
    const startMinVal = timeToMinutes(times.start);
    const endMinVal = timeToMinutes(times.end);

    const top = Math.max(0, ((startMinVal - startMin) / 60) * hourHeight);
    const height = Math.max(30, ((endMinVal - startMinVal) / 60) * hourHeight);
    const color = getSubjectColor(l.subject_id);

    // Overlap detection
    let colIdx = 0;
    while (colIdx < columns.length) {
      const lastInCol = columns[colIdx][columns[colIdx].length - 1];
      const lastTimes = getLessonTimeDetails(lastInCol.lesson);
      const lastEnd = timeToMinutes(lastTimes.end);

      if (startMinVal >= lastEnd) {
        break;
      }
      colIdx++;
    }

    if (colIdx >= columns.length) {
      columns.push([]);
    }

    const item = {
      lesson: l,
      top,
      height,
      width: 100, // default
      left: 0, // default
      zIndex: colIdx + 1,
      color,
    };

    columns[colIdx].push(item);
    positioned.push(item);
  }

  // Offset overlapping columns beautifully (layered look)
  const totalCols = columns.length;
  if (totalCols > 1) {
    for (let colIdx = 0; colIdx < totalCols; colIdx++) {
      for (const item of columns[colIdx]) {
        item.width = 72; // overlap width
        item.left = colIdx * 25; // overlay stagger offset
      }
    }
  }

  return positioned;
}

// Get timeslot that falls in a specific hour range for cell click fallback
function getSlotForHourClick(hour: number) {
  const clickedTime = hour * 60;
  const match = timeSlots.find((s) => {
    const start = timeToMinutes(s.start_time);
    return start >= clickedTime && start < clickedTime + 60;
  });
  return match ? String(match.id) : timeSlots[0]?.id || "1";
}

// Drag and Drop
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

function handleDragOver(e: DragEvent, day: number, hour: number) {
  e.preventDefault();
  if (draggedLesson) {
    dragOverCell = { day, hour };
  }
}

function handleDragLeave() {
  dragOverCell = null;
}

function handleDrop(e: DragEvent, targetDay: number, hour: number) {
  e.preventDefault();
  dragOverCell = null;
  if (!draggedLesson) return;
  const lesson = draggedLesson;
  draggedLesson = null;

  const targetSlotId = getSlotForHourClick(hour);
  if (
    Number(lesson.day_of_week) === targetDay &&
    String(lesson.time_slot_id) === targetSlotId
  ) {
    return;
  }

  onMoveLesson(lesson, targetDay, targetSlotId);
}

// Current time line calculation
const timeIndicatorTop = $derived.by(() => {
  const curHour = currentTime.getHours();
  const curMin = currentTime.getMinutes();
  const totalMin = curHour * 60 + curMin;

  if (totalMin < startMin || totalMin > endHour * 60) {
    return null;
  }
  return ((totalMin - startMin) / 60) * hourHeight;
});

// Calculate the date of a weekday relative to the Monday weekStart
function getDateForDayValue(dayValue: number) {
  const base = new Date(weekStart);
  // Calculate offset from Monday (value 2)
  let offset = dayValue - 2;
  if (offset < 0) offset += 7; // Sunday (1) is index 6

  const d = new Date(base);
  d.setDate(base.getDate() + offset);
  return d;
}

const isCurrentDay = (date: Date) => {
  const today = new Date();
  return (
    date.getDate() === today.getDate() &&
    date.getMonth() === today.getMonth() &&
    date.getFullYear() === today.getFullYear()
  );
};

// Mock all-day events matching the design (Tech Conference, Client Visit)
const allDayEvents = $derived([
  {
    day: 5,
    title: locale === "ar" ? "المؤتمر التقني" : "Tech Conference",
    color: "#a855f7",
  },
  {
    day: 6,
    title: locale === "ar" ? "زيارة العميل" : "Client Visit",
    color: "#f97316",
  },
]);
</script>

<div class="flex-1 w-full bg-white dark:bg-card border border-gray-100 dark:border-border/60 rounded-2xl flex flex-col overflow-hidden shadow-sm">
  
  <!-- Calendar Grid Header -->
  <div class="grid border-b border-gray-100 dark:border-border/50 text-[11px] font-semibold text-gray-500 uppercase select-none"
       style="grid-template-columns: 65px repeat({weekdays.length}, 1fr);">
    <!-- Empty top corner -->
    <div class="h-14 border-r border-gray-100 dark:border-border/50"></div>
    
    <!-- Weekday headers -->
    {#each weekdays as day (day.value)}
      {@const date = getDateForDayValue(day.value)}
      {@const isActive = isCurrentDay(date)}
      <div class="h-14 flex flex-col items-center justify-center border-r border-gray-100 dark:border-border/50 gap-0.5 last:border-r-0">
        <span class="text-[10px] text-gray-400 font-medium">
          {day.label}
        </span>
        <span class="text-sm font-bold flex items-center justify-center w-7 h-7 rounded-full transition-all duration-200
          {isActive ? 'bg-black text-white dark:bg-white dark:text-black font-extrabold' : 'text-gray-700 dark:text-foreground/90'}"
        >
          {date.getDate()}
        </span>
      </div>
    {/each}
  </div>

  <!-- All Day Events Row -->
  <div class="grid border-b border-gray-100 dark:border-border/50 text-[11px] select-none bg-gray-50/50 dark:bg-muted/10 min-h-[40px] items-center"
       style="grid-template-columns: 65px repeat({weekdays.length}, 1fr);">
    <div class="h-full border-r border-gray-100 dark:border-border/50 flex items-center justify-center font-bold text-[9px] text-gray-400 uppercase tracking-wider px-2">
      {locale === "ar" ? "اليوم" : "All day"}
    </div>
    
    {#each weekdays as day (day.value)}
      {@const dayEvents = allDayEvents.filter(e => Number(e.day) === day.value)}
      <div class="h-full border-r border-gray-100 dark:border-border/50 p-1 flex flex-col gap-1 justify-center last:border-r-0">
        {#each dayEvents as ev}
          <div class="px-2 py-0.5 rounded text-[9.5px] font-bold truncate text-ellipsis select-none transition-all flex items-center gap-1.5"
               style="background-color: color-mix(in srgb, {ev.color} 10%, #ffffff); color: color-mix(in srgb, {ev.color} 85%, #000000); border-left: 2px solid {ev.color}">
            <div class="w-1.5 h-1.5 rounded-full" style="background-color: {ev.color}"></div>
            <span>{ev.title}</span>
          </div>
        {/each}
      </div>
    {/each}
  </div>

  <!-- Hour Rows & Timetable Columns -->
  <div class="relative flex-1 overflow-y-auto scrollbar-none" style="height: { (endHour - startHour) * hourHeight }px">
    
    <!-- Time indicator line spanning columns -->
    {#if timeIndicatorTop !== null}
      <div class="absolute w-full flex items-center pointer-events-none z-30" style="top: {timeIndicatorTop}px">
        <!-- Black Time Badge on left axis -->
        <div class="w-[65px] flex justify-end pr-2">
          <span class="bg-black dark:bg-white text-white dark:text-black font-bold text-[9px] px-1 py-0.5 rounded leading-none font-mono tracking-tighter">
            {currentTime.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', hour12: false })}
          </span>
        </div>
        
        <!-- Indicator Line -->
        <div class="flex-1 h-[1px] bg-black/60 dark:bg-white/60 relative">
          <!-- Small dot on current day column -->
          {#each weekdays as day, idx}
            {@const date = getDateForDayValue(day.value)}
            {#if isCurrentDay(date)}
              <div class="absolute w-2.5 h-2.5 rounded-full bg-black dark:bg-white border-2 border-white dark:border-card -top-1"
                   style="left: calc((100% / {weekdays.length}) * {idx} + (100% / {weekdays.length * 2}) - 5px)"></div>
            {/if}
          {/each}
        </div>
      </div>
    {/if}

    <!-- Layout Grid: Time Grid + Weekday Columns -->
    <div class="grid relative h-full bg-grid-pattern" style="grid-template-columns: 65px repeat({weekdays.length}, 1fr);">
      
      <!-- Left hour scale Column -->
      <div class="flex flex-col bg-white dark:bg-card border-r border-gray-100 dark:border-border/50 select-none">
        {#each Array(endHour - startHour) as _, index}
          {@const hr = startHour + index}
          <div class="relative flex justify-end pr-2.5 text-[10px] text-gray-400 font-bold font-mono" style="height: {hourHeight}px; padding-top: 4px;">
            <span>{String(hr).padStart(2, '0')}:00</span>
          </div>
        {/each}
      </div>

      <!-- Days Columns -->
      {#each weekdays as day (day.value)}
        {@const dayLessons = getPositionedLessonsForDay(day.value)}
        
        <!-- Column Wrapper -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div 
          class="relative border-r border-gray-100 dark:border-border/50 h-full last:border-r-0 select-none"
          ondragover={(e) => handleDragOver(e, day.value, startHour)}
          ondragleave={handleDragLeave}
          ondrop={(e) => handleDrop(e, day.value, startHour)}
        >
          <!-- Grid Background Helper Lines inside columns -->
          {#each Array(endHour - startHour) as _, index}
            {@const hr = startHour + index}
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            <div 
              class="w-full border-b border-gray-50 dark:border-border/30 hover:bg-gray-50/20 dark:hover:bg-muted/5 transition-all duration-200 cursor-pointer"
              style="height: {hourHeight}px;"
              onclick={() => onCellClick(day.value, getSlotForHourClick(hr))}
              title={locale === "ar" ? "إضافة حصة جديدة" : "Schedule lesson"}
            >
              <!-- Hover Plus Sign -->
              <div class="w-full h-full flex items-center justify-center opacity-0 hover:opacity-100 text-gray-300 dark:text-gray-600 transition-opacity">
                <Plus size={16} />
              </div>
            </div>
          {/each}

          <!-- Absolute Event Cards inside Columns -->
          {#each dayLessons as item (item.lesson.id)}
            <div 
              class="absolute p-0.5 transition-all duration-300"
              style="top: {item.top}px; height: {item.height}px; width: {item.width}%; left: {item.left}%; z-index: {item.zIndex};"
            >
              <TimetableCard 
                lesson={item.lesson}
                {viewMode}
                {locale}
                {getSubjectName}
                {getTeacherName}
                {getClassName}
                subjectColor={item.color}
                onDelete={onDeleteLesson}
                ondragstart={(e) => handleDragStart(e, item.lesson)}
              />
            </div>
          {/each}
        </div>
      {/each}

    </div>
  </div>
</div>

<style>
  /* Custom scrollbar hides */
  .scrollbar-none::-webkit-scrollbar {
    display: none !important;
  }
  .scrollbar-none {
    -ms-overflow-style: none !important;
    scrollbar-width: none !important;
  }

  /* Grid pattern lines background */
  .bg-grid-pattern {
    background-size: 100% 84px;
    background-image: linear-gradient(to bottom, transparent 83px, #f3f4f6 83px);
  }
  :global(.dark) .bg-grid-pattern {
    background-image: linear-gradient(to bottom, transparent 83px, rgba(255, 255, 255, 0.05) 83px);
  }
</style>
