<script lang="ts">
import { Button } from "@orbis/ui/button";
import { Skeleton } from "@orbis/ui/skeleton";
import { Badge } from "@orbis/ui/badge";
import {
  Card,
  CardHeader,
  CardTitle,
  CardAction,
  CardContent,
} from "@orbis/ui/card";
import {
  ArrowRight,
  Bolt,
  ChartBar,
  CheckCircle,
  Clock,
  InfoCircle,
  Lock,
  Play,
  Refresh,
  SettingsAlt,
  Wrench,
  XMark,
} from "@orbis/icons";
import {
  createAiToolsQuery,
  createAutopilotDiagnosticsQuery,
  createAutopilotTasksQuery,
  useConfigureAutopilot,
  useDismissAutopilotTask,
  useExecuteAutopilotTask,
  useRetryAutopilotTask,
  useRunAutopilotTick,
} from "$lib/features/ai/queries";
import { toast } from "@orbis/ui";
import { extractErrorMessage } from "$lib/api";
import type { ActionWithStatus } from "$lib/features/ai/session-store";
import * as m from "$lib/paraglide/messages";

let {
  orgId = "",
  pendingActions = [],
  onBack,
}: {
  orgId: string;
  pendingActions?: ActionWithStatus[];
  onBack: () => void;
} = $props();

// 1. AI Tools Query
const toolsQuery = createAiToolsQuery();
const tools = $derived(toolsQuery.data ?? []);

// 2. Autopilot Diagnostics Query
const diagnosticsQuery = createAutopilotDiagnosticsQuery(() => orgId);
const diagnostics = $derived(
  diagnosticsQuery.data ?? {
    status: "standby",
    mode: "advisory",
    autoLockGrades: false,
    is_healthy: true,
    total_tasks_count: 0,
    pending_tasks_count: 0,
    completed_tasks_count: 0,
    failed_tasks_count: 0,
  },
);

// 3. Backend Autopilot Tasks Query
const tasksQuery = createAutopilotTasksQuery(() => orgId);
const tasks = $derived(tasksQuery.data ?? []);

// Mutations
const executeTask = useExecuteAutopilotTask();
const dismissTask = useDismissAutopilotTask();
const retryTask = useRetryAutopilotTask();
const runTick = useRunAutopilotTick();
const configureAutopilot = useConfigureAutopilot();

let toolSearch = $state("");
const filteredTools = $derived(
  tools.filter((t) => {
    if (!toolSearch.trim()) return true;
    const query = toolSearch.toLowerCase();
    return (
      t.name.toLowerCase().includes(query) ||
      t.description.toLowerCase().includes(query)
    );
  }),
);

async function handleExecuteTask(taskId: string) {
  try {
    await executeTask.mutateAsync(taskId);
    toast.success(m.ai_ui_task_executed());
  } catch (e) {
    toast.error(extractErrorMessage(e));
  }
}

async function handleDismissTask(taskId: string) {
  try {
    await dismissTask.mutateAsync(taskId);
    toast.info(m.ai_ui_task_dismissed());
  } catch (e) {
    toast.error(extractErrorMessage(e));
  }
}

async function handleRetryTask(taskId: string) {
  try {
    await retryTask.mutateAsync(taskId);
    toast.success(m.ai_ui_task_rescheduled());
  } catch (e) {
    toast.error(extractErrorMessage(e));
  }
}

async function handleTriggerTick(dryRun: boolean = false) {
  try {
    await runTick.mutateAsync({ orgId, dryRun });
    toast.success(dryRun ? m.ai_ui_dry_run_started() : m.ai_ui_policy_evaluation_started());
  } catch (e) {
    toast.error(extractErrorMessage(e));
  }
}

async function toggleAutopilotMode() {
  const newMode = diagnostics.mode === "advisory" ? "autonomous" : "advisory";
  try {
    await configureAutopilot.mutateAsync({
      orgId,
      isEnabled: true,
      paused: false,
      mode: newMode,
    });
    toast.success(m.ai_ui_autopilot_mode_changed({ mode: newMode === "autonomous" ? m.ai_ui_mode_autonomous() : m.ai_ui_mode_advisory() }));
  } catch (e) {
    toast.error(extractErrorMessage(e));
  }
}
</script>

<div class="px-4 py-2.5 border-b border-border bg-muted/30 flex items-center justify-between shrink-0 select-none">
  <Button 
    variant="ghost" 
    size="xs" 
    class="text-xs h-7 font-semibold cursor-pointer text-muted-foreground hover:text-foreground flex items-center gap-1"
    onclick={onBack}
  >
    <ArrowRight size={13} class="rtl:rotate-0 rotate-180" />
    {m.ai_ui_return_to_chat()}
  </Button>
  <span class="text-xs font-bold text-foreground flex items-center gap-1.5">
    <ChartBar size={13} class="text-primary" />
    {m.ai_ui_operations_center()}
  </span>
</div>

<div class="flex-1 overflow-y-auto p-4 space-y-5 min-h-0 select-text scrollbar-hide bg-background/50">
  
  <!-- 1. Autopilot Engine Live Control Panel -->
  <Card class="overflow-hidden shadow-xs">
    <CardHeader class="py-3 px-4 flex flex-row items-center justify-between">
      <div class="flex items-center gap-2">
        <Bolt size={15} class="text-primary" />
        <CardTitle class="text-xs font-bold text-foreground">
          {m.ai_ui_autopilot_engine()}
        </CardTitle>
      </div>
      <CardAction>
        <Badge 
          variant={diagnostics.status === 'active' ? 'lime' : 'secondary'}
          class="text-[9px] px-1.5 font-mono"
        >
          {diagnostics.status === 'active' ? m.ai_ui_status_active() : m.ai_ui_status_standby()}
        </Badge>
      </CardAction>
    </CardHeader>

    <CardContent class="p-4 space-y-3">
      <div class="grid grid-cols-2 gap-2 text-[11px]">
        <div class="bg-muted/40 p-2.5 rounded-xl border border-border/50 space-y-0.5">
          <span class="text-muted-foreground block text-[9.5px]">{m.ai_ui_current_operating_mode()}</span>
          <strong class="text-foreground capitalize">{diagnostics.mode === 'autonomous' ? m.ai_ui_mode_autonomous() : m.ai_ui_mode_advisory()}</strong>
        </div>
        <div class="bg-muted/40 p-2.5 rounded-xl border border-border/50 space-y-0.5">
          <span class="text-muted-foreground block text-[9.5px]">{m.ai_ui_pending_system_tasks()}</span>
          <strong class="text-foreground font-mono">{m.ai_ui_task_count({ count: diagnostics.pending_tasks_count })}</strong>
        </div>
      </div>

      <div class="flex items-center gap-2 pt-1">
        <Button
          variant="outline"
          size="xs"
          class="flex-1 text-xs h-7.5 font-semibold cursor-pointer border-border hover:bg-muted"
          onclick={toggleAutopilotMode}
          disabled={configureAutopilot.isPending}
        >
          <SettingsAlt size={12} class="me-1.5" />
          {m.ai_ui_switch_to_mode({ mode: diagnostics.mode === 'advisory' ? m.ai_ui_mode_autonomous() : m.ai_ui_mode_advisory() })}
        </Button>
        <Button
          variant="default"
          size="xs"
          class="text-xs h-7.5 font-semibold cursor-pointer bg-primary text-primary-foreground"
          onclick={() => handleTriggerTick(false)}
          disabled={runTick.isPending}
        >
          <Play size={11} class="me-1" />
          {m.ai_ui_run_check_cycle()}
        </Button>
      </div>
    </CardContent>
  </Card>

  <!-- 2. Backend Live Tasks List (Managed via Clean Backend Services) -->
  <Card class="overflow-hidden shadow-xs">
    <CardHeader class="py-2.5 px-4 flex flex-row items-center justify-between border-b border-border/50">
      <div class="flex items-center gap-1.5">
        <Clock size={13} class="text-primary" />
        <CardTitle class="text-xs font-bold text-foreground">
          {m.ai_ui_smart_system_tasks({ count: tasks.length })}
        </CardTitle>
      </div>
      <Button
        variant="ghost"
        size="icon-xs"
        class="h-6 w-6 rounded-md text-muted-foreground hover:text-foreground cursor-pointer"
        onclick={() => tasksQuery.refetch()}
        title={m.ai_ui_refresh_task_list()}
      >
        <Refresh size={11} />
      </Button>
    </CardHeader>

    <CardContent class="p-3 space-y-2">
      {#if tasksQuery.isLoading}
        <div class="space-y-2">
          <Skeleton class="h-14 w-full rounded-xl" />
          <Skeleton class="h-14 w-full rounded-xl" />
        </div>
      {:else if tasks.length === 0}
        <div class="text-center py-5 text-xs text-muted-foreground bg-muted/20 rounded-xl border border-dashed border-border">
          {m.ai_ui_no_pending_tasks()}
        </div>
      {:else}
        {#each tasks as task (task.id)}
          <div class="border border-border/70 bg-card rounded-xl p-3 space-y-2 text-start hover:border-primary/40 transition-colors shadow-2xs">
            <div class="flex items-center justify-between">
              <span class="text-xs font-bold text-foreground font-mono">{task.task_type}</span>
              <Badge 
                variant={task.status === 'completed' ? 'lime' : task.status === 'failed' ? 'destructive' : 'secondary'}
                class="text-[8.5px] font-medium"
              >
                {task.status}
              </Badge>
            </div>

            {#if task.reason}
              <p class="text-[11px] text-muted-foreground leading-relaxed">
                {task.reason}
              </p>
            {/if}

            <div class="flex items-center justify-between pt-1 border-t border-border/30 text-[10px] text-muted-foreground">
              <span>{m.ai_ui_risk_label()}: <strong class="text-foreground capitalize">{task.risk_level}</strong></span>
              <div class="flex items-center gap-1.5">
                {#if task.status === 'pending' || task.status === 'suggested'}
                  <Button
                    size="xs"
                    variant="default"
                    class="text-[10px] h-6 px-2.5 bg-emerald-600 hover:bg-emerald-700 text-white cursor-pointer"
                    onclick={() => handleExecuteTask(task.id)}
                    disabled={executeTask.isPending}
                  >
                    <CheckCircle size={10} class="me-1" />
                    {m.ai_ui_execute()}
                  </Button>
                  <Button
                    size="xs"
                    variant="ghost"
                    class="text-[10px] h-6 px-2 text-muted-foreground hover:text-destructive cursor-pointer"
                    onclick={() => handleDismissTask(task.id)}
                    disabled={dismissTask.isPending}
                  >
                    {m.ai_ui_cancel()}
                  </Button>
                {:else if task.status === 'failed'}
                  <Button
                    size="xs"
                    variant="outline"
                    class="text-[10px] h-6 px-2 text-destructive border-destructive/30 hover:bg-destructive/10 cursor-pointer"
                    onclick={() => handleRetryTask(task.id)}
                    disabled={retryTask.isPending}
                  >
                    <Refresh size={10} class="me-1" />
                    {m.ai_ui_retry()}
                  </Button>
                {/if}
              </div>
            </div>
          </div>
        {/each}
      {/if}
    </CardContent>
  </Card>

  <!-- 3. AI Platform Tools Capabilities -->
  <Card class="overflow-hidden shadow-xs">
    <CardHeader class="py-3 px-4 flex flex-row items-center justify-between">
      <div class="flex items-center gap-2">
        <Wrench size={14} class="text-primary" />
        <CardTitle class="text-xs font-bold text-foreground">
          {m.ai_ui_agent_available_tools({ count: tools.length })}
        </CardTitle>
      </div>
    </CardHeader>

    <CardContent class="p-4">
      {#if toolsQuery.isLoading}
        <div class="space-y-2">
          <Skeleton class="h-16 w-full rounded-lg" />
          <Skeleton class="h-16 w-full rounded-lg" />
        </div>
      {:else if toolsQuery.isError}
        <div class="border border-destructive/20 bg-destructive/5 rounded-lg p-3 text-center text-xs text-destructive">
          {m.ai_ui_failed_load_system_tools()}
        </div>
      {:else if filteredTools.length === 0}
        <div class="text-center py-6 text-xs text-muted-foreground">
          {m.ai_ui_no_matching_tools()}
        </div>
      {:else}
        <div class="space-y-2">
          {#each filteredTools as tool}
            <div class="border border-border/70 rounded-xl p-3 bg-card space-y-1.5 text-start hover:border-primary/40 transition-colors shadow-2xs">
              <div class="flex items-center justify-between">
                <span class="text-xs font-bold font-mono text-primary flex items-center gap-1.5">
                  <Wrench size={11} />
                  {tool.name}
                </span>
                <div class="flex items-center gap-1">
                  {#if tool.is_consequential}
                    <Badge variant="secondary" class="text-[8px] bg-amber-500/15 text-amber-700 dark:text-amber-300 border-amber-500/20">
                      {m.ai_ui_modify_data()}
                    </Badge>
                  {:else}
                    <Badge variant="outline" class="text-[8px] text-muted-foreground">
                      {m.ai_ui_read_only()}
                    </Badge>
                  {/if}
                </div>
              </div>

              <p class="text-[11px] text-muted-foreground leading-normal">
                {tool.description}
              </p>

              {#if tool.required_permission}
                <div class="text-[9.5px] text-muted-foreground/80 flex items-center gap-1 pt-0.5">
                  <Lock size={10} />
                  {m.ai_ui_required_permission()}: <code class="font-mono text-foreground/80">{tool.required_permission}</code>
                </div>
              {/if}
            </div>
          {/each}
        </div>
      {/if}
    </CardContent>
  </Card>

</div>
