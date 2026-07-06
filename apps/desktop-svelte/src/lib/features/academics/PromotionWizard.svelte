<!--
@component PromotionWizard
Multi-step wizard for the academic promotion workflow.
Steps: Configuration → Class Mapping → Preview → Execute.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { commands } from "$lib/core/ipc";
import { extractErrorMessage } from "$lib/core/ipc";
import { toast } from "svelte-sonner";
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
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import {
  Tabs,
  TabsList,
  TabsTrigger,
  TabsContent,
} from "@orbis/ui-svelte/tabs";
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
} from "lucide-svelte";
import type {
  PromotionConfig,
  PromotionJob,
  PromotionDetail,
  ClassMapping,
} from "@orbis/desktop-client/types";

let { orgId }: { orgId: string } = $props();

// ── Wizard State ────────────────────────────────────────────────

let currentStep = $state(0);
const steps = [
  { key: "config", label: m.promo_step_config },
  { key: "mapping", label: m.promo_step_mapping },
  { key: "preview", label: m.promo_step_preview },
  { key: "execute", label: m.promo_step_execute },
];

// ── Config Form ─────────────────────────────────────────────────────

let config = $state<PromotionConfig>({
  source_year_id: academicContext.currentYear?.id ?? "",
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

let policyLoaded = $state(false);

// Load promotion policy and apply defaults (Policy-Driven Architecture)
onMount(async () => {
  try {
    const policy = await commands.promotion.getPolicy(orgId);
    // Apply policy values as defaults (user can still override)
    config.passing_score = policy.passing_score ?? 50;
    config.min_attendance_rate = policy.min_attendance_rate ?? 75;
    config.auto_graduate_final_level =
      (policy.auto_graduate_final_level ?? 1) === 1;
    config.exclude_withdrawn = (policy.exclude_withdrawn ?? 1) === 1;
    config.carry_over_outstanding_fees =
      policy.fee_carry_over_mode !== "ignore";
    config.carry_over_guardians = (policy.carry_over_guardians ?? 1) === 1;
    if (policy.final_level) config.final_level = policy.final_level;
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
  config.source_year_id &&
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
    previewJob = await commands.promotion.dryRun(orgId, config);
    if (previewJob) {
      previewDetails = await commands.promotion.getDetails(previewJob.id);
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
    executionResult = await commands.promotion.execute(previewJob.id);
    toast.success(m.promo_complete());
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
      return m.promo_promoted();
    case "retained":
      return m.promo_retained();
    case "graduated":
      return m.promo_graduated();
    case "excluded":
      return m.promo_excluded();
    default:
      return d;
  }
}
</script>

<div class="h-full w-full overflow-y-auto bg-background/50 scrollbar-hide">
  <div class="max-w-4xl mx-auto p-6 flex flex-col gap-6">

    <!-- Header -->
    <div>
      <h1 class="text-2xl font-bold text-foreground">{m.promo_wizard_title()}</h1>
      <p class="text-sm text-muted-foreground mt-1">{m.promo_subtitle()}</p>
    </div>

    <!-- Step Indicator -->
    <div class="flex items-center gap-1">
      {#each steps as step, i}
        <button
          class="flex-1 py-2 px-3 text-sm font-medium rounded-lg transition-colors
            {i === currentStep
              ? 'bg-primary text-primary-foreground'
              : i < currentStep
                ? 'bg-accent text-accent-foreground'
                : 'bg-muted text-muted-foreground'}"
          onclick={() => { if (i <= currentStep) currentStep = i; }}
          disabled={i > currentStep}
        >
          <div class="flex items-center justify-center gap-2">
            {#if i < currentStep}
              <Check class="size-4" />
            {:else}
              <span class="size-5 flex items-center justify-center rounded-full border text-xs">
                {i + 1}
              </span>
            {/if}
            <span class="hidden sm:inline">{step.label()}</span>
          </div>
        </button>
        {#if i < steps.length - 1}
          <div class="w-6 h-px bg-border"></div>
        {/if}
      {/each}
    </div>

    <Separator />

    <!-- Step: Configuration -->
    {#if currentStep === 0}
      <Card>
        <CardHeader>
          <CardTitle>{m.promo_step_config()}</CardTitle>
        </CardHeader>
        <CardContent>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div class="space-y-2">
              <Label>{m.promo_target_year()}</Label>
              <Input bind:value={config.target_year_name} placeholder={m.year_name_placeholder()} />
            </div>
            <div class="space-y-2">
              <Label>{m.year_name_ar()}</Label>
              <Input bind:value={config.target_year_name_ar} dir="rtl" />
            </div>
            <div class="space-y-2">
              <Label>{m.promo_target_start()}</Label>
              <Input type="date" bind:value={config.target_start_date} />
            </div>
            <div class="space-y-2">
              <Label>{m.promo_target_end()}</Label>
              <Input type="date" bind:value={config.target_end_date} />
            </div>
            <div class="space-y-2">
              <Label>{m.promo_passing_score()}</Label>
              <Input type="number" bind:value={config.passing_score} min={0} max={100} />
            </div>
            <div class="space-y-2">
              <Label>{m.promo_min_attendance()}</Label>
              <Input type="number" bind:value={config.min_attendance_rate} min={0} max={100} />
            </div>
          </div>

          <Separator class="my-6" />

          <div class="space-y-4">
            <div class="flex items-center justify-between">
              <Label class="flex-1">{m.promo_auto_graduate()}</Label>
              <Switch bind:checked={config.auto_graduate_final_level} />
            </div>
            {#if config.auto_graduate_final_level}
              <div class="space-y-2 ps-4">
                <Label>{m.promo_final_level()}</Label>
                <Input bind:value={config.final_level} />
              </div>
            {/if}
            <div class="flex items-center justify-between">
              <Label class="flex-1">{m.promo_exclude_withdrawn()}</Label>
              <Switch bind:checked={config.exclude_withdrawn} />
            </div>
            <div class="flex items-center justify-between">
              <Label class="flex-1">{m.promo_carry_fees()}</Label>
              <Switch bind:checked={config.carry_over_outstanding_fees} />
            </div>
            <div class="flex items-center justify-between">
              <Label class="flex-1">{m.promo_carry_guardians()}</Label>
              <Switch bind:checked={config.carry_over_guardians} />
            </div>
          </div>
        </CardContent>
      </Card>

      <div class="flex justify-end">
        <Button variant="lime" disabled={!isConfigValid} onclick={() => currentStep = 1}>
          {m.promo_step_mapping()}
          <ArrowRight class="size-4 ms-2" />
        </Button>
      </div>

    <!-- Step: Class Mapping -->
    {:else if currentStep === 1}
      <Card>
        <CardHeader>
          <div class="flex items-center justify-between">
            <CardTitle>{m.promo_step_mapping()}</CardTitle>
            <Button variant="outline" size="sm" onclick={addMapping}>
              <Plus class="size-4 me-2" />
              {m.promo_add_mapping()}
            </Button>
          </div>
        </CardHeader>
        <CardContent>
          {#if config.class_mapping.length === 0}
            <div class="text-center py-8 text-muted-foreground">
              <p>{m.promo_add_mapping()}</p>
            </div>
          {:else}
            <div class="space-y-4">
              {#each config.class_mapping as mapping, i}
                <div class="p-4 rounded-lg border border-border/60 bg-card space-y-3">
                  <div class="flex items-center justify-between">
                    <span class="text-sm font-medium">{m.promo_source_level()} → {m.promo_target_level()}</span>
                    <Button variant="ghost" size="icon-xs" onclick={() => removeMapping(i)}>
                      <Trash2 class="size-3.5 text-destructive" />
                    </Button>
                  </div>
                  <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
                    <div class="space-y-1">
                      <Label class="text-xs">{m.promo_source_level()}</Label>
                      <Input bind:value={mapping.source_level} placeholder="e.g. Grade 5" />
                    </div>
                    <div class="space-y-1">
                      <Label class="text-xs">{m.promo_target_level()}</Label>
                      <Input bind:value={mapping.target_level} placeholder="e.g. Grade 6" />
                    </div>
                  </div>
                  <div class="space-y-1">
                    <Label class="text-xs">{m.promo_target_classes()}</Label>
                    <Input
                      value={mapping.target_class_names.join(", ")}
                      oninput={(e) => {
                        mapping.target_class_names = e.currentTarget.value.split(",").map(s => s.trim()).filter(Boolean);
                      }}
                      placeholder="6A, 6B, 6C"
                    />
                  </div>
                </div>
              {/each}
            </div>
          {/if}
        </CardContent>
      </Card>

      <div class="flex items-center justify-between flex-nowrap shrink-0">
        <Button variant="ghost" onclick={() => currentStep = 0}>
          <ArrowLeft class="size-4 me-2" />
          {m.promo_step_config()}
        </Button>
        <Button
          variant="lime"
          disabled={!isMappingValid || isAnalyzing}
          onclick={runDryRun}
        >
          {#if isAnalyzing}
            <Loader2 class="size-4 me-2 animate-spin" />
            {m.promo_running()}
          {:else}
            <Play class="size-4 me-2" />
            {m.promo_dry_run()}
          {/if}
        </Button>
      </div>

    <!-- Step: Preview -->
    {:else if currentStep === 2}
      <!-- Summary Cards -->
      <div class="grid grid-cols-2 md:grid-cols-5 gap-3">
        <Card class="text-center">
          <CardContent class="py-4">
            <Users class="size-5 mx-auto text-muted-foreground mb-1" />
            <div class="text-2xl font-bold text-foreground">{previewStats.total}</div>
            <div class="text-xs text-muted-foreground">{m.promo_total()}</div>
          </CardContent>
        </Card>
        <Card class="text-center">
          <CardContent class="py-4">
            <UserCheck class="size-5 mx-auto text-primary mb-1" />
            <div class="text-2xl font-bold text-primary">{previewStats.promoted}</div>
            <div class="text-xs text-muted-foreground">{m.promo_promoted()}</div>
          </CardContent>
        </Card>
        <Card class="text-center">
          <CardContent class="py-4">
            <UserX class="size-5 mx-auto text-destructive mb-1" />
            <div class="text-2xl font-bold text-destructive">{previewStats.retained}</div>
            <div class="text-xs text-muted-foreground">{m.promo_retained()}</div>
          </CardContent>
        </Card>
        <Card class="text-center">
          <CardContent class="py-4">
            <GraduationCap class="size-5 mx-auto text-muted-foreground mb-1" />
            <div class="text-2xl font-bold text-foreground">{previewStats.graduated}</div>
            <div class="text-xs text-muted-foreground">{m.promo_graduated()}</div>
          </CardContent>
        </Card>
        <Card class="text-center">
          <CardContent class="py-4">
            <Ban class="size-5 mx-auto text-muted-foreground mb-1" />
            <div class="text-2xl font-bold text-foreground">{previewStats.excluded}</div>
            <div class="text-xs text-muted-foreground">{m.promo_excluded()}</div>
          </CardContent>
        </Card>
      </div>

      <!-- Student Details Table -->
      <Card>
        <CardHeader>
          <CardTitle>{m.promo_student()} {m.promo_decision()}</CardTitle>
        </CardHeader>
        <CardContent class="p-0">
          <div class="overflow-x-auto">
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>{m.promo_student()}</TableHead>
                  <TableHead>{m.promo_class()}</TableHead>
                  <TableHead>{m.promo_level()}</TableHead>
                  <TableHead>{m.promo_score()}</TableHead>
                  <TableHead>{m.promo_attendance()}</TableHead>
                  <TableHead>{m.promo_decision()}</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {#each previewDetails.slice(0, 50) as detail}
                  <TableRow>
                    <TableCell>
                      <div>
                        <div class="font-medium text-sm">{detail.student_name}</div>
                        {#if detail.student_number}
                          <div class="text-xs text-muted-foreground">{detail.student_number}</div>
                        {/if}
                      </div>
                    </TableCell>
                    <TableCell class="text-sm">{detail.source_class_name ?? "-"}</TableCell>
                    <TableCell class="text-sm">{detail.source_level}</TableCell>
                    <TableCell class="text-sm">{detail.final_score ?? "-"}</TableCell>
                    <TableCell class="text-sm">
                      {detail.attendance_rate != null ? `${detail.attendance_rate.toFixed(1)}%` : "-"}
                    </TableCell>
                    <TableCell>
                      <Badge variant={decisionVariant(detail.decision)}>
                        {decisionLabel(detail.decision)}
                      </Badge>
                    </TableCell>
                  </TableRow>
                {/each}
                {#if previewDetails.length > 50}
                  <TableRow>
                    <TableCell colspan={6} class="text-center text-sm text-muted-foreground py-3">
                      +{previewDetails.length - 50} more students...
                    </TableCell>
                  </TableRow>
                {/if}
              </TableBody>
            </Table>
          </div>
        </CardContent>
      </Card>

      <div class="flex items-center justify-between flex-nowrap shrink-0">
        <Button variant="ghost" onclick={() => currentStep = 1}>
          <ArrowLeft class="size-4 me-2" />
          {m.promo_step_mapping()}
        </Button>
        <Button
          variant="lime"
          disabled={isExecuting}
          onclick={() => showExecuteConfirm = true}
        >
          {m.promo_execute()}
          <ArrowRight class="size-4 ms-2" />
        </Button>
      </div>

    <!-- Step: Execute / Results -->
    {:else if currentStep === 3}
      {#if isExecuting}
        <Card>
          <CardContent class="flex flex-col items-center justify-center py-16 text-center">
            <Loader2 class="size-12 text-primary animate-spin mb-4" />
            <h3 class="text-lg font-semibold text-foreground">{m.promo_executing()}</h3>
            <p class="text-sm text-muted-foreground mt-1">{m.promo_execute_confirm()}</p>
          </CardContent>
        </Card>
      {:else if executionResult}
        <Card class="border-primary/30">
          <CardContent class="flex flex-col items-center justify-center py-12 text-center">
            <div class="size-16 rounded-full bg-primary/10 flex items-center justify-center mb-4">
              <Check class="size-8 text-primary" />
            </div>
            <h3 class="text-xl font-bold text-foreground">{m.promo_complete()}</h3>
            <div class="grid grid-cols-2 md:grid-cols-4 gap-4 mt-6 w-full max-w-md">
              <div class="text-center">
                <div class="text-2xl font-bold text-primary">{executionResult.promoted_count ?? 0}</div>
                <div class="text-xs text-muted-foreground">{m.promo_promoted()}</div>
              </div>
              <div class="text-center">
                <div class="text-2xl font-bold text-destructive">{executionResult.retained_count ?? 0}</div>
                <div class="text-xs text-muted-foreground">{m.promo_retained()}</div>
              </div>
              <div class="text-center">
                <div class="text-2xl font-bold text-foreground">{executionResult.graduated_count ?? 0}</div>
                <div class="text-xs text-muted-foreground">{m.promo_graduated()}</div>
              </div>
              <div class="text-center">
                <div class="text-2xl font-bold text-muted-foreground">{executionResult.excluded_count ?? 0}</div>
                <div class="text-xs text-muted-foreground">{m.promo_excluded()}</div>
              </div>
            </div>
          </CardContent>
        </Card>
      {/if}
    {/if}
  </div>
</div>

<!-- Execute Confirmation -->
<AlertDialog bind:open={showExecuteConfirm}>
  <AlertDialogContent>
    <AlertDialogHeader>
      <AlertDialogTitle>{m.promo_execute()}</AlertDialogTitle>
      <AlertDialogDescription>{m.promo_execute_confirm()}</AlertDialogDescription>
    </AlertDialogHeader>
    <AlertDialogFooter>
      <AlertDialogCancel>{m.common_cancel?.() ?? "Cancel"}</AlertDialogCancel>
      <AlertDialogAction onclick={executePromotion}>{m.promo_execute()}</AlertDialogAction>
    </AlertDialogFooter>
  </AlertDialogContent>
</AlertDialog>
