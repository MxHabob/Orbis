<script lang="ts">
import * as m from "$lib/paraglide/messages";
import {
  Card,
  CardHeader,
  CardTitle,
  CardContent,
  CardFooter,
} from "@orbis/ui/card";
import { ArrowRight, Clock } from "@orbis/icons";
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

<Card class="flex flex-col justify-between text-start relative overflow-hidden shadow-sm min-h-[315px] h-full">
  <!-- Header -->
  <CardHeader class="py-3 px-4">
    <div class="flex items-center gap-2">
      <Clock size={13} class="text-muted-foreground" />
      <CardTitle class="text-xs font-bold uppercase tracking-wider text-foreground">
        {m.finance_overview_performance_title()}
      </CardTitle>
    </div>
  </CardHeader>

  <!-- Body -->
  <CardContent class="p-5 flex flex-col text-start justify-center flex-grow select-none">
    <p class="text-xs text-muted-foreground leading-relaxed">
      {m.finance_collection_rate_description({
        totalFees: formatCurrency(totalFees, "YER"),
        totalPaid: formatCurrency(totalPaid, "YER"),
      })}
    </p>
    <div class="text-4xl font-black text-foreground font-sans mt-5 tracking-tight">
      {collectionRate}%
    </div>
  </CardContent>

  <!-- Footer -->
  <CardFooter class="border-t border-border/40 py-2.5 px-4 text-[10px]">
    <a href={localizeHref(`/${orgId}/finance/reports`)} class="flex items-center gap-1.5 hover:underline text-primary font-bold group/link">
      <span>{m.finance_overview_btn_statement()}</span>
      <ArrowRight size={10} class="transition-transform group-hover/link:translate-x-0.5 rtl:rotate-180" />
    </a>
  </CardFooter>
</Card>
