<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Input } from "@orbis/ui/input";
import { FormField } from "@orbis/ui";
import { Button } from "@orbis/ui/button";
import { CalendarDays, Plus, Trash2 } from "@lucide/svelte";
import { getLocale } from "$lib/paraglide/runtime";

import { onboardingState } from "../state.svelte";
import type { TimeSlot } from "../types";

let newSlotName = $state("");
let newSlotNameAr = $state("");
let newSlotStart = $state("12:15");
let newSlotEnd = $state("13:00");
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

function validateSlotTime(slot: TimeSlot) {
  if (getDuration(slot.start_time, slot.end_time) > 0) return;
  slot.end_time = slot.start_time;
  onboardingState.academics.timeSlots = [...onboardingState.academics.timeSlots];
  onboardingState.save();
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
      slotDays.length === current.length && slotDays.every((value) => current.includes(value));
    if (followsGlobalDays) slot.applicable_days = [...next];
  }

  onboardingState.academics.workingDays = next;
  onboardingState.academics.timeSlots = [...onboardingState.academics.timeSlots];
  onboardingState.save();
}

function toggleSlotDay(slot: TimeSlot, day: number) {
  const current = slot.applicable_days ?? onboardingState.academics.workingDays;
  if (current.includes(day) && current.length === 1) return;
  slot.applicable_days = current.includes(day)
    ? current.filter((value) => value !== day)
    : [...current, day].sort((a, b) => a - b);
  onboardingState.academics.timeSlots = [...onboardingState.academics.timeSlots];
  onboardingState.save();
}

function handleAddSlot() {
  if (!newSlotName.trim() || !newSlotNameAr.trim() || newSlotDuration <= 0) return;
  const isBreak = newSlotType === "break";
  if (onboardingState.addTimeSlot(newSlotName, newSlotNameAr, newSlotStart, newSlotEnd, {
    isBreak,
    breakType: isBreak ? newBreakType : undefined,
  })) {
    newSlotName = "";
    newSlotNameAr = "";
    newSlotType = "lesson";
  }
}

function removeSlot(slot: TimeSlot) {
  const index = onboardingState.academics.timeSlots.indexOf(slot);
  if (index >= 0) onboardingState.removeTimeSlot(index);
}
</script>

<div class="w-full space-y-5 text-start">
  <section class="rounded-xl border border-border bg-card p-4">
    <div class="flex items-start gap-3">
      <CalendarDays size={18} class="mt-0.5 shrink-0 text-primary" />
      <div class="min-w-0 flex-1">
        <h3 class="text-sm font-bold text-foreground">{m.common_onboarding_working_days_title()}</h3>
        <p class="mt-1 text-xs leading-relaxed text-muted-foreground">{m.common_onboarding_working_days_desc()}</p>
        <div class="mt-3 flex flex-wrap gap-2">
          {#each dayLabels as day (day.value)}
            <button
              type="button"
              class="rounded-md border px-3 py-1.5 text-xs font-semibold transition-colors cursor-pointer {onboardingState.academics.workingDays.includes(day.value) ? 'border-primary bg-primary/10 text-primary' : 'border-border bg-background text-muted-foreground hover:bg-muted'}"
              aria-pressed={onboardingState.academics.workingDays.includes(day.value)}
              onclick={() => toggleWorkingDay(day.value)}
            >{day.label}</button>
          {/each}
        </div>
      </div>
    </div>
  </section>

  <section class="rounded-xl border border-border bg-card p-4 space-y-4">
    <div class="flex items-center justify-between gap-3 border-b border-border pb-3">
      <div>
        <h3 class="text-sm font-bold text-foreground">{m.common_onboarding_schedule_title()}</h3>
        <p class="mt-1 text-xs text-muted-foreground">{m.common_onboarding_schedule_desc()}</p>
      </div>
      <span class="shrink-0 rounded-full bg-primary/10 px-2.5 py-1 text-xs font-bold text-primary">{onboardingState.academics.timeSlots.length}</span>
    </div>

    <div class="grid grid-cols-1 gap-3 rounded-lg border border-primary/20 bg-primary/5 p-3 sm:grid-cols-2 lg:grid-cols-4">
      <FormField label={m.common_onboarding_schedule_name_ar()} required>{#snippet children()}<Input bind:value={newSlotNameAr} placeholder={m.common_onboarding_schedule_name_ar()} class="h-9 text-xs" />{/snippet}</FormField>
      <FormField label={m.common_onboarding_schedule_name_en()} required>{#snippet children()}<Input bind:value={newSlotName} placeholder={m.common_onboarding_schedule_name_en()} class="h-9 text-xs text-left" style="direction: ltr;" />{/snippet}</FormField>
      <FormField label={m.academics_schedule_start_time()} required>{#snippet children()}<Input type="time" bind:value={newSlotStart} class="h-9 text-xs font-mono" />{/snippet}</FormField>
      <FormField label={m.academics_schedule_end_time()} required>{#snippet children()}<Input type="time" bind:value={newSlotEnd} class="h-9 text-xs font-mono" />{/snippet}</FormField>
      <FormField label={m.common_onboarding_lesson_type()} required>{#snippet children()}<select bind:value={newSlotType} class="h-9 rounded-md border border-border bg-background px-2 text-xs text-foreground">
        <option value="lesson">{m.common_onboarding_lesson_type()}</option>
        <option value="break">{m.common_onboarding_break_type()}</option>
      </select>{/snippet}</FormField>
      {#if newSlotType === "break"}
        <FormField label={m.common_onboarding_break_type()} required><select bind:value={newBreakType} class="h-9 rounded-md border border-border bg-background px-2 text-xs text-foreground sm:col-span-2">
          <option value="recess">{m.common_onboarding_break_recess()}</option>
          <option value="lunch">{m.common_onboarding_break_lunch()}</option>
          <option value="prayer">{m.common_onboarding_break_prayer()}</option>
          <option value="custom">{m.common_onboarding_break_custom()}</option>
        </select></FormField>
      {/if}
      <Button class="h-9 gap-1.5 text-xs sm:col-span-2 lg:col-span-1" onclick={handleAddSlot} disabled={!newSlotName.trim() || !newSlotNameAr.trim() || newSlotDuration <= 0}>
        <Plus size={14} /> {m.common_onboarding_add_period()}
      </Button>
    </div>

    <div class="grid grid-cols-1 gap-3 md:grid-cols-2">
      {#each onboardingState.academics.timeSlots as slot, index (slot.name + slot.start_time + index)}
        {@const duration = getDuration(slot.start_time, slot.end_time)}
        <article class="rounded-lg border border-border/70 bg-background p-3 space-y-3">
          <div class="flex items-center justify-between gap-2">
            <span class="flex h-6 w-6 items-center justify-center rounded-full bg-primary/10 text-[10px] font-bold text-primary">{index + 1}</span>
            <span class="text-xs font-mono text-muted-foreground">{m.common_titlebar_timer_minutes({ minutes: duration })}</span>
            <button type="button" class="ms-auto text-muted-foreground hover:text-destructive cursor-pointer" aria-label={m.common_onboarding_delete_period()} onclick={() => removeSlot(slot)}><Trash2 size={14} /></button>
          </div>
          <div class="grid grid-cols-1 gap-2 sm:grid-cols-2">
            <Input bind:value={slot.name_ar} class="h-8 text-xs font-bold" onchange={() => onboardingState.save()} />
            <Input bind:value={slot.name} class="h-8 text-xs text-left" style="direction: ltr;" onchange={() => onboardingState.save()} />
            <Input type="time" bind:value={slot.start_time} class="h-8 text-xs font-mono" onchange={() => { validateSlotTime(slot); onboardingState.save(); }} />
            <Input type="time" bind:value={slot.end_time} class="h-8 text-xs font-mono" onchange={() => { validateSlotTime(slot); onboardingState.save(); }} />
          </div>
          {#if slot.is_break}
            <select bind:value={slot.break_type} class="h-8 w-full rounded-md border border-border bg-background px-2 text-xs text-foreground" onchange={() => { slot.is_break = true; onboardingState.save(); }}>
              <option value="recess">{m.common_onboarding_break_recess()}</option>
              <option value="lunch">{m.common_onboarding_break_lunch()}</option>
              <option value="prayer">{m.common_onboarding_break_prayer()}</option>
              <option value="custom">{m.common_onboarding_break_custom()}</option>
            </select>
          {/if}
          <div class="flex flex-wrap gap-1">
            {#each dayLabels as day (day.value)}
              <button type="button" class="rounded border px-2 py-1 text-[10px] font-semibold cursor-pointer {(slot.applicable_days ?? onboardingState.academics.workingDays).includes(day.value) ? 'border-primary bg-primary/10 text-primary' : 'border-border bg-muted/30 text-muted-foreground'}" aria-pressed={(slot.applicable_days ?? onboardingState.academics.workingDays).includes(day.value)} onclick={() => toggleSlotDay(slot, day.value)}>{day.label}</button>
            {/each}
          </div>
        </article>
      {:else}
        <div class="rounded-lg border border-dashed border-border p-8 text-center text-xs text-muted-foreground md:col-span-2">{m.common_onboarding_no_periods()}</div>
      {/each}
    </div>
  </section>
</div>
