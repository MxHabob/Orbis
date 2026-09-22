<script lang="ts">
import { AdjustmentsHorizontal, ArrowUturnLeft } from "@orbis/icons";
import * as m from "$lib/paraglide/messages";

let {
  groupedItems,
  selectedItem,
  triggerAction,
}: {
  groupedItems: Record<string, any[]>;
  selectedItem: any;
  triggerAction: (item: any) => void;
} = $props();
</script>

<div class="flex-1 overflow-y-auto p-2 scrollbar-hide bg-popover">
  {#if Object.keys(groupedItems).length === 0}
    <div class="py-6 text-center text-sm text-muted-foreground select-none">
      {m.common_no_data ? m.common_no_data() : "لا توجد نتائج مطابقة"}
    </div>
  {:else}
    {#each Object.entries(groupedItems) as [groupName, groupItems]}
      <div class="mb-4 last:mb-0">
        <div class="px-3 py-1 text-[10px] font-bold text-muted-foreground tracking-wider uppercase select-none text-start">
          {groupName}
        </div>
        <div class="mt-1 flex flex-col gap-0.5">
          {#each groupItems as item}
            {@const Icon = item.icon}
            <button
              type="button"
              onclick={() => triggerAction(item)}
              class="w-full text-start flex items-center justify-between px-3 py-2 text-sm rounded-md transition-colors group
                {item === selectedItem ? 'bg-accent text-accent-foreground font-medium' : 'text-foreground hover:bg-accent/40 hover:text-accent-foreground'}"
            >
              <div class="flex items-center gap-2.5 truncate">
                <Icon class="size-4 text-muted-foreground group-hover:text-accent-foreground shrink-0" />
                <span class="truncate">{item.title}</span>
                {#if item.subtitle}
                  <span class="text-xs text-muted-foreground font-mono truncate">{item.subtitle}</span>
                {/if}
              </div>
              <div class="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
                <ArrowUturnLeft class="size-3 text-muted-foreground group-hover:text-accent-foreground" />
              </div>
            </button>
          {/each}
        </div>
      </div>
    {/each}
  {/if}
</div>
