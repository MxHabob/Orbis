<!--
@component PromotionHistory
Shows a list of past promotion jobs with status, stats, and rollback option.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";

import { extractErrorMessage } from "$lib/core/ipc";
import { toast } from "@orbis/ui";
import {
  createPromotionJobsQuery,
  createRollbackMutation,
} from "$lib/features/academics/promotion-queries";
import { Badge } from "@orbis/ui/badge";
import { Button } from "@orbis/ui/button";
import { Skeleton } from "@orbis/ui/skeleton";
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
  History,
  Undo2,
  CheckCircle2,
  XCircle,
  Clock,
  Loader2,
  AlertTriangle,
} from "@lucide/svelte";
import type { PromotionJob } from "@orbis/sdk-core";

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
      return CheckCircle2;
    case "failed":
      return XCircle;
    case "executing":
      return Loader2;
    case "rolled_back":
      return Undo2;
    default:
      return Clock;
  }
}

function statusVariant(status: string) {
  switch (status) {
    case "completed":
      return "success" as const;
    case "failed":
      return "destructive" as const;
    case "rolled_back":
      return "secondary" as const;
    case "executing":
      return "brand" as const;
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

<div class="flex flex-col gap-6 w-full max-w-3xl mx-auto">
  <div class="space-y-2 text-start select-none">
    <h3 class="text-foreground font-bold text-xl font-serif flex items-center gap-2">
      <History class="size-5 text-muted-foreground" />
      {m.common_promo_history()}
    </h3>
    <p class="text-muted-foreground text-xs font-sans">
      {m.common_promo_desc_history_subtitle()}
    </p>
  </div>

  {#if jobsQuery.isLoading}
    <div class="space-y-4">
      {#each Array(3) as _}
        <div class="border border-border bg-card p-5 rounded-xl">
          <div class="flex items-center gap-4">
            <Skeleton class="size-10 rounded-full shrink-0" />
            <div class="flex-1 space-y-2">
              <Skeleton class="h-4 w-40" />
              <Skeleton class="h-3 w-24" />
            </div>
            <Skeleton class="h-6 w-20 shrink-0" />
          </div>
        </div>
      {/each}
    </div>
  {:else if (jobsQuery.data ?? []).length === 0}
    <div class="border-2 border-dashed border-border rounded-xl bg-card p-12 text-center flex flex-col items-center justify-center">
      <History class="size-10 text-muted-foreground/40 mb-3" />
      <h3 class="font-bold text-foreground font-serif">{m.common_promo_no_jobs()}</h3>
      <p class="text-xs text-muted-foreground mt-1 font-sans">{m.common_promo_no_jobs_desc()}</p>
    </div>
  {:else}
    <div class="space-y-4">
      {#each jobsQuery.data ?? [] as job}
        {@const Icon = statusIcon(job.status)}
        <div class="border border-border bg-card p-5 rounded-xl transition-all hover:border-border/80">
          <div class="flex items-center justify-between gap-4">
            <div class="flex items-center gap-4 min-w-0">
              <div class="size-10 rounded-full bg-muted flex items-center justify-center shrink-0 select-none">
                <Icon class="size-5 text-muted-foreground {job.status === 'executing' ? 'animate-spin' : ''}" />
              </div>

              <div class="min-w-0 text-start">
                <div class="flex flex-wrap items-center gap-2">
                  <span class="font-bold text-sm text-foreground truncate font-sans">
                    {(() => { try { return JSON.parse(job.config).target_year_name; } catch { return job.id; } })()}
                  </span>
                  <Badge variant={statusVariant(job.status)} class="shrink-0 text-[10px] py-0 px-2 select-none">
                    {statusLabel(job.status)}
                  </Badge>
                </div>
                <div class="text-[11px] text-muted-foreground mt-1 font-sans">
                  {new Date(job.created_at).toLocaleDateString()}
                  {#if job.total_students}
                    &middot; {job.total_students} {m.common_promo_total().toLowerCase()}
                  {/if}
                </div>
              </div>
            </div>

            <div class="flex items-center gap-4 shrink-0">
              <!-- Stats -->
              {#if job.promoted_count != null}
                <div class="hidden sm:flex items-center gap-3 text-[11px] font-sans font-medium select-none">
                  <span class="text-primary">{job.promoted_count} &uarr;</span>
                  <span class="text-destructive">{job.retained_count ?? 0} &darr;</span>
                  <span class="text-muted-foreground">{job.graduated_count ?? 0} 🎓</span>
                </div>
              {/if}

              <!-- Rollback -->
              {#if job.status === "completed"}
                <Button
                  variant="ghost"
                  size="icon-xs"
                  onclick={() => rollbackTarget = job}
                  class="text-muted-foreground hover:text-foreground hover:bg-muted select-none"
                >
                  <Undo2 class="size-4" />
                </Button>
              {/if}
            </div>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

<AlertDialog open={!!rollbackTarget} onOpenChange={(v) => { if (!v) rollbackTarget = null; }}>
  <AlertDialogContent>
    <AlertDialogHeader>
      <AlertDialogTitle>
        <div class="flex items-center gap-2">
          <AlertTriangle class="size-5 text-destructive" />
          {m.common_action_back()}
        </div>
      </AlertDialogTitle>
      <AlertDialogDescription>{m.common_action_back()}</AlertDialogDescription>
    </AlertDialogHeader>
    <AlertDialogFooter>
      <AlertDialogCancel onclick={() => rollbackTarget = null}>{m.common_action_cancel()}</AlertDialogCancel>
      <AlertDialogAction onclick={handleRollback}>{m.common_action_back()}</AlertDialogAction>
    </AlertDialogFooter>
  </AlertDialogContent>
</AlertDialog>
