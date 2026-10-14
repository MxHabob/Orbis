<script lang="ts">
  import * as m from "$lib/paraglide/messages";
  import { ArrowUturnLeft } from "@orbis/icons";
  import type { SearchItemGroup } from "./search-modal.types";

  let {
    groups,
    selectedItemId,
    isLoading,
    error,
    onSelect,
  }: {
    groups: SearchItemGroup[];
    selectedItemId: string | null;
    isLoading: boolean;
    error: string;
    onSelect: (item: SearchItemGroup["items"][number]) => void;
  } = $props();
</script>

<div class="flex-1 overflow-y-auto p-2 scrollbar-hide">
  {#if error}
    <p role="status" class="px-3 py-2 text-xs text-destructive">{error}</p>
  {/if}
  {#if isLoading}
    <p role="status" class="px-3 py-2 text-xs text-muted-foreground">
      {m.common_search_loading()}
    </p>
  {/if}

  {#if groups.length === 0}
    {#if isLoading}
      <div class="py-6 text-center text-sm text-muted-foreground select-none">
        {m.common_search_loading()}
      </div>
    {:else if error}
      <div
        role="status"
        class="mx-2 my-4 rounded-lg border border-warning/30 bg-warning/5 px-4 py-3 text-center text-sm text-muted-foreground"
      >
        {m.common_search_no_results_partial()}
      </div>
    {:else}
      <div class="py-6 text-center text-sm text-muted-foreground select-none">
        {m.common_search_no_results()}
      </div>
    {/if}
  {:else}
    {#each groups as group (group.label)}
      <section class="mb-4 last:mb-0">
        <h2
          class="px-3 py-1 text-[10px] font-bold text-muted-foreground tracking-wider uppercase select-none text-start"
        >
          {group.label}
        </h2>
        <div class="mt-1 flex flex-col gap-0.5">
          {#each group.items as item (item.id)}
            {@const Icon = item.icon}
            <button
              id={`search-result-${item.id}`}
              type="button"
              aria-current={item.id === selectedItemId ? "true" : undefined}
              onclick={() => onSelect(item)}
              class="w-full text-start flex items-center justify-between px-3 py-2 text-sm rounded-md transition-colors group
                {item.id === selectedItemId
                ? 'bg-accent text-accent-foreground font-medium'
                : 'text-foreground hover:bg-accent/40 hover:text-accent-foreground'}"
            >
              <span class="flex items-center gap-2.5 truncate">
                <Icon
                  class="size-4 text-muted-foreground group-hover:text-accent-foreground shrink-0"
                />
                <span class="truncate">{item.title}</span>
                {#if item.subtitle}
                  <span
                    class="text-xs text-muted-foreground font-mono truncate"
                  >
                    {item.subtitle}
                  </span>
                {/if}
              </span>
              <ArrowUturnLeft
                class="size-3 text-muted-foreground opacity-0 group-hover:opacity-100 transition-opacity"
              />
            </button>
          {/each}
        </div>
      </section>
    {/each}
  {/if}
</div>
