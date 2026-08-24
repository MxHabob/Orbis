<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui-svelte/button";
import ClassSelect from "$lib/features/academics/components/class-select.svelte";
import TeacherSelect from "$lib/features/academics/components/teacher-select.svelte";

interface Props {
  viewMode: "class" | "teacher";
  selectedClassId: string;
  selectedTeacherId: string;
  classes: any[];
  teachers: any[];
}

let {
  viewMode = $bindable(),
  selectedClassId = $bindable(),
  selectedTeacherId = $bindable(),
  classes,
  teachers,
}: Props = $props();
</script>

<div class="flex flex-col md:flex-row md:items-center md:justify-between gap-4 shrink-0 select-none">
  <div class="flex items-center gap-3">
    <div class="flex p-0.5">
      <Button
        variant="secondary"
        class="text-[10px] font-bold px-3 py-1.5 h-8 rounded-md transition-all duration-150 cursor-pointer {viewMode === 'class' ? 'bg-card text-foreground shadow-sm' : 'text-muted-foreground hover:bg-transparent'}"
        onclick={() => viewMode = 'class'}
      >
        {m.common_word_semester()}
      </Button>
      <Button
        variant="secondary"
        class="text-[10px] font-bold px-3 py-1.5 h-8 rounded-md transition-all duration-150 cursor-pointer {viewMode === 'teacher' ? 'bg-card text-foreground shadow-sm' : 'text-muted-foreground hover:bg-transparent'}"
        onclick={() => viewMode = 'teacher'}
      >
        {m.common_schedule_page_teacher()}
      </Button>
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
    {:else}
      <TeacherSelect
        bind:value={selectedTeacherId}
        {teachers}
        placeholder={m.common_schedule_page_no_teacher()}
        class="w-45 h-8 text-xs bg-background"
      />
    {/if}
  </div>
</div>
