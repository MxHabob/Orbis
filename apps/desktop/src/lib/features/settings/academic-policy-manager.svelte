<script lang="ts">
  import { untrack } from "svelte";
  import * as m from "$lib/paraglide/messages";
  import { Switch, toast } from "@orbis/ui";
  import { Button } from "@orbis/ui/button";
  import { Input } from "@orbis/ui/input";
  import { Label } from "@orbis/ui/label";
  import { Card, CardContent } from "@orbis/ui/card";
  import { Badge } from "@orbis/ui/badge";
  import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
  } from "@orbis/ui/select";
  import { Plus, Trash, Refresh } from "@orbis/icons";
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
  import SettingsBottomBar from "$lib/shared/components/settings-bottom-bar.svelte";

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

  let {
    orgId,
    onSaved,
  }: { orgId: string; onSaved?: () => void | Promise<void> } = $props();
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

  function resolveScale(rawUnit: any): string {
    if (!rawUnit) return "percentage";
    if (typeof rawUnit.scale === "string") return rawUnit.scale;
    if (
      typeof rawUnit.scale === "object" &&
      rawUnit.scale !== null &&
      typeof rawUnit.scale.scale === "string"
    ) {
      return rawUnit.scale.scale;
    }
    return "percentage";
  }

  interface PolicyPreset {
    id: string;
    label: string;
    description: string;
    badge: string;
    badgeVariant: "default" | "secondary" | "outline";
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
  }

  const YEMENI_POLICY_PRESETS: PolicyPreset[] = [
    {
      id: "yemen_school_term",
      label: "النظام المدرسي العام المعتمد (وزارة التربية والتعليم)",
      description: "محصلة الفصل من 50 درجة: محصلة الأعمال الشهرية (20 درجة / 40%) + اختبار نهاية الفصل (30 درجة / 60%، شرط اجتياز 10)",
      badge: "معتمد وزارياً",
      badgeVariant: "default",
      scale: "percentage",
      outputName: "term_result",
      normalize: true,
      roundingMode: "round_half_up",
      decimalPlaces: 2,
      inputs: [
        {
          name: "coursework",
          weight: "0.4",
          minimum: "0",
          maximum: "20",
          sample: "18",
        },
        {
          name: "examination",
          weight: "0.6",
          minimum: "0",
          maximum: "30",
          sample: "25",
        },
      ],
      outcomeEnabled: true,
      passThreshold: "50",
      honorsThreshold: "90",
      gradeBands: [
        { code: "A", label: "ممتاز", minimum: "90" },
        { code: "B", label: "جيد جداً", minimum: "80" },
        { code: "C", label: "جيد", minimum: "65" },
        { code: "D", label: "مقبول", minimum: "50" },
        { code: "F", label: "راسب", minimum: "0" },
      ],
    },
    {
      id: "yemen_thanawiya_cert",
      label: "الشهادة الثانوية العامة - التراكمي (الصف 12)",
      description: "المجموع الكلي من 100%: الاختبار الوزاري النهائي (80%) + المعدل التراكمي للسنوات السابقة (20%)",
      badge: "شهادة عامة",
      badgeVariant: "secondary",
      scale: "percentage",
      outputName: "thanawiya_score",
      normalize: true,
      roundingMode: "round_half_up",
      decimalPlaces: 2,
      inputs: [
        {
          name: "ministerial_exam",
          weight: "0.8",
          minimum: "0",
          maximum: "100",
          sample: "85",
        },
        {
          name: "cumulative_prior",
          weight: "0.2",
          minimum: "0",
          maximum: "100",
          sample: "88",
        },
      ],
      outcomeEnabled: true,
      passThreshold: "50",
      honorsThreshold: "90",
      gradeBands: [
        { code: "A", label: "ممتاز", minimum: "90" },
        { code: "B", label: "جيد جداً", minimum: "80" },
        { code: "C", label: "جيد", minimum: "65" },
        { code: "D", label: "مقبول", minimum: "50" },
        { code: "F", label: "راسب", minimum: "0" },
      ],
    },
    {
      id: "university_credit",
      label: "التعليم الجامعي (الساعات المعتمدة)",
      description: "تقييم المقرر: أعمال دراسية (20%) + اختبار نصفي (20%) + اختبار نهائي (60%)",
      badge: "جامعي",
      badgeVariant: "outline",
      scale: "percentage",
      outputName: "gpa_course",
      normalize: true,
      roundingMode: "round_half_up",
      decimalPlaces: 2,
      inputs: [
        {
          name: "coursework",
          weight: "0.2",
          minimum: "0",
          maximum: "100",
          sample: "80",
        },
        {
          name: "midterm",
          weight: "0.2",
          minimum: "0",
          maximum: "100",
          sample: "75",
        },
        {
          name: "final_exam",
          weight: "0.6",
          minimum: "0",
          maximum: "100",
          sample: "82",
        },
      ],
      outcomeEnabled: true,
      passThreshold: "60",
      honorsThreshold: "90",
      gradeBands: [
        { code: "A", label: "ممتاز", minimum: "90" },
        { code: "B", label: "جيد جداً", minimum: "80" },
        { code: "C", label: "جيد", minimum: "70" },
        { code: "D", label: "مقبول", minimum: "60" },
        { code: "F", label: "راسب", minimum: "0" },
      ],
    },
  ];

  function applyPreset(preset: PolicyPreset) {
    form.scale = preset.scale;
    form.outputName = preset.outputName;
    form.normalize = preset.normalize;
    form.roundingMode = preset.roundingMode;
    form.decimalPlaces = preset.decimalPlaces;
    form.inputs = preset.inputs.map((inp) => ({ ...inp }));
    form.outcomeEnabled = preset.outcomeEnabled;
    form.passThreshold = preset.passThreshold;
    form.honorsThreshold = preset.honorsThreshold;
    form.gradeBands = preset.gradeBands.map((gb) => ({ ...gb }));
    customPlanReadOnly = false;
    toast.success(`تم تطبيق قالب: ${preset.label}`);
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
        scale: resolveScale(unit),
        outputName: output.name,
        normalize: expression.normalize,
        roundingMode: output.rounding?.mode ?? "round_half_up",
        decimalPlaces: output.rounding?.decimal_places ?? 2,
        inputs: inputs.map((input) => ({
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
        rankingEnabled: settings.ranking_enabled,
        rankingScope: settings.ranking_scope,
        tiePolicy: settings.tie_policy,
        maxDisplayedRank:
          settings.max_displayed_rank == null
            ? ""
            : String(settings.max_displayed_rank),
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

  function roundingLabel(mode: CalculationRoundingMode): string {
    switch (mode) {
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

  function scopeLabel(
    scope: "class" | "grade_level" | "org_unit" | "cohort",
  ): string {
    switch (scope) {
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

  function tieLabel(
    policy: "standard_competition" | "dense" | "fractional",
  ): string {
    switch (policy) {
      case "standard_competition":
        return m.settings_academic_policy_tie_standard();
      case "dense":
        return m.settings_academic_policy_tie_dense();
      case "fractional":
        return m.settings_academic_policy_tie_fractional();
    }
  }

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
    if (!form.scale.trim()) return `${m.settings_calc_scale()}: Required`;
    if (!form.outputName.trim())
      return `${m.settings_calc_output_name()}: Required`;
    if (!form.inputs.length) return `${m.settings_calc_inputs()}: Required`;
    const names = form.inputs.map((input) => input.name.trim());
    if (names.some((name) => !name))
      return "Every calculation input needs a name.";
    if (new Set(names).size !== names.length) {
      return "Calculation input names must be unique.";
    }
    if (
      form.inputs.some(
        (input) =>
          !Number.isFinite(Number(input.weight)) || Number(input.weight) < 0,
      )
    ) {
      return "Input weights must be non-negative numbers.";
    }
    if (!form.inputs.some((input) => Number(input.weight) > 0)) {
      return "At least one input must have a positive weight.";
    }
    if (
      !Number.isInteger(Number(form.decimalPlaces)) ||
      form.decimalPlaces < 0 ||
      form.decimalPlaces > 28
    ) {
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
          ? (policyQuery.data?.calculation_plan ?? null)
          : buildPlan(),
        outcome_policy: customPlanReadOnly
          ? (policyQuery.data?.outcome_policy ?? null)
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
      toast.success(m.settings_calc_saved());
      await onSaved?.();
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

<div class="h-full w-full flex flex-col overflow-hidden bg-background">
  <main class="min-h-0 flex-1 overflow-y-auto p-6 scrollbar-hide pb-28">
    <div class="w-full flex flex-col gap-6">
      {#if policyQuery.isLoading}
        <SettingsFormSkeleton />
      {:else if policyQuery.error}
        <div class="rounded-xl border border-destructive/30 bg-card p-6 text-center">
          <p class="text-sm font-semibold text-destructive">
            {m.ui_failed_to_load_academic_policies_838a32()}
          </p>
          <Button
            type="button"
            variant="outline"
            size="sm"
            class="mt-4 h-8 text-xs cursor-pointer"
            onclick={() => policyQuery.refetch()}
          >
            {m.common_retry()}
          </Button>
        </div>
      {:else}
        {#if customPlanReadOnly}
          <Card class="overflow-hidden border-border/70 shadow-xs">
            <CardContent class="flex items-center justify-between gap-4 p-5">
              <div class="flex items-center gap-3">
                <span class="inline-flex items-center rounded bg-muted px-2 py-0.5 font-mono text-[11px] text-muted-foreground">
                  v{policyQuery.data?.calculation_plan?.version}
                </span>
                <span class="text-xs font-medium text-foreground">
                  {m.settings_calc_custom_plan_badge()}
                </span>
              </div>
              <Button
                variant="outline"
                size="sm"
                class="h-8 text-xs cursor-pointer"
                onclick={replaceCustomPlan}
              >
                {m.settings_calc_replace_custom()}
              </Button>
            </CardContent>
          </Card>
        {:else}
          <!-- Yemeni Presets Selector -->
          <div class="rounded-xl border border-primary/20 bg-primary/5 p-4 space-y-3">
            <div class="flex items-center justify-between">
              <div>
                <h3 class="text-sm font-semibold text-foreground flex items-center gap-2">
                  <span>🏛️</span>
                  <span>قوالب ومعايير محرك الحساب المعتمدة لليمن</span>
                </h3>
                <p class="text-xs text-muted-foreground mt-0.5">
                  اضغط على أحد القوالب المعيارية المعتمدة رسمياً لتعبئة مدخلات ومقاييس الحساب والتقديرات تلقائياً
                </p>
              </div>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-3 gap-3">
              {#each YEMENI_POLICY_PRESETS as preset}
                <button
                  type="button"
                  class="flex flex-col text-start p-3 rounded-lg border border-border/70 bg-card hover:border-primary hover:bg-card/80 transition-all cursor-pointer group shadow-xs"
                  onclick={() => applyPreset(preset)}
                >
                  <div class="flex items-center justify-between w-full mb-1">
                    <span class="text-xs font-semibold text-foreground group-hover:text-primary transition-colors">
                      {preset.label}
                    </span>
                    <Badge variant={preset.badgeVariant} class="text-[10px] px-1.5 py-0 h-4">
                      {preset.badge}
                    </Badge>
                  </div>
                  <p class="text-[11px] text-muted-foreground line-clamp-2 leading-relaxed">
                    {preset.description}
                  </p>
                </button>
              {/each}
            </div>
          </div>

          <!-- Core Calculation Parameters -->
          <Card class="overflow-hidden border-border/70 shadow-xs">
            <CardContent class="grid grid-cols-1 gap-5 p-6 sm:grid-cols-2 lg:grid-cols-4">
              <div class="flex flex-col gap-1.5">
                <Label for="calc-scale" class="text-xs font-medium text-foreground">
                  {m.settings_calc_scale()}
                </Label>
                <Input
                  id="calc-scale"
                  class="h-9 text-xs"
                  placeholder={m.settings_calc_scale_placeholder()}
                  bind:value={form.scale}
                  disabled={isSaving}
                />
              </div>

              <div class="flex flex-col gap-1.5">
                <Label for="calc-output" class="text-xs font-medium text-foreground">
                  {m.settings_calc_output_name()}
                </Label>
                <Input
                  id="calc-output"
                  class="h-9 text-xs"
                  bind:value={form.outputName}
                  disabled={isSaving}
                />
              </div>

              <div class="flex flex-col gap-1.5">
                <Label class="text-xs font-medium text-foreground">
                  {m.settings_academic_policy_rounding()}
                </Label>
                <Select
                  type="single"
                  value={form.roundingMode}
                  onValueChange={(val) => {
                    if (val) form.roundingMode = val as CalculationRoundingMode;
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
                <Label for="calc-decimals" class="text-xs font-medium text-foreground">
                  {m.settings_academic_policy_decimal_places()}
                </Label>
                <Input
                  id="calc-decimals"
                  type="number"
                  min="0"
                  max="28"
                  step="1"
                  class="h-9 text-xs"
                  bind:value={form.decimalPlaces}
                  disabled={isSaving}
                />
              </div>

              <div class="flex items-center justify-between border-t border-border/40 pt-4 sm:col-span-2 lg:col-span-4">
                <span class="text-xs font-medium text-foreground">
                  {m.settings_calc_normalize()}
                </span>
                <Switch
                  id="normalize-weights"
                  checked={form.normalize}
                  onCheckedChange={(checked) => (form.normalize = checked)}
                  disabled={isSaving}
                />
              </div>
            </CardContent>
          </Card>

          <!-- Calculation Measures / Inputs -->
          <Card class="overflow-hidden border-border/70 shadow-xs">
            <CardContent class="flex flex-col gap-4 p-6">
              <div class="flex items-center justify-between border-b border-border/40 pb-3">
                <div class="flex items-center gap-2">
                  <span class="text-xs font-semibold text-foreground">
                    {m.settings_calc_inputs()}
                  </span>
                  <span class="inline-flex items-center justify-center rounded-full bg-muted px-2 py-0.5 text-[10px] font-medium text-muted-foreground">
                    {form.inputs.length}
                  </span>
                </div>
                <Button
                  type="button"
                  size="sm"
                  variant="outline"
                  class="h-8 gap-1.5 text-xs cursor-pointer"
                  onclick={() => form.inputs.push(newInput(form.inputs.length))}
                  disabled={isSaving}
                >
                  <Plus size={13} /> {m.settings_calc_add_input()}
                </Button>
              </div>

              <!-- Column Header (Desktop) -->
              <div class="hidden grid-cols-12 gap-3 px-1 text-[11px] font-medium text-muted-foreground sm:grid">
                <span class="col-span-3">{m.settings_calc_input_name()}</span>
                <span class="col-span-2">{m.settings_calc_weight()}</span>
                <span class="col-span-2">{m.settings_calc_min()}</span>
                <span class="col-span-2">{m.settings_calc_max()}</span>
                <span class="col-span-2">{m.settings_calc_sample()}</span>
                <span class="col-span-1 text-center"></span>
              </div>

              <div class="flex flex-col gap-2.5">
                {#each form.inputs as input, index (index)}
                  <div class="grid grid-cols-1 items-center gap-2.5 rounded-lg border border-border/50 bg-card/40 p-3 sm:grid-cols-12 sm:border-0 sm:bg-transparent sm:p-0">
                    <div class="sm:col-span-3">
                      <Input
                        class="h-8 text-xs"
                        placeholder={m.settings_calc_input_name()}
                        bind:value={input.name}
                        disabled={isSaving}
                      />
                    </div>
                    <div class="sm:col-span-2">
                      <Input
                        type="number"
                        min="0"
                        step="any"
                        class="h-8 text-xs"
                        placeholder={m.settings_calc_weight()}
                        bind:value={input.weight}
                        disabled={isSaving}
                      />
                    </div>
                    <div class="sm:col-span-2">
                      <Input
                        type="number"
                        step="any"
                        class="h-8 text-xs"
                        placeholder={m.settings_calc_min()}
                        bind:value={input.minimum}
                        disabled={isSaving}
                      />
                    </div>
                    <div class="sm:col-span-2">
                      <Input
                        type="number"
                        step="any"
                        class="h-8 text-xs"
                        placeholder={m.settings_calc_max()}
                        bind:value={input.maximum}
                        disabled={isSaving}
                      />
                    </div>
                    <div class="sm:col-span-2">
                      <Input
                        type="number"
                        step="any"
                        class="h-8 text-xs"
                        placeholder={m.settings_calc_sample()}
                        bind:value={input.sample}
                        disabled={isSaving}
                      />
                    </div>
                    <div class="flex justify-end sm:col-span-1 sm:justify-center">
                      {#if form.inputs.length > 1}
                        <Button
                          type="button"
                          size="icon-sm"
                          variant="ghost"
                          class="h-8 w-8 text-muted-foreground hover:text-destructive cursor-pointer"
                          aria-label="Remove input"
                          onclick={() => form.inputs.splice(index, 1)}
                          disabled={isSaving}
                        >
                          <Trash size={14} />
                        </Button>
                      {/if}
                    </div>
                  </div>
                {/each}
              </div>
            </CardContent>
          </Card>
        {/if}

        <!-- Dual Column Grid on Widescreen: Outcomes & Ranking/Preview -->
        <div class="grid grid-cols-1 xl:grid-cols-2 gap-6 w-full items-start">
          <!-- Outcome Evaluation & Grade Bands -->
          {#if !customPlanReadOnly}
            <Card class="overflow-hidden border-border/70 shadow-xs">
              <CardContent class="flex flex-col gap-5 p-6">
                <div class="flex items-center justify-between {form.outcomeEnabled ? 'border-b border-border/40 pb-4' : ''}">
                  <span class="text-xs font-semibold text-foreground">
                    {m.settings_calc_outcome_eval()}
                  </span>
                  <Switch
                    id="outcome-policy-enabled"
                    checked={form.outcomeEnabled}
                    onCheckedChange={(checked) => (form.outcomeEnabled = checked)}
                    disabled={isSaving}
                  />
                </div>

                {#if form.outcomeEnabled}
                  <div class="grid grid-cols-1 gap-5 sm:grid-cols-2">
                    <div class="flex flex-col gap-1.5">
                      <Label for="passing-threshold" class="text-xs font-medium text-foreground">
                        {m.settings_academic_policy_passing_threshold()}
                      </Label>
                      <Input
                        id="passing-threshold"
                        type="number"
                        step="any"
                        class="h-9 text-xs"
                        bind:value={form.passThreshold}
                        disabled={isSaving}
                      />
                    </div>

                    <div class="flex flex-col gap-1.5">
                      <Label for="honors-threshold" class="text-xs font-medium text-foreground">
                        {m.settings_academic_policy_honors_threshold()}
                      </Label>
                      <Input
                        id="honors-threshold"
                        type="number"
                        step="any"
                        class="h-9 text-xs"
                        bind:value={form.honorsThreshold}
                        disabled={isSaving}
                      />
                    </div>
                  </div>

                  <div class="flex flex-col gap-4 border-t border-border/40 pt-4">
                    <div class="flex items-center justify-between">
                      <div class="flex items-center gap-2">
                        <span class="text-xs font-semibold text-foreground">
                          {m.settings_calc_grade_bands()}
                        </span>
                        <span class="inline-flex items-center justify-center rounded-full bg-muted px-2 py-0.5 text-[10px] font-medium text-muted-foreground">
                          {form.gradeBands.length}
                        </span>
                      </div>
                      <Button
                        type="button"
                        size="sm"
                        variant="outline"
                        class="h-8 gap-1.5 text-xs cursor-pointer"
                        onclick={() =>
                          form.gradeBands.push({
                            code: `band_${form.gradeBands.length + 1}`,
                            label: "",
                            minimum: "",
                          })}
                        disabled={isSaving}
                      >
                        <Plus size={13} /> {m.settings_calc_add_band()}
                      </Button>
                    </div>

                    {#if form.gradeBands.length > 0}
                      <div class="hidden grid-cols-12 gap-3 px-1 text-[11px] font-medium text-muted-foreground sm:grid">
                        <span class="col-span-3">{m.settings_calc_band_code()}</span>
                        <span class="col-span-5">{m.settings_calc_band_label()}</span>
                        <span class="col-span-3">{m.settings_calc_band_min()}</span>
                        <span class="col-span-1 text-center"></span>
                      </div>

                      <div class="flex flex-col gap-2.5">
                        {#each form.gradeBands as band, index (index)}
                          <div class="grid grid-cols-1 items-center gap-2.5 rounded-lg border border-border/50 bg-card/40 p-3 sm:grid-cols-12 sm:border-0 sm:bg-transparent sm:p-0">
                            <div class="sm:col-span-3">
                              <Input
                                class="h-8 text-xs"
                                placeholder={m.settings_calc_band_code()}
                                bind:value={band.code}
                                disabled={isSaving}
                              />
                            </div>
                            <div class="sm:col-span-5">
                              <Input
                                class="h-8 text-xs"
                                placeholder={m.settings_calc_band_label()}
                                bind:value={band.label}
                                disabled={isSaving}
                              />
                            </div>
                            <div class="sm:col-span-3">
                              <Input
                                type="number"
                                step="any"
                                class="h-8 text-xs"
                                placeholder={m.settings_calc_band_min()}
                                bind:value={band.minimum}
                                disabled={isSaving}
                              />
                            </div>
                            <div class="flex justify-end sm:col-span-1 sm:justify-center">
                              <Button
                                type="button"
                                size="icon-sm"
                                variant="ghost"
                                class="h-8 w-8 text-muted-foreground hover:text-destructive cursor-pointer"
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
                  </div>
                {/if}
              </CardContent>
            </Card>
          {/if}

          <!-- Ranking Rules & Preview Column -->
          <div class="flex flex-col gap-6">
            <!-- Ranking Policy Card -->
            <Card class="overflow-hidden border-border/70 shadow-xs">
              <CardContent class="flex flex-col gap-5 p-6">
                <div class="flex items-center justify-between {form.rankingEnabled ? 'border-b border-border/40 pb-4' : ''}">
                  <span class="text-xs font-semibold text-foreground">
                    {m.settings_academic_policy_ranking_enabled()}
                  </span>
                  <Switch
                    id="ranking-enabled"
                    checked={form.rankingEnabled}
                    onCheckedChange={(checked) => (form.rankingEnabled = checked)}
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
                          if (val)
                            form.rankingScope = val as
                              | "class"
                              | "grade_level"
                              | "org_unit"
                              | "cohort";
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
                          if (val)
                            form.tiePolicy = val as
                              | "standard_competition"
                              | "dense"
                              | "fractional";
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

                    <div class="flex flex-col gap-1.5 sm:col-span-2">
                      <Label for="calc-max-rank" class="text-xs font-medium text-foreground">
                        {m.settings_academic_policy_max_rank()}
                      </Label>
                      <Input
                        id="calc-max-rank"
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

            <!-- Preview Trigger & Results -->
            {#if !customPlanReadOnly}
              <div class="flex flex-col gap-4">
                <div class="flex items-center justify-between rounded-xl border border-border/70 bg-card/60 p-4">
                  <span class="text-xs font-medium text-muted-foreground">
                    {m.settings_calc_preview()}
                  </span>
                  <Button
                    type="button"
                    variant="outline"
                    size="sm"
                    class="h-8 gap-1.5 text-xs cursor-pointer"
                    onclick={handlePreview}
                    disabled={isSaving || isPreviewing}
                  >
                    <Refresh size={13} class={isPreviewing ? "animate-spin" : ""} />
                    {m.settings_calc_preview()}
                  </Button>
                </div>

                {#if previewResult}
                  <Card class="overflow-hidden border-border/70 shadow-xs">
                    <CardContent class="flex flex-col gap-3 p-5">
                      <div class="flex items-center justify-between border-b border-border/40 pb-2">
                        <span class="text-xs font-semibold text-foreground">
                          {m.settings_calc_preview_result()}
                        </span>
                        <span class="font-mono text-[11px] text-muted-foreground">
                          v{previewResult.result.policy_version}
                        </span>
                      </div>

                      <div class="flex flex-wrap gap-3 pt-1">
                        {#each previewResult.result.outputs as output (output.name)}
                          <div class="flex items-center gap-2 rounded-lg border border-border/60 bg-muted/40 px-3 py-1.5">
                            <span class="text-[11px] text-muted-foreground">{output.name}:</span>
                            <span class="font-mono text-xs font-semibold text-foreground">{output.value}</span>
                          </div>
                        {/each}

                        {#each previewResult.outcomes as result (result.output_name)}
                          <div class="flex items-center gap-2 rounded-lg border border-border/60 bg-muted/40 px-3 py-1.5">
                            <span class="text-[11px] text-muted-foreground">{result.output_name}:</span>
                            <span class="text-xs font-semibold text-foreground">
                              {result.outcome.grade_code ?? (result.outcome.is_passed == null
                                ? "—"
                                : result.outcome.is_passed
                                  ? "Pass"
                                  : "Fail")}
                            </span>
                          </div>
                        {/each}
                      </div>
                    </CardContent>
                  </Card>
                {/if}
              </div>
            {/if}
          </div>
        </div>
      {/if}
    </div>
  </main>

  <SettingsBottomBar
    visible={isDirty}
    {isDirty}
    {isSaving}
    onsave={handleSave}
    onreset={resetForm}
  />
</div>
