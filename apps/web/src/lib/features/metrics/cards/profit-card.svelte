<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Skeleton } from "@orbis/ui";
import { ArrowRight, ChartBar } from "@orbis/icons";
import { createAccountsQuery } from "$lib/features/finance/queries";
import { summarizeLedger } from "$lib/features/finance/components/reports/ledger-report";
import { formatCurrency } from "$lib/core/i18n";
import { localizeHref } from "$lib/paraglide/runtime";
import { orgContext } from "$lib/stores/org-context.svelte";
import DragIndicator from "../components/drag-indicator.svelte";

let { orgId, isEditing = false }: { orgId: string; isEditing?: boolean } = $props();

const accountsQuery = createAccountsQuery(() => orgId);
const currency = $derived(orgContext.baseCurrency);
const netIncome = $derived(summarizeLedger(accountsQuery.data ?? [], currency).netIncome);
</script>

<div class="card-tactile flex h-full min-h-[265px] flex-col justify-between border border-border/60 bg-card p-5 shadow-sm">
  {#if accountsQuery.isLoading}
    <Skeleton class="h-full min-h-32 w-full" />
  {:else}
    <div>
      <div class="flex items-center justify-between gap-3">
        <h2 class="flex items-center gap-2 text-sm font-medium">
          <ChartBar size={16} aria-hidden="true" />
          {m.finance_metrics_cumulative_profit()}
        </h2>
        {#if isEditing}<DragIndicator />{/if}
      </div>
      {#if accountsQuery.isError}
        <p class="mt-3 text-sm text-destructive">{m.common_loading_error()}</p>
      {:else if currency}
        <p class="mt-4 font-mono text-2xl font-semibold">{formatCurrency(netIncome, currency)}</p>
      {/if}
    </div>

    <a
      href={localizeHref(`/${orgId}/finance/reports/income-statement`)}
      class="mt-4 inline-flex items-center gap-1 self-start text-xs font-medium text-muted-foreground hover:text-foreground hover:underline"
    >
      {m.reports_profit_detailed_graph()}
      <ArrowRight size={12} class="rtl:rotate-180" aria-hidden="true" />
    </a>
  {/if}
</div>
