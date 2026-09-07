<!--
@component AiCenterOverviewPage
Main dashboard for AI Center — unified background, responsive grid, zero div bloat.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { api, } from "$lib/api";
import { fade } from "svelte/transition";
import {
  Activity,
  Zap,
  CheckCircle,
  AlertTriangle,
  Clock,
  Bot,
  Plug,
  ArrowRight,
  ShieldAlert,
  Sparkles,
} from "@lucide/svelte";
import { Button } from "@orbis/ui/button";
import { localizeHref } from "$lib/paraglide/runtime";
import AiPerformanceTrendCard from "$lib/features/ai-center/components/overview/ai-performance-trend-card.svelte";

let { data }: { data: { orgId: string } } = $props();

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
    const [tasks, diagnostics, sessions, config, integrations] = await Promise.all([
      api.ai.autopilotGetTasks().catch(() => []),
      api.ai.autopilotGetDiagnostics().catch(() => null),
      api.ai.agentGetSessions().catch(() => []),
      api.ai.aiGetAutopilotConfig(data.orgId).catch(() => null),
      api.listIntegrations().catch(() => []),
    ]);

    healthStatus = (diagnostics as any)?.is_healthy === false ? "degraded" : "healthy";
    activeSessions = Array.isArray(sessions) ? sessions : [];
    sessionCount = activeSessions.length;

    connectedIntegrations = Array.isArray(integrations) ? integrations : [];
    connectedAppsCount = connectedIntegrations.length;

    const taskList = Array.isArray(tasks) ? (tasks as any[]) : [];
    actionCount = taskList.length;
    recentLogs = taskList.slice(0, 5);

    if (config) {
      const mode = (config as any).execution_mode || (config as any).autopilot_mode;
      autopilotMode = mode === "automatic" ? "تلقائي" : mode === "manual" ? "يدوي" : "مقترح";
    }
  } catch (e) {
    console.warn("[AI Center] Overview load:", e);
  } finally {
    isLoading = false;
  }
});

const healthConfig = {
  healthy:  { color: "text-foreground font-semibold", bg: "bg-muted border border-border", label: "متصل ويعمل بكفاءة", icon: CheckCircle },
  degraded: { color: "text-foreground font-semibold", bg: "bg-muted border border-border", label: "استجابة بطيئة", icon: AlertTriangle },
  offline:  { color: "text-muted-foreground", bg: "bg-muted border border-border", label: "غير متصل", icon: Clock },
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
      
      <!-- 1. Full-Width SVG AI Activity Trend Chart at top -->
      <AiPerformanceTrendCard orgId={data.orgId} {isLoading} />

      <!-- 2. Middle Section: AI Agents Grid & System Autopilot Health Card -->
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
        
        <!-- Agents List Card (2 cols) -->
        <div class="lg:col-span-2 bg-card border border-border rounded-xl p-5 shadow-sm flex flex-col justify-between space-y-4">
          <div class="flex items-center justify-between border-b border-border/40 pb-3">
            <div class="flex items-center gap-2">
              <Bot size={16} class="text-primary" />
              <h2 class="text-sm font-bold text-foreground">
                {m.ai_center_overview_agents_title ? m.ai_center_overview_agents_title() : "وكلاء الذكاء الاصطناعي النشطين"}
              </h2>
            </div>
            <span class="text-[10px] font-mono px-2 py-0.5 rounded bg-muted text-muted-foreground font-semibold">
              {activeSessions.length} جلسة مفعّلة
            </span>
          </div>

          {#if activeSessions.length > 0}
            <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
              {#each activeSessions as session}
                <div class="border border-border/60 rounded-lg p-3.5 flex items-start justify-between bg-background/40 hover:border-border transition-colors">
                  <div class="flex items-start gap-3">
                    <div class="w-9 h-9 rounded-lg border border-border flex items-center justify-center shrink-0 bg-primary/10 text-primary">
                      <Bot size={18} />
                    </div>
                    <div>
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
            <div class="border border-border/40 rounded-lg p-6 flex flex-col items-center text-center bg-background/20 space-y-1.5">
              <Bot size={24} class="text-muted-foreground" />
              <p class="text-xs font-bold text-foreground">لا توجد جلسات وكلاء نشطة حالياً</p>
              <p class="text-[11px] text-muted-foreground">يتم إنشاء وتفعيل جلسات الوكلاء تلقائياً عند تنفيذ المهام والاستفسارات.</p>
            </div>
          {/if}

          <div class="pt-2 border-t border-border/40 flex items-center justify-between text-[10px] text-muted-foreground font-medium">
            <span>محركات المعالجة المحلية والسحابية تعمل وفقاً للسياسات المعتمدة.</span>
            <a href={localizeHref(`/${data.orgId}/ai-center/settings`)} class="text-primary hover:underline flex items-center gap-1 font-semibold">
              <span>تكوين النماذج</span>
              <ArrowRight size={10} class="rtl:rotate-180" />
            </a>
          </div>
        </div>

        <!-- System & Autopilot Status Card (1 col) -->
        <div class="bg-card border border-border rounded-xl p-5 shadow-sm flex flex-col justify-between space-y-4">
          <div class="flex items-center justify-between border-b border-border/40 pb-3">
            <div class="flex items-center gap-2">
              <Zap size={16} class="text-primary" />
              <h2 class="text-sm font-bold text-foreground">
                {m.ai_center_overview_health ? m.ai_center_overview_health() : "حالة محرك الذكاء الاصطناعي"}
              </h2>
            </div>
            <span class="text-[10px] font-bold px-2 py-0.5 rounded {healthInfo.bg} {healthInfo.color}">
              {healthInfo.label}
            </span>
          </div>

          <div class="space-y-3">
            <div class="flex items-center justify-between p-2.5 rounded-lg border border-border/40 bg-background/50">
              <div class="flex items-center gap-2">
                <Activity size={14} class="text-muted-foreground" />
                <span class="text-xs text-foreground font-medium">{m.ai_center_overview_sessions ? m.ai_center_overview_sessions() : "الجلسات النشطة"}</span>
              </div>
              <span class="text-sm font-mono font-bold text-foreground">{sessionCount}</span>
            </div>

            <div class="flex items-center justify-between p-2.5 rounded-lg border border-border/40 bg-background/50">
              <div class="flex items-center gap-2">
                <Zap size={14} class="text-muted-foreground" />
                <span class="text-xs text-foreground font-medium">{m.ai_center_overview_autopilot ? m.ai_center_overview_autopilot() : "وضع الطيار الآلي"}</span>
              </div>
              <span class="text-xs font-bold text-foreground bg-muted px-2 py-0.5 rounded border border-border">{autopilotMode}</span>
            </div>

            <div class="flex items-center justify-between p-2.5 rounded-lg border border-border/40 bg-background/50">
              <div class="flex items-center gap-2">
                <Plug size={14} class="text-muted-foreground" />
                <span class="text-xs text-foreground font-medium">{m.ai_center_overview_integrations ? m.ai_center_overview_integrations() : "التكاملات المتصلة"}</span>
              </div>
              <span class="text-sm font-mono font-bold text-foreground">{connectedAppsCount}</span>
            </div>
          </div>

          <a href={localizeHref(`/${data.orgId}/ai-center/permissions`)} class="w-full">
            <Button size="sm" variant="outline" class="w-full text-xs font-semibold justify-between">
              <span>إدارة مصفوفة الصلاحيات</span>
              <ShieldAlert size={12} />
            </Button>
          </a>
        </div>

      </div>

      <!-- 3. Bottom Section: Recent Action Logs Timeline & Connected Integrations Quick Status -->
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
        
        <!-- Recent Action Logs Timeline (2 cols) -->
        <div class="lg:col-span-2 bg-card border border-border rounded-xl p-5 shadow-sm space-y-4">
          <div class="flex items-center justify-between border-b border-border/40 pb-3">
            <div class="flex items-center gap-2">
              <Clock size={16} class="text-primary" />
              <h2 class="text-sm font-bold text-foreground">
                {m.ai_center_overview_recent_activity ? m.ai_center_overview_recent_activity() : "أحدث الأنشطة والأوامر المنفذة"}
              </h2>
            </div>
            <a href={localizeHref(`/${data.orgId}/ai-center/logs`)} class="text-xs text-primary hover:underline flex items-center gap-1 font-semibold">
              <span>عرض الكل</span>
              <ArrowRight size={10} class="rtl:rotate-180" />
            </a>
          </div>

          {#if recentLogs.length > 0}
            <div class="divide-y divide-border/30 border border-border/40 rounded-lg overflow-hidden bg-background/30">
              {#each recentLogs as log}
                <div class="px-4 py-3 flex items-center justify-between hover:bg-muted/20 transition-colors">
                  <div class="flex items-center gap-3">
                    <span class="w-2 h-2 rounded-full {log.status === 'completed' ? 'bg-primary' : log.status === 'failed' ? 'bg-destructive' : 'bg-muted-foreground/60'} shrink-0"></span>
                    <div>
                      <code class="text-[11px] font-mono font-semibold text-foreground bg-muted px-1.5 py-0.5 rounded">{log.task_type || log.action_code || log.action || "مهمة أتمتة"}</code>
                      <p class="text-[11px] text-muted-foreground mt-1 truncate max-w-sm">{log.last_error || log.reason || log.reasoning || (log.status === 'completed' ? 'تم تنفيذ المهمة بنجاح.' : 'مجدولة للتنفيذ القادم.')}</p>
                    </div>
                  </div>
                  <span class="text-[10px] text-muted-foreground font-mono shrink-0">{log.scheduled_at || log.executed_at || "—"}</span>
                </div>
              {/each}
            </div>
          {:else}
            <div class="border border-border/40 rounded-lg p-6 flex flex-col items-center text-center bg-background/20">
              <Clock size={20} class="text-muted-foreground mb-2" />
              <p class="text-xs font-semibold text-foreground">لا توجد سجلات تنفيذية جديدة</p>
              <p class="text-[11px] text-muted-foreground mt-0.5">ستظهر السجلات هنا فور قيام الوكلاء بتنفيذ مهام أو توصيات تلقائية.</p>
            </div>
          {/if}
        </div>

        <!-- Connected Providers Quick List (1 col) -->
        <div class="bg-card border border-border rounded-xl p-5 shadow-sm flex flex-col justify-between space-y-4">
          <div class="flex items-center justify-between border-b border-border/40 pb-3">
            <div class="flex items-center gap-2">
              <Plug size={16} class="text-primary" />
              <h2 class="text-sm font-bold text-foreground">
                {m.ai_center_overview_connected_providers ? m.ai_center_overview_connected_providers() : "المزودات المربوطة"}
              </h2>
            </div>
            <a href={localizeHref(`/${data.orgId}/ai-center/integrations`)} class="text-xs text-primary hover:underline font-semibold">
              المتجر
            </a>
          </div>

          {#if connectedIntegrations.length > 0}
            <div class="space-y-2.5">
              {#each connectedIntegrations as item}
                <div class="p-3 border border-border/40 rounded-lg bg-background/40 flex items-center justify-between">
                  <div class="flex items-center gap-2.5">
                    <Plug size={16} class="text-primary" />
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
            <div class="border border-border/40 rounded-lg p-6 flex flex-col items-center text-center bg-background/20 space-y-1.5">
              <Plug size={20} class="text-muted-foreground" />
              <p class="text-xs font-bold text-foreground">لا توجد خدمات مربوطة حالياً</p>
              <p class="text-[11px] text-muted-foreground">يمكنك ربط وتكتاف أجهزة المزودين من خلال متجر التكاملات.</p>
            </div>
          {/if}

          <a href={localizeHref(`/${data.orgId}/ai-center/settings`)} class="w-full">
            <Button size="sm" variant="ghost" class="w-full text-xs font-semibold justify-center">
              <span>إدارة تكوين المزودات والنماذج</span>
            </Button>
          </a>
        </div>

      </div>

    </div>
  {/if}
</div>
