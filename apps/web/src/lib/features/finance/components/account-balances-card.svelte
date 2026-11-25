<script lang="ts">
import * as m from "$lib/paraglide/messages";
import {
  Card,
  CardHeader,
  CardTitle,
  CardContent,
  CardFooter,
} from "@orbis/ui/card";
import { BuildingLibrary, Document } from "@orbis/icons";
import { formatCurrency } from "$lib/core/i18n";
import { localizeHref } from "$lib/paraglide/runtime";

let {
  treasuries,
  orgId,
}: {
  treasuries: any[];
  orgId: string;
} = $props();
</script>

<Card class="flex flex-col justify-between text-start relative overflow-hidden shadow-sm min-h-[315px] h-full">
  <CardHeader class="py-3 px-4">
    <div class="flex items-center gap-2">
      <span class="h-2 w-2 rounded-full bg-primary animate-pulse"></span>
      <CardTitle class="text-xs font-bold uppercase tracking-wider text-foreground">
        {m.finance_overview_cash_accounts_list()}
      </CardTitle>
    </div>
  </CardHeader>

  <CardContent class="p-4 flex flex-col gap-2 w-full grow justify-center">
    {#if treasuries.length === 0}
      <div class="py-8 text-center text-muted-foreground text-xs select-none">
        {m.finance_overview_no_cash_accounts()}
      </div>
    {:else}
      {#each treasuries.slice(0, 3) as t (t.id)}
        <div class="flex items-center justify-between p-2.5 rounded-xl border border-border/40 bg-card hover:bg-muted/20 transition-colors">
          <div class="flex items-center gap-2 text-start min-w-0">
            <div class="h-6 w-6 rounded-lg bg-primary/10 border border-primary/20 flex items-center justify-center text-primary shrink-0 select-none">
              <BuildingLibrary size={12} />
            </div>
            <div class="flex flex-col min-w-0">
              <span class="font-bold text-foreground truncate max-w-[130px] text-[10px]">{t.name_ar || t.name}</span>
              <span class="text-[8px] text-muted-foreground font-mono select-none">
                {t.sub_type === "bank" ? m.finance_ui_account_type_bank() : m.finance_ui_cash_treasury()}
              </span>
            </div>
          </div>
          
          <div class="flex flex-col items-end shrink-0 text-end">
            <span class="font-bold font-mono text-foreground text-[10px]">{formatCurrency(t.balances?.[t.currency] || 0, t.currency || "YER")}</span>
          </div>
        </div>
      {/each}
    {/if}
  </CardContent>

  <!-- Sentry-style link list inside the footer -->
  <CardFooter class="border-t border-border/40 py-2.5 px-4 text-[10px]">
    <a href={localizeHref(`/${orgId}/finance/accounts`)} class="flex items-center gap-2 hover:underline text-primary font-bold">
      <Document size={12} />
      <span>{m.finance_overview_btn_manage_cash_accounts()}</span>
    </a>
  </CardFooter>
</Card>
