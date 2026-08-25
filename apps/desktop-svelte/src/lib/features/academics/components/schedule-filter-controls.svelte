<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui-svelte/button";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
} from "@orbis/ui-svelte/select";
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

<div class="flex min-w-0 flex-col gap-2 shrink-0 select-none sm:flex-row sm:flex-wrap sm:items-center">
  <div class="flex min-w-0 items-center overflow-hidden border-b border-border sm:border-b-0">
    <div class="flex min-w-0 flex-wrap">
      {#each modes as mode}
        <Button
          variant="ghost"
          class="h-8 min-w-0 flex-1 rounded-none border-b-2 px-2 text-[10px] font-semibold transition-all duration-150 sm:px-3 {viewMode === mode.value ? 'border-primary text-foreground' : 'border-transparent text-muted-foreground hover:text-foreground'}"
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
      <Select
        type="single"
        value={selectedSubjectId}
        onValueChange={(value) => selectedSubjectId = value ?? ""}
      >
        <SelectTrigger class="h-8 w-44 bg-card text-[11px]">
          <span class="truncate">{subjects.find((subject) => String(subject.id) === selectedSubjectId)?.name_ar || subjects.find((subject) => String(subject.id) === selectedSubjectId)?.name || "Select subject"}</span>
        </SelectTrigger>
        <SelectContent class="max-h-56 overflow-y-auto text-start">
          {#each subjects as subject (subject.id)}
            <SelectItem value={String(subject.id)} class="text-[11px]">{subject.name_ar || subject.name}</SelectItem>
          {/each}
        </SelectContent>
      </Select>
    {:else if viewMode === 'room'}
      <Select
        type="single"
        value={selectedRoomId}
        onValueChange={(value) => selectedRoomId = value ?? ""}
      >
        <SelectTrigger class="h-8 w-44 bg-card text-[11px]">
          <span class="truncate">{rooms.find((room) => String(room.id) === selectedRoomId)?.name_ar || rooms.find((room) => String(room.id) === selectedRoomId)?.name || "Select room"}</span>
        </SelectTrigger>
        <SelectContent class="max-h-56 overflow-y-auto text-start">
          {#each rooms as room (room.id)}
            <SelectItem value={String(room.id)} class="text-[11px]">{room.name_ar || room.name}</SelectItem>
          {/each}
        </SelectContent>
      </Select>
    {/if}
  </div>
</div>
