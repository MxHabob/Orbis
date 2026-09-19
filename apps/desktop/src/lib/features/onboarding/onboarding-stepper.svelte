<!--
  Minimalist Luxury Step Capsule & Counter (Centered)
  Ultra-refined, clean centered capsule indicators matching design tokens.
-->
<script lang="ts">
import { getLocale } from "$lib/paraglide/runtime";

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
  { id: 0, name_ar: "الترحيب", name_en: "Welcome" },
  { id: 1, name_ar: "التقويم", name_en: "Calendar" },
  { id: 2, name_ar: "الجدول", name_en: "Schedule" },
  { id: 3, name_ar: "المرافق", name_en: "Facilities" },
  { id: 4, name_ar: "الصفوف", name_en: "Grades" },
  { id: 5, name_ar: "المواد", name_en: "Subjects" },
  { id: 6, name_ar: "الرسوم", name_en: "Finance" },
  { id: 7, name_ar: "المراجعة", name_en: "Review" },
  { id: 8, name_ar: "الإطلاق", name_en: "Launch" },
];

const currentStepObj = $derived(steps[currentStep] || steps[0]);
</script>

<div class="w-full flex items-center justify-center py-2 select-none shrink-0">
  <!-- Centered Luxury Stepper Capsule Unit -->
  <div class="inline-flex items-center gap-3.5 px-4 py-1.5 rounded-full bg-card/90 border border-border/80 shadow-2xs backdrop-blur-md">
    
    <!-- Step Name & Status Dot -->
    <div class="flex items-center gap-2">
      <span class="size-2 rounded-full bg-primary animate-pulse"></span>
      <span class="font-bold text-foreground text-xs tracking-tight">
        {isArabic ? currentStepObj.name_ar : currentStepObj.name_en}
      </span>
    </div>

    <!-- Capsule Indicators Track (Slightly larger, elegant, clickable) -->
    <div class="flex items-center gap-1.5 px-1">
      {#each steps as st, idx}
        {@const isActive = idx === currentStep}
        {@const isPast = idx < currentStep}
        <button
          type="button"
          onclick={() => onStepClick?.(idx)}
          class="h-2 rounded-full transition-all duration-200 cursor-pointer focus:outline-none
            {isActive
              ? 'w-10 bg-primary shadow-xs'
              : isPast
              ? 'w-3.5 bg-primary/40 hover:bg-primary/70'
              : 'w-3.5 bg-muted hover:bg-muted-foreground/30'}"
          title="{idx + 1}. {isArabic ? st.name_ar : st.name_en}"
          aria-label="{idx + 1}. {isArabic ? st.name_ar : st.name_en}"
        ></button>
      {/each}
    </div>

    <!-- Step Counter -->
    <span class="font-mono text-xs font-semibold text-muted-foreground tracking-wider border-s border-border/50 ps-2.5">
      <span class="text-foreground font-bold">{String(currentStep + 1).padStart(2, "0")}</span>
      <span class="opacity-40">/</span>
      <span>{String(totalSteps).padStart(2, "0")}</span>
    </span>

  </div>
</div>
