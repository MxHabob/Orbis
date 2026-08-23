<!--
@component LessonCreateSheet
A slide-over Sheet component for scheduling a lesson.
Uses the shared @orbis/ui-svelte/sheet component and integrates real-time conflict checking.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
  SheetDescription,
} from "@orbis/ui-svelte/sheet";
import { Calendar } from "@lucide/svelte";
import LessonForm from "./lesson-form.svelte";

let {
  open = $bindable(false),
  orgId,
  periodId,
  classId,
  dayOfWeek,
  timeSlotId,
  subjects = [],
  teachers = [],
  onSuccess,
}: {
  open: boolean;
  orgId: string;
  periodId: string;
  classId: string;
  dayOfWeek: number;
  timeSlotId: string;
  subjects: any[];
  teachers: any[];
  onSuccess: () => void;
} = $props();

const weekdays = $derived([
  { value: 1, label: m.common_schedule_page_day_sun() },
  { value: 2, label: m.common_schedule_page_day_mon() },
  { value: 3, label: m.common_schedule_page_day_tue() },
  { value: 4, label: m.common_schedule_page_day_wed() },
  { value: 5, label: m.common_schedule_page_day_thu() },
]);
</script>

<Sheet bind:open>
	<SheetContent class="flex flex-col h-full sm:max-w-md overflow-hidden p-0 gap-0">
		<LessonForm 
			{orgId} 
			{periodId} 
			{classId} 
			{dayOfWeek} 
			{timeSlotId} 
			{subjects} 
			{teachers} 
			onSuccess={() => {
				open = false;
				onSuccess();
			}} 
			onCancel={() => open = false} 
		/>
	</SheetContent>
</Sheet>
