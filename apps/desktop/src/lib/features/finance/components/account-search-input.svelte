<!--
  @component AccountSearchInput
  Live, direct-typing searchable account input component for the Financial System.
  Allows immediate inline typing of account code or name with live floating search results,
  keyboard navigation (Up/Down/Enter/Escape), and localized account type badges.
-->
<script lang="ts">
import { l } from "$lib/core/i18n";
import { api, type AccountDto } from "$lib/api";
import { onMount } from "svelte";
import { Input } from "@orbis/ui/input";
import { Badge } from "@orbis/ui/badge";
import { Check, Search, XMark } from "@orbis/icons";
import { slide } from "svelte/transition";

interface Props {
  value?: string | null;
  accounts?: AccountDto[];
  accountTypeFilter?: string[] | string;
  placeholder?: string;
  searchPlaceholder?: string;
  emptyText?: string;
  disabled?: boolean;
  size?: "sm" | "default";
  class?: string;
  id?: string;
  onselect?: (account: AccountDto) => void;
  onchange?: (accountId: string | null) => void;
}

let {
  value = $bindable<string | null>(null),
  accounts = undefined,
  accountTypeFilter = undefined,
  placeholder = "ابحث برمز أو اسم الحساب (مثال: 120101، الصندوق، البنك)...",
  emptyText = "لا يوجد حساب مطابق للبحث",
  disabled = false,
  size = "default",
  class: className = "",
  id = undefined,
  onselect = undefined,
  onchange = undefined,
}: Props = $props();

let fetchedAccounts = $state<AccountDto[]>([]);
let isLoading = $state(false);

onMount(async () => {
  if (accounts === undefined) {
    isLoading = true;
    try {
      const list = await api.listAccounts();
      if (Array.isArray(list)) {
        fetchedAccounts = list;
      }
    } catch (e) {
      console.warn("Failed to fetch accounts in AccountSearchInput:", e);
    } finally {
      isLoading = false;
    }
  }
});

const allAccounts = $derived(accounts ?? fetchedAccounts);

// Apply optional account type filter
const filteredByTypeAccounts = $derived.by(() => {
  if (!accountTypeFilter) return allAccounts;
  const types = Array.isArray(accountTypeFilter)
    ? accountTypeFilter.map((t) => t.toLowerCase())
    : [accountTypeFilter.toLowerCase()];
  return allAccounts.filter((a) => {
    const type = (a.account_type || (a as any).type || (a as any).sub_type || "").toLowerCase();
    return types.includes(type);
  });
});

let isDropdownOpen = $state(false);
let searchInputText = $state("");
let highlightedIndex = $state(0);
let inputElement = $state<HTMLInputElement | null>(null);
let containerElement = $state<HTMLDivElement | null>(null);

const selectedAccount = $derived(
  allAccounts.find((a) => String(a.id) === String(value || "") || String(a.code) === String(value || ""))
);

const selectedAccountLabel = $derived.by(() => {
  if (!selectedAccount) return "";
  const code = selectedAccount.code ? `${selectedAccount.code} - ` : "";
  const name = l(selectedAccount.name_ar, selectedAccount.name) || selectedAccount.name || "";
  return `${code}${name}`.trim();
});

// Real-time search query matching
const filteredResults = $derived.by(() => {
  const query = searchInputText.trim().toLowerCase();
  if (!query) {
    return filteredByTypeAccounts.slice(0, 50);
  }
  return filteredByTypeAccounts.filter((acc) => {
    const code = String(acc.code || "").toLowerCase();
    const name = String(acc.name || "").toLowerCase();
    const nameAr = String(acc.name_ar || "").toLowerCase();
    const type = String(acc.account_type || (acc as any).type || "").toLowerCase();
    return code.includes(query) || name.includes(query) || nameAr.includes(query) || type.includes(query);
  }).slice(0, 50);
});

const accountTypeLabels: Record<string, { ar: string; en: string; variant: "default" | "secondary" | "outline" | "lime" | "destructive" }> = {
  asset: { ar: "أصول", en: "Asset", variant: "lime" },
  liability: { ar: "التزامات", en: "Liability", variant: "destructive" },
  equity: { ar: "حقوق ملكية", en: "Equity", variant: "secondary" },
  revenue: { ar: "إيرادات", en: "Revenue", variant: "default" },
  expense: { ar: "مصروفات", en: "Expense", variant: "outline" },
  cash: { ar: "نقدية / صندوق", en: "Cash", variant: "lime" },
  bank: { ar: "حساب بنكي", en: "Bank", variant: "secondary" },
};

function getAccountTypeBadge(type?: string | null) {
  if (!type) return null;
  const key = type.toLowerCase();
  return accountTypeLabels[key] || { ar: type, en: type, variant: "outline" as const };
}

function handleFocus() {
  if (disabled) return;
  searchInputText = selectedAccountLabel;
  isDropdownOpen = true;
  highlightedIndex = 0;
}

function handleInput(e: Event) {
  const target = e.target as HTMLInputElement;
  searchInputText = target.value;
  isDropdownOpen = true;
  highlightedIndex = 0;
  if (!target.value.trim() && value) {
    value = null;
    onchange?.(null);
  }
}

function handleSelectAccount(acc: AccountDto) {
  value = String(acc.id);
  searchInputText = "";
  isDropdownOpen = false;
  onselect?.(acc);
  onchange?.(String(acc.id));
}

function handleClear(e: MouseEvent) {
  e.stopPropagation();
  value = null;
  searchInputText = "";
  isDropdownOpen = false;
  onchange?.(null);
  inputElement?.focus();
}

function handleKeyDown(e: KeyboardEvent) {
  if (!isDropdownOpen) {
    if (e.key === "ArrowDown" || e.key === "Enter") {
      isDropdownOpen = true;
      e.preventDefault();
    }
    return;
  }

  if (e.key === "ArrowDown") {
    e.preventDefault();
    highlightedIndex = (highlightedIndex + 1) % Math.max(1, filteredResults.length);
  } else if (e.key === "ArrowUp") {
    e.preventDefault();
    highlightedIndex = (highlightedIndex - 1 + filteredResults.length) % Math.max(1, filteredResults.length);
  } else if (e.key === "Enter") {
    e.preventDefault();
    if (filteredResults.length > 0 && filteredResults[highlightedIndex]) {
      handleSelectAccount(filteredResults[highlightedIndex]);
    }
  } else if (e.key === "Escape") {
    e.preventDefault();
    isDropdownOpen = false;
  }
}

function handleClickOutside(e: MouseEvent) {
  if (containerElement && !containerElement.contains(e.target as Node)) {
    isDropdownOpen = false;
    searchInputText = "";
  }
}
</script>

<svelte:window onclick={handleClickOutside} />

<div class="relative w-full text-start" bind:this={containerElement}>
  <!-- Direct Search Input Field -->
  <div class="relative flex items-center w-full">
    <Search
      size={14}
      class="absolute inset-s-3 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none transition-colors {isDropdownOpen ? 'text-primary' : ''}"
    />
    <Input
      {id}
      bind:this={inputElement}
      type="text"
      autocomplete="off"
      spellcheck="false"
      {placeholder}
      {disabled}
      value={isDropdownOpen ? searchInputText : selectedAccountLabel}
      oninput={handleInput}
      onfocus={handleFocus}
      onkeydown={handleKeyDown}
      class="{size === 'sm' ? 'h-8 text-xs ps-8.5 pe-7' : 'h-9 text-xs ps-9 pe-8'} bg-background border-border/80 focus-visible:ring-1 focus-visible:ring-primary/50 w-full font-medium transition-all {className}"
    />
    {#if (isDropdownOpen && searchInputText) || value}
      <button
        type="button"
        class="absolute inset-e-2.5 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground cursor-pointer p-0.5 rounded-full hover:bg-muted/80 transition-colors"
        onclick={handleClear}
        tabindex="-1"
        title="مسح الحساب"
      >
        <XMark size={13} />
      </button>
    {/if}
  </div>

  <!-- Live Dropdown Results List -->
  {#if isDropdownOpen}
    <div
      transition:slide={{ duration: 120 }}
      class="absolute start-0 end-0 top-full mt-1.5 z-50 rounded-xl border border-border bg-popover text-popover-foreground shadow-xl overflow-hidden backdrop-blur-md"
    >
      <div class="max-h-64 overflow-y-auto p-1.5 scrollbar-thin flex flex-col gap-1">
        {#each filteredResults as acc, idx (acc.id)}
          {@const isSelected = String(acc.id) === String(value || "")}
          {@const isHighlighted = idx === highlightedIndex}
          {@const badgeInfo = getAccountTypeBadge(acc.account_type || (acc as any).type)}
          <button
            type="button"
            class="w-full text-start px-3 py-2 rounded-lg text-xs flex items-center justify-between gap-3 transition-colors cursor-pointer {isHighlighted ? 'bg-accent/80 text-accent-foreground' : 'hover:bg-muted/60'} {isSelected ? 'bg-primary/10 font-semibold' : ''}"
            onclick={() => handleSelectAccount(acc)}
            onmouseenter={() => (highlightedIndex = idx)}
          >
            <div class="flex items-center gap-2.5 min-w-0 flex-1">
              <span class="font-mono font-bold text-[11px] px-1.5 py-0.5 rounded bg-muted/60 border border-border/50 text-foreground shrink-0">
                {acc.code || "—"}
              </span>
              <div class="flex flex-col min-w-0">
                <span class="truncate font-medium text-foreground text-xs">
                  {l(acc.name_ar, acc.name) || acc.name || acc.code}
                </span>
                {#if acc.name_ar && acc.name && acc.name_ar !== acc.name}
                  <span class="text-[10px] text-muted-foreground truncate leading-tight mt-0.5">
                    {acc.name}
                  </span>
                {/if}
              </div>
            </div>

            <div class="flex items-center gap-2 shrink-0">
              {#if badgeInfo}
                <Badge variant={badgeInfo.variant} class="text-[9px] py-0 px-1.5 font-normal">
                  {l(badgeInfo.ar, badgeInfo.en)}
                </Badge>
              {/if}
              {#if isSelected}
                <Check size={14} class="text-primary shrink-0" />
              {/if}
            </div>
          </button>
        {/each}

        {#if filteredResults.length === 0}
          <div class="py-6 px-3 text-center text-xs text-muted-foreground italic flex flex-col gap-1">
            <span>{emptyText}</span>
            <span class="text-[10px] opacity-70">جرّب البحث باسم آخر أو رمز محاسبي مختلف</span>
          </div>
        {/if}
      </div>
    </div>
  {/if}
</div>
