<script lang="ts">
import type { JournalEntryDto } from "$lib/api";
import { formatCurrency } from "$lib/core/i18n";
import * as m from "$lib/paraglide/messages";

let {
  entry,
  accountName,
}: { entry?: JournalEntryDto; accountName: (accountId: string) => string } =
  $props();
</script>

{#if entry?.lines?.length}
  <dl class="divide-y divide-border border-y border-border">
    {#each entry.lines as line (line.id)}
      <div class="grid gap-2 py-3 sm:grid-cols-[minmax(0,1fr)_auto_auto] sm:items-center">
        <dt class="text-sm font-medium">
          {accountName(line.account_id)}
          {#if line.description}<span class="block text-xs font-normal text-muted-foreground">{line.description}</span>{/if}
        </dt>
        <dd class="text-xs text-muted-foreground">{m.finance_ui_debit_account()} {formatCurrency(Number(line.debit_amount), entry.currency_code)}</dd>
        <dd class="text-xs text-muted-foreground">{m.finance_ui_credit_account()} {formatCurrency(Number(line.credit_amount), entry.currency_code)}</dd>
      </div>
    {/each}
  </dl>
{/if}
