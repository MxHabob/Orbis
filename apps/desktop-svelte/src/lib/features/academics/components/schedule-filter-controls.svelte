<script lang="ts">
import { Button } from "@orbis/ui-svelte/button";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import { getLocale } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";

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
      {m.schedule_page_view_by()}
    </span>
    <div class="flex p-0.5 rounded-lg bg-muted border border-border/60">
      <Button
        variant="ghost"
        class="text-[10px] font-bold px-3 py-1.5 h-8 rounded-md transition-all duration-150 cursor-pointer {viewMode === 'class' ? 'bg-card text-foreground shadow-sm' : 'text-muted-foreground hover:bg-transparent'}"
        onclick={() => viewMode = 'class'}
      >
        {m.schedule_page_class()}
      </Button>
      <Button
        variant="ghost"
        class="text-[10px] font-bold px-3 py-1.5 h-8 rounded-md transition-all duration-150 cursor-pointer {viewMode === 'teacher' ? 'bg-card text-foreground shadow-sm' : 'text-muted-foreground hover:bg-transparent'}"
        onclick={() => viewMode = 'teacher'}
      >
        {m.schedule_page_teacher()}
      </Button>
    </div>
  </div>

  <!-- Selector Box using Orbis Select to follow agent rules -->
  <div class="flex items-center gap-3">
    {#if viewMode === 'class'}
      <span class="text-xs text-muted-foreground">{m.schedule_page_sel_class()}</span>
      <Select type="single" value={selectedClassId} onValueChange={(v) => { if (v) selectedClassId = v; }}>
        <SelectTrigger class="w-[180px] h-8 text-xs bg-background">
          {classes.find(c => String(c.id) === selectedClassId)?.name_ar || classes.find(c => String(c.id) === selectedClassId)?.name || m.control_select_class()}
        </SelectTrigger>
        <SelectContent>
          {#each classes as cls (cls.id)}
            <SelectItem value={String(cls.id)}>{cls.name_ar || cls.name}</SelectItem>
          {/each}
        </SelectContent>
      </Select>
    {:else}
      <span class="text-xs text-muted-foreground">{m.schedule_page_sel_teacher()}</span>
      <Select type="single" value={selectedTeacherId} onValueChange={(v) => { if (v) selectedTeacherId = v; }}>
        <SelectTrigger class="w-[180px] h-8 text-xs bg-background">
          {(() => {
            const teach = teachers.find(t => String(t.id) === selectedTeacherId);
            return teach ? (getLocale() === "ar" ? (teach.name_ar || teach.full_name) : teach.full_name) : m.schedule_page_no_teacher();
          })()}
        </SelectTrigger>
        <SelectContent>
          {#each teachers as teach (teach.id)}
            <SelectItem value={String(teach.id)}>
              {getLocale() === "ar" ? (teach.name_ar || teach.full_name) : teach.full_name}
            </SelectItem>
          {/each}
        </SelectContent>
      </Select>
    {/if}
  </div>
</div>
