<!--
  @component AiCenterSettingsPage
  Comprehensive, premium configuration hub for Orbis AI Center & Autopilot Engine.
  Adheres strictly to Orbis Design System and uses Paraglide i18n messages (m.*).
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { commands } from "$lib/core/ipc";
import {
  Zap,
  ShieldAlert,
  SearchCode,
  FileSearch,
  Loader2,
  Sparkles,
  Bot,
  Sliders,
  CheckCircle2,
  Lock,
  Clock,
  Activity,
  Layers,
  GraduationCap,
  Calendar,
  CreditCard,
  UserCheck,
  FileText,
} from "@lucide/svelte";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Switch } from "@orbis/ui-svelte/switch";
import { Label } from "@orbis/ui-svelte/label";
import { Badge } from "@orbis/ui-svelte/badge";
import { Separator } from "@orbis/ui-svelte/separator";
import { toast } from "svelte-sonner";

let { data }: { data: { orgId: string } } = $props();

// ── 1. Autopilot Engine Settings ────────────────────────────
let autopilotSettings = $state({
  enabled: true,
  execution_mode: "suggested" as "automatic" | "suggested" | "manual",
  auto_open_enrollment: false,
  auto_close_enrollment: true,
  auto_record_attendance: true,
  auto_fee_assignment: true,
  auto_grade_entry_reminder: true,
  auto_period_lock: false,
  auto_promotion: false,
  auto_exam_scheduling: false,
  auto_report_generation: true,
  grace_days: 3,
  notify_before_deadline_hours: 24,
});

// ── 2. Risk Matrix & Safety Gates Settings ──────────────────
let safetySettings = $state({
  require_approval_high_risk: true,
  require_approval_critical_risk: true,
  block_irreversible_actions: true,
  audit_all_ai_actions: true,
});

// ── 3. Insight Engine & Anomaly Detection ───────────────────
let insightSettings = $state({
  at_risk_grade_drop_percent: 15,
  at_risk_absence_limit_days: 3,
  detect_grade_anomalies: true,
  detect_attendance_anomalies: true,
  periodic_scan_interval_hours: 6,
});

// ── 4. Explainability & RAG Memory Scope ────────────────────
let memorySettings = $state({
  record_reasoning_logs: true,
  rag_context_limit: 10,
  enable_semantic_resolution: true,
});

let isSaving = $state(false);
let isLoading = $state(true);

onMount(async () => {
  try {
    const config = await commands.ai.aiGetAutopilotConfig(data.orgId);
    if (config) {
      Object.assign(autopilotSettings, config);
    }
  } catch (e) {
    console.warn("[AI Center] aiGetAutopilotConfig fallback:", e);
  } finally {
    isLoading = false;
  }
});

async function saveAllSettings() {
  isSaving = true;
  try {
    await commands.ai.aiSaveAutopilotConfig(data.orgId, autopilotSettings);
    toast.success(m.ai_center_settings_save_success());
  } catch (e) {
    console.warn("[AI Center] save error:", e);
    toast.error(m.ai_center_settings_save_error());
  } finally {
    isSaving = false;
  }
}

const executionModes = $derived([
  {
    id: "suggested",
    title: m.ai_center_settings_mode_suggested_title(),
    subtitle: m.ai_center_settings_mode_suggested_sub(),
    desc: m.ai_center_settings_mode_suggested_desc(),
    icon: Sparkles,
  },
  {
    id: "automatic",
    title: m.ai_center_settings_mode_auto_title(),
    subtitle: m.ai_center_settings_mode_auto_sub(),
    desc: m.ai_center_settings_mode_auto_desc(),
    icon: Zap,
  },
  {
    id: "manual",
    title: m.ai_center_settings_mode_manual_title(),
    subtitle: m.ai_center_settings_mode_manual_sub(),
    desc: m.ai_center_settings_mode_manual_desc(),
    icon: Lock,
  },
]);

// Categorized Workflows for Clean UI
const workflowCategories = $derived([
  {
    categoryTitle: m.ai_center_settings_cat_admissions(),
    items: [
      {
        id: "auto_open_enrollment" as const,
        label: m.ai_center_settings_wf_open_enrollment(),
        desc: m.ai_center_settings_wf_open_enrollment_desc(),
        icon: UserCheck,
        risk: "low",
      },
      {
        id: "auto_close_enrollment" as const,
        label: m.ai_center_settings_wf_close_enrollment(),
        desc: m.ai_center_settings_wf_close_enrollment_desc(),
        icon: Clock,
        risk: "low",
      },
    ],
  },
  {
    categoryTitle: m.ai_center_settings_cat_academics(),
    items: [
      {
        id: "auto_record_attendance" as const,
        label: m.ai_center_settings_wf_record_attendance(),
        desc: m.ai_center_settings_wf_record_attendance_desc(),
        icon: Activity,
        risk: "medium",
      },
      {
        id: "auto_grade_entry_reminder" as const,
        label: m.ai_center_settings_wf_grade_reminders(),
        desc: m.ai_center_settings_wf_grade_reminders_desc(),
        icon: FileText,
        risk: "low",
      },
      {
        id: "auto_exam_scheduling" as const,
        label: m.ai_center_settings_wf_exam_scheduling(),
        desc: m.ai_center_settings_wf_exam_scheduling_desc(),
        icon: Calendar,
        risk: "medium",
      },
      {
        id: "auto_report_generation" as const,
        label: m.ai_center_settings_wf_report_gen(),
        desc: m.ai_center_settings_wf_report_gen_desc(),
        icon: GraduationCap,
        risk: "medium",
      },
    ],
  },
  {
    categoryTitle: m.ai_center_settings_cat_admin(),
    items: [
      {
        id: "auto_fee_assignment" as const,
        label: m.ai_center_settings_wf_fee_assignment(),
        desc: m.ai_center_settings_wf_fee_assignment_desc(),
        icon: CreditCard,
        risk: "high",
      },
      {
        id: "auto_period_lock" as const,
        label: m.ai_center_settings_wf_period_lock(),
        desc: m.ai_center_settings_wf_period_lock_desc(),
        icon: Lock,
        risk: "critical",
      },
      {
        id: "auto_promotion" as const,
        label: m.ai_center_settings_wf_promotion(),
        desc: m.ai_center_settings_wf_promotion_desc(),
        icon: Layers,
        risk: "critical",
      },
    ],
  },
]);
</script>

<svelte:head>
  <title>{m.ai_center_settings_page_title()} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-6 scrollbar-hide bg-background font-sans max-w-6xl mx-auto">
  <form onsubmit={(e) => { e.preventDefault(); saveAllSettings(); }} class="flex flex-col gap-8 w-full">

    <!-- Top Action & Overview Header -->
    <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 pb-5 border-b border-border">
      <div class="flex flex-col gap-1 text-start">
        <div class="flex items-center gap-2">
          <Bot size={22} class="text-primary" />
          <h1 class="text-lg font-bold tracking-tight text-foreground">
            {m.ai_center_settings_page_title()}
          </h1>
          <Badge variant="outline" class="text-[10px] uppercase font-mono px-2 py-0.5 border-primary/30 text-primary bg-primary/5">
            {m.ai_center_settings_badge_engine()}
          </Badge>
        </div>
        <p class="text-xs text-muted-foreground">
          {m.ai_center_settings_page_desc()}
        </p>
      </div>

      <div class="flex items-center gap-3 shrink-0">
        <Button
          type="submit"
          variant="default"
          size="sm"
          class="px-6 font-semibold h-9 text-xs cursor-pointer shadow-xs"
          disabled={isSaving || isLoading}
        >
          {#if isSaving}
            <Loader2 class="size-3.5 animate-spin me-1.5" />
          {/if}
          {isSaving ? m.common_state_saving() : m.settings_action_save_changes()}
        </Button>
      </div>
    </div>

    <!-- ── Section 1: Autopilot Operational Mode ─────────────────── -->
    <div class="border border-border rounded-xl bg-card overflow-hidden shadow-xs flex flex-col">
      <div class="bg-muted/40 px-6 py-3.5 border-b border-border flex items-center justify-between">
        <div class="flex items-center gap-2 text-start">
          <Zap size={16} class="text-primary" />
          <span class="text-xs font-bold text-foreground tracking-wide uppercase font-serif">
            {m.ai_center_settings_section_autopilot_mode()}
          </span>
        </div>
        <div class="flex items-center gap-2">
          <span class="text-[11px] font-semibold text-muted-foreground">
            {autopilotSettings.enabled ? m.ai_center_settings_autopilot_active() : m.ai_center_settings_autopilot_disabled()}
          </span>
          <Switch
            checked={autopilotSettings.enabled}
            onCheckedChange={(v) => (autopilotSettings.enabled = v)}
            disabled={isSaving}
          />
        </div>
      </div>

      <div class="p-6 flex flex-col gap-6">
        <!-- Interactive Mode Cards -->
        <div class="flex flex-col gap-2 text-start">
          <Label class="text-xs font-semibold text-foreground">
            {m.ai_center_settings_mode_authority_title()}
          </Label>
          <span class="text-[11px] text-muted-foreground">
            {m.ai_center_settings_mode_authority_desc()}
          </span>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
          {#each executionModes as mode (mode.id)}
            {@const isSelected = autopilotSettings.execution_mode === mode.id}
            {@const Icon = mode.icon}
            <button
              type="button"
              onclick={() => { autopilotSettings.execution_mode = mode.id as any; }}
              disabled={!autopilotSettings.enabled || isSaving}
              class="flex flex-col text-start p-4 rounded-xl border transition-all text-start relative cursor-pointer {isSelected ? 'border-primary bg-primary/5 shadow-xs ring-1 ring-primary/30' : 'border-border/70 bg-card hover:border-border hover:bg-muted/20'} {!autopilotSettings.enabled ? 'opacity-50 cursor-not-allowed' : ''}"
            >
              <div class="flex items-center justify-between w-full mb-2.5">
                <div class="p-2 rounded-lg {isSelected ? 'bg-primary/20 text-primary' : 'bg-muted text-muted-foreground'}">
                  <Icon size={16} />
                </div>
                {#if isSelected}
                  <span class="flex items-center gap-1 text-[11px] font-bold text-primary">
                    <CheckCircle2 size={13} />
                    {m.ai_center_settings_selected()}
                  </span>
                {:else}
                  <span class="text-[10px] text-muted-foreground font-mono font-medium">{mode.subtitle}</span>
                {/if}
              </div>

              <span class="text-xs font-bold text-foreground mb-1">{mode.title}</span>
              <p class="text-[11px] text-muted-foreground leading-relaxed">{mode.desc}</p>
            </button>
          {/each}
        </div>

        <!-- Operational Thresholds Grid -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4 pt-2 border-t border-border/40">
          <div class="flex flex-col gap-1.5 text-start">
            <Label class="text-xs font-semibold text-foreground">
              {m.ai_center_settings_grace_period_title()}
            </Label>
            <span class="text-[11px] text-muted-foreground leading-relaxed">
              {m.ai_center_settings_grace_period_desc()}
            </span>
            <Input
              type="number"
              min="1"
              max="30"
              bind:value={autopilotSettings.grace_days}
              disabled={!autopilotSettings.enabled || isSaving}
              class="h-9 text-xs font-mono bg-background max-w-xs mt-1"
            />
          </div>

          <div class="flex flex-col gap-1.5 text-start">
            <Label class="text-xs font-semibold text-foreground">
              {m.ai_center_settings_lead_time_title()}
            </Label>
            <span class="text-[11px] text-muted-foreground leading-relaxed">
              {m.ai_center_settings_lead_time_desc()}
            </span>
            <Input
              type="number"
              min="1"
              max="72"
              bind:value={autopilotSettings.notify_before_deadline_hours}
              disabled={!autopilotSettings.enabled || isSaving}
              class="h-9 text-xs font-mono bg-background max-w-xs mt-1"
            />
          </div>
        </div>
      </div>
    </div>

    <!-- ── Section 2: Autonomous Workflows ────────────────────────── -->
    <div class="border border-border rounded-xl bg-card overflow-hidden shadow-xs flex flex-col">
      <div class="bg-muted/40 px-6 py-3.5 border-b border-border flex items-center gap-2 text-start">
        <Sliders size={16} class="text-primary" />
        <span class="text-xs font-bold text-foreground tracking-wide uppercase font-serif">
          {m.ai_center_settings_section_workflows()}
        </span>
      </div>

      <div class="flex flex-col divide-y divide-border/60">
        {#each workflowCategories as category}
          <div class="flex flex-col">
            <!-- Category Subheader -->
            <div class="px-6 py-2.5 bg-muted/20 text-start text-[11px] font-bold text-muted-foreground uppercase tracking-wider">
              {category.categoryTitle}
            </div>

            <!-- Category Items -->
            {#each category.items as item (item.id)}
              {@const Icon = item.icon}
              <div class="px-6 py-3.5 flex items-center justify-between gap-4 hover:bg-muted/10 transition-colors">
                <div class="flex items-start gap-3 text-start">
                  <div class="p-1.5 rounded-md bg-muted text-muted-foreground shrink-0 mt-0.5">
                    <Icon size={15} />
                  </div>
                  <div class="flex flex-col gap-0.5">
                    <div class="flex items-center gap-2">
                      <span class="text-xs font-semibold text-foreground">{item.label}</span>
                      {#if item.risk === "critical"}
                        <Badge variant="destructive" class="text-[9px] px-1.5 py-0 h-4 font-mono font-bold">
                          {m.ai_center_settings_risk_critical()}
                        </Badge>
                      {:else if item.risk === "high"}
                        <Badge class="text-[9px] px-1.5 py-0 h-4 font-mono font-bold bg-amber-500/10 text-amber-500 border-amber-500/30">
                          {m.ai_center_settings_risk_high()}
                        </Badge>
                      {/if}
                    </div>
                    <span class="text-[11px] text-muted-foreground leading-relaxed">{item.desc}</span>
                  </div>
                </div>

                <Switch
                  checked={autopilotSettings[item.id]}
                  onCheckedChange={(v) => (autopilotSettings[item.id] = v)}
                  disabled={!autopilotSettings.enabled || isSaving}
                />
              </div>
            {/each}
          </div>
        {/each}
      </div>
    </div>

    <!-- ── Section 3: Safety Gates & Risk Controls ────────────────── -->
    <div class="border border-border rounded-xl bg-card overflow-hidden shadow-xs flex flex-col">
      <div class="bg-muted/40 px-6 py-3.5 border-b border-border flex items-center gap-2 text-start">
        <ShieldAlert size={16} class="text-primary" />
        <span class="text-xs font-bold text-foreground tracking-wide uppercase font-serif">
          {m.ai_center_settings_section_safety()}
        </span>
      </div>

      <div class="flex flex-col divide-y divide-border/60">
        <!-- High Risk Gate -->
        <div class="px-6 py-4 flex items-center justify-between gap-4">
          <div class="flex flex-col gap-1 text-start">
            <div class="flex items-center gap-2">
              <span class="text-xs font-semibold text-foreground">
                {m.ai_center_settings_safety_high_risk()}
              </span>
              <Badge class="text-[9px] px-1.5 py-0 h-4 font-mono font-bold bg-amber-500/10 text-amber-500 border-amber-500/30">
                {m.ai_center_settings_risk_mandatory()}
              </Badge>
            </div>
            <span class="text-[11px] text-muted-foreground leading-relaxed">
              {m.ai_center_settings_safety_high_risk_desc()}
            </span>
          </div>
          <Switch
            checked={safetySettings.require_approval_high_risk}
            onCheckedChange={(v) => (safetySettings.require_approval_high_risk = v)}
            disabled={isSaving}
          />
        </div>

        <!-- Critical Risk Gate -->
        <div class="px-6 py-4 flex items-center justify-between gap-4">
          <div class="flex flex-col gap-1 text-start">
            <div class="flex items-center gap-2">
              <span class="text-xs font-semibold text-foreground">
                {m.ai_center_settings_safety_critical_risk()}
              </span>
              <Badge variant="destructive" class="text-[9px] px-1.5 py-0 h-4 font-mono font-bold">
                {m.ai_center_settings_risk_critical()}
              </Badge>
            </div>
            <span class="text-[11px] text-muted-foreground leading-relaxed">
              {m.ai_center_settings_safety_critical_risk_desc()}
            </span>
          </div>
          <Switch
            checked={safetySettings.require_approval_critical_risk}
            onCheckedChange={(v) => (safetySettings.require_approval_critical_risk = v)}
            disabled={isSaving}
          />
        </div>

        <!-- Block Irreversible Actions -->
        <div class="px-6 py-4 flex items-center justify-between gap-4">
          <div class="flex flex-col gap-1 text-start">
            <span class="text-xs font-semibold text-foreground">
              {m.ai_center_settings_safety_block_irreversible()}
            </span>
            <span class="text-[11px] text-muted-foreground leading-relaxed">
              {m.ai_center_settings_safety_block_irreversible_desc()}
            </span>
          </div>
          <Switch
            checked={safetySettings.block_irreversible_actions}
            onCheckedChange={(v) => (safetySettings.block_irreversible_actions = v)}
            disabled={isSaving}
          />
        </div>

        <!-- Audit Trail -->
        <div class="px-6 py-4 flex items-center justify-between gap-4">
          <div class="flex flex-col gap-1 text-start">
            <span class="text-xs font-semibold text-foreground">
              {m.ai_center_settings_safety_audit_trail()}
            </span>
            <span class="text-[11px] text-muted-foreground leading-relaxed">
              {m.ai_center_settings_safety_audit_trail_desc()}
            </span>
          </div>
          <Switch
            checked={safetySettings.audit_all_ai_actions}
            onCheckedChange={(v) => (safetySettings.audit_all_ai_actions = v)}
            disabled={isSaving}
          />
        </div>
      </div>
    </div>

    <!-- ── Section 4: Insight Engine & Anomaly Detection ─────────── -->
    <div class="border border-border rounded-xl bg-card overflow-hidden shadow-xs flex flex-col">
      <div class="bg-muted/40 px-6 py-3.5 border-b border-border flex items-center gap-2 text-start">
        <SearchCode size={16} class="text-primary" />
        <span class="text-xs font-bold text-foreground tracking-wide uppercase font-serif">
          {m.ai_center_settings_section_insight()}
        </span>
      </div>

      <div class="p-6 flex flex-col gap-6">
        <!-- Inputs Row -->
        <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
          <div class="flex flex-col gap-1.5 text-start">
            <Label class="text-xs font-semibold text-foreground">
              {m.ai_center_settings_insight_grade_drop_title()}
            </Label>
            <span class="text-[11px] text-muted-foreground leading-relaxed">
              {m.ai_center_settings_insight_grade_drop_desc()}
            </span>
            <Input
              type="number"
              min="5"
              max="50"
              bind:value={insightSettings.at_risk_grade_drop_percent}
              disabled={isSaving}
              class="h-9 text-xs font-mono bg-background mt-1"
            />
          </div>

          <div class="flex flex-col gap-1.5 text-start">
            <Label class="text-xs font-semibold text-foreground">
              {m.ai_center_settings_insight_absence_limit_title()}
            </Label>
            <span class="text-[11px] text-muted-foreground leading-relaxed">
              {m.ai_center_settings_insight_absence_limit_desc()}
            </span>
            <Input
              type="number"
              min="1"
              max="15"
              bind:value={insightSettings.at_risk_absence_limit_days}
              disabled={isSaving}
              class="h-9 text-xs font-mono bg-background mt-1"
            />
          </div>

          <div class="flex flex-col gap-1.5 text-start">
            <Label class="text-xs font-semibold text-foreground">
              {m.ai_center_settings_insight_scan_freq_title()}
            </Label>
            <span class="text-[11px] text-muted-foreground leading-relaxed">
              {m.ai_center_settings_insight_scan_freq_desc()}
            </span>
            <Input
              type="number"
              min="1"
              max="24"
              bind:value={insightSettings.periodic_scan_interval_hours}
              disabled={isSaving}
              class="h-9 text-xs font-mono bg-background mt-1"
            />
          </div>
        </div>

        <Separator class="bg-border/60" />

        <!-- Anomaly Toggles -->
        <div class="flex flex-col divide-y divide-border/40">
          <div class="py-3 flex items-center justify-between gap-4">
            <div class="flex flex-col gap-0.5 text-start">
              <span class="text-xs font-semibold text-foreground">
                {m.ai_center_settings_insight_detect_grade()}
              </span>
              <span class="text-[11px] text-muted-foreground">
                {m.ai_center_settings_insight_detect_grade_desc()}
              </span>
            </div>
            <Switch
              checked={insightSettings.detect_grade_anomalies}
              onCheckedChange={(v) => (insightSettings.detect_grade_anomalies = v)}
              disabled={isSaving}
            />
          </div>

          <div class="py-3 flex items-center justify-between gap-4">
            <div class="flex flex-col gap-0.5 text-start">
              <span class="text-xs font-semibold text-foreground">
                {m.ai_center_settings_insight_detect_attendance()}
              </span>
              <span class="text-[11px] text-muted-foreground">
                {m.ai_center_settings_insight_detect_attendance_desc()}
              </span>
            </div>
            <Switch
              checked={insightSettings.detect_attendance_anomalies}
              onCheckedChange={(v) => (insightSettings.detect_attendance_anomalies = v)}
              disabled={isSaving}
            />
          </div>
        </div>
      </div>
    </div>

    <!-- ── Section 5: Explainability & Reasoning Scope ────────────── -->
    <div class="border border-border rounded-xl bg-card overflow-hidden shadow-xs flex flex-col">
      <div class="bg-muted/40 px-6 py-3.5 border-b border-border flex items-center gap-2 text-start">
        <FileSearch size={16} class="text-primary" />
        <span class="text-xs font-bold text-foreground tracking-wide uppercase font-serif">
          {m.ai_center_settings_section_memory()}
        </span>
      </div>

      <div class="p-6 flex flex-col gap-6">
        <div class="flex flex-col divide-y divide-border/40">
          <div class="pb-4 flex items-center justify-between gap-4">
            <div class="flex flex-col gap-0.5 text-start">
              <span class="text-xs font-semibold text-foreground">
                {m.ai_center_settings_mem_reasoning_logs()}
              </span>
              <span class="text-[11px] text-muted-foreground">
                {m.ai_center_settings_mem_reasoning_logs_desc()}
              </span>
            </div>
            <Switch
              checked={memorySettings.record_reasoning_logs}
              onCheckedChange={(v) => (memorySettings.record_reasoning_logs = v)}
              disabled={isSaving}
            />
          </div>

          <div class="py-4 flex items-center justify-between gap-4">
            <div class="flex flex-col gap-0.5 text-start">
              <span class="text-xs font-semibold text-foreground">
                {m.ai_center_settings_mem_semantic()}
              </span>
              <span class="text-[11px] text-muted-foreground">
                {m.ai_center_settings_mem_semantic_desc()}
              </span>
            </div>
            <Switch
              checked={memorySettings.enable_semantic_resolution}
              onCheckedChange={(v) => (memorySettings.enable_semantic_resolution = v)}
              disabled={isSaving}
            />
          </div>

          <div class="pt-4 flex flex-col md:flex-row md:items-center justify-between gap-4">
            <div class="flex flex-col gap-0.5 text-start">
              <Label class="text-xs font-semibold text-foreground">
                {m.ai_center_settings_mem_rag_limit()}
              </Label>
              <span class="text-[11px] text-muted-foreground">
                {m.ai_center_settings_mem_rag_limit_desc()}
              </span>
            </div>
            <Input
              type="number"
              min="3"
              max="30"
              bind:value={memorySettings.rag_context_limit}
              disabled={isSaving}
              class="h-9 text-xs font-mono bg-background w-full md:w-32"
            />
          </div>
        </div>
      </div>
    </div>

    <!-- Bottom Action Bar -->
    <div class="flex items-center justify-end gap-3 pt-4 border-t border-border/40 pb-12">
      <Button
        type="submit"
        variant="default"
        size="sm"
        class="px-8 font-semibold h-10 text-xs cursor-pointer shadow-xs"
        disabled={isSaving || isLoading}
      >
        {#if isSaving}
          <Loader2 class="size-3.5 animate-spin me-1.5" />
        {/if}
        {isSaving ? m.common_state_saving() : m.settings_action_save_changes()}
      </Button>
    </div>

  </form>
</div>
