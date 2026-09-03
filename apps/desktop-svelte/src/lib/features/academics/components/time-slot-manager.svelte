<script lang="ts">
import { Save, X } from "@lucide/svelte";
import { toast } from "@orbis/ui-svelte";
import { commands } from "$lib/core/ipc";
import { createTimeSlotInputSchema } from "@orbis/schemas";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { FormField } from "@orbis/ui-svelte";
import * as m from "$lib/paraglide/messages";
import { FormController } from "$lib/shared/forms/form-controller.svelte";

let {
  open = $bindable(false),
  periodId,
  timeSlots = [],
  slot = null,
  onSuccess,
}: {
  open?: boolean;
  periodId: string;
  timeSlots: any[];
  slot?: any | null;
  onSuccess: () => void;
} = $props();

let editingId = $state<string | null>(slot?.id ? String(slot.id) : null);
let nameAr = $state("");
let name = $state("");
let startTime = $state("");
let endTime = $state("");

const form = new FormController({
  initialValues: {
    period_id: "",
    slot_number: 1,
    name: "",
    name_ar: "",
    start_time: "",
    end_time: "",
    duration_minutes: 0,
    is_break: false,
    applicable_days: "1,2,3,4,5",
  },
  schema: createTimeSlotInputSchema,
});
const isSubmitting = $derived(form.isSubmitting);

function resetForm() {
  editingId = null;
  nameAr = "";
  name = "";
  startTime = "";
  endTime = "";
}

function editSlot(slot: any) {
  editingId = String(slot.id);
  nameAr = slot.name_ar || "";
  name = slot.name || "";
  startTime = String(slot.start_time || "").slice(0, 5);
  endTime = String(slot.end_time || "").slice(0, 5);
  open = true;
}

function beginCreate() {
  resetForm();
  const nextNumber = timeSlots.reduce(
    (highest, slot) => Math.max(highest, Number(slot.slot_number) || 0),
    0,
  ) + 1;
  nameAr = m.academics_schedule_placeholder_period({ number: nextNumber });
  name = m.academics_schedule_placeholder_period_en({ number: nextNumber });
  open = true;
}

$effect(() => {
  if (slot) {
    editingId = String(slot.id);
    nameAr = slot.name_ar || "";
    name = slot.name || "";
    startTime = String(slot.start_time || "").slice(0, 5);
    endTime = String(slot.end_time || "").slice(0, 5);
  }
});

function durationMinutes() {
  const [startHour, startMinute] = startTime.split(":").map(Number);
  const [endHour, endMinute] = endTime.split(":").map(Number);
  return endHour * 60 + endMinute - (startHour * 60 + startMinute);
}

async function saveSlot(event: SubmitEvent) {
  event.preventDefault();
  const slotNumber = timeSlots.reduce(
    (highest, slot) => Math.max(highest, Number(slot.slot_number) || 0),
    0,
  ) + 1;
  form.values = {
    period_id: periodId,
    slot_number: slotNumber,
    name: name.trim() || nameAr.trim(),
    name_ar: nameAr.trim(),
    start_time: startTime,
    end_time: endTime,
    duration_minutes: durationMinutes(),
    is_break: false,
    applicable_days: "1,2,3,4,5",
  };
  const submitted = await form.submit(async (payload) => {
    if (editingId) {
      await commands.scheduling.updateTimeSlot(editingId, {
        name: name.trim() || nameAr.trim(),
        name_ar: nameAr.trim(),
        start_time: startTime,
        end_time: endTime,
      });
      toast.success(m.academics_schedule_time_slot_updated());
    } else {
      await commands.scheduling.createTimeSlot(payload);
      toast.success(m.academics_schedule_time_slot_added());
    }
  });
  if (submitted) {
    resetForm();
    open = false;
    onSuccess();
  } else if (form.formError) toast.error(form.formError);
  else toast.error(m.academics_schedule_time_slot_required());
}

</script>

{#if open}
  <section class="shrink-0 rounded-xl border border-primary/30 bg-card/40 p-3 text-start">
    <form class="mt-3 grid grid-cols-1 gap-3 rounded-lg border border-primary/30 bg-background p-3 sm:grid-cols-2 lg:grid-cols-5" onsubmit={saveSlot}>
      <FormField label={m.academics_schedule_name_ar()} error={form.errors.name_ar?.[0]} required>{#snippet children()}<Input bind:value={nameAr} placeholder={m.academics_schedule_name_ar()} required disabled={isSubmitting} />{/snippet}</FormField>
      <FormField label={m.academics_schedule_name_en()} error={form.errors.name?.[0]}>{#snippet children()}<Input bind:value={name} placeholder={m.academics_schedule_name_en()} disabled={isSubmitting} />{/snippet}</FormField>
      <FormField label={m.academics_schedule_start_time()} error={form.errors.start_time?.[0]} required>{#snippet children()}<Input bind:value={startTime} type="time" required disabled={isSubmitting} />{/snippet}</FormField>
      <FormField label={m.academics_schedule_end_time()} error={form.errors.end_time?.[0]} required>{#snippet children()}<Input bind:value={endTime} type="time" required disabled={isSubmitting} />{/snippet}</FormField>
      <div class="flex gap-2">
        <Button type="submit" class="h-9 flex-1 gap-1.5 text-xs" disabled={isSubmitting}>
          <Save size={14} />
          {m.academics_schedule_save_slot()}
        </Button>
        <Button type="button" variant="ghost" size="icon" class="h-9 w-9" onclick={() => { open = false; resetForm(); }} aria-label={m.common_action_cancel()}>
          <X size={14} />
        </Button>
      </div>
    </form>
  </section>
{/if}
