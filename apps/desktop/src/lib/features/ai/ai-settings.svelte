<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import {
  Bolt,
  BookOpen,
  Calendar,
  ChartBar,
  CheckCircle,
  ClockRotateRight,
  Document,
  DocumentPage,
  Keyframes,
  Lock,
  Search,
  Sparkles,
  UserBadgeCheck,
} from "@orbis/icons";
import { Input } from "@orbis/ui/input";
import { Switch } from "@orbis/ui/switch";
import { Label } from "@orbis/ui/label";
import { Badge } from "@orbis/ui/badge";
import { Separator } from "@orbis/ui/separator";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
  CardAction,
} from "@orbis/ui/card";
import { toast } from "@orbis/ui";
import { api, queryKeys, type OrganizationSettingsDto } from "$lib/api";
import { useQueryClient } from "@tanstack/svelte-query";
import SettingsBottomBar from "$lib/shared/components/settings-bottom-bar.svelte";
import SettingsFormSkeleton from "$lib/shared/skeletons/settings-form-skeleton.svelte";

let { orgId }: { orgId: string } = $props();

const queryClient = useQueryClient();
let rawSettings = $state<OrganizationSettingsDto | null>(null);

// ── 1. Autopilot Engine Settings (All 18 Backend Fields) ───────
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
  auto_advance_periods: false,
  auto_lock_grades: false,
  allow_grade_edit_after_close: false,
  grace_days: 3,
  notify_before_deadline_hours: 24,
  max_retries: 3,
  grade_entry_auto_close_days: 7,
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
let initialSnapshot = $state("");

onMount(async () => {
  try {
    const s = await api.getSettings();
    rawSettings = s;
    if (s.autopilot) {
      autopilotSettings = {
        enabled: s.autopilot.enabled,
        execution_mode:
          (s.autopilot.mode as "automatic" | "suggested" | "manual") ||
          "suggested",
        auto_open_enrollment: s.autopilot.auto_open_enrollment,
        auto_close_enrollment: s.autopilot.auto_close_enrollment,
        auto_record_attendance: s.autopilot.auto_record_attendance,
        auto_fee_assignment: s.autopilot.auto_fee_assignment,
        auto_grade_entry_reminder: s.autopilot.auto_grade_entry_reminder,
        auto_period_lock: s.autopilot.auto_period_lock,
        auto_promotion: s.autopilot.auto_promotion,
        auto_exam_scheduling: s.autopilot.auto_exam_scheduling,
        auto_report_generation: s.autopilot.auto_report_generation,
        auto_advance_periods: s.autopilot.auto_advance_periods ?? false,
        auto_lock_grades: s.autopilot.auto_lock_grades ?? false,
        allow_grade_edit_after_close:
          s.autopilot.allow_grade_edit_after_close ?? false,
        grace_days: s.autopilot.grace_days,
        notify_before_deadline_hours: s.autopilot.notify_before_deadline_hours,
        max_retries: s.autopilot.max_retries ?? 3,
        grade_entry_auto_close_days:
          s.autopilot.grade_entry_auto_close_days ?? 7,
      };
    }
  } catch (err) {
    console.error("Failed to load settings in AI center:", err);
  } finally {
    initialSnapshot = JSON.stringify({
      autopilot: autopilotSettings,
      safety: safetySettings,
      insight: insightSettings,
      memory: memorySettings,
    });
    isLoading = false;
  }
});

const currentSnapshot = $derived(
  JSON.stringify({
    autopilot: autopilotSettings,
    safety: safetySettings,
    insight: insightSettings,
    memory: memorySettings,
  }),
);

const isDirty = $derived(
  Boolean(initialSnapshot && currentSnapshot !== initialSnapshot),
);

function resetToInitial() {
  if (!initialSnapshot) return;
  try {
    const parsed = JSON.parse(initialSnapshot);
    Object.assign(autopilotSettings, parsed.autopilot);
    Object.assign(safetySettings, parsed.safety);
    Object.assign(insightSettings, parsed.insight);
    Object.assign(memorySettings, parsed.memory);
  } catch (e) {
    console.warn("Reset error:", e);
  }
}

async function saveAllSettings() {
  isSaving = true;
  try {
    const current = rawSettings ?? (await api.getSettings());
    const updated = await api.updateSettings({
      ...current,
      autopilot: {
        enabled: autopilotSettings.enabled,
        mode: autopilotSettings.execution_mode,
        auto_open_enrollment: autopilotSettings.auto_open_enrollment,
        auto_close_enrollment: autopilotSettings.auto_close_enrollment,
        auto_record_attendance: autopilotSettings.auto_record_attendance,
        auto_fee_assignment: autopilotSettings.auto_fee_assignment,
        auto_grade_entry_reminder: autopilotSettings.auto_grade_entry_reminder,
        auto_period_lock: autopilotSettings.auto_period_lock,
        auto_promotion: autopilotSettings.auto_promotion,
        auto_exam_scheduling: autopilotSettings.auto_exam_scheduling,
        auto_report_generation: autopilotSettings.auto_report_generation,
        auto_advance_periods: autopilotSettings.auto_advance_periods,
        auto_lock_grades: autopilotSettings.auto_lock_grades,
        allow_grade_edit_after_close:
          autopilotSettings.allow_grade_edit_after_close,
        grace_days: Number(autopilotSettings.grace_days),
        notify_before_deadline_hours: Number(
          autopilotSettings.notify_before_deadline_hours,
        ),
        max_retries: Number(autopilotSettings.max_retries),
        grade_entry_auto_close_days: Number(
          autopilotSettings.grade_entry_auto_close_days,
        ),
      },
    });
    rawSettings = updated;
    initialSnapshot = currentSnapshot;
    queryClient.invalidateQueries({ queryKey: queryKeys.org.settings() });
    queryClient.invalidateQueries({ queryKey: queryKeys.autopilot.all });
    toast.success(m.ai_center_settings_save_success());
  } catch (err) {
    toast.error(
      m.ai_center_settings_save_error?.() ||
        "Failed to save AI Center settings",
    );
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
    icon: Bolt,
  },
  {
    id: "manual",
    title: m.ai_center_settings_mode_manual_title(),
    subtitle: m.ai_center_settings_mode_manual_sub(),
    desc: m.ai_center_settings_mode_manual_desc(),
    icon: Lock,
  },
]);

// Categorized Workflows using Design Tokens
const workflowCategories = $derived([
  {
    categoryTitle: m.ai_center_settings_cat_admissions(),
    items: [
      {
        id: "auto_open_enrollment" as const,
        label: m.ai_center_settings_wf_open_enrollment(),
        desc: m.ai_center_settings_wf_open_enrollment_desc(),
        icon: UserBadgeCheck,
        risk: "low",
      },
      {
        id: "auto_close_enrollment" as const,
        label: m.ai_center_settings_wf_close_enrollment(),
        desc: m.ai_center_settings_wf_close_enrollment_desc(),
        icon: ClockRotateRight,
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
        icon: ChartBar,
        risk: "medium",
      },
      {
        id: "auto_grade_entry_reminder" as const,
        label: m.ai_center_settings_wf_grade_reminders(),
        desc: m.ai_center_settings_wf_grade_reminders_desc(),
        icon: DocumentPage,
        risk: "low",
      },
      {
        id: "auto_lock_grades" as const,
        label: "قفل دفتر الدرجات تلقائياً",
        desc: "قفل التعديل على سجلات الدرجات تلقائياً بعد انقضاء مهلة الرصد المحددة",
        icon: Lock,
        risk: "medium",
      },
      {
        id: "allow_grade_edit_after_close" as const,
        label: "السماح بتعديل الدرجات بعد القفل",
        desc: "إمكانية فتح الدرجات للتعديل بتفويض خاص بعد انتهاء مهلة الرصد الرسمية",
        icon: DocumentPage,
        risk: "high",
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
        icon: BookOpen,
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
        icon: Document,
        risk: "high",
      },
      {
        id: "auto_advance_periods" as const,
        label: "الانتقال الآلي للفترات الأكاديمية",
        desc: "تفعيل الفترة الأكاديمية التالية تلقائياً عند حلول موعد بدء الفصل الدراسي",
        icon: Calendar,
        risk: "medium",
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
        icon: Keyframes,
        risk: "critical",
      },
    ],
  },
]);
</script>

{#if isLoading}
  <div class="h-full w-full p-6">
    <SettingsFormSkeleton />
  </div>
{:else}
  <div class="h-full w-full overflow-y-auto p-6 pb-32 flex flex-col gap-6 scrollbar-hide font-sans text-start">
    <form onsubmit={(e) => { e.preventDefault(); saveAllSettings(); }} class="flex flex-col gap-6 w-full">

      <!-- ── Section 1: Autopilot Operational Mode ─────────────────── -->
      <Card>
        <CardHeader>
          <div class="flex items-center gap-2 text-start">
            <Bolt size={14} class="text-primary" />
            <CardTitle>
              {m.ai_center_settings_section_autopilot_mode()}
            </CardTitle>
          </div>
          <CardAction>
            <div class="flex items-center gap-2.5">
              <span class="text-xs font-medium text-muted-foreground">
                {autopilotSettings.enabled ? m.ai_center_settings_autopilot_active() : m.ai_center_settings_autopilot_disabled()}
              </span>
              <Switch
                checked={autopilotSettings.enabled}
                onCheckedChange={(v) => (autopilotSettings.enabled = v)}
                disabled={isSaving}
              />
            </div>
          </CardAction>
        </CardHeader>

        <CardContent class="p-6 flex flex-col gap-6">
          <!-- Interactive Mode Cards -->
          <div class="flex flex-col gap-1 text-start">
            <Label class="text-xs font-semibold text-foreground">
              {m.ai_center_settings_mode_authority_title()}
            </Label>
            <span class="text-[11px] text-muted-foreground leading-relaxed">
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
                class="flex flex-col text-start p-4 rounded-xl border transition-all relative cursor-pointer {isSelected ? 'border-primary bg-primary/5 shadow-xs ring-1 ring-primary/30' : 'border-border bg-muted/20 hover:border-border/80 hover:bg-muted/40'} {!autopilotSettings.enabled ? 'opacity-50 cursor-not-allowed' : ''}"
              >
                <div class="flex items-center justify-between w-full mb-3">
                  <div class="p-2 rounded-lg {isSelected ? 'bg-primary/20 text-primary' : 'bg-muted text-muted-foreground'}">
                    <Icon size={16} />
                  </div>
                  {#if isSelected}
                    <span class="flex items-center gap-1 text-[11px] font-bold text-primary">
                      <CheckCircle size={13} />
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
          <div class="grid grid-cols-1 md:grid-cols-4 gap-4 pt-4 border-t border-border">
            <div class="flex flex-col gap-1.5 text-start">
              <Label class="text-xs font-semibold text-foreground">
                {m.ai_center_settings_grace_period_title()}
              </Label>
              <span class="text-[11px] text-muted-foreground leading-relaxed">
                {m.ai_center_settings_grace_period_desc()}
              </span>
              <Input
                type="number"
                min="0"
                max="30"
                bind:value={autopilotSettings.grace_days}
                disabled={!autopilotSettings.enabled || isSaving}
                class="h-9 text-xs font-mono bg-background border-border mt-1"
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
                max="168"
                bind:value={autopilotSettings.notify_before_deadline_hours}
                disabled={!autopilotSettings.enabled || isSaving}
                class="h-9 text-xs font-mono bg-background border-border mt-1"
              />
            </div>

            <div class="flex flex-col gap-1.5 text-start">
              <Label class="text-xs font-semibold text-foreground">
                مهلة رصد الدرجات (أيام)
              </Label>
              <span class="text-[11px] text-muted-foreground leading-relaxed">
                المهلة الزمنية المتاحة للمعلمين لإدخال الدرجات قبل الإغلاق
              </span>
              <Input
                type="number"
                min="1"
                max="60"
                bind:value={autopilotSettings.grade_entry_auto_close_days}
                disabled={!autopilotSettings.enabled || isSaving}
                class="h-9 text-xs font-mono bg-background border-border mt-1"
              />
            </div>

            <div class="flex flex-col gap-1.5 text-start">
              <Label class="text-xs font-semibold text-foreground">
                الحد الأقصى للمحاولات
              </Label>
              <span class="text-[11px] text-muted-foreground leading-relaxed">
                عدد محاولات إعادة تنفيذ المهام الفاشلة قبل إرسال تنبيه للمسؤول
              </span>
              <Input
                type="number"
                min="1"
                max="10"
                bind:value={autopilotSettings.max_retries}
                disabled={!autopilotSettings.enabled || isSaving}
                class="h-9 text-xs font-mono bg-background border-border mt-1"
              />
            </div>
          </div>
        </CardContent>
      </Card>

      <!-- ── Section 2: Autonomous Workflows ────────────────────────── -->
      <Card>
        <CardHeader>
          <div class="flex items-center gap-2 text-start">
            <Sparkles size={14} class="text-primary" />
            <CardTitle>
              {m.ai_center_settings_section_workflows()}
            </CardTitle>
          </div>
        </CardHeader>

        <CardContent class="p-0">
          <div class="flex flex-col divide-y divide-border">
            {#each workflowCategories as category}
              <div class="flex flex-col">
                <!-- Category Subheader -->
                <div class="px-6 py-2.5 bg-muted/30 text-start text-[10px] font-bold text-muted-foreground uppercase tracking-wider border-b border-border">
                  {category.categoryTitle}
                </div>

                <!-- Category Items -->
                {#each category.items as item (item.id)}
                  {@const Icon = item.icon}
                  <div class="px-6 py-3.5 flex items-center justify-between gap-4 hover:bg-muted/10 transition-colors border-b border-border last:border-b-0">
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
        </CardContent>
      </Card>

      <!-- ── Section 3: Safety Gates & Risk Controls ────────────────── -->
      <Card>
        <CardHeader>
          <div class="flex items-center gap-2 text-start">
            <UserBadgeCheck size={14} class="text-primary" />
            <CardTitle>
              {m.ai_center_settings_section_safety()}
            </CardTitle>
          </div>
        </CardHeader>

        <CardContent class="p-0">
          <div class="flex flex-col divide-y divide-border">
            <!-- High Risk Gate -->
            <div class="px-6 py-4 flex items-center justify-between gap-4 hover:bg-muted/10 transition-colors">
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
            <div class="px-6 py-4 flex items-center justify-between gap-4 hover:bg-muted/10 transition-colors">
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
            <div class="px-6 py-4 flex items-center justify-between gap-4 hover:bg-muted/10 transition-colors">
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
            <div class="px-6 py-4 flex items-center justify-between gap-4 hover:bg-muted/10 transition-colors">
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
        </CardContent>
      </Card>

      <!-- ── Section 4: Insight Engine & Anomaly Detection ─────────── -->
      <Card>
        <CardHeader>
          <div class="flex items-center gap-2 text-start">
            <Search size={14} class="text-primary" />
            <CardTitle>
              {m.ai_center_settings_section_insight()}
            </CardTitle>
          </div>
        </CardHeader>

        <CardContent class="p-6 flex flex-col gap-6">
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
                class="h-9 text-xs font-mono bg-background border-border mt-1"
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
                class="h-9 text-xs font-mono bg-background border-border mt-1"
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
                class="h-9 text-xs font-mono bg-background border-border mt-1"
              />
            </div>
          </div>

          <Separator class="bg-border" />

          <!-- Anomaly Toggles -->
          <div class="flex flex-col divide-y divide-border">
            <div class="py-3.5 flex items-center justify-between gap-4">
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

            <div class="py-3.5 flex items-center justify-between gap-4">
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
        </CardContent>
      </Card>

      <!-- ── Section 5: Explainability & Reasoning Scope ────────────── -->
      <Card>
        <CardHeader>
          <div class="flex items-center gap-2 text-start">
            <Search size={14} class="text-primary" />
            <CardTitle>
              {m.ai_center_settings_section_memory()}
            </CardTitle>
          </div>
        </CardHeader>

        <CardContent class="p-6 flex flex-col gap-6">
          <div class="flex flex-col divide-y divide-border">
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
                class="h-9 text-xs font-mono bg-background border-border w-full md:w-32"
              />
            </div>
          </div>
        </CardContent>
      </Card>

    </form>

    <!-- Reusable Floating Settings Bottom Bar -->
    <SettingsBottomBar
      visible={isDirty}
      {isSaving}
      {isDirty}
      onsave={saveAllSettings}
      onreset={resetToInitial}
    />
  </div>
{/if}
