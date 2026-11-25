<script lang="ts">
import type { TimeSlotDto } from "$lib/api";
import { getWeekdayOptions } from "$lib/core/i18n";
import { Badge } from "@orbis/ui/badge";
import { Button } from "@orbis/ui/button";
import { ClockRotateRight, Heart, Pencil, Trash } from "@orbis/icons";
import * as m from "$lib/paraglide/messages";
import { groupSlotsByPeriod, parseTimeToMinutes } from "./time-slot-helpers";

interface Props {
  timeSlots: TimeSlotDto[];
  isSubmitting: boolean;
  onEdit: (slot: TimeSlotDto) => void;
  onDelete: (slotId: string) => void;
  onDeleteGroup: (slots: TimeSlotDto[]) => void;
}

let {
  timeSlots = [],
  isSubmitting = false,
  onEdit,
  onDelete,
  onDeleteGroup,
}: Props = $props();

const weekdays = getWeekdayOptions();
const weekdayOrder = new Map(weekdays.map((day, idx) => [day.value, idx]));

let groupBy = $state<"period" | "day">("period");
let selectedDay = $state<number | "all">("all");

const sortedSlots = $derived(
  [...timeSlots].sort(
    (left, right) =>
      (weekdayOrder.get(left.day_of_week) ?? Number.MAX_SAFE_INTEGER) -
        (weekdayOrder.get(right.day_of_week) ?? Number.MAX_SAFE_INTEGER) ||
      left.slot_index - right.slot_index,
  ),
);

const grouped = $derived(groupSlotsByPeriod(sortedSlots));

const availableDays = $derived(
  [...new Set(sortedSlots.map((slot) => slot.day_of_week))].sort(
    (a, b) => (weekdayOrder.get(a) ?? 99) - (weekdayOrder.get(b) ?? 99),
  ),
);

const filteredSlots = $derived(
  selectedDay === "all"
    ? sortedSlots
    : sortedSlots.filter((slot) => slot.day_of_week === selectedDay),
);

function dayLabel(day: number): string {
  return weekdays.find((item) => item.value === day)?.label ?? String(day);
}

function slotDuration(start: string, end: string): number {
  return (
    parseTimeToMinutes(end.slice(0, 5)) - parseTimeToMinutes(start.slice(0, 5))
  );
}
</script>

<section class="space-y-2.5">
  <div class="flex flex-wrap items-center justify-between gap-2">
    <div class="flex items-center gap-2">
      <h3 class="text-xs font-semibold text-foreground">
        {m.academics_schedule_current_slots()}
      </h3>
      <Badge variant="secondary">{timeSlots.length}</Badge>
    </div>
    {#if timeSlots.length > 0}
      <div class="flex items-center gap-1.5">
        <div class="inline-flex rounded-lg border border-border/60 bg-background p-0.5 text-[11px]">
          <button
            type="button"
            class="rounded-md px-2 py-1 font-medium transition-colors {groupBy === 'period'
              ? 'bg-primary text-primary-foreground'
              : 'text-muted-foreground hover:text-foreground'}"
            onclick={() => (groupBy = "period")}
          >
            {m.academics_schedule_group_by_period()}
          </button>
          <button
            type="button"
            class="rounded-md px-2 py-1 font-medium transition-colors {groupBy === 'day'
              ? 'bg-primary text-primary-foreground'
              : 'text-muted-foreground hover:text-foreground'}"
            onclick={() => (groupBy = "day")}
          >
            {m.academics_schedule_group_by_day()}
          </button>
        </div>
        <Button
          type="button"
          variant="ghost"
          size="sm"
          class="h-7 px-2 text-[11px] text-destructive hover:bg-destructive/10"
          disabled={isSubmitting}
          onclick={() => onDeleteGroup(timeSlots)}
        >
          {m.academics_schedule_clear_all()}
        </Button>
      </div>
    {/if}
  </div>

  {#if sortedSlots.length === 0}
    <div class="flex flex-col items-center gap-2 rounded-xl border border-dashed border-border/60 p-8 text-center">
      <ClockRotateRight size={22} class="text-muted-foreground/70" />
      <p class="text-xs text-muted-foreground">{m.academics_schedule_no_slots()}</p>
    </div>
  {:else if groupBy === "period"}
    <div class="space-y-2">
      {#each grouped as group (group.key)}
        <article
          class="rounded-lg border border-border/60 bg-card p-3 {group.isBreak
            ? 'border-amber-500/30 bg-amber-500/[0.03]'
            : ''}"
        >
          <div class="flex items-center justify-between gap-3">
            <div class="flex min-w-0 items-center gap-3">
              <span class="flex size-8 shrink-0 items-center justify-center rounded-lg bg-muted/50 text-xs font-bold text-foreground">
                {#if group.isBreak}
                  <Heart size={14} class="text-amber-600" />
                {:else}
                  {group.slotIndex}
                {/if}
              </span>
              <div class="min-w-0">
                <div class="flex items-center gap-1.5">
                  <p class="truncate text-xs font-semibold text-foreground">
                    {group.nameAr ||
                      group.nameEn ||
                      m.academics_schedule_placeholder_period({
                        number: group.slotIndex,
                      })}
                  </p>
                  <Badge variant="outline" class="h-4 px-1.5 text-[10px]">
                    {m.academics_schedule_days_count({
                      count: group.slots.length,
                    })}
                  </Badge>
                </div>
                <p class="mt-0.5 font-mono text-[10px] text-muted-foreground">
                  {group.startTime}–{group.endTime} · {slotDuration(
                    group.startTime,
                    group.endTime,
                  )}m
                </p>
              </div>
            </div>
            <div class="flex shrink-0 items-center gap-1">
              {#if group.slots.length === 1}
                <Button
                  type="button"
                  variant="ghost"
                  size="icon"
                  class="size-7"
                  onclick={() => onEdit(group.slots[0])}
                  aria-label={m.academics_schedule_edit()}
                >
                  <Pencil size={13} />
                </Button>
              {/if}
              <Button
                type="button"
                variant="ghost"
                size="icon"
                class="size-7 text-destructive hover:bg-destructive/10"
                disabled={isSubmitting}
                onclick={() => onDeleteGroup(group.slots)}
                aria-label={m.academics_schedule_delete_group()}
              >
                <Trash size={13} />
              </Button>
            </div>
          </div>
          <div class="mt-2 flex flex-wrap gap-1">
            {#each group.slots as slot (slot.id)}
              <button
                type="button"
                class="rounded border border-border/50 bg-background px-2 py-0.5 text-[10px] text-muted-foreground transition-colors hover:border-primary/40 hover:text-foreground"
                onclick={() => onEdit(slot)}
              >
                {dayLabel(slot.day_of_week)}
              </button>
            {/each}
          </div>
        </article>
      {/each}
    </div>
  {:else}
    <div class="flex flex-wrap gap-1.5">
      <button
        type="button"
        class="rounded-md border px-2.5 py-1 text-[11px] font-medium transition-colors {selectedDay === 'all'
          ? 'border-primary bg-primary/10 text-primary'
          : 'border-border/60 bg-background text-muted-foreground hover:bg-muted'}"
        onclick={() => (selectedDay = "all")}
      >
        {m.academics_schedule_all_days()}
      </button>
      {#each availableDays as dayValue (dayValue)}
        <button
          type="button"
          class="rounded-md border px-2.5 py-1 text-[11px] font-medium transition-colors {selectedDay === dayValue
            ? 'border-primary bg-primary/10 text-primary'
            : 'border-border/60 bg-background text-muted-foreground hover:bg-muted'}"
          onclick={() => (selectedDay = dayValue)}
        >
          {dayLabel(dayValue)}
        </button>
      {/each}
    </div>
    <div class="space-y-2">
      {#each filteredSlots as slot (slot.id)}
        <article
          class="flex items-center justify-between gap-3 rounded-lg border border-border/60 bg-card p-3 {slot.is_break
            ? 'border-amber-500/30 bg-amber-500/[0.03]'
            : ''}"
        >
          <div class="flex min-w-0 items-center gap-3">
            <span class="flex size-8 shrink-0 items-center justify-center rounded-lg bg-muted/50 text-xs font-bold text-foreground">
              {#if slot.is_break}
                <Heart size={14} class="text-amber-600" />
              {:else}
                {slot.slot_index}
              {/if}
            </span>
            <div class="min-w-0">
              <p class="truncate text-xs font-semibold text-foreground">
                {dayLabel(slot.day_of_week)} · {slot.name_ar ||
                  slot.name ||
                  m.academics_schedule_placeholder_period({
                    number: slot.slot_index,
                  })}
              </p>
              <p class="mt-0.5 font-mono text-[10px] text-muted-foreground">
                {slot.start_time.slice(0, 5)}–{slot.end_time.slice(0, 5)} ·
                {slotDuration(slot.start_time, slot.end_time)}m
              </p>
            </div>
          </div>
          <div class="flex shrink-0 items-center gap-1">
            <Button
              type="button"
              variant="ghost"
              size="icon"
              class="size-7"
              onclick={() => onEdit(slot)}
              aria-label={m.academics_schedule_edit()}
            >
              <Pencil size={13} />
            </Button>
            <Button
              type="button"
              variant="ghost"
              size="icon"
              class="size-7 text-destructive hover:bg-destructive/10"
              disabled={isSubmitting}
              onclick={() => onDelete(slot.id)}
              aria-label={m.common_action_delete()}
            >
              <Trash size={13} />
            </Button>
          </div>
        </article>
      {/each}
    </div>
  {/if}
</section>
