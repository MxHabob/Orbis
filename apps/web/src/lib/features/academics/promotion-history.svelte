<!--
@component PromotionHistory
Displays persisted cohort promotion jobs, per-student audit details, and safe rollback actions.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { extractErrorMessage } from "$lib/api";
import { formatDate, l } from "$lib/core/i18n";
import {
  createPeriodsQuery,
  createSectionsQuery,
} from "$lib/features/academics/queries";
import {
  createprogressionJobsQuery,
  createProgressionJobDetailsQuery,
  createRollbackPromotionMutation,
} from "$lib/features/academics/promotion-queries";
import { toast, LoadingIndicator } from "@orbis/ui";
import { Badge } from "@orbis/ui/badge";
import { Button } from "@orbis/ui/button";
import { Card, CardContent } from "@orbis/ui/card";
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
import { ArrowPath, CheckCircle, ClockRotateRight, XMark } from "@orbis/icons";

let { orgId }: { orgId: string } = $props();

const jobsQuery = createprogressionJobsQuery(() => orgId);
const periodsQuery = createPeriodsQuery(() => orgId);
const sectionsQuery = createSectionsQuery(() => orgId);
const rollbackMutation = createRollbackPromotionMutation();

const jobs = $derived(jobsQuery.data ?? []);
const periods = $derived(periodsQuery.data ?? []);
const sections = $derived(sectionsQuery.data ?? []);

let expandedJobId = $state<string | null>(null);
let rollbackTargetId = $state<string | null>(null);
const detailsQuery = createProgressionJobDetailsQuery(() => expandedJobId);

function periodLabel(periodId?: string | null): string {
  if (!periodId) return "—";
  const period = periods.find((item) => item.id === periodId);
  return period
    ? l(period.name_ar, period.name) ||
        period.name ||
        period.name_ar ||
        periodId
    : periodId;
}

function sectionLabel(sectionId?: string | null): string {
  if (!sectionId) return "—";
  const section = sections.find((item) => item.id === sectionId);
  return section ? l(section.name_ar, section.name) || section.code : sectionId;
}

function statusIcon(status: string) {
  switch (status) {
    case "completed":
      return CheckCircle;
    case "failed":
      return XMark;
    case "executing":
      return ArrowPath;
    default:
      return ClockRotateRight;
  }
}

function statusVariant(status: string) {
  switch (status) {
    case "completed":
      return "secondary" as const;
    case "failed":
      return "destructive" as const;
    case "executing":
      return "default" as const;
    default:
      return "outline" as const;
  }
}

function statusLabel(status: string) {
  const map: Record<string, () => string> = {
    draft: m.common_promo_job_status_dry_run,
    preview_ready: m.common_promo_job_status_dry_run,
    executing: m.common_promo_job_status_executing,
    completed: m.common_schedule_ai_status_done,
    failed: m.common_calendar_status_failed,
    rolled_back: m.common_promo_rolled_back,
  };
  return map[status]?.() ?? status;
}

function decisionLabel(decision: string) {
  switch (decision) {
    case "promote":
      return m.common_promo_promoted();
    case "retain":
      return m.common_promo_retained();
    case "graduate":
      return m.common_form_status_graduated();
    default:
      return m.common_promo_excluded();
  }
}

async function confirmRollback() {
  if (!rollbackTargetId) return;
  const targetId = rollbackTargetId;
  rollbackTargetId = null;
  try {
    await rollbackMutation.mutateAsync(targetId);
    toast.success(m.common_promo_rolled_back());
  } catch (error: unknown) {
    toast.error(extractErrorMessage(error));
  }
}
</script>

<div class="mx-auto flex w-full max-w-5xl flex-col gap-5">
  <header class="space-y-1 border-b border-border/60 pb-4 text-start">
    <h1 class="text-lg font-semibold text-foreground">{m.common_promo_history()}</h1>
    <p class="text-sm text-muted-foreground">{m.common_promo_desc_history_subtitle()}</p>
  </header>

  {#if jobsQuery.isLoading}
    <div class="flex items-center justify-center gap-2 py-12 text-sm text-muted-foreground" role="status">
      <LoadingIndicator size={18} />
      {m.common_loading()}
    </div>
  {:else if jobsQuery.isError}
    <div class="flex flex-col items-start gap-3 rounded-lg border border-destructive/30 bg-destructive/5 p-4 text-sm" role="alert">
      <p class="text-destructive">{extractErrorMessage(jobsQuery.error)}</p>
      <Button variant="outline" size="sm" onclick={() => jobsQuery.refetch()}>
        <ArrowPath class="me-2 size-4" />
        {m.common_action_retry()}
      </Button>
    </div>
  {:else if jobs.length === 0}
    <div class="rounded-lg border border-dashed border-border px-6 py-12 text-center">
      <ClockRotateRight class="mx-auto mb-3 size-8 text-muted-foreground/60" />
      <h2 class="text-sm font-semibold">{m.common_promo_no_jobs()}</h2>
      <p class="mt-1 text-sm text-muted-foreground">{m.common_promo_no_jobs_desc()}</p>
    </div>
  {:else}
    <div class="overflow-hidden rounded-lg border border-border/70">
      <div class="divide-y divide-border/60">
        {#each jobs as job (job.id)}
          {@const Icon = statusIcon(job.status)}
          {@const isExpanded = expandedJobId === job.id}
          <Card class="rounded-none border-0 shadow-none">
            <CardContent class="flex flex-col gap-4 p-4 sm:px-5">
              <div class="flex flex-wrap items-center justify-between gap-4">
                <div class="flex min-w-0 items-center gap-3">
                  <div class="flex size-9 shrink-0 items-center justify-center rounded-md bg-muted text-muted-foreground">
                    {#if job.status === "executing"}
                      <LoadingIndicator size={16} />
                    {:else}
                      <Icon class="size-4" />
                    {/if}
                  </div>
                  <div class="min-w-0 text-start">
                    <div class="flex flex-wrap items-center gap-2">
                      <h2 class="max-w-full truncate text-sm font-semibold">{job.name}</h2>
                      <Badge variant={statusVariant(job.status)} class="text-xs">
                        {statusLabel(job.status)}
                      </Badge>
                    </div>
                    <p class="mt-1 text-xs text-muted-foreground">
                      {periodLabel(job.source_period_id)} → {periodLabel(job.target_period_id)}
                      <span class="mx-1">·</span>
                      {formatDate(job.created_at)}
                    </p>
                  </div>
                </div>

                <div class="flex flex-wrap items-center gap-4">
                  <dl class="grid grid-cols-2 gap-x-4 gap-y-1 text-xs sm:grid-cols-5">
                    <div><dt class="text-muted-foreground">{m.common_promo_total()}</dt><dd class="font-semibold tabular-nums">{job.total_students}</dd></div>
                    <div><dt class="text-muted-foreground">{m.common_promo_promoted()}</dt><dd class="font-semibold tabular-nums">{job.promoted_count}</dd></div>
                    <div><dt class="text-muted-foreground">{m.common_promo_retained()}</dt><dd class="font-semibold tabular-nums">{job.retained_count}</dd></div>
                    <div><dt class="text-muted-foreground">{m.common_form_status_graduated()}</dt><dd class="font-semibold tabular-nums">{job.graduated_count}</dd></div>
                    <div><dt class="text-muted-foreground">{m.common_promo_excluded()}</dt><dd class="font-semibold tabular-nums">{job.excluded_count}</dd></div>
                  </dl>

                  <div class="flex items-center gap-2">
                    <Button
                      variant="outline"
                      size="sm"
                      onclick={() => {
                        expandedJobId = isExpanded ? null : job.id;
                      }}
                    >
                      {m.common_promo_view_details()}
                    </Button>
                    {#if job.status === "completed"}
                      <Button
                        variant="outline"
                        size="sm"
                        disabled={rollbackMutation.isPending}
                        onclick={() => (rollbackTargetId = job.id)}
                      >
                        <ClockRotateRight class="me-1.5 size-3.5" />
                        {m.common_promo_rollback()}
                      </Button>
                    {/if}
                  </div>
                </div>
              </div>

              {#if isExpanded}
                <div class="rounded-md border border-border/60 bg-muted/20 p-3">
                  {#if detailsQuery.isLoading}
                    <div class="flex items-center gap-2 py-4 text-xs text-muted-foreground">
                      <LoadingIndicator size={14} />
                      {m.common_loading()}
                    </div>
                  {:else if detailsQuery.isError}
                    <p class="text-xs text-destructive">{extractErrorMessage(detailsQuery.error)}</p>
                  {:else if (detailsQuery.data ?? []).length === 0}
                    <p class="text-xs text-muted-foreground">—</p>
                  {:else}
                    <div class="divide-y divide-border/50">
                      {#each detailsQuery.data ?? [] as detail (detail.id)}
                        <div class="flex flex-wrap items-center justify-between gap-2 py-2 text-xs">
                          <div class="min-w-0 text-start">
                            <p class="font-medium text-foreground">
                              {sectionLabel(detail.source_section_id)} → {sectionLabel(detail.target_section_id)}
                            </p>
                            <p class="text-muted-foreground">
                              {detail.reason}
                              {#if detail.manual_override && detail.override_reason}
                                <span class="mx-1">·</span>
                                ({detail.override_reason})
                              {/if}
                            </p>
                          </div>
                          <div class="flex items-center gap-2">
                            {#if detail.final_gpa != null}
                              <span class="text-muted-foreground tabular-nums">GPA {detail.final_gpa}</span>
                            {/if}
                            {#if detail.attendance_rate != null}
                              <span class="text-muted-foreground tabular-nums">{detail.attendance_rate}%</span>
                            {/if}
                            <Badge variant="outline" class="text-[11px]">
                              {decisionLabel(detail.decision)}
                            </Badge>
                          </div>
                        </div>
                      {/each}
                    </div>
                  {/if}
                </div>
              {/if}
            </CardContent>
          </Card>
        {/each}
      </div>
    </div>
  {/if}
</div>

<AlertDialog
  open={rollbackTargetId !== null}
  onOpenChange={(open) => {
    if (!open) rollbackTargetId = null;
  }}
>
  <AlertDialogContent>
    <AlertDialogHeader>
      <AlertDialogTitle>{m.common_promo_rollback()}</AlertDialogTitle>
      <AlertDialogDescription>{m.common_promo_rollback_confirm()}</AlertDialogDescription>
    </AlertDialogHeader>
    <AlertDialogFooter>
      <AlertDialogCancel>{m.common_action_cancel()}</AlertDialogCancel>
      <AlertDialogAction onclick={confirmRollback}>{m.common_promo_rollback()}</AlertDialogAction>
    </AlertDialogFooter>
  </AlertDialogContent>
</AlertDialog>
