<script lang="ts">
import {
  Popover,
  PopoverTrigger,
  PopoverContent,
  Button
} from "@orbis/ui-svelte";
import { Check, ChevronRight, ChevronLeft } from "lucide-svelte";
import { onboardingState } from "$lib/components/onboarding/onboarding-state.svelte";
import { authStore } from "$lib/stores";
import { goto } from "$app/navigation";
import { localizeHref } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";

function handleChecklistItemClick(stepIndex: number) {
  onboardingState.goToStep(stepIndex + 1);
  goto(localizeHref("/onboarding"));
}

// Onboarding tasks variables
const totalTasks = 7;
const completedTasks = $derived(authStore.isSetupComplete ? 7 : onboardingState.tasksCompletedCount);
const remainingTasks = $derived(7 - completedTasks);
const progressPercent = $derived((completedTasks / totalTasks) * 100);

const activeIndex = $derived(
  authStore.isSetupComplete 
    ? -1 
    : onboardingState.completedSteps.findIndex((step) => !step)
);
</script>

<Popover>
  <PopoverTrigger>
    <Button
      variant="outline"
      size="icon"
      class="w-full h-8 flex items-center justify-center hover:bg-slate-100 dark:hover:bg-zinc-800/60 relative border-b border-[#f1f5f9] dark:border-zinc-800 cursor-pointer onboarding-popover-trigger"
    >
      <div class="relative w-6 h-6 flex items-center justify-center">
        <!-- Progress Circular Ring SVG -->
        <svg class="absolute inset-0 w-6 h-6 transform -rotate-90 select-none" viewBox="0 0 24 24">
          <!-- Background track -->
          <circle
            cx="12"
            cy="12"
            r="8.5"
            class="stroke-zinc-300 dark:stroke-zinc-700/80 fill-none"
            stroke-width="2"
          />
          <!-- Progress highlight -->
          {#if progressPercent > 0}
            <circle
              cx="12"
              cy="12"
              r="8.5"
              class="stroke-[#6366f1] dark:stroke-indigo-400 fill-none transition-all duration-300"
              stroke-width="2"
              stroke-dasharray="53.4"
              stroke-dashoffset={53.4 - (53.4 * progressPercent) / 100}
              stroke-linecap="round"
            />
          {/if}
        </svg>
        
        <!-- Inner status indicator -->
        {#if remainingTasks === 0}
          <Check class="w-3.5 h-3.5 text-emerald-500 stroke-[3px]" />
        {:else}
          <span class="text-[9px] font-bold text-slate-800 dark:text-zinc-200">
            {remainingTasks}
          </span>
          <!-- Notification dot badge -->
          <span class="absolute top-0 right-0 w-1.5 h-1.5 rounded-full bg-[#6366f1] border border-white dark:border-zinc-900"></span>
        {/if}
      </div>
    </Button>
  </PopoverTrigger>
  
  <PopoverContent
    side="right"
    align="end"
    sideOffset={12}
    class="w-[360px] bg-[#2b2836] border border-[#3c384a] shadow-[0_12px_36px_rgba(0,0,0,0.55)] rounded-xl p-0 overflow-hidden z-50 flex flex-col font-sans text-start onboarding-popover-panel relative"
  >
    <!-- Pointer Bezel/Tail pointing to the sidebar trigger -->
    <div class="absolute bottom-[20px] -left-1.5 w-3 h-3 bg-[#2b2836] border-l border-b border-[#3c384a] rotate-45 z-20"></div>

    <!-- Header Section -->
    <div class="flex items-center gap-3.5 p-4 border-b border-zinc-800/80 bg-[#2b2836] z-10 select-none">
      <!-- Outer Ring loader SVG -->
      <svg class="w-8 h-8 transform -rotate-90 text-indigo-400 shrink-0" viewBox="0 0 24 24">
        <circle cx="12" cy="12" r="9" class="stroke-[#393547] fill-none" stroke-width="2.5" />
        <circle cx="12" cy="12" r="9" class="stroke-[#a199ff] fill-none" stroke-width="2.5" stroke-dasharray="56.5" stroke-dashoffset={56.5 - (56.5 * progressPercent) / 100} stroke-linecap="round" />
      </svg>
      <div class="flex flex-col">
        <h4 class="font-bold text-sm text-zinc-100 leading-tight">
          {m.onboarding_progress_label() || 'Getting Started'}
        </h4>
        <p class="text-[11px] text-zinc-400 font-semibold mt-0.5">
          {completedTasks} out of 7 tasks completed
        </p>
      </div>
    </div>

    <!-- Scrollable Checklist Area -->
    <div class="onboarding-list flex-1 overflow-y-auto max-h-[340px] p-2 flex flex-col gap-0.5 bg-[#2b2836] z-10">
      {#each [
        { label: "Academic Calendar & Years", labelAr: "التقويم الدراسي والفصول", desc: "Define academic year periods, dates, and semesters.", descAr: "تحديد الفترات والتواريخ والفصول الدراسية." },
        { label: "Buildings & Rooms Allocation", labelAr: "توزيع المباني والقاعات", desc: "Configure classrooms, computer labs, and lecture halls.", descAr: "تهيئة الفصول الدراسية ومختبرات الحاسوب والكيمياء." },
        { label: "Academic Stages & Classes", labelAr: "المراحل الدراسية والشعب", desc: "Choose active grade levels and division sections.", descAr: "تفعيل المراحل التعليمية وتحديد شعب الصفوف." },
        { label: "Daily Schedule & Periods", labelAr: "الحصص الدراسية اليومية", desc: "Setup period times, starts, ends, and durations.", descAr: "ضبط مواعيد الحصص وفترات الاستراحة اليومية." },
        { label: "Subjects & Curriculums", labelAr: "المواد والمناهج الدراسية", desc: "Select subjects and customize identification colors.", descAr: "تحديد المواد التعليمية وألوان تمييزها." },
        { label: "Registration Fees Setup", labelAr: "رسوم تسجيل الطلاب", desc: "Define standard tuition fees per grade level.", descAr: "تحديد قيم رسوم التسجيل لكل مرحلة دراسية." },
        { label: "Review & Launch System", labelAr: "المراجعة وتدشين النظام", desc: "Finalize structure and launch the school system.", descAr: "مراجعة الهيكل الإداري وتدشين المحاسبة المالية." }
      ] as task, idx}
        {@const isCompleted = authStore.isSetupComplete ? true : onboardingState.completedSteps[idx]}
        {@const isActive = !isCompleted && idx === activeIndex}
        
        <button
          onclick={() => {
            if (!authStore.isSetupComplete) {
              handleChecklistItemClick(idx);
            }
          }}
          disabled={authStore.isSetupComplete}
          class="flex items-start justify-between p-3 rounded-lg hover:bg-[#343040]/40 text-start w-full group transition-all duration-150 cursor-pointer text-foreground bg-transparent border-0 font-sans disabled:opacity-100 disabled:hover:bg-transparent"
        >
          <div class="flex items-start gap-3 flex-1 min-w-0">
            {#if isCompleted}
              <div class="mt-0.5 shrink-0 text-zinc-500">
                <Check class="w-3.5 h-3.5 text-zinc-400 stroke-[3px]" />
              </div>
            {:else}
              <!-- Spacer to align texts of non-completed items -->
              <div class="w-3.5 h-3.5 shrink-0 mt-0.5"></div>
            {/if}
            
            <div class="flex flex-col gap-1 flex-1 min-w-0">
              <span class="text-[11.5px] leading-tight font-sans {isCompleted ? 'text-zinc-500 line-through decoration-zinc-500/40 font-normal' : 'text-zinc-100 font-bold'}">
                {onboardingState.locale === 'ar' ? task.labelAr : task.label}
              </span>
              <span class="text-[10.5px] leading-relaxed font-sans font-normal {isCompleted ? 'text-zinc-600' : isActive ? 'text-zinc-300' : 'text-zinc-400'}">
                {onboardingState.locale === 'ar' ? task.descAr : task.desc}
              </span>
            </div>
          </div>
          
          {#if isActive}
            <div class="mt-1 shrink-0 text-zinc-400 ml-2">
              {#if onboardingState.locale === 'ar'}
                <ChevronLeft size={12} strokeWidth={2.5} />
              {:else}
                <ChevronRight size={12} strokeWidth={2.5} />
              {/if}
            </div>
          {/if}
        </button>
      {/each}
    </div>
  </PopoverContent>
</Popover>

<style>
  :global(.onboarding-list) {
    scrollbar-width: thin;
    scrollbar-color: rgba(255, 255, 255, 0.15) transparent;
  }
  :global(.onboarding-list)::-webkit-scrollbar {
    width: 3px !important;
    display: block !important;
  }
  :global(.onboarding-list)::-webkit-scrollbar-track {
    background: transparent !important;
  }
  :global(.onboarding-list)::-webkit-scrollbar-thumb {
    background-color: rgba(255, 255, 255, 0.15) !important;
    border-radius: 9999px !important;
  }
  :global(.onboarding-list)::-webkit-scrollbar-thumb:hover {
    background-color: rgba(255, 255, 255, 0.25) !important;
  }
</style>
