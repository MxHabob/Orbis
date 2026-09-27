<script lang="ts">
import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
  SheetFooter,
} from "@orbis/ui/sheet";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { FormField } from "@orbis/ui";
import { Label } from "@orbis/ui/label";
import { Checkbox } from "@orbis/ui/checkbox";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import { ClockRotateRight, FloppyDisk, Heart, Pencil, Plus, Trash, XMark } from "@orbis/icons";
import { api, extractErrorMessage, type CreateTimeSlotRequest } from "$lib/api";
import { toast } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";
import { createTimeSlotInputSchema } from "@orbis/schemas";
import { FormController } from "$lib/shared/forms/form-controller.svelte";
import TimeSlotForm from "./time-slot-form.svelte";

interface Props {
  open: boolean;
  periodId: string;
  timeSlots: any[];
  onSuccess?: () => void;
}

let {
  open = $bindable(false),
  periodId,
  timeSlots = [],
  onSuccess,
}: Props = $props();

let editingSlotId = $state<string | null>(null);
let isAdding = $state(false);

// Form State
let nameAr = $state("");
let name = $state("");
let startTime = $state("08:00");
let endTime = $state("08:45");
let isBreak = $state(false);
let breakType = $state("recess");
let isSubmitting = $state(false);

const form = new FormController({
  initialValues: {
    period_id: "",
    slot_number: 1,
    name: "",
    name_ar: "",
    start_time: "08:00",
    end_time: "08:45",
    duration_minutes: 45,
    is_break: false,
    applicable_days: "1,2,3,4,5",
  },
  schema: createTimeSlotInputSchema,
});

function resetForm() {
  editingSlotId = null;
  isAdding = false;
  nameAr = "";
  name = "";
  startTime = "08:00";
  endTime = "08:45";
  isBreak = false;
  breakType = "recess";
}

function startEdit(slot: any) {
  editingSlotId = String(slot.id);
  isAdding = false;
  nameAr = slot.name_ar || "";
  name = slot.name || "";
  startTime = String(slot.start_time || "").slice(0, 5);
  endTime = String(slot.end_time || "").slice(0, 5);
  isBreak = Boolean(slot.is_break);
  breakType = slot.break_type || "recess";
}

function startAdd() {
  resetForm();
  isAdding = true;
  const nextSlotNum =
    timeSlots
      .filter((s) => !s.is_break)
      .reduce((max, s) => Math.max(max, Number(s.slot_number) || 0), 0) + 1;
  nameAr = m.academics_schedule_placeholder_period({ number: nextSlotNum });
  name = m.academics_schedule_placeholder_period_en({ number: nextSlotNum });
}

function calcDuration(start: string, end: string): number {
  const [sh, sm] = start.split(":").map(Number);
  const [eh, em] = end.split(":").map(Number);
  return eh * 60 + em - (sh * 60 + sm);
}

async function handleSaveSlot(e: SubmitEvent) {
  e.preventDefault();
  form.values = {
    period_id: periodId,
    slot_number:
      timeSlots.reduce(
        (max, s) => Math.max(max, Number(s.slot_number) || 0),
        0,
      ) + 1,
    name: name.trim() || nameAr.trim(),
    name_ar: nameAr.trim(),
    start_time: startTime,
    end_time: endTime,
    duration_minutes: calcDuration(startTime, endTime),
    is_break: isBreak,
    applicable_days: "1,2,3,4,5",
  };
  if (!form.validate()) {
    toast.error(m.academics_schedule_time_slot_end_error());
    return;
  }

  isSubmitting = true;
  try {
    if (editingSlotId) {
      // Update
      const request: CreateTimeSlotRequest = {
        period_id: periodId,
        day_of_week: 1,
        slot_index: 1,
        name: name.trim() || nameAr.trim(),
        name_ar: nameAr.trim(),
        start_time: startTime,
        end_time: endTime,
        is_break: isBreak,
        break_type: isBreak ? breakType : null,
      };
      await api.createTimeSlot(request);
      toast.success(m.academics_schedule_time_slot_updated());
    } else {
      // Create
      const slotNumber =
        timeSlots.reduce(
          (max, s) => Math.max(max, Number(s.slot_number) || 0),
          0,
        ) + 1;

      const request: CreateTimeSlotRequest = {
        period_id: periodId,
        day_of_week: 1,
        slot_index: slotNumber,
        name: name.trim() || nameAr.trim(),
        name_ar: nameAr.trim(),
        start_time: startTime,
        end_time: endTime,
        is_break: isBreak,
        break_type: isBreak ? breakType : null,
      };
      await api.createTimeSlot(request);
      toast.success(m.academics_schedule_time_slot_added());
    }

    resetForm();
    onSuccess?.();
  } catch (err) {
    toast.error(extractErrorMessage(err));
  } finally {
    isSubmitting = false;
  }
}

async function handleDeleteSlot(id: string) {
  try {
    toast.success(m.academics_schedule_time_slot_deleted());
    if (editingSlotId === id) resetForm();
    onSuccess?.();
  } catch (err) {
    toast.error(extractErrorMessage(err));
  }
}
</script>

<Sheet bind:open>
  <SheetContent class="w-full sm:max-w-lg overflow-y-auto flex flex-col p-6 text-start">
    <SheetHeader>
      <SheetTitle>
        {m.academics_schedule_time_slots_heading()}
      </SheetTitle>
    </SheetHeader>

    <div class="mt-4 flex-1 flex flex-col gap-4">
      <!-- Add Slot Form / Editor -->
      {#if editingSlotId}
        <form onsubmit={handleSaveSlot} class="rounded-xl border border-primary/30 bg-card p-4 flex flex-col gap-3 shadow-xs">
          <div class="flex items-center justify-between">
            <span class="text-xs font-bold text-primary">
              {editingSlotId ? m.academics_schedule_time_slot_edit() : m.academics_schedule_time_slot_new()}
            </span>
            <Button variant="ghost" size="icon" class="h-6 w-6" onclick={resetForm}>
              <XMark size={12} />
            </Button>
          </div>

          <div class="grid grid-cols-2 gap-3">
            <FormField label={m.academics_schedule_name_ar()} error={form.errors.name_ar?.[0]} required>
              {#snippet children()}
              <Input bind:value={nameAr} placeholder={m.academics_schedule_placeholder_period({ number: 1 })} class="h-8 text-xs" required />
              {/snippet}
            </FormField>
            <FormField label={m.academics_schedule_name_en()} error={form.errors.name?.[0]}>
              {#snippet children()}
              <Input bind:value={name} placeholder={m.academics_schedule_placeholder_period_en({ number: 1 })} class="h-8 text-xs" />
              {/snippet}
            </FormField>
          </div>

          <div class="grid grid-cols-2 gap-3">
            <FormField label={m.academics_schedule_start_time()} error={form.errors.start_time?.[0]} required>
              {#snippet children()}
              <Input type="time" bind:value={startTime} class="h-8 text-xs font-mono" required />
              {/snippet}
            </FormField>
            <FormField label={m.academics_schedule_end_time()} error={form.errors.end_time?.[0]} required>
              {#snippet children()}
              <Input type="time" bind:value={endTime} class="h-8 text-xs font-mono" required />
              {/snippet}
            </FormField>
          </div>

          <div class="flex items-center gap-2 pt-1">
            <Checkbox id="is-break-check" checked={isBreak} onCheckedChange={(val: boolean | "indeterminate") => (isBreak = Boolean(val))} />
            <Label for="is-break-check" class="text-xs cursor-pointer font-medium">
              {m.academics_schedule_break_slot()}
            </Label>
          </div>

          {#if isBreak}
            <FormField label={m.academics_schedule_break_type()}>
              {#snippet children()}
              <Select type="single" value={breakType} onValueChange={(val: string | undefined) => (breakType = val ?? "recess")}>
                <SelectTrigger class="h-8 text-xs">
                  <span>
                    {breakType === "recess"
                      ? m.common_onboarding_break_recess()
                      : breakType === "lunch"
                        ? m.common_onboarding_break_lunch()
                        : breakType === "prayer"
                          ? m.common_onboarding_break_prayer()
                          : m.academics_schedule_morning_assembly()}
                  </span>
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="recess" class="text-xs">{m.common_onboarding_break_recess()}</SelectItem>
                  <SelectItem value="lunch" class="text-xs">{m.common_onboarding_break_lunch()}</SelectItem>
                  <SelectItem value="prayer" class="text-xs">{m.common_onboarding_break_prayer()}</SelectItem>
                  <SelectItem value="morning" class="text-xs">{m.academics_schedule_morning_assembly()}</SelectItem>
                </SelectContent>
              </Select>
              {/snippet}
            </FormField>
          {/if}

          <div class="flex items-center justify-end gap-2 pt-2 border-t border-border/40">
            <Button type="button" variant="outline" size="sm" class="h-8 text-xs" onclick={resetForm}>
              {m.common_action_cancel()}
            </Button>
            <Button type="submit" size="sm" class="h-8 text-xs font-semibold" disabled={isSubmitting}>
              <FloppyDisk size={13} class="me-1" />
              {m.academics_schedule_save_slot()}
            </Button>
          </div>
        </form>
      {:else}
        <Button
          type="button"
          variant="outline"
          class="w-full border-dashed gap-1.5 text-xs font-semibold text-primary hover:bg-primary/5"
          onclick={startAdd}
        >
          <Plus size={14} />
          <span>{m.academics_schedule_add_slot()}</span>
        </Button>
      {/if}

      <!-- List of Existing Slots -->
      <div class="flex flex-col gap-1.5 mt-2">
        <span class="text-[11px] font-bold text-muted-foreground uppercase tracking-wider">
          {m.academics_schedule_current_slots()} ({timeSlots.length})
        </span>

        {#if timeSlots.length === 0}
          <div class="flex flex-col items-center justify-center p-8 border border-dashed border-border/60 rounded-xl bg-muted/10 text-center gap-2">
            <ClockRotateRight size={24} class="text-muted-foreground/60" />
            <p class="text-xs text-muted-foreground">
              {m.academics_schedule_no_slots()}
            </p>
          </div>
        {:else}
          {#each timeSlots as slot (slot.id)}
            {@const isSlotBreak = Boolean(slot.is_break)}
            <div
              class="flex items-center justify-between p-2.5 rounded-lg border border-border/60 bg-card transition-colors hover:border-primary/40 {isSlotBreak ? 'bg-amber-500/5 border-amber-500/30' : ''}"
            >
              <div class="flex items-center gap-2.5 min-w-0">
                <div
                  class="flex h-7 w-7 shrink-0 items-center justify-center rounded-md font-bold text-xs {isSlotBreak ? 'bg-amber-500/20 text-amber-600 dark:text-amber-400' : 'bg-primary/10 text-primary'}"
                >
                  {#if isSlotBreak}
                    <Heart size={13} />
                  {:else}
                    {slot.slot_number}
                  {/if}
                </div>

                <div class="flex flex-col min-w-0">
                  <span class="text-xs font-bold text-foreground truncate">
                    {slot.name_ar || slot.name || m.academics_schedule_placeholder_period_en({ number: slot.slot_number })}
                  </span>
                  <span class="text-[10px] text-muted-foreground font-mono">
                    {slot.start_time?.slice(0, 5)} - {slot.end_time?.slice(0, 5)} ({slot.duration_minutes || calcDuration(slot.start_time, slot.end_time)} {m.academics_schedule_minutes()})
                  </span>
                </div>
              </div>

              <div class="flex items-center gap-1">
                <Button
                  variant="ghost"
                  size="icon"
                  class="h-7 w-7 text-muted-foreground hover:text-foreground"
                  onclick={() => startEdit(slot)}
                  aria-label={m.academics_schedule_edit()}
                >
                  <Pencil size={12} />
                </Button>
                <Button
                  variant="ghost"
                  size="icon"
                  class="h-7 w-7 text-destructive/70 hover:text-destructive hover:bg-destructive/10"
                  onclick={() => handleDeleteSlot(String(slot.id))}
                  aria-label={m.common_action_delete()}
                >
                  <Trash size={12} />
                </Button>
              </div>
            </div>
          {/each}
        {/if}
      </div>
    </div>

    <SheetFooter class="mt-6 border-t border-border/40 pt-4">
      <Button variant="outline" size="sm" class="w-full text-xs" onclick={() => (open = false)}>
        {m.common_action_close()}
      </Button>
    </SheetFooter>
  </SheetContent>
</Sheet>

<TimeSlotForm
  bind:open={isAdding}
  {periodId}
  nextSlotNumber={timeSlots.filter((slot) => !slot.is_break).reduce((max, slot) => Math.max(max, Number(slot.slot_number) || 0), 0) + 1}
  onSuccess={() => {
    isAdding = false;
    onSuccess?.();
  }}
/> 
