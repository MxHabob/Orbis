<!--
@component AiCenterOverview
Main dashboard for AI Center. Shows live stats via SDK.
Wired to: commands.aiCenter.checkHealth / listActionLogs / listAutopilotTasks / listConnectedIntegrations
-->
<script lang="ts">
import { onMount } from "svelte";
import { commands } from "$lib/core/ipc";
import {
  Activity,
  Brain,
  Cpu,
  Gauge,
  Layers,
  Zap,
  CheckCircle,
  AlertTriangle,
  Clock,
  Bot,
  Plug,
} from "lucide-svelte";

let { data }: { data: { orgId: string } } = $props();

// ── State ───────────────────────────────────────────────────
let healthStatus = $state<"healthy" | "degraded" | "offline">("offline");
let sessionCount = $state(0);
let actionCount = $state(0);
let autopilotMode = $state("Manual");
let connectedApps = $state(0);
let recentLogs = $state<any[]>([]);
let isLoading = $state(true);

// ── Load Data ───────────────────────────────────────────────
onMount(async () => {
  try {
    const [health, logs, tasks, integrations, sessions] = await Promise.all([
      commands.aiCenter.checkHealth().catch(() => null),
      commands.aiCenter.listActionLogs(data.orgId).catch(() => []),
      commands.aiCenter.listAutopilotTasks(data.orgId).catch(() => []),
      commands.aiCenter.listConnectedIntegrations().catch(() => []),
      commands.agent.listSessions(data.orgId).catch(() => []),
    ]);

    if (health) healthStatus = "healthy";
    sessionCount = (sessions as any[])?.length ?? 0;
    actionCount = (logs as any[])?.length ?? 0;
    connectedApps = (integrations as any[])?.length ?? 0;
    recentLogs = ((logs as any[]) ?? []).slice(0, 5);

    const config = await commands.aiCenter.getAutopilotConfig(data.orgId).catch(() => null);
    if (config && (config as any).enabled) {
      autopilotMode = (config as any).execution_mode === "automatic" ? "Automatic" : "Suggested";
    }
  } catch (e) {
    console.warn("[AI Center] Overview load:", e);
  } finally {
    isLoading = false;
  }
});

const agents = [
  { name: "Academic Agent", icon: Brain, domain: "academic, scheduling", color: "text-blue-500" },
  { name: "Finance Agent", icon: Gauge, domain: "finance, fees", color: "text-emerald-500" },
  { name: "Exam Agent", icon: Layers, domain: "exams, grades", color: "text-purple-500" },
  { name: "Admin Agent", icon: Bot, domain: "admin, reports", color: "text-orange-500" },
];

const healthConfig = {
  healthy:  { color: "text-emerald-600 dark:text-emerald-400", bg: "bg-emerald-50 dark:bg-emerald-950/30", label: "Healthy", icon: CheckCircle },
  degraded: { color: "text-amber-600 dark:text-amber-400",   bg: "bg-amber-50 dark:bg-amber-950/30",   label: "Degraded", icon: AlertTriangle },
  offline:  { color: "text-muted-foreground",                bg: "bg-muted",                             label: "Offline",  icon: Clock },
};

const healthInfo = $derived(healthConfig[healthStatus]);
</script>

<div class="h-full overflow-y-auto">
  <div class="px-8 py-6 max-w-5xl space-y-5">
    <div>
      <h1 class="text-lg font-semibold text-foreground mb-1">AI Center</h1>
      <p class="text-[11px] text-muted-foreground">Monitor and manage AI agents, integrations, and automated workflows.</p>
    </div>

    <!-- Stats Grid -->
    <div class="grid grid-cols-2 lg:grid-cols-4 gap-3">
      <!-- Health -->
      <div class="border border-border/60 rounded-lg p-4">
        <div class="flex items-center gap-2 mb-2.5">
          <div class="w-8 h-8 rounded-md {healthInfo.bg} flex items-center justify-center">
            <healthInfo.icon size={15} class={healthInfo.color} />
          </div>
          <span class="text-[10px] font-medium text-muted-foreground uppercase tracking-wider">Health</span>
        </div>
        <p class="text-lg font-semibold {healthInfo.color}">{healthInfo.label}</p>
      </div>

      <!-- Sessions -->
      <div class="border border-border/60 rounded-lg p-4">
        <div class="flex items-center gap-2 mb-2.5">
          <div class="w-8 h-8 rounded-md bg-blue-50 dark:bg-blue-950/30 flex items-center justify-center">
            <Activity size={15} class="text-blue-500" />
          </div>
          <span class="text-[10px] font-medium text-muted-foreground uppercase tracking-wider">Sessions</span>
        </div>
        <p class="text-lg font-semibold text-foreground">{sessionCount}</p>
      </div>

      <!-- Autopilot -->
      <div class="border border-border/60 rounded-lg p-4">
        <div class="flex items-center gap-2 mb-2.5">
          <div class="w-8 h-8 rounded-md bg-purple-50 dark:bg-purple-950/30 flex items-center justify-center">
            <Zap size={15} class="text-purple-500" />
          </div>
          <span class="text-[10px] font-medium text-muted-foreground uppercase tracking-wider">Autopilot</span>
        </div>
        <p class="text-lg font-semibold text-foreground">{autopilotMode}</p>
      </div>

      <!-- Connected Apps -->
      <div class="border border-border/60 rounded-lg p-4">
        <div class="flex items-center gap-2 mb-2.5">
          <div class="w-8 h-8 rounded-md bg-orange-50 dark:bg-orange-950/30 flex items-center justify-center">
            <Plug size={15} class="text-orange-500" />
          </div>
          <span class="text-[10px] font-medium text-muted-foreground uppercase tracking-wider">Integrations</span>
        </div>
        <p class="text-lg font-semibold text-foreground">{connectedApps}</p>
      </div>
    </div>

    <!-- Agents -->
    <div>
      <h2 class="text-sm font-medium text-foreground mb-2.5">AI Agents</h2>
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
        {#each agents as agent}
          <div class="border border-border/60 rounded-lg p-3.5 flex items-start gap-3">
            <div class="w-9 h-9 rounded-lg bg-muted/50 flex items-center justify-center shrink-0">
              <agent.icon size={18} class={agent.color} />
            </div>
            <div>
              <h3 class="text-xs font-medium text-foreground mb-0.5">{agent.name}</h3>
              <p class="text-[10px] text-muted-foreground">{agent.domain}</p>
            </div>
          </div>
        {/each}
      </div>
    </div>

    <!-- Recent Activity -->
    <div>
      <h2 class="text-sm font-medium text-foreground mb-2.5">Recent Activity</h2>
      {#if recentLogs.length > 0}
        <div class="border border-border/60 rounded-lg divide-y divide-border/30">
          {#each recentLogs as log}
            <div class="px-4 py-3 flex items-center justify-between">
              <div>
                <code class="text-[10px] font-mono text-foreground">{log.action_code}</code>
                <p class="text-[10px] text-muted-foreground mt-0.5 truncate max-w-xs">{log.reasoning}</p>
              </div>
              <span class="text-[9px] text-muted-foreground shrink-0">{log.executed_at}</span>
            </div>
          {/each}
        </div>
      {:else}
        <div class="border border-border/60 rounded-lg p-5 flex flex-col items-center">
          <Clock size={16} class="text-muted-foreground mb-1.5" />
          <p class="text-xs text-muted-foreground">No recent activity. Connect an AI provider to get started.</p>
        </div>
      {/if}
    </div>
  </div>
</div>
