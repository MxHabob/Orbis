<script lang="ts">
import { onMount } from "svelte";
import { goto } from "$app/navigation";
import { getLocale } from "$lib/paraglide/runtime";
import { api, type AcademicYearDto, type AcademicPeriodDto } from "$lib/api";
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
let isSubmittingYear = $state(false);
let isSubmittingPeriod = $state(false);
let errorMessage = $state<string | null>(null);
let successMessage = $state<string | null>(null);

// Existing Data
let years = $state<AcademicYearDto[]>([]);
let periods = $state<AcademicPeriodDto[]>([]);

// Academic Year Form
let yearName = $state("2026/2027");
let yearNameAr = $state("العام الدراسي 1447-1448هـ / 2026-2027م");
let yearStartDate = $state("2026-09-01");
let yearEndDate = $state("2027-06-30");

// Selected Year for Period Creation
let selectedYearId = $state<string>("");

// Custom Period Form
let periodName = $state("Semester 1");
let periodNameAr = $state("الفصل الدراسي الأول");
let periodType = $state("semester");
let periodStartDate = $state("2026-09-01");
let periodEndDate = $state("2027-01-20");

const PERIOD_TYPES = [
  { value: "semester", label: "Semester (فصل دراسي - نظام فصلي)" },
  { value: "trimester", label: "Trimester (ثلث دراسي - نظام 3 فصول)" },
  { value: "quarter", label: "Quarter (ربع دراسي - نظام 4 فترات)" },
  { value: "term", label: "General Term (فترة دراسية)" },
];

const stepTasks = $derived(getTasksForStep("calendar", systemStore.onboardingTasks));
const isTaskComplete = $derived(stepTasks.length > 0 && stepTasks.every((t) => t.status === "completed"));

onMount(async () => {
  await loadData();
});

async function loadData() {
  try {
    isLoading = true;
    const [yearsRes, periodsRes] = await Promise.all([
      api.listAcademicYears().catch(() => []),
      api.listAcademicPeriods().catch(() => []),
    ]);
    years = yearsRes;
    periods = periodsRes;
    if (years.length > 0 && !selectedYearId) {
      selectedYearId = String(years[0].id);
    }
  } catch (err: any) {
    console.error("Failed to load calendar data:", err);
    errorMessage = err?.message || "Failed to load academic calendar";
  } finally {
    isLoading = false;
  }
}

async function handleCreateYear() {
  if (!yearName.trim() && !yearNameAr.trim()) {
    errorMessage = isArabic ? "يرجى كتابة اسم العام الدراسي" : "Academic year name is required";
    return;
  }
  if (!yearStartDate || !yearEndDate) {
    errorMessage = isArabic ? "يرجى تحديد تاريخي البداية والنهاية" : "Start and end dates are required";
    return;
  }

  isSubmittingYear = true;
  errorMessage = null;
  successMessage = null;

  try {
    const newYear = await api.createAcademicYear({
      name: yearName.trim() || yearNameAr.trim(),
      name_ar: yearNameAr.trim() || undefined,
      start_date: new Date(yearStartDate).toISOString(),
      end_date: new Date(yearEndDate).toISOString(),
    });

    years = [...years, newYear];
    selectedYearId = String(newYear.id);
    await systemStore.loadOnboardingState(orgId);
    successMessage = isArabic ? "تم إنشاء العام الدراسي بنجاح" : "Academic year created successfully";
  } catch (err: any) {
    console.error("Failed to create academic year:", err);
    errorMessage = err?.message || "Failed to create academic year";
  } finally {
    isSubmittingYear = false;
  }
}

async function handleGenerateSemesters(scheme: "2_semesters" | "3_trimesters" | "4_quarters") {
  if (!selectedYearId) {
    errorMessage = isArabic ? "يرجى إنشاء وتحديد عام دراسي أولاً" : "Please select an academic year first";
    return;
  }

  isSubmittingPeriod = true;
  errorMessage = null;
  successMessage = null;

  try {
    const currentYear = years.find((y) => String(y.id) === String(selectedYearId));
    const startYear = currentYear ? new Date(currentYear.start_date).getFullYear() : 2026;
    const endYear = currentYear ? new Date(currentYear.end_date).getFullYear() : 2027;

    const templates = scheme === "2_semesters" ? [
      { name: "First Semester", nameAr: "الفصل الدراسي الأول", type: "semester", start: `${startYear}-09-01`, end: `${startYear + 1}-01-20` },
      { name: "Second Semester", nameAr: "الفصل الدراسي الثاني", type: "semester", start: `${startYear + 1}-02-01`, end: `${endYear}-06-25` },
    ] : scheme === "3_trimesters" ? [
      { name: "First Trimester", nameAr: "الفصل الدراسي الأول", type: "trimester", start: `${startYear}-09-01`, end: `${startYear}-11-30` },
      { name: "Second Trimester", nameAr: "الفصل الدراسي الثاني", type: "trimester", start: `${startYear}-12-05`, end: `${startYear + 1}-03-05` },
      { name: "Third Trimester", nameAr: "الفصل الدراسي الثالث", type: "trimester", start: `${startYear + 1}-03-15`, end: `${endYear}-06-25` },
    ] : [
      { name: "Quarter 1", nameAr: "الربع الأول", type: "quarter", start: `${startYear}-09-01`, end: `${startYear}-11-10` },
      { name: "Quarter 2", nameAr: "الربع الثاني", type: "quarter", start: `${startYear}-11-15`, end: `${startYear + 1}-01-20` },
      { name: "Quarter 3", nameAr: "الربع الثالث", type: "quarter", start: `${startYear + 1}-02-01`, end: `${startYear + 1}-04-10` },
      { name: "Quarter 4", nameAr: "الربع الرابع", type: "quarter", start: `${startYear + 1}-04-15`, end: `${endYear}-06-25` },
    ];

    for (const t of templates) {
      const created = await api.createAcademicPeriod({
        year_id: selectedYearId,
        name: t.name,
        name_ar: t.nameAr,
        period_type: t.type,
        start_date: new Date(t.start).toISOString(),
        end_date: new Date(t.end).toISOString(),
      });
      periods = [...periods, created];
    }

    await systemStore.loadOnboardingState(orgId);
    successMessage = isArabic ? "تم توليد الفصول الدراسية بنجاح" : "Academic periods generated successfully";
  } catch (err: any) {
    console.error("Failed to generate periods:", err);
    errorMessage = err?.message || "Failed to generate periods";
  } finally {
    isSubmittingPeriod = false;
  }
}

async function handleCreatePeriod() {
  if (!selectedYearId) {
    errorMessage = isArabic ? "يرجى اختيار العام الدراسي" : "Select an academic year first";
    return;
  }
  if (!periodName.trim() && !periodNameAr.trim()) {
    errorMessage = isArabic ? "يرجى كتابة اسم الفترة الدراسية" : "Period name is required";
    return;
  }

  isSubmittingPeriod = true;
  errorMessage = null;
  successMessage = null;

  try {
    const newPeriod = await api.createAcademicPeriod({
      year_id: selectedYearId,
      name: periodName.trim() || periodNameAr.trim(),
      name_ar: periodNameAr.trim() || undefined,
      period_type: periodType,
      start_date: new Date(periodStartDate).toISOString(),
      end_date: new Date(periodEndDate).toISOString(),
    });

    periods = [...periods, newPeriod];
    periodName = "";
    periodNameAr = "";
    await systemStore.loadOnboardingState(orgId);
    successMessage = isArabic ? "تمت إضافة الفترة الدراسية بنجاح" : "Period created successfully";
  } catch (err: any) {
    console.error("Failed to create period:", err);
    errorMessage = err?.message || "Failed to create period";
  } finally {
    isSubmittingPeriod = false;
  }
}

async function handleContinue() {
  if (years.length === 0 && yearName.trim()) {
    await handleCreateYear();
  }
  await goto(`/${orgId}/onboarding/academics`);
}
</script>

<div class="space-y-6">
  <OnboardingStepHeader
    stepNumber={3}
    title="Academic Calendar & Evaluation Periods"
    titleAr="التقويم الدراسي والفصول والتقييمات"
    description="Configure the primary academic operational year, semesters/terms, and grading milestones."
    descriptionAr="حدد العام الدراسي التشغيلي والفصول الدراسية وفترات التقييم ورصد الدرجات."
    isRequired={true}
    status={isTaskComplete ? "completed" : years.length > 0 ? "in_progress" : "not_started"}
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
      <span>{isArabic ? "جاري تحميل بيانات التقويم..." : "Loading academic calendar..."}</span>
    </div>
  {:else}
    <!-- Section 1: Academic Year -->
    <OnboardingSection
      title="Active Academic Year"
      titleAr="العام الدراسي النشط"
      description="Create the primary operational academic year."
      descriptionAr="أنشئ العام الدراسي النشط الذي سترتبط به جميع الفصول والمواد وسجلات الطلاب."
      badge={years.length > 0 ? `${years.length} ${isArabic ? "عام دراسي" : "Academic Year(s)"}` : undefined}
      isComplete={years.length > 0}
    >
      <div class="space-y-4">
        <!-- Year Form -->
        <div class="rounded-lg border border-border/40 bg-muted/20 p-4 space-y-4">
          <div class="grid gap-3 sm:grid-cols-2">
            <div class="space-y-1.5">
              <Label class="text-xs font-medium">
                {isArabic ? "اسم العام (بالإنجليزية / اللاتينية) *" : "Academic Year Name (English) *"}
              </Label>
              <Input
                bind:value={yearName}
                placeholder="2026/2027"
                dir="ltr"
              />
            </div>
            <div class="space-y-1.5">
              <Label class="text-xs font-medium">
                {isArabic ? "اسم العام بالعربية" : "Academic Year Name (Arabic)"}
              </Label>
              <Input
                bind:value={yearNameAr}
                placeholder="1447-1448هـ / 2026-2027م"
                dir="rtl"
              />
            </div>
            <div class="space-y-1.5">
              <Label class="text-xs font-medium">
                {isArabic ? "تاريخ بدء العام الدراسي *" : "Start Date *"}
              </Label>
              <Input
                type="date"
                bind:value={yearStartDate}
              />
            </div>
            <div class="space-y-1.5">
              <Label class="text-xs font-medium">
                {isArabic ? "تاريخ نهاية العام الدراسي *" : "End Date *"}
              </Label>
              <Input
                type="date"
                bind:value={yearEndDate}
              />
            </div>
          </div>

          <div class="flex justify-end">
            <Button
              variant="default"
              size="sm"
              disabled={isSubmittingYear || (!yearName.trim() && !yearNameAr.trim())}
              onclick={handleCreateYear}
              class="text-xs font-semibold"
            >
              {isSubmittingYear ? (isArabic ? "جاري الإنشاء..." : "Creating...") : (isArabic ? "+ إنشاء العام الدراسي" : "+ Create Academic Year")}
            </Button>
          </div>
        </div>

        <!-- Configured Years List -->
        {#if years.length > 0}
          <div class="space-y-2">
            <span class="text-xs font-semibold text-muted-foreground uppercase">
              {isArabic ? "السنوات الدراسية المسجلة" : "Configured Academic Years"}
            </span>
            <div class="divide-y divide-border/40 rounded-lg border border-border/50 bg-card overflow-hidden">
              {#each years as yr (yr.id)}
                <div class="flex items-center justify-between p-3 text-xs">
                  <div class="flex items-center gap-3">
                    <span class="flex size-7 items-center justify-center rounded-md bg-primary/10 font-bold text-primary text-[11px]">
                      📅
                    </span>
                    <div>
                      <div class="font-medium text-foreground">
                        {isArabic && yr.name_ar ? yr.name_ar : yr.name}
                        {#if !isArabic && yr.name_ar}
                          <span class="text-muted-foreground text-[11px] font-sans ml-1">({yr.name_ar})</span>
                        {/if}
                      </div>
                      <div class="text-[11px] font-mono text-muted-foreground">
                        {yr.start_date.split('T')[0]} → {yr.end_date.split('T')[0]}
                      </div>
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

    <!-- Section 2: Academic Terms / Periods -->
    <OnboardingSection
      title="Terms & Evaluation Periods"
      titleAr="الفصول والفترات الدراسية"
      description="Divide the academic year into semesters, trimesters, or quarters."
      descriptionAr="قسّم العام الدراسي إلى فصول أو أثلاث دراسية لرصد الدرجات والتقارير الدورية."
      badge={periods.length > 0 ? `${periods.length} ${isArabic ? "فترة مسجلة" : "Periods"}` : undefined}
      isComplete={periods.length > 0}
    >
      <div class="space-y-4">
        <!-- Quick Preset Generator Buttons -->
        <div class="rounded-lg border border-primary/20 bg-primary/5 p-4 space-y-3">
          <div class="text-xs font-semibold text-foreground">
            ⚡ {isArabic ? "التوليد السريع لنظام الفصول" : "Quick Preset Schemes"}
          </div>
          <p class="text-xs text-muted-foreground">
            {isArabic ? "اختر نظام الفصول المعتمد لتوليد فترات العام بنقرة واحدة:" : "Choose your institution's system to generate periods automatically:"}
          </p>
          <div class="flex flex-wrap gap-2">
            <Button
              variant="outline"
              size="sm"
              disabled={isSubmittingPeriod || years.length === 0}
              onclick={() => handleGenerateSemesters("2_semesters")}
              class="text-xs bg-background hover:bg-muted"
            >
              {isArabic ? "نظام الفصلين (Semesters 2)" : "2 Semesters (فصلان)"}
            </Button>
            <Button
              variant="outline"
              size="sm"
              disabled={isSubmittingPeriod || years.length === 0}
              onclick={() => handleGenerateSemesters("3_trimesters")}
              class="text-xs bg-background hover:bg-muted"
            >
              {isArabic ? "نظام 3 فصول (Trimesters 3)" : "3 Trimesters (3 فصول)"}
            </Button>
            <Button
              variant="outline"
              size="sm"
              disabled={isSubmittingPeriod || years.length === 0}
              onclick={() => handleGenerateSemesters("4_quarters")}
              class="text-xs bg-background hover:bg-muted"
            >
              {isArabic ? "نظام 4 فترات (Quarters 4)" : "4 Quarters (4 فترات)"}
            </Button>
          </div>
        </div>

        <!-- Existing Periods List -->
        {#if periods.length > 0}
          <div class="space-y-2">
            <span class="text-xs font-semibold text-muted-foreground uppercase">
              {isArabic ? "الفترات والفصول المقيدة" : "Configured Periods"}
            </span>
            <div class="grid gap-2 sm:grid-cols-2">
              {#each periods as p (p.id)}
                <div class="flex items-center justify-between rounded-lg border border-border/50 bg-card p-3 text-xs">
                  <div class="flex items-center gap-2">
                    <span class="flex size-6 items-center justify-center rounded bg-primary/10 text-primary text-[10px] font-bold">
                      📑
                    </span>
                    <div>
                      <div class="font-medium text-foreground">
                        {isArabic && p.name_ar ? p.name_ar : p.name}
                      </div>
                      <div class="text-[10px] text-muted-foreground">
                        {p.start_date.split('T')[0]} → {p.end_date.split('T')[0]}
                      </div>
                    </div>
                  </div>
                  <Badge variant="secondary" class="text-[10px] uppercase">
                    {p.period_type}
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
        onclick={() => goto(`/${orgId}/onboarding/structure`)}
        class="text-xs"
      >
        {isArabic ? "← الخطوة السابقة (الهيكل)" : "← Back to Structure"}
      </Button>

      <div class="flex items-center gap-3">
        <Button
          variant="default"
          disabled={years.length === 0 && !yearName.trim()}
          onclick={handleContinue}
          class="text-xs font-semibold shadow-xs"
        >
          {isArabic ? "متابعة إلى البنية الأكاديمية →" : "Continue to Academic Structure →"}
        </Button>
      </div>
    </div>
  {/if}
</div>
