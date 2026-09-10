<script lang="ts">
import { Sheet, SheetContent } from "@orbis/ui/sheet";
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
  classes = [],
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
  classes?: any[];
  subjects: any[];
  teachers: any[];
  rooms: any[];
  onSuccess: () => void;
} = $props();
</script>

<Sheet bind:open>
  <SheetContent class="flex h-full w-full flex-col gap-0 overflow-hidden p-0 sm:max-w-md">
    <LessonForm
      {orgId}
      {periodId}
      {classId}
      {dayOfWeek}
      {timeSlotId}
      {timeSlots}
      {lesson}
      {classes}
      {subjects}
      {teachers}
      {rooms}
      onSuccess={() => {
        open = false;
        onSuccess();
      }}
      onCancel={() => (open = false)}
    />
  </SheetContent>
</Sheet>
