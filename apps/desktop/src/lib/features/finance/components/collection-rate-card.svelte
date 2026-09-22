<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Card } from "@orbis/ui";
import { AdjustmentsHorizontal, ArrowRight, Clock } from "@orbis/icons";
import { formatCurrency } from "$lib/core/i18n";
import { localizeHref } from "$lib/paraglide/runtime";

let {
  collectionRate,
  totalFees,
  totalPaid,
  orgId,
}: {
  collectionRate: number;
  totalFees: number;
  totalPaid: number;
  orgId: string;
} = $props();
</script>

<Card class="p-6 bg-card border border-border/60 flex flex-col justify-between text-start relative overflow-hidden shadow-sm min-h-[315px] transition-all duration-200 card-tactile">
  <!-- Header -->
  <div class="flex items-center justify-between w-full select-none text-[9px] font-bold text-muted-foreground/80 mb-3">
    <div class="flex items-center gap-1.5 font-sans font-extrabold uppercase tracking-wider text-[8px] text-muted-foreground/75">
      <Clock size={11} class="text-muted-foreground/80" />
      <span>{m.finance_overview_performance_title()}</span>
    </div>
  </div>

  <!-- Body -->
  <div class="flex flex-col text-start justify-center flex-grow py-4 select-none">
    <p class="text-xs text-muted-foreground leading-relaxed">
      {m.finance_collection_rate_description({
        totalFees: formatCurrency(totalFees, "YER"),
        totalPaid: formatCurrency(totalPaid, "YER"),
      })}
    </p>
    <div class="text-4xl font-black text-foreground font-sans mt-5 tracking-tight">
      {collectionRate}%
    </div>
  </div>

  <!-- Footer -->
  <div class="flex flex-col gap-2 pt-2 border-t border-border/40 w-full mt-2 text-[10px]">
    <a href={localizeHref(`/${orgId}/finance/reports`)} class="flex items-center gap-1.5 hover:underline text-primary font-bold group/link">
      <span>{m.finance_overview_btn_statement()}</span>
      <ArrowRight size={10} class="transition-transform group-hover/link:translate-x-0.5 rtl:rotate-180" />
    </a>
  </div>
</Card>
