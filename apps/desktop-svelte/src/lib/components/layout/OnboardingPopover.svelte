<script lang="ts">
import {
  Popover,
  PopoverTrigger,
  PopoverContent,
  Button
} from "@orbis/ui-svelte";
import { CheckCircle2, Circle } from "lucide-svelte";
import { onboardingState } from "$lib/components/onboarding/onboarding-state.svelte";
import { authStore } from "$lib/stores";
import { goto } from "$app/navigation";
import { localizeHref } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";

function handleChecklistItemClick(stepIndex: number) {
  onboardingState.goToStep(stepIndex + 1);
  goto(localizeHref("/onboarding"));
}
</script>

<Popover>
  <PopoverTrigger>
    <Button
      variant="outline"
      size="icon"
      class="w-full h-8 flex items-center justify-center hover:bg-slate-50 dark:hover:bg-zinc-800/60 relative border-b border-[#f1f5f9] dark:border-zinc-800 cursor-pointer onboarding-popover-trigger"
    >
      <div class="relative">
        <div class="w-[20px] h-[20px] rounded-full border border-[#6366f1] dark:border-indigo-400 text-[#6366f1] dark:text-indigo-400 flex items-center justify-center text-[9px] font-bold">
          {7 - onboardingState.tasksCompletedCount}
        </div>
        <span class="absolute -top-0.5 -right-0.5 w-1.5 h-1.5 rounded-full bg-[#6366f1] border border-white dark:border-zinc-900"></span>
      </div>
    </Button>
  </PopoverTrigger>
  
  <PopoverContent
    side="right"
    align="end"
    sideOffset={12}
    class="w-80 bg-card border border-border shadow-2xl rounded-xl p-5 z-50 flex flex-col gap-4 font-sans text-start onboarding-popover-panel"
  >
    <div class="flex flex-col gap-1">
      <h4 class="font-bold text-sm text-foreground flex items-center gap-2">
        <span>{m.onboarding_progress_label() || 'Getting Started'}</span>
      </h4>
      <p class="text-xs text-muted-foreground font-sans">
        {onboardingState.tasksCompletedCount} out of 7 tasks completed
      </p>
      <div class="w-full bg-slate-100 dark:bg-zinc-800 h-1.5 rounded-full overflow-hidden mt-1">
        <div class="bg-primary h-full transition-all duration-300" style="width: {(onboardingState.tasksCompletedCount / 7) * 100}%"></div>
      </div>
    </div>

    <div class="flex flex-col gap-1 font-sans">
      {#each [
        { label: "Academic Calendar & Years", labelAr: "التقويم الدراسي والفصول", desc: "Define academic year periods, dates, and semesters.", descAr: "تحديد الفترات والتواريخ والفصول الدراسية." },
        { label: "Buildings & Rooms Allocation", labelAr: "توزيع المباني والقاعات", desc: "Configure classrooms, computer labs, and lecture halls.", descAr: "تهيئة الفصول الدراسية ومختبرات الحاسوب والكيمياء." },
        { label: "Academic Stages & Classes", labelAr: "المراحل الدراسية والشعب", desc: "Choose active grade levels and division sections.", descAr: "تفعيل المراحل التعليمية وتحديد شعب الصفوف." },
        { label: "Daily Schedule & Periods", labelAr: "الحصص الدراسية اليومية", desc: "Setup period times, starts, ends, and durations.", descAr: "ضبط مواعيد الحصص وفترات الاستراحة اليومية." },
        { label: "Subjects & Curriculums", labelAr: "المواد والمناهج الدراسية", desc: "Select subjects and customize identification colors.", descAr: "تحديد المواد التعليمية وألوان تمييزها." },
        { label: "Registration Fees Setup", labelAr: "رسوم تسجيل الطلاب", desc: "Define standard tuition fees per grade level.", descAr: "تحديد قيم رسوم التسجيل لكل مرحلة دراسية." },
        { label: "Review & Launch System", labelAr: "المراجعة وتدشين النظام", desc: "Finalize structure and launch the school system.", descAr: "مراجعة الهيكل الإداري وتدشين المحاسبة المالية." }
      ] as task, idx}
        {@const isCompleted = onboardingState.completedSteps[idx]}
        <button
          onclick={() => handleChecklistItemClick(idx)}
          class="flex items-start gap-3 p-2 rounded-lg hover:bg-muted text-start w-full group transition-all duration-150 cursor-pointer text-foreground bg-transparent border-0 font-sans"
        >
          <div class="mt-0.5 shrink-0 text-primary transition-transform group-hover:scale-110 font-sans">
            {#if isCompleted}
              <CheckCircle2 class="w-4 h-4 text-emerald-500 font-sans" />
            {:else}
              <Circle class="w-4 h-4 text-muted-foreground font-sans" />
            {/if}
          </div>
          <div class="flex flex-col gap-0.5 font-sans">
            <span class="text-xs font-semibold leading-none {isCompleted ? 'text-muted-foreground line-through decoration-muted-foreground/50 font-normal font-sans' : 'text-foreground font-bold font-sans'}">
              {onboardingState.locale === 'ar' ? task.labelAr : task.label}
            </span>
            <span class="text-[10px] text-muted-foreground leading-normal font-sans font-light">
              {onboardingState.locale === 'ar' ? task.descAr : task.desc}
            </span>
          </div>
        </button>
      {/each}
    </div>
  </PopoverContent>
</Popover>
