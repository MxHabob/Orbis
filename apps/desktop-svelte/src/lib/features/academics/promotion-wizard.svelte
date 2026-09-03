<!--
@component PromotionWizard
Multi-step wizard for the academic promotion workflow.
Steps: Configuration → Class Mapping → Preview → Execute.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";

import { commands } from "$lib/core/ipc";
import { extractErrorMessage } from "$lib/core/ipc";
import { toast } from "@orbis/ui-svelte";
import { academicContext } from "$lib/stores";
import { onMount } from "svelte";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@orbis/ui-svelte/card";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Label } from "@orbis/ui-svelte/label";
import { Switch } from "@orbis/ui-svelte/switch";
import { Checkbox } from "@orbis/ui-svelte/checkbox";
import { Badge } from "@orbis/ui-svelte/badge";
import { Separator } from "@orbis/ui-svelte/separator";
import {
  Table,
  TableBody,
  TableHead,
  TableHeader,
  TableRow,
  TableCell,
} from "@orbis/ui-svelte/table";
import {
  AlertDialog,
  AlertDialogContent,
  AlertDialogHeader,
  AlertDialogFooter,
  AlertDialogTitle,
  AlertDialogDescription,
  AlertDialogAction,
  AlertDialogCancel,
} from "@orbis/ui-svelte/alert-dialog";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { Progress } from "@orbis/ui-svelte/progress";
import {
  ArrowLeft,
  ArrowRight,
  Play,
  Check,
  Loader2,
  Plus,
  Trash2,
  Users,
  UserCheck,
  UserX,
  GraduationCap,
  Ban,
} from "@lucide/svelte";
import type {
  PromotionConfig,
  PromotionJob,
  PromotionDetail,
  ClassMapping,
} from "@orbis/sdk-core";

let { orgId }: { orgId: string } = $props();

// ── Wizard State ────────────────────────────────────────────────

let currentStep = $state(0);
const steps = [
  { key: "config", label: m.common_word_settings },
  { key: "mapping", label: m.common_promo_step_mapping },
  { key: "preview", label: m.common_action_preview },
  { key: "execute", label: m.common_calendar_execute },
];

const progressValue = $derived(((currentStep + 1) / steps.length) * 100);

// ── Config Form ─────────────────────────────────────────────────────

let config = $state<PromotionConfig>({
  source_period_id: academicContext.effectivePeriodId ?? "",
  target_year_name: "",
  target_year_name_ar: null,
  target_start_date: "",
  target_end_date: "",
  passing_score: 50,
  min_attendance_rate: 75,
  auto_graduate_final_level: true,
  final_level: null,
  exclude_withdrawn: true,
  carry_over_outstanding_fees: false,
  carry_over_guardians: true,
  class_mapping: [],
});

$effect(() => {
  if (academicContext.effectivePeriodId && !config.source_period_id) {
    config.source_period_id = academicContext.effectivePeriodId;
  }
});

let policyLoaded = $state(false);

// Load promotion policy and apply defaults (Policy-Driven Architecture)
onMount(async () => {
  try {
    const policy = await commands.platform_policy.getAutopilotConfig();
    if (policy) {
      // Apply policy values as defaults (user can still override)
      config.passing_score = policy.passing_score ?? 50;
      config.min_attendance_rate = policy.min_attendance_rate ?? 75;
      config.auto_graduate_final_level = policy.auto_graduate_final_level === 1;
      config.exclude_withdrawn = policy.exclude_withdrawn === 1;
      // Default to false for these since they aren't explicitly tracked in AcademicPolicy anymore
      config.carry_over_outstanding_fees = false;
      config.carry_over_guardians = true;
      if (policy.final_level) config.final_level = policy.final_level;
    }
    policyLoaded = true;
  } catch (e) {
    // Policy not found — keep hardcoded defaults
    policyLoaded = true;
  }
});

// ── Preview State ───────────────────────────────────────────────

let isAnalyzing = $state(false);
let previewJob = $state<PromotionJob | null>(null);
let previewDetails = $state<PromotionDetail[]>([]);
let previewStats = $state({
  total: 0,
  promoted: 0,
  retained: 0,
  graduated: 0,
  excluded: 0,
});

// ── Execute State ───────────────────────────────────────────────

let isExecuting = $state(false);
let showExecuteConfirm = $state(false);
let executionResult = $state<PromotionJob | null>(null);

// ── Config step valid ───────────────────────────────────────────

const isConfigValid = $derived(
  config.source_period_id &&
    config.target_year_name &&
    config.target_start_date &&
    config.target_end_date,
);

const isMappingValid = $derived(config.class_mapping.length > 0);

// ── Actions ─────────────────────────────────────────────────────

function addMapping() {
  config.class_mapping = [
    ...config.class_mapping,
    {
      source_level: "",
      target_level: "",
      target_class_names: [""],
      distribution_strategy: "balanced",
    },
  ];
}

function removeMapping(index: number) {
  config.class_mapping = config.class_mapping.filter((_, i) => i !== index);
}

async function runDryRun() {
  isAnalyzing = true;
  try {
    previewJob = await commands.enrollment_workflow.startDryRun(config);
    if (previewJob) {
      previewDetails = await commands.enrollment_workflow.getJob(
        previewJob.id,
      );
      if (previewJob.dry_run_result) {
        previewStats = JSON.parse(previewJob.dry_run_result);
      }
    }
    currentStep = 2; // Move to preview
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e));
  } finally {
    isAnalyzing = false;
  }
}

async function executePromotion() {
  if (!previewJob) return;
  showExecuteConfirm = false;
  isExecuting = true;
  try {
    executionResult = await commands.enrollment_workflow.execute(previewJob.id);
    toast.success(m.common_promo_complete());
    currentStep = 3; // Move to results
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e));
  } finally {
    isExecuting = false;
  }
}

function decisionVariant(d: string) {
  switch (d) {
    case "promoted":
      return "success" as const;
    case "retained":
      return "destructive" as const;
    case "graduated":
      return "brand" as const;
    case "excluded":
      return "secondary" as const;
    default:
      return "outline" as const;
  }
}

function decisionLabel(d: string) {
  switch (d) {
    case "promoted":
      return m.common_promo_promoted();
    case "retained":
      return m.common_promo_retained();
    case "graduated":
      return m.common_form_status_graduated();
    case "excluded":
      return m.common_promo_excluded();
    default:
      return d;
  }
}
</script>

<div class="h-full w-full overflow-y-auto bg-background/50 scrollbar-hide pb-10">
  <div class="w-full px-6 py-6 flex flex-col gap-6">
    <div class="w-full max-w-md mx-auto mb-8 px-4 flex flex-col gap-2 items-center select-none">
      <Progress value={progressValue} class="h-1" />
      <span class="text-[10px] text-muted-foreground uppercase tracking-wider font-semibold">
        {m.common_onboarding_progress_label()} {currentStep + 1} / {steps.length}: {steps[currentStep].label()}
      </span>
    </div>

    <!-- Step: Configuration -->
    {#if currentStep === 0}
      <div class="w-full space-y-6">
        <!-- General Config Card -->
        <Card class="w-full">
          <CardHeader class="bg-muted/50 px-6 py-3 border-b border-border">
            <CardTitle class="text-[11px] font-bold text-muted-foreground tracking-wider uppercase text-start select-none">
              {m.common_word_settings()}
            </CardTitle>
          </CardHeader>
          <CardContent class="p-0">
            <div class="flex flex-col divide-y divide-border">
              <!-- Target Year -->
              <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between gap-4">
                <div class="flex flex-col gap-1 text-start">
                  <Label for="target_year_name" class="text-xs font-semibold text-foreground">{m.common_promo_target_year()}</Label>
                  <span class="text-[11px] text-muted-foreground leading-relaxed">{m.common_promo_desc_target_year()}</span>
                </div>
                <div class="w-full md:w-[480px] shrink-0 flex items-center justify-end">
                  <Input id="target_year_name" bind:value={config.target_year_name} placeholder={m.common_year_name_placeholder()} class="h-9 w-full bg-background" />
                </div>
              </div>

              <!-- Target Year Arabic -->
              <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between gap-4">
                <div class="flex flex-col gap-1 text-start">
                  <Label for="target_year_name_ar" class="text-xs font-semibold text-foreground">{m.common_year_name_ar()}</Label>
                  <span class="text-[11px] text-muted-foreground leading-relaxed">{m.common_promo_desc_target_year_ar()}</span>
                </div>
                <div class="w-full md:w-[480px] shrink-0 flex items-center justify-end">
                  <Input id="target_year_name_ar" bind:value={config.target_year_name_ar} class="h-9 w-full bg-background" />
                </div>
              </div>

              <!-- Start Date -->
              <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between gap-4">
                <div class="flex flex-col gap-1 text-start">
                  <Label for="target_start_date" class="text-xs font-semibold text-foreground">{m.common_promo_target_start()}</Label>
                  <span class="text-[11px] text-muted-foreground leading-relaxed">{m.common_promo_desc_target_start()}</span>
                </div>
                <div class="w-full md:w-[480px] shrink-0 flex items-center justify-end">
                  <Input id="target_start_date" type="date" bind:value={config.target_start_date} class="h-9 w-full bg-background" />
                </div>
              </div>

              <!-- End Date -->
              <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between gap-4">
                <div class="flex flex-col gap-1 text-start">
                  <Label for="target_end_date" class="text-xs font-semibold text-foreground">{m.common_promo_target_end()}</Label>
                  <span class="text-[11px] text-muted-foreground leading-relaxed">{m.common_promo_desc_target_end()}</span>
                </div>
                <div class="w-full md:w-[480px] shrink-0 flex items-center justify-end">
                  <Input id="target_end_date" type="date" bind:value={config.target_end_date} class="h-9 w-full bg-background" />
                </div>
              </div>

              <!-- Passing Score -->
              <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between gap-4">
                <div class="flex flex-col gap-1 text-start">
                  <Label for="passing_score" class="text-xs font-semibold text-foreground">{m.common_promo_passing_score()}</Label>
                  <span class="text-[11px] text-muted-foreground leading-relaxed">{m.common_promo_desc_passing_score()}</span>
                </div>
                <div class="w-full md:w-[480px] shrink-0 flex items-center justify-end">
                  <Input id="passing_score" type="number" bind:value={config.passing_score} min={0} max={100} class="h-9 w-full bg-background" />
                </div>
              </div>

              <!-- Min Attendance Rate -->
              <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between gap-4">
                <div class="flex flex-col gap-1 text-start">
                  <Label for="min_attendance_rate" class="text-xs font-semibold text-foreground">{m.common_promo_min_attendance()}</Label>
                  <span class="text-[11px] text-muted-foreground leading-relaxed">{m.common_promo_desc_min_attendance()}</span>
                </div>
                <div class="w-full md:w-[480px] shrink-0 flex items-center justify-end">
                  <Input id="min_attendance_rate" type="number" bind:value={config.min_attendance_rate} min={0} max={100} class="h-9 w-full bg-background" />
                </div>
              </div>
            </div>
          </CardContent>
        </Card>

        <!-- Policies / Switches Card -->
        <Card class="w-full">
          <CardHeader class="bg-muted/50 px-6 py-3 border-b border-border">
            <CardTitle class="text-[11px] font-bold text-muted-foreground tracking-wider uppercase text-start select-none font-serif">
              {m.common_promo_desc_policies_title()}
            </CardTitle>
          </CardHeader>

          <CardContent class="p-0">
            <div class="flex flex-col divide-y divide-border">
              <!-- Auto Graduate Final Level -->
              <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between gap-4">
                <div class="flex flex-col gap-1 text-start">
                  <Label for="auto_graduate" class="text-xs font-semibold text-foreground">
                    {m.common_promo_auto_graduate()}
                  </Label>
                  <span class="text-[11px] text-muted-foreground leading-relaxed">{m.common_promo_desc_auto_graduate()}</span>
                </div>
                <div class="shrink-0">
                  <Switch id="auto_graduate" bind:checked={config.auto_graduate_final_level} />
                </div>
              </div>

              {#if config.auto_graduate_final_level}
                <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between gap-4 bg-muted/5">
                  <div class="flex flex-col gap-1 text-start">
                    <Label for="final_level" class="text-xs font-semibold text-foreground">
                      {m.common_promo_final_level()}
                    </Label>
                    <span class="text-[11px] text-muted-foreground leading-relaxed">{m.common_promo_desc_final_level()}</span>
                  </div>
                  <div class="w-full md:w-[480px] shrink-0 flex items-center justify-end">
                    <Input id="final_level" bind:value={config.final_level} placeholder={m.common_promo_final_level_placeholder()} class="h-9 w-full bg-background" />
                  </div>
                </div>
              {/if}

              <!-- Exclude Withdrawn -->
              <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between gap-4">
                <div class="flex flex-col gap-1 text-start">
                  <Label for="exclude_withdrawn" class="text-xs font-semibold text-foreground">
                    {m.common_promo_exclude_withdrawn()}
                  </Label>
                  <span class="text-[11px] text-muted-foreground leading-relaxed">{m.common_promo_desc_exclude_withdrawn()}</span>
                </div>
                <div class="shrink-0">
                  <Switch id="exclude_withdrawn" bind:checked={config.exclude_withdrawn} />
                </div>
              </div>

              <!-- Carry Fees -->
              <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between gap-4">
                <div class="flex flex-col gap-1 text-start">
                  <Label for="carry_fees" class="text-xs font-semibold text-foreground">
                    {m.common_promo_carry_fees()}
                  </Label>
                  <span class="text-[11px] text-muted-foreground leading-relaxed font-sans">{m.common_promo_desc_carry_fees()}</span>
                </div>
                <div class="shrink-0">
                  <Switch id="carry_fees" bind:checked={config.carry_over_outstanding_fees} />
                </div>
              </div>

              <!-- Carry Guardians -->
              <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between gap-4">
                <div class="flex flex-col gap-1 text-start">
                  <Label for="carry_guardians" class="text-xs font-semibold text-foreground">
                    {m.common_promo_carry_guardians()}
                  </Label>
                  <span class="text-[11px] text-muted-foreground leading-relaxed font-sans">{m.common_promo_desc_carry_guardians()}</span>
                </div>
                <div class="shrink-0">
                  <Switch id="carry_guardians" bind:checked={config.carry_over_guardians} />
                </div>
              </div>
            </div>
          </CardContent>
        </Card>
      </div>

    <!-- Step: Class Mapping -->
    {:else if currentStep === 1}
      <div class="w-full space-y-6">
        <div class="flex justify-end gap-4">
          <Button variant="outline" size="sm" onclick={addMapping} class="border-border">
            <Plus class="size-4 me-2" />
            {m.common_promo_add_mapping()}
          </Button>
        </div>

        {#if config.class_mapping.length === 0}
          <div class="text-center py-12 text-muted-foreground border border-dashed border-border rounded-xl bg-card">
            <p>{m.common_promo_add_mapping()}</p>
          </div>
        {:else}
          <div class="space-y-6">
            {#each config.class_mapping as mapping, i}
              <Card class="w-full">
                <CardHeader class="bg-muted/50 px-6 py-3 border-b border-border flex flex-row items-center justify-between">
                  <CardTitle class="text-[11px] font-bold text-muted-foreground tracking-wider uppercase select-none">
                    {m.common_promo_source_level()} &rarr; {m.common_promo_target_level()}
                  </CardTitle>
                  <Button variant="ghost" size="icon-xs" onclick={() => removeMapping(i)} class="text-destructive hover:text-destructive hover:bg-destructive/5 shrink-0">
                    <Trash2 class="size-3.5" />
                  </Button>
                </CardHeader>
                <CardContent class="p-0">
                  <div class="flex flex-col divide-y divide-border">
                    <!-- Source Level -->
                    <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between gap-4">
                      <div class="flex flex-col gap-1 text-start">
                        <Label class="text-xs font-semibold text-foreground">{m.common_promo_source_level()}</Label>
                        <span class="text-[11px] text-muted-foreground">{m.common_promo_desc_source_level()}</span>
                      </div>
                      <div class="w-full md:w-[480px] shrink-0">
                        <Input bind:value={mapping.source_level} placeholder={m.common_promo_source_level_placeholder()} class="h-9 w-full bg-background" />
                      </div>
                    </div>
                    
                    <!-- Target Level -->
                    <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between gap-4">
                      <div class="flex flex-col gap-1 text-start">
                        <Label class="text-xs font-semibold text-foreground">{m.common_promo_target_level()}</Label>
                        <span class="text-[11px] text-muted-foreground">{m.common_promo_desc_target_level()}</span>
                      </div>
                      <div class="w-full md:w-[480px] shrink-0">
                        <Input bind:value={mapping.target_level} placeholder={m.common_promo_target_level_placeholder()} class="h-9 w-full bg-background" />
                      </div>
                    </div>

                    <!-- Target Classes -->
                    <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between gap-4">
                      <div class="flex flex-col gap-1 text-start">
                        <Label class="text-xs font-semibold text-foreground">{m.common_promo_target_classes()}</Label>
                        <span class="text-[11px] text-muted-foreground">{m.common_promo_desc_target_classes()}</span>
                      </div>
                      <div class="w-full md:w-[480px] shrink-0">
                        <Input
                          value={mapping.target_class_names.join(", ")}
                          oninput={(e) => {
                            mapping.target_class_names = e.currentTarget.value.split(",").map(s => s.trim()).filter(Boolean);
                          }}
                          placeholder={m.common_promo_target_classes_placeholder()}
                          class="h-9 w-full bg-background"
                        />
                      </div>
                    </div>
                  </div>
                </CardContent>
              </Card>
            {/each}
          </div>
        {/if}
      </div>

    <!-- Step: Preview -->
    {:else if currentStep === 2}
      <div class="w-full space-y-6">
        <!-- Summary Cards Grid -->
        <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-5 gap-4">
          <!-- Total -->
          <div class="p-4 rounded-xl border border-border bg-card text-center flex flex-col justify-between h-28 select-none">
            <Users class="size-5 mx-auto text-muted-foreground" />
            <div>
              <div class="text-2xl font-bold text-foreground">{previewStats.total}</div>
              <div class="text-[10px] text-muted-foreground font-semibold uppercase">{m.common_promo_total()}</div>
            </div>
          </div>
          <!-- Promoted -->
          <div class="p-4 rounded-xl border border-border bg-card text-center flex flex-col justify-between h-28 select-none">
            <UserCheck class="size-5 mx-auto text-primary" />
            <div>
              <div class="text-2xl font-bold text-primary">{previewStats.promoted}</div>
              <div class="text-[10px] text-muted-foreground font-semibold uppercase">{m.common_promo_promoted()}</div>
            </div>
          </div>
          <!-- Retained -->
          <div class="p-4 rounded-xl border border-border bg-card text-center flex flex-col justify-between h-28 select-none">
            <UserX class="size-5 mx-auto text-destructive" />
            <div>
              <div class="text-2xl font-bold text-destructive">{previewStats.retained}</div>
              <div class="text-[10px] text-muted-foreground font-semibold uppercase">{m.common_promo_retained()}</div>
            </div>
          </div>
          <!-- Graduated -->
          <div class="p-4 rounded-xl border border-border bg-card text-center flex flex-col justify-between h-28 select-none">
            <GraduationCap class="size-5 mx-auto text-foreground" />
            <div>
              <div class="text-2xl font-bold text-foreground">{previewStats.graduated}</div>
              <div class="text-[10px] text-muted-foreground font-semibold uppercase">{m.common_form_status_graduated()}</div>
            </div>
          </div>
          <!-- Excluded -->
          <div class="p-4 rounded-xl border border-border bg-card text-center flex flex-col justify-between h-28 select-none">
            <Ban class="size-5 mx-auto text-muted-foreground" />
            <div>
              <div class="text-2xl font-bold text-foreground">{previewStats.excluded}</div>
              <div class="text-[10px] text-muted-foreground font-semibold uppercase">{m.common_promo_excluded()}</div>
            </div>
          </div>
        </div>

        <!-- Student Details Table Card -->
        <Card class="w-full">
          <CardHeader class="bg-muted/50 px-6 py-3 border-b border-border">
            <CardTitle class="text-[11px] font-bold text-muted-foreground tracking-wider uppercase text-start select-none">
              {m.common_word_student()} {m.common_promo_decision()}
            </CardTitle>
          </CardHeader>
          <CardContent class="p-0">
            <div class="overflow-x-auto">
              <Table>
                <TableHeader>
                  <TableRow>
                    <TableHead class="text-start">{m.common_word_student()}</TableHead>
                    <TableHead class="text-start">{m.common_promo_decision()}</TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {#each previewDetails.slice(0, 50) as detail}
                    <TableRow>
                      <TableCell class="text-start font-sans">
                        <div>
                          <div class="font-medium text-sm text-foreground">{detail.student_name}</div>
                        </div>
                      </TableCell>
                      <TableCell class="text-start font-sans">
                        <Badge variant={decisionVariant(detail.decision)}>
                          {decisionLabel(detail.decision)}
                        </Badge>
                      </TableCell>
                    </TableRow>
                  {/each}
                  {#if previewDetails.length > 50}
                    <TableRow>
                      <TableCell colspan={6} class="text-center text-sm text-muted-foreground py-4 font-sans">
                        +{previewDetails.length - 50} more students...
                      </TableCell>
                    </TableRow>
                  {/if}
                </TableBody>
              </Table>
            </div>
          </CardContent>
        </Card>
      </div>

    <!-- Step: Execute / Results -->
    {:else if currentStep === 3}
      <div class="w-full space-y-6">
        {#if isExecuting}
          <div class="border border-border rounded-xl bg-card flex flex-col items-center justify-center py-16 text-center select-none">
            <Loader2 class="size-12 text-primary animate-spin mb-4" />
            <h3 class="text-lg font-semibold text-foreground font-serif">{m.common_promo_executing()}</h3>
            <p class="text-sm text-muted-foreground mt-1">{m.common_promo_execute_confirm()}</p>
          </div>
        {:else if executionResult}
          <div class="border border-primary/20 rounded-xl bg-card flex flex-col items-center justify-center py-12 text-center">
            <div class="size-16 rounded-full bg-primary/10 flex items-center justify-center mb-6 select-none">
              <Check class="size-8 text-primary" />
            </div>
            <h3 class="text-xl font-bold text-foreground font-serif select-none">{m.common_promo_complete()}</h3>
            
            <div class="grid grid-cols-2 md:grid-cols-4 gap-6 mt-8 w-full max-w-md px-6 select-none">
              <div class="text-center p-3 rounded-lg border border-border/50">
                <div class="text-2xl font-bold text-primary font-sans">{executionResult.promoted_count ?? 0}</div>
                <div class="text-xs text-muted-foreground mt-1">{m.common_promo_promoted()}</div>
              </div>
              <div class="text-center p-3 rounded-lg border border-border/50">
                <div class="text-2xl font-bold text-destructive font-sans">{executionResult.retained_count ?? 0}</div>
                <div class="text-xs text-muted-foreground mt-1">{m.common_promo_retained()}</div>
              </div>
              <div class="text-center p-3 rounded-lg border border-border/50">
                <div class="text-2xl font-bold text-foreground font-sans">{executionResult.graduated_count ?? 0}</div>
                <div class="text-xs text-muted-foreground mt-1">{m.common_form_status_graduated()}</div>
              </div>
              <div class="text-center p-3 rounded-lg border border-border/50">
                <div class="text-2xl font-bold text-muted-foreground font-sans">{executionResult.excluded_count ?? 0}</div>
                <div class="text-xs text-muted-foreground mt-1">{m.common_promo_excluded()}</div>
              </div>
            </div>
          </div>
        {/if}
      </div>
    {/if}

    <!-- Bottom Actions Control Bar (onboarding-footer style) -->
    {#if currentStep < 3}
      <div class="w-full border-t border-border/60 bg-muted/5 py-4 px-6 shrink-0 mt-8 rounded-xl">
        <div class="w-full flex justify-between items-center">
          <!-- Back button -->
          <Button
            variant="outline"
            onclick={() => { if (currentStep > 0) currentStep -= 1; }}
            disabled={currentStep === 0 || isAnalyzing || isExecuting}
            class="border-border text-muted-foreground hover:text-foreground hover:bg-muted flex gap-2 items-center select-none"
          >
            <ArrowRight class="w-4 h-4 rtl:rotate-180" />
            <span>{m.common_onboarding_prev_btn()}</span>
          </Button>

          <!-- Next / Action Button -->
          {#if currentStep === 0}
            <Button
              variant="lime"
              disabled={!isConfigValid}
              onclick={() => currentStep = 1}
              class="flex gap-2 items-center font-bold select-none"
            >
              <span>{m.common_promo_step_mapping()}</span>
              <ArrowLeft class="w-4 h-4 rtl:rotate-180" />
            </Button>
          {:else if currentStep === 1}
            <Button
              variant="lime"
              disabled={!isMappingValid || isAnalyzing}
              onclick={runDryRun}
              class="flex gap-2 items-center font-bold select-none"
            >
              {#if isAnalyzing}
                <Loader2 class="size-4 animate-spin" />
                <span>{m.common_promo_running()}</span>
              {:else}
                <Play class="size-4" />
                <span>{m.common_promo_dry_run()}</span>
              {/if}
            </Button>
          {:else if currentStep === 2}
            <Button
              variant="lime"
              disabled={isExecuting}
              onclick={() => showExecuteConfirm = true}
              class="flex gap-2 items-center font-bold select-none"
            >
              <span>{m.common_promo_execute()}</span>
              <ArrowLeft class="w-4 h-4 rtl:rotate-180" />
            </Button>
          {/if}
        </div>
      </div>
    {/if}
  </div>
</div>

<!-- Execute Confirmation -->
<AlertDialog bind:open={showExecuteConfirm}>
  <AlertDialogContent>
    <AlertDialogHeader>
      <AlertDialogTitle>{m.common_promo_execute()}</AlertDialogTitle>
      <AlertDialogDescription>{m.common_promo_execute_confirm()}</AlertDialogDescription>
    </AlertDialogHeader>
    <AlertDialogFooter>
      <AlertDialogCancel>{m.common_action_cancel?.() ?? "Cancel"}</AlertDialogCancel>
      <AlertDialogAction onclick={executePromotion}>{m.common_promo_execute()}</AlertDialogAction>
    </AlertDialogFooter>
  </AlertDialogContent>
</AlertDialog>
