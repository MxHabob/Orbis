<!--
@component PromotionWizard
Reviews server-evaluated cohort progression and submits one confirmed promotion request.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import {
  api,
  extractErrorMessage,
  type AcademicPeriodDto,
  type SectionDto,
  type StudentProgressionItemInput,
  type PromoteCohortInput,
  type progressionJobDto,
} from "$lib/api";
import { toast, FormField, LoadingIndicator } from "@orbis/ui";
import { Card, CardContent, CardHeader, CardTitle } from "@orbis/ui/card";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
} from "@orbis/ui/select";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from "@orbis/ui/alert-dialog";
import {
  Check,
  ChevronLeft,
  ChevronRight,
  Play,
  Refresh,
  Users,
} from "@orbis/icons";
import { z } from "zod";
import { FormController } from "$lib/shared/forms/form-controller.svelte";
import { createPeriodsQuery, createSectionsQuery } from "./queries";
import { l } from "$lib/core/i18n";

let { orgId }: { orgId: string } = $props();

const PAGE_SIZE = 40;
const form = new FormController({
  initialValues: {
    source_period_id: "",
    target_period_id: "",
    job_name: "",
  },
  schema: z
    .object({
      source_period_id: z
        .string()
        .min(1, m.common_form_error_required_fields()),
      target_period_id: z
        .string()
        .min(1, m.common_form_error_required_fields()),
      job_name: z.string().trim().min(1, m.common_form_error_required_fields()),
    })
    .refine((values) => values.source_period_id !== values.target_period_id, {
      path: ["target_period_id"],
      message: m.common_form_error_required_fields(),
    }),
});

const periodsQuery = createPeriodsQuery(() => orgId);
const sectionsQuery = createSectionsQuery(() => orgId);
const periods = $derived(periodsQuery.data ?? []);
const sections = $derived(
  (sectionsQuery.data ?? []).filter((section) => section.is_active),
);
const targetPeriods = $derived(
  periods.filter(
    (period) => period.id !== form.values.source_period_id && !period.is_locked,
  ),
);
const targetSections = $derived(
  sections.filter(
    (section) =>
      section.period_id === form.values.target_period_id || !section.period_id,
  ),
);

let step = $state<"setup" | "review" | "complete">("setup");
let cohort = $state<StudentProgressionItemInput[]>([]);
let sourceSectionTargets = $state<Record<string, string>>({});
let page = $state(0);
let isEvaluating = $state(false);
let showExecuteConfirm = $state(false);
let result = $state<progressionJobDto | null>(null);

const sourceSectionIds = $derived([
  ...new Set(
    cohort
      .map((item) => item.source_section_id)
      .filter((sectionId): sectionId is string => !!sectionId),
  ),
]);
const promotedCount = $derived(
  cohort.filter((item) => item.decision === "promote").length,
);
const retainedCount = $derived(
  cohort.filter((item) => item.decision === "retain").length,
);
const graduatedCount = $derived(
  cohort.filter((item) => item.decision === "graduate").length,
);
const excludedCount = $derived(
  cohort.filter((item) => item.decision === "exclude").length,
);
const missingTargetCount = $derived(
  cohort.filter(
    (item) =>
      item.decision === "promote" &&
      !!item.source_section_id &&
      !item.target_section_id,
  ).length,
);
const pageCount = $derived(Math.max(1, Math.ceil(cohort.length / PAGE_SIZE)));
const pageItems = $derived(
  cohort.slice(page * PAGE_SIZE, (page + 1) * PAGE_SIZE),
);

function periodLabel(period: AcademicPeriodDto): string {
  return l(period.name_ar, period.name);
}

function sectionLabel(sectionId: string): string {
  const section = sections.find((item) => item.id === sectionId);
  return section ? l(section.name_ar, section.name) || section.code : sectionId;
}

function setSourcePeriod(value: string | undefined) {
  form.setField("source_period_id", value ?? "");
  form.markTouched("source_period_id");
  cohort = [];
  sourceSectionTargets = {};
  if (form.values.target_period_id === value) {
    form.setField("target_period_id", "");
  }
}

function setTargetPeriod(value: string | undefined) {
  form.setField("target_period_id", value ?? "");
  form.markTouched("target_period_id");
  cohort = [];
  sourceSectionTargets = {};
  const targetName = periods.find((period) => period.id === value);
  if (!form.values.job_name.trim() && targetName) {
    form.setField(
      "job_name",
      `${periodLabel(targetName)} — ${m.common_promo_execute()}`,
    );
  }
}

async function evaluateCohort() {
  form.formError = undefined;
  if (!form.validate()) return;

  isEvaluating = true;
  try {
    const evaluated = await api.evaluateCohort(form.values.source_period_id);
    cohort = evaluated.map((item) => ({ ...item }));
    sourceSectionTargets = {};
    page = 0;
    step = "review";
  } catch (error: unknown) {
    toast.error(extractErrorMessage(error));
  } finally {
    isEvaluating = false;
  }
}

function setSectionTarget(sourceSectionId: string, targetSectionId: string) {
  sourceSectionTargets = {
    ...sourceSectionTargets,
    [sourceSectionId]: targetSectionId,
  };
  cohort = cohort.map((item) =>
    item.source_section_id === sourceSectionId
      ? { ...item, target_section_id: targetSectionId || null }
      : item,
  );
}

function setDecision(index: number, decision: string) {
  const itemIndex = page * PAGE_SIZE + index;
  const item = cohort[itemIndex];
  if (!item) return;
  cohort[itemIndex] = {
    ...item,
    decision,
    target_section_id:
      decision === "promote" && item.source_section_id
        ? sourceSectionTargets[item.source_section_id] || null
        : item.target_section_id,
  };
}

async function executePromotion() {
  showExecuteConfirm = false;
  if (missingTargetCount > 0) {
    toast.error(m.common_form_error_required_fields());
    return;
  }

  const submitted = await form.submit(async (values) => {
    const request: PromoteCohortInput = {
      source_period_id: values.source_period_id,
      target_period_id: values.target_period_id,
      job_name: values.job_name,
      items: cohort.map((item) => ({
        student_id: item.student_id,
        enrollment_id: item.enrollment_id,
        source_section_id: item.source_section_id,
        target_section_id: item.target_section_id,
        decision: item.decision,
        reason: item.reason,
      })),
    };
    result = await api.promoteCohort(request);
    step = "complete";
    toast.success(m.common_promo_complete());
  });

  if (!submitted && form.formError) toast.error(form.formError);
  else if (!submitted) {
    toast.error(
      Object.values(form.errors).flat()[0] ||
        m.common_form_error_required_fields(),
    );
  }
}

function changeItemDecision(index: number, value: string | undefined) {
  setDecision(index, value ?? "promote");
}
</script>

<div class="min-h-full w-full overflow-y-auto bg-background">
  <div class="mx-auto flex w-full max-w-7xl flex-col gap-6 px-4 py-5 sm:px-6 lg:px-8">
    <header class="flex flex-wrap items-start justify-between gap-3 border-b border-border/60 pb-4">
      <div class="space-y-1 text-start">
        <h1 class="text-xl font-semibold text-foreground">{m.common_promo_execute()}</h1>
        <p class="text-sm text-muted-foreground">{m.common_promo_desc_history_subtitle()}</p>
      </div>
      {#if step !== "complete"}
        <div class="flex items-center gap-2 text-xs text-muted-foreground">
          <span class:font-semibold={step === "setup"} class:text-foreground={step === "setup"}>1</span>
          <span aria-hidden="true">/</span>
          <span class:font-semibold={step === "review"} class:text-foreground={step === "review"}>2</span>
        </div>
      {/if}
    </header>

    {#if step === "setup"}
      <Card>
        <CardHeader class="border-b border-border/60 px-4 py-4 sm:px-6">
          <CardTitle class="text-base">{m.common_word_settings()}</CardTitle>
        </CardHeader>
        <CardContent class="grid gap-5 p-4 sm:grid-cols-2 sm:p-6">
          <FormField
            id="promotion-source-period"
            label={m.student_registration_active_period()}
            error={form.errors.source_period_id?.[0]}
            required
          >
            {#snippet children()}
              <Select
                type="single"
                value={form.values.source_period_id}
                onValueChange={setSourcePeriod}
              >
                <SelectTrigger id="promotion-source-period" class="h-10 w-full">
                  {periods.find((period) => period.id === form.values.source_period_id)
                    ? periodLabel(periods.find((period) => period.id === form.values.source_period_id)!)
                    : m.common_metrics_period()}
                </SelectTrigger>
                <SelectContent>
                  {#each periods as period (period.id)}
                    <SelectItem value={period.id}>{periodLabel(period)}</SelectItem>
                  {/each}
                </SelectContent>
              </Select>
            {/snippet}
          </FormField>

          <FormField
            id="promotion-target-period"
            label={m.common_metrics_period()}
            error={form.errors.target_period_id?.[0]}
            required
          >
            {#snippet children()}
              <Select
                type="single"
                value={form.values.target_period_id}
                onValueChange={setTargetPeriod}
              >
                <SelectTrigger id="promotion-target-period" class="h-10 w-full">
                  {targetPeriods.find((period) => period.id === form.values.target_period_id)
                    ? periodLabel(targetPeriods.find((period) => period.id === form.values.target_period_id)!)
                    : m.common_metrics_period()}
                </SelectTrigger>
                <SelectContent>
                  {#each targetPeriods as period (period.id)}
                    <SelectItem value={period.id}>{periodLabel(period)}</SelectItem>
                  {/each}
                </SelectContent>
              </Select>
            {/snippet}
          </FormField>

          <div class="sm:col-span-2">
            <FormField
              id="promotion-job-name"
              label={m.common_cycle_form_name()}
              error={form.errors.job_name?.[0]}
              required
            >
              {#snippet children()}
                <Input
                  id="promotion-job-name"
                  value={form.values.job_name}
                  oninput={(event) => form.setField("job_name", event.currentTarget.value)}
                  onblur={() => form.markTouched("job_name")}
                  maxlength={120}
                  class="h-10"
                />
              {/snippet}
            </FormField>
          </div>
          {#if periodsQuery.isLoading || sectionsQuery.isLoading}
            <div class="sm:col-span-2 flex items-center gap-2 text-sm text-muted-foreground" role="status">
              <LoadingIndicator size={16} />
              {m.common_loading()}
            </div>
          {:else if periodsQuery.isError || sectionsQuery.isError}
            <div class="sm:col-span-2 rounded-md border border-destructive/30 bg-destructive/5 p-3 text-sm text-destructive" role="alert">
              {extractErrorMessage(periodsQuery.error || sectionsQuery.error)}
            </div>
          {/if}
        </CardContent>
      </Card>

      <div class="flex justify-end">
        <Button
          variant="lime"
          disabled={isEvaluating || periodsQuery.isLoading || sectionsQuery.isLoading || periodsQuery.isError || sectionsQuery.isError}
          onclick={evaluateCohort}
        >
          {#if isEvaluating}
            <LoadingIndicator class="me-2" size={16} />
            {m.common_promo_running()}
          {:else}
            <Play class="me-2 size-4" />
            {m.common_action_preview()}
          {/if}
        </Button>
      </div>
    {:else if step === "review"}
      <section class="flex flex-col gap-5">
        <div class="grid gap-3 sm:grid-cols-2 lg:grid-cols-4">
          {#each [
            { label: m.common_promo_total(), value: cohort.length },
            { label: m.common_promo_promoted(), value: promotedCount },
            { label: m.common_promo_retained(), value: retainedCount },
            { label: m.common_form_status_graduated(), value: graduatedCount },
          ] as stat (stat.label)}
            <div class="rounded-lg border border-border/70 bg-card px-4 py-3">
              <p class="text-xs text-muted-foreground">{stat.label}</p>
              <p class="mt-1 text-xl font-semibold tabular-nums">{stat.value}</p>
            </div>
          {/each}
        </div>

        {#if cohort.length === 0}
          <div class="rounded-lg border border-dashed border-border px-6 py-12 text-center">
            <Users class="mx-auto mb-3 size-8 text-muted-foreground" />
            <p class="font-medium">{m.common_promo_no_jobs_desc()}</p>
          </div>
        {:else}
          {#if sourceSectionIds.length > 0}
            <Card>
              <CardHeader class="border-b border-border/60 px-4 py-4 sm:px-6">
                <CardTitle class="text-base">{m.common_promo_mapping()}</CardTitle>
              </CardHeader>
              <CardContent class="grid gap-4 p-4 sm:grid-cols-2 sm:p-6 xl:grid-cols-3">
                {#each sourceSectionIds as sourceSectionId (sourceSectionId)}
                  <FormField
                    id={`promotion-section-${sourceSectionId}`}
                    label={sectionLabel(sourceSectionId)}
                    required
                  >
                    {#snippet children()}
                      <Select
                        type="single"
                        value={sourceSectionTargets[sourceSectionId] ?? ""}
                        onValueChange={(value) => setSectionTarget(sourceSectionId, value ?? "")}
                      >
                        <SelectTrigger id={`promotion-section-${sourceSectionId}`} class="h-10 w-full">
                          {targetSections.find((section) => section.id === sourceSectionTargets[sourceSectionId])
                            ? sectionLabel(sourceSectionTargets[sourceSectionId])
                            : m.common_form_select_class()}
                        </SelectTrigger>
                        <SelectContent>
                          {#each targetSections as section (section.id)}
                            <SelectItem value={section.id}>
                              {l(section.name_ar, section.name) || section.code}
                            </SelectItem>
                          {/each}
                        </SelectContent>
                      </Select>
                    {/snippet}
                  </FormField>
                {/each}
              </CardContent>
            </Card>
          {/if}

          {#if missingTargetCount > 0}
            <div class="rounded-md border border-amber-500/30 bg-amber-500/5 px-4 py-3 text-sm text-amber-800 dark:text-amber-200" role="status">
              {m.common_form_error_required_fields()}
            </div>
          {/if}

          <div class="overflow-hidden rounded-lg border border-border/70">
            <div class="flex flex-wrap items-center justify-between gap-3 border-b border-border/60 px-4 py-3">
              <div>
                <h2 class="text-sm font-semibold">{m.common_word_student()} · {m.common_promo_decision()}</h2>
                <p class="text-xs text-muted-foreground">{m.common_promo_total()}: {cohort.length}</p>
              </div>
              <div class="flex items-center gap-2">
                <Button
                  variant="outline"
                  size="sm"
                  disabled={page === 0}
                  onclick={() => page -= 1}
                  aria-label={m.common_onboarding_prev_btn()}
                >
                  <ChevronLeft class="size-4 rtl:rotate-180" />
                </Button>
                <span class="min-w-16 text-center text-xs tabular-nums text-muted-foreground">
                  {page + 1} / {pageCount}
                </span>
                <Button
                  variant="outline"
                  size="sm"
                  disabled={page + 1 >= pageCount}
                  onclick={() => page += 1}
                  aria-label={m.common_action_preview()}
                >
                  <ChevronRight class="size-4 rtl:rotate-180" />
                </Button>
              </div>
            </div>

            <div class="divide-y divide-border/60">
              {#each pageItems as item, index (item.enrollment_id)}
                <div class="grid gap-3 px-4 py-4 md:grid-cols-[minmax(0,1fr)_minmax(10rem,0.7fr)_minmax(12rem,0.8fr)] md:items-center">
                  <div class="min-w-0 text-start">
                    <p class="truncate text-sm font-medium">
                      {item.student_number || item.student_id}
                    </p>
                    <p class="mt-0.5 text-xs text-muted-foreground">
                      {item.source_section_id ? sectionLabel(item.source_section_id) : "—"}
                    </p>
                    {#if item.reason}
                      <p class="mt-1 text-xs text-muted-foreground">{item.reason}</p>
                    {/if}
                  </div>

                  <Select
                    type="single"
                    value={item.decision}
                    onValueChange={(value) => changeItemDecision(index, value)}
                  >
                    <SelectTrigger class="h-9 w-full" aria-label={m.common_promo_decision()}>
                      {item.decision === "promote"
                        ? m.common_promo_promoted()
                        : item.decision === "retain"
                          ? m.common_promo_retained()
                          : item.decision === "graduate"
                            ? m.common_form_status_graduated()
                            : m.common_promo_excluded()}
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="promote">{m.common_promo_promoted()}</SelectItem>
                      <SelectItem value="retain">{m.common_promo_retained()}</SelectItem>
                      <SelectItem value="graduate">{m.common_form_status_graduated()}</SelectItem>
                      <SelectItem value="exclude">{m.common_promo_excluded()}</SelectItem>
                    </SelectContent>
                  </Select>

                  <p class="text-xs text-muted-foreground md:text-end">
                    {item.target_section_id ? sectionLabel(item.target_section_id) : "—"}
                  </p>
                </div>
              {/each}
            </div>
          </div>
        {/if}

        <div class="flex flex-wrap items-center justify-between gap-3 border-t border-border/60 pt-4">
          <Button variant="outline" onclick={() => (step = "setup")}>
            <ChevronLeft class="me-2 size-4 rtl:rotate-180" />
            {m.common_onboarding_prev_btn()}
          </Button>
          <Button
            variant="lime"
            disabled={cohort.length === 0 || missingTargetCount > 0 || form.isSubmitting}
            onclick={() => (showExecuteConfirm = true)}
          >
            {#if form.isSubmitting}
              <LoadingIndicator class="me-2" size={16} />
            {:else}
              <Check class="me-2 size-4" />
            {/if}
            {m.common_promo_execute()}
          </Button>
        </div>
      </section>
    {:else if result}
      <Card>
        <CardContent class="flex flex-col items-center gap-5 px-4 py-10 text-center sm:px-8">
          <div class="flex size-12 items-center justify-center rounded-full bg-primary/10 text-primary">
            <Check class="size-6" />
          </div>
          <div>
            <h2 class="text-lg font-semibold">{m.common_promo_complete()}</h2>
            <p class="mt-1 text-sm text-muted-foreground">{result.name}</p>
          </div>
          <div class="grid w-full max-w-3xl grid-cols-2 gap-3 sm:grid-cols-5">
            <div class="rounded-md border border-border p-3"><strong class="block text-xl tabular-nums">{result.total_students}</strong><span class="text-xs text-muted-foreground">{m.common_promo_total()}</span></div>
            <div class="rounded-md border border-border p-3"><strong class="block text-xl tabular-nums">{result.promoted_count}</strong><span class="text-xs text-muted-foreground">{m.common_promo_promoted()}</span></div>
            <div class="rounded-md border border-border p-3"><strong class="block text-xl tabular-nums">{result.retained_count}</strong><span class="text-xs text-muted-foreground">{m.common_promo_retained()}</span></div>
            <div class="rounded-md border border-border p-3"><strong class="block text-xl tabular-nums">{result.graduated_count}</strong><span class="text-xs text-muted-foreground">{m.common_form_status_graduated()}</span></div>
            <div class="rounded-md border border-border p-3"><strong class="block text-xl tabular-nums">{result.excluded_count}</strong><span class="text-xs text-muted-foreground">{m.common_promo_excluded()}</span></div>
          </div>
        </CardContent>
      </Card>
      <div class="flex justify-end">
        <Button variant="outline" onclick={() => {
          cohort = [];
          result = null;
          step = "setup";
          form.reset({
            source_period_id: form.values.target_period_id,
            target_period_id: "",
            job_name: "",
          });
        }}>
          <Refresh class="me-2 size-4" />
          {m.common_promo_execute()}
        </Button>
      </div>
    {/if}
  </div>
</div>

<AlertDialog bind:open={showExecuteConfirm}>
  <AlertDialogContent>
    <AlertDialogHeader>
      <AlertDialogTitle>{m.common_promo_execute()}</AlertDialogTitle>
      <AlertDialogDescription>{m.common_promo_execute_confirm()}</AlertDialogDescription>
    </AlertDialogHeader>
    <AlertDialogFooter>
      <AlertDialogCancel>{m.common_action_cancel()}</AlertDialogCancel>
      <AlertDialogAction onclick={executePromotion}>{m.common_promo_execute()}</AlertDialogAction>
    </AlertDialogFooter>
  </AlertDialogContent>
</AlertDialog>
