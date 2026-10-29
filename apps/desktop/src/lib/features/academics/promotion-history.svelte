<!--
@component PromotionHistory
Displays persisted cohort promotion jobs and their server-reported outcomes.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { extractErrorMessage } from "$lib/api";
import { formatDate, l } from "$lib/core/i18n";
import { createPeriodsQuery } from "$lib/features/academics/queries";
import { createprogressionJobsQuery } from "$lib/features/academics/promotion-queries";
import { Badge } from "@orbis/ui/badge";
import { Button } from "@orbis/ui/button";
import { LoadingIndicator } from "@orbis/ui";
import { Card, CardContent } from "@orbis/ui/card";
import { ArrowPath, CheckCircle, ClockRotateRight, XMark } from "@orbis/icons";

let { orgId }: { orgId: string } = $props();

const jobsQuery = createprogressionJobsQuery(() => orgId);
const periodsQuery = createPeriodsQuery(() => orgId);
const jobs = $derived(jobsQuery.data ?? []);
const periods = $derived(periodsQuery.data ?? []);

function periodLabel(periodId?: string | null): string {
  if (!periodId) return "—";
  const period = periods.find((item) => item.id === periodId);
  return period ? l(period.name_ar, period.name) : periodId;
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
    rolled_back: m.common_action_back,
  };
  return map[status]?.() ?? status;
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
          <Card class="rounded-none border-0 shadow-none">
            <CardContent class="flex flex-wrap items-center justify-between gap-4 p-4 sm:px-5">
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

              <dl class="grid grid-cols-2 gap-x-4 gap-y-1 text-xs sm:grid-cols-5">
                <div><dt class="text-muted-foreground">{m.common_promo_total()}</dt><dd class="font-semibold tabular-nums">{job.total_students}</dd></div>
                <div><dt class="text-muted-foreground">{m.common_promo_promoted()}</dt><dd class="font-semibold tabular-nums">{job.promoted_count}</dd></div>
                <div><dt class="text-muted-foreground">{m.common_promo_retained()}</dt><dd class="font-semibold tabular-nums">{job.retained_count}</dd></div>
                <div><dt class="text-muted-foreground">{m.common_form_status_graduated()}</dt><dd class="font-semibold tabular-nums">{job.graduated_count}</dd></div>
                <div><dt class="text-muted-foreground">{m.common_promo_excluded()}</dt><dd class="font-semibold tabular-nums">{job.excluded_count}</dd></div>
              </dl>
            </CardContent>
          </Card>
        {/each}
      </div>
    </div>
  {/if}
</div>
