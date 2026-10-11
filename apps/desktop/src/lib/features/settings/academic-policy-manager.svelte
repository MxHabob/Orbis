<script lang="ts">
  import { untrack } from "svelte";
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
  import { Calculator, Plus, Trash, Refresh } from "@orbis/icons";
  import {
    createOrgAcademicPolicyQuery,
    useUpdateAcademicPolicy,
  } from "$lib/features/academics/queries-policy";
  import { api, extractErrorMessage } from "$lib/api";
  import type {
    CalculationPreview,
    CalculationEngineSettings,
    CalculationExpression,
    CalculationInput,
    CalculationOutcomePolicy,
    CalculationPlan,
    CalculationRoundingMode,
    CalculationUnit,
  } from "$lib/api";
  import SettingsFormSkeleton from "$lib/shared/skeletons/settings-form-skeleton.svelte";

  interface InputEditor {
    name: string;
    weight: string;
    minimum: string;
    maximum: string;
    sample: string;
  }

  interface GradeBandEditor {
    code: string;
    label: string;
    minimum: string;
  }

  interface EngineForm {
    scale: string;
    outputName: string;
    normalize: boolean;
    roundingMode: CalculationRoundingMode;
    decimalPlaces: number;
    inputs: InputEditor[];
    outcomeEnabled: boolean;
    passThreshold: string;
    honorsThreshold: string;
    gradeBands: GradeBandEditor[];
    rankingEnabled: boolean;
    rankingScope: "class" | "grade_level" | "org_unit" | "cohort";
    tiePolicy: "standard_competition" | "dense" | "fractional";
    maxDisplayedRank: string;
  }

  let { orgId }: { orgId: string } = $props();
  const policyQuery = createOrgAcademicPolicyQuery(() => orgId);
  const updateMutation = useUpdateAcademicPolicy();

  const newInput = (index: number): InputEditor => ({
    name: `measure_${index + 1}`,
    weight: "1",
    minimum: "",
    maximum: "",
    sample: "0",
  });

  const initialForm = (): EngineForm => ({
    scale: "",
    outputName: "result",
    normalize: true,
    roundingMode: "round_half_up",
    decimalPlaces: 2,
    inputs: [newInput(0)],
    outcomeEnabled: false,
    passThreshold: "",
    honorsThreshold: "",
    gradeBands: [],
    rankingEnabled: true,
    rankingScope: "class",
    tiePolicy: "standard_competition",
    maxDisplayedRank: "",
  });

  let form = $state<EngineForm>(initialForm());
  let initialSnapshot = $state("");
  let initialized = $state(false);
  let customPlanReadOnly = $state(false);
  let isSaving = $state(false);
  let isPreviewing = $state(false);
  let previewResult = $state<CalculationPreview | null>(null);

  function formSnapshot(value = form): string {
    return JSON.stringify(value);
  }

  function hydrateFromSettings(settings: CalculationEngineSettings) {
    const plan = settings.calculation_plan;
    const output = plan?.outputs.length === 1 ? plan.outputs[0] : undefined;
    const expression = output?.expression;
    const weightedItems =
      expression?.type === "weighted_sum" ? expression.items : undefined;
    const variableItems = weightedItems?.every(
      (item) => item.node.type === "variable",
    );
    const unit = output?.unit;

    form.rankingEnabled = settings.ranking_enabled;
    form.rankingScope = settings.ranking_scope;
    form.tiePolicy = settings.tie_policy;
    form.maxDisplayedRank =
      settings.max_displayed_rank == null
        ? ""
        : String(settings.max_displayed_rank);

    if (
      plan &&
      output &&
      expression?.type === "weighted_sum" &&
      variableItems &&
      unit?.type === "score"
    ) {
      const inputs: CalculationInput[] = plan.inputs;
      const weights = new Map(
        weightedItems.map((item) => [
          item.node.type === "variable" ? item.node.name : "",
          String(item.weight),
        ]),
      );
      const outcomePolicy = settings.outcome_policy;
      form = {
        scale: unit.scale,
        outputName: output.name,
        normalize: expression.normalize,
        roundingMode: output.rounding?.mode ?? "round_half_up",
        decimalPlaces: output.rounding?.decimal_places ?? 2,
        inputs: inputs.map((input, index) => ({
          name: input.name,
          weight: weights.get(input.name) ?? "1",
          minimum: input.minimum == null ? "" : String(input.minimum),
          maximum: input.maximum == null ? "" : String(input.maximum),
          sample: "0",
        })),
        outcomeEnabled: outcomePolicy != null,
        passThreshold:
          outcomePolicy?.pass_at_or_above == null
            ? ""
            : String(outcomePolicy.pass_at_or_above),
        honorsThreshold:
          outcomePolicy?.honors_at_or_above == null
            ? ""
            : String(outcomePolicy.honors_at_or_above),
        gradeBands:
          outcomePolicy?.grade_bands.map((band) => ({
            code: band.code,
            label: band.label,
            minimum: String(band.minimum_score),
          })) ?? [],
      };
      customPlanReadOnly = false;
    } else if (plan) {
      customPlanReadOnly = true;
    } else {
      form = initialForm();
      customPlanReadOnly = false;
    }
    initialSnapshot = untrack(() => formSnapshot());
    initialized = true;
  }

  $effect(() => {
    if (policyQuery.data && !initialized) {
      hydrateFromSettings(policyQuery.data);
    }
  });

  const isDirty = $derived(
    initialSnapshot !== "" && formSnapshot() !== initialSnapshot,
  );

  function buildPlan(): CalculationPlan {
    const unit: CalculationUnit = { type: "score", scale: form.scale.trim() };
    const inputs: CalculationInput[] = form.inputs.map((input) => ({
      name: input.name.trim(),
      unit,
      minimum: input.minimum.trim() === "" ? null : input.minimum,
      maximum: input.maximum.trim() === "" ? null : input.maximum,
      is_required: true,
    }));
    const expression: CalculationExpression = {
      type: "weighted_sum",
      items: form.inputs.map((input) => ({
        node: { type: "variable", name: input.name.trim() },
        weight: input.weight,
      })),
      normalize: form.normalize,
    };

    return {
      version: (policyQuery.data?.calculation_plan?.version ?? 0) + 1,
      inputs,
      outputs: [
        {
          name: form.outputName.trim(),
          unit,
          expression,
          rounding: {
            mode: form.roundingMode,
            decimal_places: Number(form.decimalPlaces),
          },
        },
      ],
    };
  }

  function buildOutcomePolicy(): CalculationOutcomePolicy | null {
    if (!form.outcomeEnabled) return null;
    return {
      unit: { type: "score", scale: form.scale.trim() },
      pass_at_or_above:
        form.passThreshold.trim() === "" ? null : form.passThreshold,
      honors_at_or_above:
        form.honorsThreshold.trim() === "" ? null : form.honorsThreshold,
      grade_bands: form.gradeBands.map((band) => ({
        code: band.code.trim(),
        label: band.label.trim(),
        minimum_score: band.minimum,
      })),
    };
  }

  function getValidationError(): string | null {
    if (customPlanReadOnly) return null;
    if (!form.scale.trim()) return "Specify a measurement scale before saving.";
    if (!form.outputName.trim()) return "Specify an output name before saving.";
    if (!form.inputs.length) return "Add at least one calculation input.";
    const names = form.inputs.map((input) => input.name.trim());
    if (names.some((name) => !name)) return "Every calculation input needs a name.";
    if (new Set(names).size !== names.length) {
      return "Calculation input names must be unique.";
    }
    if (
      form.inputs.some(
        (input) => !Number.isFinite(Number(input.weight)) || Number(input.weight) < 0,
      )
    ) {
      return "Input weights must be non-negative numbers.";
    }
    if (!form.inputs.some((input) => Number(input.weight) > 0)) {
      return "At least one input must have a positive weight.";
    }
    if (!Number.isInteger(Number(form.decimalPlaces)) || form.decimalPlaces < 0 || form.decimalPlaces > 28) {
      return "Decimal places must be between 0 and 28.";
    }
    if (
      form.inputs.some(
        (input) =>
          input.minimum.trim() !== "" &&
          input.maximum.trim() !== "" &&
          Number(input.minimum) > Number(input.maximum),
      )
    ) {
      return "An input minimum cannot exceed its maximum.";
    }
    if (
      form.outcomeEnabled &&
      form.passThreshold.trim() !== "" &&
      form.honorsThreshold.trim() !== "" &&
      Number(form.honorsThreshold) < Number(form.passThreshold)
    ) {
      return "The honors threshold cannot be lower than the passing threshold.";
    }
    return null;
  }

  async function handleSave() {
    const validationError = getValidationError();
    if (validationError) {
      toast.error(validationError);
      return;
    }
    isSaving = true;
    try {
      const saved = await updateMutation.mutateAsync({
        calculation_plan: customPlanReadOnly
          ? policyQuery.data?.calculation_plan ?? null
          : buildPlan(),
        outcome_policy: customPlanReadOnly
          ? policyQuery.data?.outcome_policy ?? null
          : buildOutcomePolicy(),
        ranking_enabled: form.rankingEnabled,
        ranking_scope: form.rankingScope,
        tie_policy: form.tiePolicy,
        max_displayed_rank:
          form.maxDisplayedRank.trim() === ""
            ? null
            : Number(form.maxDisplayedRank),
      });
      hydrateFromSettings(saved);
      previewResult = null;
      toast.success("Calculation policy saved.");
    } catch (error: unknown) {
      toast.error(extractErrorMessage(error));
    } finally {
      isSaving = false;
    }
  }

  async function handlePreview() {
    const validationError = getValidationError();
    if (validationError) {
      toast.error(validationError);
      return;
    }
    isPreviewing = true;
    try {
      previewResult = await api.previewCalculation(
        Object.fromEntries(
          form.inputs.map((input) => [
            input.name.trim(),
            input.sample.trim() === "" ? "0" : input.sample,
          ]),
        ),
      );
    } catch (error: unknown) {
      toast.error(extractErrorMessage(error));
    } finally {
      isPreviewing = false;
    }
  }

  function resetForm() {
    if (!initialSnapshot) return;
    form = JSON.parse(initialSnapshot) as EngineForm;
  }

  function replaceCustomPlan() {
    form = {
      ...form,
      ...initialForm(),
      rankingEnabled: form.rankingEnabled,
      rankingScope: form.rankingScope,
      tiePolicy: form.tiePolicy,
      maxDisplayedRank: form.maxDisplayedRank,
    };
    form.scale = "score";
    customPlanReadOnly = false;
  }
</script>

<div class="flex h-full w-full flex-col gap-6 overflow-y-auto p-6 scrollbar-hide">
  {#if policyQuery.isLoading}
    <div class="w-full"><SettingsFormSkeleton /></div>
  {:else if policyQuery.error}
    <div class="w-full py-12 text-center text-destructive">
      <p>{m.ui_failed_to_load_academic_policies_838a32()}</p>
      <Button
        variant="outline"
        class="mt-4 cursor-pointer"
        onclick={() => policyQuery.refetch()}
      >
        {m.common_retry()}
      </Button>
    </div>
  {:else}
    <div class="flex w-full max-w-5xl flex-col gap-5">
      <header class="flex items-center gap-3 border-b border-border pb-3">
        <Calculator size={18} class="text-primary" />
        <div>
          <h1 class="text-sm font-semibold text-foreground">Calculation engine</h1>
          <p class="mt-1 text-xs text-muted-foreground">
            Configure this organization’s typed measure, aggregation, rounding, and outcome rules.
          </p>
        </div>
      </header>

      {#if customPlanReadOnly}
        <Card>
          <CardHeader>
            <CardTitle>Existing calculation policy</CardTitle>
            <CardDescription>
              This policy uses formula nodes not represented by the visual weighted-aggregation editor. It remains unchanged until you explicitly replace it.
            </CardDescription>
          </CardHeader>
          <CardContent class="flex flex-wrap items-center gap-3">
            <span class="text-xs text-muted-foreground">
              Version {policyQuery.data?.calculation_plan?.version}
            </span>
            <Button variant="outline" onclick={replaceCustomPlan}>
              Replace with weighted aggregation
            </Button>
          </CardContent>
        </Card>
      {:else}
        <Card>
          <CardHeader>
            <CardTitle>Calculation plan</CardTitle>
            <CardDescription>
              Inputs share one configured score scale. Choose a weighted sum or normalized weighted average; the engine validates units and ranges before saving.
            </CardDescription>
          </CardHeader>
          <CardContent class="flex flex-col gap-5">
            <div class="grid gap-4 sm:grid-cols-2">
              <div class="flex flex-col gap-1.5">
                <Label for="calculation-scale">Measure scale</Label>
                <Input
                  id="calculation-scale"
                  bind:value={form.scale}
                  placeholder="For example: points, percentage, GPA"
                  disabled={isSaving}
                />
              </div>
              <div class="flex flex-col gap-1.5">
                <Label for="calculation-output">Output name</Label>
                <Input
                  id="calculation-output"
                  bind:value={form.outputName}
                  disabled={isSaving}
                />
              </div>
            </div>

            <div class="flex flex-wrap items-center justify-between gap-3 rounded-md border border-border p-3">
              <div>
                <Label for="normalize-weights">Normalize weighted result</Label>
                <p class="mt-1 text-[11px] text-muted-foreground">
                  Enabled calculates a weighted average; disabled returns the weighted sum.
                </p>
              </div>
              <Switch
                id="normalize-weights"
                checked={form.normalize}
                onCheckedChange={(checked) => (form.normalize = checked)}
                disabled={isSaving}
              />
            </div>

            <div class="flex flex-col gap-3">
              <div class="flex items-center justify-between gap-3">
                <h2 class="text-xs font-semibold">Calculation inputs</h2>
                <Button
                  type="button"
                  size="sm"
                  variant="outline"
                  class="gap-1.5"
                  onclick={() => form.inputs.push(newInput(form.inputs.length))}
                  disabled={isSaving}
                >
                  <Plus size={13} /> Add input
                </Button>
              </div>
              {#each form.inputs as input, index (index)}
                <div class="grid gap-3 rounded-md border border-border p-3 sm:grid-cols-6">
                  <div class="flex flex-col gap-1.5 sm:col-span-2">
                    <Label for={`input-name-${index}`}>Input name</Label>
                    <Input id={`input-name-${index}`} bind:value={input.name} disabled={isSaving} />
                  </div>
                  <div class="flex flex-col gap-1.5">
                    <Label for={`input-weight-${index}`}>Weight</Label>
                    <Input id={`input-weight-${index}`} type="number" min="0" step="any" bind:value={input.weight} disabled={isSaving} />
                  </div>
                  <div class="flex flex-col gap-1.5">
                    <Label for={`input-minimum-${index}`}>Minimum</Label>
                    <Input id={`input-minimum-${index}`} type="number" step="any" bind:value={input.minimum} disabled={isSaving} />
                  </div>
                  <div class="flex flex-col gap-1.5">
                    <Label for={`input-maximum-${index}`}>Maximum</Label>
                    <Input id={`input-maximum-${index}`} type="number" step="any" bind:value={input.maximum} disabled={isSaving} />
                  </div>
                  <div class="flex flex-col gap-1.5">
                    <Label for={`input-sample-${index}`}>Preview value</Label>
                    <Input id={`input-sample-${index}`} type="number" step="any" bind:value={input.sample} disabled={isSaving} />
                  </div>
                  {#if form.inputs.length > 1}
                    <div class="flex items-end sm:col-span-6 sm:justify-end">
                      <Button
                        type="button"
                        size="icon-sm"
                        variant="ghost"
                        aria-label="Remove input"
                        onclick={() => form.inputs.splice(index, 1)}
                        disabled={isSaving}
                      >
                        <Trash size={14} />
                      </Button>
                    </div>
                  {/if}
                </div>
              {/each}
            </div>

            <div class="grid gap-4 sm:grid-cols-2">
              <div class="flex flex-col gap-1.5">
                <Label for="rounding-mode">Rounding mode</Label>
                <select
                  id="rounding-mode"
                  class="h-9 rounded-md border border-input bg-background px-3 text-xs"
                  bind:value={form.roundingMode}
                  disabled={isSaving}
                >
                  <option value="round_half_up">Round half up</option>
                  <option value="round_up">Round away from zero</option>
                  <option value="round_down">Round toward zero</option>
                  <option value="truncate">Truncate</option>
                </select>
              </div>
              <div class="flex flex-col gap-1.5">
                <Label for="decimal-places">Decimal places</Label>
                <Input
                  id="decimal-places"
                  type="number"
                  min="0"
                  max="28"
                  step="1"
                  bind:value={form.decimalPlaces}
                  disabled={isSaving}
                />
              </div>
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>Outcome policy</CardTitle>
            <CardDescription>
              Pass thresholds and grade bands are optional and use the configured output scale.
            </CardDescription>
          </CardHeader>
          <CardContent class="flex flex-col gap-4">
            <div class="flex items-center justify-between gap-3">
              <Label for="outcome-policy-enabled">Evaluate outcome thresholds</Label>
              <Switch
                id="outcome-policy-enabled"
                checked={form.outcomeEnabled}
                onCheckedChange={(checked) => (form.outcomeEnabled = checked)}
                disabled={isSaving}
              />
            </div>
            {#if form.outcomeEnabled}
              <div class="grid gap-4 sm:grid-cols-2">
                <div class="flex flex-col gap-1.5">
                  <Label for="passing-threshold">Passing threshold</Label>
                  <Input id="passing-threshold" type="number" step="any" bind:value={form.passThreshold} disabled={isSaving} />
                </div>
                <div class="flex flex-col gap-1.5">
                  <Label for="honors-threshold">Honors threshold</Label>
                  <Input id="honors-threshold" type="number" step="any" bind:value={form.honorsThreshold} disabled={isSaving} />
                </div>
              </div>

              <div class="flex flex-col gap-3">
                <div class="flex items-center justify-between gap-3">
                  <h3 class="text-xs font-semibold">Grade bands</h3>
                  <Button
                    type="button"
                    size="sm"
                    variant="outline"
                    class="gap-1.5"
                    onclick={() =>
                      form.gradeBands.push({
                        code: `band_${form.gradeBands.length + 1}`,
                        label: "",
                        minimum: "",
                      })}
                    disabled={isSaving}
                  >
                    <Plus size={13} /> Add band
                  </Button>
                </div>
                {#each form.gradeBands as band, index (index)}
                  <div class="grid gap-3 rounded-md border border-border p-3 sm:grid-cols-4">
                    <div class="flex flex-col gap-1.5">
                      <Label for={`band-code-${index}`}>Code</Label>
                      <Input id={`band-code-${index}`} bind:value={band.code} disabled={isSaving} />
                    </div>
                    <div class="flex flex-col gap-1.5 sm:col-span-2">
                      <Label for={`band-label-${index}`}>Label</Label>
                      <Input id={`band-label-${index}`} bind:value={band.label} disabled={isSaving} />
                    </div>
                    <div class="flex flex-col gap-1.5">
                      <Label for={`band-minimum-${index}`}>Minimum score</Label>
                      <Input id={`band-minimum-${index}`} type="number" step="any" bind:value={band.minimum} disabled={isSaving} />
                    </div>
                    <div class="sm:col-span-4 sm:flex sm:justify-end">
                      <Button
                        type="button"
                        size="icon-sm"
                        variant="ghost"
                        aria-label="Remove grade band"
                        onclick={() => form.gradeBands.splice(index, 1)}
                        disabled={isSaving}
                      >
                        <Trash size={14} />
                      </Button>
                    </div>
                  </div>
                {/each}
              </div>
            {/if}
          </CardContent>
        </Card>
      {/if}

      <Card>
        <CardHeader>
          <CardTitle>Ranking policy</CardTitle>
          <CardDescription>
            Rankings use this organization-level tie rule and comparison scope when requested.
          </CardDescription>
        </CardHeader>
        <CardContent class="flex flex-col gap-4">
          <div class="flex items-center justify-between gap-3">
            <Label for="ranking-enabled">Enable ranking calculations</Label>
            <Switch
              id="ranking-enabled"
              checked={form.rankingEnabled}
              onCheckedChange={(checked) => (form.rankingEnabled = checked)}
              disabled={isSaving}
            />
          </div>
          <div class="grid gap-4 sm:grid-cols-3">
            <div class="flex flex-col gap-1.5">
              <Label for="ranking-scope">Comparison scope</Label>
              <select
                id="ranking-scope"
                class="h-9 rounded-md border border-input bg-background px-3 text-xs"
                bind:value={form.rankingScope}
                disabled={isSaving || !form.rankingEnabled}
              >
                <option value="class">Class or group</option>
                <option value="grade_level">Level</option>
                <option value="org_unit">Organization unit</option>
                <option value="cohort">Cohort</option>
              </select>
            </div>
            <div class="flex flex-col gap-1.5">
              <Label for="tie-policy">Tie handling</Label>
              <select
                id="tie-policy"
                class="h-9 rounded-md border border-input bg-background px-3 text-xs"
                bind:value={form.tiePolicy}
                disabled={isSaving || !form.rankingEnabled}
              >
                <option value="standard_competition">Standard competition</option>
                <option value="dense">Dense</option>
                <option value="fractional">Fractional</option>
              </select>
            </div>
            <div class="flex flex-col gap-1.5">
              <Label for="max-displayed-rank">Display rank limit (blank means no limit)</Label>
              <Input
                id="max-displayed-rank"
                type="number"
                min="1"
                step="1"
                bind:value={form.maxDisplayedRank}
                disabled={isSaving || !form.rankingEnabled}
              />
            </div>
          </div>
        </CardContent>
      </Card>

      {#if previewResult}
        <Card>
          <CardHeader>
            <CardTitle>Calculation preview</CardTitle>
            <CardDescription>
              Policy version {previewResult.result.policy_version}
            </CardDescription>
          </CardHeader>
          <CardContent class="flex flex-wrap gap-3">
            {#each previewResult.result.outputs as output (output.name)}
              <div class="rounded-md border border-border px-3 py-2">
                <p class="text-[11px] text-muted-foreground">{output.name}</p>
                <p class="font-mono text-sm font-semibold">{output.value}</p>
              </div>
            {/each}
            {#each previewResult.outcomes as result (result.output_name)}
              <div class="rounded-md border border-border px-3 py-2">
                <p class="text-[11px] text-muted-foreground">{result.output_name} outcome</p>
                <p class="text-xs font-semibold">
                  {result.outcome.grade_code ?? (result.outcome.is_passed == null
                    ? "No grade band"
                    : result.outcome.is_passed
                      ? "Passed"
                      : "Not passed")}
                </p>
              </div>
            {/each}
          </CardContent>
        </Card>
      {/if}

      <div class="flex flex-wrap justify-end gap-2 border-t border-border pt-4">
        {#if !customPlanReadOnly}
          <Button
            variant="outline"
            class="gap-1.5"
            onclick={handlePreview}
            disabled={isSaving || isPreviewing}
          >
            <Refresh size={14} /> Preview with sample values
          </Button>
        {/if}
        {#if !customPlanReadOnly}
          <Button
            variant="outline"
            onclick={resetForm}
            disabled={!isDirty || isSaving}
          >
            Reset
          </Button>
        {/if}
        <Button onclick={handleSave} disabled={isSaving || !isDirty}>
          {isSaving ? "Saving…" : "Save calculation policy"}
        </Button>
      </div>
    </div>
  {/if}
</div>
