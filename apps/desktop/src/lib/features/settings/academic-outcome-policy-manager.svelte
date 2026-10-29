<script lang="ts">
import { onMount } from "svelte";
import * as m from "$lib/paraglide/messages";
import { toast } from "@orbis/ui";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Switch } from "@orbis/ui/switch";
import { Card, CardContent } from "@orbis/ui/card";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
} from "@orbis/ui/select";
import {
  api,
  extractErrorMessage,
  type OrganizationSettingsDto,
} from "$lib/api";
import SettingsFormSkeleton from "$lib/shared/skeletons/settings-form-skeleton.svelte";
import SettingsBottomBar from "$lib/shared/components/settings-bottom-bar.svelte";

type RoundingMode = "round_half_up" | "round_up" | "round_down" | "truncate";
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
let initialSnapshot = $state("");

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
  if (!academic)
    throw new Error("Academic settings were not returned by the API.");
  settings = value;
  const nextForm: PolicyForm = {
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
  form = nextForm;
  initialSnapshot = JSON.stringify(nextForm);
}

const isDirty = $derived(
  initialSnapshot !== "" &&
    form !== null &&
    JSON.stringify(form) !== initialSnapshot,
);

function resetPolicy() {
  if (initialSnapshot) {
    form = JSON.parse(initialSnapshot);
  }
}

function scopeLabel(value: RankingScope): string {
  switch (value) {
    case "class":
      return m.settings_academic_policy_scope_class();
    case "grade_level":
      return m.settings_academic_policy_scope_grade();
    case "org_unit":
      return m.settings_academic_policy_scope_org();
    case "cohort":
      return m.settings_academic_policy_scope_cohort();
  }
}

function tieLabel(value: TiePolicy): string {
  switch (value) {
    case "standard_competition":
      return m.settings_academic_policy_tie_standard();
    case "dense":
      return m.settings_academic_policy_tie_dense();
    case "fractional":
      return m.settings_academic_policy_tie_fractional();
  }
}

function roundingLabel(value: RoundingMode): string {
  switch (value) {
    case "round_half_up":
      return m.settings_academic_policy_round_half_up();
    case "round_up":
      return m.settings_academic_policy_round_up();
    case "round_down":
      return m.settings_academic_policy_round_down();
    case "truncate":
      return m.settings_academic_policy_truncate();
  }
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

function numericValue(
  value: string,
  label: string,
  options?: {
    integer?: boolean;
    minimum?: number;
    maximum?: number;
  },
): number {
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

<div class="h-full w-full flex flex-col overflow-hidden bg-background">
  <main class="min-h-0 flex-1 overflow-y-auto p-6 scrollbar-hide pb-28">
    <div class="w-full flex flex-col gap-6">
      {#if isLoading}
        <SettingsFormSkeleton />
      {:else if form}
        <div class="grid grid-cols-1 xl:grid-cols-2 gap-6 w-full items-start">
          <!-- Outcomes & Thresholds -->
          <Card class="overflow-hidden border-border/70 shadow-xs">
            <CardContent class="grid grid-cols-1 gap-5 p-6 sm:grid-cols-2">
              <div class="flex flex-col gap-1.5">
                <Label for="policy-passing" class="text-xs font-medium text-foreground">
                  {m.settings_academic_policy_passing_threshold()}
                </Label>
                <Input
                  id="policy-passing"
                  type="number"
                  min="0"
                  max="100"
                  class="h-9 text-xs"
                  bind:value={form.passingThreshold}
                  disabled={isSaving}
                />
              </div>

              <div class="flex flex-col gap-1.5">
                <Label for="policy-honors" class="text-xs font-medium text-foreground">
                  {m.settings_academic_policy_honors_threshold()}
                </Label>
                <Input
                  id="policy-honors"
                  type="number"
                  min="0"
                  max="100"
                  class="h-9 text-xs"
                  bind:value={form.honorsThreshold}
                  disabled={isSaving}
                />
              </div>

              <div class="flex flex-col gap-1.5">
                <Label for="policy-final-exam" class="text-xs font-medium text-foreground">
                  {m.settings_academic_policy_minimum_exam()}
                </Label>
                <Input
                  id="policy-final-exam"
                  type="number"
                  min="0"
                  class="h-9 text-xs"
                  bind:value={form.minimumFinalExamScore}
                  disabled={isSaving}
                />
              </div>

              <div class="flex flex-col gap-1.5">
                <Label for="policy-failed-subjects" class="text-xs font-medium text-foreground">
                  {m.settings_academic_policy_max_failed()}
                </Label>
                <Input
                  id="policy-failed-subjects"
                  type="number"
                  min="0"
                  step="1"
                  class="h-9 text-xs"
                  bind:value={form.maxFailedSubjects}
                  disabled={isSaving}
                />
              </div>

              <div class="flex flex-col gap-1.5 sm:col-span-2">
                <Label for="policy-attendance" class="text-xs font-medium text-foreground">
                  {m.settings_academic_policy_min_attendance()}
                </Label>
                <Input
                  id="policy-attendance"
                  type="number"
                  min="0"
                  max="100"
                  class="h-9 text-xs"
                  bind:value={form.minimumAttendance}
                  disabled={isSaving}
                />
              </div>
            </CardContent>
          </Card>

          <!-- Ranking & Precision -->
          <Card class="overflow-hidden border-border/70 shadow-xs">
            <CardContent class="flex flex-col gap-5 p-6">
              <div class="flex items-center justify-between {form.rankingEnabled ? 'border-b border-border/40 pb-4' : ''}">
                <span class="text-xs font-semibold text-foreground">
                  {m.settings_academic_policy_ranking_enabled()}
                </span>
                <Switch
                  id="policy-ranking"
                  checked={form.rankingEnabled}
                  onCheckedChange={(checked) => {
                    if (form) form.rankingEnabled = checked;
                  }}
                  disabled={isSaving}
                />
              </div>

              {#if form.rankingEnabled}
                <div class="grid grid-cols-1 gap-5 sm:grid-cols-2">
                  <div class="flex flex-col gap-1.5">
                    <Label class="text-xs font-medium text-foreground">
                      {m.settings_academic_policy_ranking_scope()}
                    </Label>
                    <Select
                      type="single"
                      value={form.rankingScope}
                      onValueChange={(val) => {
                        if (form && val) form.rankingScope = val as RankingScope;
                      }}
                      disabled={isSaving}
                    >
                      <SelectTrigger class="h-9 w-full text-xs">
                        {scopeLabel(form.rankingScope)}
                      </SelectTrigger>
                      <SelectContent>
                        <SelectItem value="class">{m.settings_academic_policy_scope_class()}</SelectItem>
                        <SelectItem value="grade_level">{m.settings_academic_policy_scope_grade()}</SelectItem>
                        <SelectItem value="org_unit">{m.settings_academic_policy_scope_org()}</SelectItem>
                        <SelectItem value="cohort">{m.settings_academic_policy_scope_cohort()}</SelectItem>
                      </SelectContent>
                    </Select>
                  </div>

                  <div class="flex flex-col gap-1.5">
                    <Label class="text-xs font-medium text-foreground">
                      {m.settings_academic_policy_tie_policy()}
                    </Label>
                    <Select
                      type="single"
                      value={form.tiePolicy}
                      onValueChange={(val) => {
                        if (form && val) form.tiePolicy = val as TiePolicy;
                      }}
                      disabled={isSaving}
                    >
                      <SelectTrigger class="h-9 w-full text-xs">
                        {tieLabel(form.tiePolicy)}
                      </SelectTrigger>
                      <SelectContent>
                        <SelectItem value="standard_competition">{m.settings_academic_policy_tie_standard()}</SelectItem>
                        <SelectItem value="dense">{m.settings_academic_policy_tie_dense()}</SelectItem>
                        <SelectItem value="fractional">{m.settings_academic_policy_tie_fractional()}</SelectItem>
                      </SelectContent>
                    </Select>
                  </div>

                  <div class="flex flex-col gap-1.5">
                    <Label class="text-xs font-medium text-foreground">
                      {m.settings_academic_policy_rounding()}
                    </Label>
                    <Select
                      type="single"
                      value={form.roundingMode}
                      onValueChange={(val) => {
                        if (form && val) form.roundingMode = val as RoundingMode;
                      }}
                      disabled={isSaving}
                    >
                      <SelectTrigger class="h-9 w-full text-xs">
                        {roundingLabel(form.roundingMode)}
                      </SelectTrigger>
                      <SelectContent>
                        <SelectItem value="round_half_up">{m.settings_academic_policy_round_half_up()}</SelectItem>
                        <SelectItem value="round_up">{m.settings_academic_policy_round_up()}</SelectItem>
                        <SelectItem value="round_down">{m.settings_academic_policy_round_down()}</SelectItem>
                        <SelectItem value="truncate">{m.settings_academic_policy_truncate()}</SelectItem>
                      </SelectContent>
                    </Select>
                  </div>

                  <div class="flex flex-col gap-1.5">
                    <Label for="policy-decimals" class="text-xs font-medium text-foreground">
                      {m.settings_academic_policy_decimal_places()}
                    </Label>
                    <Input
                      id="policy-decimals"
                      type="number"
                      min="0"
                      max="28"
                      step="1"
                      class="h-9 text-xs"
                      bind:value={form.decimalPlaces}
                      disabled={isSaving}
                    />
                  </div>

                  <div class="flex flex-col gap-1.5 sm:col-span-2">
                    <Label for="policy-max-rank" class="text-xs font-medium text-foreground">
                      {m.settings_academic_policy_max_rank()}
                    </Label>
                    <Input
                      id="policy-max-rank"
                      type="number"
                      min="1"
                      step="1"
                      class="h-9 text-xs"
                      bind:value={form.maxDisplayedRank}
                      disabled={isSaving}
                    />
                  </div>
                </div>
              {/if}
            </CardContent>
          </Card>
        </div>
      {:else}
        <div class="rounded-xl border border-destructive/30 bg-card p-6 text-center">
          <p class="text-sm font-semibold text-destructive">
            {m.ui_failed_to_load_academic_policies_838a32()}
          </p>
          <Button
            type="button"
            variant="outline"
            size="sm"
            class="mt-4 h-8 text-xs cursor-pointer"
            onclick={loadSettings}
          >
            {m.common_retry()}
          </Button>
        </div>
      {/if}
    </div>
  </main>

  <SettingsBottomBar
    visible={isDirty}
    {isDirty}
    {isSaving}
    onsave={savePolicy}
    onreset={resetPolicy}
  />
</div>
