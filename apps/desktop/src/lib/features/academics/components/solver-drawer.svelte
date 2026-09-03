<script lang="ts">
  import {
    Sheet,
    SheetContent,
    SheetHeader,
    SheetTitle,
    SheetFooter,
  } from "@orbis/ui/sheet";
  import { Button } from "@orbis/ui/button";
  import { Badge } from "@orbis/ui/badge";
  import { Checkbox } from "@orbis/ui/checkbox";
  import { Label } from "@orbis/ui/label";
  import {
    WandSparkles,
    Loader2,
    CheckCircle2,
    AlertTriangle,
    Eye,
    Check,
    Clock,
    Sparkles,
  } from "@lucide/svelte";
  import { commands, extractErrorMessage } from "$lib/core/ipc";
  import { toast } from "@orbis/ui";
  import * as m from "$lib/paraglide/messages";

  interface Props {
    open: boolean;
    periodId: string;
    previewDraft: boolean;
    activeJob: any;
    onApply?: () => void;
  }

  let {
    open = $bindable(false),
    periodId,
    previewDraft = $bindable(false),
    activeJob = $bindable(null),
    onApply,
  }: Props = $props();

  let isLaunching = $state(false);
  let isApplying = $state(false);
  let solveJobs = $state<any[]>([]);
  let pollTimer: ReturnType<typeof setInterval> | null = null;

  $effect(() => {
    if (open && periodId) {
      loadJobs();
      pollTimer = setInterval(() => {
        if (periodId) loadJobs();
      }, 3000);
    }
    return () => {
      if (pollTimer) {
        clearInterval(pollTimer);
        pollTimer = null;
      }
    };
  });

  async function loadJobs() {
    try {
      const jobs = await commands.scheduling.listJobs(periodId);
      solveJobs = jobs || [];
      if (jobs.length > 0) {
        activeJob = jobs[0];
      }
    } catch (e) {
      console.error("Error loading solve jobs:", e);
    }
  }

  async function handleStartSolve() {
    isLaunching = true;
    try {
      const res = await commands.scheduling.start(periodId);
      activeJob = res;
      toast.success(m.academics_schedule_solver_started());
      await loadJobs();
    } catch (err) {
      toast.error(extractErrorMessage(err));
    } finally {
      isLaunching = false;
    }
  }

  async function handleApplySolve(jobId: string) {
    isApplying = true;
    try {
      await commands.scheduling.apply(jobId);
      toast.success(m.academics_schedule_solver_applied());
      previewDraft = false;
      open = false;
      onApply?.();
    } catch (err) {
      toast.error(extractErrorMessage(err));
    } finally {
      isApplying = false;
    }
  }

  async function handleCancelJob(jobId: string) {
    try {
      await commands.scheduling.cancel(jobId);
      toast.info(m.academics_schedule_solver_cancelled());
      loadJobs();
    } catch (err) {
      toast.error(extractErrorMessage(err));
    }
  }
</script>

<Sheet bind:open>
  <SheetContent class="w-full sm:max-w-lg overflow-y-auto flex flex-col p-6 text-start">
    <SheetHeader>
      <div class="flex items-center gap-2">
        <div class="p-2 rounded-lg bg-primary/10 text-primary">
          <WandSparkles size={18} />
        </div>
        <div>
          <SheetTitle>
            {m.academics_schedule_solver_title()}
          </SheetTitle>
        </div>
      </div>
    </SheetHeader>

    <div class="mt-4 flex-1 flex flex-col gap-4">
      <!-- Solver Trigger Banner -->
      <div class="rounded-xl border border-primary/20 bg-primary/5 p-4 flex flex-col gap-3">
        <div class="flex items-start justify-between">
          <div class="space-y-0.5">
            <h4 class="text-xs font-bold text-foreground">
              {m.academics_schedule_generate_current()}
            </h4>
            <p class="text-[11px] text-muted-foreground leading-relaxed">
              {m.academics_schedule_generate_desc()}
            </p>
          </div>
        </div>

        <Button
          type="button"
          size="sm"
          class="w-full gap-2 text-xs font-bold shadow-xs"
          disabled={isLaunching || activeJob?.status === "running" || activeJob?.status === "pending"}
          onclick={handleStartSolve}
        >
          {#if isLaunching || activeJob?.status === "running" || activeJob?.status === "pending"}
            <Loader2 size={13} class="animate-spin" />
            <span>{m.academics_schedule_computing()}</span>
          {:else}
            <Sparkles size={13} />
            <span>{m.academics_schedule_start_now()}</span>
          {/if}
        </Button>
      </div>

      <!-- Active / Latest Job Status Card -->
      {#if activeJob}
        <div class="rounded-xl border border-border/60 bg-card p-4 flex flex-col gap-3 shadow-xs">
          <div class="flex items-center justify-between">
            <span class="text-xs font-bold text-foreground">
              {m.academics_schedule_latest_job()}
            </span>

            {#if activeJob.status === "completed"}
              <Badge variant="default" class="bg-emerald-600 text-white gap-1 text-[10px]">
                <CheckCircle2 size={10} />
                {m.academics_schedule_status_completed()}
              </Badge>
            {:else if activeJob.status === "running" || activeJob.status === "pending"}
              <Badge variant="secondary" class="bg-primary/10 text-primary gap-1 text-[10px] animate-pulse">
                <Loader2 size={10} class="animate-spin" />
                {m.academics_schedule_status_running()}
              </Badge>
            {:else if activeJob.status === "failed"}
              <Badge variant="destructive" class="gap-1 text-[10px]">
                <AlertTriangle size={10} />
                {m.academics_schedule_status_failed()}
              </Badge>
            {:else}
              <Badge variant="outline" class="text-[10px]">
                {activeJob.status}
              </Badge>
            {/if}
          </div>

          <div class="grid grid-cols-2 gap-2 text-xs text-muted-foreground">
            <div>
              <span class="text-[10px] block">{m.academics_schedule_items_created()}:</span>
              <span class="font-bold text-foreground">{activeJob.items_created || 0}</span>
            </div>
            <div>
              <span class="text-[10px] block">{m.academics_schedule_score()}:</span>
              <span class="font-bold text-foreground font-mono">{activeJob.score || "N/A"}</span>
            </div>
          </div>

          {#if activeJob.status === "running"}
            <Button
              type="button"
              variant="outline"
              size="sm"
              class="h-7 text-xs text-destructive border-destructive/30 hover:bg-destructive/10"
              onclick={() => handleCancelJob(activeJob.id)}
            >
              {m.academics_schedule_cancel()}
            </Button>
          {/if}

          <!-- Draft Preview & Apply Actions (If completed) -->
          {#if activeJob.status === "completed" && activeJob.best_solution}
            <div class="flex flex-col gap-2 pt-2 border-t border-border/40">
              <div class="flex items-center gap-2">
                <Checkbox
                  id="preview-draft-check"
                  checked={previewDraft}
                  onCheckedChange={(val) => (previewDraft = Boolean(val))}
                />
                <Label for="preview-draft-check" class="text-xs cursor-pointer font-semibold flex items-center gap-1">
                  <Eye size={12} class="text-primary" />
                  <span>{m.academics_schedule_preview_grid()}</span>
                </Label>
              </div>

              <Button
                type="button"
                variant="default"
                size="sm"
                class="w-full gap-1.5 text-xs font-bold bg-emerald-600 hover:bg-emerald-700 text-white shadow-xs mt-1"
                disabled={isApplying}
                onclick={() => handleApplySolve(activeJob.id)}
              >
                {#if isApplying}
                  <Loader2 size={13} class="animate-spin" />
                  <span>{m.academics_schedule_applying()}</span>
                {:else}
                  <Check size={14} />
                  <span>{m.academics_schedule_apply_proposed()}</span>
                {/if}
              </Button>
            </div>
          {/if}
        </div>
      {/if}

      <!-- Past Solve Jobs History -->
      {#if solveJobs.length > 1}
        <div class="flex flex-col gap-1.5 mt-2">
          <span class="text-[11px] font-bold text-muted-foreground uppercase tracking-wider">
            {m.academics_schedule_previous_runs()}
          </span>

          <div class="flex flex-col gap-1 max-h-48 overflow-y-auto">
            {#each solveJobs.slice(1) as job (job.id)}
              <div class="flex items-center justify-between p-2 rounded-md border border-border/40 bg-card text-xs">
                <div class="flex items-center gap-1.5">
                  <Clock size={11} class="text-muted-foreground" />
                  <span class="text-[10px] text-muted-foreground font-mono">
                    {new Date(job.created_at).toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" })}
                  </span>
                  <Badge variant="outline" class="h-4 text-[9px]">{job.status}</Badge>
                </div>

                {#if job.status === "completed"}
                  <Button
                    variant="ghost"
                    size="sm"
                    class="h-6 text-[11px] font-semibold text-primary"
                    onclick={() => handleApplySolve(job.id)}
                  >
                    {m.academics_schedule_apply()}
                  </Button>
                {/if}
              </div>
            {/each}
          </div>
        </div>
      {/if}
    </div>

    <SheetFooter class="mt-6 border-t border-border/40 pt-4">
      <Button variant="outline" size="sm" class="w-full text-xs" onclick={() => (open = false)}>
        {m.common_action_close()}
      </Button>
    </SheetFooter>
  </SheetContent>
</Sheet>
