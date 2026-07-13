<!--
  Step 5: Daily Time Slots Setup
  Add, remove, and list school daily periods/lessons.
-->
<script lang="ts">
import { Input } from "@orbis/ui-svelte/input";
import { Button } from "@orbis/ui-svelte/button";
import { Plus, Trash } from "lucide-svelte";
import * as m from "$lib/paraglide/messages";
import { onboardingState } from "../state.svelte";

let newSlotName = $state("");
let newSlotNameAr = $state("");
let newSlotStart = $state("12:15");
let newSlotEnd = $state("13:00");

function handleAddTimeSlot() {
  const success = onboardingState.addTimeSlot(
    newSlotName,
    newSlotNameAr,
    newSlotStart,
    newSlotEnd,
  );
  if (success) {
    newSlotName = "";
    newSlotNameAr = "";
  }
}
</script>

<div class="w-full space-y-6">

  <!-- Time Slots List (Clean, borderless list of rows) -->
  <div class="w-full overflow-x-auto scrollbar-hide border border-border/60 rounded-2xl bg-card shadow-sm">
    <div class="min-w-[850px] divide-y divide-border/60">
      <!-- Header Row -->
      <div class="grid grid-cols-[60px_2fr_2fr_1.5fr_1.5fr_1.2fr_auto] gap-4 p-4 bg-muted/20 text-xs font-semibold text-muted-foreground select-none text-start items-center">
        <span class="text-center">{m.onboarding_schedule_col_num()}</span>
        <span>{m.onboarding_schedule_name_ar()}</span>
        <span>{m.onboarding_schedule_name_en()}</span>
        <span>{m.onboarding_schedule_col_start()}</span>
        <span>{m.onboarding_schedule_col_end()}</span>
        <span class="text-center">{m.onboarding_schedule_col_duration()}</span>
        <span class="w-8"></span>
      </div>

      <!-- Inline Add Time Slot Row -->
      <div class="grid grid-cols-[60px_2fr_2fr_1.5fr_1.5fr_1.2fr_auto] gap-4 px-4 py-2.5 items-center bg-muted/5 transition-colors text-start">
        <!-- Number indicator -->
        <span class="text-center text-xs font-mono text-muted-foreground/60">+</span>

        <!-- Name Ar -->
        <Input bind:value={newSlotNameAr} placeholder={m.onboarding_schedule_name_ar()} class="h-8 text-xs font-bold font-sans bg-background/50 border-dashed border-border/80" />

        <!-- Name En -->
        <Input bind:value={newSlotName} placeholder={m.onboarding_schedule_name_en()} class="h-8 text-xs font-mono text-left font-sans bg-background/50 border-dashed border-border/80" style="direction: ltr;" />

        <!-- Start Time -->
        <Input type="time" bind:value={newSlotStart} class="h-8 text-xs font-mono bg-background/50 border-dashed border-border/80" />

        <!-- End Time -->
        <Input type="time" bind:value={newSlotEnd} class="h-8 text-xs font-mono bg-background/50 border-dashed border-border/80" />

        <!-- Duration indicator -->
        <span class="text-center text-xs text-muted-foreground/60 font-sans">—</span>

        <!-- Add Button -->
        <div class="flex justify-center items-center">
          <Button variant="default" size="icon-sm" class="h-8 w-8 rounded-lg bg-primary text-primary-foreground hover:bg-primary/90 animate-pulse" onclick={handleAddTimeSlot}>
            <Plus size={14} />
          </Button>
        </div>
      </div>

      <!-- Slot Rows -->
      {#each onboardingState.academics.timeSlots as slot, idx}
        {@const startParts = slot.start_time.split(":").map(Number)}
        {@const endParts = slot.end_time.split(":").map(Number)}
        {@const dur = (endParts[0] * 60 + endParts[1]) - (startParts[0] * 60 + startParts[1])}
        <div class="grid grid-cols-[60px_2fr_2fr_1.5fr_1.5fr_1.2fr_auto] gap-4 px-4 py-3 items-center hover:bg-muted/10 transition-colors text-start">
          <!-- Number -->
          <span class="text-center text-xs font-mono text-muted-foreground">{idx + 1}</span>

          <!-- Name Ar -->
          <Input bind:value={slot.name_ar} class="h-8 text-xs font-bold font-sans" onchange={() => onboardingState.save()} />

          <!-- Name En -->
          <Input bind:value={slot.name} class="h-8 text-xs font-mono text-left font-sans" style="direction: ltr;" onchange={() => onboardingState.save()} />

          <!-- Start Time -->
          <Input type="time" bind:value={slot.start_time} class="h-8 text-xs font-mono" onchange={() => onboardingState.save()} />

          <!-- End Time -->
          <Input type="time" bind:value={slot.end_time} class="h-8 text-xs font-mono" onchange={() => onboardingState.save()} />

          <!-- Duration -->
          <div class="text-center">
            <span class="text-xs bg-muted text-muted-foreground border border-border px-2 py-0.5 rounded font-sans">
              {m.onboarding_schedule_duration_value({ minutes: dur })}
            </span>
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
