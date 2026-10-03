<script lang="ts">
import { Avatar, type AvatarController } from "@orbis/avatar-engine";
import { Sparkles, ArrowLeft, ArrowRight } from "@orbis/icons";
import { Badge } from "@orbis/ui/badge";
import { getLocale } from "$lib/paraglide/runtime";

let {
  avatarController,
  suggestions = [],
  onselectSuggestion,
}: {
  avatarController?: AvatarController;
  suggestions?: string[];
  onselectSuggestion: (suggestion: string) => void;
} = $props();

const isRtl = $derived(getLocale() === "ar");

const defaultSuggestions = [
  {
    title: isRtl ? "إحصائيات الطلاب والدرجات" : "Student Stats & Grades",
    desc: isRtl ? "ما هي إحصائيات الحضور والغياب والدرجات لهذا الفصل؟" : "What are the attendance and academic grade statistics for this term?",
  },
  {
    title: isRtl ? "المستحقات والمتابعة المالية" : "Financial Follow-up",
    desc: isRtl ? "ابحث عن الطلاب وقدم ملخصاً عن الرسوم المستحقة وغير المسددة" : "Search students and summarize outstanding tuition fees",
  },
  {
    title: isRtl ? "الفصول والجدول الأسبوعي" : "Active Classes & Schedule",
    desc: isRtl ? "ما هي الفصول والمواد الدراسية النشطة حالياً في النظام؟" : "List active classrooms, courses and weekly schedule",
  },
  {
    title: isRtl ? "تقرير إداري تحليلي" : "Comprehensive Management Report",
    desc: isRtl ? "أنشئ تقريراً إحصائياً شاملاً عن أداء المدرسة هذا الفصل" : "Generate an executive report analyzing overall school performance",
  },
];
</script>

<div class="flex-1 flex flex-col items-center justify-center px-4 py-6 select-none overflow-y-auto">
  <!-- Glowing Hero Avatar Capsule from @orbis/avatar-engine -->
  <div class="mb-3.5 flex items-center justify-center">
      <Avatar controller={avatarController} hat="pilot" width="98px" height="98px" />
  </div>

  <p class="text-[11.5px] text-muted-foreground text-center max-w-[310px] leading-relaxed mb-5 select-none">
    {isRtl ? "مساعدك المتكامل للتحليل الأكاديمي، كشوف الطلاب، الجداول، والمتابعة المالية." : "Your unified copilot for school management, academic analysis, and financial tracking."}
  </p>

  <!-- Quick Action Suggestion Cards -->
  <div class="flex flex-col gap-2 w-full max-w-[360px]">
    {#each defaultSuggestions as item}
      <button
        type="button"
        onclick={() => onselectSuggestion(item.desc)}
        class="w-full text-start p-2.5 bg-card/80 hover:bg-card border border-border/70 hover:border-primary/40 rounded-2xl cursor-pointer transition-all active:scale-[0.99] shadow-2xs hover:shadow-xs flex items-center justify-between group"
      >
        <div class="flex flex-col min-w-0 pe-2">
          <span class="text-[12px] font-bold text-foreground group-hover:text-primary transition-colors truncate">
            {item.title}
          </span>
          <span class="text-[10.5px] text-muted-foreground line-clamp-1">
            {item.desc}
          </span>
        </div>
        <div class="w-6 h-6 rounded-lg bg-muted/50 group-hover:bg-primary/10 flex items-center justify-center text-muted-foreground group-hover:text-primary transition-colors shrink-0">
          {#if isRtl}
            <ArrowLeft size={12} />
          {:else}
            <ArrowRight size={12} />
          {/if}
        </div>
      </button>
    {/each}
  </div>
</div>
