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
      '[aria-selected="true"]',
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
    class="absolute bottom-full mb-1.5 left-3.5 right-3.5 z-50 bg-popover border border-border/80 rounded-xl shadow-2xl flex flex-col max-h-56 overflow-y-auto scrollbar-hide py-1 animate-in fade-in slide-in-from-bottom-2 duration-150"
  >
    {#each filteredSuggestions as item, idx}
      <button
        type="button"
        aria-selected={idx === selectedIndex}
        class="w-full text-start px-3.5 py-2 flex flex-col gap-0.5 transition-colors cursor-pointer select-none {idx === selectedIndex ? 'bg-accent text-accent-foreground' : 'text-foreground hover:bg-muted/50'}"
        onclick={() => onSelect(item)}
        onmouseenter={() => onHover(idx)}
      >
        <span class="text-[11.5px] font-semibold">{item.label}</span>
        <span class="text-[9px] text-muted-foreground/80 font-normal truncate">{item.subtitle}</span>
      </button>
    {/each}
  </div>
{/if}
