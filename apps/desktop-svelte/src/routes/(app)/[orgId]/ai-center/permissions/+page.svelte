<!--
@component PermissionsPage
AI Center Permissions — Risk matrix from ActionRegistry, approval gates, pending approvals.
Wired to SDK: commands.aiCenter.listRegisteredActions / approveAction / rejectAction
-->
<script lang="ts">
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
} from "lucide-svelte";
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

// ── Load from SDK ───────────────────────────────────────────
async function loadData() {
  isLoading = true;
  try {
    const [registered, logs] = await Promise.all([
      commands.aiCenter.listRegisteredActions().catch(() => []),
      commands.aiCenter.listActionLogs(data.orgId).catch(() => []),
    ]);
    actions = (registered as ActionDef[]) ?? [];
    pendingApprovals = ((logs as any[]) ?? []).filter((l: any) => l.status === "pending" && l.approval_required);
  } catch (e) {
    console.warn("[AI Center] loadData:", e);
    // Fallback: use hardcoded registry (same as backend)
    actions = getDefaultActions();
  } finally {
    isLoading = false;
  }
}

onMount(() => { if (data.orgId) loadData(); });

// ── Approve / Reject ────────────────────────────────────────
async function handleApprove(id: string) {
  try {
    await commands.aiCenter.approveAction(id);
    pendingApprovals = pendingApprovals.filter(a => a.id !== id);
  } catch (e) { console.error(e); }
}

async function handleReject(id: string) {
  try {
    await commands.aiCenter.rejectAction(id, "Manually rejected by admin");
    pendingApprovals = pendingApprovals.filter(a => a.id !== id);
  } catch (e) { console.error(e); }
}

// ── Computed ────────────────────────────────────────────────
const domains = $derived([...new Set(actions.map(a => a.domain))]);
const riskSummary = $derived({
  low: actions.filter(a => a.risk_level === "low").length,
  medium: actions.filter(a => a.risk_level === "medium").length,
  high: actions.filter(a => a.risk_level === "high").length,
  critical: actions.filter(a => a.risk_level === "critical").length,
});
const approvalCount = $derived(actions.filter(a => a.requires_approval).length);

const riskColors: Record<string, { bg: string; text: string; border: string }> = {
  low:      { bg: "bg-emerald-50 dark:bg-emerald-950/30", text: "text-emerald-600 dark:text-emerald-400", border: "border-emerald-200 dark:border-emerald-900" },
  medium:   { bg: "bg-amber-50 dark:bg-amber-950/30",   text: "text-amber-600 dark:text-amber-400",     border: "border-amber-200 dark:border-amber-900" },
  high:     { bg: "bg-orange-50 dark:bg-orange-950/30",  text: "text-orange-600 dark:text-orange-400",   border: "border-orange-200 dark:border-orange-900" },
  critical: { bg: "bg-red-50 dark:bg-red-950/30",       text: "text-red-600 dark:text-red-400",          border: "border-red-200 dark:border-red-900" },
};

const domainLabels: Record<string, string> = {
  iam: "Identity & Access", academic: "Academic", assessment: "Assessment",
  finance: "Finance", lifecycle: "Lifecycle", scheduling: "Scheduling", exam: "Exams",
};

// ── Fallback action registry ────────────────────────────────
function getDefaultActions(): ActionDef[] {
  return [
    { code: "iam.user.list", name: "List Users", domain: "iam", risk_level: "low", requires_approval: false, is_reversible: false },
    { code: "iam.user.get", name: "Get User", domain: "iam", risk_level: "low", requires_approval: false, is_reversible: false },
    { code: "iam.user.update_role", name: "Update User Role", domain: "iam", risk_level: "high", requires_approval: true, is_reversible: true },
    { code: "academic.enrollment.list", name: "List Enrollments", domain: "academic", risk_level: "low", requires_approval: false, is_reversible: false },
    { code: "academic.enrollment.create", name: "Create Enrollment", domain: "academic", risk_level: "medium", requires_approval: false, is_reversible: true },
    { code: "academic.enrollment.transfer", name: "Transfer Student", domain: "academic", risk_level: "medium", requires_approval: false, is_reversible: true },
    { code: "academic.enrollment.withdraw", name: "Withdraw Student", domain: "academic", risk_level: "high", requires_approval: true, is_reversible: true },
    { code: "academic.class.list", name: "List Classes", domain: "academic", risk_level: "low", requires_approval: false, is_reversible: false },
    { code: "academic.subject.list", name: "List Subjects", domain: "academic", risk_level: "low", requires_approval: false, is_reversible: false },
    { code: "assessment.grade.list", name: "List Grades", domain: "assessment", risk_level: "low", requires_approval: false, is_reversible: false },
    { code: "assessment.grade.enter", name: "Enter Grades", domain: "assessment", risk_level: "medium", requires_approval: false, is_reversible: true },
    { code: "assessment.attendance.list", name: "List Attendance", domain: "assessment", risk_level: "low", requires_approval: false, is_reversible: false },
    { code: "assessment.attendance.record", name: "Record Attendance", domain: "assessment", risk_level: "low", requires_approval: false, is_reversible: false },
    { code: "assessment.behavior.list", name: "List Behavior", domain: "assessment", risk_level: "low", requires_approval: false, is_reversible: false },
    { code: "assessment.behavior.create", name: "Create Behavior Record", domain: "assessment", risk_level: "medium", requires_approval: false, is_reversible: true },
    { code: "finance.fee.list", name: "List Fees", domain: "finance", risk_level: "low", requires_approval: false, is_reversible: false },
    { code: "finance.fee.assign", name: "Assign Fee", domain: "finance", risk_level: "medium", requires_approval: false, is_reversible: true },
    { code: "finance.payment.list", name: "List Payments", domain: "finance", risk_level: "low", requires_approval: false, is_reversible: false },
    { code: "finance.payment.record", name: "Record Payment", domain: "finance", risk_level: "medium", requires_approval: false, is_reversible: true },
    { code: "finance.treasury.balance", name: "Treasury Balance", domain: "finance", risk_level: "low", requires_approval: false, is_reversible: false },
    { code: "finance.report.generate", name: "Financial Report", domain: "finance", risk_level: "low", requires_approval: false, is_reversible: false },
    { code: "lifecycle.period.list", name: "List Periods", domain: "lifecycle", risk_level: "low", requires_approval: false, is_reversible: false },
    { code: "lifecycle.period.lock", name: "Lock Period", domain: "lifecycle", risk_level: "critical", requires_approval: true, is_reversible: false },
    { code: "lifecycle.calendar.list", name: "List Calendar", domain: "lifecycle", risk_level: "low", requires_approval: false, is_reversible: false },
    { code: "lifecycle.promotion.run", name: "Run Promotion", domain: "lifecycle", risk_level: "critical", requires_approval: true, is_reversible: false },
    { code: "scheduling.timetable.generate", name: "Generate Timetable", domain: "scheduling", risk_level: "high", requires_approval: true, is_reversible: false },
    { code: "scheduling.timetable.view", name: "View Timetable", domain: "scheduling", risk_level: "low", requires_approval: false, is_reversible: false },
    { code: "exam.session.list", name: "List Exam Sessions", domain: "exam", risk_level: "low", requires_approval: false, is_reversible: false },
    { code: "exam.session.create", name: "Create Exam Session", domain: "exam", risk_level: "medium", requires_approval: false, is_reversible: true },
    { code: "exam.numbers.assign", name: "Assign Exam Numbers", domain: "exam", risk_level: "high", requires_approval: true, is_reversible: false },
    { code: "exam.schedule.generate", name: "Auto-Schedule Exams", domain: "exam", risk_level: "high", requires_approval: true, is_reversible: false },
  ];
}
</script>

<div class="h-full overflow-y-auto">
  <div class="px-8 py-6 max-w-4xl space-y-5">
    <div>
      <h1 class="text-lg font-semibold text-foreground mb-1">Permissions</h1>
      <p class="text-[11px] text-muted-foreground">
        AI agent permissions, risk levels, and approval workflows for {actions.length} registered actions.
      </p>
    </div>

    <!-- Risk Summary -->
    <div class="grid grid-cols-4 gap-3">
      {#each [
        { level: "low", label: "Low Risk", count: riskSummary.low, icon: ShieldCheck },
        { level: "medium", label: "Medium", count: riskSummary.medium, icon: Shield },
        { level: "high", label: "High Risk", count: riskSummary.high, icon: ShieldAlert },
        { level: "critical", label: "Critical", count: riskSummary.critical, icon: ShieldX },
      ] as card}
        {@const c = riskColors[card.level]}
        <div class="border {c.border} rounded-lg p-3.5 {c.bg}">
          <div class="flex items-center gap-1.5 mb-1.5">
            <card.icon size={14} class={c.text} />
            <span class="text-[10px] font-medium uppercase tracking-wider {c.text}">{card.label}</span>
          </div>
          <p class="text-xl font-semibold {c.text}">{card.count}</p>
        </div>
      {/each}
    </div>

    <!-- Approval Gates -->
    <div class="border border-border/60 rounded-lg p-4">
      <div class="flex items-center justify-between mb-2.5">
        <h2 class="text-xs font-medium text-foreground flex items-center gap-1.5"><Lock size={13} /> Approval Gates</h2>
        <span class="text-[10px] text-muted-foreground">{approvalCount} of {actions.length} require approval</span>
      </div>
      <div class="flex flex-wrap gap-1.5">
        {#each actions.filter(a => a.requires_approval) as action}
          {@const c = riskColors[action.risk_level]}
          <span class="inline-flex items-center gap-1 px-2 py-0.5 rounded text-[9px] font-mono {c.bg} {c.text} border {c.border}">
            <Lock size={7} /> {action.code}
          </span>
        {/each}
      </div>
    </div>

    <!-- Domains -->
    <div class="space-y-2">
      {#each domains as domain}
        {@const domainActions = actions.filter(a => a.domain === domain)}
        <div class="border border-border/60 rounded-lg overflow-hidden">
          <button
            class="w-full px-4 py-2.5 flex items-center justify-between hover:bg-muted/30 transition-colors cursor-pointer"
            onclick={() => expandedDomain = expandedDomain === domain ? null : domain}
          >
            <div class="flex items-center gap-2">
              <Bot size={13} class="text-foreground" />
              <span class="text-xs font-medium text-foreground">{domainLabels[domain] ?? domain}</span>
              <span class="text-[10px] text-muted-foreground">({domainActions.length})</span>
            </div>
            <div class="flex items-center gap-1.5">
              {#each ["low", "medium", "high", "critical"] as level}
                {@const cnt = domainActions.filter(a => a.risk_level === level).length}
                {#if cnt > 0}
                  {@const c = riskColors[level]}
                  <span class="px-1.5 py-0.5 rounded text-[9px] font-medium {c.bg} {c.text}">{cnt} {level}</span>
                {/if}
              {/each}
              <ChevronDown size={12} class="text-muted-foreground transition-transform {expandedDomain === domain ? 'rotate-180' : ''}" />
            </div>
          </button>

          {#if expandedDomain === domain}
            <div class="border-t border-border/40">
              <table class="w-full text-xs">
                <thead class="bg-muted/30">
                  <tr>
                    <th class="text-left px-4 py-2 font-medium text-muted-foreground">Action Code</th>
                    <th class="text-left px-4 py-2 font-medium text-muted-foreground">Name</th>
                    <th class="text-center px-4 py-2 font-medium text-muted-foreground">Risk</th>
                    <th class="text-center px-4 py-2 font-medium text-muted-foreground">Approval</th>
                    <th class="text-center px-4 py-2 font-medium text-muted-foreground">Reversible</th>
                  </tr>
                </thead>
                <tbody>
                  {#each domainActions as action}
                    {@const c = riskColors[action.risk_level]}
                    <tr class="border-t border-border/20 hover:bg-muted/20">
                      <td class="px-4 py-2"><code class="text-[10px] font-mono">{action.code}</code></td>
                      <td class="px-4 py-2 text-foreground">{action.name}</td>
                      <td class="px-4 py-2 text-center">
                        <span class="px-1.5 py-0.5 rounded text-[10px] font-medium {c.bg} {c.text}">{action.risk_level}</span>
                      </td>
                      <td class="px-4 py-2 text-center">
                        {#if action.requires_approval}<Lock size={11} class="inline text-orange-500" />{:else}<span class="text-muted-foreground/40">—</span>{/if}
                      </td>
                      <td class="px-4 py-2 text-center">
                        {#if action.is_reversible}<CheckCircle size={11} class="inline text-emerald-500" />{:else}<span class="text-muted-foreground/40">—</span>{/if}
                      </td>
                    </tr>
                  {/each}
                </tbody>
              </table>
            </div>
          {/if}
        </div>
      {/each}
    </div>

    <!-- Pending Approvals -->
    <div>
      <h2 class="text-sm font-medium text-foreground mb-2">Pending Approvals</h2>
      {#if pendingApprovals.length > 0}
        <div class="space-y-2">
          {#each pendingApprovals as approval (approval.id)}
            <div class="border border-border/60 rounded-lg p-4 flex items-center justify-between">
              <div>
                <code class="text-[11px] font-mono text-foreground">{approval.action_code}</code>
                <p class="text-[10px] text-muted-foreground mt-0.5">{approval.reasoning}</p>
              </div>
              <div class="flex gap-2">
                <Button size="sm" variant="outline" onclick={() => handleReject(approval.id)}>Reject</Button>
                <Button size="sm" onclick={() => handleApprove(approval.id)}>Approve</Button>
              </div>
            </div>
          {/each}
        </div>
      {:else}
        <div class="border border-border/60 rounded-lg p-5 flex flex-col items-center">
          <CheckCircle size={18} class="text-emerald-500 mb-1.5" />
          <p class="text-xs text-muted-foreground">No pending approvals</p>
        </div>
      {/if}
    </div>
  </div>
</div>
