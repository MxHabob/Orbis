<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Skeleton } from "@orbis/ui/skeleton";
import { Card } from "@orbis/ui/card";
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
  onCalculateRankings?: () => void;
  isCalculating?: boolean;
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
  onCalculateRankings,
  isCalculating = false,
}: Props = $props();
</script>

<Card class="flex min-h-0 flex-1 flex-col gap-3 overflow-hidden rounded-xl border-border/60 bg-card p-3 shadow-xs sm:p-4">
  <div class="flex shrink-0 flex-col gap-2 xl:flex-row xl:items-center">
    <!-- Class Selector -->
    <ClassSelect
      bind:value={selectedClassId}
      disabled={!selectedPeriodId}
      placeholder={m.common_control_select_class()}
      class="h-9 w-full min-w-0 text-xs xl:min-w-[180px] xl:flex-1"
    />

    <!-- Subject Selector -->
    <SubjectSelect
      bind:value={selectedSubjectId}
      disabled={!selectedClassId}
      placeholder={m.common_control_select_subject()}
      class="h-9 w-full min-w-0 text-xs xl:min-w-[180px] xl:flex-1"
    />

    {#if matrixData && canEnterGrades}
      <div class="flex shrink-0 flex-wrap items-center gap-2 xl:ms-auto xl:flex-nowrap">
        {#if onCalculateRankings}
          <Button
            variant="outline"
            size="sm"
            class="h-8 px-3 text-xs text-primary"
            onclick={onCalculateRankings}
            disabled={isCalculating}
          >
            {#if isCalculating}<LoadingIndicator size={12} />{/if}
            <span>{m.common_control_calculate_ranking()}</span>
          </Button>
        {/if}
        <Button
          variant="outline"
          size="sm"
          class="h-8 px-3 text-xs"
          onclick={onRefresh}
          disabled={isMatrixLoading}
        >
          {#if isMatrixLoading}<LoadingIndicator size={12} />{/if}
          {m.common_control_refresh()}
        </Button>
        <Button
          variant="lime"
          size="sm"
          class="h-8 px-3 text-xs"
          onclick={onSaveAll}
          disabled={isSaving || !isDirty}
        >
          {#if isSaving}
            <LoadingIndicator size={12} />
          {/if}
          {m.common_action_save()}
        </Button>
      </div>
    {/if}
  </div>

  {#if isMatrixLoading}
    <div class="flex min-h-0 flex-1 flex-col gap-2 overflow-hidden">
      {#each Array(6) as _, index (index)}
        <Skeleton class="min-h-8 w-full flex-1 rounded-lg" />
      {/each}
    </div>
  {:else if matrixData && gridSource.length > 0}
    <div class="min-h-0 flex-1 overflow-hidden">
      <OrbisGrid
        columns={gridColumns}
        source={gridSource}
        height="100%"
        rtl={getLocale() === "ar"}
        readonly={!canEnterGrades}
        oncelledit={onCellEdit}
      />
    </div>
  {:else if selectedPeriodId && selectedClassId && selectedSubjectId && !isMatrixLoading}
    <div class="flex min-h-0 flex-1 flex-col items-center justify-center gap-3 text-muted-foreground">
      <p class="text-sm">{m.common_control_no_data()}</p>
      <p class="text-xs">{m.common_control_no_data_hint()}</p>
    </div>
  {:else}
    <div class="flex min-h-0 flex-1 flex-col items-center justify-center gap-3 text-muted-foreground">
      <p class="text-sm">{m.common_control_select_filters()}</p>
    </div>
  {/if}
</Card>
