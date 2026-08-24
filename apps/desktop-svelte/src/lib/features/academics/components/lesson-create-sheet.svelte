<!--
@component LessonCreateSheet
A slide-over Sheet component for scheduling a lesson.
Uses the shared @orbis/ui-svelte/sheet component and integrates real-time conflict checking.
-->
<script lang="ts">
import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
  SheetDescription,
} from "@orbis/ui-svelte/sheet";
import LessonForm from "./lesson-form.svelte";

let {
  open = $bindable(false),
  orgId,
  periodId,
  classId,
  dayOfWeek,
  timeSlotId,
  timeSlots = [],
  lesson = null,
  subjects = [],
  teachers = [],
  rooms = [],
  onSuccess,
}: {
  open: boolean;
  orgId: string;
  periodId: string;
  classId: string;
  dayOfWeek: number;
  timeSlotId: string;
  timeSlots?: any[];
  lesson?: any | null;
  subjects: any[];
  teachers: any[];
  rooms: any[];
  onSuccess: () => void;
} = $props();

</script>

<Sheet bind:open>
	<SheetContent class="flex flex-col h-full sm:max-w-md overflow-hidden p-0 gap-0">
    <LessonForm
			{orgId} 
			{periodId} 
			{classId} 
			{dayOfWeek} 
			{timeSlotId} 
      {timeSlots}
      {lesson}
			{subjects} 
			{teachers} 
      {rooms}
			onSuccess={() => {
				open = false;
				onSuccess();
			}} 
			onCancel={() => open = false} 
		/>
	</SheetContent>
</Sheet>
