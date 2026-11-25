<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { ArrowUturnLeft, Sparkles } from "@orbis/icons";
import { Button } from "@orbis/ui/button";
import type { SearchItemGroup } from "./search-modal.types";

let {
  groups,
  selectedItemId,
  isLoading,
  error,
  showAskOrbis,
  onAskOrbis,
  onSelect,
}: {
  groups: SearchItemGroup[];
  selectedItemId: string | null;
  isLoading: boolean;
  error: string;
  showAskOrbis: boolean;
  onAskOrbis: () => void;
  onSelect: (item: SearchItemGroup["items"][number]) => void;
} = $props();
</script>

<div class="min-h-0 flex-1 overflow-y-auto overscroll-contain p-2" aria-busy={isLoading}>
  {#if error && groups.length > 0}
    <p role="status" class="px-3 py-2 text-xs text-destructive">{error}</p>
  {/if}
  {#if isLoading && groups.length > 0}
    <p role="status" class="px-3 py-2 text-xs text-muted-foreground">
      {m.common_search_loading()}
    </p>
  {/if}

  {#if groups.length === 0}
    {#if isLoading}
      <div role="status" class="py-10 text-center text-sm text-muted-foreground">
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
      <div class="flex flex-col items-center gap-3 py-6 text-center select-none">
        <p class="text-sm text-muted-foreground">
          {m.common_search_no_results()}
        </p>
        {#if showAskOrbis}
          <Button
            variant="outline"
            size="sm"
            onclick={onAskOrbis}
            class="gap-1.5 border-primary/20 bg-primary/5 font-semibold text-primary shadow-sm transition-colors hover:bg-primary/10 hover:text-primary"
            aria-label={m.common_search_ask_orbis_hint()}
          >
            <Sparkles class="size-3.5" />
            <span>{m.shared_ui_ask_orbis()}</span>
          </Button>
        {/if}
      </div>
    {/if}
  {:else}
    {#each groups as group (group.label)}
      <section class="mb-4 last:mb-0">
        <h2
          class="px-3 py-1.5 text-xs font-semibold text-muted-foreground text-start"
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
              class="group flex min-h-12 w-full items-center justify-between gap-3 rounded-md px-3 py-2 text-start text-sm transition-colors focus-visible:outline-2 focus-visible:outline-ring
                {item.id === selectedItemId
                ? 'bg-accent text-accent-foreground font-medium'
                : 'text-foreground hover:bg-accent/40 hover:text-accent-foreground'}"
            >
              <span class="flex min-w-0 items-center gap-3">
                <Icon
                  class="size-4 text-muted-foreground group-hover:text-accent-foreground shrink-0"
                />
                <span class="flex min-w-0 flex-col gap-0.5">
                  <span class="truncate" dir="auto">{item.title}</span>
                  {#if item.subtitle}
                    <span class="truncate text-xs font-normal text-muted-foreground" dir="auto">
                      {item.subtitle}
                    </span>
                  {/if}
                </span>
              </span>
              <ArrowUturnLeft
                class="size-3 shrink-0 text-muted-foreground transition-opacity {item.id === selectedItemId ? 'opacity-100' : 'opacity-0 group-hover:opacity-100 group-focus-visible:opacity-100'}"
              />
            </button>
          {/each}
        </div>
      </section>
    {/each}
  {/if}
</div>
