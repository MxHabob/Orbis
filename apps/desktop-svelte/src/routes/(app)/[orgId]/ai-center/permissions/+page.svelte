<!--
@component PermissionsPage
AI Center Governance — Risk matrix from ActionRegistry, approval gates, pending approvals.
Clean Midday inspired layout with elegant subheadings.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { commands } from "$lib/core/ipc";
import {
  Shield,
  ShieldAlert,
  ShieldCheck,
  ShieldX,
  CheckCircle,
  Lock,
  Bot,
  ChevronDown,
  AlertCircle,
  Check,
  X,
} from "@lucide/svelte";
import { Button } from "@orbis/ui-svelte/button";

let { data }: { data: { orgId: string } } = $props();

// ── Types ───────────────────────────────────────────────────
interface ActionDef {
  code: string;
  name: string;
  name_ar?: string;
  domain: string;
  risk_level: "low" | "medium" | "high" | "critical";
  requires_approval: boolean;
  is_reversible: boolean;
}

interface PendingApproval {
  id: string;
  action_code: string;
  reasoning: string;
  risk_level: string;
  executed_at: string;
}

// ── State ───────────────────────────────────────────────────
let actions = $state<ActionDef[]>([]);
let pendingApprovals = $state<PendingApproval[]>([]);
let isLoading = $state(true);
let expandedDomain = $state<string | null>(null);

async function loadData() {
  isLoading = true;
  try {
    const [registered, logs] = await Promise.all([
      (commands as any).aiCenter?.listRegisteredActions?.().catch(() => []) ?? Promise.resolve([]),
      (commands as any).aiCenter?.listActionLogs?.(data.orgId).catch(() => []) ?? commands.iam?.listAuditLogs?.().catch(() => []) ?? Promise.resolve([]),
    ]);
    actions = (registered as ActionDef[]) ?? [];
    pendingApprovals = ((logs as any[]) ?? []).filter((l: any) => l.status === "pending" && l.approval_required);
  } catch (e) {
    console.warn("[AI Center] loadData:", e);
    actions = [];
  } finally {
    isLoading = false;
  }
}

onMount(() => { if (data.orgId) loadData(); });

async function handleApprove(id: string) {
  try {
    await (commands as any).aiCenter?.approveAction?.(id);
    pendingApprovals = pendingApprovals.filter(a => a.id !== id);
  } catch (e) { console.error(e); }
}

async function handleReject(id: string) {
  try {
    await (commands as any).aiCenter?.rejectAction?.(id, "تم الرفض بواسطة المسؤول");
    pendingApprovals = pendingApprovals.filter(a => a.id !== id);
  } catch (e) { console.error(e); }
}

const domains = $derived([...new Set(actions.map(a => a.domain))]);
const riskSummary = $derived({
  low: actions.filter(a => a.risk_level === "low").length,
  medium: actions.filter(a => a.risk_level === "medium").length,
  high: actions.filter(a => a.risk_level === "high").length,
  critical: actions.filter(a => a.risk_level === "critical").length,
});
const approvalCount = $derived(actions.filter(a => a.requires_approval).length);

const domainLabels: Record<string, string> = {
  iam: "الهوية والحسابات", academic: "الشؤون الأكاديمية", assessment: "التقييم والنتائج",
  finance: "الشؤون المالية", lifecycle: "دورة العام الأكاديمي", scheduling: "الجدولة والحصص", exam: "الاختبارات",
};
</script>

<div class="h-full overflow-y-auto font-sans p-6 scrollbar-hide">
  <div class="w-full space-y-6">

    <!-- Risk Summary Cards (Full Grid Width) -->
    <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
      {#each [
        { level: "low", label: "منخفض المخاطر", count: riskSummary.low, icon: ShieldCheck },
        { level: "medium", label: "متوسط المخاطر", count: riskSummary.medium, icon: Shield },
        { level: "high", label: "مرتفع المخاطر", count: riskSummary.high, icon: ShieldAlert },
        { level: "critical", label: "حرج للغاية", count: riskSummary.critical, icon: ShieldX },
      ] as card}
        <div class="border border-border rounded-xl p-4 bg-card flex flex-col justify-between shadow-sm">
          <div class="flex items-center justify-between mb-2">
            <span class="text-[11px] font-semibold text-muted-foreground">{card.label}</span>
            <card.icon size={15} class="text-muted-foreground/70" />
          </div>
          <p class="text-xl font-bold font-mono text-foreground">{card.count}</p>
        </div>
      {/each}
    </div>

    <!-- Approval Gates Pill List -->
    <div class="bg-card border border-border rounded-xl p-5 shadow-sm space-y-3">
      <div class="flex items-center justify-between border-b border-border/40 pb-2.5">
        <h2 class="text-xs font-semibold text-foreground flex items-center gap-2">
          <Lock size={14} class="text-primary" />
          <span>{m.ai_center_permissions_gates ? m.ai_center_permissions_gates() : "بوابات الاعتماد والموافقة البشرية"}</span>
        </h2>
        <span class="text-[10px] font-mono text-muted-foreground">{approvalCount} من {actions.length} تتطلب الإذن</span>
      </div>
      {#if approvalCount > 0}
        <div class="flex flex-wrap gap-2">
          {#each actions.filter(a => a.requires_approval) as action}
            <span class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-md text-[10px] font-mono font-medium bg-muted text-foreground border border-border">
              <Lock size={9} class="text-primary" /> {action.code}
            </span>
          {/each}
        </div>
      {:else}
        <p class="text-xs text-muted-foreground">لا توجد بوابات اعتماد معينة حالياً.</p>
      {/if}
    </div>

    <!-- Domains Accordion -->
    <div class="space-y-3">
      <h2 class="text-xs font-semibold text-muted-foreground uppercase tracking-wider px-0.5">المجالات التشغيلية وإجراءاتها</h2>
      {#if domains.length > 0}
        {#each domains as domain}
          {@const domainActions = actions.filter(a => a.domain === domain)}
          <div class="bg-card border border-border rounded-xl overflow-hidden shadow-sm transition-all">
            <button
              type="button"
              class="w-full px-5 py-3.5 flex items-center justify-between hover:bg-muted/30 transition-colors cursor-pointer"
              onclick={() => expandedDomain = expandedDomain === domain ? null : domain}
            >
              <div class="flex items-center gap-2.5">
                <Bot size={15} class="text-primary" />
                <span class="text-xs font-semibold text-foreground">{domainLabels[domain] ?? domain}</span>
                <span class="text-[10px] text-muted-foreground font-mono">({domainActions.length} إجراء)</span>
              </div>
              <ChevronDown size={14} class="text-muted-foreground transition-transform {expandedDomain === domain ? 'rotate-180' : ''}" />
            </button>

            {#if expandedDomain === domain}
              <div class="border-t border-border/40 bg-background/40">
                <table class="w-full text-xs text-start">
                  <thead class="bg-muted/40 border-b border-border/30">
                    <tr>
                      <th class="text-start px-4 py-2.5 font-semibold text-muted-foreground">رمز الإجراء (Action Code)</th>
                      <th class="text-start px-4 py-2.5 font-semibold text-muted-foreground">الوصف</th>
                      <th class="text-center px-4 py-2.5 font-semibold text-muted-foreground">المخاطرة</th>
                      <th class="text-center px-4 py-2.5 font-semibold text-muted-foreground">الموافقة</th>
                      <th class="text-center px-4 py-2.5 font-semibold text-muted-foreground">القابلية للتراجع</th>
                    </tr>
                  </thead>
                  <tbody class="divide-y divide-border/20">
                    {#each domainActions as action}
                      <tr class="hover:bg-muted/20 transition-colors">
                        <td class="px-4 py-2.5 font-mono text-[11px] font-semibold text-foreground">{action.code}</td>
                        <td class="px-4 py-2.5 text-foreground">{action.name}</td>
                        <td class="px-4 py-2.5 text-center">
                          <span class="px-2 py-0.5 rounded text-[10px] font-medium uppercase bg-muted text-foreground border border-border">{action.risk_level}</span>
                        </td>
                        <td class="px-4 py-2.5 text-center">
                          {#if action.requires_approval}
                            <Lock size={12} class="inline text-primary" />
                          {:else}
                            <span class="text-muted-foreground/30">—</span>
                          {/if}
                        </td>
                        <td class="px-4 py-2.5 text-center">
                          {#if action.is_reversible}
                            <CheckCircle size={12} class="inline text-primary" />
                          {:else}
                            <span class="text-muted-foreground/30">—</span>
                          {/if}
                        </td>
                      </tr>
                    {/each}
                  </tbody>
                </table>
              </div>
            {/if}
          </div>
        {/each}
      {:else}
        <div class="bg-card border border-border rounded-xl p-8 flex flex-col items-center text-center">
          <Shield size={24} class="text-muted-foreground/60 mb-2" />
          <p class="text-xs font-semibold text-foreground">لا توجد إجراءات مسجلة حالياً في مصفوفة الصلاحيات</p>
          <p class="text-[11px] text-muted-foreground mt-0.5">يتم تسجيل وتصنيف الأوامر الصادرة من الوكلاء تلقائياً عند تنفيذ العمليات.</p>
        </div>
      {/if}
    </div>

    <!-- Pending Approvals -->
    <div class="space-y-3">
      <h2 class="text-xs font-semibold text-muted-foreground uppercase tracking-wider px-0.5">طلبات الاعتماد المعلقة</h2>
      {#if pendingApprovals.length > 0}
        <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
          {#each pendingApprovals as approval (approval.id)}
            <div class="bg-card border border-border rounded-xl p-4 flex items-center justify-between shadow-sm">
              <div>
                <code class="text-xs font-mono font-semibold text-foreground">{approval.action_code}</code>
                <p class="text-[11px] text-muted-foreground mt-0.5">{approval.reasoning}</p>
              </div>
              <div class="flex items-center gap-2">
                <Button size="sm" variant="outline" class="text-xs" onclick={() => handleReject(approval.id)}>
                  <X size={12} class="ml-1" /> رفض
                </Button>
                <Button size="sm" class="text-xs font-semibold" onclick={() => handleApprove(approval.id)}>
                  <Check size={12} class="ml-1" /> اعتماد الإجراء
                </Button>
              </div>
            </div>
          {/each}
        </div>
      {:else}
        <div class="bg-card border border-border rounded-xl p-8 flex flex-col items-center text-center">
          <CheckCircle size={22} class="text-primary mb-2" />
          <p class="text-xs font-semibold text-foreground">لا توجد طلبات موافقة معلقة حالياً</p>
          <p class="text-[11px] text-muted-foreground mt-0.5">سيتم إخطارك فور صدور إجراء عالي المخاطر يحتاج إذن المدير.</p>
        </div>
      {/if}
    </div>

  </div>
</div>
