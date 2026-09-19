<!--
  Modern, sleek Step Indicator and Progress Bar for the Onboarding Wizard.
-->
<script lang="ts">
import { getLocale } from "$lib/paraglide/runtime";
import { Progress } from "@orbis/ui/progress";
import {
  Sparkles,
  School,
  Calendar,
  Clock,
  Building2,
  GraduationCap,
  BookOpen,
  DollarSign,
  CheckCircle2,
  Rocket,
} from "@lucide/svelte";

let {
  currentStep,
  totalSteps = 10,
  onStepClick,
}: {
  currentStep: number;
  totalSteps?: number;
  onStepClick?: (step: number) => void;
} = $props();

const isArabic = $derived(getLocale().startsWith("ar"));

const steps = [
  { id: 0, icon: Sparkles, name_ar: "الترحيب", name_en: "Welcome" },
  { id: 1, icon: School, name_ar: "نوع المؤسسة", name_en: "Institution" },
  { id: 2, icon: Calendar, name_ar: "التقويم", name_en: "Calendar" },
  { id: 3, icon: Clock, name_ar: "الجدول", name_en: "Schedule" },
  { id: 4, icon: Building2, name_ar: "المرافق", name_en: "Facilities" },
  { id: 5, icon: GraduationCap, name_ar: "الصفوف", name_en: "Grades" },
  { id: 6, icon: BookOpen, name_ar: "المواد", name_en: "Subjects" },
  { id: 7, icon: DollarSign, name_ar: "الرسوم", name_en: "Finance" },
  { id: 8, icon: CheckCircle2, name_ar: "المراجعة", name_en: "Review" },
  { id: 9, icon: Rocket, name_ar: "الإطلاق", name_en: "Launch" },
];

const progressPercent = $derived(
  Math.round(((currentStep + 1) / totalSteps) * 100)
);

const currentStepObj = $derived(steps[currentStep] || steps[0]);
</script>

<div class="w-full max-w-4xl mx-auto mb-6 px-4 space-y-3">
  <!-- Stepper Navigation Pills (Desktop & Tablet) -->
  <div class="hidden md:flex items-center justify-between gap-1 overflow-x-auto py-1 scrollbar-hide select-none">
    {#each steps as step}
      {@const isCompleted = step.id < currentStep}
      {@const isCurrent = step.id === currentStep}
      
      <button
        type="button"
        disabled={step.id > currentStep}
        onclick={() => onStepClick?.(step.id)}
        class="flex items-center gap-1.5 px-2.5 py-1.5 rounded-lg text-xs font-medium transition-all duration-200 cursor-pointer disabled:cursor-not-allowed disabled:opacity-40 {isCurrent ? 'bg-primary text-primary-foreground font-semibold shadow-xs' : isCompleted ? 'bg-card border border-border/60 text-foreground hover:bg-muted/50' : 'text-muted-foreground bg-transparent'}"
      >
        <step.icon size={13} class={isCompleted ? "text-emerald-500" : ""} />
        <span class="truncate">{isArabic ? step.name_ar : step.name_en}</span>
      </button>
      
      {#if step.id < steps.length - 1}
        <div class="w-2 h-px bg-border/40 shrink-0"></div>
      {/if}
    {/each}
  </div>

  <!-- Mobile Step Header & Progress Line -->
  <div class="flex md:hidden items-center justify-between text-xs px-1">
    <div class="flex items-center gap-2">
      <currentStepObj.icon size={14} class="text-primary" />
      <span class="font-bold text-foreground">
        {isArabic ? currentStepObj.name_ar : currentStepObj.name_en}
      </span>
    </div>
    <span class="font-mono text-muted-foreground text-[11px]">
      {currentStep + 1} / {totalSteps}
    </span>
  </div>

  <!-- Animated Progress Bar -->
  <Progress value={progressPercent} class="h-1 bg-muted/60" />
</div>
