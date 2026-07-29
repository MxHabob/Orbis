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

<div class="flex flex-col md:flex-row md:items-center md:justify-between gap-4 bg-card/25 p-4 rounded-xl border border-border/80 shrink-0 select-none">
  <div class="flex items-center gap-3">
    <span class="text-xs font-bold text-muted-foreground uppercase tracking-wider">
      {m.common_schedule_page_view_by()}
    </span>
    <div class="flex p-0.5 rounded-lg bg-muted border border-border/60">
      <Button
        variant="ghost"
        class="text-[10px] font-bold px-3 py-1.5 h-8 rounded-md transition-all duration-150 cursor-pointer {viewMode === 'class' ? 'bg-card text-foreground shadow-sm' : 'text-muted-foreground hover:bg-transparent'}"
        onclick={() => viewMode = 'class'}
      >
        {m.common_word_semester()}
      </Button>
      <Button
        variant="ghost"
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
      <span class="text-xs text-muted-foreground">{m.common_schedule_page_sel_class()}</span>
      <ClassSelect
        bind:value={selectedClassId}
        {classes}
        placeholder={m.common_control_select_class()}
        class="w-[180px] h-8 text-xs bg-background"
      />
    {:else}
      <span class="text-xs text-muted-foreground">{m.common_schedule_page_sel_teacher()}</span>
      <TeacherSelect
        bind:value={selectedTeacherId}
        {teachers}
        placeholder={m.common_schedule_page_no_teacher()}
        class="w-[180px] h-8 text-xs bg-background"
      />
    {/if}
  </div>
</div>
