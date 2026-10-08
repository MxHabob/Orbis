<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import { onMount } from "svelte";
import * as m from "$lib/paraglide/messages";
import { toast } from "@orbis/ui";
import { api, type AssessmentSchemeDto } from "$lib/api";
import { systemStore } from "$lib/stores";
import { l } from "$lib/core/i18n";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Badge } from "@orbis/ui/badge";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import OnboardingStepHeader from "../onboarding-step-header.svelte";
import OnboardingSection from "../onboarding-section.svelte";
import OnboardingPresetBar from "../components/onboarding-preset-bar.svelte";
import { getTasksForStep } from "../onboarding-steps";
import { refreshOnboardingState } from "../refresh-onboarding-state";
import { BilingualNameFields } from "@orbis/ui/bilingual-name-fields";
import { suggestLocalTranslation } from "$lib/shared/translation/local-engine";
import DeleteRecordButton from "../components/delete-record-button.svelte";

let { orgId }: { orgId: string } = $props();

let isLoading = $state(true);
let isSubmitting = $state(false);
let successMessage = $state<string | null>(null);

// Existing Data
let schemes = $state<AssessmentSchemeDto[]>([]);

// Scheme Form
let schemeName = $state("");
let schemeNameAr = $state("");
let scaleType = $state("percentage");
let maxScore = $state<number>(100);
let passingScore = $state<number>(50);

const scaleTypes = $derived([
  { value: "percentage", label: m.onboarding_step6_scale_percentage() },
  { value: "gpa_4", label: m.onboarding_step6_scale_gpa4() },
  { value: "gpa_5", label: m.onboarding_step6_scale_gpa5() },
  { value: "letter", label: m.onboarding_step6_scale_letter() },
]);

const PRESET_SCHEMES = [
  {
    name: "Standard Percentage (100% Scale)",
    nameAr: "النظام المئوي القياسي (100 درجة)",
    scale: "percentage",
    max: 100,
    pass: 50,
    components: [
      { name: "Coursework & Homework", nameAr: "الأعمال والواجبات", weight: 20 },
      { name: "Midterm Evaluation", nameAr: "اختبار منتصف الفصل", weight: 30 },
      { name: "Final Examination", nameAr: "الاختبار النهائي", weight: 40 },
      { name: "Attendance & Participation", nameAr: "الحضور والمشاركة", weight: 10 },
    ],
  },
  {
    name: "Higher Education GPA 4.0 Scale",
    nameAr: "نظام المعدل التراكمي الجامعي 4.0",
    scale: "gpa_4",
    max: 4,
    pass: 2,
    components: [
      { name: "Continuous Assessment", nameAr: "التقييم المستمر", weight: 40 },
      { name: "Final Project & Exam", nameAr: "المشروع والاختبار النهائي", weight: 60 },
    ],
  },
  {
    name: "Saudi University GPA 5.0 Scale",
    nameAr: "نظام المعدل التراكمي 5.0 (المعيار السعودي)",
    scale: "gpa_5",
    max: 5,
    pass: 2.75,
    components: [
      { name: "Semester Activities", nameAr: "أعمال الفصل", weight: 50 },
      { name: "Comprehensive Final", nameAr: "الاختبار الشامل", weight: 50 },
    ],
  },
];
type AssessmentComponent = (typeof PRESET_SCHEMES)[number]["components"][number];
let schemeComponents = $state<AssessmentComponent[]>([]);

const stepTasks = $derived(getTasksForStep("assessment", systemStore.onboardingTasks));
const isTaskComplete = $derived(stepTasks.length > 0 && stepTasks.every((t) => t.status === "completed"));

onMount(async () => {
  await loadData();
});

async function loadData() {
  try {
    isLoading = true;
    schemes = await api.listAssessmentSchemes();
  } catch (err: any) {
    console.error("Failed to load assessment schemes:", err);
    toast.error(err?.message || m.onboarding_step6_err_load());
  } finally {
    isLoading = false;
  }
}

async function handleCreateScheme() {
  if (!schemeName.trim() && !schemeNameAr.trim()) {
    toast.error(m.onboarding_step6_scheme_err_name());
    return;
  }

  isSubmitting = true;
  successMessage = null;

  try {
    const created = await api.createAssessmentScheme({
      name: schemeName.trim() || schemeNameAr.trim(),
      name_ar: schemeNameAr.trim() || undefined,
      scale_type: scaleType,
      passing_score: passingScore,
      max_score: maxScore,
      formula: {
        components: schemeComponents.map((component) => ({
          name: component.name,
          name_ar: component.nameAr,
          weight: component.weight,
          max_score: component.weight,
          passing_score: component.weight / 2,
        })),
      },
      is_active: true,
    });

    schemes = [...schemes, created];
    schemeName = "";
    schemeNameAr = "";
    schemeComponents = [];
    await refreshOnboardingState(orgId, m.common_action_failed());
    successMessage = m.onboarding_step6_scheme_created_success();
  } catch (err: any) {
    console.error("Failed to create assessment scheme:", err);
    toast.error(err?.message || m.onboarding_step6_err_create());
  } finally {
    isSubmitting = false;
  }
}

function applySchemePreset(preset: typeof PRESET_SCHEMES[number]) {
  schemeName = preset.name;
  schemeNameAr = preset.nameAr;
  scaleType = preset.scale;
  maxScore = preset.max;
  passingScore = preset.pass;
  schemeComponents = preset.components;
}

async function deleteScheme(scheme: AssessmentSchemeDto) {
  await api.deleteAssessmentScheme(String(scheme.id));
  schemes = schemes.filter((item) => item.id !== scheme.id);
  await refreshOnboardingState(orgId, m.common_action_failed());
}

</script>

<div class="space-y-6">
  <OnboardingStepHeader
    stepNumber={7}
    title={m.onboarding_step6_title()}
    description={m.onboarding_step6_subtitle()}
    isRequired={true}
    status={isTaskComplete ? "completed" : schemes.length > 0 ? "in_progress" : "not_started"}
  />


  {#if successMessage}
    <div class="rounded-lg border border-emerald-500/30 bg-emerald-500/10 p-4 text-sm text-emerald-600 dark:text-emerald-400 font-medium flex items-center justify-between">
      <span>{successMessage}</span>
      <Button variant="ghost" size="sm" onclick={() => (successMessage = null)}>✕</Button>
    </div>
  {/if}

  {#if isLoading}
    <div class="flex items-center justify-center p-12 text-muted-foreground text-sm gap-2">
      <LoadingIndicator size={16} class="size-4 text-primary" />
      <span>{m.onboarding_step6_loading()}</span>
    </div>
  {:else}
    <!-- Section 1: Presets & Custom Scheme -->
    <OnboardingSection
      title={m.onboarding_step6_schemes_title()}
      description={m.onboarding_step6_schemes_desc()}
      badge={schemes.length > 0 ? m.onboarding_step6_schemes_count({ count: schemes.length }) : undefined}
      isComplete={schemes.length > 0}
    >
      <div class="space-y-4">
        <OnboardingPresetBar title={m.onboarding_step6_quick_schemes()}>
          {#each PRESET_SCHEMES as preset (preset.name)}
            <Button
              variant="outline"
              size="sm"
              disabled={isSubmitting}
              onclick={() => applySchemePreset(preset)}
              class="text-xs"
            >
              {l(preset.nameAr, preset.name)} · {m.onboarding_step6_max_pass_desc({ max: preset.max, pass: preset.pass })}
            </Button>
          {/each}
        </OnboardingPresetBar>
        {#if schemeComponents.length > 0}
          <div class="flex flex-wrap gap-2">
            {#each schemeComponents as component (component.name)}
              <span class="rounded-full bg-muted/50 px-2.5 py-1 text-xs">
                {l(component.nameAr, component.name)} · {component.weight}%
              </span>
            {/each}
          </div>
        {/if}

        <!-- Scheme Form -->
        <div class="rounded-lg border border-border/40 bg-muted/20 p-4 space-y-4">
          <div class="grid gap-3 sm:grid-cols-2">
            <div class="sm:col-span-2">
              <BilingualNameFields
                arId="onboarding-scheme-name-ar"
                enId="onboarding-scheme-name-en"
                arLabel={m.onboarding_step6_scheme_name_ar()}
                enLabel={m.onboarding_step6_scheme_name_en()}
                bind:arValue={schemeNameAr}
                bind:enValue={schemeName}
                arPlaceholder={m.onboarding_step6_scheme_name_ar_placeholder()}
                enPlaceholder={m.onboarding_step6_scheme_name_en_placeholder()}
                kind="label"
                requestSuggestion={suggestLocalTranslation}
                replaceUneditedInitialValues
                disabled={isSubmitting}
                loadingMessage={m.bilingual_entry_loading()}
                failureMessage={m.bilingual_entry_failure()}
              />
            </div>
            <div class="space-y-1.5">
              <Label class="text-xs font-medium">
                {m.onboarding_step6_scale_type()}
              </Label>
              <Select type="single" bind:value={scaleType}>
                <SelectTrigger class="w-full">
                  {scaleTypes.find((s) => s.value === scaleType)?.label || scaleType}
                </SelectTrigger>
                <SelectContent>
                  {#each scaleTypes as s (s.value)}
                    <SelectItem value={s.value}>{s.label}</SelectItem>
                  {/each}
                </SelectContent>
              </Select>
            </div>
            <div class="grid grid-cols-2 gap-2">
              <div class="space-y-1.5">
                <Label class="text-xs font-medium">
                  {m.onboarding_step6_max_score()}
                </Label>
                <Input
                  type="number"
                  bind:value={maxScore}
                  placeholder="100"
                />
              </div>
              <div class="space-y-1.5">
                <Label class="text-xs font-medium">
                  {m.onboarding_step6_passing_score()}
                </Label>
                <Input
                  type="number"
                  bind:value={passingScore}
                  placeholder="50"
                />
              </div>
            </div>
          </div>

          <div class="flex justify-end">
            <Button
              variant="default"
              size="sm"
              disabled={isSubmitting || (!schemeName.trim() && !schemeNameAr.trim())}
              onclick={handleCreateScheme}
              class="text-xs font-semibold"
            >
              {isSubmitting ? m.onboarding_step6_creating_btn() : m.onboarding_step6_create_scheme_btn()}
            </Button>
          </div>
        </div>

        <!-- Configured Schemes List -->
        {#if schemes.length > 0}
          <div class="space-y-2">
            <span class="text-xs font-semibold text-muted-foreground uppercase">
              {m.onboarding_step6_configured_schemes()}
            </span>
            <div class="grid gap-2 sm:grid-cols-2">
              {#each schemes as sc (sc.id)}
                <div class="flex items-center justify-between rounded-lg border border-border/50 bg-card p-3 text-xs">
                  <div>
                    <div class="font-medium text-foreground">
                      {sc.name_ar || sc.name}
                    </div>
                    <div class="text-[10px] text-muted-foreground">
                      {sc.scale_type || "percentage"} | {m.onboarding_step6_passing_label({ score: sc.passing_score })}
                    </div>
                  </div>
                  <div class="flex items-center gap-2">
                    <Badge variant="outline" class="border-emerald-500/30 text-emerald-600 dark:text-emerald-400 text-[10px]">
                      {m.onboarding_step6_active_badge()}
                    </Badge>
                    <DeleteRecordButton label={sc.name_ar || sc.name} onDelete={() => deleteScheme(sc)} />
                  </div>
                </div>
              {/each}
            </div>
          </div>
        {/if}
      </div>
    </OnboardingSection>

  {/if}
</div>
