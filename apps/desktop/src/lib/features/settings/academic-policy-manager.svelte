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
      m.ui_academic_policies_saved_successf_560bbb(),
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
        {m.ui_failed_to_load_academic_policies_838a32()}
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
            {m.ui_academic_policies_evaluation_eng_e18364()}
          </h1>
          <p class="text-xs text-muted-foreground mt-0.5">
            {m.ui_configure_grading_scales_pass_th_23486d()}
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
              <CardTitle>{m.ui_grading_passing_criteria_74787f()}</CardTitle>
            </div>
            <CardDescription class="text-xs">
              {m.ui_grading_scale_and_subject_passin_563452()}
            </CardDescription>
          </CardHeader>

          <CardContent class="p-6 flex flex-col gap-5 text-start">
            <!-- Grading System -->
            <div class="flex flex-col gap-1.5 text-start">
              <Label class="text-xs font-semibold">{m.ui_default_grading_system_ec7f85()}</Label>
              <Select
                type="single"
                value={form.default_grading_system}
                onValueChange={(v) => { if (v) form.default_grading_system = v; }}
              >
                <SelectTrigger class="text-xs h-9 w-full">
                  {form.default_grading_system === "percentage" ? m.ui_percentage_100_points_772039() : form.default_grading_system === "gpa_4_0" ? m.ui_gpa_4_0_scale_29e27a() : m.ui_pass_fail_24ce15()}
                </SelectTrigger>
                <SelectContent class="text-xs">
                  <SelectItem value="percentage">{m.ui_percentage_100_points_772039()}</SelectItem>
                  <SelectItem value="gpa_4_0">{m.ui_gpa_4_0_scale_29e27a()}</SelectItem>
                  <SelectItem value="pass_fail">{m.ui_pass_fail_24ce15()}</SelectItem>
                </SelectContent>
              </Select>
            </div>

            <!-- Scores Grid -->
            <div class="grid grid-cols-2 gap-4">
              <div class="flex flex-col gap-1.5 text-start">
                <Label class="text-xs font-semibold">{m.ui_passing_threshold_a74bf1()}</Label>
                <Input
                  type="number"
                  step="0.5"
                  min="0"
                  max="100"
                  bind:value={form.default_passing_grade}
                  class="text-xs font-mono h-9"
                  disabled={isSaving}
                />
                <span class="text-[10px] text-muted-foreground">{m.ui_minimum_score_required_to_pass_03e49e()}</span>
              </div>

              <div class="flex flex-col gap-1.5 text-start">
                <Label class="text-xs font-semibold">{m.ui_honors_threshold_3d4b7c()}</Label>
                <Input
                  type="number"
                  step="0.5"
                  min="50"
                  max="100"
                  bind:value={form.honors_threshold_percentage}
                  class="text-xs font-mono h-9"
                  disabled={isSaving}
                />
                <span class="text-[10px] text-muted-foreground">{m.ui_percentage_for_honor_standing_6f3da2()}</span>
              </div>
            </div>

            <!-- Minimum final exam score & conditional pass -->
            <div class="grid grid-cols-2 gap-4 pt-1">
              <div class="flex flex-col gap-1.5 text-start">
                <Label class="text-xs font-semibold">{m.ui_min_final_exam_score_a29aea()}</Label>
                <Input
                  type="number"
                  step="0.5"
                  min="0"
                  max="50"
                  bind:value={form.minimum_final_exam_score}
                  class="text-xs font-mono h-9"
                  disabled={isSaving}
                />
                <span class="text-[10px] text-muted-foreground">{m.ui_min_final_exam_points_to_pass_daba47()}</span>
              </div>

              <div class="flex flex-col gap-1.5 text-start">
                <Label class="text-xs font-semibold">{m.ui_max_failed_for_conditional_4276ee()}</Label>
                <Input
                  type="number"
                  step="1"
                  min="0"
                  max="10"
                  bind:value={form.max_failed_subjects_for_conditional_pass}
                  class="text-xs font-mono h-9"
                  disabled={isSaving}
                />
                <span class="text-[10px] text-muted-foreground">{m.ui_max_failed_subjects_for_conditio_0d1f8e()}</span>
              </div>
            </div>

            <!-- Min Attendance Rate -->
            <div class="flex flex-col gap-1.5 text-start pt-1">
              <Label class="text-xs font-semibold">{m.ui_minimum_attendance_rate_47d8e4()}</Label>
              <Input
                type="number"
                min="0"
                max="100"
                bind:value={form.min_attendance_rate}
                class="text-xs font-mono h-9"
                disabled={isSaving}
              />
              <span class="text-[10px] text-muted-foreground">{m.ui_required_attendance_rate_to_sit__6afed1()}</span>
            </div>
          </CardContent>
        </Card>

        <!-- 2. Ranking & Classification Policy -->
        <Card class="border-border/80 shadow-xs">
          <CardHeader>
            <div class="flex items-center gap-2">
              <ChartBar size={14} class="text-primary" />
              <CardTitle>{m.ui_ranking_classification_policy_40c8cf()}</CardTitle>
            </div>
            <CardDescription class="text-xs">
              {m.ui_rules_for_calculating_student_ra_15624e()}
            </CardDescription>
          </CardHeader>

          <CardContent class="p-6 flex flex-col gap-5 text-start">
            <!-- Ranking Enabled Switch -->
            <div class="flex items-center justify-between py-2 border-b border-border">
              <div class="flex flex-col text-start gap-0.5 pe-4">
                <span class="text-xs font-semibold text-foreground">{m.ui_enable_student_ranking_f47315()}</span>
                <span class="text-[10px] text-muted-foreground leading-relaxed">
                  {m.ui_calculate_student_ranks_across_c_088d03()}
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
                <Label class="text-xs font-semibold">{m.ui_ranking_scope_b26087()}</Label>
                <Select
                  type="single"
                  value={form.ranking_scope}
                  onValueChange={(v) => { if (v) form.ranking_scope = v; }}
                  disabled={isSaving || !form.ranking_enabled}
                >
                  <SelectTrigger class="text-xs h-9 w-full">
                    {form.ranking_scope === "class" ? m.ui_class_level_037b4e() : form.ranking_scope === "grade_level" ? m.ui_grade_level_b93e8d() : m.ui_cohort_general_1d2289()}
                  </SelectTrigger>
                  <SelectContent class="text-xs">
                    <SelectItem value="class">{m.ui_class_level_037b4e()}</SelectItem>
                    <SelectItem value="grade_level">{m.ui_grade_level_b93e8d()}</SelectItem>
                    <SelectItem value="cohort">{m.ui_cohort_general_1d2289()}</SelectItem>
                  </SelectContent>
                </Select>
                <span class="text-[10px] text-muted-foreground">{m.ui_reference_comparison_group_e37b0a()}</span>
              </div>

              <div class="flex flex-col gap-1.5 text-start">
                <Label class="text-xs font-semibold">{m.ui_tie_policy_b9970e()}</Label>
                <Select
                  type="single"
                  value={form.tie_policy}
                  onValueChange={(v) => { if (v) form.tie_policy = v; }}
                  disabled={isSaving || !form.ranking_enabled}
                >
                  <SelectTrigger class="text-xs h-9 w-full">
                    {form.tie_policy === "standard" ? m.ui_standard_1_2_2_4_7eca5d() : form.tie_policy === "dense" ? m.ui_dense_1_2_2_3_a59dc6() : m.ui_fractional_1_2_5_2_5_4_efa744()}
                  </SelectTrigger>
                  <SelectContent class="text-xs">
                    <SelectItem value="standard">{m.ui_standard_1_2_2_4_7eca5d()}</SelectItem>
                    <SelectItem value="dense">{m.ui_dense_1_2_2_3_a59dc6()}</SelectItem>
                    <SelectItem value="fractional">{m.ui_fractional_1_2_5_2_5_4_efa744()}</SelectItem>
                  </SelectContent>
                </Select>
                <span class="text-[10px] text-muted-foreground">{m.ui_how_equal_scores_are_handled_b7f263()}</span>
              </div>
            </div>

            <!-- Max Displayed Rank -->
            <div class="flex flex-col gap-1.5 text-start pt-1">
              <Label class="text-xs font-semibold">{m.ui_max_displayed_rank_on_cards_f5b6fd()}</Label>
              <Input
                type="number"
                min="0"
                max="50"
                bind:value={form.max_displayed_rank}
                class="text-xs font-mono h-9"
                disabled={isSaving || !form.ranking_enabled}
              />
              <span class="text-[10px] text-muted-foreground">{m.ui_e_g_top_10_honors_only_or_0_to_d_a0e17f()}</span>
            </div>
          </CardContent>
        </Card>

        <!-- 3. Rounding & Precision Rules -->
        <Card class="border-border/80 shadow-xs">
          <CardHeader>
            <div class="flex items-center gap-2">
              <Calculator size={14} class="text-primary" />
              <CardTitle>{m.ui_rounding_precision_rules_db7196()}</CardTitle>
            </div>
            <CardDescription class="text-xs">
              {m.ui_decimal_rounding_modes_and_preci_9c871e()}
            </CardDescription>
          </CardHeader>

          <CardContent class="p-6 flex flex-col gap-5 text-start">
            <div class="grid grid-cols-2 gap-4">
              <div class="flex flex-col gap-1.5 text-start">
                <Label class="text-xs font-semibold">{m.ui_rounding_mode_27bf92()}</Label>
                <Select
                  type="single"
                  value={form.rounding_mode}
                  onValueChange={(v) => { if (v) form.rounding_mode = v; }}
                  disabled={isSaving}
                >
                  <SelectTrigger class="text-xs h-9 w-full">
                    {form.rounding_mode === "half_up" ? m.ui_half_up_standard_0d80cf() : form.rounding_mode === "floor" ? m.ui_floor_truncate_c3299b() : form.rounding_mode === "ceil" ? m.ui_ceiling_dffa17() : m.ui_none_30ca52()}
                  </SelectTrigger>
                  <SelectContent class="text-xs">
                    <SelectItem value="half_up">{m.ui_half_up_standard_0d80cf()}</SelectItem>
                    <SelectItem value="floor">{m.ui_floor_truncate_c3299b()}</SelectItem>
                    <SelectItem value="ceil">{m.ui_ceiling_dffa17()}</SelectItem>
                    <SelectItem value="none">{m.ui_none_30ca52()}</SelectItem>
                  </SelectContent>
                </Select>
                <span class="text-[10px] text-muted-foreground">{m.ui_mathematical_rounding_rule_a4dfc8()}</span>
              </div>

              <div class="flex flex-col gap-1.5 text-start">
                <Label class="text-xs font-semibold">{m.ui_decimal_places_989019()}</Label>
                <Input
                  type="number"
                  min="0"
                  max="4"
                  bind:value={form.decimal_places}
                  disabled={isSaving}
                  class="text-xs font-mono h-9"
                />
                <span class="text-[10px] text-muted-foreground">{m.ui_precision_in_grade_reports_e_g_2_1febb3()}</span>
              </div>
            </div>
          </CardContent>
        </Card>

        <!-- 4. Grade Entry & Promotion Rules -->
        <Card class="border-border/80 shadow-xs">
          <CardHeader>
            <div class="flex items-center gap-2">
              <Lock size={14} class="text-primary" />
              <CardTitle>{m.ui_grade_entry_promotion_rules_f83c8c()}</CardTitle>
            </div>
            <CardDescription class="text-xs">
              {m.ui_grade_book_locking_edit_approval_666867()}
            </CardDescription>
          </CardHeader>

          <CardContent class="p-6 flex flex-col gap-4 text-start">
            <!-- Grade Entry Auto Close -->
            <div class="flex items-center justify-between py-2 border-b border-border">
              <div class="flex flex-col text-start gap-0.5 pe-4">
                <span class="text-xs font-semibold text-foreground">{m.ui_auto_close_grade_entry_bfd566()}</span>
                <span class="text-[10px] text-muted-foreground leading-relaxed">
                  {m.ui_automatically_close_grade_entry__ece9b5()}
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
                <span class="text-xs font-semibold text-foreground">{m.ui_allow_grade_edits_after_close_4c5d1a()}</span>
                <span class="text-[10px] text-muted-foreground leading-relaxed">
                  {m.ui_allow_edit_requests_for_locked_p_037712()}
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
                <span class="text-xs font-semibold text-foreground">{m.ui_grade_edits_require_approval_09b902()}</span>
                <span class="text-[10px] text-muted-foreground leading-relaxed">
                  {m.ui_requires_managerial_approval_bef_152354()}
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
                <span class="text-xs font-semibold text-foreground">{m.ui_auto_graduate_final_level_764dc2()}</span>
                <span class="text-[10px] text-muted-foreground leading-relaxed">
                  {m.ui_mark_final_grade_students_as_gra_1743d8()}
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
                <span class="text-xs font-semibold text-foreground">{m.ui_exclude_withdrawn_students_78d08c()}</span>
                <span class="text-[10px] text-muted-foreground leading-relaxed">
                  {m.ui_exclude_withdrawn_students_from__569a88()}
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
