<script lang="ts">
import { Button } from "@orbis/ui-svelte/button";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { Badge } from "@orbis/ui-svelte/badge";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import {
  Users,
  RefreshCw,
  Save,
  Loader2,
  GraduationCap,
  BookOpen,
  Info,
  BarChart3,
} from "lucide-svelte";
import { getLocale } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";
import { OrbisGrid } from "@orbis/svelte-grid";

interface Props {
  selectedClassId: string;
  selectedSubjectId: string;
  classes: any[];
  subjects: any[];
  matrixData: any;
  gridSource: any[];
  gridColumns: any[];
  isMatrixLoading: boolean;
  isSaving: boolean;
  isDirty: boolean;
  canEnterGrades: boolean;
  selectedCycleId: string;
  onRefresh: () => void;
  onSaveAll: () => void;
  onCellEdit: (e: any) => void;
}

let {
  selectedClassId = $bindable(),
  selectedSubjectId = $bindable(),
  classes,
  subjects,
  matrixData,
  gridSource,
  gridColumns,
  isMatrixLoading,
  isSaving,
  isDirty,
  canEnterGrades,
  selectedCycleId,
  onRefresh,
  onSaveAll,
  onCellEdit,
}: Props = $props();
</script>

<div class="bg-card border border-border rounded-xl p-5 flex flex-col gap-4">
  <div class="flex items-center justify-between">
    <h2 class="text-xs font-bold text-foreground flex items-center gap-2">
      <Users size={14} class="text-primary" />
      {m.control_bulk_entry_title()}
    </h2>
    {#if matrixData && canEnterGrades}
      <div class="flex items-center gap-2 flex-nowrap shrink-0">
        {#if isDirty}
          <Badge variant="secondary" class="text-[10px]">●</Badge>
        {/if}
        <Button
          variant="outline"
          size="sm"
          class="text-[10px] h-7 px-2.5 gap-1.5 cursor-pointer"
          onclick={onRefresh}
          disabled={isMatrixLoading}
        >
          <RefreshCw size={11} class={isMatrixLoading ? "animate-spin" : ""} />
          {m.control_refresh()}
        </Button>
        <Button
          variant="lime"
          size="sm"
          class="text-[10px] h-7 px-3 gap-1.5 cursor-pointer"
          onclick={onSaveAll}
          disabled={isSaving || !isDirty}
        >
          {#if isSaving}
            <Loader2 size={11} class="animate-spin" />
          {:else}
            <Save size={11} />
          {/if}
          {m.control_save_all()}
        </Button>
      </div>
    {/if}
  </div>

  <!-- Filters Row -->
  <div class="flex items-center gap-3 flex-wrap">
    <!-- Class Selector -->
    <Select type="single" value={selectedClassId} onValueChange={(v) => { if (v) selectedClassId = v; }}>
      <SelectTrigger class="min-w-[180px] text-xs h-9" disabled={!selectedCycleId}>
        <div class="flex items-center gap-2">
          <GraduationCap size={12} class="text-muted-foreground shrink-0" />
          <span class="truncate">{classes.find((c) => String(c.id) === selectedClassId)?.name_ar || classes.find((c) => String(c.id) === selectedClassId)?.name || m.control_select_class()}</span>
        </div>
      </SelectTrigger>
      <SelectContent>
        {#each classes as cls}
          <SelectItem value={String(cls.id)}>{cls.name_ar || cls.name}</SelectItem>
        {/each}
      </SelectContent>
    </Select>

    <!-- Subject Selector -->
    <Select type="single" value={selectedSubjectId} onValueChange={(v) => { if (v) selectedSubjectId = v; }}>
      <SelectTrigger class="min-w-[180px] text-xs h-9" disabled={!selectedClassId || subjects.length === 0}>
        <div class="flex items-center gap-2">
          <BookOpen size={12} class="text-muted-foreground shrink-0" />
          <span class="truncate">{subjects.find((s) => String(s.id) === selectedSubjectId)?.name_ar || subjects.find((s) => String(s.id) === selectedSubjectId)?.name || m.control_select_subject()}</span>
        </div>
      </SelectTrigger>
      <SelectContent>
        {#each subjects as sub}
          <SelectItem value={String(sub.id)}>{sub.name_ar || sub.name}</SelectItem>
        {/each}
      </SelectContent>
    </Select>
  </div>

  <!-- Grade Matrix Grid -->
  {#if isMatrixLoading}
    <div class="flex flex-col gap-2">
      {#each Array(6) as _}
        <Skeleton class="h-10 w-full rounded-lg" />
      {/each}
    </div>
  {:else if matrixData && gridSource.length > 0}
    <div>
      <OrbisGrid
        columns={gridColumns}
        source={gridSource}
        height="calc(100vh - 420px)"
        rtl={getLocale() === "ar"}
        readonly={!canEnterGrades}
        theme="compact"
        range={true}
        clipboard={true}
        rowSize={42}
        oncelledit={onCellEdit}
      />
    </div>
  {:else if selectedCycleId && selectedClassId && selectedSubjectId && !isMatrixLoading}
    <div class="flex flex-col items-center justify-center py-12 gap-3 text-muted-foreground">
      <Info size={32} class="opacity-40" />
      <p class="text-xs">{m.control_no_data()}</p>
      <p class="text-[10px]">{m.control_no_data_hint()}</p>
    </div>
  {:else}
    <div class="flex flex-col items-center justify-center py-12 gap-3 text-muted-foreground">
      <BarChart3 size={32} class="opacity-40" />
      <p class="text-xs">{m.control_select_filters()}</p>
    </div>
  {/if}
</div>
