<script lang="ts">
import { toast } from "@orbis/ui";
import { api, extractErrorMessage, type TimeSlotDto } from "$lib/api";
import { Badge } from "@orbis/ui/badge";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import {
  Sheet,
  SheetContent,
  SheetFooter,
  SheetHeader,
  SheetTitle,
} from "@orbis/ui/sheet";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
} from "@orbis/ui/select";
import {
  ClockRotateRight,
  FloppyDisk,
  Heart,
  Pencil,
  Plus,
  Trash,
  XMark,
} from "@orbis/icons";
import { BilingualNameFields } from "@orbis/ui/bilingual-name-fields";
import { suggestLocalTranslation } from "$lib/shared/translation/local-engine";
import * as m from "$lib/paraglide/messages";

interface Props {
  open: boolean;
  periodId: string;
  timeSlots: TimeSlotDto[];
  onSuccess?: () => void;
}

let {
  open = $bindable(false),
  periodId,
  timeSlots = [],
  onSuccess,
}: Props = $props();

const weekdays = [
  { value: 7, label: m.common_day_saturday() },
  { value: 1, label: m.common_day_sunday() },
  { value: 2, label: m.common_day_monday() },
  { value: 3, label: m.common_day_tuesday() },
  { value: 4, label: m.common_day_wednesday() },
  { value: 5, label: m.common_day_thursday() },
  { value: 6, label: m.common_day_friday() },
];

let isEditing = $state(false);
let editingSlot = $state<TimeSlotDto | null>(null);
let nameAr = $state("");
let name = $state("");
let startTime = $state("08:00");
let endTime = $state("08:45");
let slotIndex = $state("1");
let selectedDays = $state<number[]>([7, 1, 2, 3, 4, 5]);
let isBreak = $state(false);
let breakType = $state("recess");
let isSubmitting = $state(false);

const sortedTimeSlots = $derived(
  [...timeSlots].sort((left, right) => {
    const dayOrder = [7, 1, 2, 3, 4, 5, 6];
    return (
      dayOrder.indexOf(left.day_of_week) -
        dayOrder.indexOf(right.day_of_week) ||
      left.slot_index - right.slot_index
    );
  }),
);
const duration = $derived.by(() => {
  const [startHour, startMinute] = startTime.split(":").map(Number);
  const [endHour, endMinute] = endTime.split(":").map(Number);
  return endHour * 60 + endMinute - (startHour * 60 + startMinute);
});

function resetForm() {
  isEditing = false;
  editingSlot = null;
  nameAr = "";
  name = "";
  startTime = "08:00";
  endTime = "08:45";
  slotIndex = String(
    Math.max(0, ...timeSlots.map((slot) => Number(slot.slot_index) || 0)) + 1,
  );
  selectedDays = [7, 1, 2, 3, 4, 5];
  isBreak = false;
  breakType = "recess";
}

function startAdd() {
  resetForm();
  isEditing = true;
  const nextIndex =
    Math.max(0, ...timeSlots.map((slot) => Number(slot.slot_index) || 0)) + 1;
  slotIndex = String(nextIndex);
  nameAr = m.academics_schedule_placeholder_period({ number: nextIndex });
  name = m.academics_schedule_placeholder_period_en({ number: nextIndex });
}

function startEdit(slot: TimeSlotDto) {
  editingSlot = slot;
  isEditing = true;
  nameAr = slot.name_ar ?? "";
  name = slot.name ?? "";
  startTime = slot.start_time.slice(0, 5);
  endTime = slot.end_time.slice(0, 5);
  slotIndex = String(slot.slot_index);
  selectedDays = [slot.day_of_week];
  isBreak = slot.is_break;
  breakType = slot.break_type ?? "recess";
}

function toggleDay(day: number) {
  if (editingSlot) {
    selectedDays = [day];
    return;
  }
  selectedDays = selectedDays.includes(day)
    ? selectedDays.filter((selected) => selected !== day)
    : [...selectedDays, day];
}

async function handleSave(event: SubmitEvent) {
  event.preventDefault();
  const index = Number(slotIndex);
  const label = name.trim() || nameAr.trim();
  if (
    !periodId ||
    !label ||
    !nameAr.trim() ||
    !startTime ||
    !endTime ||
    duration <= 0 ||
    !Number.isInteger(index) ||
    index < 1 ||
    selectedDays.length === 0 ||
    (editingSlot && selectedDays.length !== 1)
  ) {
    toast.error(m.academics_schedule_time_slot_required());
    return;
  }

  isSubmitting = true;
  try {
    if (editingSlot) {
      await api.updateTimeSlot(editingSlot.id, {
        day_of_week: selectedDays[0],
        slot_index: index,
        name: label,
        name_ar: nameAr.trim(),
        start_time: startTime,
        end_time: endTime,
        is_break: isBreak,
        break_type: isBreak ? breakType : null,
      });
      toast.success(m.academics_schedule_time_slot_updated());
    } else {
      const slots = selectedDays.map((day) => ({
        day_of_week: day,
        slot_index: index,
        name: label,
        name_ar: nameAr.trim(),
        start_time: startTime,
        end_time: endTime,
        is_break: isBreak,
        break_type: isBreak ? breakType : null,
      }));
      await api.batchCreateTimeSlots({ period_id: periodId, slots });
      toast.success(m.academics_schedule_time_slot_added());
    }
    resetForm();
    onSuccess?.();
  } catch (error) {
    toast.error(extractErrorMessage(error));
  } finally {
    isSubmitting = false;
  }
}

async function handleDelete(slotId: string) {
  try {
    await api.deleteTimeSlot(slotId);
    toast.success(m.academics_schedule_time_slot_deleted());
    if (editingSlot?.id === slotId) resetForm();
    onSuccess?.();
  } catch (error) {
    toast.error(extractErrorMessage(error));
  }
}

function getWeekdayLabel(day: number): string {
  return weekdays.find((item) => item.value === day)?.label ?? String(day);
}

function getDuration(slot: TimeSlotDto): number {
  const [startHour, startMinute] = slot.start_time.split(":").map(Number);
  const [endHour, endMinute] = slot.end_time.split(":").map(Number);
  return endHour * 60 + endMinute - (startHour * 60 + startMinute);
}
</script>

<Sheet bind:open>
  <SheetContent
    contentClass="data-[side=end]:sm:!max-w-xl"
    class="flex h-full w-full flex-col overflow-hidden p-0 text-start"
  >
    <SheetHeader class="border-b border-border/50 px-5 py-4">
      <SheetTitle>{m.academics_schedule_time_slots_heading()}</SheetTitle>
      <p class="text-xs text-muted-foreground">{m.academics_studio_manage_slots()}</p>
    </SheetHeader>

    <div class="min-h-0 flex-1 space-y-4 overflow-y-auto p-5">
      {#if isEditing}
        <form class="space-y-4 rounded-xl border border-primary/20 bg-card p-4 shadow-xs" onsubmit={handleSave}>
          <div class="flex items-center justify-between gap-3">
            <h3 class="text-sm font-semibold text-foreground">
              {editingSlot ? m.academics_schedule_time_slot_edit() : m.academics_schedule_time_slot_new()}
            </h3>
            <Button
              type="button"
              variant="ghost"
              size="icon"
              class="h-7 w-7"
              onclick={resetForm}
              aria-label={m.common_action_cancel()}
            >
              <XMark size={14} />
            </Button>
          </div>

          <BilingualNameFields
            arId="studio-slot-name-ar"
            enId="studio-slot-name-en"
            arLabel={m.academics_schedule_name_ar()}
            enLabel={m.academics_schedule_name_en()}
            bind:arValue={nameAr}
            bind:enValue={name}
            arPlaceholder={m.academics_schedule_name_ar()}
            enPlaceholder={m.academics_schedule_name_en()}
            kind="label"
            requestSuggestion={suggestLocalTranslation}
            arRequired
            disabled={isSubmitting}
            loadingMessage={m.bilingual_entry_loading()}
            failureMessage={m.bilingual_entry_failure()}
          />

          <div class="grid gap-3 sm:grid-cols-3">
            <div class="space-y-1.5">
              <Label for="studio-slot-index" class="text-xs">{m.academics_studio_slot_index()}</Label>
              <Input
                id="studio-slot-index"
                type="number"
                min="1"
                step="1"
                bind:value={slotIndex}
                disabled={isSubmitting}
                required
              />
            </div>
            <div class="space-y-1.5">
              <Label for="studio-slot-start" class="text-xs">{m.academics_schedule_start_time()}</Label>
              <Input
                id="studio-slot-start"
                type="time"
                bind:value={startTime}
                disabled={isSubmitting}
                required
              />
            </div>
            <div class="space-y-1.5">
              <Label for="studio-slot-end" class="text-xs">{m.academics_schedule_end_time()}</Label>
              <Input
                id="studio-slot-end"
                type="time"
                bind:value={endTime}
                disabled={isSubmitting}
                required
              />
            </div>
          </div>

          <fieldset class="space-y-2">
            <legend class="text-xs font-medium text-foreground">{m.academics_studio_teaching_days()}</legend>
            <div class="flex flex-wrap gap-1.5">
              {#each weekdays as day (day.value)}
                <button
                  type="button"
                  aria-pressed={selectedDays.includes(day.value)}
                  disabled={isSubmitting}
                  class="rounded-lg border px-2.5 py-1.5 text-xs font-medium transition-colors {selectedDays.includes(day.value)
                    ? 'border-primary bg-primary/10 text-primary'
                    : 'border-border/60 bg-background text-muted-foreground hover:bg-muted'}"
                  onclick={() => toggleDay(day.value)}
                >
                  {day.label}
                </button>
              {/each}
            </div>
          </fieldset>

          <div class="flex flex-wrap items-center gap-4">
            <label class="flex items-center gap-2 text-xs font-medium text-foreground">
              <input
                type="checkbox"
                bind:checked={isBreak}
                disabled={isSubmitting}
                class="size-4 accent-primary"
              />
              {m.academics_schedule_break_slot()}
            </label>
            {#if isBreak}
              <div class="min-w-48 flex-1">
                <Select
                  type="single"
                  value={breakType}
                  onValueChange={(value) => (breakType = value ?? "recess")}
                >
                  <SelectTrigger class="h-9 text-xs">
                    <span>{m.academics_schedule_break_type()}</span>
                  </SelectTrigger>
                  <SelectContent class="text-start">
                    <SelectItem value="recess" class="text-xs">{m.common_onboarding_break_recess()}</SelectItem>
                    <SelectItem value="lunch" class="text-xs">{m.common_onboarding_break_lunch()}</SelectItem>
                    <SelectItem value="prayer" class="text-xs">{m.common_onboarding_break_prayer()}</SelectItem>
                    <SelectItem value="morning" class="text-xs">{m.academics_schedule_morning_assembly()}</SelectItem>
                  </SelectContent>
                </Select>
              </div>
            {/if}
          </div>

          <div class="flex items-center justify-between border-t border-border/50 pt-3">
            <span class="text-xs text-muted-foreground">
              {m.academics_schedule_minutes()}: {duration > 0 ? duration : "—"}
            </span>
            <div class="flex gap-2">
              <Button type="button" variant="outline" size="sm" onclick={resetForm}>
                {m.common_action_cancel()}
              </Button>
              <Button type="submit" size="sm" class="gap-1.5" disabled={isSubmitting}>
                <FloppyDisk size={14} />
                {m.academics_schedule_save_slot()}
              </Button>
            </div>
          </div>
        </form>
      {:else}
        <Button
          type="button"
          variant="outline"
          class="w-full gap-2 border-dashed text-xs"
          disabled={!periodId}
          onclick={startAdd}
        >
          <Plus size={14} />
          {m.academics_schedule_add_slot()}
        </Button>
      {/if}

      <section class="space-y-2">
        <div class="flex items-center justify-between">
          <h3 class="text-xs font-semibold text-foreground">{m.academics_schedule_current_slots()}</h3>
          <Badge variant="secondary">{timeSlots.length}</Badge>
        </div>
        {#if sortedTimeSlots.length === 0}
          <div class="flex flex-col items-center gap-2 rounded-xl border border-dashed border-border/60 p-8 text-center">
            <ClockRotateRight size={22} class="text-muted-foreground/70" />
            <p class="text-xs text-muted-foreground">{m.academics_schedule_no_slots()}</p>
          </div>
        {:else}
          <div class="space-y-2">
            {#each sortedTimeSlots as slot (slot.id)}
              <article class="flex items-center justify-between gap-3 rounded-lg border border-border/60 bg-card p-3 {slot.is_break ? 'border-amber-500/30 bg-amber-500/[0.03]' : ''}">
                <div class="flex min-w-0 items-center gap-3">
                  <span class="flex size-9 shrink-0 items-center justify-center rounded-lg bg-muted/50 text-xs font-bold text-foreground">
                    {#if slot.is_break}
                      <Heart size={14} class="text-amber-600" />
                    {:else}
                      {slot.slot_index}
                    {/if}
                  </span>
                  <div class="min-w-0">
                    <p class="truncate text-xs font-semibold text-foreground">
                      {getWeekdayLabel(slot.day_of_week)} · {slot.name_ar || slot.name || m.academics_schedule_placeholder_period_en({ number: slot.slot_index })}
                    </p>
                    <p class="mt-1 font-mono text-[10px] text-muted-foreground">
                      {slot.start_time.slice(0, 5)}–{slot.end_time.slice(0, 5)}
                      · {m.academics_schedule_minutes()}: {getDuration(slot)}
                    </p>
                  </div>
                </div>
                <div class="flex shrink-0 items-center gap-1">
                  <Button
                    type="button"
                    variant="ghost"
                    size="icon"
                    class="size-8"
                    onclick={() => startEdit(slot)}
                    aria-label={m.academics_schedule_edit()}
                  >
                    <Pencil size={13} />
                  </Button>
                  <Button
                    type="button"
                    variant="ghost"
                    size="icon"
                    class="size-8 text-destructive hover:bg-destructive/10"
                    onclick={() => handleDelete(slot.id)}
                    aria-label={m.common_action_delete()}
                  >
                    <Trash size={13} />
                  </Button>
                </div>
              </article>
            {/each}
          </div>
        {/if}
      </section>
    </div>

    <SheetFooter class="border-t border-border/50 p-4">
      <Button variant="outline" class="w-full" onclick={() => (open = false)}>
        {m.common_action_close()}
      </Button>
    </SheetFooter>
  </SheetContent>
</Sheet>
