<!--
  Step 2: Daily Schedule & Time Slots Setup
  Working days configuration and daily periods schedule with clean, elegant layout.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Input } from "@orbis/ui/input";
import { FormField } from "@orbis/ui";
import { Button } from "@orbis/ui/button";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import { Grid, Plus, Trash } from "@orbis/icons";
import { getLocale } from "$lib/paraglide/runtime";

import { onboardingState } from "../state.svelte";
import type { TimeSlot } from "../types";
import { l } from "$lib/core/i18n";

let newSlotName = $state("");
let newSlotNameAr = $state("");
let newSlotStart = $state("08:00");
let newSlotEnd = $state("08:40");
let newSlotType = $state<"lesson" | "break">("lesson");
let newBreakType = $state<"recess" | "lunch" | "prayer" | "custom">("recess");

const dayLabels = $derived(
  [7, 1, 2, 3, 4, 5, 6].map((day) => ({
    value: day,
    label: new Intl.DateTimeFormat(getLocale(), { weekday: "short" }).format(
      new Date(Date.UTC(2023, 0, day)),
    ),
  })),
);

const newSlotDuration = $derived(getDuration(newSlotStart, newSlotEnd));

function getDuration(start: string, end: string) {
  const [startHour, startMinute] = start.split(":").map(Number);
  const [endHour, endMinute] = end.split(":").map(Number);
  return endHour * 60 + endMinute - (startHour * 60 + startMinute);
}

function toggleWorkingDay(day: number) {
  const current = onboardingState.academics.workingDays;
  if (current.includes(day) && current.length === 1) return;
  const next = current.includes(day)
    ? current.filter((value) => value !== day)
    : [...current, day].sort((a, b) => a - b);

  for (const slot of onboardingState.academics.timeSlots) {
    const slotDays = slot.applicable_days ?? current;
    const followsGlobalDays =
      slotDays.length === current.length &&
      slotDays.every((value) => current.includes(value));
    if (followsGlobalDays) slot.applicable_days = [...next];
  }

  onboardingState.academics.workingDays = next;
  onboardingState.academics.timeSlots = [...onboardingState.academics.timeSlots];
  onboardingState.save();
}

function handleAddSlot() {
  if (!newSlotNameAr.trim() && !newSlotName.trim()) return;
  const isBreak = newSlotType === "break";
  const nameAr = newSlotNameAr.trim() || newSlotName.trim();
  const nameEn = newSlotName.trim() || newSlotNameAr.trim();

  if (
    onboardingState.addTimeSlot(
      nameEn,
      nameAr,
      newSlotStart,
      newSlotEnd,
      {
        isBreak,
        breakType: isBreak ? newBreakType : undefined,
      },
    )
  ) {
    newSlotName = "";
    newSlotNameAr = "";
    newSlotType = "lesson";
  }
}

function removeSlot(slot: TimeSlot) {
  const index = onboardingState.academics.timeSlots.indexOf(slot);
  if (index >= 0) onboardingState.removeTimeSlot(index);
}

const breakTypeLabels: Record<string, () => string> = {
  recess: () => m.common_onboarding_break_recess(),
  lunch: () => m.common_onboarding_break_lunch(),
  prayer: () => m.common_onboarding_break_prayer(),
  custom: () => m.common_onboarding_break_custom(),
};
</script>

<div class="w-full h-full flex flex-col min-h-0 space-y-3 text-start animate-in fade-in duration-300">

  <!-- Top Working Days Bar -->
  <div class="rounded-2xl border border-border/70 bg-card p-3.5 flex flex-col sm:flex-row items-center justify-between gap-3 shadow-2xs shrink-0">
    <span class="text-xs font-bold text-foreground">{m.common_onboarding_working_days_title()}</span>

    <div class="flex flex-wrap gap-1.5">
      {#each dayLabels as day (day.value)}
        <button
          type="button"
          class="rounded-xl border px-3.5 py-1 text-[11px] font-bold transition-all duration-150 cursor-pointer {onboardingState.academics.workingDays.includes(day.value) ? 'border-primary/40 bg-primary/10 text-primary shadow-2xs' : 'border-border/60 bg-muted/20 text-muted-foreground hover:bg-muted/40'}"
          aria-pressed={onboardingState.academics.workingDays.includes(day.value)}
          onclick={() => toggleWorkingDay(day.value)}
        >{day.label}</button>
      {/each}
    </div>
  </div>

  <!-- Main Schedule Grid -->
  <div class="flex-1 min-h-0 grid grid-cols-1 md:grid-cols-12 gap-3.5">

    <!-- Add Slot Form (4 cols) -->
    <div class="md:col-span-4 rounded-2xl border border-border/70 bg-card p-4 flex flex-col justify-between shadow-2xs space-y-3 overflow-y-auto">
      <div class="flex items-center justify-between border-b border-border/50 pb-2 shrink-0">
        <h4 class="text-xs font-bold text-foreground">{m.common_onboarding_add_period()}</h4>
        <span class="text-[10px] text-muted-foreground font-mono">
          {newSlotDuration > 0 ? `${newSlotDuration} min` : ""}
        </span>
      </div>

      <div class="space-y-2.5 flex-1">
        <FormField label={m.common_onboarding_schedule_name_ar()} required>
          {#snippet children()}
            <Input bind:value={newSlotNameAr} placeholder="الحصة الأولى" class="h-8 text-xs font-bold font-sans bg-background" />
          {/snippet}
        </FormField>

        <FormField label={m.common_onboarding_schedule_name_en()}>
          {#snippet children()}
            <Input bind:value={newSlotName} placeholder="Period 1" class="h-8 text-xs font-sans bg-background text-left" style="direction: ltr;" />
          {/snippet}
        </FormField>

        <div class="grid grid-cols-2 gap-2">
          <FormField label={m.academics_schedule_start_time()} required>
            {#snippet children()}
              <Input type="time" bind:value={newSlotStart} class="h-8 text-xs font-mono bg-background" />
            {/snippet}
          </FormField>
          <FormField label={m.academics_schedule_end_time()} required>
            {#snippet children()}
              <Input type="time" bind:value={newSlotEnd} class="h-8 text-xs font-mono bg-background" />
            {/snippet}
          </FormField>
        </div>

        <div class="grid grid-cols-2 gap-2">
          <FormField label={m.common_onboarding_lesson_type()} required>
            {#snippet children()}
              <Select
                type="single"
                value={newSlotType}
                onValueChange={(v) => { if (v === "lesson" || v === "break") newSlotType = v; }}
              >
                <SelectTrigger class="h-8 text-xs w-full bg-background">
                  {newSlotType === "lesson" ? m.common_onboarding_lesson_type() : m.common_onboarding_break_type()}
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="lesson">{m.common_onboarding_lesson_type()}</SelectItem>
                  <SelectItem value="break">{m.common_onboarding_break_type()}</SelectItem>
                </SelectContent>
              </Select>
            {/snippet}
          </FormField>

          {#if newSlotType === "break"}
            <FormField label={m.common_onboarding_break_type()} required>
              {#snippet children()}
                <Select
                  type="single"
                  value={newBreakType}
                  onValueChange={(v) => { if (v) newBreakType = v as any; }}
                >
                  <SelectTrigger class="h-8 text-xs w-full bg-background">
                    {breakTypeLabels[newBreakType]?.() || newBreakType}
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="recess">{m.common_onboarding_break_recess()}</SelectItem>
                    <SelectItem value="lunch">{m.common_onboarding_break_lunch()}</SelectItem>
                    <SelectItem value="prayer">{m.common_onboarding_break_prayer()}</SelectItem>
                    <SelectItem value="custom">{m.common_onboarding_break_custom()}</SelectItem>
                  </SelectContent>
                </Select>
              {/snippet}
            </FormField>
          {/if}
        </div>
      </div>

      <Button size="sm" class="h-8 w-full gap-1.5 text-xs font-bold cursor-pointer shrink-0" onclick={handleAddSlot} disabled={(!newSlotName.trim() && !newSlotNameAr.trim()) || newSlotDuration <= 0}>
        <Plus size={13} /> {m.common_onboarding_add_period()}
      </Button>
    </div>

    <!-- Slots Grid / List (8 cols) -->
    <div class="md:col-span-8 rounded-2xl border border-border/70 bg-card p-4 flex flex-col justify-between shadow-2xs space-y-2 min-h-0">
      <div class="flex items-center justify-between border-b border-border/50 pb-2 shrink-0">
        <h4 class="text-xs font-bold text-foreground">{m.common_onboarding_schedule_title()}</h4>
        <span class="rounded-full bg-primary/10 px-2.5 py-0.5 text-[11px] font-bold text-primary font-mono">
          {onboardingState.academics.timeSlots.length} فترات
        </span>
      </div>

      <!-- Scrollable List -->
      <div class="flex-1 min-h-0 overflow-y-auto scrollbar-hide divide-y divide-border/40 pe-1">
        {#each onboardingState.academics.timeSlots as slot, index (slot.name + slot.start_time + index)}
          {@const duration = getDuration(slot.start_time, slot.end_time)}
          <div class="py-2 flex items-center justify-between gap-3 text-xs hover:bg-muted/10 px-2 rounded-xl transition-colors">
            <div class="flex items-center gap-2.5 min-w-0">
              <span class="flex size-5.5 shrink-0 items-center justify-center rounded-full {slot.is_break ? 'bg-amber-500/10 text-amber-600 dark:text-amber-400' : 'bg-primary/10 text-primary'} text-[10px] font-bold font-mono">
                {index + 1}
              </span>
              <div class="flex items-center gap-2 min-w-0">
                <span class="font-bold text-foreground text-xs truncate">{l(slot.name_ar, slot.name)}</span>
                {#if slot.is_break}
                  <span class="text-[9.5px] px-2 py-0.5 rounded-full bg-amber-500/15 text-amber-600 dark:text-amber-400 font-medium shrink-0">
                    {breakTypeLabels[slot.break_type || "recess"]?.() || "استراحة"}
                  </span>
                {/if}
              </div>
            </div>

            <div class="flex items-center gap-2.5 shrink-0 font-mono text-xs text-muted-foreground">
              <span class="font-medium text-foreground text-[11px]">{slot.start_time} - {slot.end_time}</span>
              <span class="px-2 py-0.5 rounded-md bg-muted/60 text-[10px] font-medium">{duration} min</span>
              <button
                type="button"
                class="text-muted-foreground hover:text-destructive p-1 rounded transition-colors cursor-pointer"
                aria-label={m.common_onboarding_delete_period()}
                onclick={() => removeSlot(slot)}
              >
                <Trash size={13} />
              </button>
            </div>
          </div>
        {:else}
          <div class="py-12 text-center text-xs text-muted-foreground">
            {m.common_onboarding_no_periods()}
          </div>
        {/each}
      </div>
    </div>

  </div>

</div>

