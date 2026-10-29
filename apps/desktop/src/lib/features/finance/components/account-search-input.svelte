<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { l } from "$lib/core/i18n";
import { api, type AccountDto } from "$lib/api";
import { LoadingIndicator } from "@orbis/ui";
import { Input } from "@orbis/ui/input";
import { Check, Search, XMark } from "@orbis/icons";
import { onDestroy, onMount } from "svelte";
import { slide } from "svelte/transition";

interface Props {
  value?: string | null;
  accounts?: AccountDto[];
  accountTypeFilter?: string[] | string;
  includeGroupAccounts?: boolean;
  placeholder?: string;
  searchPlaceholder?: string;
  emptyText?: string;
  disabled?: boolean;
  size?: "sm" | "default";
  class?: string;
  id?: string;
  onselect?: (account: AccountDto) => void;
  onchange?: (accountId: string | null) => void;
  onkeydown?: (event: KeyboardEvent) => void;
}

let {
  value = $bindable<string | null>(null),
  accounts = undefined,
  accountTypeFilter = undefined,
  includeGroupAccounts = false,
  placeholder = m.finance_ui_account_search_placeholder(),
  searchPlaceholder = m.finance_ui_account_search_placeholder(),
  emptyText = m.finance_ui_account_search_empty(),
  disabled = false,
  size = "default",
  class: className = "",
  id = undefined,
  onselect = undefined,
  onchange = undefined,
  onkeydown = undefined,
}: Props = $props();

const generatedId = $props.id();
const inputId = $derived(id ?? generatedId);
const listboxId = $derived(`${inputId}-account-options`);
let fetchedAccounts = $state<AccountDto[]>([]);
let isLoading = $state(false);
let isOpen = $state(false);
let searchText = $state("");
let debouncedQuery = $state("");
let highlightedIndex = $state(0);
let inputElement = $state<HTMLInputElement | null>(null);
let containerElement = $state<HTMLDivElement | null>(null);
let optionElements = $state<HTMLButtonElement[]>([]);
let debounceTimer: ReturnType<typeof setTimeout> | undefined;

onMount(async () => {
  if (accounts !== undefined) return;

  isLoading = true;
  try {
    fetchedAccounts = await api.listAccounts();
  } catch (error) {
    console.error("Failed to fetch accounts in AccountSearchInput:", error);
  } finally {
    isLoading = false;
  }
});

onDestroy(() => {
  if (debounceTimer) clearTimeout(debounceTimer);
});

const allAccounts = $derived(accounts ?? fetchedAccounts);
const parentAccountIds = $derived(
  new Set(
    allAccounts.flatMap((account) =>
      account.parent_id ? [account.parent_id] : [],
    ),
  ),
);
const filteredByTypeAccounts = $derived.by(() => {
  if (!accountTypeFilter) return allAccounts;
  const types = Array.isArray(accountTypeFilter)
    ? accountTypeFilter.map((type) => type.toLowerCase())
    : [accountTypeFilter.toLowerCase()];

  return allAccounts.filter((account) => {
    const accountTypes = [account.account_type, account.sub_type]
      .filter((type): type is string => Boolean(type))
      .map((type) => type.toLowerCase());
    return accountTypes.some((type) => types.includes(type));
  });
});
const selectableAccounts = $derived(
  filteredByTypeAccounts.filter(
    (account) =>
      account.is_active &&
      (includeGroupAccounts || !parentAccountIds.has(account.id)),
  ),
);
const selectedAccount = $derived(
  allAccounts.find(
    (account) =>
      String(account.id) === String(value || "") ||
      String(account.code) === String(value || ""),
  ),
);
const selectedAccountLabel = $derived(
  selectedAccount
    ? `${selectedAccount.code ? `${selectedAccount.code} - ` : ""}${getAccountName(selectedAccount)}`
    : "",
);
const searchableAccounts = $derived(
  selectableAccounts.map((account) => ({
    account,
    searchText: [
      account.code,
      account.name,
      account.name_ar,
      account.account_type,
      account.sub_type,
    ]
      .filter(Boolean)
      .join(" ")
      .toLowerCase(),
  })),
);
const filteredResults = $derived.by(() => {
  const query = debouncedQuery.trim().toLowerCase();
  if (!query) return [];
  return searchableAccounts
    .filter((item) => item.searchText.includes(query))
    .slice(0, 40)
    .map((item) => item.account);
});
const isSearching = $derived(searchText.trim() !== debouncedQuery);

function getAccountName(account: AccountDto) {
  return (
    l(account.name_ar, account.name) ||
    account.name ||
    account.code ||
    String(account.id)
  );
}

function clearPendingSearch() {
  if (debounceTimer) {
    clearTimeout(debounceTimer);
    debounceTimer = undefined;
  }
}

function handleInput(event: Event) {
  const target = event.currentTarget as HTMLInputElement;
  if (value) {
    value = null;
    onchange?.(null);
  }
  searchText = target.value;
  isOpen = Boolean(searchText.trim());
  highlightedIndex = 0;

  clearPendingSearch();
  const query = searchText.trim();
  if (!query) {
    debouncedQuery = "";
    return;
  }

  debounceTimer = setTimeout(() => {
    debouncedQuery = query;
    debounceTimer = undefined;
  }, 180);
}

function handleFocus() {
  if (value) requestAnimationFrame(() => inputElement?.select?.());
}

function handleSelectAccount(account: AccountDto) {
  value = String(account.id);
  searchText = "";
  debouncedQuery = "";
  isOpen = false;
  onselect?.(account);
  onchange?.(String(account.id));
}

function handleClear(event: MouseEvent) {
  event.stopPropagation();
  value = null;
  searchText = "";
  debouncedQuery = "";
  isOpen = false;
  clearPendingSearch();
  onchange?.(null);
  inputElement?.focus?.();
}

function scrollHighlightedOptionIntoView() {
  requestAnimationFrame(() => {
    optionElements[highlightedIndex]?.scrollIntoView({ block: "nearest" });
  });
}

function handleKeyDown(event: KeyboardEvent) {
  if (!(event.key === "Enter" && event.shiftKey)) {
    if (isOpen && filteredResults.length > 0 && event.key === "ArrowDown") {
      event.preventDefault();
      highlightedIndex = (highlightedIndex + 1) % filteredResults.length;
      scrollHighlightedOptionIntoView();
    } else if (
      isOpen &&
      filteredResults.length > 0 &&
      event.key === "ArrowUp"
    ) {
      event.preventDefault();
      highlightedIndex =
        (highlightedIndex - 1 + filteredResults.length) %
        filteredResults.length;
      scrollHighlightedOptionIntoView();
    } else if (isOpen && event.key === "Enter") {
      event.preventDefault();
      const account = filteredResults[highlightedIndex];
      if (account) handleSelectAccount(account);
    } else if (isOpen && event.key === "Escape") {
      event.preventDefault();
      isOpen = false;
      searchText = "";
      debouncedQuery = "";
      clearPendingSearch();
    }
  }

  onkeydown?.(event);
}

function handleClickOutside(event: MouseEvent) {
  const target = event.target;
  if (target instanceof Node && !containerElement?.contains(target)) {
    isOpen = false;
    searchText = "";
    debouncedQuery = "";
    clearPendingSearch();
  }
}
</script>

<svelte:window onclick={handleClickOutside} />

<div bind:this={containerElement} class="relative w-full min-w-0 text-start">
  <div class="relative flex w-full min-w-0 items-center">
    <Search
      size={14}
      class="pointer-events-none absolute inset-s-3 top-1/2 -translate-y-1/2 text-muted-foreground"
    />
    <Input
      id={inputId}
      bind:ref={inputElement}
      type="text"
      autocomplete="off"
      spellcheck="false"
      placeholder={searchPlaceholder || placeholder}
      {disabled}
      role="combobox"
      aria-expanded={isOpen}
      aria-autocomplete="list"
      aria-controls={listboxId}
      aria-activedescendant={isOpen && filteredResults[highlightedIndex]
        ? `${inputId}-option-${filteredResults[highlightedIndex].id}`
        : undefined}
      value={isOpen ? searchText : selectedAccountLabel}
      oninput={handleInput}
      onfocus={handleFocus}
      onkeydown={handleKeyDown}
      class="{size === 'sm' ? 'h-8 ps-8 pe-8 text-xs' : 'h-9 ps-9 pe-9 text-sm'} w-full bg-background font-normal focus-visible:border-ring focus-visible:ring-ring/50 {className}"
    />
    {#if (isOpen && searchText) || value}
      <button
        type="button"
        class="absolute inset-e-2 top-1/2 -translate-y-1/2 rounded-sm p-1 text-muted-foreground transition-colors hover:bg-accent hover:text-accent-foreground"
        onclick={handleClear}
        tabindex="-1"
        aria-label={m.finance_ui_account_clear()}
        title={m.finance_ui_account_clear()}
      >
        <XMark size={13} />
      </button>
    {/if}
  </div>

  {#if isOpen}
    <div
      transition:slide={{ duration: 100 }}
      id={listboxId}
      role="listbox"
      class="absolute inset-x-0 top-full z-50 mt-1 max-h-[min(18rem,40vh)] w-full min-w-0 overflow-y-auto overscroll-contain rounded-md border border-border bg-popover p-1 text-popover-foreground shadow-lg"
    >
      {#if isSearching || isLoading}
        <div class="flex justify-center px-3 py-3 text-muted-foreground">
          <LoadingIndicator size={14} />
        </div>
      {:else if filteredResults.length === 0}
        <div class="px-3 py-4 text-center text-sm text-muted-foreground">{emptyText}</div>
      {:else}
        {#each filteredResults as account, index (account.id)}
          {@const selected = String(account.id) === String(value || "")}
          {@const highlighted = index === highlightedIndex}
          <button
            id="{inputId}-option-{account.id}"
            bind:this={optionElements[index]}
            type="button"
            role="option"
            aria-selected={selected}
            class="flex w-full min-w-0 items-center justify-between gap-3 rounded px-2.5 py-2 text-start text-sm transition-colors hover:bg-accent hover:text-accent-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {highlighted ? 'bg-accent text-accent-foreground' : ''}"
            onclick={() => handleSelectAccount(account)}
            onmouseenter={() => (highlightedIndex = index)}
          >
            <span class="min-w-0 flex-1 truncate font-medium">
              {getAccountName(account)}
            </span>
            <span class="flex shrink-0 items-center gap-2">
              <span class="font-mono text-xs text-muted-foreground">{account.code || "—"}</span>
              {#if selected}
                <Check size={14} class="text-primary" />
              {/if}
            </span>
          </button>
        {/each}
      {/if}
    </div>
  {/if}
</div>
