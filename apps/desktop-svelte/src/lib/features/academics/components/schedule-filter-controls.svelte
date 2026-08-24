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

<div class="flex min-w-0 flex-col gap-2 shrink-0 select-none sm:flex-row sm:items-center">
  <div class="flex min-w-0 items-center overflow-x-auto border-b border-border sm:border-b-0">
    <div class="flex min-w-max">
      {#each modes as mode}
        <Button
          variant="ghost"
          class="h-8 rounded-none border-b-2 px-3 text-[10px] font-semibold transition-all duration-150 {viewMode === mode.value ? 'border-primary text-foreground' : 'border-transparent text-muted-foreground hover:text-foreground'}"
          onclick={() => viewMode = mode.value}
        >{mode.label}</Button>
      {/each}
    </div>
  </div>

  <!-- Selector Box -->
  <div class="flex items-center gap-2">
    {#if viewMode === 'class'}
      <ClassSelect
        bind:value={selectedClassId}
        {classes}
        placeholder={m.common_control_select_class()}
        class="h-8 w-44 bg-card text-[11px]"
      />
    {:else if viewMode === 'teacher'}
      <TeacherSelect
        bind:value={selectedTeacherId}
        {teachers}
        placeholder={m.common_schedule_page_no_teacher()}
        class="h-8 w-44 bg-card text-[11px]"
      />
    {:else if viewMode === 'subject'}
      <select bind:value={selectedSubjectId} class="h-8 w-44 rounded-md border border-border bg-card px-2 text-[11px]">
        <option value="">Select subject</option>
        {#each subjects as subject (subject.id)}
          <option value={String(subject.id)}>{subject.name_ar || subject.name}</option>
        {/each}
      </select>
    {:else if viewMode === 'room'}
      <select bind:value={selectedRoomId} class="h-8 w-44 rounded-md border border-border bg-card px-2 text-[11px]">
        <option value="">Select room</option>
        {#each rooms as room (room.id)}
          <option value={String(room.id)}>{room.name_ar || room.name}</option>
        {/each}
      </select>
    {/if}
  </div>
</div>
