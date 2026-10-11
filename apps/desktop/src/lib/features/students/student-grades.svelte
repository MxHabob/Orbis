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
  import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
  } from "@orbis/ui/select";
  import { OrbisGrid, type OrbisColumnDef } from "@orbis/svelte-grid";
  import { AcademicCap, InfoCircle, Refresh } from "@orbis/icons";
  import { api, extractErrorMessage, type GradeDto } from "$lib/api";
  import { academicContext } from "$lib/stores";
  import { getLocale } from "$lib/paraglide/runtime";
  import * as m from "$lib/paraglide/messages";

  let {
    orgId,
    studentId,
  }: {
    orgId: string;
    studentId: string;
    classId?: string;
  } = $props();

  let selectedPeriodId = $state("all");

  const gradesQuery = createQuery(() => ({
    queryKey: ["assessment-grades", "student", orgId, studentId],
    queryFn: () => api.listGrades({ studentId }),
    enabled: !!orgId && !!studentId,
  }));

  const periodNames = $derived(
    new Map(
      academicContext.allPeriods.map((period) => [
        period.id,
        period.name_ar || period.name,
      ]),
    ),
  );

  const filteredGrades = $derived(
    (gradesQuery.data ?? []).filter(
      (grade) =>
        selectedPeriodId === "all" || grade.period_id === selectedPeriodId,
    ),
  );

  const rows = $derived(
    filteredGrades.map((grade: GradeDto) => ({
      period: periodNames.get(grade.period_id) ?? grade.period_id,
      offering_id: grade.class_subject_id,
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
      updated_at: grade.updated_at,
    })),
  );

  const columns: OrbisColumnDef[] = [
    {
      id: "period",
      prop: "period",
      header: "Period",
      name: "Period",
      width: 170,
      size: 170,
    },
    {
      id: "offering_id",
      prop: "offering_id",
      header: "Course or offering",
      name: "Course or offering",
      width: 230,
      size: 230,
      pin: "left",
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
      width: 95,
      size: 95,
      align: "end",
      type: "number",
    },
    {
      id: "maximum",
      prop: "maximum",
      header: "Maximum",
      name: "Maximum",
      width: 95,
      size: 95,
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
    {
      id: "updated_at",
      prop: "updated_at",
      header: "Last updated",
      name: "Last updated",
      width: 180,
      size: 180,
    },
  ];
</script>

<div class="flex flex-col gap-4 text-xs text-start font-sans print:p-0">
  <Card class="overflow-hidden">
    <CardHeader class="flex flex-row flex-wrap items-center justify-between gap-3 border-b border-border px-4 py-3">
      <div class="flex items-center gap-2">
        <AcademicCap size={16} class="text-primary" />
        <div class="flex flex-col">
          <CardTitle class="text-xs font-bold uppercase tracking-wider">
            Assessment records
          </CardTitle>
          <CardDescription class="text-[10px]">
            {filteredGrades.length} records
          </CardDescription>
        </div>
      </div>

      <CardAction>
        <div class="flex items-center gap-2 print:hidden">
          <Select
            type="single"
            value={selectedPeriodId}
            onValueChange={(value) => {
              if (value) selectedPeriodId = value;
            }}
          >
            <SelectTrigger class="h-8 min-w-[160px] text-xs">
              {selectedPeriodId === "all"
                ? "All periods"
                : periodNames.get(selectedPeriodId) ?? "Select period"}
            </SelectTrigger>
            <SelectContent class="text-xs">
              <SelectItem value="all">All periods</SelectItem>
              {#each academicContext.allPeriods as period (period.id)}
                <SelectItem value={period.id}>
                  {period.name_ar || period.name}
                </SelectItem>
              {/each}
            </SelectContent>
          </Select>
          <Button
            variant="outline"
            size="sm"
            class="h-8 gap-1.5 text-xs"
            onclick={() => gradesQuery.refetch()}
            disabled={gradesQuery.isFetching}
          >
            {#if gradesQuery.isFetching}<LoadingIndicator size={13} />{:else}<Refresh size={13} />{/if}
            {m.common_retry()}
          </Button>
        </div>
      </CardAction>
    </CardHeader>

    <CardContent class="p-0">
      {#if gradesQuery.isLoading}
        <div class="flex min-h-64 flex-col items-center justify-center gap-2">
          <LoadingIndicator size={24} />
          <span class="text-xs text-muted-foreground">{m.common_loading_data()}</span>
        </div>
      {:else if gradesQuery.error}
        <div class="flex min-h-64 flex-col items-center justify-center p-6 text-center text-destructive">
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
        <div class="h-[420px] w-full">
          <OrbisGrid
            {columns}
            source={rows}
            loading={gradesQuery.isFetching}
            clientQuery
            emptyTitle={m.academics_grades_empty_title()}
            emptyDescription={m.academics_grades_empty_subtitle()}
            rtl={getLocale() === "ar"}
            readonly
          />
        </div>
      {/if}
    </CardContent>
  </Card>
</div>
