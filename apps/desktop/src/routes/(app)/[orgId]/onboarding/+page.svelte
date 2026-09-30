<!--
  Orbis Onboarding & Institutional Readiness Center
  Authoritative DAG orchestration dashboard for post-bootstrap institution configuration.
-->
<script lang="ts">
import { page } from "$app/state";
import { goto } from "$app/navigation";
import { localizeHref, getLocale } from "$lib/paraglide/runtime";
import { systemStore } from "$lib/stores/system.svelte";
import type { SetupTaskDto, OnboardingTaskCategory, OnboardingTaskStatus } from "$lib/api/types";
import {
  AcademicCap,
  ArrowLeft,
  ArrowRight,
  ArrowPath,
  Banknotes,
  BookOpen,
  Calendar,
  Check,
  CheckCircle,
  ClockRotateRight,
  DocumentChartBar,
  DocumentPage,
  Grid,
  InfoCircle,
  Keyframes,
  Lock,
  Plus,
  Search,
  Settings,
  Sparkles,
  UserGroup,
  Users,
  XMark,
} from "@orbis/icons";
import { onMount } from "svelte";

const orgId = $derived(page.params.orgId);
const isArabic = $derived(getLocale().startsWith("ar"));

// Local UI Filter States
let searchQuery = $state("");
let selectedCategory = $state<string>("all");
let selectedStatus = $state<string>("all");
let isProvisionModalOpen = $state(false);
let isProvisioning = $state(false);
let provisioningError = $state<string | null>(null);

// Provision Form Defaults
let provYearName = $state("");
let provYearNameAr = $state("");
let provStagesAndGrades = $state(true);
let provSubjects = $state(true);
let provAssessmentScheme = $state(true);
let provTuitionFee = $state(true);

onMount(async () => {
  await loadState();
});

async function loadState() {
  await systemStore.loadOnboardingState(orgId);
}

const state = $derived(systemStore.onboardingState);
const tasks = $derived(systemStore.onboardingTasks);
const categories = $derived(state?.categories ?? []);

// Category Icon Mapping
function getCategoryIcon(cat: string) {
  switch (cat) {
    case "organization": return Keyframes;
    case "calendar": return Calendar;
    case "academics": return BookOpen;
    case "assessment": return AcademicCap;
    case "finance": return Banknotes;
    case "students": return Users;
    case "templates": return DocumentChartBar;
    case "integrations": return Grid;
    default: return Settings;
  }
}

// Status Color & Label Helpers
function getStatusBadge(status: OnboardingTaskStatus) {
  switch (status) {
    case "completed":
      return {
        label: isArabic ? "مكتملة" : "Completed",
        bg: "bg-emerald-500/10 text-emerald-600 dark:text-emerald-400 border-emerald-500/20",
        dot: "bg-emerald-500",
      };
    case "in_progress":
      return {
        label: isArabic ? "قيد الإعداد" : "In Progress",
        bg: "bg-amber-500/10 text-amber-600 dark:text-amber-400 border-amber-500/20",
        dot: "bg-amber-500",
      };
    case "blocked":
      return {
        label: isArabic ? "بانتظار المتطلبات" : "Blocked",
        bg: "bg-rose-500/10 text-rose-600 dark:text-rose-400 border-rose-500/20",
        dot: "bg-rose-500",
      };
    case "optional":
      return {
        label: isArabic ? "اختيارية" : "Optional",
        bg: "bg-muted text-muted-foreground border-border",
        dot: "bg-muted-foreground",
      };
    default:
      return {
        label: isArabic ? "لم تبدأ" : "Not Started",
        bg: "bg-blue-500/10 text-blue-600 dark:text-blue-400 border-blue-500/20",
        dot: "bg-blue-500",
      };
  }
}

// Filtered Tasks
const filteredTasks = $derived.by(() => {
  return tasks.filter((t) => {
    // Search query filter
    if (searchQuery.trim()) {
      const q = searchQuery.toLowerCase();
      const matchEn = t.title.toLowerCase().includes(q) || t.description.toLowerCase().includes(q);
      const matchAr = t.title_ar.includes(q) || t.description_ar.includes(q);
      if (!matchEn && !matchAr) return false;
    }

    // Category filter
    if (selectedCategory !== "all" && t.category !== selectedCategory) {
      return false;
    }

    // Status filter
    if (selectedStatus !== "all") {
      if (selectedStatus === "required" && !t.is_required) return false;
      if (selectedStatus === "pending" && t.status === "completed") return false;
      if (selectedStatus === "completed" && t.status !== "completed") return false;
      if (selectedStatus === "blocked" && t.status !== "blocked") return false;
    }

    return true;
  });
});

async function handleNavigate(task: SetupTaskDto) {
  if (task.status === "blocked") return;
  const route = task.route.startsWith("/") ? task.route : `/${task.route}`;
  const target = localizeHref(`/${orgId}${route}`);
  await goto(target);
}

async function handleProvisionDefaults() {
  isProvisioning = true;
  provisioningError = null;
  try {
    await systemStore.provisionBlueprintDefaults(
      {
        academic_year_name: provYearName || undefined,
        academic_year_name_ar: provYearNameAr || undefined,
        provision_stages_and_grades: provStagesAndGrades,
        provision_subjects: provSubjects,
        provision_assessment_scheme: provAssessmentScheme,
        provision_tuition_fee_template: provTuitionFee,
      },
      orgId,
    );
    isProvisionModalOpen = false;
  } catch (e: any) {
    provisioningError = e?.message || (isArabic ? "فشل تجهيز الهيكل الأساسي" : "Failed to provision blueprint defaults");
  } finally {
    isProvisioning = false;
  }
}
</script>

<svelte:head>
  <title>{isArabic ? "مركز التهيئة والجاهزية المؤسسية" : "Onboarding & Institution Readiness"} — Orbis</title>
</svelte:head>

<div class="space-y-6 pb-20">
  <!-- Top Banner / Hero -->
  <div class="relative overflow-hidden rounded-2xl border border-border bg-gradient-to-br from-card via-card/90 to-primary/5 p-6 md:p-8 shadow-sm">
    <div class="relative z-10 flex flex-col md:flex-row md:items-center md:justify-between gap-6">
      <div class="space-y-2 max-w-2xl">
        <div class="flex items-center gap-2">
          <span class="inline-flex items-center gap-1.5 px-2.5 py-0.5 rounded-full text-xs font-medium bg-primary/10 text-primary border border-primary/20">
            <Sparkles size={12} />
            {isArabic ? "دليل الجاهزية التشغيلية" : "Operational Readiness Guide"}
          </span>
          <span class="inline-flex items-center px-2 py-0.5 rounded-md text-xs font-mono bg-muted text-muted-foreground uppercase">
            {state?.institution_type ?? "SCHOOL"}
          </span>
        </div>
        <h1 class="text-2xl md:text-3xl font-bold tracking-tight">
          {isArabic ? "تهيئة المؤسسة التعليمية" : "Institution Setup & Readiness"}
        </h1>
        <p class="text-sm text-muted-foreground leading-relaxed">
          {isArabic
            ? "مسار موجه لتهيئة كافة متطلبات المنشأة، التقويم، المواد، الفصول، الرسوم ولائحة التقييم قبل بدء العمليات اليومية."
            : "A guided dependency-aware roadmap to configure organization profiles, academic calendar, curriculum, classes, fee policies, and evaluation schemes."}
        </p>
      </div>

      <!-- Quick Action / Auto Scaffolding -->
      <div class="flex flex-col sm:flex-row items-stretch sm:items-center gap-3">
        <button
          type="button"
          onclick={() => (isProvisionModalOpen = true)}
          class="inline-flex items-center justify-center gap-2 px-4 py-2.5 rounded-xl text-sm font-medium bg-primary text-primary-foreground hover:bg-primary/90 shadow-sm transition-all hover:scale-[1.02] active:scale-[0.98]"
        >
          <Sparkles size={16} />
          {isArabic ? "توليد الهيكل الموصى به" : "Auto-Scaffold Blueprint"}
        </button>

        <button
          type="button"
          onclick={loadState}
          class="inline-flex items-center justify-center gap-2 px-3 py-2.5 rounded-xl text-sm font-medium border border-border bg-background hover:bg-muted/50 text-foreground transition-colors"
          title={isArabic ? "تحديث الحالة" : "Refresh"}
        >
          <ArrowPath size={16} class={systemStore.isLoading ? "animate-spin" : ""} />
        </button>
      </div>
    </div>

    <!-- Progress Metric Bar -->
    <div class="mt-8 pt-6 border-t border-border/60 grid grid-cols-2 sm:grid-cols-4 gap-4">
      <div class="space-y-1">
        <span class="text-xs text-muted-foreground">{isArabic ? "نسبة الإنجاز الكلية" : "Overall Progress"}</span>
        <div class="flex items-baseline gap-2">
          <span class="text-2xl font-bold font-mono">{state?.progress_percentage ?? 0}%</span>
          <span class="text-xs text-muted-foreground">({state?.completed_tasks ?? 0}/{state?.total_tasks ?? 0})</span>
        </div>
        <div class="h-1.5 w-full bg-muted rounded-full overflow-hidden">
          <div
            class="h-full bg-primary transition-all duration-500 rounded-full"
            style="width: {state?.progress_percentage ?? 0}%"
          ></div>
        </div>
      </div>

      <div class="space-y-1">
        <span class="text-xs text-muted-foreground">{isArabic ? "المهام الإلزامية" : "Required Tasks"}</span>
        <div class="flex items-baseline gap-2">
          <span class="text-2xl font-bold font-mono text-emerald-600 dark:text-emerald-400">
            {state?.required_completed ?? 0}/{state?.required_tasks ?? 0}
          </span>
          <span class="text-xs text-muted-foreground">({state?.required_progress_percentage ?? 0}%)</span>
        </div>
        <div class="h-1.5 w-full bg-muted rounded-full overflow-hidden">
          <div
            class="h-full bg-emerald-500 transition-all duration-500 rounded-full"
            style="width: {state?.required_progress_percentage ?? 0}%"
          ></div>
        </div>
      </div>

      <div class="space-y-1">
        <span class="text-xs text-muted-foreground">{isArabic ? "المهام المقفلة" : "Blocked Tasks"}</span>
        <div class="flex items-baseline gap-2">
          <span class="text-2xl font-bold font-mono {state?.blocked_tasks ? 'text-amber-500' : 'text-muted-foreground'}">
            {state?.blocked_tasks ?? 0}
          </span>
          <span class="text-xs text-muted-foreground">{isArabic ? "بانتظار متطلبات" : "needs prereqs"}</span>
        </div>
        <p class="text-[11px] text-muted-foreground">{isArabic ? "تفتح تلقائياً عند إكمال الاعتماد" : "Unlocks upon prerequisites"}</p>
      </div>

      <div class="space-y-1">
        <span class="text-xs text-muted-foreground">{isArabic ? "حالة الجاهزية" : "Readiness Status"}</span>
        <div class="flex items-center gap-2 mt-1">
          {#if state?.is_institution_ready}
            <span class="inline-flex items-center gap-1 text-xs font-semibold text-emerald-600 dark:text-emerald-400 bg-emerald-500/10 px-2.5 py-1 rounded-md border border-emerald-500/20">
              <CheckCircle size={14} />
              {isArabic ? "جاهز للتشغيل" : "Ready for Operations"}
            </span>
          {:else}
            <span class="inline-flex items-center gap-1 text-xs font-semibold text-amber-600 dark:text-amber-400 bg-amber-500/10 px-2.5 py-1 rounded-md border border-amber-500/20">
              <ClockRotateRight size={14} />
              {isArabic ? "قيد التجهيز" : "In Preparation"}
            </span>
          {/if}
        </div>
      </div>
    </div>
  </div>

  <!-- Recommended Next Action Banner (if any) -->
  {#if state?.recommended_next_task}
    {@const next = state.recommended_next_task}
    {@const NextIcon = getCategoryIcon(next.category)}
    <div class="rounded-xl border border-primary/30 bg-primary/5 p-4 flex flex-col sm:flex-row sm:items-center justify-between gap-4">
      <div class="flex items-start sm:items-center gap-3">
        <div class="p-2.5 rounded-lg bg-primary text-primary-foreground">
          <NextIcon size={20} />
        </div>
        <div>
          <span class="text-xs font-medium text-primary uppercase tracking-wider">
            {isArabic ? "الخطوة التالية الموصى بها" : "Recommended Next Action"}
          </span>
          <h3 class="text-base font-semibold text-foreground">
            {isArabic ? next.title_ar : next.title}
          </h3>
          <p class="text-xs text-muted-foreground">
            {isArabic ? next.description_ar : next.description}
          </p>
        </div>
      </div>

      <button
        type="button"
        onclick={() => handleNavigate(next)}
        class="inline-flex items-center justify-center gap-2 px-4 py-2 rounded-lg text-sm font-medium bg-primary text-primary-foreground hover:bg-primary/90 transition-colors shrink-0"
      >
        <span>{isArabic ? "بدء الإعداد الآن" : "Configure Now"}</span>
        {#if isArabic}
          <ArrowLeft size={16} />
        {:else}
          <ArrowRight size={16} />
        {/if}
      </button>
    </div>
  {/if}

  <!-- Category Overview Pills -->
  <div class="grid grid-cols-2 sm:grid-cols-4 lg:grid-cols-8 gap-2.5">
    {#each categories as cat}
      {@const CatIcon = getCategoryIcon(cat.category)}
      <button
        type="button"
        onclick={() => (selectedCategory = selectedCategory === cat.category ? "all" : cat.category)}
        class="flex flex-col items-center text-center p-3 rounded-xl border transition-all {selectedCategory === cat.category ? 'border-primary bg-primary/5 shadow-sm' : 'border-border bg-card hover:bg-muted/40'}"
      >
        <div class="p-2 rounded-lg bg-muted text-foreground mb-1.5">
          <CatIcon size={16} />
        </div>
        <span class="text-xs font-medium text-foreground line-clamp-1">
          {isArabic ? cat.label_ar : cat.label}
        </span>
        <span class="text-[10px] text-muted-foreground font-mono mt-0.5">
          {cat.completed_tasks}/{cat.total_tasks} ({cat.progress_percentage}%)
        </span>
      </button>
    {/each}
  </div>

  <!-- Search & Filter Controls -->
  <div class="flex flex-col sm:flex-row items-stretch sm:items-center justify-between gap-3 pt-2">
    <div class="relative flex-1 max-w-md">
      <Search size={16} class="absolute {isArabic ? 'right-3' : 'left-3'} top-1/2 -translate-y-1/2 text-muted-foreground" />
      <input
        type="text"
        bind:value={searchQuery}
        placeholder={isArabic ? "البحث في مهام التهيئة..." : "Search setup tasks..."}
        class="w-full bg-background border border-border rounded-xl {isArabic ? 'pr-9 pl-3' : 'pl-9 pr-3'} py-2 text-sm focus:outline-none focus:ring-2 focus:ring-primary/20 focus:border-primary transition-all"
      />
      {#if searchQuery}
        <button
          type="button"
          onclick={() => (searchQuery = "")}
          class="absolute {isArabic ? 'left-3' : 'right-3'} top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground"
        >
          <XMark size={14} />
        </button>
      {/if}
    </div>

    <!-- Status Tabs -->
    <div class="flex items-center gap-1.5 p-1 rounded-xl border border-border bg-muted/20 overflow-x-auto text-xs">
      <button
        type="button"
        onclick={() => (selectedStatus = "all")}
        class="px-3 py-1.5 rounded-lg font-medium transition-colors {selectedStatus === 'all' ? 'bg-background text-foreground shadow-sm' : 'text-muted-foreground hover:text-foreground'}"
      >
        {isArabic ? "الكل" : "All"} ({tasks.length})
      </button>
      <button
        type="button"
        onclick={() => (selectedStatus = "required")}
        class="px-3 py-1.5 rounded-lg font-medium transition-colors {selectedStatus === 'required' ? 'bg-background text-foreground shadow-sm' : 'text-muted-foreground hover:text-foreground'}"
      >
        {isArabic ? "الإلزامية" : "Required"} ({tasks.filter((t) => t.is_required).length})
      </button>
      <button
        type="button"
        onclick={() => (selectedStatus = "pending")}
        class="px-3 py-1.5 rounded-lg font-medium transition-colors {selectedStatus === 'pending' ? 'bg-background text-foreground shadow-sm' : 'text-muted-foreground hover:text-foreground'}"
      >
        {isArabic ? "المتبقية" : "Pending"} ({tasks.filter((t) => t.status !== 'completed').length})
      </button>
      <button
        type="button"
        onclick={() => (selectedStatus = "blocked")}
        class="px-3 py-1.5 rounded-lg font-medium transition-colors {selectedStatus === 'blocked' ? 'bg-background text-foreground shadow-sm' : 'text-muted-foreground hover:text-foreground'}"
      >
        {isArabic ? "المقفلة" : "Blocked"} ({tasks.filter((t) => t.status === 'blocked').length})
      </button>
      <button
        type="button"
        onclick={() => (selectedStatus = "completed")}
        class="px-3 py-1.5 rounded-lg font-medium transition-colors {selectedStatus === 'completed' ? 'bg-background text-foreground shadow-sm' : 'text-muted-foreground hover:text-foreground'}"
      >
        {isArabic ? "المكتملة" : "Done"} ({tasks.filter((t) => t.status === 'completed').length})
      </button>
    </div>
  </div>

  <!-- Tasks DAG Grid -->
  <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
    {#each filteredTasks as task (task.id)}
      {@const TaskIcon = getCategoryIcon(task.category)}
      {@const badge = getStatusBadge(task.status)}
      {@const isBlocked = task.status === "blocked"}
      {@const isComplete = task.status === "completed"}

      <div class="group relative rounded-xl border border-border bg-card p-5 shadow-xs transition-all hover:border-primary/40 hover:shadow-sm flex flex-col justify-between {isBlocked ? 'opacity-75 bg-muted/10' : ''}">
        <div class="space-y-3">
          <!-- Card Header -->
          <div class="flex items-start justify-between gap-3">
            <div class="flex items-center gap-3">
              <div class="p-2.5 rounded-xl {isComplete ? 'bg-emerald-500/10 text-emerald-600 dark:text-emerald-400' : isBlocked ? 'bg-muted text-muted-foreground' : 'bg-primary/10 text-primary'}">
                {#if isComplete}
                  <Check size={18} />
                {:else if isBlocked}
                  <Lock size={18} />
                {:else}
                  <TaskIcon size={18} />
                {/if}
              </div>
              <div>
                <div class="flex items-center gap-2">
                  <h3 class="text-base font-semibold text-foreground">
                    {isArabic ? task.title_ar : task.title}
                  </h3>
                  {#if task.is_required}
                    <span class="text-[10px] font-semibold text-rose-600 dark:text-rose-400 bg-rose-500/10 px-1.5 py-0.5 rounded">
                      {isArabic ? "مطلوب" : "Required"}
                    </span>
                  {/if}
                </div>
                <span class="text-xs text-muted-foreground uppercase font-mono tracking-wider">
                  {task.id}
                </span>
              </div>
            </div>

            <!-- Status Badge -->
            <span class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full text-xs font-medium border {badge.bg}">
              <span class="w-1.5 h-1.5 rounded-full {badge.dot}"></span>
              {badge.label}
            </span>
          </div>

          <!-- Description -->
          <p class="text-sm text-muted-foreground leading-normal">
            {isArabic ? task.description_ar : task.description}
          </p>

          <!-- Completion Condition / Verified Rule -->
          <div class="p-2.5 rounded-lg bg-muted/40 border border-border/40 text-xs text-muted-foreground space-y-1">
            <div class="flex items-center gap-1.5 font-medium text-foreground">
              <InfoCircle size={13} />
              <span>{isArabic ? "شرط الاكتمال الفعلي:" : "Backend Completion Condition:"}</span>
            </div>
            <p class="text-[11px] leading-relaxed">
              {task.completion_condition}
            </p>
          </div>

          <!-- Blocking Warning if Blocked -->
          {#if isBlocked && task.blocking_reason}
            <div class="p-2.5 rounded-lg bg-rose-500/10 border border-rose-500/20 text-xs text-rose-700 dark:text-rose-400 flex items-start gap-2">
              <Lock size={14} class="shrink-0 mt-0.5" />
              <div>
                <span class="font-semibold">{isArabic ? "متطلبات غير مكتملة:" : "Missing Prerequisites:"}</span>
                <p class="text-[11px] mt-0.5">{isArabic && task.blocking_reason_ar ? task.blocking_reason_ar : task.blocking_reason}</p>
              </div>
            </div>
          {/if}

          <!-- Dependencies Tags -->
          {#if task.dependencies.length > 0}
            <div class="flex items-center gap-1.5 flex-wrap text-xs pt-1">
              <span class="text-[11px] text-muted-foreground">{isArabic ? "يعتمد على:" : "Depends on:"}</span>
              {#each task.dependencies as dep}
                <span class="px-2 py-0.5 rounded-md bg-muted text-muted-foreground font-mono text-[10px]">
                  {dep}
                </span>
              {/each}
            </div>
          {/if}
        </div>

        <!-- Action Footer -->
        <div class="pt-4 mt-4 border-t border-border flex items-center justify-between gap-3">
          <div class="flex items-center gap-2">
            <div class="w-16 h-2 bg-muted rounded-full overflow-hidden">
              <div
                class="h-full {isComplete ? 'bg-emerald-500' : 'bg-primary'} transition-all duration-300"
                style="width: {task.progress}%"
              ></div>
            </div>
            <span class="text-xs font-mono text-muted-foreground">{task.progress}%</span>
          </div>

          <button
            type="button"
            disabled={isBlocked}
            onclick={() => handleNavigate(task)}
            class="inline-flex items-center justify-center gap-1.5 px-3.5 py-1.5 rounded-lg text-xs font-medium transition-all {isComplete
              ? 'border border-border bg-background hover:bg-muted text-foreground'
              : isBlocked
                ? 'bg-muted text-muted-foreground cursor-not-allowed'
                : 'bg-primary text-primary-foreground hover:bg-primary/90 shadow-xs'}"
          >
            {#if isComplete}
              <span>{isArabic ? "مراجعة الإعدادات" : "Review Settings"}</span>
            {:else if isBlocked}
              <Lock size={12} />
              <span>{isArabic ? "مقفل" : "Locked"}</span>
            {:else}
              <span>{isArabic ? "بدء التكوين" : "Configure"}</span>
              {#if isArabic}
                <ArrowLeft size={12} />
              {:else}
                <ArrowRight size={12} />
              {/if}
            {/if}
          </button>
        </div>
      </div>
    {:else}
      <div class="col-span-full py-12 text-center text-muted-foreground space-y-2">
        <InfoCircle size={32} class="mx-auto text-muted-foreground/40" />
        <p class="text-sm">{isArabic ? "لا توجد مهام تطابق خيارات التصفية المحددة." : "No setup tasks match your search or filter."}</p>
      </div>
    {/each}
  </div>
</div>

<!-- Auto-Scaffolding Blueprint Modal -->
{#if isProvisionModalOpen}
  <div class="fixed inset-0 z-50 bg-black/50 backdrop-blur-xs flex items-center justify-center p-4">
    <div class="relative w-full max-w-lg rounded-2xl border border-border bg-card p-6 shadow-2xl space-y-5 animate-in fade-in zoom-in-95 duration-200">
      <div class="flex items-start justify-between gap-4">
        <div class="flex items-center gap-3">
          <div class="p-2.5 rounded-xl bg-primary/10 text-primary">
            <Sparkles size={20} />
          </div>
          <div>
            <h3 class="text-lg font-bold text-foreground">
              {isArabic ? "توليد الهيكل القياسي للمؤسسة" : "Auto-Scaffold Blueprint Defaults"}
            </h3>
            <p class="text-xs text-muted-foreground">
              {isArabic
                ? "إنشاء العام الدراسي، الفصول، المواد، واللائحة التقييمية بنقرة واحدة."
                : "Generate initial academic year, periods, subjects, classes, and assessment schemes."}
            </p>
          </div>
        </div>

        <button
          type="button"
          onclick={() => (isProvisionModalOpen = false)}
          class="p-1 rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted"
        >
          <XMark size={18} />
        </button>
      </div>

      {#if provisioningError}
        <div class="p-3 rounded-xl bg-rose-500/10 border border-rose-500/20 text-xs text-rose-600 dark:text-rose-400">
          {provisioningError}
        </div>
      {/if}

      <div class="space-y-4 text-xs">
        <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
          <div class="space-y-1">
            <label for="prov-year-name" class="font-medium text-foreground">{isArabic ? "اسم العام الدراسي (EN)" : "Academic Year Name (EN)"}</label>
            <input
              id="prov-year-name"
              type="text"
              bind:value={provYearName}
              placeholder="e.g. 2026-2027"
              class="w-full bg-background border border-border rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-1 focus:ring-primary"
            />
          </div>
          <div class="space-y-1">
            <label for="prov-year-name-ar" class="font-medium text-foreground">{isArabic ? "اسم العام الدراسي (AR)" : "Academic Year Name (AR)"}</label>
            <input
              id="prov-year-name-ar"
              type="text"
              bind:value={provYearNameAr}
              placeholder="مثال: 2026-2027"
              class="w-full bg-background border border-border rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-1 focus:ring-primary"
            />
          </div>
        </div>

        <div class="space-y-2.5 pt-2 border-t border-border">
          <span class="font-medium text-foreground">{isArabic ? "العناصر المراد إنشاؤها:" : "Components to Generate:"}</span>

          <label class="flex items-center gap-2.5 p-2 rounded-lg border border-border bg-muted/20 cursor-pointer hover:bg-muted/40 transition-colors">
            <input type="checkbox" bind:checked={provStagesAndGrades} class="rounded text-primary focus:ring-primary" />
            <div>
              <span class="font-medium text-foreground block">{isArabic ? "الشعب والفصول الدراسية الأولية" : "Starter Classroom Sections"}</span>
              <span class="text-muted-foreground text-[11px] block">{isArabic ? "إنشاء فصول وشعب أولية للعام النشط" : "Generates Grade 1-3 classroom sections"}</span>
            </div>
          </label>

          <label class="flex items-center gap-2.5 p-2 rounded-lg border border-border bg-muted/20 cursor-pointer hover:bg-muted/40 transition-colors">
            <input type="checkbox" bind:checked={provSubjects} class="rounded text-primary focus:ring-primary" />
            <div>
              <span class="font-medium text-foreground block">{isArabic ? "المواد الدراسية الأساسية" : "Core Curriculum Subjects"}</span>
              <span class="text-muted-foreground text-[11px] block">{isArabic ? "رياضيات، لغة عربية، علوم، إنجليزية، إسلامية" : "Math, Arabic, Science, English, Islamic Studies"}</span>
            </div>
          </label>

          <label class="flex items-center gap-2.5 p-2 rounded-lg border border-border bg-muted/20 cursor-pointer hover:bg-muted/40 transition-colors">
            <input type="checkbox" bind:checked={provAssessmentScheme} class="rounded text-primary focus:ring-primary" />
            <div>
              <span class="font-medium text-foreground block">{isArabic ? "لائحة التقييم الشهري القياسية (100 درجة)" : "Standard 100-Point Assessment Formula"}</span>
              <span class="text-muted-foreground text-[11px] block">{isArabic ? "20 واجبات + 20 شفهي + 20 مواظبة + 40 اختبار" : "20 homework + 20 oral + 20 attendance + 40 written test"}</span>
            </div>
          </label>

          <label class="flex items-center gap-2.5 p-2 rounded-lg border border-border bg-muted/20 cursor-pointer hover:bg-muted/40 transition-colors">
            <input type="checkbox" bind:checked={provTuitionFee} class="rounded text-primary focus:ring-primary" />
            <div>
              <span class="font-medium text-foreground block">{isArabic ? "قالب الرسوم الدراسية السنوية" : "Standard Annual Tuition Template"}</span>
              <span class="text-muted-foreground text-[11px] block">{isArabic ? "مرتبط بحساب الإيراد والذمم المدنية الفعلي" : "Mapped to real revenue & receivable accounts"}</span>
            </div>
          </label>
        </div>
      </div>

      <div class="flex items-center justify-end gap-3 pt-3 border-t border-border">
        <button
          type="button"
          onclick={() => (isProvisionModalOpen = false)}
          disabled={isProvisioning}
          class="px-4 py-2 rounded-xl text-xs font-medium border border-border hover:bg-muted transition-colors"
        >
          {isArabic ? "إلغاء" : "Cancel"}
        </button>

        <button
          type="button"
          onclick={handleProvisionDefaults}
          disabled={isProvisioning}
          class="inline-flex items-center gap-2 px-5 py-2 rounded-xl text-xs font-medium bg-primary text-primary-foreground hover:bg-primary/90 shadow-sm transition-all"
        >
          {#if isProvisioning}
            <ArrowPath size={14} class="animate-spin" />
            <span>{isArabic ? "جاري الإنشاء..." : "Provisioning..."}</span>
          {:else}
            <Sparkles size={14} />
            <span>{isArabic ? "إنشاء الهيكل الأساسي" : "Scaffold Now"}</span>
          {/if}
        </button>
      </div>
    </div>
  </div>
{/if}
