<script lang="ts">
import { Avatar, type AvatarController } from "@orbis/avatar-engine";
import { ArrowLeft, ArrowRight } from "@orbis/icons";
import { getLocale } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";

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

const defaultSuggestions = $derived([
  {
    title: m.ai_copilot_suggestion_student_stats_title(),
    desc: m.ai_copilot_suggestion_student_stats_description(),
  },
  {
    title: m.ai_copilot_suggestion_financial_title(),
    desc: m.ai_copilot_suggestion_financial_description(),
  },
  {
    title: m.ai_copilot_suggestion_classes_title(),
    desc: m.ai_copilot_suggestion_classes_description(),
  },
  {
    title: m.ai_copilot_suggestion_report_title(),
    desc: m.ai_copilot_suggestion_report_description(),
  },
]);
</script>

<div class="flex-1 flex flex-col items-center justify-center px-4 py-6 select-none overflow-y-auto">
  <!-- Glowing Hero Avatar Capsule from @orbis/avatar-engine -->
  <div class="mb-3.5 flex items-center justify-center">
      <Avatar controller={avatarController} hat="pilot" width="98px" height="98px" />
  </div>

  <p class="text-[11.5px] text-muted-foreground text-center max-w-[310px] leading-relaxed mb-5 select-none">
    {m.ai_copilot_empty_state_description()}
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
