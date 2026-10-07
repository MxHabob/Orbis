<!--
@component AcademicPolicyManager
Manages organization-wide academic, grading, ranking, calculation, and evaluation policies.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { untrack } from "svelte";
import {
  createOrgAcademicPolicyQuery,
  useUpdateAcademicPolicy,
} from "$lib/features/academics/queries-policy";
import { extractErrorMessage } from "$lib/api";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Switch } from "@orbis/ui/switch";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
} from "@orbis/ui/card";
import { toast } from "@orbis/ui";
import { l } from "$lib/core/i18n";
import { AcademicCap, BookOpen, Calculator, ChartBar, Lock, Scale } from "@orbis/icons";
import SettingsFormSkeleton from "$lib/shared/skeletons/settings-form-skeleton.svelte";
import SettingsBottomBar from "$lib/shared/components/settings-bottom-bar.svelte";

let { orgId }: { orgId: string } = $props();

const policyQuery = createOrgAcademicPolicyQuery(() => orgId);
const updateMutation = useUpdateAcademicPolicy();

// Form states
let isSaving = $state(false);
let initialSnapshot = $state("");

let form = $state({
  // Grading & Scoring Defaults
  default_grading_system: "percentage",
  default_passing_grade: 50,
  default_max_grade: 100,
  honors_threshold_percentage: 85,
  minimum_final_exam_score: 10,
  max_failed_subjects_for_conditional_pass: 2,
  min_attendance_rate: 75,

  // Ranking & Classification
  ranking_enabled: true,
  max_displayed_rank: 10,
  ranking_scope: "class",
  tie_policy: "standard",

  // Rounding & Precision
  rounding_mode: "half_up",
  decimal_places: 2,

  // Grade Entry & Locking
  grade_entry_auto_close: true,
  allow_grade_edit_after_close: false,
  grade_edit_approval: true,

  // Promotion & Cohort Rules
  auto_graduate_final_level: false,
  exclude_withdrawn: true,
});

// Sync database settings to local form state
$effect(() => {
  if (policyQuery.data) {
    const d = policyQuery.data as any;
    form = {
      default_grading_system: d.default_grading_system || "percentage",
      default_passing_grade:
        d.passing_threshold_percentage ?? d.default_passing_score ?? d.default_passing_grade ?? 50,
      default_max_grade: d.default_max_score ?? d.default_max_grade ?? 100,
      honors_threshold_percentage: d.honors_threshold_percentage ?? 85,
      minimum_final_exam_score: d.minimum_final_exam_score ?? 10,
      max_failed_subjects_for_conditional_pass: d.max_failed_subjects_for_conditional_pass ?? 2,
      min_attendance_rate: d.min_attendance_rate ?? 75,

      ranking_enabled: d.ranking_enabled ?? true,
      max_displayed_rank: d.max_displayed_rank ?? 10,
      ranking_scope: d.ranking_scope || "class",
      tie_policy: d.tie_policy || "standard",

      rounding_mode: d.rounding_mode || "half_up",
      decimal_places: d.decimal_places ?? 2,

      grade_entry_auto_close: d.grade_entry_auto_close === true || d.grade_entry_auto_close === 1,
      allow_grade_edit_after_close: d.allow_grade_edit_after_close === true || d.allow_grade_edit_after_close === 1,
      grade_edit_approval: d.grade_edit_approval === true || d.grade_edit_approval === 1,

      auto_graduate_final_level: d.auto_graduate_final_level === true || d.auto_graduate_final_level === 1,
      exclude_withdrawn: d.exclude_withdrawn !== false && d.exclude_withdrawn !== 0,
    };

    initialSnapshot = untrack(() => JSON.stringify(form));
  }
});

const isDirty = $derived(
  initialSnapshot !== "" && JSON.stringify(form) !== initialSnapshot,
);

function handleReset() {
  if (!initialSnapshot) return;
  const parsed = JSON.parse(initialSnapshot);
  form = { ...parsed };
}

async function handleSave(e?: Event) {
  e?.preventDefault();
  isSaving = true;
  try {
    const payload = {
      default_grading_system: form.default_grading_system,
      passing_threshold_percentage: Number(form.default_passing_grade),
      default_passing_score: Number(form.default_passing_grade),
      default_max_score: Number(form.default_max_grade),
      honors_threshold_percentage: Number(form.honors_threshold_percentage),
      minimum_final_exam_score: Number(form.minimum_final_exam_score),
      max_failed_subjects_for_conditional_pass: Number(form.max_failed_subjects_for_conditional_pass),
      min_attendance_rate: Number(form.min_attendance_rate),

      ranking_enabled: Boolean(form.ranking_enabled),
      max_displayed_rank: Number(form.max_displayed_rank),
      ranking_scope: form.ranking_scope,
      tie_policy: form.tie_policy,

      rounding_mode: form.rounding_mode,
      decimal_places: Number(form.decimal_places),

      grade_entry_auto_close: Boolean(form.grade_entry_auto_close),
      allow_grade_edit_after_close: Boolean(form.allow_grade_edit_after_close),
      grade_edit_approval: Boolean(form.grade_edit_approval),

      auto_graduate_final_level: Boolean(form.auto_graduate_final_level),
      exclude_withdrawn: Boolean(form.exclude_withdrawn),
    };

    await $updateMutation.mutateAsync({
      orgId,
      payload,
    });

    initialSnapshot = JSON.stringify(form);
    toast.success(
      l("تم حفظ السياسات الأكاديمية بنجاح", "Academic policies saved successfully"),
    );
  } catch (err: unknown) {
    toast.error(extractErrorMessage(err));
  } finally {
    isSaving = false;
  }
}
</script>

<div
  class="h-full w-full overflow-y-auto p-6 flex flex-col gap-6 scrollbar-hide"
>
  {#if policyQuery.isLoading}
    <div class="w-full">
      <SettingsFormSkeleton />
    </div>
  {:else if policyQuery.error}
    <div class="w-full text-center py-12 text-destructive">
      <p>
        {l("فشل في تحميل السياسات الأكاديمية", "Failed to load academic policies")}
      </p>
      <Button
        variant="outline"
        class="mt-4 cursor-pointer"
        onclick={() => policyQuery.refetch()}
      >
        {m.common_retry()}
      </Button>
    </div>
  {:else}
    <form onsubmit={handleSave} class="flex flex-col gap-6 w-full max-w-6xl">
      <!-- Top Header -->
      <div class="flex items-center justify-between gap-3 w-full select-none pb-2 border-b border-border">
        <div class="flex flex-col text-start">
          <h1 class="text-sm font-semibold text-foreground">
            {l("السياسات الأكاديمية ونظام التقييم", "Academic Policies & Evaluation Engine")}
          </h1>
          <p class="text-xs text-muted-foreground mt-0.5">
            {l("تحديد مقاييس الدرجات، عتبات النجاح، ضوابط الترتيب، والدقة الحسابية للمؤسسة التعليمية", "Configure grading scales, pass thresholds, ranking policies, and calculation precision")}
          </p>
        </div>
      </div>

      <!-- Two-Column Settings Grid -->
      <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <!-- 1. Grading & Evaluation Defaults Section -->
        <Card class="border-border/80 shadow-xs">
          <CardHeader>
            <div class="flex items-center gap-2">
              <Scale size={14} class="text-primary" />
              <CardTitle>{l("نظام ومقاييس التقييم والنجاح", "Grading & Passing Criteria")}</CardTitle>
            </div>
            <CardDescription class="text-xs">
              {l("معايير حساب المعدل وعتبات اجتياز المقررات الدراسية", "Grading scale and subject passing requirements")}
            </CardDescription>
          </CardHeader>

          <CardContent class="p-6 flex flex-col gap-5 text-start">
            <!-- Grading System -->
            <div class="flex flex-col gap-1.5 text-start">
              <Label class="text-xs font-semibold">{l("نظام التقييم الأساسي", "Default Grading System")}</Label>
              <Select
                type="single"
                value={form.default_grading_system}
                onValueChange={(v) => { if (v) form.default_grading_system = v; }}
              >
                <SelectTrigger class="text-xs h-9 w-full">
                  {form.default_grading_system === "percentage" ? l("النسبة المئوية (100 نقطة)", "Percentage (100 Points)") : form.default_grading_system === "gpa_4_0" ? l("المعدل التراكمي (GPA 4.0)", "GPA (4.0 Scale)") : l("ناجح / راسب", "Pass / Fail")}
                </SelectTrigger>
                <SelectContent class="text-xs">
                  <SelectItem value="percentage">{l("النسبة المئوية (100 نقطة)", "Percentage (100 Points)")}</SelectItem>
                  <SelectItem value="gpa_4_0">{l("المعدل التراكمي (GPA 4.0)", "GPA (4.0 Scale)")}</SelectItem>
                  <SelectItem value="pass_fail">{l("ناجح / راسب", "Pass / Fail")}</SelectItem>
                </SelectContent>
              </Select>
            </div>

            <!-- Scores Grid -->
            <div class="grid grid-cols-2 gap-4">
              <div class="flex flex-col gap-1.5 text-start">
                <Label class="text-xs font-semibold">{l("درجة النجاح الدنيا (%)", "Passing Threshold (%)")}</Label>
                <Input
                  type="number"
                  step="0.5"
                  min="0"
                  max="100"
                  bind:value={form.default_passing_grade}
                  class="text-xs font-mono h-9"
                  disabled={isSaving}
                />
                <span class="text-[10px] text-muted-foreground">{l("النسبة المئوية المطلوبة للنجاح بالمادة (افتراضياً 50%)", "Minimum score required to pass")}</span>
              </div>

              <div class="flex flex-col gap-1.5 text-start">
                <Label class="text-xs font-semibold">{l("نسبة مرتبة الشرف / التفوق (%)", "Honors Threshold (%)")}</Label>
                <Input
                  type="number"
                  step="0.5"
                  min="50"
                  max="100"
                  bind:value={form.honors_threshold_percentage}
                  class="text-xs font-mono h-9"
                  disabled={isSaving}
                />
                <span class="text-[10px] text-muted-foreground">{l("النسبة المستحقة للشهادات التقديرية (افتراضياً 85%)", "Percentage for honor standing")}</span>
              </div>
            </div>

            <!-- Minimum final exam score & conditional pass -->
            <div class="grid grid-cols-2 gap-4 pt-1">
              <div class="flex flex-col gap-1.5 text-start">
                <Label class="text-xs font-semibold">{l("الحد الأدنى للاختبار النهائي", "Min Final Exam Score")}</Label>
                <Input
                  type="number"
                  step="0.5"
                  min="0"
                  max="50"
                  bind:value={form.minimum_final_exam_score}
                  class="text-xs font-mono h-9"
                  disabled={isSaving}
                />
                <span class="text-[10px] text-muted-foreground">{l("درجة النجاح الشرطية في ورقة الاختبار النهائي (مثلاً 10 من 30)", "Min final exam points to pass")}</span>
              </div>

              <div class="flex flex-col gap-1.5 text-start">
                <Label class="text-xs font-semibold">{l("أقصى مواد متعثرة للترفيع المشروط", "Max Failed for Conditional")}</Label>
                <Input
                  type="number"
                  step="1"
                  min="0"
                  max="10"
                  bind:value={form.max_failed_subjects_for_conditional_pass}
                  class="text-xs font-mono h-9"
                  disabled={isSaving}
                />
                <span class="text-[10px] text-muted-foreground">{l("عدد المواد المتعثرة المسموح بها للانتقال للمستوى التالي", "Max failed subjects for conditional pass")}</span>
              </div>
            </div>

            <!-- Min Attendance Rate -->
            <div class="flex flex-col gap-1.5 text-start pt-1">
              <Label class="text-xs font-semibold">{l("الحد الأدنى لنسبة الحضور (%)", "Minimum Attendance Rate (%)")}</Label>
              <Input
                type="number"
                min="0"
                max="100"
                bind:value={form.min_attendance_rate}
                class="text-xs font-mono h-9"
                disabled={isSaving}
              />
              <span class="text-[10px] text-muted-foreground">{l("نسبة الحضور المطلوبة لدخول الاختبارات والترقية (افتراضياً 75%)", "Required attendance rate to sit for final exams")}</span>
            </div>
          </CardContent>
        </Card>

        <!-- 2. Ranking & Classification Policy -->
        <Card class="border-border/80 shadow-xs">
          <CardHeader>
            <div class="flex items-center gap-2">
              <ChartBar size={14} class="text-primary" />
              <CardTitle>{l("سياسة الترتيب وقوائم الأوائل", "Ranking & Classification Policy")}</CardTitle>
            </div>
            <CardDescription class="text-xs">
              {l("قواعد ترتيب الطلاب والتفوق وحسم حالات التساوي", "Rules for calculating student ranks and handling ties")}
            </CardDescription>
          </CardHeader>

          <CardContent class="p-6 flex flex-col gap-5 text-start">
            <!-- Ranking Enabled Switch -->
            <div class="flex items-center justify-between py-2 border-b border-border">
              <div class="flex flex-col text-start gap-0.5 pe-4">
                <span class="text-xs font-semibold text-foreground">{l("تفعيل احتساب الترتيب", "Enable Student Ranking")}</span>
                <span class="text-[10px] text-muted-foreground leading-relaxed">
                  {l("حساب مراتب الطلاب على مستوى الشعبة والصف وعرضها في كشوف النتائج", "Calculate student ranks across classes and periods")}
                </span>
              </div>
              <Switch
                checked={form.ranking_enabled}
                onCheckedChange={(v) => (form.ranking_enabled = v)}
                disabled={isSaving}
              />
            </div>

            <!-- Scope & Tie Policy Grid -->
            <div class="grid grid-cols-2 gap-4">
              <div class="flex flex-col gap-1.5 text-start">
                <Label class="text-xs font-semibold">{l("نطاق الترتيب", "Ranking Scope")}</Label>
                <Select
                  type="single"
                  value={form.ranking_scope}
                  onValueChange={(v) => { if (v) form.ranking_scope = v; }}
                  disabled={isSaving || !form.ranking_enabled}
                >
                  <SelectTrigger class="text-xs h-9 w-full">
                    {form.ranking_scope === "class" ? l("على مستوى الشعبة", "Class Level") : form.ranking_scope === "grade_level" ? l("على مستوى الصف الدراسي", "Grade Level") : l("على مستوى الدفعة", "Cohort / General")}
                  </SelectTrigger>
                  <SelectContent class="text-xs">
                    <SelectItem value="class">{l("على مستوى الشعبة", "Class Level")}</SelectItem>
                    <SelectItem value="grade_level">{l("على مستوى الصف الدراسي", "Grade Level")}</SelectItem>
                    <SelectItem value="cohort">{l("على مستوى الدفعة", "Cohort / General")}</SelectItem>
                  </SelectContent>
                </Select>
                <span class="text-[10px] text-muted-foreground">{l("المجموعة المرجعية للمقارنة", "Reference comparison group")}</span>
              </div>

              <div class="flex flex-col gap-1.5 text-start">
                <Label class="text-xs font-semibold">{l("معالجة التساوي (Tie Policy)", "Tie Policy")}</Label>
                <Select
                  type="single"
                  value={form.tie_policy}
                  onValueChange={(v) => { if (v) form.tie_policy = v; }}
                  disabled={isSaving || !form.ranking_enabled}
                >
                  <SelectTrigger class="text-xs h-9 w-full">
                    {form.tie_policy === "standard" ? l("قياسي (1, 2, 2, 4)", "Standard (1, 2, 2, 4)") : form.tie_policy === "dense" ? l("متتابع (1, 2, 2, 3)", "Dense (1, 2, 2, 3)") : l("كسري (1, 2.5, 2.5, 4)", "Fractional (1, 2.5, 2.5, 4)")}
                  </SelectTrigger>
                  <SelectContent class="text-xs">
                    <SelectItem value="standard">{l("قياسي (1, 2, 2, 4)", "Standard (1, 2, 2, 4)")}</SelectItem>
                    <SelectItem value="dense">{l("متتابع (1, 2, 2, 3)", "Dense (1, 2, 2, 3)")}</SelectItem>
                    <SelectItem value="fractional">{l("كسري (1, 2.5, 2.5, 4)", "Fractional (1, 2.5, 2.5, 4)")}</SelectItem>
                  </SelectContent>
                </Select>
                <span class="text-[10px] text-muted-foreground">{l("آلية التعامل مع تعادل المجموع", "How equal scores are handled")}</span>
              </div>
            </div>

            <!-- Max Displayed Rank -->
            <div class="flex flex-col gap-1.5 text-start pt-1">
              <Label class="text-xs font-semibold">{l("أقصى رتبة معروضة في الشهادات", "Max Displayed Rank on Cards")}</Label>
              <Input
                type="number"
                min="0"
                max="50"
                bind:value={form.max_displayed_rank}
                class="text-xs font-mono h-9"
                disabled={isSaving || !form.ranking_enabled}
              />
              <span class="text-[10px] text-muted-foreground">{l("مثلاً أول 10 طلاب متميزين فقط، أو ضع 0 لعرض رتبة جميع الطلاب", "e.g., top 10 honors only, or 0 to display all")}</span>
            </div>
          </CardContent>
        </Card>

        <!-- 3. Rounding & Precision Rules -->
        <Card class="border-border/80 shadow-xs">
          <CardHeader>
            <div class="flex items-center gap-2">
              <Calculator size={14} class="text-primary" />
              <CardTitle>{l("قواعد التقريب والدقة الحسابية", "Rounding & Precision Rules")}</CardTitle>
            </div>
            <CardDescription class="text-xs">
              {l("طريقة تقريب الكسور العشرية وعدد الخانات المعتمدة للمعدلات", "Decimal rounding modes and precision for averages")}
            </CardDescription>
          </CardHeader>

          <CardContent class="p-6 flex flex-col gap-5 text-start">
            <div class="grid grid-cols-2 gap-4">
              <div class="flex flex-col gap-1.5 text-start">
                <Label class="text-xs font-semibold">{l("طريقة التقريب", "Rounding Mode")}</Label>
                <Select
                  type="single"
                  value={form.rounding_mode}
                  onValueChange={(v) => { if (v) form.rounding_mode = v; }}
                  disabled={isSaving}
                >
                  <SelectTrigger class="text-xs h-9 w-full">
                    {form.rounding_mode === "half_up" ? l("اعتيادي (نصف لأعلى)", "Half Up (Standard)") : form.rounding_mode === "floor" ? l("اقتصار للأدنى (Floor)", "Floor (Truncate)") : form.rounding_mode === "ceil" ? l("تقريب للأعلى (Ceil)", "Ceiling") : l("بدون تقريب", "None")}
                  </SelectTrigger>
                  <SelectContent class="text-xs">
                    <SelectItem value="half_up">{l("اعتيادي (نصف لأعلى)", "Half Up (Standard)")}</SelectItem>
                    <SelectItem value="floor">{l("اقتصار للأدنى (Floor)", "Floor (Truncate)")}</SelectItem>
                    <SelectItem value="ceil">{l("تقريب للأعلى (Ceil)", "Ceiling")}</SelectItem>
                    <SelectItem value="none">{l("بدون تقريب", "None")}</SelectItem>
                  </SelectContent>
                </Select>
                <span class="text-[10px] text-muted-foreground">{l("القاعدة الرياضية لحسم الكسور", "Mathematical rounding rule")}</span>
              </div>

              <div class="flex flex-col gap-1.5 text-start">
                <Label class="text-xs font-semibold">{l("عدد المنازل العشرية", "Decimal Places")}</Label>
                <Input
                  type="number"
                  min="0"
                  max="4"
                  bind:value={form.decimal_places}
                  disabled={isSaving}
                  class="text-xs font-mono h-9"
                />
                <span class="text-[10px] text-muted-foreground">{l("دقة إظهار المعدلات (مثلاً 2 = 98.75)", "Precision in grade reports (e.g. 2)")}</span>
              </div>
            </div>
          </CardContent>
        </Card>

        <!-- 4. Grade Entry & Promotion Rules -->
        <Card class="border-border/80 shadow-xs">
          <CardHeader>
            <div class="flex items-center gap-2">
              <Lock size={14} class="text-primary" />
              <CardTitle>{l("ضوابط الرصد والترقية", "Grade Entry & Promotion Rules")}</CardTitle>
            </div>
            <CardDescription class="text-xs">
              {l("قواعد إغلاق فترات الرصد وتعديل الدرجات والترقية", "Grade book locking, edit approvals, and cohort graduation")}
            </CardDescription>
          </CardHeader>

          <CardContent class="p-6 flex flex-col gap-4 text-start">
            <!-- Grade Entry Auto Close -->
            <div class="flex items-center justify-between py-2 border-b border-border">
              <div class="flex flex-col text-start gap-0.5 pe-4">
                <span class="text-xs font-semibold text-foreground">{l("الإغلاق التلقائي لفترات الرصد", "Auto-Close Grade Entry")}</span>
                <span class="text-[10px] text-muted-foreground leading-relaxed">
                  {l("إغلاق دفتر الدرجات تلقائياً عند انتهاء المدة الزمنية المحددة للفترة", "Automatically close grade entry upon period deadline")}
                </span>
              </div>
              <Switch
                checked={form.grade_entry_auto_close}
                onCheckedChange={(v) => (form.grade_entry_auto_close = v)}
                disabled={isSaving}
              />
            </div>

            <!-- Allow Grade Edit After Close -->
            <div class="flex items-center justify-between py-2 border-b border-border">
              <div class="flex flex-col text-start gap-0.5 pe-4">
                <span class="text-xs font-semibold text-foreground">{l("السماح بالتعديل بعد الإغلاق", "Allow Grade Edits After Close")}</span>
                <span class="text-[10px] text-muted-foreground leading-relaxed">
                  {l("إتاحة طلب تعديل درجات رصدت في فترات مغلقة", "Allow edit requests for locked periods")}
                </span>
              </div>
              <Switch
                checked={form.allow_grade_edit_after_close}
                onCheckedChange={(v) => (form.allow_grade_edit_after_close = v)}
                disabled={isSaving}
              />
            </div>

            <!-- Grade Edit Requires Approval -->
            <div class="flex items-center justify-between py-2 border-b border-border">
              <div class="flex flex-col text-start gap-0.5 pe-4">
                <span class="text-xs font-semibold text-foreground">{l("اشتراط الموافقة الإدارية لتعديل الدرجات", "Grade Edits Require Approval")}</span>
                <span class="text-[10px] text-muted-foreground leading-relaxed">
                  {l("يتطلب أي تعديل بعد الإغلاق اعتماد وكيل الشؤون التعليمية أو الكنترول", "Requires managerial approval before unfreezing grades")}
                </span>
              </div>
              <Switch
                checked={form.grade_edit_approval}
                onCheckedChange={(v) => (form.grade_edit_approval = v)}
                disabled={isSaving || !form.allow_grade_edit_after_close}
              />
            </div>

            <!-- Auto Graduate Final Level -->
            <div class="flex items-center justify-between py-2 border-b border-border">
              <div class="flex flex-col text-start gap-0.5 pe-4">
                <span class="text-xs font-semibold text-foreground">{l("تخريج تلقائي للمستوى النهائي", "Auto-Graduate Final Level")}</span>
                <span class="text-[10px] text-muted-foreground leading-relaxed">
                  {l("ترقية طلاب الصف الأخير مباشرة إلى حالة (متخرج) عند اجتياز كافة المقررات", "Mark final grade students as graduated upon passing all requirements")}
                </span>
              </div>
              <Switch
                checked={form.auto_graduate_final_level}
                onCheckedChange={(v) => (form.auto_graduate_final_level = v)}
                disabled={isSaving}
              />
            </div>

            <!-- Exclude Withdrawn -->
            <div class="flex items-center justify-between py-2">
              <div class="flex flex-col text-start gap-0.5 pe-4">
                <span class="text-xs font-semibold text-foreground">{l("استبعاد المنسحبين من التقارير", "Exclude Withdrawn Students")}</span>
                <span class="text-[10px] text-muted-foreground leading-relaxed">
                  {l("استبعاد الطلاب المنسحبين والمطوي قيدهم من حساب النسب والترتيب والإحصائيات", "Exclude withdrawn students from ranking and statistics")}
                </span>
              </div>
              <Switch
                checked={form.exclude_withdrawn}
                onCheckedChange={(v) => (form.exclude_withdrawn = v)}
                disabled={isSaving}
              />
            </div>
          </CardContent>
        </Card>
      </div>
    </form>
    <SettingsBottomBar
      {isDirty}
      {isSaving}
      onsave={handleSave}
      onreset={handleReset}
    />
  {/if}
</div>
