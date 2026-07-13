<script lang="ts">
import { Card } from "@orbis/ui-svelte";
import { Landmark, Wallet } from "lucide-svelte";
import { formatCurrency } from "$lib/core/i18n";
import { localizeHref } from "$lib/paraglide/runtime";
import * as finance from "$lib/paraglide/messages/finance";

let {
  treasuries,
  orgId,
}: {
  treasuries: any[];
  orgId: string;
} = $props();
</script>

<Card class="p-5 bg-card border border-border/60 flex flex-col justify-between text-start relative overflow-hidden shadow-sm min-h-[315px] h-full transition-all duration-200 card-tactile">
  <div class="flex items-center justify-between w-full select-none text-[9px] font-bold text-muted-foreground/80 mb-3.5">
    <div class="flex items-center gap-1.5 font-sans font-extrabold uppercase tracking-wider text-[8px] text-muted-foreground/75">
      <span class="h-1.5 w-1.5 rounded-full bg-brand-lime animate-pulse"></span>
      {finance.overview_treasuries_list ? finance.overview_treasuries_list() : "أرصدة الخزائن والبنوك"}
    </div>
  </div>

  <div class="flex flex-col gap-2 mt-2 w-full flex-grow">
    {#if treasuries.length === 0}
      <div class="py-8 text-center text-muted-foreground text-xs select-none">
        {finance.overview_no_treasuries()}
      </div>
    {:else}
      {#each treasuries.slice(0, 3) as t (t.id)}
        <div class="flex items-center justify-between p-2 rounded-xl border border-border/40 bg-card/30 hover:bg-card/50 transition-colors">
          <div class="flex items-center gap-2 text-start min-w-0">
            <div class="h-6 w-6 rounded-lg bg-brand-lime/10 border border-brand-lime/20 flex items-center justify-center text-brand-lime shrink-0 select-none">
              <Landmark size={12} />
            </div>
            <div class="flex flex-col min-w-0">
              <span class="font-bold text-foreground truncate max-w-[130px] text-[10px]">{t.name_ar || t.name}</span>
              <span class="text-[8px] text-muted-foreground font-mono select-none">
                {t.item_type === "Bank" ? "حساب بنكي" : "خزينة نقدية"}
              </span>
            </div>
          </div>
          
          <div class="flex flex-col items-end shrink-0 text-end">
            <span class="font-bold font-mono text-foreground text-[10px]">{formatCurrency(t.balance || t.current_balance || 0, t.currency || "YER")}</span>
          </div>
        </div>
      {/each}
    {/if}
  </div>

  <!-- Sentry-style link list inside the body -->
  <div class="flex flex-col gap-2 pt-2 border-t border-border/40 w-full mt-2 text-[10px]">
    <a href={localizeHref(`/${orgId}/finance/treasuries`)} class="flex items-center gap-2 hover:underline text-primary font-bold">
      <Wallet size={12} />
      <span>{finance.overview_btn_manage_treasuries ? finance.overview_btn_manage_treasuries() : "إدارة الحسابات البنكية والخزائن"}</span>
    </a>
  </div>
</Card>
