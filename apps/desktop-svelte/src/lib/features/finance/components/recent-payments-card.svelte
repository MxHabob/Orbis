<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Card } from "@orbis/ui-svelte";
import { Button } from "@orbis/ui-svelte/button";
import { ArrowUpRight } from "lucide-svelte";
import { formatCurrency } from "$lib/core/i18n";
import { localizeHref } from "$lib/paraglide/runtime";

let {
  recentPayments,
  orgId,
}: {
  recentPayments: any[];
  orgId: string;
} = $props();
</script>

<Card class="lg:col-span-2 p-5 bg-card border border-border/60 flex flex-col justify-between text-start shadow-sm min-h-[315px] h-full transition-all duration-200 card-tactile">
  <div class="flex items-center justify-between w-full select-none text-[9px] font-bold text-muted-foreground/80 mb-3.5">
    <div class="flex items-center gap-1.5 font-sans font-extrabold uppercase tracking-wider text-[8px] text-muted-foreground/75">
      <span class="h-1.5 w-1.5 rounded-full bg-emerald-500 animate-pulse"></span>
      {m.finance_overview_recent_payments ? m.finance_overview_recent_payments() : "آخر المدفوعات المستلمة"}
    </div>
    <div class="flex items-center gap-2">
      <a href={localizeHref(`/${orgId}/finance/payments`)}>
        <Button variant="outline" size="xs" class="h-6 text-[9px] font-bold cursor-pointer">
          {m.finance_overview_btn_new_payment ? m.finance_overview_btn_new_payment() : "سند جديد"}
        </Button>
      </a>
      <a href={localizeHref(`/${orgId}/finance/payments`)}>
        <Button variant="outline" size="xs" class="h-6 text-[9px] font-bold cursor-pointer">
          {m.common_overview_view_all ? m.common_overview_view_all() : "عرض الكل"}
        </Button>
      </a>
    </div>
  </div>

  <div class="grid grid-cols-1 md:grid-cols-2 gap-2.5 mt-2 flex-grow">
    {#if recentPayments.length === 0}
      <div class="col-span-2 py-8 text-center text-muted-foreground text-xs select-none">
        {m.finance_overview_no_payments()}
      </div>
    {:else}
      {#each recentPayments as p (p.id)}
        <div class="flex items-center justify-between p-2 rounded-xl border border-border/40 bg-card/30 hover:bg-card/50 transition-colors">
          <div class="flex items-center gap-2 text-start min-w-0">
            <div class="h-6 w-6 rounded-lg bg-emerald-500/10 border border-emerald-500/20 flex items-center justify-center text-emerald-500 shrink-0 select-none">
              <ArrowUpRight size={12} />
            </div>
            <div class="flex flex-col min-w-0">
              <span class="font-bold text-foreground truncate max-w-[150px] text-[10px]">{p.studentName}</span>
              <span class="text-[8px] text-muted-foreground font-mono flex items-center gap-1.5 select-none">
                <span>{p.receiptNumber}</span>
                <span>•</span>
                <span>{p.payment_method === "Cash" ? "نقداً" : p.payment_method === "Check" ? "شيك" : "حوالة"}</span>
              </span>
            </div>
          </div>
          
          <div class="flex flex-col items-end shrink-0 text-end">
            <span class="font-bold font-mono text-emerald-500 text-[10px]">+{formatCurrency(p.amount, p.currency || "YER")}</span>
            <span class="text-[7px] text-muted-foreground select-none font-mono mt-0.5">
              {p.payment_date ? p.payment_date.split("T")[0] : p.created_at ? p.created_at.split("T")[0] : ""}
            </span>
          </div>
        </div>
      {/each}
    {/if}
  </div>
</Card>
