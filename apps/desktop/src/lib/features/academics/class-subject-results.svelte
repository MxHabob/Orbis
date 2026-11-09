<script lang="ts">
import { createQuery } from "@tanstack/svelte-query";
import { LoadingIndicator } from "@orbis/ui";
import { Button } from "@orbis/ui/button";
import { OrbisGrid, type OrbisColumnDef } from "@orbis/svelte-grid";
import { Refresh } from "@orbis/icons";
import { api, extractErrorMessage, type GradeDto } from "$lib/api";
import { academicContext } from "$lib/stores";
import * as m from "$lib/paraglide/messages";
import { getLocale } from "$lib/paraglide/runtime";

let {
  SectionSubjectId,
}: {
  orgId: string;
  classId: string;
  SectionSubjectId: string;
} = $props();

const gradesQuery = createQuery(() => ({
  queryKey: ["assessment-grades", "class-subject", SectionSubjectId],
  queryFn: () => api.listGrades({ SectionSubjectId }),
  enabled: !!SectionSubjectId,
}));

const periodNames = $derived(
  new Map(
    academicContext.allPeriods.map((period) => [
      period.id,
      getLocale() === "ar" ? period.name_ar || period.name : period.name,
    ]),
  ),
);

const rows = $derived(
  (gradesQuery.data ?? []).map((grade: GradeDto) => ({
    student_id: grade.student_id,
    period: periodNames.get(grade.period_id) ?? grade.period_id,
    component_id: grade.component_id ?? "—",
    score: Number(grade.score),
    maximum: Number(grade.max_score),
    weight: grade.weight == null ? "—" : Number(grade.weight),
    letter_grade: grade.letter_grade ?? "—",
    record_state: grade.is_exempt
      ? m.academics_grade_state_exempt()
      : grade.is_excused
        ? m.academics_grade_state_excused()
        : grade.is_final
          ? m.academics_grade_state_final()
          : m.academics_grade_state_recorded(),
  })),
);

const columns = $derived<OrbisColumnDef[]>([
  {
    id: "student_id",
    prop: "student_id",
    header: m.common_grade_column_student(),
    name: m.common_grade_column_student(),
    width: 230,
    size: 230,
    pin: "left",
  },
  {
    id: "period",
    prop: "period",
    header: m.academics_grade_column_period(),
    name: m.academics_grade_column_period(),
    width: 170,
    size: 170,
  },
  {
    id: "component_id",
    prop: "component_id",
    header: m.academics_grade_column_component(),
    name: m.academics_grade_column_component(),
    width: 190,
    size: 190,
  },
  {
    id: "score",
    prop: "score",
    header: m.academics_grade_column_score(),
    name: m.academics_grade_column_score(),
    width: 100,
    size: 100,
    align: "end",
    type: "number",
  },
  {
    id: "maximum",
    prop: "maximum",
    header: m.academics_grade_column_maximum(),
    name: m.academics_grade_column_maximum(),
    width: 100,
    size: 100,
    align: "end",
    type: "number",
  },
  {
    id: "weight",
    prop: "weight",
    header: m.academics_grade_column_weight(),
    name: m.academics_grade_column_weight(),
    width: 90,
    size: 90,
    align: "end",
    type: "number",
  },
  {
    id: "letter_grade",
    prop: "letter_grade",
    header: m.common_grade(),
    name: m.common_grade(),
    width: 90,
    size: 90,
  },
  {
    id: "record_state",
    prop: "record_state",
    header: m.academics_grade_column_record_state(),
    name: m.academics_grade_column_record_state(),
    width: 115,
    size: 115,
  },
]);
</script>

<div class="h-full min-h-0 w-full font-sans">
  {#if gradesQuery.error}
    <div class="flex h-full flex-col items-center justify-center gap-2 p-6 text-center">
      <p class="text-xs font-semibold text-destructive">{m.common_load_error()}</p>
      <p class="text-[10px] text-muted-foreground">
        {extractErrorMessage(gradesQuery.error)}
      </p>
      <Button
        variant="outline"
        size="sm"
        class="mt-2 h-8 gap-1.5 text-xs"
        onclick={() => gradesQuery.refetch()}
      >
        <Refresh size={12} />
        {m.common_retry()}
      </Button>
    </div>
  {:else}
    <OrbisGrid
      {columns}
      source={rows}
      loading={gradesQuery.isLoading}
      rtl={getLocale() === "ar"}
      emptyTitle={m.academics_assessment_records_empty_title()}
      emptyDescription={m.academics_assessment_records_empty_description()}
      readonly
    >
      {#snippet toolbarActions()}
        <Button
          variant="outline"
          size="sm"
          class="h-8 gap-1.5 px-2.5 text-xs"
          onclick={() => gradesQuery.refetch()}
          disabled={gradesQuery.isFetching}
        >
          {#if gradesQuery.isFetching}<LoadingIndicator size={12} />{:else}<Refresh size={12} />{/if}
          <span>{m.common_retry()}</span>
        </Button>
      {/snippet}
    </OrbisGrid>
  {/if}
</div>
