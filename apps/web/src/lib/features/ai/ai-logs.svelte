<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { createQuery } from "@tanstack/svelte-query";
import { Card, CardContent, CardFooter } from "@orbis/ui/card";
import { Badge } from "@orbis/ui/badge";
import { Button } from "@orbis/ui/button";
import SearchField from "$lib/shared/components/search-field.svelte";
import AiEmptyState from "$lib/shared/components/ai-empty-state.svelte";
import { Bolt, InfoCircle, Sparkles } from "@orbis/icons";
import { api } from "$lib/api";
import { formatDate } from "$lib/core/i18n";
import { createAutopilotTasksQuery } from "./queries";

let { orgId }: { orgId: string } = $props();
let searchQuery = $state("");
let activeCategory = $state("all");

const tasks = createAutopilotTasksQuery(() => orgId, () => ({ limit: 50 }));
const sessions = createQuery(() => ({
  queryKey: ["ai", "stored-sessions", orgId],
  queryFn: () => api.aiLoadSessions(orgId),
  enabled: Boolean(orgId),
}));
const isLoading = $derived(tasks.isLoading || sessions.isLoading);
const isRefreshing = $derived(tasks.isFetching || sessions.isFetching);
const hasError = $derived(tasks.isError || sessions.isError);

function refreshLogs() {
  return Promise.all([tasks.refetch(), sessions.refetch()]);
}

interface LogItem {
  id: string;
  source: "autopilot" | "agent";
  title: string;
  description: string;
  status?: string;
  timestamp: number | null;
  target?: string;
  attempts?: number;
  lastError?: string | null;
}

function firstDate(...values: (string | number | null | undefined)[]) {
  for (const value of values) {
    if (value == null || value === "" || value === 0) continue;
    const timestamp = new Date(value).getTime();
    if (Number.isFinite(timestamp)) return timestamp;
  }
  return null;
}

const categories = $derived([
  { id: "all", label: m.ai_ui_all() },
  { id: "autopilot", label: m.ai_ui_autopilot() },
  { id: "agent", label: m.ai_ui_agents() },
  { id: "completed", label: m.ai_ui_successful() },
  { id: "pending", label: m.ai_ui_scheduled() },
  { id: "failed", label: m.ai_ui_failed() },
]);

const statusLabels: Record<string, () => string> = {
  completed: m.ai_ui_successful,
  pending: m.ai_ui_scheduled,
  executing: m.common_promo_job_status_executing,
  suggested: m.ui_autopilot_suggested,
  failed: m.ai_ui_failed,
  skipped: m.ai_log_skipped,
  cancelled: m.ai_log_cancelled,
};

const filteredLogs = $derived.by(() => {
  const logs: LogItem[] = [
    ...(tasks.data ?? []).map((task): LogItem => ({
      id: "task-" + task.id,
      source: "autopilot",
      title: task.task_type,
      description: task.reason ?? "",
      status: task.status,
      timestamp: firstDate(task.executed_at, task.scheduled_at, task.created_at),
      target: task.target_id ?? undefined,
      attempts: task.attempts,
      lastError: task.error_log,
    })),
    ...(sessions.data ?? []).map((session): LogItem => ({
      id: "session-" + session.id,
      source: "agent",
      title: session.title || m.ai_ui_smart_processing_session(),
      description: "",
      timestamp: firstDate(session.updated_at, session.created_at),
    })),
  ];
  const query = searchQuery.trim().toLocaleLowerCase();

  return logs.filter((log) => {
    if (activeCategory !== "all" && log.source !== activeCategory && log.status !== activeCategory) return false;
    return !query || [log.title, log.description, log.target]
      .some((value) => value?.toLocaleLowerCase().includes(query));
  }).sort((a, b) => (b.timestamp ?? -Infinity) - (a.timestamp ?? -Infinity));
});
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
        onclick={refreshLogs}
        disabled={isRefreshing}
        class="h-8.5 px-3.5 text-xs font-semibold gap-1.5 cursor-pointer shadow-xs rounded-lg"
      >
        <span>{m.common_control_refresh()}</span>
      </Button>
    </div>
  </div>

  <!-- Cards Grid or AI Empty State -->
  <main>
    {#if hasError}
      <p class="mb-4 rounded-lg border border-border p-3 text-sm text-destructive" role="alert">{m.common_loading_error()}</p>
    {/if}
    {#if isLoading && filteredLogs.length === 0}
      <p class="text-sm text-muted-foreground" role="status">{m.common_loading()}</p>
    {:else if filteredLogs.length > 0}
      <div class="grid grid-cols-[repeat(auto-fit,minmax(min(100%,13rem),1fr))] gap-4">
        {#each filteredLogs as log (log.id)}
          {@const isCompleted = log.status === "completed"}
          {@const isFailed = log.status === "failed"}
          {@const statusLabel = log.status && Object.hasOwn(statusLabels, log.status) ? statusLabels[log.status]() : log.status}

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
                  {#if statusLabel}
                    <Badge variant={isCompleted ? "lime" : isFailed ? "destructive" : "secondary"}>
                      {statusLabel}
                    </Badge>
                  {/if}

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
                {log.timestamp === null ? m.ai_log_date_unavailable() : formatDate(new Date(log.timestamp), "datetime")}
              </span>
            </CardFooter>
          </Card>
        {/each}
      </div>
    {:else if !hasError}
      <AiEmptyState
        text={searchQuery
          ? m.ai_ui_logs_search_empty({ query: searchQuery })
          : m.ai_ui_logs_category_empty()}
        actionLabel={searchQuery || activeCategory !== "all" ? m.ai_ui_reset_filters_search() : undefined}
        onaction={() => {
          searchQuery = "";
          activeCategory = "all";
        }}
        hat="pilot"
      />
    {/if}
  </main>
</div>
