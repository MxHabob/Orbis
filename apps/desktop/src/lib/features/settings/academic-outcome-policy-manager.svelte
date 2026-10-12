<script lang="ts">
  import { onMount } from "svelte";
  import * as m from "$lib/paraglide/messages";
  import { toast } from "@orbis/ui";
  import { Button } from "@orbis/ui/button";
  import { Input } from "@orbis/ui/input";
  import { Label } from "@orbis/ui/label";
  import { Switch } from "@orbis/ui/switch";
  import {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
  } from "@orbis/ui/card";
  import { api, extractErrorMessage, type OrganizationSettingsDto } from "$lib/api";
  import SettingsFormSkeleton from "$lib/shared/skeletons/settings-form-skeleton.svelte";

  type RoundingMode =
    | "round_half_up"
    | "round_up"
    | "round_down"
    | "truncate";
  type RankingScope = "class" | "grade_level" | "org_unit" | "cohort";
  type TiePolicy = "standard_competition" | "dense" | "fractional";

  interface PolicyForm {
    passingThreshold: string;
    honorsThreshold: string;
    minimumFinalExamScore: string;
    maxFailedSubjects: string;
    minimumAttendance: string;
    rankingEnabled: boolean;
    rankingScope: RankingScope;
    tiePolicy: TiePolicy;
    roundingMode: RoundingMode;
    decimalPlaces: string;
    maxDisplayedRank: string;
  }

  let isLoading = $state(true);
  let isSaving = $state(false);
  let settings = $state<OrganizationSettingsDto | null>(null);
  let form = $state<PolicyForm | null>(null);

  function enumValue<T extends string>(
    value: string | undefined,
    choices: readonly T[],
    label: string,
    fallback: T,
  ): T {
    if (value === undefined) return fallback;
    const choice = choices.find((item) => item === value);
    if (choice !== undefined) return choice;
    throw new Error(`The saved ${label} value is unsupported: ${value}`);
  }

  function hydrate(value: OrganizationSettingsDto) {
    const academic = value.academic;
    if (!academic) throw new Error("Academic settings were not returned by the API.");
    settings = value;
    form = {
      passingThreshold: String(academic.passing_threshold_percentage ?? 50),
      honorsThreshold: String(academic.honors_threshold_percentage ?? 85),
      minimumFinalExamScore:
        academic.minimum_final_exam_score == null
          ? ""
          : String(academic.minimum_final_exam_score),
      maxFailedSubjects: String(
        academic.max_failed_subjects_for_conditional_pass ?? 2,
      ),
      minimumAttendance: String(academic.min_attendance_rate ?? 75),
      rankingEnabled: academic.ranking_enabled ?? true,
      rankingScope: enumValue(
        academic.ranking_scope,
        ["class", "grade_level", "org_unit", "cohort"],
        "ranking scope",
        "class",
      ),
      tiePolicy: enumValue(
        academic.tie_policy,
        ["standard_competition", "dense", "fractional"],
        "tie policy",
        "standard_competition",
      ),
      roundingMode: enumValue(
        academic.rounding_mode,
        ["round_half_up", "round_up", "round_down", "truncate"],
        "rounding mode",
        "round_half_up",
      ),
      decimalPlaces: String(academic.decimal_places ?? 2),
      maxDisplayedRank:
        academic.max_displayed_rank == null
          ? ""
          : String(academic.max_displayed_rank),
    };
  }

  async function loadSettings() {
    isLoading = true;
    try {
      hydrate(await api.getSettings());
    } catch (error: unknown) {
      toast.error(extractErrorMessage(error));
    } finally {
      isLoading = false;
    }
  }

  onMount(loadSettings);

  function numericValue(value: string, label: string, options?: {
    integer?: boolean;
    minimum?: number;
    maximum?: number;
  }): number {
    const number = Number(value);
    if (
      value.trim() === "" ||
      !Number.isFinite(number) ||
      (options?.integer && !Number.isInteger(number)) ||
      (options?.minimum !== undefined && number < options.minimum) ||
      (options?.maximum !== undefined && number > options.maximum)
    ) {
      throw new Error(`${label} has an invalid value.`);
    }
    return number;
  }

  function optionalNumericValue(
    value: string,
    label: string,
    options?: { integer?: boolean; minimum?: number; maximum?: number },
  ): number | null {
    return value.trim() === "" ? null : numericValue(value, label, options);
  }

  async function savePolicy() {
    if (!settings?.academic || !form) return;
    isSaving = true;
    try {
      const latestSettings = await api.getSettings();
      if (!latestSettings.academic) {
        throw new Error("Academic settings were not returned by the API.");
      }
      const updatedSettings = await api.updateSettings({
        ...latestSettings,
        academic: {
          ...latestSettings.academic,
          passing_threshold_percentage: numericValue(
            form.passingThreshold,
            "Passing threshold",
            { minimum: 0, maximum: 100 },
          ),
          honors_threshold_percentage: numericValue(
            form.honorsThreshold,
            "Honors threshold",
            { minimum: 0, maximum: 100 },
          ),
          minimum_final_exam_score: optionalNumericValue(
            form.minimumFinalExamScore,
            "Minimum final exam score",
            { minimum: 0 },
          ),
          max_failed_subjects_for_conditional_pass: numericValue(
            form.maxFailedSubjects,
            "Maximum failed subjects",
            { integer: true, minimum: 0 },
          ),
          min_attendance_rate: numericValue(
            form.minimumAttendance,
            "Minimum attendance rate",
            { minimum: 0, maximum: 100 },
          ),
          ranking_enabled: form.rankingEnabled,
          ranking_scope: form.rankingScope,
          tie_policy: form.tiePolicy,
          rounding_mode: form.roundingMode,
          decimal_places: numericValue(form.decimalPlaces, "Decimal places", {
            integer: true,
            minimum: 0,
            maximum: 28,
          }),
          max_displayed_rank: optionalNumericValue(
            form.maxDisplayedRank,
            "Maximum displayed rank",
            { integer: true, minimum: 1 },
          ),
        },
      });
      hydrate(updatedSettings);
      toast.success(m.settings_academic_policy_saved());
    } catch (error: unknown) {
      toast.error(extractErrorMessage(error));
    } finally {
      isSaving = false;
    }
  }
</script>

<div class="flex h-full w-full flex-col gap-6 overflow-y-auto p-6 scrollbar-hide">
  {#if isLoading}
    <div class="w-full"><SettingsFormSkeleton /></div>
  {:else if form}
    <div class="flex w-full max-w-5xl flex-col gap-5">
      <header class="border-b border-border pb-3">
        <h1 class="text-sm font-semibold text-foreground">
          {m.common_nav_academic_policy()}
        </h1>
        <p class="mt-1 text-xs text-muted-foreground">
          {m.settings_academic_policy_description()}
        </p>
      </header>

      <Card>
        <CardHeader>
          <CardTitle>{m.settings_academic_policy_outcomes_title()}</CardTitle>
          <CardDescription>
            {m.settings_academic_policy_outcomes_description()}
          </CardDescription>
        </CardHeader>
        <CardContent class="grid gap-4 sm:grid-cols-2">
          <div class="flex flex-col gap-1.5">
            <Label for="policy-passing">{m.settings_academic_policy_passing_threshold()}</Label>
            <Input id="policy-passing" type="number" min="0" max="100" bind:value={form.passingThreshold} disabled={isSaving} />
          </div>
          <div class="flex flex-col gap-1.5">
            <Label for="policy-honors">{m.settings_academic_policy_honors_threshold()}</Label>
            <Input id="policy-honors" type="number" min="0" max="100" bind:value={form.honorsThreshold} disabled={isSaving} />
          </div>
          <div class="flex flex-col gap-1.5">
            <Label for="policy-final-exam">{m.settings_academic_policy_minimum_exam()}</Label>
            <Input id="policy-final-exam" type="number" min="0" bind:value={form.minimumFinalExamScore} disabled={isSaving} />
          </div>
          <div class="flex flex-col gap-1.5">
            <Label for="policy-failed-subjects">{m.settings_academic_policy_max_failed()}</Label>
            <Input id="policy-failed-subjects" type="number" min="0" step="1" bind:value={form.maxFailedSubjects} disabled={isSaving} />
          </div>
          <div class="flex flex-col gap-1.5">
            <Label for="policy-attendance">{m.settings_academic_policy_min_attendance()}</Label>
            <Input id="policy-attendance" type="number" min="0" max="100" bind:value={form.minimumAttendance} disabled={isSaving} />
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>{m.settings_academic_policy_ranking_title()}</CardTitle>
          <CardDescription>
            {m.settings_academic_policy_ranking_description()}
          </CardDescription>
        </CardHeader>
        <CardContent class="grid gap-4 sm:grid-cols-2">
          <div class="flex items-center justify-between gap-3 rounded-md border border-border p-3 sm:col-span-2">
            <Label for="policy-ranking">{m.settings_academic_policy_ranking_enabled()}</Label>
            <Switch id="policy-ranking" checked={form.rankingEnabled} onCheckedChange={(checked) => (form!.rankingEnabled = checked)} disabled={isSaving} />
          </div>
          <div class="flex flex-col gap-1.5">
            <Label for="policy-ranking-scope">{m.settings_academic_policy_ranking_scope()}</Label>
            <select id="policy-ranking-scope" class="h-10 rounded-md border border-input bg-background px-3 text-sm" bind:value={form.rankingScope} disabled={isSaving}>
              <option value="class">{m.settings_academic_policy_scope_class()}</option>
              <option value="grade_level">{m.settings_academic_policy_scope_grade()}</option>
              <option value="org_unit">{m.settings_academic_policy_scope_org()}</option>
              <option value="cohort">{m.settings_academic_policy_scope_cohort()}</option>
            </select>
          </div>
          <div class="flex flex-col gap-1.5">
            <Label for="policy-tie">{m.settings_academic_policy_tie_policy()}</Label>
            <select id="policy-tie" class="h-10 rounded-md border border-input bg-background px-3 text-sm" bind:value={form.tiePolicy} disabled={isSaving}>
              <option value="standard_competition">{m.settings_academic_policy_tie_standard()}</option>
              <option value="dense">{m.settings_academic_policy_tie_dense()}</option>
              <option value="fractional">{m.settings_academic_policy_tie_fractional()}</option>
            </select>
          </div>
          <div class="flex flex-col gap-1.5">
            <Label for="policy-rounding">{m.settings_academic_policy_rounding()}</Label>
            <select id="policy-rounding" class="h-10 rounded-md border border-input bg-background px-3 text-sm" bind:value={form.roundingMode} disabled={isSaving}>
              <option value="round_half_up">{m.settings_academic_policy_round_half_up()}</option>
              <option value="round_up">{m.settings_academic_policy_round_up()}</option>
              <option value="round_down">{m.settings_academic_policy_round_down()}</option>
              <option value="truncate">{m.settings_academic_policy_truncate()}</option>
            </select>
          </div>
          <div class="flex flex-col gap-1.5">
            <Label for="policy-decimals">{m.settings_academic_policy_decimal_places()}</Label>
            <Input id="policy-decimals" type="number" min="0" max="28" step="1" bind:value={form.decimalPlaces} disabled={isSaving} />
          </div>
          <div class="flex flex-col gap-1.5">
            <Label for="policy-max-rank">{m.settings_academic_policy_max_rank()}</Label>
            <Input id="policy-max-rank" type="number" min="1" step="1" bind:value={form.maxDisplayedRank} disabled={isSaving} />
          </div>
        </CardContent>
      </Card>

      <div class="flex justify-end border-t border-border pt-4">
        <Button onclick={savePolicy} disabled={isSaving}>
          {m.common_action_save()}
        </Button>
      </div>
    </div>
  {:else}
    <div class="py-12 text-center text-destructive">
      <p>{m.ui_failed_to_load_academic_policies_838a32()}</p>
      <Button variant="outline" class="mt-4" onclick={loadSettings}>
        {m.common_retry()}
      </Button>
    </div>
  {/if}
</div>
