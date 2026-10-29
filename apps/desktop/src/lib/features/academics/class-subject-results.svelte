<script lang="ts">
import { createQuery } from "@tanstack/svelte-query";
import { LoadingIndicator } from "@orbis/ui";
import { Button } from "@orbis/ui/button";
import {
  Card,
  CardAction,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@orbis/ui/card";
import { OrbisGrid, type OrbisColumnDef } from "@orbis/svelte-grid";
import { DocumentCheck, InfoCircle, Refresh } from "@orbis/icons";
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
      period.name_ar || period.name,
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
      ? "Exempt"
      : grade.is_excused
        ? "Excused"
        : grade.is_final
          ? "Final"
          : "Recorded",
  })),
);

const columns: OrbisColumnDef[] = [
  {
    id: "student_id",
    prop: "student_id",
    header: "Learner",
    name: "Learner",
    width: 230,
    size: 230,
    pin: "left",
  },
  {
    id: "period",
    prop: "period",
    header: "Period",
    name: "Period",
    width: 170,
    size: 170,
  },
  {
    id: "component_id",
    prop: "component_id",
    header: "Assessment component",
    name: "Assessment component",
    width: 190,
    size: 190,
  },
  {
    id: "score",
    prop: "score",
    header: "Score",
    name: "Score",
    width: 100,
    size: 100,
    align: "end",
    type: "number",
  },
  {
    id: "maximum",
    prop: "maximum",
    header: "Maximum",
    name: "Maximum",
    width: 100,
    size: 100,
    align: "end",
    type: "number",
  },
  {
    id: "weight",
    prop: "weight",
    header: "Weight",
    name: "Weight",
    width: 90,
    size: 90,
    align: "end",
    type: "number",
  },
  {
    id: "letter_grade",
    prop: "letter_grade",
    header: "Grade",
    name: "Grade",
    width: 90,
    size: 90,
  },
  {
    id: "record_state",
    prop: "record_state",
    header: "Record state",
    name: "Record state",
    width: 115,
    size: 115,
  },
];
</script>

<div class="flex h-full w-full flex-col gap-2.5 font-sans">
  <Card class="flex min-h-0 flex-1 flex-col overflow-hidden">
    <CardHeader class="flex flex-row items-center justify-between border-b border-border px-4 py-3">
      <div class="flex items-center gap-2.5">
        <div class="flex h-8 w-8 shrink-0 items-center justify-center rounded-lg bg-primary/10 text-primary">
          <DocumentCheck class="h-4 w-4" />
        </div>
        <div class="flex flex-col text-start">
          <CardTitle class="text-xs font-bold uppercase tracking-wider text-foreground">
            Assessment records
          </CardTitle>
          <CardDescription class="text-[10px]">
            {rows.length} records
          </CardDescription>
        </div>
      </div>

      <CardAction>
        <Button
          variant="outline"
          size="sm"
          class="h-8 cursor-pointer gap-1.5 px-2.5 text-xs font-semibold shadow-2xs"
          onclick={() => gradesQuery.refetch()}
          disabled={gradesQuery.isFetching}
        >
          {#if gradesQuery.isFetching}<LoadingIndicator size={12} />{:else}<Refresh size={12} />{/if}
          <span>{m.common_retry()}</span>
        </Button>
      </CardAction>
    </CardHeader>

    <CardContent class="relative min-h-0 flex-1 p-0">
      {#if gradesQuery.isLoading}
        <div class="absolute inset-0 z-10 flex flex-col items-center justify-center gap-2 bg-background/50 backdrop-blur-2xs">
          <LoadingIndicator size={24} />
          <span class="text-xs text-muted-foreground">{m.common_loading_data()}</span>
        </div>
      {:else if gradesQuery.error}
        <div class="absolute inset-0 flex flex-col items-center justify-center p-6 text-center text-destructive">
          <InfoCircle class="mb-2 h-8 w-8 stroke-current opacity-80" />
          <p class="text-xs font-semibold">{m.common_load_error()}</p>
          <p class="mt-1 text-[10px] text-muted-foreground">
            {extractErrorMessage(gradesQuery.error)}
          </p>
          <Button
            variant="outline"
            size="sm"
            class="mt-4"
            onclick={() => gradesQuery.refetch()}
          >
            {m.common_retry()}
          </Button>
        </div>
      {:else}
        <OrbisGrid
          {columns}
          source={rows}
          rtl={getLocale() === "ar"}
          emptyTitle="No assessment records yet"
          emptyDescription="Assessment records for this offering will appear here."
          readonly
        />
      {/if}
    </CardContent>
  </Card>
</div>
