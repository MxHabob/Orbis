<!--
@component LessonCreateSheet
A slide-over Sheet component for scheduling a lesson.
Uses the shared @orbis/ui-svelte/sheet component and integrates real-time conflict checking.
-->
<script lang="ts">
import { commands } from "$lib/services/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { 
  Sheet, SheetContent, SheetHeader, SheetTitle 
} from "@orbis/ui-svelte/sheet";
import { ShieldAlert, Calendar, X } from "lucide-svelte";
import { toast } from "svelte-sonner";
import { getLocale } from "$lib/paraglide/runtime";
import { fade } from "svelte/transition";

let { 
  open = $bindable(false), 
  orgId, 
  cycleId,
  classId,
  dayOfWeek,
  timeSlotId,
  subjects = [],
  teachers = [],
  onSuccess 
}: { 
  open: boolean; 
  orgId: string; 
  cycleId: string;
  classId: string;
  dayOfWeek: number;
  timeSlotId: string;
  subjects: any[];
  teachers: any[];
  onSuccess: () => void;
} = $props();

const isAr = $derived(getLocale() === "ar");

// Form state
let isSubmitting = $state(false);
let formSubjectId = $state("");
let formTeacherId = $state("");
let formRoom = $state("");

// Conflict state
let conflictError = $state<string | null>(null);
let isCheckingConflict = $state(false);

const weekdays = $derived([
  { value: 1, label: isAr ? "الأحد" : "Sunday" },
  { value: 2, label: isAr ? "الأثنين" : "Monday" },
  { value: 3, label: isAr ? "الثلاثاء" : "Tuesday" },
  { value: 4, label: isAr ? "الأربعاء" : "Wednesday" },
  { value: 5, label: isAr ? "الخميس" : "Thursday" },
]);

// Check conflict when teacher, room or subject selection changes
$effect(() => {
  if (open && (formTeacherId || formRoom || formSubjectId)) {
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
      const first = conflicts[0];
      conflictError = isAr 
        ? `تعارض: المعلم أو الغرفة محجوزة بالفعل في هذا الوقت! (${first.conflict_type})`
        : `Conflict detected: Teacher or Room already booked! (${first.conflict_type})`;
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
    toast.error(isAr ? "الرجاء اختيار المادة" : "Please select subject");
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
    toast.success(isAr ? "تم إدراج الحصة بنجاح" : "Lesson scheduled successfully");
    open = false;
    
    // Reset fields
    formSubjectId = "";
    formTeacherId = "";
    formRoom = "";
    conflictError = null;

    onSuccess();
  } catch (e) {
    toast.error(isAr ? "فشل جدولة الحصة" : "Failed to schedule lesson");
  } finally {
    isSubmitting = false;
  }
}
</script>

<Sheet bind:open>
	<SheetContent class="w-full sm:max-w-md bg-card border-l border-border/80 flex flex-col h-full overflow-hidden text-xs">
		<SheetHeader class="pb-4 border-b border-border shrink-0 select-none text-start">
			<SheetTitle class="text-sm font-bold text-foreground uppercase tracking-wider flex items-center gap-2">
				<Calendar size={14} class="text-primary" />
				<span>{isAr ? "جدولة حصة دراسية" : "Schedule Curricular Lesson"}</span>
			</SheetTitle>
			<p class="text-[10px] text-muted-foreground mt-0.5">
				{isAr 
					? `إدراج حصة جديدة يوم ${weekdays.find(w => w.value === dayOfWeek)?.label}`
					: `Schedule a new lesson on ${weekdays.find(w => w.value === dayOfWeek)?.label}`}
			</p>
		</SheetHeader>

		<!-- Conflict Warning Box -->
		{#if conflictError}
			<div class="alert-brand-warning flex items-start gap-2.5 p-3 rounded-lg text-[10px] mt-4 text-start" transition:fade>
				<ShieldAlert size={14} class="text-amber-500 shrink-0 mt-0.5" />
				<span class="font-medium">{conflictError}</span>
			</div>
		{/if}

		<!-- Scrollable Form Container -->
		<form onsubmit={handleSubmit} class="flex-1 overflow-y-auto py-4 flex flex-col gap-4 scrollbar-hide">
			<!-- Subject Selector -->
			<div class="flex flex-col gap-1.5 text-start">
				<label for="sched_sub" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
					{isAr ? "المادة الدراسية *" : "Select Subject *"}
				</label>
				<select 
					id="sched_sub"
					bind:value={formSubjectId}
					class="bg-background text-foreground border border-border rounded-md px-3 py-1.5 text-xs focus:ring-1 focus:ring-ring outline-none"
					required
				>
					<option value="">{isAr ? "-- اختر المادة --" : "-- Select Subject --"}</option>
					{#each subjects as sub (sub.id)}
						<option value={String(sub.id)}>{sub.name_ar || sub.name}</option>
					{/each}
				</select>
			</div>

			<!-- Teacher Selector -->
			<div class="flex flex-col gap-1.5 text-start">
				<label for="sched_teach" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
					{isAr ? "المعلم المشرف" : "Select Teacher"}
				</label>
				<select 
					id="sched_teach"
					bind:value={formTeacherId}
					class="bg-background text-foreground border border-border rounded-md px-3 py-1.5 text-xs focus:ring-1 focus:ring-ring outline-none"
				>
					<option value="">{isAr ? "-- بدون معلم (اختياري) --" : "-- No Teacher (Optional) --"}</option>
					{#each teachers as teach (teach.id)}
						<option value={String(teach.id)}>{teach.full_name}</option>
					{/each}
				</select>
			</div>

			<!-- Room Number -->
			<div class="flex flex-col gap-1.5 text-start">
				<label for="sched_room" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
					{isAr ? "رقم الفصل / الغرفة" : "Classroom Room / Location"}
				</label>
				<Input 
					id="sched_room"
					type="text" 
					bind:value={formRoom}
					placeholder="e.g., Room 104"
					class="input-sunken text-xs py-1.5"
				/>
			</div>

			<!-- Form Actions -->
			<div class="flex gap-2 justify-end mt-6 pt-4 border-t border-border shrink-0">
				<Button 
					type="button" 
					variant="ghost" 
					class="text-xs h-9 px-4" 
					onclick={() => open = false}
					disabled={isSubmitting}
				>
					{isAr ? "إلغاء" : "Cancel"}
				</Button>
				<Button 
					type="submit" 
					variant="lime" 
					class="text-xs h-9 px-4 font-semibold"
					disabled={isSubmitting || !!conflictError}
				>
					{isSubmitting ? (isAr ? "جاري الحفظ..." : "Saving...") : (isAr ? "إدراج الحصة" : "Schedule")}
				</Button>
			</div>
		</form>
	</SheetContent>
</Sheet>
