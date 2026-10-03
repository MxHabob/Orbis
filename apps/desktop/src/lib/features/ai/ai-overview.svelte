<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { api } from "$lib/api";
import { fade } from "svelte/transition";
import { ArrowRight, Bolt, ChartBar, CheckCircle, ClockRotateRight, Cube, InfoCircle, Sparkles, UserBadgeCheck } from "@orbis/icons";
import { Button } from "@orbis/ui/button";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
  CardFooter,
  CardAction,
} from "@orbis/ui/card";
import { localizeHref } from "$lib/paraglide/runtime";
import AiPerformanceTrendCard from "$lib/features/ai/components/overview/ai-performance-trend-card.svelte";

let { orgId }: { orgId: string } = $props();

// ── State ───────────────────────────────────────────────────
let healthStatus = $state<"healthy" | "degraded" | "offline">("healthy");
let sessionCount = $state(0);
let actionCount = $state(0);
let autopilotMode = $state("تلقائي");
let connectedAppsCount = $state(0);
let connectedIntegrations = $state<any[]>([]);
let activeSessions = $state<any[]>([]);
let recentLogs = $state<any[]>([]);
let isLoading = $state(true);

onMount(async () => {
  try {
    const [integrations, taskRes, diagnostics, sessions, settings] = await Promise.all([
      api.listIntegrations().catch(() => []),
      api.autopilotListTasks({ limit: 10 }).catch(() => ({ total_count: 0, items: [] })),
      api.autopilotGetDiagnostics().catch(() => null),
      api.aiLoadSessions(orgId).catch(() => []),
      api.getSettings().catch(() => null),
    ]);

    healthStatus =
      diagnostics?.is_healthy === false ? "degraded" : "healthy";
    activeSessions = Array.isArray(sessions) ? sessions : [];
    sessionCount = activeSessions.length;

    connectedIntegrations = Array.isArray(integrations) ? integrations : [];
    connectedAppsCount = connectedIntegrations.length;

    const taskList = Array.isArray(taskRes?.items) ? taskRes.items : [];
    actionCount = taskRes?.total_count ?? taskList.length;
    recentLogs = taskList.slice(0, 5);

    if (settings?.autopilot) {
      const mode = settings.autopilot.mode;
      autopilotMode =
        mode === "automatic" ? "تلقائي" : mode === "manual" ? "يدوي" : "مقترح";
    }
  } catch (e) {
    console.warn("[AI Center] Overview load:", e);
  } finally {
    isLoading = false;
  }
});

const healthConfig = {
  healthy: {
    color: "text-foreground font-semibold",
    bg: "bg-muted border border-border",
    label: "متصل ويعمل بكفاءة",
    icon: CheckCircle,
  },
  degraded: {
    color: "text-foreground font-semibold",
    bg: "bg-muted border border-border",
    label: "استجابة بطيئة",
    icon: InfoCircle,
  },
  offline: {
    color: "text-muted-foreground",
    bg: "bg-muted border border-border",
    label: "غير متصل",
    icon: ClockRotateRight,
  },
};

const healthInfo = $derived(healthConfig[healthStatus]);
</script>

<div class="h-full w-full overflow-y-auto scrollbar-hide p-6 flex flex-col gap-6 font-sans">
  {#if isLoading}
    <div class="h-64 flex items-center justify-center">
      <div class="flex items-center gap-3 text-muted-foreground text-sm font-medium">
        <Sparkles size={18} class="animate-spin text-primary" />
        <span>جاري تحميل بيانات مركز الذكاء الاصطناعي...</span>
      </div>
    </div>
  {:else}
    <div class="flex flex-col gap-6" in:fade={{ duration: 150 }}>
      
      <!-- 1. Full-Width SVG AI Chart Trend Chart at top -->
      <AiPerformanceTrendCard {orgId} {isLoading} />

      <!-- 2. Middle Section: AI Agents Grid & System Autopilot Health Card -->
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
        
        <!-- Agents List Card (2 cols) -->
        <Card class="lg:col-span-2 flex flex-col justify-between">
          <CardHeader>
            <div class="flex items-center gap-2">
              <Sparkles size={14} class="text-primary" />
              <CardTitle>
                {m.ai_center_overview_agents_title()}
              </CardTitle>
            </div>
            <CardAction>
              <span class="text-[10px] font-mono px-2 py-0.5 rounded bg-muted text-muted-foreground font-semibold">
                {activeSessions.length} جلسة مفعّلة
              </span>
            </CardAction>
          </CardHeader>

          <CardContent class="p-6">
            {#if activeSessions.length > 0}
              <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
                {#each activeSessions as session}
                  <div class="border border-border rounded-lg p-3.5 flex items-start justify-between bg-muted/20 hover:border-foreground/20 transition-colors">
                    <div class="flex items-start gap-3">
                      <div class="w-8 h-8 rounded-lg border border-border flex items-center justify-center shrink-0 bg-primary/10 text-primary">
                        <Sparkles size={16} />
                      </div>
                      <div class="text-start">
                        <h3 class="text-xs font-bold text-foreground mb-0.5">{session.title || session.id}</h3>
                        <p class="text-[10px] text-muted-foreground">{session.agent_id || "وكيل ذكي"}</p>
                      </div>
                    </div>
                    <span class="px-2 py-0.5 text-[9px] font-bold rounded-full bg-primary/10 text-primary border border-primary/20 shrink-0">
                      نشط
                    </span>
                  </div>
                {/each}
              </div>
            {:else}
              <div class="border border-border rounded-lg p-6 flex flex-col items-center text-center bg-muted/10 space-y-1.5">
                <Sparkles size={24} class="text-muted-foreground" />
                <p class="text-xs font-bold text-foreground">لا توجد جلسات وكلاء نشطة حالياً</p>
                <p class="text-[11px] text-muted-foreground">يتم إنشاء وتفعيل جلسات الوكلاء تلقائياً عند تنفيذ المهام والاستفسارات.</p>
              </div>
            {/if}
          </CardContent>

          <CardFooter class="justify-between text-[10px] text-muted-foreground font-medium">
            <span>محركات المعالجة المحلية والسحابية تعمل وفقاً للسياسات المعتمدة.</span>
            <a href={localizeHref(`/${orgId}/ai-center/settings`)} class="text-primary hover:underline flex items-center gap-1 font-semibold">
              <span>تكوين النماذج</span>
              <ArrowRight size={10} class="rtl:rotate-180" />
            </a>
          </CardFooter>
        </Card>

        <!-- System & Autopilot Status Card (1 col) -->
        <Card class="flex flex-col justify-between">
          <CardHeader>
            <div class="flex items-center gap-2">
              <Bolt size={14} class="text-primary" />
              <CardTitle>
                {m.ai_center_overview_health()}
              </CardTitle>
            </div>
            <CardAction>
              <span class="text-[10px] font-bold px-2 py-0.5 rounded {healthInfo.bg} {healthInfo.color}">
                {healthInfo.label}
              </span>
            </CardAction>
          </CardHeader>

          <CardContent class="p-6 space-y-3">
            <div class="flex items-center justify-between p-2.5 rounded-md border border-border bg-muted/20">
              <div class="flex items-center gap-2">
                <ChartBar size={14} class="text-muted-foreground" />
                <span class="text-xs text-foreground font-medium">{m.ai_center_overview_sessions()}</span>
              </div>
              <span class="text-sm font-mono font-bold text-foreground">{sessionCount}</span>
            </div>

            <div class="flex items-center justify-between p-2.5 rounded-md border border-border bg-muted/20">
              <div class="flex items-center gap-2">
                <Bolt size={14} class="text-muted-foreground" />
                <span class="text-xs text-foreground font-medium">{m.ai_center_overview_autopilot()}</span>
              </div>
              <span class="text-xs font-bold text-foreground bg-muted px-2 py-0.5 rounded border border-border">{autopilotMode}</span>
            </div>

            <div class="flex items-center justify-between p-2.5 rounded-md border border-border bg-muted/20">
              <div class="flex items-center gap-2">
                <Cube size={14} class="text-muted-foreground" />
                <span class="text-xs text-foreground font-medium">{m.ai_center_overview_integrations()}</span>
              </div>
              <span class="text-sm font-mono font-bold text-foreground">{connectedAppsCount}</span>
            </div>
          </CardContent>

          <CardFooter>
            <a href={localizeHref(`/${orgId}/ai-center/permissions`)} class="w-full">
              <Button size="sm" variant="outline" class="w-full text-xs font-semibold justify-between">
                <span>إدارة مصفوفة الصلاحيات</span>
                <UserBadgeCheck size={12} />
              </Button>
            </a>
          </CardFooter>
        </Card>

      </div>

      <!-- 3. Bottom Section: Recent Action Logs Timeline & Connected Integrations Quick Status -->
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
        
        <!-- Recent Action Logs Timeline (2 cols) -->
        <Card class="lg:col-span-2">
          <CardHeader>
            <div class="flex items-center gap-2">
              <ClockRotateRight size={14} class="text-primary" />
              <CardTitle>
                {m.ai_center_overview_recent_activity()}
              </CardTitle>
            </div>
            <CardAction>
              <a href={localizeHref(`/${orgId}/ai-center/logs`)} class="text-xs text-primary hover:underline flex items-center gap-1 font-semibold">
                <span>عرض الكل</span>
                <ArrowRight size={10} class="rtl:rotate-180" />
              </a>
            </CardAction>
          </CardHeader>

          <CardContent class="p-6">
            {#if recentLogs.length > 0}
              <div class="divide-y divide-border border border-border rounded-md overflow-hidden bg-muted/10">
                {#each recentLogs as log}
                  <div class="px-4 py-3 flex items-center justify-between hover:bg-muted/20 transition-colors">
                    <div class="flex items-center gap-3">
                      <span class="w-2 h-2 rounded-full {log.status === 'completed' ? 'bg-primary' : log.status === 'failed' ? 'bg-destructive' : 'bg-muted-foreground/60'} shrink-0"></span>
                      <div class="text-start">
                        <code class="text-[11px] font-mono font-semibold text-foreground bg-muted px-1.5 py-0.5 rounded">{log.task_type || log.action_code || log.action || "مهمة أتمتة"}</code>
                        <p class="text-[11px] text-muted-foreground mt-1 truncate max-w-sm">{log.last_error || log.reason || log.reasoning || (log.status === 'completed' ? 'تم تنفيذ المهمة بنجاح.' : 'مجدولة للتنفيذ القادم.')}</p>
                      </div>
                    </div>
                    <span class="text-[10px] text-muted-foreground font-mono shrink-0">{log.scheduled_at || log.executed_at || "—"}</span>
                  </div>
                {/each}
              </div>
            {:else}
              <div class="border border-border rounded-md p-6 flex flex-col items-center text-center bg-muted/10">
                <ClockRotateRight size={20} class="text-muted-foreground mb-2" />
                <p class="text-xs font-semibold text-foreground">لا توجد سجلات تنفيذية جديدة</p>
                <p class="text-[11px] text-muted-foreground mt-0.5">ستظهر السجلات هنا فور قيام الوكلاء بتنفيذ مهام أو توصيات تلقائية.</p>
              </div>
            {/if}
          </CardContent>
        </Card>

        <!-- Connected Providers Quick List (1 col) -->
        <Card class="flex flex-col justify-between">
          <CardHeader>
            <div class="flex items-center gap-2">
              <Cube size={14} class="text-primary" />
              <CardTitle>
                {m.ai_center_overview_connected_providers()}
              </CardTitle>
            </div>
            <CardAction>
              <a href={localizeHref(`/${orgId}/ai-center/integrations`)} class="text-xs text-primary hover:underline font-semibold">
                المتجر
              </a>
            </CardAction>
          </CardHeader>

          <CardContent class="p-6">
            {#if connectedIntegrations.length > 0}
              <div class="space-y-2.5">
                {#each connectedIntegrations as item}
                  <div class="p-3 border border-border rounded-md bg-muted/20 flex items-center justify-between">
                    <div class="flex items-center gap-2.5 text-start">
                      <Cube size={15} class="text-primary" />
                      <div>
                        <h4 class="text-xs font-bold text-foreground">{item.provider_name || item.name || item.id}</h4>
                        <p class="text-[10px] text-muted-foreground">{item.category || item.provider || "خدمة متصلة"}</p>
                      </div>
                    </div>
                    <span class="w-2 h-2 rounded-full bg-primary" title="متصل"></span>
                  </div>
                {/each}
              </div>
            {:else}
              <div class="border border-border rounded-md p-6 flex flex-col items-center text-center bg-muted/10 space-y-1.5">
                <Cube size={20} class="text-muted-foreground" />
                <p class="text-xs font-bold text-foreground">لا توجد خدمات مربوطة حالياً</p>
                <p class="text-[11px] text-muted-foreground">يمكنك ربط المزودين من خلال متجر التكاملات.</p>
              </div>
            {/if}
          </CardContent>

          <CardFooter>
            <a href={localizeHref(`/${orgId}/ai-center/settings`)} class="w-full">
              <Button size="sm" variant="ghost" class="w-full text-xs font-semibold justify-center">
                <span>إدارة تكوين المزودات والنماذج</span>
              </Button>
            </a>
          </CardFooter>
        </Card>

      </div>

    </div>
  {/if}
</div>
