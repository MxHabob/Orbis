<script lang="ts">
import { untrack } from "svelte";
import { toast } from "@orbis/ui";
import { api, extractErrorMessage, type TimeSlotDto } from "$lib/api";
import { getWeekdayOptions } from "$lib/core/i18n";
import { Button } from "@orbis/ui/button";
import { Checkbox } from "@orbis/ui/checkbox";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
} from "@orbis/ui/select";
import { FloppyDisk, Plus, Trash, XMark } from "@orbis/icons";
import * as m from "$lib/paraglide/messages";
import {
  buildGeneratedSlots,
  getBreakLabel,
  getDefaultStudyDays,
  parseTimeToMinutes,
  type BreakRule,
  type PreviewSlot,
} from "./time-slot-helpers";

interface Props {
  periodId: string;
  timeSlots: TimeSlotDto[];
  isSubmitting: boolean;
  onCancel: () => void;
  onSuccess: () => void;
}

let {
  periodId,
  timeSlots = [],
  isSubmitting = $bindable(false),
  onCancel,
  onSuccess,
}: Props = $props();

const weekdays = getWeekdayOptions();
const weekdayOrder = new Map(weekdays.map((day, idx) => [day.value, idx]));

let selectedDays = $state<number[]>(
  untrack(() => getDefaultStudyDays(timeSlots, weekdayOrder)),
);
let startTime = $state("08:00");
let periodsCount = $state(5);
let periodDuration = $state(45);
let gapMinutes = $state(0);
let breaks = $state<BreakRule[]>([
  { id: 1, afterPeriod: 2, duration: 15, type: "recess" },
]);
let nextBreakId = 2;
let replaceExisting = $state(untrack(() => timeSlots.length > 0));
let overrides = $state<
  Record<string, Partial<Pick<PreviewSlot, "nameAr" | "startTime" | "endTime">>>
>({});

const preview = $derived(
  buildGeneratedSlots({
    startTime,
    periodsCount,
    periodDuration,
    gapMinutes,
    breaks,
    overrides,
  }),
);

function toggleDay(day: number) {
  selectedDays = selectedDays.includes(day)
    ? selectedDays.filter((item) => item !== day)
    : [...selectedDays, day];
}

function addBreak() {
  const after = Math.min(
    periodsCount,
    Math.max(1, breaks.length === 0 ? 2 : breaks[breaks.length - 1].afterPeriod + 2),
  );
  breaks = [
    ...breaks,
    { id: nextBreakId++, afterPeriod: after, duration: 15, type: "recess" },
  ];
}

function setOverride(
  key: string,
  field: "nameAr" | "startTime" | "endTime",
  value: string,
) {
  overrides = {
    ...overrides,
    [key]: { ...overrides[key], [field]: value },
  };
}

async function handleSubmit(event: SubmitEvent) {
  event.preventDefault();
  if (!periodId || selectedDays.length === 0 || preview.length === 0) {
    toast.error(m.academics_schedule_time_slot_required());
    return;
  }
  const invalid = preview.some(
    (row) =>
      !row.nameAr.trim() ||
      parseTimeToMinutes(row.endTime) <= parseTimeToMinutes(row.startTime),
  );
  if (invalid) {
    toast.error(m.academics_schedule_time_slot_end_error());
    return;
  }

  isSubmitting = true;
  try {
    if (replaceExisting && timeSlots.length > 0) {
      await Promise.all(timeSlots.map((slot) => api.deleteTimeSlot(slot.id)));
    }
    const slots = selectedDays.flatMap((day) =>
      preview.map((row) => ({
        day_of_week: day,
        slot_index: row.slotIndex,
        name: row.nameEn.trim() || row.nameAr.trim(),
        name_ar: row.nameAr.trim(),
        start_time: row.startTime,
        end_time: row.endTime,
        is_break: row.isBreak,
        break_type: row.isBreak ? row.breakType : null,
      })),
    );
    const created = await api.batchCreateTimeSlots({
      period_id: periodId,
      slots,
    });
    toast.success(
      m.academics_schedule_generated_success({ count: created.length }),
    );
    onSuccess();
  } catch (error) {
    toast.error(extractErrorMessage(error));
  } finally {
    isSubmitting = false;
  }
}
</script>

<form
  class="space-y-3 rounded-xl border border-primary/25 bg-card p-3.5 shadow-xs"
  onsubmit={handleSubmit}
>
  <div class="flex items-center justify-between">
    <h3 class="text-sm font-semibold text-foreground">
      {m.academics_schedule_quick_generate()}
    </h3>
    <Button
      type="button"
      variant="ghost"
      size="icon"
      class="size-7"
      onclick={onCancel}
      aria-label={m.common_action_cancel()}
    >
      <XMark size={14} />
    </Button>
  </div>

  <div class="flex flex-wrap gap-1.5">
    {#each weekdays as day (day.value)}
      <button
        type="button"
        aria-pressed={selectedDays.includes(day.value)}
        disabled={isSubmitting}
        class="rounded-md border px-2.5 py-1 text-xs font-medium transition-colors {selectedDays.includes(day.value)
          ? 'border-primary bg-primary/10 text-primary'
          : 'border-border/60 bg-background text-muted-foreground hover:bg-muted'}"
        onclick={() => toggleDay(day.value)}
      >
        {day.label}
      </button>
    {/each}
  </div>

  <div class="grid grid-cols-2 gap-2.5 sm:grid-cols-4">
    <div class="space-y-1">
      <Label for="gen-start" class="text-[11px]">
        {m.academics_schedule_day_start_time()}
      </Label>
      <Input
        id="gen-start"
        type="time"
        bind:value={startTime}
        class="h-8 text-xs font-mono"
        disabled={isSubmitting}
        required
      />
    </div>
    <div class="space-y-1">
      <Label for="gen-count" class="text-[11px]">
        {m.academics_schedule_periods_count()}
      </Label>
      <Input
        id="gen-count"
        type="number"
        min="1"
        max="12"
        bind:value={periodsCount}
        class="h-8 text-xs"
        disabled={isSubmitting}
        required
      />
    </div>
    <div class="space-y-1">
      <Label for="gen-dur" class="text-[11px]">
        {m.academics_schedule_period_duration()}
      </Label>
      <Input
        id="gen-dur"
        type="number"
        min="10"
        max="180"
        step="5"
        bind:value={periodDuration}
        class="h-8 text-xs"
        disabled={isSubmitting}
        required
      />
    </div>
    <div class="space-y-1">
      <Label for="gen-gap" class="text-[11px]">
        {m.academics_schedule_gap_minutes()}
      </Label>
      <Input
        id="gen-gap"
        type="number"
        min="0"
        max="30"
        step="5"
        bind:value={gapMinutes}
        class="h-8 text-xs"
        disabled={isSubmitting}
      />
    </div>
  </div>

  <div class="space-y-1.5 rounded-lg border border-border/50 bg-background/60 p-2.5">
    <div class="flex items-center justify-between">
      <span class="text-xs font-medium text-foreground">
        {m.academics_schedule_breaks_section()}
      </span>
      <Button
        type="button"
        variant="ghost"
        size="sm"
        class="h-6 gap-1 px-2 text-[11px]"
        onclick={addBreak}
        disabled={isSubmitting}
      >
        <Plus size={12} />
        {m.academics_schedule_add_break()}
      </Button>
    </div>
    {#each breaks as rule (rule.id)}
      <div class="grid grid-cols-12 items-center gap-1.5">
        <div class="col-span-4 flex items-center gap-1">
          <span class="shrink-0 text-[10px] text-muted-foreground">
            {m.academics_schedule_after_period()}
          </span>
          <Input
            type="number"
            min="1"
            max={periodsCount}
            bind:value={rule.afterPeriod}
            class="h-7 text-xs"
            disabled={isSubmitting}
          />
        </div>
        <Input
          type="number"
          min="5"
          max="90"
          step="5"
          bind:value={rule.duration}
          class="col-span-3 h-7 text-xs"
          disabled={isSubmitting}
        />
        <div class="col-span-4">
          <Select
            type="single"
            value={rule.type}
            onValueChange={(value) => {
              rule.type = (value as BreakRule["type"]) ?? "recess";
            }}
          >
            <SelectTrigger class="h-7 text-xs">
              <span class="truncate">{getBreakLabel(rule.type)}</span>
            </SelectTrigger>
            <SelectContent class="text-start">
              <SelectItem value="recess" class="text-xs">
                {m.common_onboarding_break_recess()}
              </SelectItem>
              <SelectItem value="lunch" class="text-xs">
                {m.common_onboarding_break_lunch()}
              </SelectItem>
              <SelectItem value="prayer" class="text-xs">
                {m.common_onboarding_break_prayer()}
              </SelectItem>
              <SelectItem value="morning" class="text-xs">
                {m.academics_schedule_morning_assembly()}
              </SelectItem>
            </SelectContent>
          </Select>
        </div>
        <Button
          type="button"
          variant="ghost"
          size="icon"
          class="col-span-1 size-7 text-destructive hover:bg-destructive/10"
          onclick={() => (breaks = breaks.filter((item) => item.id !== rule.id))}
          aria-label={m.common_action_delete()}
        >
          <Trash size={12} />
        </Button>
      </div>
    {/each}
  </div>

  <div class="max-h-48 space-y-1 overflow-y-auto rounded-lg border border-border/50 bg-background/50 p-2">
    {#each preview as row (row.key)}
      <div
        class="grid grid-cols-12 items-center gap-1.5 rounded-md border border-border/40 bg-card px-2 py-1 {row.isBreak
          ? 'border-amber-500/30 bg-amber-500/[0.04]'
          : ''}"
      >
        <span class="col-span-1 text-center font-mono text-[11px] font-bold text-muted-foreground">
          {row.slotIndex}
        </span>
        <Input
          value={row.nameAr}
          oninput={(e) =>
            setOverride(row.key, "nameAr", (e.currentTarget as HTMLInputElement).value)}
          class="col-span-5 h-7 text-xs"
          disabled={isSubmitting}
        />
        <Input
          type="time"
          value={row.startTime}
          oninput={(e) =>
            setOverride(row.key, "startTime", (e.currentTarget as HTMLInputElement).value)}
          class="col-span-3 h-7 font-mono text-[11px]"
          disabled={isSubmitting}
        />
        <Input
          type="time"
          value={row.endTime}
          oninput={(e) =>
            setOverride(row.key, "endTime", (e.currentTarget as HTMLInputElement).value)}
          class="col-span-3 h-7 font-mono text-[11px]"
          disabled={isSubmitting}
        />
      </div>
    {/each}
  </div>

  {#if timeSlots.length > 0}
    <div class="flex items-center gap-2">
      <Checkbox
        id="replace-existing-slots"
        checked={replaceExisting}
        onCheckedChange={(value: boolean | "indeterminate") =>
          (replaceExisting = Boolean(value))}
        disabled={isSubmitting}
      />
      <Label for="replace-existing-slots" class="cursor-pointer text-xs">
        {m.academics_schedule_replace_existing()}
      </Label>
    </div>
  {/if}

  <div class="flex justify-end gap-2 border-t border-border/50 pt-2.5">
    <Button type="button" variant="outline" size="sm" onclick={onCancel}>
      {m.common_action_cancel()}
    </Button>
    <Button
      type="submit"
      size="sm"
      class="gap-1.5"
      disabled={isSubmitting || selectedDays.length === 0 || preview.length === 0}
    >
      <FloppyDisk size={14} />
      {m.academics_schedule_generate_save({
        count: selectedDays.length * preview.length,
      })}
    </Button>
  </div>
</form>
