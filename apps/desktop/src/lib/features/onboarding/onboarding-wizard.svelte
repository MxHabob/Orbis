<!--
  Onboarding Wizard — Streamlined 3-step initial setup.
  Step 0: Academic Year & Periods
  Step 1: Classes & Grade Levels
  Step 2: Core Subjects & Finalization
-->
<script lang="ts">
import { goto } from "$app/navigation";
import { localizeHref, getLocale } from "$lib/paraglide/runtime";
import { authStore } from "$lib/stores/auth.svelte";
import { onboardingState } from "./state.svelte";
import {
  Calendar,
  Layers,
  BookOpen,
  ArrowRight,
  ArrowLeft,
  CheckCircle2,
  Plus,
  Trash2,
  AlertCircle,
  Loader2,
} from "@lucide/svelte";

const isArabic = $derived(getLocale().startsWith("ar"));

let newClassName = $state("");
let newClassLevel = $state("Grade 1");
let newClassDivision = $state("A");

let newSubjectCode = $state("");
let newSubjectName = $state("");
let newSubjectNameAr = $state("");
let newSubjectIsCore = $state(true);

function handleAddClass() {
  if (!newClassName.trim()) return;
  onboardingState.addClass(
    newClassName.trim(),
    newClassLevel.trim(),
    newClassDivision.trim() || undefined
  );
  newClassName = "";
}

function handleAddSubject() {
  if (!newSubjectCode.trim() || !newSubjectName.trim()) return;
  onboardingState.addSubject(
    newSubjectCode.trim().toUpperCase(),
    newSubjectName.trim(),
    newSubjectNameAr.trim() || undefined,
    newSubjectIsCore
  );
  newSubjectCode = "";
  newSubjectName = "";
  newSubjectNameAr = "";
}

async function handleFinish() {
  const success = await onboardingState.submit(authStore.orgId);
  if (success) {
    goto(localizeHref(`/${authStore.orgId}`));
  }
}
</script>

<div class="min-h-screen bg-gradient-to-br from-background via-background/95 to-primary/5 text-foreground flex flex-col justify-between p-6 md:p-10 font-sans">
  <!-- Top Navigation & Stepper Header -->
  <div class="max-w-4xl w-full mx-auto">
    <div class="flex items-center justify-between pb-6 border-b border-border/40">
      <div>
        <h1 class="text-2xl font-bold tracking-tight text-foreground">
          {isArabic ? "تهيئة الهيكل الأكاديمي" : "Initial Academic Setup"}
        </h1>
        <p class="text-xs text-muted-foreground mt-0.5">
          {isArabic
            ? "خطوات سريعة لإعداد السنة الدراسية والفصول والمواد"
            : "Quick setup for academic calendar, classes, and subjects"}
        </p>
      </div>

      <!-- Stepper Pill -->
      <div class="flex items-center gap-2 bg-card border border-border/80 px-3 py-1.5 rounded-full shadow-sm text-xs font-medium">
        <span class="text-primary font-bold">{onboardingState.currentStep + 1}</span>
        <span class="text-muted-foreground">/</span>
        <span class="text-muted-foreground">3</span>
      </div>
    </div>

    <!-- Stepper Tabs -->
    <div class="grid grid-cols-3 gap-3 my-6">
      <!-- Step 0 Tab -->
      <div
        class="flex items-center gap-3 p-3 rounded-xl border transition-all duration-200 {onboardingState.currentStep === 0
          ? 'bg-primary/10 border-primary/40 text-primary shadow-sm'
          : onboardingState.currentStep > 0
          ? 'bg-card border-border text-foreground'
          : 'bg-card/40 border-border/40 text-muted-foreground opacity-60'}"
      >
        <div class="w-8 h-8 rounded-lg flex items-center justify-center {onboardingState.currentStep === 0 ? 'bg-primary text-primary-foreground' : 'bg-muted'}">
          {#if onboardingState.currentStep > 0}
            <CheckCircle2 class="w-4 h-4 text-emerald-500" />
          {:else}
            <Calendar class="w-4 h-4" />
          {/if}
        </div>
        <div class="flex flex-col min-w-0">
          <span class="text-xs font-semibold truncate">{isArabic ? "1. التقويم الدراسي" : "1. Calendar"}</span>
          <span class="text-[10px] text-muted-foreground truncate">{isArabic ? "العام والفصول" : "Year & Terms"}</span>
        </div>
      </div>

      <!-- Step 1 Tab -->
      <div
        class="flex items-center gap-3 p-3 rounded-xl border transition-all duration-200 {onboardingState.currentStep === 1
          ? 'bg-primary/10 border-primary/40 text-primary shadow-sm'
          : onboardingState.currentStep > 1
          ? 'bg-card border-border text-foreground'
          : 'bg-card/40 border-border/40 text-muted-foreground opacity-60'}"
      >
        <div class="w-8 h-8 rounded-lg flex items-center justify-center {onboardingState.currentStep === 1 ? 'bg-primary text-primary-foreground' : 'bg-muted'}">
          {#if onboardingState.currentStep > 1}
            <CheckCircle2 class="w-4 h-4 text-emerald-500" />
          {:else}
            <Layers class="w-4 h-4" />
          {/if}
        </div>
        <div class="flex flex-col min-w-0">
          <span class="text-xs font-semibold truncate">{isArabic ? "2. الفصول والصفوف" : "2. Classes"}</span>
          <span class="text-[10px] text-muted-foreground truncate">{isArabic ? "المستويات والشعب" : "Grades & Sections"}</span>
        </div>
      </div>

      <!-- Step 2 Tab -->
      <div
        class="flex items-center gap-3 p-3 rounded-xl border transition-all duration-200 {onboardingState.currentStep === 2
          ? 'bg-primary/10 border-primary/40 text-primary shadow-sm'
          : 'bg-card/40 border-border/40 text-muted-foreground opacity-60'}"
      >
        <div class="w-8 h-8 rounded-lg flex items-center justify-center {onboardingState.currentStep === 2 ? 'bg-primary text-primary-foreground' : 'bg-muted'}">
          <BookOpen class="w-4 h-4" />
        </div>
        <div class="flex flex-col min-w-0">
          <span class="text-xs font-semibold truncate">{isArabic ? "3. المواد الدراسية" : "3. Subjects"}</span>
          <span class="text-[10px] text-muted-foreground truncate">{isArabic ? "المناهج الأساسية" : "Core Curriculum"}</span>
        </div>
      </div>
    </div>
  </div>

  <!-- Step Content Container -->
  <div class="flex-1 max-w-4xl w-full mx-auto my-2 overflow-y-auto">
    {#if onboardingState.error}
      <div class="mb-4 flex items-center gap-3 p-4 rounded-xl bg-destructive/10 border border-destructive/20 text-destructive text-sm">
        <AlertCircle class="w-5 h-5 shrink-0" />
        <span>{onboardingState.error}</span>
      </div>
    {/if}

    <!-- STEP 0: Academic Year & Periods -->
    {#if onboardingState.currentStep === 0}
      <div class="bg-card border border-border/80 rounded-2xl p-6 shadow-sm space-y-6">
        <div class="border-b border-border/40 pb-4">
          <h2 class="text-lg font-semibold">{isArabic ? "تحديد العام الدراسي والفصول" : "Academic Year & Terms"}</h2>
          <p class="text-xs text-muted-foreground mt-0.5">
            {isArabic ? "قم بتحديد مسمى العام الدراسي وتواريخ البداية والنهاية" : "Specify the academic year name and session date ranges"}
          </p>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <label class="block text-xs font-medium text-foreground mb-1.5">{isArabic ? "اسم العام الدراسي (EN)" : "Academic Year Name"}</label>
            <input
              type="text"
              bind:value={onboardingState.academicYearName}
              class="w-full bg-background border border-border rounded-xl px-3.5 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-primary/40"
              placeholder="e.g. 2026/2027"
            />
          </div>
          <div>
            <label class="block text-xs font-medium text-foreground mb-1.5">{isArabic ? "اسم العام الدراسي (عربي)" : "Academic Year Name (Arabic)"}</label>
            <input
              type="text"
              bind:value={onboardingState.academicYearNameAr}
              class="w-full bg-background border border-border rounded-xl px-3.5 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-primary/40 text-right"
              placeholder="مثال: العام الدراسي 2026/2027"
            />
          </div>
          <div>
            <label class="block text-xs font-medium text-foreground mb-1.5">{isArabic ? "تاريخ البداية" : "Start Date"}</label>
            <input
              type="date"
              bind:value={onboardingState.academicYearStart}
              class="w-full bg-background border border-border rounded-xl px-3.5 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-primary/40"
            />
          </div>
          <div>
            <label class="block text-xs font-medium text-foreground mb-1.5">{isArabic ? "تاريخ النهاية" : "End Date"}</label>
            <input
              type="date"
              bind:value={onboardingState.academicYearEnd}
              class="w-full bg-background border border-border rounded-xl px-3.5 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-primary/40"
            />
          </div>
        </div>

        <!-- Periods List -->
        <div class="pt-4 border-t border-border/40">
          <div class="flex items-center justify-between mb-3">
            <h3 class="text-sm font-semibold">{isArabic ? "الفصول الدراسية (Terms / Semesters)" : "Academic Terms / Semesters"}</h3>
            <button
              type="button"
              onclick={() => onboardingState.addPeriod()}
              class="inline-flex items-center gap-1 text-xs text-primary hover:text-primary/80 font-medium px-2 py-1 rounded-lg hover:bg-primary/10 transition-colors"
            >
              <Plus class="w-3.5 h-3.5" />
              <span>{isArabic ? "إضافة فصل" : "Add Term"}</span>
            </button>
          </div>

          <div class="space-y-2.5">
            {#each onboardingState.periods as period, idx}
              <div class="flex items-center gap-3 p-3 bg-muted/30 border border-border/60 rounded-xl">
                <input
                  type="text"
                  bind:value={period.name}
                  class="flex-1 bg-background border border-border rounded-lg px-3 py-1.5 text-xs focus:ring-2 focus:ring-primary/40"
                  placeholder="Term name (EN)"
                />
                <input
                  type="text"
                  bind:value={period.name_ar}
                  class="flex-1 bg-background border border-border rounded-lg px-3 py-1.5 text-xs text-right focus:ring-2 focus:ring-primary/40"
                  placeholder="اسم الفصل (عربي)"
                />
                <input
                  type="date"
                  bind:value={period.start_date}
                  class="w-32 bg-background border border-border rounded-lg px-2 py-1.5 text-xs focus:ring-2 focus:ring-primary/40"
                />
                <input
                  type="date"
                  bind:value={period.end_date}
                  class="w-32 bg-background border border-border rounded-lg px-2 py-1.5 text-xs focus:ring-2 focus:ring-primary/40"
                />
                {#if onboardingState.periods.length > 1}
                  <button
                    type="button"
                    onclick={() => onboardingState.removePeriod(idx)}
                    class="text-muted-foreground hover:text-destructive p-1 rounded-md"
                  >
                    <Trash2 class="w-3.5 h-3.5" />
                  </button>
                {/if}
              </div>
            {/each}
          </div>
        </div>
      </div>

    <!-- STEP 1: Classes -->
    {:else if onboardingState.currentStep === 1}
      <div class="bg-card border border-border/80 rounded-2xl p-6 shadow-sm space-y-6">
        <div class="border-b border-border/40 pb-4">
          <h2 class="text-lg font-semibold">{isArabic ? "الفصول والشعب الدراسية" : "Classes & Sections"}</h2>
          <p class="text-xs text-muted-foreground mt-0.5">
            {isArabic ? "إعداد الصفوف الأولية والشعب لتسجيل الطلاب لاحقاً" : "Configure primary grade classes and section divisions"}
          </p>
        </div>

        <!-- Add Class Form -->
        <div class="p-4 bg-muted/20 border border-border/60 rounded-xl space-y-3">
          <span class="text-xs font-semibold text-foreground">{isArabic ? "إضافة فصل جديد" : "Add New Class"}</span>
          <div class="grid grid-cols-1 md:grid-cols-4 gap-3">
            <input
              type="text"
              bind:value={newClassName}
              class="bg-background border border-border rounded-xl px-3 py-2 text-xs focus:ring-2 focus:ring-primary/40"
              placeholder={isArabic ? "اسم الفصل (مثال: Grade 1 - A)" : "Class name (e.g. Grade 1 - A)"}
            />
            <input
              type="text"
              bind:value={newClassLevel}
              class="bg-background border border-border rounded-xl px-3 py-2 text-xs focus:ring-2 focus:ring-primary/40"
              placeholder={isArabic ? "المستوى (Grade 1)" : "Level (Grade 1)"}
            />
            <input
              type="text"
              bind:value={newClassDivision}
              class="bg-background border border-border rounded-xl px-3 py-2 text-xs focus:ring-2 focus:ring-primary/40"
              placeholder={isArabic ? "الشعبة (A)" : "Section (A)"}
            />
            <button
              type="button"
              onclick={handleAddClass}
              class="inline-flex items-center justify-center gap-1.5 bg-primary text-primary-foreground font-medium px-4 py-2 rounded-xl text-xs hover:bg-primary/90 transition-colors"
            >
              <Plus class="w-3.5 h-3.5" />
              <span>{isArabic ? "إضافة" : "Add"}</span>
            </button>
          </div>
        </div>

        <!-- Classes Grid -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
          {#each onboardingState.classes as cls, idx}
            <div class="flex items-center justify-between p-3.5 bg-muted/30 border border-border/60 rounded-xl">
              <div class="flex items-center gap-3">
                <div class="w-8 h-8 rounded-lg bg-primary/10 text-primary flex items-center justify-center font-bold text-xs">
                  {cls.division || cls.level.charAt(0)}
                </div>
                <div>
                  <div class="font-medium text-xs text-foreground">{cls.name}</div>
                  <div class="text-[10px] text-muted-foreground">{cls.level} {cls.division ? `• Section ${cls.division}` : ""}</div>
                </div>
              </div>

              {#if onboardingState.classes.length > 1}
                <button
                  type="button"
                  onclick={() => onboardingState.removeClass(idx)}
                  class="text-muted-foreground hover:text-destructive p-1 rounded-md"
                >
                  <Trash2 class="w-3.5 h-3.5" />
                </button>
              {/if}
            </div>
          {/each}
        </div>
      </div>

    <!-- STEP 2: Subjects -->
    {:else if onboardingState.currentStep === 2}
      <div class="bg-card border border-border/80 rounded-2xl p-6 shadow-sm space-y-6">
        <div class="border-b border-border/40 pb-4">
          <h2 class="text-lg font-semibold">{isArabic ? "المواد الدراسية والمقررات" : "Subjects & Courses"}</h2>
          <p class="text-xs text-muted-foreground mt-0.5">
            {isArabic ? "حدد المواد الأساسية والاختيارية للنظام" : "Specify core and elective curriculum subjects"}
          </p>
        </div>

        <!-- Add Subject Form -->
        <div class="p-4 bg-muted/20 border border-border/60 rounded-xl space-y-3">
          <span class="text-xs font-semibold text-foreground">{isArabic ? "إضافة مادة جديدة" : "Add New Subject"}</span>
          <div class="grid grid-cols-1 md:grid-cols-4 gap-3">
            <input
              type="text"
              bind:value={newSubjectCode}
              class="bg-background border border-border rounded-xl px-3 py-2 text-xs uppercase focus:ring-2 focus:ring-primary/40"
              placeholder="Code (MATH101)"
            />
            <input
              type="text"
              bind:value={newSubjectName}
              class="bg-background border border-border rounded-xl px-3 py-2 text-xs focus:ring-2 focus:ring-primary/40"
              placeholder="Name (Mathematics)"
            />
            <input
              type="text"
              bind:value={newSubjectNameAr}
              class="bg-background border border-border rounded-xl px-3 py-2 text-xs text-right focus:ring-2 focus:ring-primary/40"
              placeholder="الاسم بالعربي"
            />
            <button
              type="button"
              onclick={handleAddSubject}
              class="inline-flex items-center justify-center gap-1.5 bg-primary text-primary-foreground font-medium px-4 py-2 rounded-xl text-xs hover:bg-primary/90 transition-colors"
            >
              <Plus class="w-3.5 h-3.5" />
              <span>{isArabic ? "إضافة" : "Add"}</span>
            </button>
          </div>
        </div>

        <!-- Subjects Grid -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
          {#each onboardingState.subjects as sub, idx}
            <div class="flex items-center justify-between p-3.5 bg-muted/30 border border-border/60 rounded-xl">
              <div class="flex items-center gap-3">
                <div class="w-8 h-8 rounded-lg bg-emerald-500/10 text-emerald-500 flex items-center justify-center font-mono font-bold text-[10px]">
                  {sub.code.slice(0, 4)}
                </div>
                <div>
                  <div class="font-medium text-xs text-foreground">{sub.name} {sub.name_ar ? `(${sub.name_ar})` : ""}</div>
                  <div class="text-[10px] text-muted-foreground">{sub.code} • {sub.is_core ? (isArabic ? "إجباري" : "Core") : (isArabic ? "اختياري" : "Elective")}</div>
                </div>
              </div>

              {#if onboardingState.subjects.length > 1}
                <button
                  type="button"
                  onclick={() => onboardingState.removeSubject(idx)}
                  class="text-muted-foreground hover:text-destructive p-1 rounded-md"
                >
                  <Trash2 class="w-3.5 h-3.5" />
                </button>
              {/if}
            </div>
          {/each}
        </div>
      </div>
    {/if}
  </div>

  <!-- Bottom Navigation Actions -->
  <div class="max-w-4xl w-full mx-auto pt-6 border-t border-border/40 flex items-center justify-between">
    <button
      type="button"
      onclick={() => onboardingState.prevStep()}
      disabled={onboardingState.currentStep === 0 || onboardingState.isLoading}
      class="inline-flex items-center gap-2 px-5 py-2.5 rounded-xl border border-border text-sm font-medium text-muted-foreground hover:text-foreground hover:bg-muted/40 transition-colors disabled:opacity-40 disabled:cursor-not-allowed"
    >
      {#if isArabic}
        <ArrowRight class="w-4 h-4" />
      {:else}
        <ArrowLeft class="w-4 h-4" />
      {/if}
      <span>{isArabic ? "السابق" : "Previous"}</span>
    </button>

    {#if onboardingState.currentStep < 2}
      <button
        type="button"
        onclick={() => onboardingState.nextStep()}
        class="inline-flex items-center gap-2 px-6 py-2.5 rounded-xl bg-primary text-primary-foreground text-sm font-semibold shadow-md hover:bg-primary/90 transition-all active:scale-[0.98]"
      >
        <span>{isArabic ? "التالي" : "Next"}</span>
        {#if isArabic}
          <ArrowLeft class="w-4 h-4" />
        {:else}
          <ArrowRight class="w-4 h-4" />
        {/if}
      </button>
    {:else}
      <button
        type="button"
        onclick={handleFinish}
        disabled={onboardingState.isLoading}
        class="inline-flex items-center gap-2 px-7 py-2.5 rounded-xl bg-gradient-to-r from-primary to-emerald-600 text-white text-sm font-semibold shadow-lg hover:shadow-primary/25 transition-all active:scale-[0.98] disabled:opacity-60"
      >
        {#if onboardingState.isLoading}
          <Loader2 class="w-4 h-4 animate-spin" />
          <span>{isArabic ? "جاري الإعداد..." : "Completing Setup..."}</span>
        {:else}
          <CheckCircle2 class="w-4 h-4" />
          <span>{isArabic ? "إنهاء والبدء" : "Complete & Launch"}</span>
        {/if}
      </button>
    {/if}
  </div>
</div>
