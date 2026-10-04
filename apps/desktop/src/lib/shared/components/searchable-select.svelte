<!--
  @component SearchableSelect
  A modern, responsive, reusable Select component with a sticky search bar at the top
  and an optional "Create new..." action button at the bottom.
  Follows Orbis Design System and logical RTL/LTR CSS.
-->
<script lang="ts" generics="T = any">
import * as m from "$lib/paraglide/messages";
import { l } from "$lib/core/i18n";
import { Plus, Search, XMark } from "@orbis/icons";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
} from "@orbis/ui/select";
import type { Snippet } from "svelte";

interface Props {
  value?: string | null;
  items?: T[];
  placeholder?: string;
  searchPlaceholder?: string;
  emptyText?: string;
  createLabel?: string;
  onCreate?: () => void;
  getItemValue?: (item: T) => string;
  getItemLabel?: (item: T) => string;
  getItemSubtitle?: (item: T) => string | undefined;
  getItemBadge?: (item: T) => string | undefined;
  showAllOption?: boolean;
  allOptionLabel?: string;
  disabled?: boolean;
  size?: "sm" | "default";
  class?: string;
  contentClass?: string;
  id?: string;
  onchange?: (val: string | null) => void;
  itemSnippet?: Snippet<[T]>;
}

let {
  value = $bindable<string | null>(null),
  items = [],
  placeholder = m.shared_ui_select_placeholder(),
  searchPlaceholder = m.shared_ui_search_placeholder(),
  emptyText = m.shared_ui_select_no_results(),
  createLabel,
  onCreate,
  getItemValue = (item: any) => String(item?.id ?? item?.value ?? item ?? ""),
  getItemLabel = (item: any) => {
    if (!item) return "";
    if (typeof item === "string") return item;
    const localized = l(
      item.name_ar || item.nameAr || item.label_ar || item.title_ar,
      item.name_en || item.nameEn || item.name || item.label || item.title || item.full_name,
    );
    if (localized) return localized;
    const composite = `${item.first_name || ""} ${item.last_name || ""}`.trim();
    if (composite) return composite;
    return String(item.full_name || item.name || item.label || item.code || item.id || "");
  },
  getItemSubtitle = (item: any) => item?.employee_number ? `#${item.employee_number}` : item?.code || item?.subtitle || undefined,
  getItemBadge = (item: any) => item?.badge || item?.status || undefined,
  showAllOption = false,
  allOptionLabel = m.shared_ui_all(),
  disabled = false,
  size = "default",
  class: className = "",
  contentClass = "",
  id = undefined,
  onchange = undefined,
  itemSnippet,
}: Props = $props();

let searchQuery = $state("");

const filteredItems = $derived.by(() => {
  if (!searchQuery.trim()) return items;
  const q = searchQuery.toLowerCase().trim();
  return items.filter((item: any) => {
    if (!item) return false;
    if (typeof item === "string") return item.toLowerCase().includes(q);
    const label = getItemLabel(item).toLowerCase();
    const sub = (getItemSubtitle(item) || "").toLowerCase();
    const val = getItemValue(item).toLowerCase();
    const nameAr = String(item.name_ar || item.nameAr || "").toLowerCase();
    const nameEn = String(item.name_en || item.nameEn || item.name || "").toLowerCase();
    const empNum = String(item.employee_number || "").toLowerCase();
    return (
      label.includes(q) ||
      sub.includes(q) ||
      val.includes(q) ||
      nameAr.includes(q) ||
      nameEn.includes(q) ||
      empNum.includes(q)
    );
  });
});

const selectedItem = $derived(
  items.find((item: any) => getItemValue(item) === String(value || "")),
);

const displayLabel = $derived.by(() => {
  if (value === "all" || value === "" || value === null || value === undefined) {
    return showAllOption ? allOptionLabel : placeholder;
  }
  if (selectedItem) {
    return getItemLabel(selectedItem);
  }
  return String(value);
});

function handleValueChange(newVal: string | undefined) {
  const val = newVal ?? null;
  value = val;
  searchQuery = "";
  if (onchange) {
    onchange(val);
  }
}

function handleCreateClick(e: MouseEvent) {
  e.stopPropagation();
  if (onCreate) {
    onCreate();
  }
}
</script>

<Select
  type="single"
  value={value ?? ""}
  onValueChange={handleValueChange}
  {disabled}
>
  <SelectTrigger
    {id}
    class="{size === 'sm' ? 'h-8 text-xs' : 'h-9 text-xs'} w-full bg-background text-start flex items-center justify-between gap-2 overflow-hidden {className}"
  >
    <span class="truncate font-normal flex-1 text-start {(!value || value === 'all') && !selectedItem ? 'text-muted-foreground' : 'text-foreground'}">
      {displayLabel}
    </span>
  </SelectTrigger>

  <SelectContent
    class="text-xs font-sans max-h-72 w-(--bits-select-anchor-width,100%) min-w-(--bits-select-anchor-width,200px) max-w-(--bits-select-anchor-width,100%) p-0 shadow-xl rounded-xl border border-border bg-popover text-popover-foreground z-50 overflow-hidden {contentClass}"
    align="start"
  >
    <!-- Sticky Search Header -->
    <div class="p-1.5 border-b border-border/50 sticky top-0 bg-popover/95 backdrop-blur-xs z-10">
      <div class="relative flex items-center">
        <Search size={13} class="absolute inset-s-2.5 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none" />
        <Input
          type="text"
          placeholder={searchPlaceholder}
          class="h-7 ps-7 pe-6 text-[11px] bg-muted/40 border-border/60 focus-visible:ring-1 focus-visible:ring-primary/40 w-full"
          bind:value={searchQuery}
        />
        {#if searchQuery}
          <button
            type="button"
            class="absolute inset-e-2 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground cursor-pointer p-0.5"
            onclick={() => (searchQuery = "")}
          >
            <XMark size={11} />
          </button>
        {/if}
      </div>
    </div>

    <!-- Scrollable Items List -->
    <div class="max-h-48 overflow-y-auto p-1 scrollbar-thin">
      {#if showAllOption}
        <SelectItem
          value="all"
          class="text-xs text-start justify-start cursor-pointer rounded-lg px-2.5 py-2 font-medium hover:bg-muted/60 focus:bg-muted/60"
        >
          {allOptionLabel}
        </SelectItem>
      {/if}

      {#each filteredItems as item (getItemValue(item))}
        {@const itemVal = getItemValue(item)}
        {@const itemLab = getItemLabel(item)}
        {@const itemSub = getItemSubtitle(item)}
        {@const itemBadge = getItemBadge(item)}
        <SelectItem
          value={itemVal}
          class="text-xs text-start justify-start cursor-pointer rounded-lg px-2.5 py-2 hover:bg-muted/60 focus:bg-muted/60"
        >
          {#if itemSnippet}
            {@render itemSnippet(item)}
          {:else}
            <div class="flex items-center justify-between w-full gap-3 min-w-0">
              <div class="flex flex-col min-w-0 flex-1 text-start">
                <span class="truncate font-medium text-foreground text-xs leading-snug">{itemLab}</span>
                {#if itemSub}
                  <span class="text-[10px] font-mono text-muted-foreground/80 truncate leading-tight mt-0.5">
                    {itemSub}
                  </span>
                {/if}
              </div>
              {#if itemBadge}
                <span class="text-[9px] px-1.5 py-0.5 rounded font-mono font-medium bg-muted text-muted-foreground shrink-0 self-center">
                  {itemBadge}
                </span>
              {/if}
            </div>
          {/if}
        </SelectItem>
      {/each}

      {#if filteredItems.length === 0}
        <div class="p-3 text-center text-xs text-muted-foreground italic">
          {emptyText}
        </div>
      {/if}
    </div>

    <!-- Sticky Bottom Create / Add Button -->
    {#if onCreate}
      <div class="p-1 border-t border-border/50 sticky bottom-0 bg-popover/95 backdrop-blur-xs z-10">
        <Button
          type="button"
          variant="ghost"
          size="xs"
          class="w-full text-[11px] h-7 text-primary hover:text-primary hover:bg-primary/10 justify-start gap-1.5 cursor-pointer font-medium rounded-lg"
          onclick={handleCreateClick}
        >
          <Plus size={12} />
          <span>{createLabel || "إضافة جديد..."}</span>
        </Button>
      </div>
    {/if}
  </SelectContent>
</Select>
