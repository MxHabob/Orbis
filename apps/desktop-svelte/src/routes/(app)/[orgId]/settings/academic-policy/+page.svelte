<!--
@component AcademicPolicySettingsPage
Manages organization-wide academic, grading, and enrollment policies.
Follows standard settings layout pattern (section headers, clean two-column grid, top right save action).
-->
<script lang="ts">
  import * as m from "$lib/paraglide/messages";
  import { useQueryClient } from "@tanstack/svelte-query";
  import { createOrgAcademicPolicyQuery } from "$lib/features/academics/queries-policy";
  import { commands, extractErrorMessage } from "$lib/core/ipc";
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
  import { Separator } from "@orbis/ui-svelte/separator";
  import { toast } from "svelte-sonner";

  import { ShieldAlert, Award, Loader2 } from "lucide-svelte";
  import SettingsFormSkeleton from "$lib/shared/skeletons/settings-form-skeleton.svelte";

  let { data }: { data: { orgId: string } } = $props();
  const orgId = $derived(data.orgId);

  const queryClient = useQueryClient();
  const policyQuery = createOrgAcademicPolicyQuery(() => orgId);

  // Form states
  let isSaving = $state(false);
  let form = $state({
    default_grading_system: "percentage",
    default_passing_grade: 50,
    default_max_grade: 100,
    enrollment_auto_close: false,
    allow_late_enrollment: false,
    late_enrollment_approval: false,
    grade_entry_auto_close: false,
    allow_grade_edit_after_close: false,
    grade_edit_approval: false,
  });

  // Sync database record to local form state
  $effect(() => {
    if ($policyQuery.data) {
      const d = $policyQuery.data as any;
      form = {
        default_grading_system: d.default_grading_system || "percentage",
        default_passing_grade: d.default_passing_grade ?? 50,
        default_max_grade: d.default_max_grade ?? 100,
        enrollment_auto_close: d.enrollment_auto_close === 1,
        allow_late_enrollment: d.allow_late_enrollment === 1,
        late_enrollment_approval: d.late_enrollment_approval === 1,
        grade_entry_auto_close: d.grade_entry_auto_close === 1,
        allow_grade_edit_after_close: d.allow_grade_edit_after_close === 1,
        grade_edit_approval: d.grade_edit_approval === 1,
      };
    }
  });

  async function handleSave(e: Event) {
    e.preventDefault();
    isSaving = true;
    try {
      const payload = {
        default_grading_system: form.default_grading_system,
        default_passing_grade: form.default_passing_grade,
        default_max_grade: form.default_max_grade,
        enrollment_auto_close: form.enrollment_auto_close ? 1 : 0,
        allow_late_enrollment: form.allow_late_enrollment ? 1 : 0,
        late_enrollment_approval: form.late_enrollment_approval ? 1 : 0,
        grade_entry_auto_close: form.grade_entry_auto_close ? 1 : 0,
        allow_grade_edit_after_close: form.allow_grade_edit_after_close ? 1 : 0,
        grade_edit_approval: form.grade_edit_approval ? 1 : 0,
        settings: null,
      };

      await commands.platform_policy.updateAutopilotConfig(data.orgId, payload);

      toast.success(
        m.settings_general_save_success
          ? m.settings_general_save_success()
          : "تم حفظ الإعدادات بنجاح",
      );
      queryClient.invalidateQueries({
        queryKey: ["org-academic-policy", orgId],
      });
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
  {#if $policyQuery.isLoading}
    <div class="w-full">
      <SettingsFormSkeleton />
    </div>
  {:else if $policyQuery.error}
    <div class="w-full text-center py-12 text-destructive">
      <p>
        {m.settings_policy_load_error
          ? m.settings_policy_load_error()
          : "فشل تحميل السياسات الأكاديمية"}
      </p>
      <Button
        variant="outline"
        class="mt-4 cursor-pointer"
        onclick={() => $policyQuery.refetch()}
      >
        إعادة المحاولة
      </Button>
    </div>
  {:else}
    <form onsubmit={handleSave} class="flex flex-col gap-8 w-full">
      <!-- Top Action Bar matching system settings pages -->
      <div
        class="flex items-center justify-end gap-3 w-full select-none"
      >
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
      <div class="grid grid-cols-1 md:grid-cols-2 gap-10">
        <!-- 1. Enrollment Rules Section -->
        <div class="flex flex-col gap-6 text-start">
          <div class="border-b border-border/60 pb-3 flex items-center gap-2">
            <ShieldAlert size={16} class="text-primary" />
            <h2 class="text-sm font-semibold text-foreground">
              {m.common_policy_card_enrollment
                ? m.common_policy_card_enrollment()
                : "قواعد وضوابط التسجيل"}
            </h2>
          </div>

          <div class="flex flex-col gap-4">
            <!-- Allow Late Enrollment -->
            <div
              class="flex items-center justify-between py-2 border-b border-border/20"
            >
              <div class="flex flex-col text-start gap-0.5">
                <span class="text-xs font-semibold text-foreground">
                  {m.common_policy_field_allow_late
                    ? m.common_policy_field_allow_late()
                    : "السماح بالتسجيل المتأخر"}
                </span>
                <span class="text-[10px] text-muted-foreground"
                  >السماح بتسجيل الطلاب بعد انتهاء الفترة الأكاديمية النشطة.</span
                >
              </div>
              <Switch
                checked={form.allow_late_enrollment}
                onCheckedChange={(v) => (form.allow_late_enrollment = v)}
                disabled={isSaving}
              />
            </div>

            <!-- Late Enrollment Requires Approval -->
            <div
              class="flex items-center justify-between py-2 border-b border-border/20"
            >
              <div class="flex flex-col text-start gap-0.5">
                <span class="text-xs font-semibold text-foreground">
                  {m.common_policy_field_late_approval
                    ? m.common_policy_field_late_approval()
                    : "التسجيل المتأخر يتطلب موافقة"}
                </span>
                <span class="text-[10px] text-muted-foreground"
                  >توجيه طلبات التسجيل المتأخر لصندوق الاعتماد والمراجعة.</span
                >
              </div>
              <Switch
                checked={form.late_enrollment_approval}
                onCheckedChange={(v) => (form.late_enrollment_approval = v)}
                disabled={isSaving || !form.allow_late_enrollment}
              />
            </div>

            <!-- Auto Close on Capacity -->
            <div class="flex items-center justify-between py-2">
              <div class="flex flex-col text-start gap-0.5">
                <span class="text-xs font-semibold text-foreground">
                  {m.common_policy_field_auto_close_capacity
                    ? m.common_policy_field_auto_close_capacity()
                    : "الإغلاق التلقائي عند اكتمال الطاقة"}
                </span>
                <span class="text-[10px] text-muted-foreground"
                  >تأمين وإغلاق الشعب الأكاديمية تلقائياً عند وصول الطاقة
                  الاستيعابية.</span
                >
              </div>
              <Switch
                checked={form.enrollment_auto_close}
                onCheckedChange={(v) => (form.enrollment_auto_close = v)}
                disabled={isSaving}
              />
            </div>
          </div>
        </div>
      </div>
    </form>
  {/if}
</div>
