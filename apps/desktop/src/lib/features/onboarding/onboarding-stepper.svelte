<!--
  Modern, sleek Step Indicator and Progress Bar for the Onboarding Wizard.
  9 Focused Steps eliminating duplicate setup screens.
-->
<script lang="ts">
import { getLocale } from "$lib/paraglide/runtime";
import { Progress } from "@orbis/ui/progress";
import {
  Sparkles,
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
  totalSteps = 9,
  onStepClick,
}: {
  currentStep: number;
  totalSteps?: number;
  onStepClick?: (step: number) => void;
} = $props();

const isArabic = $derived(getLocale().startsWith("ar"));

const steps = [
  { id: 0, icon: Sparkles, name_ar: "الترحيب", name_en: "Welcome" },
  { id: 1, icon: Calendar, name_ar: "التقويم", name_en: "Calendar" },
  { id: 2, icon: Clock, name_ar: "الجدول", name_en: "Schedule" },
  { id: 3, icon: Building2, name_ar: "المرافق", name_en: "Facilities" },
  { id: 4, icon: GraduationCap, name_ar: "الصفوف", name_en: "Grades" },
  { id: 5, icon: BookOpen, name_ar: "المواد", name_en: "Subjects" },
  { id: 6, icon: DollarSign, name_ar: "الرسوم", name_en: "Finance" },
  { id: 7, icon: CheckCircle2, name_ar: "المراجعة", name_en: "Review" },
  { id: 8, icon: Rocket, name_ar: "الإطلاق", name_en: "Launch" },
];

const progressPercent = $derived(
  Math.round(((currentStep + 1) / totalSteps) * 100)
);

const currentStepObj = $derived(steps[currentStep] || steps[0]);
</script>

<div class="w-full max-w-5xl mx-auto px-4 space-y-2 select-none">
  <!-- Desktop Stepper Navigation Tabs -->
  <div class="hidden md:flex items-center justify-between gap-1 overflow-x-auto scrollbar-hide py-1">
    {#each steps as st, idx}
      {@const isActive = currentStep === idx}
      {@const isPast = currentStep > idx}
      
      <button
        type="button"
        onclick={() => onStepClick?.(idx)}
        class="flex items-center gap-1.5 px-2.5 py-1 rounded-lg text-[11px] font-medium transition-all duration-150 cursor-pointer {isActive ? 'bg-primary text-primary-foreground font-bold shadow-xs' : isPast ? 'text-foreground hover:bg-muted/50' : 'text-muted-foreground/60 hover:text-muted-foreground'}"
      >
        <st.icon size={12} class={isActive ? 'text-primary-foreground' : isPast ? 'text-primary' : 'text-muted-foreground/60'} />
        <span>{isArabic ? st.name_ar : st.name_en}</span>
      </button>

      {#if idx < steps.length - 1}
        <div class="h-px flex-1 min-w-2 bg-border/40 shrink-0"></div>
      {/if}
    {/each}
  </div>

  <!-- Mobile Step Header -->
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
