<!--
  Step 5: Daily Time Slots Setup
  Add, remove, and list school daily periods/lessons.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Input } from "@orbis/ui-svelte/input";
import { Button } from "@orbis/ui-svelte/button";
import { Plus, Trash, CalendarDays } from "@lucide/svelte";
import { getLocale } from "$lib/paraglide/runtime";

import { onboardingState } from "../state.svelte";

let newSlotName = $state("");
let newSlotNameAr = $state("");
let newSlotStart = $state("12:15");
let newSlotEnd = $state("13:00");
let newSlotType = $state("lesson");

function getDuration(start: string, end: string) {
  const [startHour, startMinute] = start.split(":").map(Number);
  const [endHour, endMinute] = end.split(":").map(Number);
  return endHour * 60 + endMinute - (startHour * 60 + startMinute);
}

function validateSlotTime(slot: { start_time: string; end_time: string }) {
  if (getDuration(slot.start_time, slot.end_time) > 0) return;
  slot.end_time = slot.start_time;
  onboardingState.academics.timeSlots = [...onboardingState.academics.timeSlots];
  onboardingState.save();
}

const newSlotDuration = $derived(getDuration(newSlotStart, newSlotEnd));

const dayLabels = $derived(
  [7, 1, 2, 3, 4, 5, 6].map((day) => {
    return {
      value: day,
      label: new Intl.DateTimeFormat(getLocale(), { weekday: "short" }).format(
        new Date(Date.UTC(2023, 0, day)),
      ),
    };
  }),
);

function toggleWorkingDay(day: number) {
  const current = onboardingState.academics.workingDays;
  const next = current.includes(day)
    ? current.length === 1
      ? current
      : current.filter((value) => value !== day)
    : [...current, day].sort((a, b) => a - b);

  for (const slot of onboardingState.academics.timeSlots) {
    const slotDays = slot.applicable_days ?? current;
    const followsGlobalDays =
      slotDays.length === current.length && slotDays.every((value) => current.includes(value));
    if (followsGlobalDays) slot.applicable_days = [...next];
  }

  if (current.includes(day)) {
    if (current.length === 1) return;
    onboardingState.academics.workingDays = next;
  } else {
    onboardingState.academics.workingDays = next;
  }
  onboardingState.academics.timeSlots = [...onboardingState.academics.timeSlots];
  onboardingState.save();
}

function toggleSlotDay(slot: any, day: number) {
  const current = slot.applicable_days ?? onboardingState.academics.workingDays;
  if (current.includes(day)) {
    if (current.length === 1) return;
    slot.applicable_days = current.filter((value: number) => value !== day);
  } else {
    slot.applicable_days = [...current, day].sort((a: number, b: number) => a - b);
  }
  onboardingState.academics.timeSlots = [...onboardingState.academics.timeSlots];
  onboardingState.save();
}

function handleAddTimeSlot() {
  if (newSlotDuration <= 0) return;
  const success = onboardingState.addTimeSlot(
    newSlotName,
    newSlotNameAr,
    newSlotStart,
    newSlotEnd,
    {
      isBreak: newSlotType !== "lesson",
      breakType: newSlotType === "lesson" ? undefined : newSlotType as "recess" | "lunch" | "prayer" | "custom",
    },
  );
  if (success) {
    newSlotName = "";
    newSlotNameAr = "";
    newSlotType = "lesson";
  }
}
</script>

<div class="w-full space-y-6">
  <div class="rounded-xl border border-border bg-card p-4 text-start">
    <div class="flex items-start gap-3">
      <CalendarDays size={18} class="mt-0.5 shrink-0 text-primary" />
      <div class="min-w-0 flex-1">
        <h3 class="text-sm font-bold text-foreground">أيام الدراسة</h3>
        <p class="mt-1 text-xs leading-relaxed text-muted-foreground">
          اختر أيام العمل الفعلية للمؤسسة. يمكن تخصيص أيام كل فترة بشكل مستقل.
        </p>
        <div class="mt-3 flex flex-wrap gap-2">
          {#each dayLabels as day (day.value)}
            <button
              type="button"
              class="rounded-md border px-3 py-1.5 text-xs font-semibold transition-colors cursor-pointer {onboardingState.academics.workingDays.includes(day.value) ? 'border-primary bg-primary/10 text-primary' : 'border-border bg-background text-muted-foreground hover:bg-muted'}"
              aria-pressed={onboardingState.academics.workingDays.includes(day.value)}
              onclick={() => toggleWorkingDay(day.value)}
            >
              {day.label}
            </button>
          {/each}
        </div>
      </div>
    </div>
  </div>

  <!-- Time Slots List (Clean, borderless list of rows) -->
  <div class="w-full overflow-x-auto scrollbar-hide border border-border/60 rounded-2xl bg-card shadow-sm">
    <div class="min-w-280 divide-y divide-border/60">
      <!-- Header Row -->
      <div class="sticky top-0 z-10 grid grid-cols-[60px_2fr_2fr_1.5fr_1.5fr_1fr_1.5fr_2fr_auto] gap-3 border-b border-border bg-muted/90 p-3 text-xs font-semibold text-muted-foreground backdrop-blur-sm select-none text-start items-center">
        <span class="text-center">{m.common_onboarding_schedule_col_num()}</span>
        <span>{m.common_onboarding_schedule_name_ar()}</span>
        <span>{m.common_onboarding_schedule_name_en()}</span>
        <span>{m.common_onboarding_schedule_col_start()}</span>
        <span>{m.common_onboarding_schedule_col_end()}</span>
        <span class="text-center">{m.common_onboarding_schedule_col_duration()}</span>
        <span class="text-center">نوع الفترة</span>
        <span class="text-center">أيام التطبيق</span>
        <span class="w-8"></span>
      </div>

      <!-- Inline Add Time Slot Row -->
      <div class="grid grid-cols-[60px_2fr_2fr_1.5fr_1.5fr_1fr_1.5fr_2fr_auto] gap-3 border-b border-primary/20 bg-primary/5 px-4 py-3 items-center text-start">
        <!-- Number indicator -->
        <span class="text-center text-xs font-mono text-muted-foreground/60">+</span>

        <!-- Name Ar -->
        <Input bind:value={newSlotNameAr} placeholder={m.common_onboarding_schedule_name_ar()} class="h-8 text-xs font-bold font-sans bg-background/50 border-dashed border-border/80" />

        <!-- Name En -->
        <Input bind:value={newSlotName} placeholder={m.common_onboarding_schedule_name_en()} class="h-8 text-xs text-left font-sans bg-background/50 border-dashed border-border/80" style="direction: ltr;" />

        <!-- Start Time -->
        <Input type="time" bind:value={newSlotStart} class="h-8 text-xs font-mono bg-background/50 border-dashed border-border/80" />

        <!-- End Time -->
        <Input type="time" bind:value={newSlotEnd} class="h-8 text-xs font-mono bg-background/50 border-dashed border-border/80" />

        <!-- Duration indicator -->
        <span class="text-center text-xs text-muted-foreground/60 font-sans">
          {newSlotDuration > 0 ? m.common_titlebar_timer_minutes({ minutes: newSlotDuration }) : "—"}
        </span>

        <select bind:value={newSlotType} class="h-8 rounded-md border border-border bg-background px-2 text-xs text-foreground">
          <option value="lesson">حصة دراسية</option>
          <option value="recess">استراحة</option>
          <option value="lunch">غداء</option>
          <option value="prayer">صلاة</option>
          <option value="custom">مخصص</option>
        </select>

        <span class="text-center text-[10px] text-muted-foreground">أيام العمل</span>

        <!-- Add Button -->
        <div class="flex justify-center items-center">
          <Button variant="default" size="icon-sm" class="h-8 w-8 rounded-lg bg-primary text-primary-foreground hover:bg-primary/90" onclick={handleAddTimeSlot} disabled={newSlotDuration <= 0} title="إضافة الفترة">
            <Plus size={14} />
          </Button>
        </div>
      </div>

      <!-- Slot Rows -->
      {#each onboardingState.academics.timeSlots as slot, idx}
        {@const dur = getDuration(slot.start_time, slot.end_time)}
        <div class="grid grid-cols-[60px_2fr_2fr_1.5fr_1.5fr_1fr_1.5fr_2fr_auto] gap-3 px-4 py-3 items-center hover:bg-muted/10 transition-colors text-start">
          <!-- Number -->
          <span class="text-center text-xs font-mono text-muted-foreground">{idx + 1}</span>

          <!-- Name Ar -->
          <Input bind:value={slot.name_ar} class="h-8 text-xs font-bold font-sans" onchange={() => onboardingState.save()} />

          <!-- Name En -->
          <Input bind:value={slot.name} class="h-8 text-xs text-left font-sans" style="direction: ltr;" onchange={() => onboardingState.save()} />

          <!-- Start Time -->
          <Input type="time" bind:value={slot.start_time} class="h-8 text-xs font-mono" onchange={() => { validateSlotTime(slot); onboardingState.save(); }} />

          <!-- End Time -->
          <Input type="time" bind:value={slot.end_time} class="h-8 text-xs font-mono" onchange={() => { validateSlotTime(slot); onboardingState.save(); }} />

          <!-- Duration -->
          <div class="text-center">
            <span class="text-xs bg-muted text-muted-foreground border border-border px-2 py-0.5 rounded font-sans">
              {m.common_titlebar_timer_minutes({ minutes: dur })}
            </span>
          </div>

          <select bind:value={slot.break_type} class="h-8 rounded-md border border-border bg-background px-2 text-xs text-foreground" onchange={() => { slot.is_break = !!slot.break_type; onboardingState.save(); }}>
            <option value="">حصة دراسية</option>
            <option value="recess">استراحة</option>
            <option value="lunch">غداء</option>
            <option value="prayer">صلاة</option>
            <option value="custom">مخصص</option>
          </select>

          <div class="flex flex-wrap justify-center gap-1">
            {#each dayLabels as day (day.value)}
              <button
                type="button"
                class="h-6 min-w-6 rounded border px-1 text-[9px] font-semibold cursor-pointer {(slot.applicable_days ?? onboardingState.academics.workingDays).includes(day.value) ? 'border-primary bg-primary/10 text-primary' : 'border-border bg-muted/30 text-muted-foreground'}"
                aria-label={day.label}
                aria-pressed={(slot.applicable_days ?? onboardingState.academics.workingDays).includes(day.value)}
                onclick={() => toggleSlotDay(slot, day.value)}
              >
                {day.label}
              </button>
            {/each}
          </div>

          <!-- Actions -->
          <div class="flex justify-center items-center">
            <Button variant="ghost" size="icon-sm" class="text-muted-foreground hover:text-destructive h-8 w-8 hover:bg-destructive/10" onclick={() => onboardingState.removeTimeSlot(idx)}>
              <Trash size={14} />
            </Button>
          </div>
        </div>
      {/each}
    </div>
  </div>
</div>
