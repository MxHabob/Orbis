<!--
@component AiCenterOverview
Main dashboard page for AI Center.
Shows: Health Status, Active Sessions, Autopilot, Quick Stats, Agents, Recent Tasks.
Data comes from the backend via SDK — currently uses placeholder structure.
-->
<script lang="ts">
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
} from "lucide-svelte";

let { data }: { data: { orgId: string } } = $props();

// These will be replaced with real SDK queries
const agents = [
  { name: "Academic Agent", icon: Brain, domain: "academic, scheduling", color: "text-blue-500" },
  { name: "Finance Agent", icon: Gauge, domain: "finance, fees", color: "text-emerald-500" },
  { name: "Exam Agent", icon: Layers, domain: "exams, grades", color: "text-purple-500" },
  { name: "Admin Agent", icon: Bot, domain: "admin, reports", color: "text-orange-500" },
];
</script>

<div class="h-full overflow-y-auto">
  <div class="px-8 py-6 max-w-5xl space-y-6">
    <!-- Header -->
    <div>
      <h1 class="text-xl font-semibold text-foreground mb-1">AI Center</h1>
      <p class="text-xs text-muted-foreground">
        Monitor and manage AI agents, integrations, and automated workflows.
      </p>
    </div>

    <!-- Stats Grid -->
    <div class="grid grid-cols-2 lg:grid-cols-4 gap-3">
      <!-- Health -->
      <div class="border border-border/60 rounded-lg p-4">
        <div class="flex items-center gap-2 mb-3">
          <div class="w-8 h-8 rounded-md bg-emerald-50 dark:bg-emerald-950/30 flex items-center justify-center">
            <CheckCircle size={16} class="text-emerald-500" />
          </div>
          <span class="text-[10px] font-medium text-muted-foreground uppercase tracking-wider">System Health</span>
        </div>
        <p class="text-lg font-semibold text-emerald-600 dark:text-emerald-400">Healthy</p>
        <p class="text-[10px] text-muted-foreground mt-0.5">All services running</p>
      </div>

      <!-- Active Sessions -->
      <div class="border border-border/60 rounded-lg p-4">
        <div class="flex items-center gap-2 mb-3">
          <div class="w-8 h-8 rounded-md bg-blue-50 dark:bg-blue-950/30 flex items-center justify-center">
            <Activity size={16} class="text-blue-500" />
          </div>
          <span class="text-[10px] font-medium text-muted-foreground uppercase tracking-wider">Sessions</span>
        </div>
        <p class="text-lg font-semibold text-foreground">0</p>
        <p class="text-[10px] text-muted-foreground mt-0.5">Active AI sessions</p>
      </div>

      <!-- Autopilot -->
      <div class="border border-border/60 rounded-lg p-4">
        <div class="flex items-center gap-2 mb-3">
          <div class="w-8 h-8 rounded-md bg-purple-50 dark:bg-purple-950/30 flex items-center justify-center">
            <Zap size={16} class="text-purple-500" />
          </div>
          <span class="text-[10px] font-medium text-muted-foreground uppercase tracking-wider">Autopilot</span>
        </div>
        <p class="text-lg font-semibold text-foreground">Manual</p>
        <p class="text-[10px] text-muted-foreground mt-0.5">Awaiting configuration</p>
      </div>

      <!-- Tools -->
      <div class="border border-border/60 rounded-lg p-4">
        <div class="flex items-center gap-2 mb-3">
          <div class="w-8 h-8 rounded-md bg-orange-50 dark:bg-orange-950/30 flex items-center justify-center">
            <Cpu size={16} class="text-orange-500" />
          </div>
          <span class="text-[10px] font-medium text-muted-foreground uppercase tracking-wider">Tools</span>
        </div>
        <p class="text-lg font-semibold text-foreground">28+</p>
        <p class="text-[10px] text-muted-foreground mt-0.5">Registered actions</p>
      </div>
    </div>

    <!-- Agents Section -->
    <div>
      <h2 class="text-sm font-medium text-foreground mb-3">AI Agents</h2>
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
        {#each agents as agent}
          <div class="border border-border/60 rounded-lg p-4 flex items-start gap-3.5">
            <div class="w-10 h-10 rounded-lg bg-muted/50 flex items-center justify-center shrink-0">
              <agent.icon size={20} class={agent.color} />
            </div>
            <div class="min-w-0">
              <h3 class="text-xs font-medium text-foreground mb-0.5">{agent.name}</h3>
              <p class="text-[10px] text-muted-foreground">Domains: {agent.domain}</p>
            </div>
          </div>
        {/each}
      </div>
    </div>

    <!-- Recent Activity -->
    <div>
      <h2 class="text-sm font-medium text-foreground mb-3">Recent Activity</h2>
      <div class="border border-border/60 rounded-lg p-6">
        <div class="flex flex-col items-center justify-center py-6">
          <div class="w-10 h-10 rounded-full bg-muted flex items-center justify-center mb-3">
            <Clock size={18} class="text-muted-foreground" />
          </div>
          <p class="text-xs text-muted-foreground text-center">
            No recent AI activity. Connect an AI provider to get started.
          </p>
        </div>
      </div>
    </div>
  </div>
</div>
