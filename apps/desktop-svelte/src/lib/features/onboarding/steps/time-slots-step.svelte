<!--
  Step 5: Daily Time Slots Setup
  Add, remove, and list school daily periods/lessons.
-->
<script lang="ts">
import { Label } from "@orbis/ui-svelte/label";
import { Input } from "@orbis/ui-svelte/input";
import { Button } from "@orbis/ui-svelte/button";
import { Plus, Trash } from "lucide-svelte";
import * as m from "$lib/paraglide/messages";
import { onboardingState } from "../onboarding-state.svelte";

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

<div class="max-w-3xl space-y-6">
  <div class="space-y-2 text-start">
    <h3 class="text-foreground font-bold text-xl font-serif">
      {m.onboarding_schedule_title()}
    </h3>
    <p class="text-muted-foreground text-sm">
      {m.onboarding_schedule_desc()}
    </p>
  </div>

  <!-- Add Period Form -->
  <div class="p-6 bg-muted/40 rounded-xl border border-border grid grid-cols-1 md:grid-cols-4 gap-4 items-end">
    <div class="space-y-2 text-start">
      <Label class="text-xs">{m.onboarding_schedule_name_ar()}</Label>
      <Input bind:value={newSlotNameAr} placeholder="الحصة السادسة" class="text-xs h-9" />
    </div>

    <div class="space-y-2 text-start">
      <Label class="text-xs">{m.onboarding_schedule_name_en()}</Label>
      <Input bind:value={newSlotName} placeholder="6th Period" class="text-xs h-9 text-left font-sans" style="direction: ltr;" />
    </div>

    <div class="space-y-2 text-start">
      <Label class="text-xs">{m.onboarding_schedule_start_end()}</Label>
      <div class="grid grid-cols-2 gap-2">
        <Input type="time" bind:value={newSlotStart} class="text-xs h-9 font-sans" />
        <Input type="time" bind:value={newSlotEnd} class="text-xs h-9 font-sans" />
      </div>
    </div>

    <Button onclick={handleAddTimeSlot} class="w-full h-9 text-xs flex gap-1.5 items-center justify-center">
      <Plus class="w-4 h-4" />
      <span>{m.onboarding_schedule_add_period()}</span>
    </Button>
  </div>

  <!-- Time Slots table -->
  <div class="bg-card border border-border rounded-xl overflow-hidden">
    <table class="w-full border-collapse">
      <thead>
        <tr class="bg-muted text-muted-foreground border-b border-border text-xs font-semibold">
          <th class="p-4 text-start w-12">{m.onboarding_schedule_col_num()}</th>
          <th class="p-4 text-start">{m.onboarding_schedule_col_name()}</th>
          <th class="p-4 text-center">{m.onboarding_schedule_col_start()}</th>
          <th class="p-4 text-center">{m.onboarding_schedule_col_end()}</th>
          <th class="p-4 text-center">{m.onboarding_schedule_col_duration()}</th>
          <th class="p-4 text-center">{m.onboarding_schedule_col_actions()}</th>
        </tr>
      </thead>
      <tbody class="divide-y divide-border text-sm">
        {#each onboardingState.timeSlots as slot, idx}
          {@const startParts = slot.start_time.split(":").map(Number)}
          {@const endParts = slot.end_time.split(":").map(Number)}
          {@const dur = (endParts[0] * 60 + endParts[1]) - (startParts[0] * 60 + startParts[1])}
          <tr class="hover:bg-muted/30">
            <td class="p-4 text-muted-foreground font-mono text-xs">{idx + 1}</td>
            <td class="p-4 text-start">
              <div class="font-medium text-foreground">{slot.name_ar}</div>
              <div class="text-xs text-muted-foreground font-mono">{slot.name}</div>
            </td>
            <td class="p-4 text-center font-mono text-xs text-foreground">{slot.start_time}</td>
            <td class="p-4 text-center font-mono text-xs text-foreground">{slot.end_time}</td>
            <td class="p-4 text-center">
              <span class="text-xs bg-muted text-muted-foreground border border-border px-2 py-0.5 rounded font-sans">
                {m.onboarding_schedule_duration_value({ minutes: dur })}
              </span>
            </td>
            <td class="p-4 text-center">
              <button
                onclick={() => onboardingState.removeTimeSlot(idx)}
                class="text-muted-foreground hover:text-destructive p-1.5 hover:bg-destructive/10 rounded-lg transition-colors"
              >
                <Trash class="w-4 h-4" />
              </button>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</div>
