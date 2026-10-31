<script lang="ts">
import { untrack } from "svelte";
import * as m from "$lib/paraglide/messages";
import { api, extractErrorMessage, type CreateTimeSlotRequest } from "$lib/api";
import { createTimeSlotInputSchema } from "@orbis/schemas";
import { Button } from "@orbis/ui/button";
import { Checkbox } from "@orbis/ui/checkbox";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from "@orbis/ui/dialog";
import { Input } from "@orbis/ui/input";
import { FormField } from "@orbis/ui";
import { Label } from "@orbis/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
} from "@orbis/ui/select";
import { toast } from "@orbis/ui";
import { FormController } from "$lib/shared/forms/form-controller.svelte";
import { BilingualNameFields } from "@orbis/ui/bilingual-name-fields";
import { suggestLocalTranslation } from "$lib/shared/translation/local-engine";

let {
  open = $bindable(false),
  periodId,
  nextSlotNumber = 1,
  onSuccess,
}: {
  open: boolean;
  periodId: string;
  nextSlotNumber: number;
  onSuccess?: () => void;
} = $props();

let breakType = $state("recess");

function calcDuration(start: string, end: string): number {
  if (!start || !end) return 0;
  const [startHours, startMinutes] = start.split(":").map(Number);
  const [endHours, endMinutes] = end.split(":").map(Number);
  return endHours * 60 + endMinutes - (startHours * 60 + startMinutes);
}

const form = new FormController({
  initialValues: {
    period_id: periodId || "",
    slot_number: nextSlotNumber || 1,
    name: "",
    name_ar: untrack(() =>
      m.academics_schedule_placeholder_period({ number: nextSlotNumber || 1 }),
    ),
    start_time: "08:00",
    end_time: "08:45",
    duration_minutes: 45,
    is_break: false,
    applicable_days: "1,2,3,4,5",
  },
  normalize: (v) => {
    const sTime = v.start_time || "08:00";
    const eTime = v.end_time || "08:45";
    const nAr = (v.name_ar || "").trim();
    const nEn = (v.name || "").trim();
    const pId = periodId || v.period_id || "";
    return {
      ...v,
      period_id: pId,
      slot_number: nextSlotNumber || v.slot_number || 1,
      name: nEn || nAr,
      name_ar: nAr || nEn,
      start_time: sTime,
      end_time: eTime,
      duration_minutes: calcDuration(sTime, eTime),
      is_break: Boolean(v.is_break),
      applicable_days: v.applicable_days || "1,2,3,4,5",
    };
  },
  schema: createTimeSlotInputSchema,
});

async function handleSubmit(event: SubmitEvent) {
  event.preventDefault();

  if (!periodId && !form.values.period_id) {
    toast.error(m.academics_studio_select_period());
    return;
  }

  form.values.period_id = periodId || form.values.period_id;
  form.values.slot_number = nextSlotNumber || form.values.slot_number;
  form.values.duration_minutes = calcDuration(
    form.values.start_time,
    form.values.end_time,
  );

  if (!form.validate()) {
    const errorMsg =
      form.firstError ||
      form.errors.end_time?.[0] ||
      form.errors.name_ar?.[0] ||
      m.academics_schedule_time_slot_end_error();
    toast.error(errorMsg);
    return;
  }

  const submitted = await form.submit(async (values) => {
    const request: CreateTimeSlotRequest = {
      period_id: values.period_id,
      day_of_week: 1,
      slot_index: values.slot_number,
      name: values.name,
      name_ar: values.name_ar,
      start_time: values.start_time,
      end_time: values.end_time,
      is_break: values.is_break,
      break_type: values.is_break ? breakType : null,
    };
    await api.createTimeSlot(request);
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
        <BilingualNameFields
          arId="time-slot-name-ar"
          enId="time-slot-name-en"
          arLabel={m.academics_schedule_name_ar()}
          enLabel={m.academics_schedule_name_en()}
          bind:arValue={form.values.name_ar}
          bind:enValue={form.values.name}
          arPlaceholder={m.academics_schedule_placeholder_period({ number: nextSlotNumber || 1 })}
          enPlaceholder={m.academics_schedule_placeholder_period_en({ number: nextSlotNumber || 1 })}
          arError={form.errors.name_ar?.[0]}
          enError={form.errors.name?.[0]}
          kind="label"
          requestSuggestion={suggestLocalTranslation}
          arRequired
          disabled={form.isSubmitting}
          loadingMessage={m.bilingual_entry_loading()}
          failureMessage={m.bilingual_entry_failure()}
        />

        <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
          <FormField label={m.academics_schedule_start_time()} error={form.errors.start_time?.[0]} required>
            {#snippet children()}
            <Input type="time" bind:value={form.values.start_time} class="h-9 text-sm font-mono" required />
            {/snippet}
          </FormField>
          <FormField label={m.academics_schedule_end_time()} error={form.errors.end_time?.[0]} required>
            {#snippet children()}
            <Input type="time" bind:value={form.values.end_time} class="h-9 text-sm font-mono" required />
            {/snippet}
          </FormField>
        </div>

        <div class="flex items-center gap-2">
          <Checkbox
            id="time-slot-break"
            checked={form.values.is_break}
            onCheckedChange={(value: boolean | "indeterminate") => (form.values.is_break = Boolean(value))}
          />
          <Label for="time-slot-break" class="cursor-pointer text-sm font-medium">
            {m.academics_schedule_break_slot()}
          </Label>
        </div>

        {#if form.values.is_break}
          <FormField label={m.academics_schedule_break_type()}>
            {#snippet children()}
            <Select
              type="single"
              value={breakType}
              onValueChange={(value: string | undefined) => (breakType = value ?? "recess")}
            >
              <SelectTrigger class="h-9 text-sm">
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
        <Button
          type="button"
          variant="outline"
          size="sm"
          class="text-xs"
          onclick={() => (open = false)}
          disabled={form.isSubmitting}
        >
          {m.common_action_cancel()}
        </Button>
        <Button
          type="submit"
          variant="lime"
          size="sm"
          class="text-xs font-semibold"
          disabled={!form.canSubmit}
        >
          {form.isSubmitting ? m.common_state_saving() : m.academics_schedule_save_slot()}
        </Button>
      </div>
    </form>
  </DialogContent>
</Dialog>
