<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui-svelte/button";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { Timetable } from "@orbis/ui-svelte/timetable";
import { getLocale } from "$lib/paraglide/runtime";
import { Clock } from "@lucide/svelte";

import ErrorState from "$lib/shared/components/error-state.svelte";

interface Props {
  isLoading: boolean;
  error: string | null;
  timeSlots: any[];
  weekdays: any[];
  lessons: any[];
  draftLessons: any[];
  previewDraft: boolean;
  viewMode: "class" | "teacher";
  getSubjectName: (id: string) => string;
  getTeacherName: (id: string) => string;
  getClassName: (id: string) => string;
  getSubjectColor: (id: string) => string;
  onCellClick: (day: number, slotId: string) => void;
  onDeleteLesson: (id: string) => void;
  onEditLesson?: (lesson: any) => void;
  onMoveLesson: (lesson: any, day: number, slotId: string) => void;
  onRetry: () => void;
}

let {
  isLoading,
  error,
  timeSlots,
  weekdays,
  lessons,
  draftLessons,
  previewDraft,
  viewMode,
  getSubjectName,
  getTeacherName,
  getClassName,
  getSubjectColor,
  onCellClick,
  onDeleteLesson,
  onEditLesson,
  onMoveLesson,
  onRetry,
}: Props = $props();
</script>

{#if isLoading}
  <div class="flex-1 w-full flex flex-col bg-card border border-border rounded-lg overflow-hidden">
    <Skeleton class="h-10 w-full rounded-none" />
    <Skeleton class="flex-1 w-full rounded-none" />
  </div>
{:else if error}
  <ErrorState 
    title={m.common_schedule_page_error()} 
    message={error} 
    onRetry={onRetry} 
  />
{:else if !timeSlots || timeSlots.length === 0}
  <div class="flex-1 flex flex-col items-center justify-center p-12 border border-border/60 bg-card rounded-2xl text-center gap-4 max-w-lg mx-auto my-8">
    <div class="p-3 bg-primary/10 rounded-full text-primary">
      <Clock size={28} />
    </div>
    <div class="flex flex-col gap-1">
      <h3 class="font-bold text-sm text-foreground">
        {m.common_schedule_page_no_timeslots_title()}
      </h3>
      <p class="text-xs text-muted-foreground leading-relaxed">
        {m.common_schedule_page_no_timeslots_desc()}
      </p>
    </div>
  </div>
{:else}
  <div class="flex-1 min-h-0 w-full flex flex-col">
    <Timetable
      {timeSlots}
      {weekdays}
      {lessons}
      {draftLessons}
      {previewDraft}
      {viewMode}
      locale={getLocale()}
      {getSubjectName}
      {getTeacherName}
      {getClassName}
      {getSubjectColor}
      onCellClick={onCellClick}
      onDeleteLesson={onDeleteLesson}
      onEditLesson={onEditLesson}
      onMoveLesson={onMoveLesson}
    />
  </div>
{/if}
