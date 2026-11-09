<script lang="ts">
import { api } from "$lib/api";
import * as m from "$lib/paraglide/messages";
import {
  Check,
  CheckCircle,
  ChevronDown,
  Lock,
  Sparkles,
  UserBadgeCheck,
  XMark,
} from "@orbis/icons";
import { Button } from "@orbis/ui/button";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
  CardAction,
} from "@orbis/ui/card";
import { onMount } from "svelte";

let { orgId }: { orgId: string } = $props();

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
      (api as any).aiCenter?.listRegisteredActions?.().catch(() => []) ??
        Promise.resolve([]),
      (api as any).aiCenter?.listActionLogs?.(orgId).catch(() => []) ??
        api?.listAuditLogs?.().catch(() => []) ??
        Promise.resolve([]),
    ]);
    actions = (registered as ActionDef[]) ?? [];
    pendingApprovals = ((logs as any[]) ?? []).filter(
      (l: any) => l.status === "pending" && l.approval_required,
    );
  } catch (e) {
    console.warn("[AI Center] loadData:", e);
    actions = [];
  } finally {
    isLoading = false;
  }
}

onMount(() => {
  if (orgId) loadData();
});

async function handleApprove(id: string) {
  try {
    await (api as any).aiCenter?.approveAction?.(id);
    pendingApprovals = pendingApprovals.filter((a) => a.id !== id);
  } catch (e) {
    console.error(e);
  }
}

async function handleReject(id: string) {
  try {
    await (api as any).aiCenter?.rejectAction?.(
      id,
      m.ai_permission_rejection_reason(),
    );
    pendingApprovals = pendingApprovals.filter((a) => a.id !== id);
  } catch (e) {
    console.error(e);
  }
}

const domains = $derived([...new Set(actions.map((a) => a.domain))]);
const riskSummary = $derived({
  low: actions.filter((a) => a.risk_level === "low").length,
  medium: actions.filter((a) => a.risk_level === "medium").length,
  high: actions.filter((a) => a.risk_level === "high").length,
  critical: actions.filter((a) => a.risk_level === "critical").length,
});
const approvalCount = $derived(
  actions.filter((a) => a.requires_approval).length,
);

const domainLabels: Record<string, string> = {
  iam: m.ai_ui_domain_identity_accounts(),
  academic: m.ai_ui_domain_academic_affairs(),
  assessment: m.ai_ui_domain_assessment_results(),
  finance: m.ai_ui_domain_finance(),
  lifecycle: m.ai_ui_domain_academic_calendar(),
  scheduling: m.ai_ui_domain_scheduling_classes(),
  exam: m.ai_ui_domain_exams(),
};

const riskLabels: Record<string, string> = {
  low: m.ai_ui_risk_low(),
  medium: m.ai_ui_risk_medium(),
  high: m.ai_ui_risk_high(),
  critical: m.ai_ui_risk_critical(),
};
</script>

<div class="h-full overflow-y-auto font-sans p-6 scrollbar-hide">
  <div class="w-full space-y-6">
    <!-- Risk Summary Cards (Full Grid Width) -->
    <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
      {#each [{ level: "low", label: m.ai_ui_risk_low(), count: riskSummary.low, icon: UserBadgeCheck }, { level: "medium", label: m.ai_ui_risk_medium(), count: riskSummary.medium, icon: UserBadgeCheck }, { level: "high", label: m.ai_ui_risk_high(), count: riskSummary.high, icon: UserBadgeCheck }, { level: "critical", label: m.ai_ui_risk_critical(), count: riskSummary.critical, icon: UserBadgeCheck }] as card}
        <Card>
          <CardContent class="p-4 flex flex-col justify-between">
            <div class="flex items-center justify-between mb-2">
              <span class="text-[11px] font-semibold text-muted-foreground"
                >{card.label}</span
              >
              <card.icon size={15} class="text-muted-foreground/70" />
            </div>
            <p class="text-xl font-bold font-mono text-foreground">
              {card.count}
            </p>
          </CardContent>
        </Card>
      {/each}
    </div>

    <!-- Approval Gates Pill List -->
    <Card>
      <CardHeader>
        <div class="flex items-center gap-2">
          <Lock size={14} class="text-primary" />
          <CardTitle>
            {m.ai_center_permissions_gates()}
          </CardTitle>
        </div>
        <CardAction>
          <span class="text-[10px] font-mono text-muted-foreground">
            {m.ai_ui_approval_count({ approvalCount, actionCount: actions.length })}
          </span>
        </CardAction>
      </CardHeader>
      <CardContent class="p-6">
        {#if approvalCount > 0}
          <div class="flex flex-wrap gap-2">
            {#each actions.filter((a) => a.requires_approval) as action}
              <span
                class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-md text-[10px] font-mono font-medium bg-muted text-foreground border border-border"
              >
                <Lock size={9} class="text-primary" />
                {action.code}
              </span>
            {/each}
          </div>
        {:else}
          <p class="text-xs text-muted-foreground text-start">
            {m.ai_ui_no_approval_gates()}
          </p>
        {/if}
      </CardContent>
    </Card>

    <!-- Domains Accordion -->
    <div class="space-y-3">
      <h2
        class="text-xs font-semibold text-muted-foreground uppercase tracking-wider px-0.5 text-start"
      >
        {m.ai_ui_operational_domains_actions()}
      </h2>
      {#if domains.length > 0}
        {#each domains as domain}
          {@const domainActions = actions.filter((a) => a.domain === domain)}
          <Card>
            <button
              type="button"
              class="w-full px-5 py-3.5 flex items-center justify-between hover:bg-muted/30 transition-colors cursor-pointer bg-muted/40 border-b border-border"
              onclick={() =>
                (expandedDomain = expandedDomain === domain ? null : domain)}
            >
              <div class="flex items-center gap-2.5">
                <Sparkles size={14} class="text-primary" />
                <span class="text-xs font-semibold text-foreground"
                  >{domainLabels[domain] ?? domain}</span
                >
                <span class="text-[10px] text-muted-foreground font-mono"
                  >{m.ai_ui_action_count_parenthetical({ count: domainActions.length })}</span
                >
              </div>
              <ChevronDown
                size={14}
                class="text-muted-foreground transition-transform {expandedDomain ===
                domain
                  ? 'rotate-180'
                  : ''}"
              />
            </button>

            {#if expandedDomain === domain}
              <CardContent class="p-0">
                <table class="w-full text-xs text-start">
                  <thead class="bg-muted/20 border-b border-border">
                    <tr>
                      <th
                        class="text-start px-4 py-2.5 font-semibold text-muted-foreground"
                        >{m.ai_ui_action_code()}</th
                      >
                      <th
                        class="text-start px-4 py-2.5 font-semibold text-muted-foreground"
                        >{m.ai_ui_description()}</th
                      >
                      <th
                        class="text-center px-4 py-2.5 font-semibold text-muted-foreground"
                        >{m.ai_ui_risk()}</th
                      >
                      <th
                        class="text-center px-4 py-2.5 font-semibold text-muted-foreground"
                        >{m.ai_ui_approval()}</th
                      >
                      <th
                        class="text-center px-4 py-2.5 font-semibold text-muted-foreground"
                        >{m.ai_ui_reversibility()}</th
                      >
                    </tr>
                  </thead>
                  <tbody class="divide-y divide-border">
                    {#each domainActions as action}
                      <tr class="hover:bg-muted/20 transition-colors">
                        <td
                          class="px-4 py-2.5 font-mono text-[11px] font-semibold text-foreground text-start"
                          >{action.code}</td
                        >
                        <td class="px-4 py-2.5 text-foreground text-start"
                          >{action.name}</td
                        >
                        <td class="px-4 py-2.5 text-center">
                          <span
                            class="px-2 py-0.5 rounded text-[10px] font-medium uppercase bg-muted text-foreground border border-border"
                            >{riskLabels[action.risk_level] ?? action.risk_level}</span
                          >
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
                            <Check size={12} class="inline text-primary" />
                          {:else}
                            <XMark size={12} class="inline text-destructive" />
                          {/if}
                        </td>
                      </tr>
                    {/each}
                  </tbody>
                </table>
              </CardContent>
            {/if}
          </Card>
        {/each}
      {/if}
    </div>

    <!-- Pending Approvals Section -->
    {#if pendingApprovals.length > 0}
      <Card class="border-amber-500/30">
        <CardHeader class="bg-amber-500/10 border-b border-amber-500/20">
          <div class="flex items-center gap-2">
            <Lock size={14} class="text-amber-500" />
            <CardTitle class="text-amber-500">
              {m.ai_ui_pending_human_approvals({ count: pendingApprovals.length })}
            </CardTitle>
          </div>
        </CardHeader>
        <CardContent class="p-0 divide-y divide-border">
          {#each pendingApprovals as item}
            <div class="p-4 flex items-center justify-between gap-4">
              <div class="space-y-1 text-start">
                <div class="flex items-center gap-2">
                  <span class="font-mono text-xs font-semibold text-foreground"
                    >{item.action_code}</span
                  >
                  <span
                    class="px-2 py-0.5 rounded text-[9px] font-bold uppercase bg-amber-500/10 text-amber-500 border border-amber-500/20"
                    >{riskLabels[item.risk_level] ?? item.risk_level}</span
                  >
                </div>
                <p class="text-xs text-muted-foreground">{item.reasoning}</p>
                <p class="text-[10px] text-muted-foreground font-mono">
                  {item.executed_at}
                </p>
              </div>
              <div class="flex items-center gap-2 shrink-0">
                <Button
                  size="sm"
                  variant="outline"
                  class="text-xs text-destructive hover:bg-destructive/10"
                  onclick={() => handleReject(item.id)}
                >
                  <XMark size={12} class="me-1" />
                  {m.ai_ui_reject()}
                </Button>
                <Button
                  size="sm"
                  class="text-xs"
                  onclick={() => handleApprove(item.id)}
                >
                  <Check size={12} class="me-1" />
                  {m.ai_ui_approve_and_execute()}
                </Button>
              </div>
            </div>
          {/each}
        </CardContent>
      </Card>
    {/if}
  </div>
</div>
