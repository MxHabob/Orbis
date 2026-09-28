<!--
  @component AccountTree
  Ultra-compact, professional C# / .NET WinForms & WPF style TreeView.
  Renders the authentic hierarchical Chart of Accounts directly from the backend (Root nodes -> Child nodes).
  Features:
  - Dense, compact 22px item rows with classic hierarchical tree guide lines
  - Authentic [+] / [-] collapsible node toggles and crisp folder icons
  - Account code, account name, subtype badge, and balance
  - Expand/Collapse All and fast search filtering
-->
<script lang="ts">
import { getLocale } from "$lib/paraglide/runtime";
import { formatCurrency } from "$lib/core/i18n";
import { Input } from "@orbis/ui/input";
import { Button } from "@orbis/ui/button";
import {
  Folder,
  FolderOpen,
  Document,
  Search,
  Expand,
  Collapse,
} from "@orbis/icons";

export interface AccountNode {
  id: string;
  code: string;
  name: string;
  name_ar?: string;
  name_en?: string;
  type?: string;
  account_type?: string;
  sub_type?: string;
  item_type?: string;
  currency?: string;
  balances?: Record<string, number>;
  is_active?: number | boolean;
  children?: AccountNode[];
}

let {
  accounts = [],
  orgId = "",
  onSelectAccount,
}: {
  accounts?: AccountNode[];
  orgId?: string;
  onSelectAccount?: (account: AccountNode) => void;
} = $props();

// ── State ──────────────────────────────────────────────────────────────
let searchQuery = $state("");
let expandedNodeIds = $state<Set<string>>(new Set());
let selectedAccountId = $state<string | null>(null);

// Initialize root nodes as expanded by default
$effect(() => {
  if (accounts.length > 0 && expandedNodeIds.size === 0) {
    expandedNodeIds = new Set(accounts.map((n) => n.id));
  }
});

const rtl = $derived(getLocale() === "ar");

function isExpanded(id: string): boolean {
  return expandedNodeIds.has(id);
}

function toggleExpand(id: string, event?: MouseEvent) {
  if (event) event.stopPropagation();
  const next = new Set(expandedNodeIds);
  if (next.has(id)) {
    next.delete(id);
  } else {
    next.add(id);
  }
  expandedNodeIds = next;
}

function expandAll() {
  const allIds = new Set<string>();
  function collect(nodes: AccountNode[]) {
    for (const node of nodes) {
      allIds.add(node.id);
      if (node.children && node.children.length > 0) {
        collect(node.children);
      }
    }
  }
  collect(accounts);
  expandedNodeIds = allIds;
}

function collapseAll() {
  expandedNodeIds = new Set();
}

function filterTree(nodes: AccountNode[], query: string): AccountNode[] {
  if (!query.trim()) return nodes;
  const q = query.trim().toLowerCase();

  const res: AccountNode[] = [];
  for (const node of nodes) {
    const nameMatch = (node.name_ar || node.name || "").toLowerCase().includes(q);
    const codeMatch = (node.code || "").toLowerCase().includes(q);
    const matchingChildren = node.children ? filterTree(node.children, query) : [];

    if (nameMatch || codeMatch || matchingChildren.length > 0) {
      res.push({
        ...node,
        children: matchingChildren.length > 0 ? matchingChildren : node.children,
      });
    }
  }
  return res;
}

const filteredTree = $derived.by(() => {
  if (!searchQuery.trim()) return accounts;
  return filterTree(accounts, searchQuery);
});

$effect(() => {
  if (searchQuery.trim().length > 0) {
    expandAll();
  }
});

function handleSelect(node: AccountNode) {
  selectedAccountId = node.id;
  if (onSelectAccount) {
    onSelectAccount(node);
  }
}

function getBalance(node: AccountNode): number {
  if (!node.balances) return 0;
  const curr = node.currency || "YER";
  return Number(node.balances[curr] || node.balances[curr.toUpperCase()] || 0);
}

function getSubtype(node: AccountNode): string | null {
  const type = String(node.sub_type || node.item_type || "").toLowerCase();
  const map: Record<string, string> = {
    cash: "صندوق",
    bank: "بنك",
    supplier: "مورد",
    customer: "عميل",
    employee: "موظف",
  };
  return map[type] || null;
}
</script>

<div class="flex h-full min-h-0 w-full flex-col overflow-hidden font-sans select-none text-start bg-card/60">
  
  <!-- Mini .NET Style Toolbar -->
  <div class="flex shrink-0 items-center justify-between gap-2 px-2.5 py-1.5 border-b border-border/50 bg-muted/20 text-xs">
    <div class="relative flex-1 max-w-xs">
      <Search size={12} class="absolute inset-s-2 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none" />
      <Input
        type="text"
        placeholder="بحث سريع في الشجرة بالاسم أو الرقم..."
        bind:value={searchQuery}
        class="h-6 ps-7 text-[11px] bg-background border-border/60 py-0"
      />
    </div>

    <div class="flex items-center gap-1">
      <Button
        variant="ghost"
        size="sm"
        class="h-6 text-[11px] px-1.5 gap-1 text-muted-foreground hover:text-foreground"
        onclick={expandAll}
      >
        <Expand size={12} />
        <span>توسيع</span>
      </Button>

      <Button
        variant="ghost"
        size="sm"
        class="h-6 text-[11px] px-1.5 gap-1 text-muted-foreground hover:text-foreground"
        onclick={collapseAll}
      >
        <Collapse size={12} />
        <span>طي</span>
      </Button>
    </div>
  </div>

  <!-- Tree Viewport with Classic Connecting Hierarchy Lines -->
  <div class="flex-1 min-h-0 overflow-auto p-2 font-sans text-xs">
    {#if filteredTree.length === 0}
      <div class="py-6 text-center text-[11px] text-muted-foreground">
        لا توجد حسابات مطابقة
      </div>
    {:else}
      {#snippet renderNodes(nodes: AccountNode[], depth: number)}
        {#each nodes as node (node.id)}
          {@const hasChildren = node.children && node.children.length > 0}
          {@const expanded = isExpanded(node.id)}
          {@const isSelected = selectedAccountId === node.id}
          {@const bal = getBalance(node)}
          {@const subtype = getSubtype(node)}
          {@const name = node.name_ar || node.name || "حساب"}

          <div class="flex flex-col">
            <!-- Tree Item Row (Classic 22px Compact) -->
            <div
              class="flex items-center justify-between gap-1.5 px-1.5 py-0.5 rounded cursor-pointer text-[11px] transition-colors {isSelected ? 'bg-primary text-primary-foreground font-bold shadow-xs' : 'hover:bg-muted/40 text-foreground'}"
              onclick={() => handleSelect(node)}
            >
              <div class="flex items-center gap-1.5 min-w-0 flex-1">
                <!-- Expand Toggle [+] / [-] or Leaf Indicator -->
                {#if hasChildren}
                  <button
                    type="button"
                    class="inline-flex items-center justify-center w-3.5 h-3.5 border border-border {isSelected ? 'bg-primary-foreground/20 text-primary-foreground' : 'bg-background text-muted-foreground'} rounded-xs text-[10px] font-mono leading-none shrink-0 cursor-pointer"
                    onclick={(e) => toggleExpand(node.id, e)}
                  >
                    {expanded ? "−" : "+"}
                  </button>
                  {#if expanded}
                    <FolderOpen size={13} class={isSelected ? "text-primary-foreground" : "text-amber-500 shrink-0"} />
                  {:else}
                    <Folder size={13} class={isSelected ? "text-primary-foreground" : "text-amber-500 shrink-0"} />
                  {/if}
                {:else}
                  <span class="w-3.5 shrink-0 text-center text-muted-foreground/40 text-[9px]">─</span>
                  <Document size={12} class={isSelected ? "text-primary-foreground" : "text-muted-foreground shrink-0"} />
                {/if}

                <!-- Account Code -->
                <span class="font-mono text-[10px] px-1 py-0.2 rounded {isSelected ? 'bg-primary-foreground/20 text-primary-foreground' : 'bg-muted/70 text-muted-foreground font-semibold'} shrink-0">
                  {node.code}
                </span>

                <!-- Account Name -->
                <span class="truncate {isSelected ? 'text-primary-foreground font-bold' : 'text-foreground font-medium'}">
                  {name}
                </span>

                {#if subtype}
                  <span class="text-[9px] px-1 py-0.2 rounded border {isSelected ? 'border-primary-foreground/40 text-primary-foreground' : 'border-border text-muted-foreground'} font-normal shrink-0">
                    {subtype}
                  </span>
                {/if}
              </div>

              <!-- Balance Display -->
              {#if bal !== 0}
                <div class="shrink-0 font-mono text-[10px] {isSelected ? 'text-primary-foreground' : 'text-emerald-600 dark:text-emerald-400'} font-bold">
                  {formatCurrency(bal, node.currency || "YER")}
                </div>
              {/if}
            </div>

            <!-- Recursive Sub-tree with connecting line -->
            {#if hasChildren && expanded}
              <div class="border-s border-border/60 ms-2 ps-1">
                {@render renderNodes(node.children || [], depth + 1)}
              </div>
            {/if}
          </div>
        {/each}
      {/snippet}

      {@render renderNodes(filteredTree, 0)}
    {/if}
  </div>
</div>
