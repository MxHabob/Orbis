<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { getLocale } from "$lib/paraglide/runtime";
import {
  ChevronDown,
  ChevronRight,
  CheckSquare,
  FileText,
  Folder,
  FolderOpen,
  Square,
} from "@lucide/svelte";
import { slide } from "svelte/transition";

let {
  categories = [],
  selectedIds = $bindable(new Set<string>()),
}: {
  categories?: any[];
  selectedIds?: Set<string>;
} = $props();

let expandedCategories = $state<Record<string, boolean>>({ asset: true });
let expandedAccounts = $state<Record<string, boolean>>({});
let isTreeExpanded = $state(true);

const selectableAccounts = $derived(
  categories.flatMap((category) => flatten(category.accounts || [])),
);

function flatten(nodes: any[]): any[] {
  return nodes.flatMap((node) => [node, ...flatten(node.children || [])]);
}

function descendants(node: any): string[] {
  return [node.id, ...flatten(node.children || []).map((child) => child.id)];
}

function checkboxState(node: any): "checked" | "partial" | "unchecked" {
  const ids = descendants(node);
  const count = ids.filter((id) => selectedIds.has(id)).length;
  return count === ids.length ? "checked" : count > 0 ? "partial" : "unchecked";
}

function toggleIds(ids: string[]) {
  const next = new Set(selectedIds);
  const shouldSelect = ids.some((id) => !next.has(id));
  ids.forEach((id) => (shouldSelect ? next.add(id) : next.delete(id)));
  selectedIds = next;
}

function toggleCategory(category: any) {
  const ids = flatten(category.accounts || []).map((account) => account.id);
  toggleIds(ids);
}

function toggleAll() {
  const ids = selectableAccounts.map((account) => account.id);
  toggleIds(ids);
}
</script>

<section class="flex h-46 min-h-0 flex-col overflow-hidden rounded-lg border border-border bg-background p-2 shadow-sm" aria-label={m.finance_ledger_chart_of_accounts()}>
  <div class="flex shrink-0 items-center justify-between border-b border-border/60 pb-1.5">
    <button
      type="button"
      class="flex min-w-0 items-center gap-1 text-start text-[10px] font-semibold text-foreground hover:text-primary"
      onclick={() => isTreeExpanded = !isTreeExpanded}
    >
      {#if isTreeExpanded}
        <ChevronDown size={12} class="shrink-0 text-muted-foreground" />
        <FolderOpen size={12} class="shrink-0 text-primary" />
      {:else}
        <ChevronRight size={12} class="shrink-0 text-muted-foreground {getLocale() === 'ar' ? 'rotate-180' : ''}" />
        <Folder size={12} class="shrink-0 text-primary" />
      {/if}
      <span class="truncate">{m.finance_ledger_chart_of_accounts()}</span>
    </button>
    <button type="button" class="shrink-0 p-0.5 text-muted-foreground hover:text-primary" onclick={toggleAll} title={m.finance_ledger_chart_of_accounts()}>
      {#if selectedIds.size === selectableAccounts.length && selectableAccounts.length > 0}
        <CheckSquare size={12} class="text-primary" />
      {:else if selectedIds.size > 0}
        <span class="flex h-3 w-3 items-center justify-center rounded border border-primary bg-primary/20"><span class="h-1 w-1 rounded-sm bg-primary"></span></span>
      {:else}
        <Square size={12} />
      {/if}
    </button>
  </div>

  {#if isTreeExpanded}
    <div class="min-h-0 flex-1 overflow-y-auto pt-1" transition:slide={{ duration: 150 }}>
      {#snippet renderNodes(nodes: any[], depth: number)}
        {#each nodes as node (node.id)}
          {@const hasChildren = (node.children || []).length > 0}
          {@const isExpanded = expandedAccounts[node.id] !== false}
          {@const state = checkboxState(node)}
          <div class="flex flex-col" style="padding-inline-start: {depth * 10}px">
            <div class="flex min-h-6 items-center justify-between gap-1 rounded px-1 hover:bg-accent/15">
              <button type="button" class="flex min-w-0 flex-1 items-center gap-1 text-start text-[9px] text-foreground" onclick={() => hasChildren ? expandedAccounts = { ...expandedAccounts, [node.id]: !isExpanded } : toggleIds([node.id])}>
                {#if hasChildren}
                  {#if isExpanded}<ChevronDown size={10} class="shrink-0 text-muted-foreground" />{:else}<ChevronRight size={10} class="shrink-0 text-muted-foreground {getLocale() === 'ar' ? 'rotate-180' : ''}" />{/if}
                  <Folder size={10} class="shrink-0 text-primary/80" />
                {:else}
                  <FileText size={10} class="shrink-0 text-muted-foreground/70" />
                {/if}
                <span class="truncate">{node.name_ar || node.name}</span>
              </button>
              <div class="flex shrink-0 items-center gap-1">
                <span class="font-mono text-[8px] text-muted-foreground">{node.code}</span>
                <button type="button" class="p-0.5 text-muted-foreground hover:text-primary" onclick={() => toggleIds(descendants(node))}>
                  {#if state === "checked"}<CheckSquare size={10} class="text-primary" />{:else if state === "partial"}<span class="flex h-2.5 w-2.5 items-center justify-center rounded border border-primary bg-primary/20"><span class="h-0.5 w-0.5 bg-primary"></span></span>{:else}<Square size={10} />{/if}
                </button>
              </div>
            </div>
            {#if hasChildren && isExpanded}
              <div class="border-s border-border/40" transition:slide={{ duration: 120 }}>{@render renderNodes(node.children, depth + 1)}</div>
            {/if}
          </div>
        {/each}
      {/snippet}

      {#each categories as category (category.id)}
        {@const categoryAccounts = flatten(category.accounts || [])}
        {@const categoryState = categoryAccounts.length === 0 ? "unchecked" : checkboxState({ id: category.id, children: categoryAccounts })}
        <div class="mb-0.5">
          <div class="flex min-h-6 items-center justify-between gap-1 rounded px-1 hover:bg-accent/15">
            <button type="button" class="flex min-w-0 flex-1 items-center gap-1 text-start text-[9px] font-semibold text-foreground hover:text-primary" onclick={() => expandedCategories = { ...expandedCategories, [category.id]: !expandedCategories[category.id] }}>
              {#if expandedCategories[category.id]}<ChevronDown size={10} class="text-muted-foreground" />{:else}<ChevronRight size={10} class="text-muted-foreground {getLocale() === 'ar' ? 'rotate-180' : ''}" />{/if}
              <Folder size={10} class="text-primary/80" /><span class="truncate">{getLocale() === "ar" ? category.name_ar : category.name_en}</span>
            </button>
            <button type="button" class="p-0.5 text-muted-foreground hover:text-primary" onclick={() => toggleCategory(category)}>
              {#if categoryState === "checked"}<CheckSquare size={10} class="text-primary" />{:else if categoryState === "partial"}<span class="flex h-2.5 w-2.5 items-center justify-center rounded border border-primary bg-primary/20"><span class="h-0.5 w-0.5 bg-primary"></span></span>{:else}<Square size={10} />{/if}
            </button>
          </div>
          {#if expandedCategories[category.id]}<div class="border-s border-border/40">{@render renderNodes(category.accounts || [], 1)}</div>{/if}
        </div>
      {/each}
    </div>
  {/if}
</section>
