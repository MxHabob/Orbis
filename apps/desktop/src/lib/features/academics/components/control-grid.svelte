<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Skeleton } from "@orbis/ui/skeleton";
import { Badge } from "@orbis/ui/badge";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import { AdjustmentsHorizontal, ArrowPath, BookOpen, ChartBar, FloppyDisk, Info, Refresh, Users } from "@orbis/icons";
import { getLocale } from "$lib/paraglide/runtime";
import ClassSelect from "$lib/features/academics/components/class-select.svelte";
import SubjectSelect from "$lib/features/academics/components/subject-select.svelte";

import { OrbisGrid } from "@orbis/svelte-grid";

interface Props {
  selectedClassId: string;
  selectedSubjectId: string;
  matrixData: any;
  gridSource: any[];
  gridColumns: any[];
  isMatrixLoading: boolean;
  isSaving: boolean;
  isDirty: boolean;
  canEnterGrades: boolean;
  selectedPeriodId: string;
  onRefresh: () => void;
  onSaveAll: () => void;
  onCellEdit: (e: any) => void;
}

let {
  selectedClassId = $bindable(),
  selectedSubjectId = $bindable(),
  matrixData,
  gridSource,
  gridColumns,
  isMatrixLoading,
  isSaving,
  isDirty,
  canEnterGrades,
  selectedPeriodId,
  onRefresh,
  onSaveAll,
  onCellEdit,
}: Props = $props();
</script>

<div class="bg-card border border-border rounded-xl p-5 flex flex-col gap-4">
  <div class="flex items-center justify-between">
    <h2 class="text-xs font-bold text-foreground flex items-center gap-2">
      <Users size={14} class="text-primary" />
      {m.common_bulk_grades_title()}
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
          {#if isMatrixLoading}<LoadingIndicator  size={11} />{:else}<Refresh size={11}   />{/if}
          {m.common_control_refresh()}
        </Button>
        <Button
          variant="lime"
          size="sm"
          class="text-[10px] h-7 px-3 gap-1.5 cursor-pointer"
          onclick={onSaveAll}
          disabled={isSaving || !isDirty}
        >
          {#if isSaving}
            <LoadingIndicator   size={11} />
          {:else}
            <FloppyDisk size={11} />
          {/if}
          {m.common_action_save()}
        </Button>
      </div>
    {/if}
  </div>

  <!-- Filters Row -->
  <div class="flex items-center gap-3 flex-wrap">
    <!-- Class Selector -->
    <ClassSelect
      bind:value={selectedClassId}
      disabled={!selectedPeriodId}
      placeholder={m.common_control_select_class()}
      class="min-w-[180px] text-xs h-9"
    />

    <!-- Subject Selector -->
    <SubjectSelect
      bind:value={selectedSubjectId}
      disabled={!selectedClassId}
      placeholder={m.common_control_select_subject()}
      class="min-w-[180px] text-xs h-9"
    />
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
        oncelledit={onCellEdit}
      />
    </div>
  {:else if selectedPeriodId && selectedClassId && selectedSubjectId && !isMatrixLoading}
    <div class="flex flex-col items-center justify-center py-12 gap-3 text-muted-foreground">
      <Info size={32} class="opacity-40" />
      <p class="text-xs">{m.common_control_no_data()}</p>
      <p class="text-[10px]">{m.common_control_no_data_hint()}</p>
    </div>
  {:else}
    <div class="flex flex-col items-center justify-center py-12 gap-3 text-muted-foreground">
      <ChartBar size={32} class="opacity-40" />
      <p class="text-xs">{m.common_control_select_filters()}</p>
    </div>
  {/if}
</div>
