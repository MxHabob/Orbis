<script lang="ts">
import { commands } from "$lib/core/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { ShieldAlert } from "lucide-svelte";
import { toast } from "svelte-sonner";
import { getLocale } from "$lib/paraglide/runtime";
import { fade } from "svelte/transition";
import * as m from "$lib/paraglide/messages";

let {
  orgId,
  cycleId,
  classId,
  dayOfWeek,
  timeSlotId,
  subjects = [],
  teachers = [],
  onSuccess,
  onCancel,
}: {
  orgId: string;
  cycleId: string;
  classId: string;
  dayOfWeek: number;
  timeSlotId: string;
  subjects: any[];
  teachers: any[];
  onSuccess: () => void;
  onCancel: () => void;
} = $props();

// Form state
let isSubmitting = $state(false);
let formSubjectId = $state("");
let formTeacherId = $state("");
let formRoom = $state("");

// Conflict state
let conflictError = $state<string | null>(null);
let isCheckingConflict = $state(false);

const weekdays = $derived([
  { value: 1, label: m.schedule_page_day_sun() },
  { value: 2, label: m.schedule_page_day_mon() },
  { value: 3, label: m.schedule_page_day_tue() },
  { value: 4, label: m.schedule_page_day_wed() },
  { value: 5, label: m.schedule_page_day_thu() },
]);

// Check conflict when teacher, room or subject selection changes
$effect(() => {
  if (formTeacherId || formRoom || formSubjectId) {
    checkScheduleConflict();
  }
});

async function checkScheduleConflict() {
  if (!formSubjectId || !classId || !timeSlotId) return;
  isCheckingConflict = true;
  conflictError = null;
  try {
    const conflicts = await commands.schedule.checkConflicts({
      orgId,
      cycleId,
      timeSlotId,
      dayOfWeek,
      teacherId: formTeacherId || "no_teacher",
      classId,
      roomId: formRoom || undefined,
    });
    if (conflicts && conflicts.length > 0) {
      const first = conflicts[0] as any;
      conflictError = m.lesson_sheet_conflict_err({
        type: String(first.conflict_type || ""),
      });
    }
  } catch (e) {
    console.warn("Conflict check bypassed:", e);
  } finally {
    isCheckingConflict = false;
  }
}

async function handleSubmit(e: SubmitEvent) {
  e.preventDefault();
  if (!formSubjectId) {
    toast.error(m.lesson_toast_error_subject());
    return;
  }
  isSubmitting = true;
  try {
    const request = {
      cycle_id: cycleId,
      class_id: classId,
      subject_id: formSubjectId,
      teacher_id: formTeacherId || undefined,
      time_slot_id: timeSlotId,
      day_of_week: dayOfWeek,
      room: formRoom || undefined,
    };
    await commands.schedule.createLesson(orgId, request);
    toast.success(m.lesson_toast_success());

    // Reset fields
    formSubjectId = "";
    formTeacherId = "";
    formRoom = "";
    conflictError = null;

    onSuccess();
  } catch (e) {
    toast.error(m.lesson_toast_error());
  } finally {
    isSubmitting = false;
  }
}
</script>

<!-- Conflict Warning Box -->
{#if conflictError}
	<div class="alert-brand-warning flex items-start gap-2.5 p-3 rounded-lg text-xs mt-4 text-start bg-amber-500/10 border border-amber-500/20 text-amber-600 dark:text-amber-400" transition:fade>
		<ShieldAlert size={16} class="text-amber-500 shrink-0 mt-0.5" />
		<span class="font-medium">{conflictError}</span>
	</div>
{/if}

<form onsubmit={handleSubmit} class="flex-1 overflow-y-auto py-6 flex flex-col gap-6 scrollbar-hide">
	<!-- Subject Selector -->
	<div class="flex flex-col gap-2 text-start">
		<label for="sched_sub" class="text-sm font-medium text-foreground">
			{m.lesson_label_subject()}
		</label>
		<select 
			id="sched_sub"
			bind:value={formSubjectId}
			class="bg-background text-foreground border border-border rounded-md px-3 py-2 text-sm focus:ring-1 focus:ring-ring outline-none h-9"
			required
		>
			<option value="">{m.lesson_option_select_subject()}</option>
			{#each subjects as sub (sub.id)}
				<option value={String(sub.id)}>{sub.name_ar || sub.name}</option>
			{/each}
		</select>
	</div>

	<!-- Teacher Selector -->
	<div class="flex flex-col gap-2 text-start">
		<label for="sched_teach" class="text-sm font-medium text-foreground">
			{m.lesson_label_teacher()}
		</label>
		<select 
			id="sched_teach"
			bind:value={formTeacherId}
			class="bg-background text-foreground border border-border rounded-md px-3 py-2 text-sm focus:ring-1 focus:ring-ring outline-none h-9"
		>
			<option value="">{m.lesson_option_no_teacher()}</option>
			{#each teachers as teach (teach.id)}
				<option value={String(teach.id)}>{getLocale() === "ar" ? (teach.name_ar || teach.full_name) : teach.full_name}</option>
			{/each}
		</select>
	</div>

	<!-- Room Number -->
	<div class="flex flex-col gap-2 text-start">
		<label for="sched_room" class="text-sm font-medium text-foreground">
			{m.lesson_label_room()}
		</label>
		<Input 
			id="sched_room"
			type="text" 
			bind:value={formRoom}
			placeholder="e.g., Room 104"
		/>
	</div>

	<!-- Form Actions -->
	<div class="flex gap-2 justify-end mt-auto pt-6">
		<Button 
			type="button" 
			variant="ghost" 
			onclick={onCancel}
			disabled={isSubmitting}
		>
			{m.lesson_button_cancel()}
		</Button>
		<Button 
			type="submit" 
			variant="lime" 
			class="font-semibold"
			disabled={isSubmitting || !!conflictError}
		>
			{isSubmitting ? m.lesson_toast_saving() : m.lesson_button_save()}
		</Button>
	</div>
</form>
