<!--
  @component AutopilotSettingsCard
  Configures Autonomous Operations Engine & Autopilot Rules (all 17 backend fields).
-->
<script lang="ts">
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from "@orbis/ui/card";
import { Label } from "@orbis/ui/label";
import { Switch } from "@orbis/ui/switch";
import { Input } from "@orbis/ui/input";
import { Select, SelectTrigger, SelectContent, SelectItem } from "@orbis/ui/select";
import { Sparkles } from "@orbis/icons";
import { l } from "$lib/core/i18n";

let {
  enabled = $bindable(true),
  mode = $bindable("suggested"),
  autoOpenEnrollment = $bindable(false),
  autoCloseEnrollment = $bindable(true),
  autoRecordAttendance = $bindable(true),
  autoFeeAssignment = $bindable(true),
  autoGradeEntryReminder = $bindable(true),
  autoPeriodLock = $bindable(false),
  autoPromotion = $bindable(false),
  autoExamScheduling = $bindable(false),
  autoReportGeneration = $bindable(true),
  graceDays = $bindable(3),
  notifyBeforeDeadlineHours = $bindable(24),
  maxRetries = $bindable(3),
  autoAdvancePeriods = $bindable(false),
  autoLockGrades = $bindable(false),
  gradeEntryAutoCloseDays = $bindable(7),
  allowGradeEditAfterClose = $bindable(false),
  isSaving = false,
}: {
  enabled?: boolean;
  mode?: string;
  autoOpenEnrollment?: boolean;
  autoCloseEnrollment?: boolean;
  autoRecordAttendance?: boolean;
  autoFeeAssignment?: boolean;
  autoGradeEntryReminder?: boolean;
  autoPeriodLock?: boolean;
  autoPromotion?: boolean;
  autoExamScheduling?: boolean;
  autoReportGeneration?: boolean;
  graceDays?: number;
  notifyBeforeDeadlineHours?: number;
  maxRetries?: number;
  autoAdvancePeriods?: boolean;
  autoLockGrades?: boolean;
  gradeEntryAutoCloseDays?: number;
  allowGradeEditAfterClose?: boolean;
  isSaving?: boolean;
} = $props();
</script>

<Card class="border-border/80 shadow-xs">
  <CardHeader>
    <div class="flex items-center gap-3 text-start">
      <div class="w-8 h-8 rounded-lg bg-primary/10 border border-primary/20 flex items-center justify-center text-primary shrink-0">
        <Sparkles class="w-4 h-4" />
      </div>
      <div class="flex flex-col">
        <CardTitle>{l("إعدادات محرك التشغيل الذاتي (Autopilot)", "Autopilot Operations Engine")}</CardTitle>
        <CardDescription>{l("أتمتة العمليات الأكاديمية والمالية والإشعارات دون تدخل يدوي", "Autonomous event triggers, approval automation, and proactive task workflows")}</CardDescription>
      </div>
    </div>
  </CardHeader>
  <CardContent class="flex flex-col gap-6">
    <!-- Master Switch & Mode -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-5 p-4 rounded-xl border border-primary/20 bg-primary/5">
      <div class="flex items-center justify-between">
        <div class="flex flex-col text-start">
          <Label class="text-xs font-bold text-foreground">{l("تفعيل المحرك الذاتي العام", "Enable Autopilot Master")}</Label>
          <span class="text-[11px] text-muted-foreground">{l("تشغيل المهام المجدولة والأتمتة", "Master switch for scheduled tasks")}</span>
        </div>
        <Switch bind:checked={enabled} disabled={isSaving} />
      </div>

      <div class="flex flex-col gap-2 text-start">
        <Label class="text-xs font-bold text-foreground">{l("نمط التنفيذ", "Execution Mode")}</Label>
        <Select bind:value={mode} disabled={isSaving || !enabled}>
          <SelectTrigger class="h-8 text-xs w-56">
            <span>
              {mode === "automatic" ? l("تلقائي كامل (Autonomous)", "Full Automatic") : mode === "suggested" ? l("اقتراح مع موافقة (Suggested)", "Suggested with Approval") : l("يدوي فقط (Manual)", "Manual Only")}
            </span>
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="automatic">{l("تلقائي كامل (Autonomous)", "Full Automatic")}</SelectItem>
            <SelectItem value="suggested">{l("اقتراح مع موافقة (Suggested)", "Suggested with Approval")}</SelectItem>
            <SelectItem value="manual">{l("يدوي فقط (Manual)", "Manual Only")}</SelectItem>
          </SelectContent>
        </Select>
      </div>
    </div>

    <!-- Automated Workflows Switches -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <div class="flex items-center justify-between p-3 rounded-lg border border-border/50 bg-muted/20">
        <div class="flex flex-col text-start">
          <Label class="text-xs font-semibold">{l("إغلاق القيد تلقائياً", "Auto Close Enrollment")}</Label>
          <span class="text-[10px] text-muted-foreground">{l("إغلاق القيد عند موعد نهاية فترة التسجيل", "Close registration on end date")}</span>
        </div>
        <Switch bind:checked={autoCloseEnrollment} disabled={isSaving || !enabled} />
      </div>

      <div class="flex items-center justify-between p-3 rounded-lg border border-border/50 bg-muted/20">
        <div class="flex flex-col text-start">
          <Label class="text-xs font-semibold">{l("تعيين الرسوم تلقائياً", "Auto Fee Assignment")}</Label>
          <span class="text-[10px] text-muted-foreground">{l("تطبيق قوالب الرسوم آلياً عند اعتماد قيد الطالب", "Apply fee templates on enrollment approval")}</span>
        </div>
        <Switch bind:checked={autoFeeAssignment} disabled={isSaving || !enabled} />
      </div>

      <div class="flex items-center justify-between p-3 rounded-lg border border-border/50 bg-muted/20">
        <div class="flex flex-col text-start">
          <Label class="text-xs font-semibold">{l("تذكير رصد الدرجات", "Auto Grade Reminder")}</Label>
          <span class="text-[10px] text-muted-foreground">{l("إشعار المعلمين قبل موعد إغلاق دفتر الدرجات", "Notify teachers before grade lock")}</span>
        </div>
        <Switch bind:checked={autoGradeEntryReminder} disabled={isSaving || !enabled} />
      </div>

      <div class="flex items-center justify-between p-3 rounded-lg border border-border/50 bg-muted/20">
        <div class="flex flex-col text-start">
          <Label class="text-xs font-semibold">{l("قفل الدرجات تلقائياً", "Auto Lock Grades")}</Label>
          <span class="text-[10px] text-muted-foreground">{l("قفل التعديل على الدرجات بعد انتهاء المهلة", "Lock grade entries after deadline")}</span>
        </div>
        <Switch bind:checked={autoLockGrades} disabled={isSaving || !enabled} />
      </div>

      <div class="flex items-center justify-between p-3 rounded-lg border border-border/50 bg-muted/20">
        <div class="flex flex-col text-start">
          <Label class="text-xs font-semibold">{l("توليد التقارير دورياً", "Auto Report Generation")}</Label>
          <span class="text-[10px] text-muted-foreground">{l("بناء الشهادات والتقارير عند اعتماد النتائج", "Generate report cards on grade publish")}</span>
        </div>
        <Switch bind:checked={autoReportGeneration} disabled={isSaving || !enabled} />
      </div>

      <div class="flex items-center justify-between p-3 rounded-lg border border-border/50 bg-muted/20">
        <div class="flex flex-col text-start">
          <Label class="text-xs font-semibold">{l("رصد الحضور الذاتي", "Auto Record Attendance")}</Label>
          <span class="text-[10px] text-muted-foreground">{l("توليد سجلات الحضور الافتراضية للحصص", "Generate default attendance records")}</span>
        </div>
        <Switch bind:checked={autoRecordAttendance} disabled={isSaving || !enabled} />
      </div>

      <div class="flex items-center justify-between p-3 rounded-lg border border-border/50 bg-muted/20">
        <div class="flex flex-col text-start">
          <Label class="text-xs font-semibold">{l("ترحيل الطلاب تلقائياً", "Auto Promotion")}</Label>
          <span class="text-[10px] text-muted-foreground">{l("ترحيل الطلاب الناجحين للمستوى التالي", "Promote passing students at year end")}</span>
        </div>
        <Switch bind:checked={autoPromotion} disabled={isSaving || !enabled} />
      </div>

      <div class="flex items-center justify-between p-3 rounded-lg border border-border/50 bg-muted/20">
        <div class="flex flex-col text-start">
          <Label class="text-xs font-semibold">{l("جدولة الامتحانات آلياً", "Auto Exam Scheduling")}</Label>
          <span class="text-[10px] text-muted-foreground">{l("توزيع جداول الاختبارات والقاعات تلقائياً", "Generate exam slots and seating plans")}</span>
        </div>
        <Switch bind:checked={autoExamScheduling} disabled={isSaving || !enabled} />
      </div>
    </div>

    <!-- Numerical Policy Parameters -->
    <div class="grid grid-cols-1 md:grid-cols-3 gap-4 pt-2 border-t border-border/60">
      <div class="flex flex-col gap-1.5 text-start">
        <Label class="text-xs font-semibold">{l("فترة السماح (أيام)", "Grace Period (Days)")}</Label>
        <Input type="number" min="0" max="30" bind:value={graceDays} disabled={isSaving || !enabled} class="h-8 text-xs" />
      </div>

      <div class="flex flex-col gap-1.5 text-start">
        <Label class="text-xs font-semibold">{l("الإشعار قبل المهلة (ساعات)", "Lead Notification (Hours)")}</Label>
        <Input type="number" min="1" max="168" bind:value={notifyBeforeDeadlineHours} disabled={isSaving || !enabled} class="h-8 text-xs" />
      </div>

      <div class="flex flex-col gap-1.5 text-start">
        <Label class="text-xs font-semibold">{l("مهلة إغلاق رصد الدرجات (أيام)", "Grade Entry Window (Days)")}</Label>
        <Input type="number" min="1" max="60" bind:value={gradeEntryAutoCloseDays} disabled={isSaving || !enabled} class="h-8 text-xs" />
      </div>
    </div>
  </CardContent>
</Card>
