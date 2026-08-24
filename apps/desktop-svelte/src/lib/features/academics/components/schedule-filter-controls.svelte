<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui-svelte/button";
import ClassSelect from "$lib/features/academics/components/class-select.svelte";
import TeacherSelect from "$lib/features/academics/components/teacher-select.svelte";

interface Props {
  viewMode: "whole" | "class" | "teacher" | "subject" | "room";
  selectedClassId: string;
  selectedTeacherId: string;
  selectedSubjectId: string;
  selectedRoomId: string;
  classes: any[];
  teachers: any[];
  subjects?: any[];
  rooms?: any[];
}

let {
  viewMode = $bindable(),
  selectedClassId = $bindable(),
  selectedTeacherId = $bindable(),
  selectedSubjectId = $bindable(),
  selectedRoomId = $bindable(),
  classes,
  teachers,
  subjects = [],
  rooms = [],
}: Props = $props();

const modes = [
  { value: "whole", label: "Whole" },
  { value: "class", label: "Classes" },
  { value: "teacher", label: "Teachers" },
  { value: "subject", label: "Subjects" },
  { value: "room", label: "Rooms" },
] as const;
</script>

<div class="flex flex-col md:flex-row md:items-center md:justify-between gap-4 shrink-0 select-none">
  <div class="flex items-center gap-3">
    <div class="flex p-0.5">
      {#each modes as mode}
        <Button
          variant="secondary"
          class="text-[10px] font-bold px-3 py-1.5 h-8 rounded-md transition-all duration-150 cursor-pointer {viewMode === mode.value ? 'bg-card text-foreground shadow-sm' : 'text-muted-foreground hover:bg-transparent'}"
          onclick={() => viewMode = mode.value}
        >{mode.label}</Button>
      {/each}
    </div>
  </div>

  <!-- Selector Box -->
  <div class="flex items-center gap-3">
    {#if viewMode === 'class'}
      <ClassSelect
        bind:value={selectedClassId}
        {classes}
        placeholder={m.common_control_select_class()}
        class="w-45 h-8 text-xs bg-background"
      />
    {:else if viewMode === 'teacher'}
      <TeacherSelect
        bind:value={selectedTeacherId}
        {teachers}
        placeholder={m.common_schedule_page_no_teacher()}
        class="w-45 h-8 text-xs bg-background"
      />
    {:else if viewMode === 'subject'}
      <select bind:value={selectedSubjectId} class="h-8 w-45 rounded-md border border-border bg-background px-2 text-xs">
        <option value="">Select subject</option>
        {#each subjects as subject (subject.id)}
          <option value={String(subject.id)}>{subject.name_ar || subject.name}</option>
        {/each}
      </select>
    {:else if viewMode === 'room'}
      <select bind:value={selectedRoomId} class="h-8 w-45 rounded-md border border-border bg-background px-2 text-xs">
        <option value="">Select room</option>
        {#each rooms as room (room.id)}
          <option value={String(room.id)}>{room.name_ar || room.name}</option>
        {/each}
      </select>
    {/if}
  </div>
</div>
