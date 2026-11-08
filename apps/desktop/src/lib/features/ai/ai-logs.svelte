<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { Card, CardContent, CardFooter } from "@orbis/ui/card";
import { Badge } from "@orbis/ui/badge";
import { Button } from "@orbis/ui/button";
import SearchField from "$lib/shared/components/search-field.svelte";
import AiEmptyState from "$lib/shared/components/ai-empty-state.svelte";
import { Bolt, InfoCircle, Play, Sparkles } from "@orbis/icons";
import { toast } from "@orbis/ui";
import { formatDate } from "$lib/core/i18n";

let { orgId }: { orgId: string } = $props();

// ── State ─────────────────────────────────────────────────────────────
let isLoading = $state(true);
let isExecuting = $state(false);
let searchQuery = $state("");
let activeCategory = $state("all");

let tasks = $state<any[]>([]);
let agentSessions = $state<any[]>([]);

async function loadData() {
  tasks = [];
  agentSessions = [];
  isLoading = false;
  isExecuting = false;
}

onMount(() => {
  loadData();
});

async function handleRunTick() {
  isExecuting = true;
  await loadData();
  toast.success(m.ai_ui_check_cycle_logs_updated());
  isExecuting = false;
}

// ── Unified Log Items ─────────────────────────────────────────────────
interface LogItem {
  id: string;
  source: "autopilot" | "agent";
  title: string;
  description: string;
  status: "completed" | "pending" | "failed" | "suggested" | "executing";
  timestamp: string;
  target?: string;
  attempts?: number;
  lastError?: string;
}

const allLogs = $derived.by<LogItem[]>(() => {
  const list: LogItem[] = [];

  for (const t of tasks) {
    list.push({
      id: t.id || `task-${Math.random()}`,
      source: "autopilot",
      title: t.task_type || m.ai_ui_automation_task(),
      description:
        t.reason || t.task_type || m.ai_ui_periodic_autopilot_operation(),
      status: t.status || "pending",
      timestamp: t.scheduled_at || t.created_at || new Date().toISOString(),
      target: t.target_id
        ? `${t.target_type || ""} (${t.target_id.slice(0, 8)})`
        : undefined,
      attempts: t.attempts,
      lastError: t.last_error,
    });
  }

  for (const s of agentSessions) {
    list.push({
      id: s.id || `session-${Math.random()}`,
      source: "agent",
      title: s.title || m.ai_ui_smart_processing_session(),
      description: m.ai_ui_agent_description({
        agent: s.agent_id || m.ai_ui_ai_assistant(),
      }),
      status: s.status === "active" ? "completed" : "pending",
      timestamp: s.created_at || s.updated_at || new Date().toISOString(),
      target: s.acting_as_role || undefined,
    });
  }

  return list.sort((a, b) => b.timestamp.localeCompare(a.timestamp));
});

// Categories matching design tokens
const categories = [
  { id: "all", label: m.ai_ui_all() },
  { id: "autopilot", label: m.ai_ui_autopilot() },
  { id: "agent", label: m.ai_ui_agents() },
  { id: "completed", label: m.ai_ui_successful() },
  { id: "pending", label: m.ai_ui_scheduled() },
  { id: "failed", label: m.ai_ui_failed() },
];

const filteredLogs = $derived.by(() => {
  return allLogs.filter((item) => {
    // Category filter
    if (activeCategory === "autopilot" && item.source !== "autopilot")
      return false;
    if (activeCategory === "agent" && item.source !== "agent") return false;
    if (activeCategory === "completed" && item.status !== "completed")
      return false;
    if (
      activeCategory === "pending" &&
      item.status !== "pending" &&
      item.status !== "executing"
    )
      return false;
    if (activeCategory === "failed" && item.status !== "failed") return false;

    // Search query
    if (searchQuery.trim()) {
      const q = searchQuery.toLowerCase().trim();
      const matchTitle = item.title.toLowerCase().includes(q);
      const matchDesc = item.description.toLowerCase().includes(q);
      const matchTarget = item.target?.toLowerCase().includes(q);
      if (!matchTitle && !matchDesc && !matchTarget) return false;
    }

    return true;
  });
});

function formatTime(dtStr: string): string {
  return formatDate(dtStr, "datetime") || dtStr;
}
</script>

<div class="h-full w-full overflow-y-auto font-sans p-6 scrollbar-hide space-y-6 text-start">
  <!-- Top Action & Search Bar (Headerless) -->
  <div class="flex flex-col sm:flex-row items-center justify-between gap-4">
    <!-- Centered / Flexible Search Bar -->
    <div class="w-full sm:max-w-md">
      <SearchField
        bind:value={searchQuery}
        placeholder={m.ai_center_logs_search_placeholder()}
        class="w-full"
      />
    </div>

    <!-- Category Pills Bar -->
    <div class="flex flex-wrap gap-1.5 items-center justify-center sm:justify-start">
      {#each categories as cat (cat.id)}
        <Button
          size="sm"
          variant={activeCategory === cat.id ? "default" : "outline"}
          onclick={() => (activeCategory = cat.id)}
          class="text-xs cursor-pointer"
        >
          {cat.label}
        </Button>
      {/each}
    </div>

    <!-- Quick Run Tick Action -->
    <div class="shrink-0">
      <Button
        size="sm"
        onclick={handleRunTick}
        disabled={isExecuting}
        class="h-8.5 px-3.5 text-xs font-semibold gap-1.5 cursor-pointer shadow-xs rounded-lg"
      >
        <Play size={12} fill="currentColor" />
        <span>{m.ai_center_logs_btn_run_tick()}</span>
      </Button>
    </div>
  </div>

  <!-- Cards Grid or AI Empty State -->
  <main>
    {#if filteredLogs.length > 0}
      <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
        {#each filteredLogs as log (log.id)}
          {@const isCompleted = log.status === "completed"}
          {@const isFailed = log.status === "failed"}
          {@const isPending = log.status === "pending" || log.status === "executing"}

          <Card class="h-full flex flex-col justify-between hover:border-foreground/30 transition-colors">
            <CardContent class="p-5 flex flex-col flex-1 text-start">
              <!-- Top: Icon + UI Status Badges -->
              <div class="flex items-center justify-between gap-3 mb-4">
                <div class="w-9 h-9 rounded-lg border border-border flex items-center justify-center shrink-0 {log.source === 'autopilot' ? 'bg-primary/10 text-primary' : 'bg-brand-lime/10 text-brand-lime'}">
                  {#if log.source === "autopilot"}
                    <Bolt size={18} />
                  {:else}
                    <Sparkles size={18} />
                  {/if}
                </div>

                <div class="flex gap-1.5 flex-wrap justify-end">
                  <Badge
                    variant={isCompleted ? 'lime' : isFailed ? 'destructive' : isPending ? 'secondary' : 'outline'}
                    class="text-[10px] font-semibold px-2 py-0.5"
                  >
                    {#if isCompleted}
                      {m.ai_ui_successful()}
                    {:else if isFailed}
                      {m.ai_ui_failed()}
                    {:else}
                      {m.ai_ui_scheduled()}
                    {/if}
                  </Badge>

                  {#if log.attempts && log.attempts > 1}
                    <Badge variant="outline" class="text-[10px] font-mono px-1.5 py-0.5 text-muted-foreground">
                      {log.attempts}x
                    </Badge>
                  {/if}
                </div>
              </div>

              <!-- Title / Code -->
              <h3 class="text-sm font-semibold text-foreground mb-1.5 font-mono truncate" title={log.title}>
                {log.title}
              </h3>

              <!-- Description -->
              <p class="text-xs text-muted-foreground leading-relaxed flex-1 line-clamp-2" title={log.description}>
                {log.description}
              </p>

              {#if log.lastError}
                <div class="mt-2 p-2 rounded-md bg-destructive/10 border border-destructive/20 text-[10px] text-destructive flex items-start gap-1.5 font-medium">
                  <InfoCircle size={12} class="shrink-0 mt-0.5" />
                  <span class="line-clamp-2">{log.lastError}</span>
                </div>
              {/if}
            </CardContent>

            <!-- Footer: Source & Timestamp Tag -->
            <CardFooter class="py-2.5 px-5 justify-between text-[10px] text-muted-foreground font-mono">
              <Badge variant="outline" class="text-[10px] font-medium text-muted-foreground border-border bg-muted/40">
                {log.source === "autopilot" ? m.ai_ui_autopilot() : m.ai_ui_smart_agent()}
              </Badge>

              <span>
                {formatTime(log.timestamp)}
              </span>
            </CardFooter>
          </Card>
        {/each}
      </div>
    {:else}
      <AiEmptyState
        text={searchQuery
          ? m.ai_ui_logs_search_empty({ query: searchQuery })
          : m.ai_ui_logs_category_empty()}
        actionLabel={searchQuery ? m.ai_ui_reset_filters_search() : m.ai_ui_run_automation_scan()}
        onaction={() => {
          if (searchQuery) {
            searchQuery = "";
            activeCategory = "all";
          } else {
            handleRunTick();
          }
        }}
        hat="pilot"
      />
    {/if}
  </main>
</div>
