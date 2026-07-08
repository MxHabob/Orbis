<script lang="ts">
import { Button } from "@orbis/ui-svelte/button";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { Timetable } from "@orbis/ui-svelte/timetable";
import { getLocale } from "$lib/paraglide/runtime";
import { Clock } from "lucide-svelte";
import * as m from "$lib/paraglide/messages";

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
  onMoveLesson,
  onRetry,
}: Props = $props();
</script>

{#if isLoading}
  <div class="flex-1 w-full p-4 flex flex-col gap-3 bg-card border border-border rounded-lg">
    <Skeleton class="h-10 w-full" />
    <Skeleton class="h-20 w-full" />
    <Skeleton class="h-20 w-full" />
  </div>
{:else if error}
  <div class="flex flex-col items-center justify-center p-8 border border-destructive/20 bg-destructive/5 rounded-xl text-center gap-3">
    <p class="text-xs text-destructive font-semibold">{m.schedule_page_error()}</p>
    <p class="text-xs text-muted-foreground">{error}</p>
    <Button variant="outline" class="text-xs" onclick={onRetry}>{m.error_retry()}</Button>
  </div>
{:else if !timeSlots || timeSlots.length === 0}
  <div class="flex-1 flex flex-col items-center justify-center p-12 border border-border/60 bg-card rounded-2xl text-center gap-4 max-w-lg mx-auto my-8">
    <div class="p-3 bg-primary/10 rounded-full text-primary">
      <Clock size={28} />
    </div>
    <div class="flex flex-col gap-1">
      <h3 class="font-bold text-sm text-foreground">
        {m.schedule_page_no_timeslots_title()}
      </h3>
      <p class="text-xs text-muted-foreground leading-relaxed">
        {m.schedule_page_no_timeslots_desc()}
      </p>
    </div>
  </div>
{:else}
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
    onMoveLesson={onMoveLesson}
  />
{/if}
