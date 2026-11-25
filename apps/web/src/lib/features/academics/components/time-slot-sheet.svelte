<script lang="ts">
import { toast } from "@orbis/ui";
import { api, extractErrorMessage, type TimeSlotDto } from "$lib/api";
import { getWeekdayOptions } from "$lib/core/i18n";
import { Button } from "@orbis/ui/button";
import { Checkbox } from "@orbis/ui/checkbox";
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
import { ClockRotateRight, FloppyDisk, Plus, XMark } from "@orbis/icons";
import { BilingualNameFields } from "@orbis/ui/bilingual-name-fields";
import { suggestLocalTranslation } from "$lib/shared/translation/local-engine";
import { authStore, uiStore } from "$lib/stores";
import * as m from "$lib/paraglide/messages";
import TimeSlotGenerator from "./time-slot-generator.svelte";
import TimeSlotList from "./time-slot-list.svelte";
import {
  formatMinutesToTime,
  getBreakLabel,
  getDefaultStudyDays,
  getPeriodNames,
  parseTimeToMinutes,
} from "./time-slot-helpers";

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

const weekdays = getWeekdayOptions();
const weekdayOrder = new Map(weekdays.map((day, idx) => [day.value, idx]));

let mode = $state<"idle" | "single" | "generate">("idle");
let editingSlot = $state<TimeSlotDto | null>(null);
let nameAr = $state("");
let name = $state("");
let startTime = $state("08:00");
let endTime = $state("08:45");
let slotIndex = $state("1");
let selectedDays = $state<number[]>([]);
let isBreak = $state(false);
let breakType = $state("recess");
let isSubmitting = $state(false);

const preferenceScope = () => ({
  userId: authStore.userId,
  organizationId: authStore.orgId,
});

const duration = $derived(
  parseTimeToMinutes(endTime) - parseTimeToMinutes(startTime),
);

function resetForm() {
  mode = "idle";
  editingSlot = null;
  nameAr = "";
  name = "";
  isBreak = false;
  breakType = "recess";
}

function startAdd() {
  editingSlot = null;
  mode = "single";
  const nextIndex =
    Math.max(0, ...timeSlots.map((slot) => Number(slot.slot_index) || 0)) + 1;
  slotIndex = String(nextIndex);

  const teachingCount =
    Math.max(
      0,
      ...timeSlots
        .filter((slot) => !slot.is_break)
        .map((slot) => Number(slot.slot_index) || 0),
    ) + 1;
  const autoNames = getPeriodNames(teachingCount);
  nameAr = autoNames.ar;
  name = autoNames.en;

  const pref = uiStore.getTimeSlotRangePreference(preferenceScope());
  const prefDur = Math.max(
    15,
    parseTimeToMinutes(pref.endTime) - parseTimeToMinutes(pref.startTime) || 45,
  );
  const latest = [...timeSlots].sort((a, b) =>
    b.end_time.localeCompare(a.end_time),
  )[0];
  if (latest) {
    startTime = latest.end_time.slice(0, 5);
    endTime = formatMinutesToTime(parseTimeToMinutes(startTime) + prefDur);
  } else {
    startTime = pref.startTime;
    endTime = pref.endTime;
  }

  if (selectedDays.length === 0) {
    selectedDays = getDefaultStudyDays(timeSlots, weekdayOrder);
  }
  isBreak = false;
  breakType = "recess";
}

function startEdit(slot: TimeSlotDto) {
  editingSlot = slot;
  mode = "single";
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
    ? selectedDays.filter((item) => item !== day)
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
      await api.batchCreateTimeSlots({
        period_id: periodId,
        slots: selectedDays.map((day) => ({
          day_of_week: day,
          slot_index: index,
          name: label,
          name_ar: nameAr.trim(),
          start_time: startTime,
          end_time: endTime,
          is_break: isBreak,
          break_type: isBreak ? breakType : null,
        })),
      });
      toast.success(m.academics_schedule_time_slot_added());
    }
    uiStore.setTimeSlotRangePreference(preferenceScope(), {
      startTime,
      endTime,
    });
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

async function handleDeleteGroup(slots: TimeSlotDto[]) {
  if (slots.length === 0) return;
  isSubmitting = true;
  try {
    await Promise.all(slots.map((slot) => api.deleteTimeSlot(slot.id)));
    toast.success(m.academics_schedule_time_slot_deleted());
    if (editingSlot && slots.some((slot) => slot.id === editingSlot?.id)) {
      resetForm();
    }
    onSuccess?.();
  } catch (error) {
    toast.error(extractErrorMessage(error));
  } finally {
    isSubmitting = false;
  }
}
</script>

<Sheet bind:open>
  <SheetContent
    contentClass="data-[side=end]:sm:!max-w-xl"
    class="flex h-full w-full flex-col overflow-hidden p-0 text-start"
  >
    <SheetHeader class="border-b border-border/50 px-5 py-4">
      <SheetTitle>{m.academics_schedule_time_slots_heading()}</SheetTitle>
    </SheetHeader>

    <div class="min-h-0 flex-1 space-y-4 overflow-y-auto p-5">
      {#if mode === "idle"}
        <div class="grid grid-cols-1 gap-2 sm:grid-cols-2">
          <Button
            type="button"
            class="gap-2 text-xs font-semibold"
            disabled={!periodId}
            onclick={() => {
              editingSlot = null;
              mode = "generate";
            }}
          >
            <ClockRotateRight size={14} />
            {m.academics_schedule_quick_generate()}
          </Button>
          <Button
            type="button"
            variant="outline"
            class="gap-2 border-dashed text-xs"
            disabled={!periodId}
            onclick={startAdd}
          >
            <Plus size={14} />
            {m.academics_schedule_add_slot()}
          </Button>
        </div>
      {:else if mode === "generate"}
        <TimeSlotGenerator
          {periodId}
          {timeSlots}
          bind:isSubmitting
          onCancel={resetForm}
          onSuccess={() => {
            resetForm();
            onSuccess?.();
          }}
        />
      {:else}
        <form
          class="space-y-3 rounded-xl border border-primary/20 bg-card p-3.5 shadow-xs"
          onsubmit={handleSave}
        >
          <div class="flex items-center justify-between">
            <h3 class="text-sm font-semibold text-foreground">
              {editingSlot
                ? m.academics_schedule_time_slot_edit()
                : m.academics_schedule_time_slot_new()}
            </h3>
            <Button
              type="button"
              variant="ghost"
              size="icon"
              class="size-7"
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
            <div class="space-y-1">
              <Label for="studio-slot-index" class="text-xs">
                {m.academics_studio_slot_index()}
              </Label>
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
            <div class="space-y-1">
              <Label for="studio-slot-start" class="text-xs">
                {m.academics_schedule_start_time()}
              </Label>
              <Input
                id="studio-slot-start"
                type="time"
                bind:value={startTime}
                disabled={isSubmitting}
                required
              />
            </div>
            <div class="space-y-1">
              <Label for="studio-slot-end" class="text-xs">
                {m.academics_schedule_end_time()}
              </Label>
              <Input
                id="studio-slot-end"
                type="time"
                bind:value={endTime}
                disabled={isSubmitting}
                required
              />
            </div>
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

          <div class="flex flex-wrap items-center gap-3">
            <div class="flex items-center gap-2">
              <Checkbox
                id="single-slot-break"
                checked={isBreak}
                onCheckedChange={(value: boolean | "indeterminate") =>
                  (isBreak = Boolean(value))}
                disabled={isSubmitting}
              />
              <Label for="single-slot-break" class="cursor-pointer text-xs">
                {m.academics_schedule_break_slot()}
              </Label>
            </div>
            {#if isBreak}
              <div class="min-w-44 flex-1">
                <Select
                  type="single"
                  value={breakType}
                  onValueChange={(value) => (breakType = value ?? "recess")}
                >
                  <SelectTrigger class="h-8 text-xs">
                    <span>{getBreakLabel(breakType)}</span>
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
            {/if}
          </div>

          <div class="flex items-center justify-between border-t border-border/50 pt-2.5">
            <span class="text-xs text-muted-foreground">
              {m.academics_schedule_minutes()}: {duration > 0 ? duration : "—"}
            </span>
            <div class="flex gap-2">
              <Button type="button" variant="outline" size="sm" onclick={resetForm}>
                {m.common_action_cancel()}
              </Button>
              <Button
                type="submit"
                size="sm"
                class="gap-1.5"
                disabled={isSubmitting || selectedDays.length === 0}
              >
                <FloppyDisk size={14} />
                {m.academics_schedule_save_slot()}
              </Button>
            </div>
          </div>
        </form>
      {/if}

      <TimeSlotList
        {timeSlots}
        {isSubmitting}
        onEdit={startEdit}
        onDelete={handleDelete}
        onDeleteGroup={handleDeleteGroup}
      />
    </div>

    <SheetFooter class="border-t border-border/50 p-4">
      <Button variant="outline" class="w-full" onclick={() => (open = false)}>
        {m.common_action_close()}
      </Button>
    </SheetFooter>
  </SheetContent>
</Sheet>
