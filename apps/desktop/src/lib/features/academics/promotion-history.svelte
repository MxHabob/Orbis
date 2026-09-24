<!--
@component PromotionHistory
Shows a list of past promotion jobs with status, stats, and rollback option using standardized Cards.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";

import { extractErrorMessage } from "$lib/api";
import { toast } from "@orbis/ui";
import { formatDate } from "$lib/core/i18n";
import {
  createPromotionJobsQuery,
  createRollbackMutation,
} from "$lib/features/academics/promotion-queries";
import { Badge } from "@orbis/ui/badge";
import { Button } from "@orbis/ui/button";
import { Skeleton } from "@orbis/ui/skeleton";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardAction,
  CardContent,
} from "@orbis/ui/card";
import {
  AlertDialog,
  AlertDialogContent,
  AlertDialogHeader,
  AlertDialogFooter,
  AlertDialogTitle,
  AlertDialogDescription,
  AlertDialogAction,
  AlertDialogCancel,
} from "@orbis/ui/alert-dialog";
import {
  ArrowPath,
  CheckCircle,
  ClockRotateRight,
  InfoCircle,
  UndoAction,
  XMark,
} from "@orbis/icons";
import type { PromotionJob } from "@orbis/sdk";

let { orgId, userId }: { orgId: string; userId: string } = $props();

const jobsQuery = createPromotionJobsQuery(() => orgId);
const rollbackMutation = createRollbackMutation();

let rollbackTarget = $state<PromotionJob | null>(null);

async function handleRollback() {
  if (!rollbackTarget) return;
  try {
    await rollbackMutation.mutateAsync({
      jobId: rollbackTarget.id,
      userId,
    });
    toast.success(m.common_action_back());
    rollbackTarget = null;
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e));
  }
}

function statusIcon(status: string) {
  switch (status) {
    case "completed":
      return CheckCircle;
    case "failed":
      return XMark;
    case "executing":
      return ArrowPath;
    case "rolled_back":
      return UndoAction;
    default:
      return ClockRotateRight;
  }
}

function statusVariant(status: string) {
  switch (status) {
    case "completed":
      return "secondary" as const;
    case "failed":
      return "outline" as const;
    case "rolled_back":
      return "secondary" as const;
    case "executing":
      return "default" as const;
    default:
      return "outline" as const;
  }
}

function statusLabel(status: string) {
  const map: Record<string, () => string> = {
    validating: m.common_promo_job_status_validating,
    dry_run: m.common_promo_job_status_dry_run,
    confirmed: m.common_promo_job_status_confirmed,
    executing: m.common_promo_job_status_executing,
    completed: m.common_schedule_ai_status_done,
    failed: m.common_calendar_status_failed,
    rolled_back: m.common_action_back,
  };
  return (map[status] ?? (() => status))();
}
</script>

<div class="flex flex-col gap-6 w-full max-w-4xl mx-auto font-sans">
  <div class="space-y-1 text-start select-none border-b border-border/40 pb-3">
    <h3 class="text-foreground font-bold text-base flex items-center gap-2">
      <ClockRotateRight class="size-4 text-primary" />
      {m.common_promo_history()}
    </h3>
    <p class="text-muted-foreground text-xs">
      {m.common_promo_desc_history_subtitle()}
    </p>
  </div>

  {#if jobsQuery.isLoading}
    <div class="space-y-4">
      {#each Array(3) as _}
        <Card>
          <CardContent class="p-5">
            <div class="flex items-center gap-4">
              <Skeleton class="size-10 rounded-full shrink-0" />
              <div class="flex-1 space-y-2">
                <Skeleton class="h-4 w-40" />
                <Skeleton class="h-3 w-24" />
              </div>
              <Skeleton class="h-6 w-20 shrink-0" />
            </div>
          </CardContent>
        </Card>
      {/each}
    </div>
  {:else if (jobsQuery.data ?? []).length === 0}
    <Card class="border-2 border-dashed p-12 text-center flex flex-col items-center justify-center">
      <ClockRotateRight class="size-10 text-muted-foreground/40 mb-3" />
      <CardTitle class="text-sm font-bold text-foreground">{m.common_promo_no_jobs()}</CardTitle>
      <CardDescription class="text-xs text-muted-foreground mt-1">{m.common_promo_no_jobs_desc()}</CardDescription>
    </Card>
  {:else}
    <div class="space-y-3">
      {#each jobsQuery.data ?? [] as job}
        {@const Icon = statusIcon(job.status)}
        <Card class="transition-all hover:border-primary/50">
          <CardContent class="p-4">
            <div class="flex items-center justify-between gap-4">
              <div class="flex items-center gap-3.5 min-w-0">
                <div class="size-9 rounded-lg bg-primary/10 flex items-center justify-center shrink-0 select-none text-primary">
                  <Icon class="size-4 {job.status === 'executing' ? 'animate-spin' : ''}" />
                </div>

                <div class="min-w-0 text-start">
                  <div class="flex flex-wrap items-center gap-2">
                    <span class="font-bold text-xs text-foreground truncate">
                      {(() => { try { return JSON.parse(job.config).target_year_name; } catch { return job.id; } })()}
                    </span>
                    <Badge variant={statusVariant(job.status)} class="shrink-0 text-[9px] py-0 px-1.5 select-none">
                      {statusLabel(job.status)}
                    </Badge>
                  </div>
                  <div class="text-[10px] text-muted-foreground mt-0.5 font-mono">
                    {formatDate(job.created_at)}
                    {#if job.total_students}
                      &middot; {job.total_students} {m.common_promo_total().toLowerCase()}
                    {/if}
                  </div>
                </div>
              </div>

              <div class="flex items-center gap-3 shrink-0">
                <!-- Stats -->
                {#if job.promoted_count != null}
                  <div class="hidden sm:flex items-center gap-2.5 text-[10px] font-mono font-medium select-none">
                    <span class="text-emerald-600 font-bold">{job.promoted_count} &uarr;</span>
                    <span class="text-rose-600 font-bold">{job.retained_count ?? 0} &darr;</span>
                    <span class="text-muted-foreground">{job.graduated_count ?? 0} 🎓</span>
                  </div>
                {/if}

                <!-- Rollback -->
                {#if job.status === "completed"}
                  <Button
                    variant="ghost"
                    size="icon-xs"
                    onclick={() => (rollbackTarget = job)}
                    class="text-muted-foreground hover:text-foreground select-none"
                    title={m.common_action_back()}
                  >
                    <UndoAction class="size-3.5" />
                  </Button>
                {/if}
              </div>
            </div>
          </CardContent>
        </Card>
      {/each}
    </div>
  {/if}
</div>

<AlertDialog open={!!rollbackTarget} onOpenChange={(v) => { if (!v) rollbackTarget = null; }}>
  <AlertDialogContent>
    <AlertDialogHeader>
      <AlertDialogTitle>
        <div class="flex items-center gap-2">
          <InfoCircle class="size-5 text-destructive" />
          {m.common_action_back()}
        </div>
      </AlertDialogTitle>
      <AlertDialogDescription>{m.common_action_back()}</AlertDialogDescription>
    </AlertDialogHeader>
    <AlertDialogFooter>
      <AlertDialogCancel onclick={() => (rollbackTarget = null)}>{m.common_action_cancel()}</AlertDialogCancel>
      <AlertDialogAction onclick={handleRollback}>{m.common_action_back()}</AlertDialogAction>
    </AlertDialogFooter>
  </AlertDialogContent>
</AlertDialog>
