<!--
@component ComboboxDropdown
A generic searchable dropdown (combobox) using Svelte 5 runes and bits-ui primitives.
-->
<script lang="ts">
import { Popover, PopoverTrigger, PopoverContent } from "../popover/index.js";
import { Input } from "../input/index.js";
import { Button } from "../button/index.js";
import { cn } from "../../../utils.js";
import { Check, ChevronsUpDown, Search } from "@lucide/svelte";

interface ComboboxItem {
  id: string;
  label: string;
  value: string;
}

let {
  placeholder = "Select item...",
  searchPlaceholder = "Search...",
  selectedItem = $bindable(undefined),
  items = [],
  class: className,
  onSelect,
}: {
  placeholder?: string;
  searchPlaceholder?: string;
  selectedItem?: ComboboxItem;
  items: ComboboxItem[];
  class?: string;
  onSelect?: (item: ComboboxItem) => void;
} = $props();

let isOpen = $state(false);
let searchQuery = $state("");

const filteredItems = $derived(
  searchQuery.trim() === ""
    ? items
    : items.filter(
        (item) =>
          item.label.toLowerCase().includes(searchQuery.toLowerCase()) ||
          item.value.toLowerCase().includes(searchQuery.toLowerCase()),
      ),
);

function handleSelect(item: ComboboxItem) {
  selectedItem = item;
  isOpen = false;
  searchQuery = "";
  onSelect?.(item);
}
</script>

<Popover bind:open={isOpen}>
  <PopoverTrigger>
    {#snippet child({ props })}
      <Button
        {...props}
        variant="outline"
        role="combobox"
        aria-expanded={isOpen}
        class={cn("w-full justify-between text-xs h-9 font-normal px-3 cursor-pointer", className, props.class as string)}
      >
        <span class="truncate">
          {selectedItem ? selectedItem.label : placeholder}
        </span>
        <ChevronsUpDown class="ms-2 h-4 w-4 shrink-0 opacity-50" />
      </Button>
    {/snippet}
  </PopoverTrigger>
  
  <PopoverContent class="w-[250px] p-0" align="start">
    <div class="flex flex-col max-h-[300px]" style="font-family: var(--font-sans);">
      <!-- Search Input Header -->
      <div class="flex items-center border-b border-border px-3 py-2 shrink-0 gap-2">
        <Search class="h-3.5 w-3.5 text-muted-foreground opacity-60" />
        <Input
          type="text"
          placeholder={searchPlaceholder}
          bind:value={searchQuery}
          class="h-7 w-full !border-0 !bg-transparent !p-0 !shadow-none !focus:ring-0 !focus-visible:ring-0 !focus-visible:ring-offset-0 text-xs"
          autocomplete="off"
        />
      </div>

      <!-- Items List -->
      <div class="overflow-y-auto py-1 flex-1 min-h-[50px] scrollbar-thin">
        {#if filteredItems.length === 0}
          <div class="px-3 py-4 text-center text-xs text-muted-foreground">
            No items found.
          </div>
        {:else}
          {#each filteredItems as item}
            <button
              type="button"
              onclick={() => handleSelect(item)}
              class={cn(
                "w-full flex items-center justify-between px-3 py-2 text-start text-xs hover:bg-muted/60 transition-colors cursor-pointer",
                selectedItem?.value === item.value && "bg-muted font-medium"
              )}
            >
              <span class="truncate">{item.label}</span>
              {#if selectedItem?.value === item.value}
                <Check class="h-3.5 w-3.5 text-primary shrink-0 ms-2" />
              {/if}
            </button>
          {/each}
        {/if}
      </div>
    </div>
  </PopoverContent>
</Popover>
