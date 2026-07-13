<script lang="ts">
import { Button } from "@orbis/ui-svelte/button";
import { Avatar } from "@orbis/avatar-engine";
import * as m from "$lib/paraglide/messages";

let {
  suggestions = [],
  onselectSuggestion,
}: {
  suggestions: string[];
  onselectSuggestion: (suggestion: string) => void;
} = $props();
</script>

<div class="flex-1 flex flex-col items-center justify-center px-6 pb-8 select-none">
  <!-- Notion-style Agent Avatar -->
  <div class="mb-6 flex justify-center w-full">
    <Avatar hat="graduation" width="96px" height="96px" />
  </div>

  <!-- Subtitle -->
  <p class="text-[13px] text-foreground/90 text-center font-medium max-w-[280px] leading-relaxed mb-6 select-none">
    {m.agent_subtitle()}
  </p>

  <!-- Suggestion Buttons -->
  <div class="flex flex-col gap-2 w-full max-w-[340px]">
    {#each suggestions as sug}
      <Button
        variant="outline"
        onclick={() => onselectSuggestion(sug)}
        class="w-full text-center py-3 px-4 text-[11.5px] font-medium text-foreground/80 bg-card border border-border/80 hover:border-zinc-400 dark:hover:border-zinc-600 hover:bg-muted/40 hover:text-foreground rounded-lg cursor-pointer transition-all duration-150 active:scale-[0.99] leading-relaxed whitespace-normal h-auto wrap-break-word shadow-sm"
      >
        {sug}
      </Button>
    {/each}
  </div>
</div>
