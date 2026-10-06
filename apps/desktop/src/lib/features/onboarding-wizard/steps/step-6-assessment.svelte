<script lang="ts">
import { onMount } from "svelte";
import { goto } from "$app/navigation";
import { getLocale } from "$lib/paraglide/runtime";
import { api, type AssessmentSchemeDto } from "$lib/api";
import { systemStore } from "$lib/stores";
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
import { getTasksForStep } from "../onboarding-steps";

let { orgId }: { orgId: string } = $props();

const isArabic = $derived(getLocale().startsWith("ar"));

let isLoading = $state(true);
let isSubmitting = $state(false);
let errorMessage = $state<string | null>(null);
let successMessage = $state<string | null>(null);

// Existing Data
let schemes = $state<AssessmentSchemeDto[]>([]);

// Scheme Form
let schemeName = $state("Standard 100-Point Grading Scheme");
let schemeNameAr = $state("نظام التقييم والدرجات المئوي المعتمد");
let scaleType = $state("percentage");
let maxScore = $state<number>(100);
let passingScore = $state<number>(50);

const SCALE_TYPES = [
  { value: "percentage", label: "Percentage 0–100% (نسبة مئوية)" },
  { value: "gpa_4", label: "4.0 GPA Scale (معدل تراكمي 4.0)" },
  { value: "gpa_5", label: "5.0 GPA Scale (معدل تراكمي 5.0)" },
  { value: "letter", label: "Letter Grade A–F (حروف تقديرية)" },
];

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

const stepTasks = $derived(getTasksForStep("assessment", systemStore.onboardingTasks));
const isTaskComplete = $derived(stepTasks.length > 0 && stepTasks.every((t) => t.status === "completed"));

onMount(async () => {
  await loadData();
});

async function loadData() {
  try {
    isLoading = true;
    schemes = await api.listAssessmentSchemes().catch(() => []);
  } catch (err: any) {
    console.error("Failed to load assessment schemes:", err);
    errorMessage = err?.message || "Failed to load assessment settings";
  } finally {
    isLoading = false;
  }
}

async function handleCreateScheme() {
  if (!schemeName.trim() && !schemeNameAr.trim()) {
    errorMessage = isArabic ? "يرجى كتابة اسم خطة التقييم" : "Assessment scheme name is required";
    return;
  }

  isSubmitting = true;
  errorMessage = null;
  successMessage = null;

  try {
    const created = await api.createAssessmentScheme({
      name: schemeName.trim() || schemeNameAr.trim(),
      name_ar: schemeNameAr.trim() || undefined,
      scale_type: scaleType,
      passing_score: passingScore,
      max_score: maxScore,
      formula: {
        components: [
          { name: "Coursework", weight: 20, max_score: 20, passing_score: 10 },
          { name: "Midterm", weight: 30, max_score: 30, passing_score: 15 },
          { name: "Final Exam", weight: 40, max_score: 40, passing_score: 20 },
          { name: "Participation", weight: 10, max_score: 10, passing_score: 5 },
        ],
      },
      is_active: true,
    });

    schemes = [...schemes, created];
    schemeName = "";
    schemeNameAr = "";
    await systemStore.loadOnboardingState(orgId);
    successMessage = isArabic ? "تم إنشاء خطة التقييم بنجاح" : "Assessment scheme created successfully";
  } catch (err: any) {
    console.error("Failed to create assessment scheme:", err);
    errorMessage = err?.message || "Failed to create assessment scheme";
  } finally {
    isSubmitting = false;
  }
}

async function handleAddPresetScheme(preset: typeof PRESET_SCHEMES[0]) {
  isSubmitting = true;
  errorMessage = null;
  successMessage = null;

  try {
    const created = await api.createAssessmentScheme({
      name: preset.name,
      name_ar: preset.nameAr,
      scale_type: preset.scale,
      passing_score: preset.pass,
      max_score: preset.max,
      formula: {
        components: preset.components.map((c) => ({
          name: c.name,
          name_ar: c.nameAr,
          weight: c.weight,
          max_score: c.weight,
          passing_score: c.weight / 2,
        })),
      },
      is_active: true,
    });

    schemes = [...schemes, created];
    await systemStore.loadOnboardingState(orgId);
    successMessage = isArabic ? `تم اعتماد ${preset.nameAr}` : `Applied ${preset.name}`;
  } catch (err: any) {
    console.error("Failed to add preset scheme:", err);
    errorMessage = err?.message || "Failed to add assessment scheme";
  } finally {
    isSubmitting = false;
  }
}

async function handleContinue() {
  if (schemes.length === 0 && schemeName.trim()) {
    await handleCreateScheme();
  }
  await goto(`/${orgId}/onboarding/operations`);
}
</script>

<div class="space-y-6">
  <OnboardingStepHeader
    stepNumber={6}
    title="Assessment & Grading Schemes"
    titleAr="خطط التقييم والدرجات والسياسات الأكاديمية"
    description="Establish your academic grading scale, passing score thresholds, and exam component weights."
    descriptionAr="حدد مقياس الدرجات المعتمد، درجات النجاح، وأوزان الاختبارات وأعمال الفصل."
    isRequired={true}
    status={isTaskComplete ? "completed" : schemes.length > 0 ? "in_progress" : "not_started"}
  />

  {#if errorMessage}
    <div class="rounded-lg border border-destructive/30 bg-destructive/10 p-4 text-sm text-destructive font-medium flex items-center justify-between">
      <span>{errorMessage}</span>
      <Button variant="ghost" size="sm" onclick={() => errorMessage = null}>✕</Button>
    </div>
  {/if}

  {#if successMessage}
    <div class="rounded-lg border border-emerald-500/30 bg-emerald-500/10 p-4 text-sm text-emerald-600 dark:text-emerald-400 font-medium flex items-center justify-between">
      <span>{successMessage}</span>
      <Button variant="ghost" size="sm" onclick={() => successMessage = null}>✕</Button>
    </div>
  {/if}

  {#if isLoading}
    <div class="flex items-center justify-center p-12 text-muted-foreground text-sm gap-2">
      <span class="size-4 animate-spin rounded-full border-2 border-primary border-t-transparent"></span>
      <span>{isArabic ? "جاري تحميل خطط التقييم..." : "Loading assessment schemes..."}</span>
    </div>
  {:else}
    <!-- Section 1: Presets & Custom Scheme -->
    <OnboardingSection
      title="Grading Systems & Evaluation Standards"
      titleAr="أنظمة التقييم والدرجات المعتمدة"
      description="Choose a pre-configured grading scale or create a custom grading scheme."
      descriptionAr="اختر نظام التقييم المناسب لمنشأتك أو أنشئ خطة درجات مخصصة."
      badge={schemes.length > 0 ? `${schemes.length} ${isArabic ? "نظام معتمد" : "Schemes"}` : undefined}
      isComplete={schemes.length > 0}
    >
      <div class="space-y-4">
        <!-- Quick Preset Schemes -->
        {#if schemes.length === 0}
          <div class="rounded-lg border border-primary/20 bg-primary/5 p-4 space-y-3">
            <div class="text-xs font-semibold text-foreground">
              🎯 {isArabic ? "اعتماد نظام درجات قياسي بنقرة واحدة" : "Adopt Standard Grading Schemes"}
            </div>
            <div class="grid gap-2 sm:grid-cols-3">
              {#each PRESET_SCHEMES as preset}
                <div class="rounded-lg border border-border/60 bg-card p-3 flex flex-col justify-between gap-3 text-xs">
                  <div>
                    <div class="font-semibold text-foreground">
                      {isArabic ? preset.nameAr : preset.name}
                    </div>
                    <div class="text-[11px] text-muted-foreground mt-1">
                      {isArabic ? `الدرجة العظمى: ${preset.max} | النجاح: ${preset.pass}` : `Max: ${preset.max} | Pass: ${preset.pass}`}
                    </div>
                  </div>
                  <Button
                    variant="outline"
                    size="sm"
                    disabled={isSubmitting}
                    onclick={() => handleAddPresetScheme(preset)}
                    class="text-xs font-semibold w-full bg-background"
                  >
                    {isArabic ? "اعتماد هذا النظام" : "Adopt Scheme"}
                  </Button>
                </div>
              {/each}
            </div>
          </div>
        {/if}

        <!-- Scheme Form -->
        <div class="rounded-lg border border-border/40 bg-muted/20 p-4 space-y-4">
          <div class="grid gap-3 sm:grid-cols-2">
            <div class="space-y-1.5">
              <Label class="text-xs font-medium">
                {isArabic ? "اسم خطة التقييم (بالإنكليزية) *" : "Scheme Name (English) *"}
              </Label>
              <Input
                bind:value={schemeName}
                placeholder="e.g. Standard 100-Point Grading Scheme"
                dir="ltr"
              />
            </div>
            <div class="space-y-1.5">
              <Label class="text-xs font-medium">
                {isArabic ? "اسم خطة التقييم بالعربية" : "Scheme Name (Arabic)"}
              </Label>
              <Input
                bind:value={schemeNameAr}
                placeholder="مثال: نظام التقييم والدرجات المئوي المعتمد"
                dir="rtl"
              />
            </div>
            <div class="space-y-1.5">
              <Label class="text-xs font-medium">
                {isArabic ? "نوع المقياس" : "Scale Type"}
              </Label>
              <Select type="single" bind:value={scaleType}>
                <SelectTrigger class="w-full">
                  {SCALE_TYPES.find((s) => s.value === scaleType)?.label || scaleType}
                </SelectTrigger>
                <SelectContent>
                  {#each SCALE_TYPES as s}
                    <SelectItem value={s.value}>{s.label}</SelectItem>
                  {/each}
                </SelectContent>
              </Select>
            </div>
            <div class="grid grid-cols-2 gap-2">
              <div class="space-y-1.5">
                <Label class="text-xs font-medium">
                  {isArabic ? "الدرجة العظمى" : "Max Score"}
                </Label>
                <Input
                  type="number"
                  bind:value={maxScore}
                  placeholder="100"
                />
              </div>
              <div class="space-y-1.5">
                <Label class="text-xs font-medium">
                  {isArabic ? "درجة النجاح" : "Passing Score"}
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
              {isSubmitting ? (isArabic ? "جاري الإنشاء..." : "Creating...") : (isArabic ? "+ إنشاء خطة التقييم" : "+ Create Scheme")}
            </Button>
          </div>
        </div>

        <!-- Configured Schemes List -->
        {#if schemes.length > 0}
          <div class="space-y-2">
            <span class="text-xs font-semibold text-muted-foreground uppercase">
              {isArabic ? "خطط التقييم المعتمدة" : "Configured Assessment Schemes"}
            </span>
            <div class="grid gap-2 sm:grid-cols-2">
              {#each schemes as sc (sc.id)}
                <div class="flex items-center justify-between rounded-lg border border-border/50 bg-card p-3 text-xs">
                  <div>
                    <div class="font-medium text-foreground">
                      {isArabic && sc.name_ar ? sc.name_ar : sc.name}
                    </div>
                    <div class="text-[10px] text-muted-foreground">
                      {sc.scale_type || "percentage"} | {isArabic ? `النجاح: ${sc.passing_score}` : `Passing: ${sc.passing_score}`}
                    </div>
                  </div>
                  <Badge variant="outline" class="border-emerald-500/30 text-emerald-600 dark:text-emerald-400 text-[10px]">
                    {isArabic ? "نشط" : "Active"}
                  </Badge>
                </div>
              {/each}
            </div>
          </div>
        {/if}
      </div>
    </OnboardingSection>

    <!-- Bottom Actions Bar -->
    <div class="flex items-center justify-between border-t border-border/50 pt-6">
      <Button
        variant="outline"
        onclick={() => goto(`/${orgId}/onboarding/finance`)}
        class="text-xs"
      >
        {isArabic ? "← الخطوة السابقة (المالية)" : "← Back to Finance"}
      </Button>

      <div class="flex items-center gap-3">
        <Button
          variant="default"
          disabled={schemes.length === 0 && !schemeName.trim()}
          onclick={handleContinue}
          class="text-xs font-semibold shadow-xs"
        >
          {isArabic ? "متابعة إلى العمليات والتسجيل →" : "Continue to Operations →"}
        </Button>
      </div>
    </div>
  {/if}
</div>
