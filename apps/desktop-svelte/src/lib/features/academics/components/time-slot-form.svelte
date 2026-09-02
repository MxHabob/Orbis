<script lang="ts">
  import * as m from "$lib/paraglide/messages";
  import { commands, extractErrorMessage } from "$lib/core/ipc";
  import { createTimeSlotInputSchema } from "@orbis/schemas";
  import { Button } from "@orbis/ui-svelte/button";
  import { Checkbox } from "@orbis/ui-svelte/checkbox";
  import { Dialog, DialogContent, DialogHeader, DialogTitle } from "@orbis/ui-svelte/dialog";
  import { Input } from "@orbis/ui-svelte/input";
  import { FormField } from "@orbis/ui-svelte";
  import { Label } from "@orbis/ui-svelte/label";
  import { Select, SelectContent, SelectItem, SelectTrigger } from "@orbis/ui-svelte/select";
  import { toast } from "svelte-sonner";
  import { FormController } from "$lib/shared/forms/form-controller.svelte";

  let {
    open = $bindable(false),
    periodId,
    nextSlotNumber,
    onSuccess,
  }: {
    open: boolean;
    periodId: string;
    nextSlotNumber: number;
    onSuccess?: () => void;
  } = $props();

  let nameAr = $state("");
  let name = $state("");
  let startTime = $state("08:00");
  let endTime = $state("08:45");
  let isBreak = $state(false);
  let breakType = $state("recess");
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

  $effect(() => {
    if (open) {
      nameAr = m.academics_schedule_placeholder_period({ number: nextSlotNumber });
      name = m.academics_schedule_placeholder_period_en({ number: nextSlotNumber });
      startTime = "08:00";
      endTime = "08:45";
      isBreak = false;
      breakType = "recess";
    }
  });

  function calcDuration(start: string, end: string): number {
    const [startHours, startMinutes] = start.split(":").map(Number);
    const [endHours, endMinutes] = end.split(":").map(Number);
    return endHours * 60 + endMinutes - (startHours * 60 + startMinutes);
  }

  async function handleSubmit(event: SubmitEvent) {
    event.preventDefault();
    const duration = calcDuration(startTime, endTime);
    form.values = {
      period_id: periodId,
      slot_number: nextSlotNumber,
      name: name.trim() || nameAr.trim(),
      name_ar: nameAr.trim(),
      start_time: startTime,
      end_time: endTime,
      duration_minutes: duration,
      is_break: isBreak,
      applicable_days: "1,2,3,4,5",
    };
    if (!form.validate()) {
      toast.error(m.academics_schedule_time_slot_end_error());
      return;
    }

    const submitted = await form.submit(async (values) => {
      await commands.scheduling.createTimeSlot({
        ...values,
        break_type: isBreak ? breakType : null,
      } as any);
      toast.success(m.academics_schedule_time_slot_added());
      open = false;
      onSuccess?.();
    });
    if (!submitted && form.formError) {
      toast.error(extractErrorMessage(form.formError));
    }
  }
</script>

<Dialog bind:open>
  <DialogContent class="max-w-md text-start">
    <DialogHeader>
      <DialogTitle class="text-lg font-bold text-foreground">
        {m.academics_schedule_time_slot_new()}
      </DialogTitle>
    </DialogHeader>

    <form onsubmit={handleSubmit} class="flex flex-col gap-4 py-2">
      <div class="flex flex-col gap-4">
            <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
              <FormField label={m.academics_schedule_name_ar()} error={form.errors.name_ar?.[0]} required>
                {#snippet children()}
                <Input bind:value={nameAr} placeholder={m.academics_schedule_placeholder_period({ number: 1 })} class="h-9 text-sm" required />
                {/snippet}
              </FormField>
              <FormField label={m.academics_schedule_name_en()} error={form.errors.name?.[0]}>
                {#snippet children()}
                <Input bind:value={name} placeholder={m.academics_schedule_placeholder_period_en({ number: 1 })} class="h-9 text-sm" />
                {/snippet}
              </FormField>
            </div>

            <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
              <FormField label={m.academics_schedule_start_time()} error={form.errors.start_time?.[0]} required>
                {#snippet children()}
                <Input type="time" bind:value={startTime} class="h-9 text-sm font-mono" required />
                {/snippet}
              </FormField>
              <FormField label={m.academics_schedule_end_time()} error={form.errors.end_time?.[0]} required>
                {#snippet children()}
                <Input type="time" bind:value={endTime} class="h-9 text-sm font-mono" required />
                {/snippet}
              </FormField>
            </div>

            <div class="flex items-center gap-2">
              <Checkbox id="time-slot-break" checked={isBreak} onCheckedChange={(value: boolean | "indeterminate") => (isBreak = Boolean(value))} />
              <Label for="time-slot-break" class="cursor-pointer text-sm font-medium">{m.academics_schedule_break_slot()}</Label>
            </div>

            {#if isBreak}
              <FormField label={m.academics_schedule_break_type()}>
                {#snippet children()}
                <Select type="single" value={breakType} onValueChange={(value: string | undefined) => (breakType = value ?? "recess")}>
                  <SelectTrigger class="h-9 text-sm">{breakType === "recess" ? m.common_onboarding_break_recess() : breakType === "lunch" ? m.common_onboarding_break_lunch() : breakType === "prayer" ? m.common_onboarding_break_prayer() : m.academics_schedule_morning_assembly()}</SelectTrigger>
                  <SelectContent>
                    <SelectItem value="recess">{m.common_onboarding_break_recess()}</SelectItem>
                    <SelectItem value="lunch">{m.common_onboarding_break_lunch()}</SelectItem>
                    <SelectItem value="prayer">{m.common_onboarding_break_prayer()}</SelectItem>
                    <SelectItem value="morning">{m.academics_schedule_morning_assembly()}</SelectItem>
                  </SelectContent>
                </Select>
                {/snippet}
              </FormField>
            {/if}
      </div>

      <div class="flex justify-end gap-2 border-t border-border/40 pt-3">
        <Button type="button" variant="outline" size="sm" class="text-xs" onclick={() => (open = false)} disabled={form.isSubmitting}>{m.common_action_cancel()}</Button>
        <Button type="submit" variant="lime" size="sm" class="text-xs font-semibold" disabled={form.isSubmitting}>{form.isSubmitting ? m.common_state_saving() : m.academics_schedule_save_slot()}</Button>
      </div>
    </form>
  </DialogContent>
</Dialog>
