<script lang="ts">
import * as m from "$lib/paraglide/messages";
let {
  filteredSuggestions,
  selectedIndex,
  showSuggestions,
  suggestionsContainer = $bindable(null),
  onSelect,
  onHover,
}: {
  filteredSuggestions: any[];
  selectedIndex: number;
  showSuggestions: boolean;
  suggestionsContainer?: HTMLDivElement | null;
  onSelect: (item: any) => void;
  onHover: (idx: number) => void;
} = $props();

$effect(() => {
  if (showSuggestions && suggestionsContainer) {
    const activeEl = suggestionsContainer.querySelector(
      'button[aria-pressed="true"]',
    );
    if (activeEl) {
      activeEl.scrollIntoView({ block: "nearest", behavior: "smooth" });
    }
  }
});
</script>

{#if showSuggestions && filteredSuggestions.length > 0}
  <div
    bind:this={suggestionsContainer}
    class="absolute bottom-full left-3.5 right-3.5 z-50 mb-1.5 flex max-h-56 flex-col overflow-y-auto rounded-md border border-border bg-card py-1 shadow-md animate-in fade-in slide-in-from-bottom-2 duration-150"
  >
    {#each filteredSuggestions as item, idx}
      <button
        type="button"
        aria-pressed={idx === selectedIndex}
        class="flex w-full cursor-pointer flex-col gap-0.5 px-3.5 py-2 text-start transition-colors select-none {idx === selectedIndex ? 'bg-accent text-accent-foreground' : 'text-foreground hover:bg-muted/50'}"
        onclick={() => onSelect(item)}
        onmouseenter={() => onHover(idx)}
      >
        <span class="min-w-0 text-xs font-semibold [overflow-wrap:anywhere]">{item.label}</span>
        <span class="line-clamp-2 text-[10px] font-normal text-muted-foreground/80 [overflow-wrap:anywhere]">{item.subtitle}</span>
      </button>
    {/each}
  </div>
{/if}
