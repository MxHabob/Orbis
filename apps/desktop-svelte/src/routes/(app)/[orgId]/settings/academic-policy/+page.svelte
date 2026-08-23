<!--
@component AcademicPolicySettingsPage
Manages organization-wide academic, grading, enrollment, and evaluation policies.
Follows standard settings layout pattern (section headers, clean two-column grid, top right save action).
-->
<script lang="ts">
  import * as m from "$lib/paraglide/messages";
  import { useQueryClient } from "@tanstack/svelte-query";
  import {
    createOrgAcademicPolicyQuery,
    useUpdateAcademicPolicy,
  } from "$lib/features/academics/queries-policy";
  import { extractErrorMessage } from "$lib/core/ipc";
  import { Button } from "@orbis/ui-svelte/button";
  import { Input } from "@orbis/ui-svelte/input";
  import { Label } from "@orbis/ui-svelte/label";
  import { Switch } from "@orbis/ui-svelte/switch";
  import {
    Select,
    SelectTrigger,
    SelectContent,
    SelectItem,
  } from "@orbis/ui-svelte/select";
  import { toast } from "svelte-sonner";

  import {
    ShieldAlert,
    Award,
    Loader2,
    BookCheck,
    GraduationCap,
    LockKeyhole,
  } from "@lucide/svelte";
  import SettingsFormSkeleton from "$lib/shared/skeletons/settings-form-skeleton.svelte";

  let { data }: { data: { orgId: string } } = $props();
  const orgId = $derived(data.orgId);

  const policyQuery = createOrgAcademicPolicyQuery(() => orgId);
  const updateMutation = useUpdateAcademicPolicy();

  // Form states
  let isSaving = $state(false);
  let form = $state({
    default_grading_system: "percentage",
    default_passing_grade: 50,
    default_max_grade: 100,
    min_attendance_rate: 75,
    enrollment_auto_close: true,
    allow_late_enrollment: false,
    late_enrollment_approval: true,
    grade_entry_auto_close: true,
    allow_grade_edit_after_close: false,
    grade_edit_approval: true,
    auto_graduate_final_level: false,
    exclude_withdrawn: true,
  });

  // Sync database record to local form state
  $effect(() => {
    if (policyQuery.data) {
      const d = policyQuery.data as any;
      form = {
        default_grading_system: d.default_grading_system || "percentage",
        default_passing_grade: d.default_passing_score ?? d.default_passing_grade ?? 50,
        default_max_grade: d.default_max_score ?? d.default_max_grade ?? 100,
        min_attendance_rate: d.min_attendance_rate ?? 75,
        enrollment_auto_close: d.enrollment_auto_close === 1,
        allow_late_enrollment: d.allow_late_enrollment === 1,
        late_enrollment_approval: d.late_enrollment_approval === 1,
        grade_entry_auto_close: d.grade_entry_auto_close === 1,
        allow_grade_edit_after_close: d.allow_grade_edit_after_close === 1,
        grade_edit_approval: d.grade_edit_approval === 1,
        auto_graduate_final_level: d.auto_graduate_final_level === 1,
        exclude_withdrawn: d.exclude_withdrawn !== 0,
      };
    }
  });

  async function handleSave(e: Event) {
    e.preventDefault();
    isSaving = true;
    try {
      const payload = {
        default_grading_system: form.default_grading_system,
        default_passing_score: Number(form.default_passing_grade),
        default_max_score: Number(form.default_max_grade),
        min_attendance_rate: Number(form.min_attendance_rate),
        enrollment_auto_close: form.enrollment_auto_close ? 1 : 0,
        allow_late_enrollment: form.allow_late_enrollment ? 1 : 0,
        late_enrollment_approval: form.late_enrollment_approval ? 1 : 0,
        grade_entry_auto_close: form.grade_entry_auto_close ? 1 : 0,
        allow_grade_edit_after_close: form.allow_grade_edit_after_close ? 1 : 0,
        grade_edit_approval: form.grade_edit_approval ? 1 : 0,
        auto_graduate_final_level: form.auto_graduate_final_level ? 1 : 0,
        exclude_withdrawn: form.exclude_withdrawn ? 1 : 0,
      };

      await updateMutation.mutateAsync({ orgId: data.orgId, payload });

      toast.success(
        m.settings_general_save_success
          ? m.settings_general_save_success()
          : "تم حفظ السياسات الأكاديمية بنجاح",
      );
    } catch (err: unknown) {
      toast.error(extractErrorMessage(err));
    } finally {
      isSaving = false;
    }
  }
</script>

<svelte:head>
  <title>
    {m.settings_academic_policy_title
      ? m.settings_academic_policy_title()
      : "السياسات الأكاديمية"} | Orbis
  </title>
</svelte:head>

<div
  class="h-full w-full overflow-y-auto p-6 flex flex-col gap-6 scrollbar-hide bg-background"
>
  {#if policyQuery.isLoading}
    <div class="w-full">
      <SettingsFormSkeleton />
    </div>
  {:else if policyQuery.error}
    <div class="w-full text-center py-12 text-destructive">
      <p>
        {m.settings_policy_load_error
          ? m.settings_policy_load_error()
          : "فشل تحميل السياسات الأكاديمية"}
      </p>
      <Button
        variant="outline"
        class="mt-4 cursor-pointer"
        onclick={() => policyQuery.refetch()}
      >
        إعادة المحاولة
      </Button>
    </div>
  {:else}
    <form onsubmit={handleSave} class="flex flex-col gap-8 w-full max-w-6xl">
      <!-- Top Action Bar -->
      <div class="flex items-center justify-between gap-3 w-full select-none pb-4 border-b border-border/40">
        <div class="flex flex-col text-start">
          <h1 class="text-sm font-semibold text-foreground">السياسات والضوابط الأكاديمية</h1>
          <p class="text-xs text-muted-foreground">إعدادات النظم الدراسية، درجات النجاح، رصد الدرجات، وضوابط التسجيل والترفيع.</p>
        </div>

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
          {isSaving
            ? m.common_state_saving
              ? m.common_state_saving()
              : "جاري الحفظ..."
            : m.settings_action_save_changes
              ? m.settings_action_save_changes()
              : "حفظ التغييرات"}
        </Button>
      </div>

      <!-- Two-Column Settings Layout -->
      <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
        
        <!-- 1. Grading & Evaluation Defaults Section -->
        <div class="flex flex-col gap-5 text-start bg-card border border-border/50 rounded-xl p-5 shadow-2xs">
          <div class="border-b border-border/60 pb-3 flex items-center gap-2">
            <Award size={16} class="text-primary" />
            <h2 class="text-xs font-semibold text-foreground">
              نظام الدرجات والتقييم الأساسي
            </h2>
          </div>

          <div class="flex flex-col gap-4">
            <!-- Grading System -->
            <div class="flex flex-col gap-1.5 text-start">
              <Label class="text-xs font-semibold">نظام الدرجات الافتراضي للمؤسسة</Label>
              <Select
                type="single"
                value={form.default_grading_system}
                onValueChange={(v) => { if (v) form.default_grading_system = v; }}
              >
                <SelectTrigger class="text-xs h-9 w-full">
                  {form.default_grading_system === "percentage" ? "نسبة مئوية (100%)" : form.default_grading_system === "gpa_4_0" ? "معدل تراكمي (GPA 4.0)" : "ناجح / راسب (Pass/Fail)"}
                </SelectTrigger>
                <SelectContent class="text-xs">
                  <SelectItem value="percentage">نسبة مئوية (100%)</SelectItem>
                  <SelectItem value="gpa_4_0">معدل تراكمي (GPA 4.0)</SelectItem>
                  <SelectItem value="pass_fail">ناجح / راسب (Pass/Fail)</SelectItem>
                </SelectContent>
              </Select>
            </div>

            <!-- Scores Grid -->
            <div class="grid grid-cols-2 gap-4">
              <div class="flex flex-col gap-1.5 text-start">
                <Label class="text-xs font-semibold">درجة النجاح الافتراضية</Label>
                <Input
                  type="number"
                  step="0.5"
                  bind:value={form.default_passing_grade}
                  class="text-xs font-mono h-9"
                  disabled={isSaving}
                />
              </div>

              <div class="flex flex-col gap-1.5 text-start">
                <Label class="text-xs font-semibold">الدرجة القصوى الافتراضية</Label>
                <Input
                  type="number"
                  step="1"
                  bind:value={form.default_max_grade}
                  class="text-xs font-mono h-9"
                  disabled={isSaving}
                />
              </div>
            </div>

            <!-- Min Attendance Rate -->
            <div class="flex flex-col gap-1.5 text-start pt-1">
              <Label class="text-xs font-semibold">الحد الأدنى لنسبة الحضور للترفيع (%)</Label>
              <Input
                type="number"
                min="0"
                max="100"
                bind:value={form.min_attendance_rate}
                class="text-xs font-mono h-9"
                disabled={isSaving}
              />
              <span class="text-[10px] text-muted-foreground">نسبة حضور الطالب الإلزامية لاجتياز العام الدراسي والتأهل للترفيع.</span>
            </div>
          </div>
        </div>

        <!-- 2. Enrollment Rules Section -->
        <div class="flex flex-col gap-5 text-start bg-card border border-border/50 rounded-xl p-5 shadow-2xs">
          <div class="border-b border-border/60 pb-3 flex items-center gap-2">
            <ShieldAlert size={16} class="text-primary" />
            <h2 class="text-xs font-semibold text-foreground">
              {m.common_policy_card_enrollment
                ? m.common_policy_card_enrollment()
                : "قواعد وضوابط التسجيل"}
            </h2>
          </div>

          <div class="flex flex-col gap-3.5">
            <!-- Allow Late Enrollment -->
            <div class="flex items-center justify-between py-2 border-b border-border/20">
              <div class="flex flex-col text-start gap-0.5 pe-4">
                <span class="text-xs font-semibold text-foreground">
                  {m.common_policy_field_allow_late
                    ? m.common_policy_field_allow_late()
                    : "السماح بالتسجيل المتأخر"}
                </span>
                <span class="text-[10px] text-muted-foreground leading-relaxed">
                  السماح بتسجيل الطلاب وقبول ملفاتهم بعد انقضاء الموعد النهائي للتسجيل.
                </span>
              </div>
              <Switch
                checked={form.allow_late_enrollment}
                onCheckedChange={(v) => (form.allow_late_enrollment = v)}
                disabled={isSaving}
              />
            </div>

            <!-- Late Enrollment Requires Approval -->
            <div class="flex items-center justify-between py-2 border-b border-border/20">
              <div class="flex flex-col text-start gap-0.5 pe-4">
                <span class="text-xs font-semibold text-foreground">
                  {m.common_policy_field_late_approval
                    ? m.common_policy_field_late_approval()
                    : "التسجيل المتأخر يتطلب موافقة"}
                </span>
                <span class="text-[10px] text-muted-foreground leading-relaxed">
                  توجيه طلبات التسجيل المتأخر إلى قائمة الموافقات الإدارية للاعتماد.
                </span>
              </div>
              <Switch
                checked={form.late_enrollment_approval}
                onCheckedChange={(v) => (form.late_enrollment_approval = v)}
                disabled={isSaving || !form.allow_late_enrollment}
              />
            </div>

            <!-- Auto Close on Capacity -->
            <div class="flex items-center justify-between py-2">
              <div class="flex flex-col text-start gap-0.5 pe-4">
                <span class="text-xs font-semibold text-foreground">
                  {m.common_policy_field_auto_close_capacity
                    ? m.common_policy_field_auto_close_capacity()
                    : "الإغلاق التلقائي عند اكتمال الطاقة"}
                </span>
                <span class="text-[10px] text-muted-foreground leading-relaxed">
                  تأمين وإغلاق الشعب الأكاديمية تلقائياً عند وصول الطاقة الاستيعابية المحددة للفصل.
                </span>
              </div>
              <Switch
                checked={form.enrollment_auto_close}
                onCheckedChange={(v) => (form.enrollment_auto_close = v)}
                disabled={isSaving}
              />
            </div>
          </div>
        </div>

        <!-- 3. Grade Entry & Locking Policies Section -->
        <div class="flex flex-col gap-5 text-start bg-card border border-border/50 rounded-xl p-5 shadow-2xs">
          <div class="border-b border-border/60 pb-3 flex items-center gap-2">
            <LockKeyhole size={16} class="text-primary" />
            <h2 class="text-xs font-semibold text-foreground">
              ضوابط رصد وقفل الدرجات
            </h2>
          </div>

          <div class="flex flex-col gap-3.5">
            <!-- Grade Entry Auto Close -->
            <div class="flex items-center justify-between py-2 border-b border-border/20">
              <div class="flex flex-col text-start gap-0.5 pe-4">
                <span class="text-xs font-semibold text-foreground">الإغلاق التلقائي لرصد الدرجات</span>
                <span class="text-[10px] text-muted-foreground leading-relaxed">
                  قفل إمكانية إدخال درجات الاختبارات والأنشطة تلقائياً بمجرد انتهاء الموعد المحدد للفترة.
                </span>
              </div>
              <Switch
                checked={form.grade_entry_auto_close}
                onCheckedChange={(v) => (form.grade_entry_auto_close = v)}
                disabled={isSaving}
              />
            </div>

            <!-- Allow Grade Edit After Close -->
            <div class="flex items-center justify-between py-2 border-b border-border/20">
              <div class="flex flex-col text-start gap-0.5 pe-4">
                <span class="text-xs font-semibold text-foreground">السماح بتعديل الدرجات بعد الإغلاق</span>
                <span class="text-[10px] text-muted-foreground leading-relaxed">
                  إمكانية فتح طلب تعديل لدرجة طالب بعد اعتماد الفترة وإغلاق الرصد.
                </span>
              </div>
              <Switch
                checked={form.allow_grade_edit_after_close}
                onCheckedChange={(v) => (form.allow_grade_edit_after_close = v)}
                disabled={isSaving}
              />
            </div>

            <!-- Grade Edit Requires Approval -->
            <div class="flex items-center justify-between py-2">
              <div class="flex flex-col text-start gap-0.5 pe-4">
                <span class="text-xs font-semibold text-foreground">تعديل الدرجة يتطلب موافقة واعتماد</span>
                <span class="text-[10px] text-muted-foreground leading-relaxed">
                  إحالة أي طلب تعديل درجات إلى الشؤون الأكاديمية/الإدارة للمراجعة والتوثيق في سجل التدقيق.
                </span>
              </div>
              <Switch
                checked={form.grade_edit_approval}
                onCheckedChange={(v) => (form.grade_edit_approval = v)}
                disabled={isSaving || !form.allow_grade_edit_after_close}
              />
            </div>
          </div>
        </div>

        <!-- 4. Promotion & Graduation Rules Section -->
        <div class="flex flex-col gap-5 text-start bg-card border border-border/50 rounded-xl p-5 shadow-2xs">
          <div class="border-b border-border/60 pb-3 flex items-center gap-2">
            <GraduationCap size={16} class="text-primary" />
            <h2 class="text-xs font-semibold text-foreground">
              ضوابط الترفيع الأكاديمي والتخرج
            </h2>
          </div>

          <div class="flex flex-col gap-3.5">
            <!-- Auto Graduate Final Level -->
            <div class="flex items-center justify-between py-2 border-b border-border/20">
              <div class="flex flex-col text-start gap-0.5 pe-4">
                <span class="text-xs font-semibold text-foreground">تخريج طلاب الصف النهائي تلقائياً</span>
                <span class="text-[10px] text-muted-foreground leading-relaxed">
                  تحويل حالة الطلاب الناجحين في أعلى مستوى دراسي بالمؤسسة إلى "خريج" (Graduated) تلقائياً عند تنفيذ الترفيع.
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
                <span class="text-xs font-semibold text-foreground">استبعاد الطلاب المنسحبين والمعلقين</span>
                <span class="text-[10px] text-muted-foreground leading-relaxed">
                  عدم إدراج الطلاب ذوي حالة "منسحب" أو "موقوف" في مسارات الترفيع للعام الدراسي الجديد.
                </span>
              </div>
              <Switch
                checked={form.exclude_withdrawn}
                onCheckedChange={(v) => (form.exclude_withdrawn = v)}
                disabled={isSaving}
              />
            </div>
          </div>
        </div>

      </div>
    </form>
  {/if}
</div>
