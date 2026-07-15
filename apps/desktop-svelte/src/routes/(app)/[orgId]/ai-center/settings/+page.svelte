<!--
@component SettingsPage
AI Center Settings — Real functional backend AI configuration.
Follows standard settings layout pattern (section headers, clean two-column grid, top right save action).
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
} from "lucide-svelte";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Switch } from "@orbis/ui-svelte/switch";
import { Label } from "@orbis/ui-svelte/label";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
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

onMount(async () => {
  try {
    const config = await commands.aiCenter.getAutopilotConfig(data.orgId);
    if (config) Object.assign(autopilotSettings, config);
  } catch (e) {
    console.warn("[AI Center] getAutopilotConfig not available:", e);
  }
});

async function saveAllSettings() {
  isSaving = true;
  try {
    await commands.aiCenter.saveAutopilotConfig(data.orgId, autopilotSettings);
    toast.success(
      m.settings_general_save_success
        ? m.settings_general_save_success()
        : "تم حفظ كافة إعدادات الذكاء الاصطناعي بنجاح",
    );
  } catch (e) {
    console.warn("[AI Center] save error:", e);
    toast.error("فشل حفظ بعض الإعدادات، يرجى إعادة المحاولة.");
  } finally {
    isSaving = false;
  }
}

const autopilotToggles = [
  { id: "auto_open_enrollment", label: "فتح التسجيل التلقائي", desc: "فتح التسجيل فور بداية الفترة الأكاديمية" },
  { id: "auto_close_enrollment", label: "إغلاق التسجيل عند الموعد", desc: "إغلاق استقبال طلبات التسجيل فور استيفاء الموعد" },
  { id: "auto_record_attendance", label: "الرصد التلقائي للحضور", desc: "الرصد المباشر عبر البصمة وكاميرات المراقبة" },
  { id: "auto_fee_assignment", label: "تعيين الرسوم التلقائي", desc: "ربط القوائم المالية بالطلاب الجدد تلقائياً" },
  { id: "auto_grade_entry_reminder", label: "تنبيهات إدخال الدرجات", desc: "تذكير المعلمين بمواعيد تسليم التقييمات" },
  { id: "auto_period_lock", label: "إغلاق الفترات الأكاديمية", desc: "تأمين الفترة الأكاديمية وحظر التعديل عند الانتهاء (حرج)" },
  { id: "auto_promotion", label: "ترفيع الطلاب السنوي", desc: "تشغيل أتمتة نقل وترفيع الناجحين بنهاية العام (حرج)" },
  { id: "auto_exam_scheduling", label: "جدولة الاختبارات تلقائياً", desc: "توليد جدول الاختبارات ومنع التعارضات" },
  { id: "auto_report_generation", label: "إصدار التقارير التلقائي", desc: "توليد كشوفات الأداء فور إغلاق الفترة" },
] as const;

const modeLabels = $derived<Record<string, string>>({
  automatic: m.ai_center_mode_automatic ? m.ai_center_mode_automatic() : "تنفيذ تلقائي مباشر",
  suggested: m.ai_center_mode_suggested ? m.ai_center_mode_suggested() : "توصيات وموافقة سريعة",
  manual: m.ai_center_mode_manual ? m.ai_center_mode_manual() : "يدوي فقط",
});
</script>

<svelte:head>
  <title>{m.ai_center_settings_title ? m.ai_center_settings_title() : "إعدادات الذكاء الاصطناعي"} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-6 scrollbar-hide bg-background font-sans">
  <form onsubmit={(e) => { e.preventDefault(); saveAllSettings(); }} class="flex flex-col gap-8 w-full">
    <!-- Top Action Bar matching standard system settings layout -->
    <div class="flex items-center justify-end gap-3 pb-5 border-b border-border/40 w-full select-none">
      <Button
        type="submit"
        variant="default"
        size="sm"
        class="px-6 font-semibold h-9 text-xs cursor-pointer"
        disabled={isSaving}
      >
        {#if isSaving}
          <Loader2 class="size-3.5 animate-spin me-1.5" />
        {/if}
        {isSaving ? (m.common_state_saving ? m.common_state_saving() : "جاري الحفظ...") : (m.settings_action_save_changes ? m.settings_action_save_changes() : "حفظ التغييرات")}
      </Button>
    </div>

    <!-- Two-Column Grid Layout for Settings Sections -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-10">

      <!-- 1. Autopilot Engine Automation Section -->
      <div class="flex flex-col gap-6 text-start">
        <div class="border-b border-border/60 pb-3 flex items-center gap-2">
          <Zap size={16} class="text-primary" />
          <h2 class="text-sm font-semibold text-foreground">
            أتمتة محرك الطيار الآلي (Autopilot Engine)
          </h2>
        </div>

        <div class="flex flex-col gap-4">
          <!-- Enabled Switch -->
          <div class="flex items-center justify-between py-2 border-b border-border/20">
            <div class="flex flex-col text-start gap-0.5">
              <span class="text-xs font-semibold text-foreground">تفعيل الطيار الآلي</span>
              <span class="text-[10px] text-muted-foreground">تشغيل محرك الأتمتة والعمليات التلقائية في الخلفية.</span>
            </div>
            <Switch checked={autopilotSettings.enabled} onCheckedChange={(v) => (autopilotSettings.enabled = v)} disabled={isSaving} />
          </div>

          {#if autopilotSettings.enabled}
            <!-- Execution Mode Select -->
            <div class="flex flex-col gap-2 py-1">
              <Label class="text-xs text-muted-foreground block font-medium">نمط التنفيذ التشغيلي</Label>
              <Select
                type="single"
                value={autopilotSettings.execution_mode}
                onValueChange={(v) => { if (v) autopilotSettings.execution_mode = v as any; }}
              >
                <SelectTrigger class="text-xs h-9 bg-background border-border/60">
                  {modeLabels[autopilotSettings.execution_mode] ?? autopilotSettings.execution_mode}
                </SelectTrigger>
                <SelectContent class="text-xs">
                  <SelectItem value="automatic">{modeLabels.automatic}</SelectItem>
                  <SelectItem value="suggested">{modeLabels.suggested}</SelectItem>
                  <SelectItem value="manual">{modeLabels.manual}</SelectItem>
                </SelectContent>
              </Select>
            </div>

            <div class="grid grid-cols-2 gap-4">
              <!-- Grace Days -->
              <div class="flex flex-col gap-2">
                <Label class="text-xs text-muted-foreground block font-medium">فترة مهلة الاعتماد (أيام)</Label>
                <Input type="number" min="1" max="30" bind:value={autopilotSettings.grace_days} disabled={isSaving} class="text-xs font-mono bg-background" />
              </div>
              <!-- Advance Notification Hours -->
              <div class="flex flex-col gap-2">
                <Label class="text-xs text-muted-foreground block font-medium">الإشعار المبكر (ساعات)</Label>
                <Input type="number" min="1" max="72" bind:value={autopilotSettings.notify_before_deadline_hours} disabled={isSaving} class="text-xs font-mono bg-background" />
              </div>
            </div>

            <Separator class="bg-border/40" />

            <!-- Sub-Toggles -->
            {#each autopilotToggles as toggle (toggle.id)}
              <div class="flex items-center justify-between py-2 border-b border-border/20 last:border-b-0">
                <div class="flex flex-col text-start gap-0.5">
                  <span class="text-xs font-semibold text-foreground">{toggle.label}</span>
                  <span class="text-[10px] text-muted-foreground">{toggle.desc}</span>
                </div>
                <Switch checked={autopilotSettings[toggle.id]} onCheckedChange={(v) => (autopilotSettings[toggle.id] = v)} disabled={isSaving} />
              </div>
            {/each}
          {/if}
        </div>
      </div>

      <!-- 2. Risk Matrix & Safety Gates Section -->
      <div class="flex flex-col gap-6 text-start">
        <div class="border-b border-border/60 pb-3 flex items-center gap-2">
          <ShieldAlert size={16} class="text-primary" />
          <h2 class="text-sm font-semibold text-foreground">
            مصفوفة المخاطر وبوابات الأمان (Risk Gates & Safety)
          </h2>
        </div>

        <div class="flex flex-col gap-4">
          <!-- Require Approval High Risk -->
          <div class="flex items-center justify-between py-2 border-b border-border/20">
            <div class="flex flex-col text-start gap-0.5">
              <span class="text-xs font-semibold text-foreground">اعتماد قرارات المخاطر العالية (High Risk)</span>
              <span class="text-[10px] text-muted-foreground">إلزام مرور القرارات المالية وتعديل التقييمات على صندوق التثبت.</span>
            </div>
            <Switch checked={safetySettings.require_approval_high_risk} onCheckedChange={(v) => (safetySettings.require_approval_high_risk = v)} disabled={isSaving} />
          </div>

          <!-- Require Approval Critical Risk -->
          <div class="flex items-center justify-between py-2 border-b border-border/20">
            <div class="flex flex-col text-start gap-0.5">
              <span class="text-xs font-semibold text-foreground">اعتماد قرارات المخاطر الحرجة (Critical Risk)</span>
              <span class="text-[10px] text-muted-foreground">إلزام الاعتماد اليدوي المزدوج فور تنفيذ ترفيع الطلاب أو قفل الفترات.</span>
            </div>
            <Switch checked={safetySettings.require_approval_critical_risk} onCheckedChange={(v) => (safetySettings.require_approval_critical_risk = v)} disabled={isSaving} />
          </div>

          <!-- Block Irreversible Actions -->
          <div class="flex items-center justify-between py-2 border-b border-border/20">
            <div class="flex flex-col text-start gap-0.5">
              <span class="text-xs font-semibold text-foreground">حظر التنفيذ التلقائي للعمليات غير القابلة للتراجع</span>
              <span class="text-[10px] text-muted-foreground">منع الأتمتة المباشرة للقرارات الدائمة التي لا يمكن التراجع عنها.</span>
            </div>
            <Switch checked={safetySettings.block_irreversible_actions} onCheckedChange={(v) => (safetySettings.block_irreversible_actions = v)} disabled={isSaving} />
          </div>

          <!-- Audit Trail -->
          <div class="flex items-center justify-between py-2">
            <div class="flex flex-col text-start gap-0.5">
              <span class="text-xs font-semibold text-foreground">التوثيق الجنائي لكافة أوامر الوكلاء (Audit Trail)</span>
              <span class="text-[10px] text-muted-foreground">حفظ وحماية كافة السجلات والـ Payloads المتبادلة في سجل الأمان.</span>
            </div>
            <Switch checked={safetySettings.audit_all_ai_actions} onCheckedChange={(v) => (safetySettings.audit_all_ai_actions = v)} disabled={isSaving} />
          </div>
        </div>
      </div>

      <!-- 3. Insight Engine & Anomaly Detection Section -->
      <div class="flex flex-col gap-6 text-start">
        <div class="border-b border-border/60 pb-3 flex items-center gap-2">
          <SearchCode size={16} class="text-primary" />
          <h2 class="text-sm font-semibold text-foreground">
            محرك الرصد والكشف التلقائي للشواذ (Insight Engine)
          </h2>
        </div>

        <div class="flex flex-col gap-5">
          <div class="grid grid-cols-2 gap-4">
            <div class="flex flex-col gap-2">
              <Label class="text-xs text-muted-foreground block font-medium">نسبة هبوط الدرجات الحرجة (%)</Label>
              <Input type="number" min="5" max="50" bind:value={insightSettings.at_risk_grade_drop_percent} disabled={isSaving} class="text-xs font-mono bg-background" />
            </div>
            <div class="flex flex-col gap-2">
              <Label class="text-xs text-muted-foreground block font-medium">حد غياب الأيام المتتالي (أيام)</Label>
              <Input type="number" min="1" max="15" bind:value={insightSettings.at_risk_absence_limit_days} disabled={isSaving} class="text-xs font-mono bg-background" />
            </div>
          </div>

          <Separator class="bg-border/40" />

          <div class="flex items-center justify-between py-2 border-b border-border/20">
            <div class="flex flex-col text-start gap-0.5">
              <span class="text-xs font-semibold text-foreground">رصد شواذ الدرجات والتقييمات التلقائي</span>
              <span class="text-[10px] text-muted-foreground">اكتشاف التقييمات المتشابهة أو غير المنطقية فور إدخالها.</span>
            </div>
            <Switch checked={insightSettings.detect_grade_anomalies} onCheckedChange={(v) => (insightSettings.detect_grade_anomalies = v)} disabled={isSaving} />
          </div>

          <div class="flex items-center justify-between py-2 border-b border-border/20">
            <div class="flex flex-col text-start gap-0.5">
              <span class="text-xs font-semibold text-foreground">رصد شواذ وانخفاضات الحضور الفجائية</span>
              <span class="text-[10px] text-muted-foreground">توليد إنذار مبكر فور انخفاض متوسط حضور الشعبة أو الطالب.</span>
            </div>
            <Switch checked={insightSettings.detect_attendance_anomalies} onCheckedChange={(v) => (insightSettings.detect_attendance_anomalies = v)} disabled={isSaving} />
          </div>

          <div class="flex flex-col gap-2">
            <Label class="text-xs text-muted-foreground block font-medium">تكرار المسح والتحليل الدوري للنظام (ساعات)</Label>
            <Input type="number" min="1" max="24" bind:value={insightSettings.periodic_scan_interval_hours} disabled={isSaving} class="text-xs font-mono bg-background" />
          </div>
        </div>
      </div>

      <!-- 4. Explainability & RAG Scope Section -->
      <div class="flex flex-col gap-6 text-start">
        <div class="border-b border-border/60 pb-3 flex items-center gap-2">
          <FileSearch size={16} class="text-primary" />
          <h2 class="text-sm font-semibold text-foreground">
            سياق التعليل وتثبيت الذاكرة (Explainability & Memory)
          </h2>
        </div>

        <div class="flex flex-col gap-5">
          <div class="flex items-center justify-between py-2 border-b border-border/20">
            <div class="flex flex-col text-start gap-0.5">
              <span class="text-xs font-semibold text-foreground">تسجيل التعليل المنطقي المفصل للوكلاء</span>
              <span class="text-[10px] text-muted-foreground">حفظ خطوات التفكير والتحليل المنطقي للقرار في سجل التفاصيل.</span>
            </div>
            <Switch checked={memorySettings.record_reasoning_logs} onCheckedChange={(v) => (memorySettings.record_reasoning_logs = v)} disabled={isSaving} />
          </div>

          <div class="flex items-center justify-between py-2 border-b border-border/20">
            <div class="flex flex-col text-start gap-0.5">
              <span class="text-xs font-semibold text-foreground">تفعيل المطابقة والبحث الدلالي (Semantic Resolution)</span>
              <span class="text-[10px] text-muted-foreground">استخدام محرك الفهرسة الدلالية لمطابقة أسماء الطلاب والصفوف.</span>
            </div>
            <Switch checked={memorySettings.enable_semantic_resolution} onCheckedChange={(v) => (memorySettings.enable_semantic_resolution = v)} disabled={isSaving} />
          </div>

          <div class="flex flex-col gap-2">
            <Label class="text-xs text-muted-foreground block font-medium">أقصى حد لاسترجاع الفقرات الأكاديمية (RAG Context Limit)</Label>
            <Input type="number" min="3" max="30" bind:value={memorySettings.rag_context_limit} disabled={isSaving} class="text-xs font-mono bg-background" />
          </div>
        </div>
      </div>

    </div>
  </form>
</div>
