<!--
@component PromotionHistory
Shows a list of past promotion jobs with status, stats, and rollback option.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { commands } from "$lib/core/ipc";
import { extractErrorMessage } from "$lib/core/ipc";
import { toast } from "svelte-sonner";
import { createPromotionJobsQuery, createRollbackMutation } from "$lib/features/academics/year-queries";
import {
  Card, CardContent, CardHeader, CardTitle,
} from "@orbis/ui-svelte/card";
import { Badge } from "@orbis/ui-svelte/badge";
import { Button } from "@orbis/ui-svelte/button";
import { Separator } from "@orbis/ui-svelte/separator";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import {
  AlertDialog, AlertDialogContent, AlertDialogHeader, AlertDialogFooter,
  AlertDialogTitle, AlertDialogDescription, AlertDialogAction, AlertDialogCancel,
} from "@orbis/ui-svelte/alert-dialog";
import {
  Table, TableBody, TableHead, TableHeader, TableRow, TableCell,
} from "@orbis/ui-svelte/table";
import {
  History, Undo2, CheckCircle2, XCircle, Clock, Loader2, AlertTriangle,
} from "lucide-svelte";
import type { PromotionJob } from "@orbis/desktop-client/types";

let { orgId, userId }: { orgId: string; userId: string } = $props();

const jobsQuery = createPromotionJobsQuery(orgId);
const rollbackMutation = createRollbackMutation();

let rollbackTarget = $state<PromotionJob | null>(null);

async function handleRollback() {
  if (!rollbackTarget) return;
  try {
    await $rollbackMutation.mutateAsync({
      jobId: rollbackTarget.id,
      userId,
    });
    toast.success(m.promo_rollback());
    rollbackTarget = null;
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e));
  }
}

function statusIcon(status: string) {
  switch (status) {
    case "completed": return CheckCircle2;
    case "failed": return XCircle;
    case "executing": return Loader2;
    case "rolled_back": return Undo2;
    default: return Clock;
  }
}

function statusVariant(status: string) {
  switch (status) {
    case "completed": return "success" as const;
    case "failed": return "destructive" as const;
    case "rolled_back": return "secondary" as const;
    case "executing": return "brand" as const;
    default: return "outline" as const;
  }
}

function statusLabel(status: string) {
  const map: Record<string, () => string> = {
    validating: m.promo_job_status_validating,
    dry_run: m.promo_job_status_dry_run,
    confirmed: m.promo_job_status_confirmed,
    executing: m.promo_job_status_executing,
    completed: m.promo_job_status_completed,
    failed: m.promo_job_status_failed,
    rolled_back: m.promo_job_status_rolled_back,
  };
  return (map[status] ?? (() => status))();
}
</script>

<div class="flex flex-col gap-4">
  <div class="flex items-center justify-between">
    <div class="flex items-center gap-2">
      <History class="size-5 text-muted-foreground" />
      <h2 class="text-lg font-semibold text-foreground">{m.promo_history()}</h2>
    </div>
  </div>

  {#if $jobsQuery.isLoading}
    <div class="space-y-3">
      {#each Array(3) as _}
        <Card>
          <CardContent class="py-4">
            <div class="flex items-center gap-4">
              <Skeleton class="size-10 rounded-full" />
              <div class="flex-1 space-y-2">
                <Skeleton class="h-4 w-40" />
                <Skeleton class="h-3 w-24" />
              </div>
              <Skeleton class="h-6 w-20" />
            </div>
          </CardContent>
        </Card>
      {/each}
    </div>
  {:else if ($jobsQuery.data ?? []).length === 0}
    <Card class="border-dashed">
      <CardContent class="flex flex-col items-center justify-center py-10 text-center">
        <History class="size-10 text-muted-foreground/40 mb-3" />
        <h3 class="font-semibold text-foreground">{m.promo_no_jobs()}</h3>
        <p class="text-sm text-muted-foreground mt-1">{m.promo_no_jobs_desc()}</p>
      </CardContent>
    </Card>
  {:else}
    <div class="space-y-3">
      {#each $jobsQuery.data ?? [] as job}
        {@const Icon = statusIcon(job.status)}
        <Card>
          <CardContent class="py-4">
            <div class="flex items-center gap-4">
              <div class="size-10 rounded-full bg-muted flex items-center justify-center shrink-0">
                <Icon class="size-5 text-muted-foreground {job.status === 'executing' ? 'animate-spin' : ''}" />
              </div>

              <div class="flex-1 min-w-0">
                <div class="flex items-center gap-2">
                  <span class="font-medium text-sm text-foreground truncate">
                    {(() => { try { return JSON.parse(job.config).target_year_name; } catch { return job.id; } })()}
                  </span>
                  <Badge variant={statusVariant(job.status)} class="shrink-0">
                    {statusLabel(job.status)}
                  </Badge>
                </div>
                <div class="text-xs text-muted-foreground mt-0.5">
                  {new Date(job.created_at).toLocaleDateString()}
                  {#if job.total_students}
                    · {job.total_students} {m.promo_total().toLowerCase()}
                  {/if}
                </div>
              </div>

              <!-- Stats -->
              {#if job.promoted_count != null}
                <div class="hidden md:flex items-center gap-3 text-xs shrink-0">
                  <span class="text-primary font-medium">{job.promoted_count} ↑</span>
                  <span class="text-destructive font-medium">{job.retained_count ?? 0} ↓</span>
                  <span class="text-muted-foreground">{job.graduated_count ?? 0} 🎓</span>
                </div>
              {/if}

              <!-- Rollback -->
              {#if job.status === "completed" && job.rollback_available === 1}
                <Button
                  variant="ghost"
                  size="icon-sm"
                  onclick={() => rollbackTarget = job}
                >
                  <Undo2 class="size-4 text-muted-foreground" />
                </Button>
              {/if}
            </div>
          </CardContent>
        </Card>
      {/each}
    </div>
  {/if}
</div>

<!-- Rollback Confirmation -->
<AlertDialog open={!!rollbackTarget} onOpenChange={(v) => { if (!v) rollbackTarget = null; }}>
  <AlertDialogContent>
    <AlertDialogHeader>
      <AlertDialogTitle>
        <div class="flex items-center gap-2">
          <AlertTriangle class="size-5 text-destructive" />
          {m.promo_rollback()}
        </div>
      </AlertDialogTitle>
      <AlertDialogDescription>{m.promo_rollback_confirm()}</AlertDialogDescription>
    </AlertDialogHeader>
    <AlertDialogFooter>
      <AlertDialogCancel onclick={() => rollbackTarget = null}>{m.common_cancel?.() ?? "Cancel"}</AlertDialogCancel>
      <AlertDialogAction onclick={handleRollback}>{m.promo_rollback()}</AlertDialogAction>
    </AlertDialogFooter>
  </AlertDialogContent>
</AlertDialog>
