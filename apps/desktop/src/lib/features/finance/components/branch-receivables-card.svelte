<script lang="ts">
import * as m from "$lib/paraglide/messages";
import {
  Card,
  CardHeader,
  CardTitle,
  CardContent,
} from "@orbis/ui/card";
import { formatCurrency } from "$lib/core/i18n";

type BranchReceivable = {
  id: string;
  name: string;
  balance: number;
};

let {
  branches,
  selectedBranchId,
  currency,
  onSelect,
}: {
  branches: BranchReceivable[];
  selectedBranchId: string;
  currency: string;
  onSelect: (branchId: string) => void;
} = $props();

const maximumBalance = $derived(
  Math.max(0, ...branches.map((branch) => branch.balance)),
);
</script>

<Card class="flex h-full min-h-[315px] flex-col text-start shadow-sm">
  <CardHeader class="px-4 py-3">
    <CardTitle class="text-xs font-bold uppercase tracking-wider">
      {m.finance_overview_branch_receivables()}
    </CardTitle>
  </CardHeader>
  <CardContent class="flex flex-col justify-center gap-3 p-4">
    {#if branches.length === 0 || maximumBalance === 0}
      <p class="py-8 text-center text-xs text-muted-foreground">
        {m.finance_overview_no_late_students()}
      </p>
    {:else}
      {#each branches as branch (branch.id)}
        <button
          type="button"
          class="group w-full rounded-md text-start outline-none focus-visible:ring-2 focus-visible:ring-ring"
          aria-pressed={selectedBranchId === branch.id}
          onclick={() => onSelect(selectedBranchId === branch.id ? "all" : branch.id)}
        >
          <span class="mb-1.5 flex items-center justify-between gap-3">
            <span class="truncate text-xs font-medium text-foreground group-hover:text-primary">
              {branch.name}
            </span>
            <span class="shrink-0 font-mono text-[10px] font-semibold text-muted-foreground">
              {formatCurrency(branch.balance, currency)}
            </span>
          </span>
          <span class="block h-1.5 overflow-hidden rounded-full bg-muted">
            <span
              class="block h-full rounded-full bg-primary transition-[width]"
              style={`width: ${Math.max(3, (branch.balance / maximumBalance) * 100)}%`}
            ></span>
          </span>
        </button>
      {/each}
    {/if}
  </CardContent>
</Card>
