<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Avatar } from "@orbis/avatar-engine";
import { Sparkles } from "@lucide/svelte";

let {
  suggestions = [],
  onselectSuggestion,
}: {
  suggestions?: string[];
  onselectSuggestion: (suggestion: string) => void;
} = $props();

const defaultSuggestions = [
  "ما هي إحصائيات الحضور والغياب والدرجات لهذا الفصل؟",
  "ابحث عن الطلاب وقدم ملخصاً عن الرسوم المستحقة وغير المسددة",
  "ما هي الفصول والمواد الدراسية النشطة حالياً في النظام؟",
  "أنشئ تقريراً إدارياً وتحليلياً شاملاً عن أداء المدرسة",
];

const activeSuggestions = $derived(
  suggestions && suggestions.length > 0 ? suggestions : defaultSuggestions,
);
</script>

<div class="flex-1 flex flex-col items-center justify-center px-5 py-6 select-none overflow-y-auto">
  <!-- Avatar -->
  <div class="mb-4 flex justify-center w-full">
    <Avatar hat="graduation" width="80px" height="80px" />
  </div>

  <!-- Title & Subtitle -->
  <h3 class="text-sm font-bold text-foreground mb-1 text-center flex items-center gap-1.5">
    <Sparkles size={14} class="text-primary" />
    مساعد Orbis الذكي
  </h3>
  <p class="text-xs text-muted-foreground text-center max-w-[280px] leading-relaxed mb-6 select-none">
    مساعدك المتكامل لإدارة البيانات المدرسية، الطلاب، الرسوم، الجداول، والتقارير الإدارية الذكية.
  </p>

  <!-- Suggestion Buttons -->
  <div class="flex flex-col gap-2 w-full max-w-[340px]">
    {#each activeSuggestions as sug}
      <button
        type="button"
        onclick={() => onselectSuggestion(sug)}
        class="w-full text-start py-2.5 px-3.5 text-xs font-medium text-foreground/80 bg-card border border-border/80 hover:border-primary/50 hover:bg-muted/60 hover:text-foreground rounded-xl cursor-pointer transition-all active:scale-[0.99] leading-relaxed shadow-2xs"
      >
        {sug}
      </button>
    {/each}
  </div>
</div>
