<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Check, ChevronDown, ChevronRight, Search } from "@orbis/icons";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Popover, PopoverContent, PopoverTrigger } from "@orbis/ui/popover";

export interface SearchableMultiFilterOption {
  value: string;
  label: string;
}

export interface SearchableMultiFilterGroup {
  id: string;
  label: string;
  options: SearchableMultiFilterOption[];
}

let {
  groups,
  selectedValues = $bindable<string[]>([]),
  placeholder,
  searchPlaceholder = m.shared_ui_search_placeholder(),
  emptyText = m.shared_ui_select_no_results(),
  clearLabel = m.common_filter_clear(),
}: {
  groups: SearchableMultiFilterGroup[];
  selectedValues?: string[];
  placeholder: string;
  searchPlaceholder?: string;
  emptyText?: string;
  clearLabel?: string;
} = $props();

let open = $state(false);
let searchQuery = $state("");
let collapsedGroups = $state<string[]>([]);

const filteredGroups = $derived.by(() => {
  const query = searchQuery.trim().toLocaleLowerCase();
  if (!query) return groups;

  return groups
    .map((group) => ({
      ...group,
      options: group.options.filter((option) =>
        `${option.label} ${option.value}`.toLocaleLowerCase().includes(query),
      ),
    }))
    .filter((group) => group.options.length > 0);
});

function isSelected(value: string) {
  return selectedValues.includes(value);
}

function toggle(value: string) {
  selectedValues = isSelected(value)
    ? selectedValues.filter((selected) => selected !== value)
    : [...selectedValues, value];
}

function toggleGroup(groupId: string) {
  collapsedGroups = collapsedGroups.includes(groupId)
    ? collapsedGroups.filter((id) => id !== groupId)
    : [...collapsedGroups, groupId];
}

function isGroupCollapsed(groupId: string) {
  return !searchQuery.trim() && collapsedGroups.includes(groupId);
}
</script>

<Popover bind:open>
  <PopoverTrigger>
    {#snippet child({ props })}
      <Button
        {...props}
        variant="outline"
        size="sm"
        role="combobox"
        aria-expanded={open}
        class="h-8 min-w-32 justify-between gap-2 px-2.5 text-xs font-normal"
      >
        <span class="truncate">{placeholder}</span>
        {#if selectedValues.length > 0}
          <span class="rounded bg-muted px-1.5 py-0.5 text-[10px] font-medium text-muted-foreground">
            {selectedValues.length}
          </span>
        {/if}
        <ChevronDown class="size-3.5 shrink-0 text-muted-foreground" />
      </Button>
    {/snippet}
  </PopoverTrigger>

  <PopoverContent
    align="end"
    class="w-[min(22rem,calc(100vw-2rem))] overflow-hidden rounded-lg p-0"
  >
    <div class="flex items-center gap-2 border-b border-border/60 px-3 py-2">
      <Search class="size-3.5 shrink-0 text-muted-foreground" />
      <Input
        bind:value={searchQuery}
        placeholder={searchPlaceholder}
        class="h-7 border-0 bg-transparent px-0 text-xs shadow-none focus-visible:ring-0"
        autocomplete="off"
      />
    </div>

    <div class="max-h-72 overflow-y-auto p-1.5">
      {#if filteredGroups.length === 0}
        <p class="px-3 py-5 text-center text-xs text-muted-foreground">
          {emptyText}
        </p>
      {:else}
        {#each filteredGroups as group (group.id)}
          {@const groupSelectedCount = group.options.filter((option) => isSelected(option.value)).length}
          <section class="py-1">
            <button
              type="button"
              aria-expanded={!isGroupCollapsed(group.id)}
              onclick={() => toggleGroup(group.id)}
              class="flex w-full items-center gap-2 rounded-md px-2.5 py-1.5 text-start text-[10px] font-medium text-muted-foreground hover:bg-muted/50"
            >
              {#if isGroupCollapsed(group.id)}
                <ChevronRight class="size-3.5 shrink-0" />
              {:else}
                <ChevronDown class="size-3.5 shrink-0" />
              {/if}
              <span class="flex-1">{group.label}</span>
              {#if groupSelectedCount > 0}
                <span class="rounded-full bg-muted px-1.5 py-0.5 text-[10px]">
                  {groupSelectedCount}
                </span>
              {/if}
            </button>
            {#if !isGroupCollapsed(group.id)}
              {#each group.options as option (option.value)}
                <button
                  type="button"
                  aria-pressed={isSelected(option.value)}
                  onclick={() => toggle(option.value)}
                  class="flex w-full items-center gap-2.5 rounded-md px-2.5 py-2 ps-7 text-start text-xs text-foreground transition-colors hover:bg-muted/70"
                >
                  <span
                    class="flex size-4 shrink-0 items-center justify-center rounded border border-input {isSelected(option.value) ? 'border-primary bg-primary text-primary-foreground' : 'text-transparent'}"
                  >
                    <Check class="size-3" />
                  </span>
                  <span class="truncate">{option.label}</span>
                </button>
              {/each}
            {/if}
          </section>
        {/each}
      {/if}
    </div>

    {#if selectedValues.length > 0}
      <div class="border-t border-border/60 p-1.5">
        <Button
          variant="ghost"
          size="sm"
          class="h-8 w-full text-xs text-muted-foreground"
          onclick={() => (selectedValues = [])}
        >
          {clearLabel}
        </Button>
      </div>
    {/if}
  </PopoverContent>
</Popover>
