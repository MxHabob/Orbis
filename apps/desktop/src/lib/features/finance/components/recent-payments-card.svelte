<script lang="ts">
import { resolve } from "$app/paths";
import type { Pathname } from "$app/types";
import * as m from "$lib/paraglide/messages";
import {
  Card,
  CardHeader,
  CardTitle,
  CardAction,
  CardContent,
} from "@orbis/ui/card";
import { Button } from "@orbis/ui/button";
import { CreditCard } from "@orbis/icons";
import { formatCurrency } from "$lib/core/i18n";
import { localizeHref } from "$lib/paraglide/runtime";
import type { PaymentDto } from "$lib/api";

type RecentPayment = PaymentDto & {
  studentName: string;
  receiptNumber: string;
};

let {
  recentPayments,
  orgId,
}: {
  recentPayments: RecentPayment[];
  orgId: string;
} = $props();

function getPaymentMethodLabel(method: string): string {
  const normalized = method.toLowerCase().replace(/[\s-]/g, "_");
  if (normalized === "cash") return m.finance_pos_cash();
  if (normalized === "check") return m.finance_payments_check();
  if (
    normalized === "bank" ||
    normalized === "bank_transfer" ||
    normalized === "transfer"
  ) {
    return m.finance_payment_method_transfer();
  }
  return method;
}
</script>

<Card class="lg:col-span-2 flex flex-col justify-between text-start shadow-sm min-h-[315px] h-full">
  <CardHeader class="py-3 px-4">
    <div class="flex items-center gap-2">
      <span class="h-2 w-2 rounded-full bg-emerald-500 animate-pulse"></span>
      <CardTitle class="text-xs font-bold uppercase tracking-wider text-foreground">
        {m.finance_overview_recent_payments()}
      </CardTitle>
    </div>

    <CardAction>
      <div class="flex items-center gap-1.5">
        <a href={resolve(localizeHref(`/${orgId}/finance/payments`) as Pathname)}>
          <Button variant="outline" size="xs" class="h-6 text-[9px] font-bold cursor-pointer">
            {m.finance_overview_btn_new_payment()}
          </Button>
        </a>
        <a href={resolve(localizeHref(`/${orgId}/finance/payments`) as Pathname)}>
          <Button variant="outline" size="xs" class="h-6 text-[9px] font-bold cursor-pointer">
            {m.common_overview_view_all()}
          </Button>
        </a>
      </div>
    </CardAction>
  </CardHeader>

  <CardContent class="p-4 flex-grow flex flex-col justify-center">
    <div class="grid grid-cols-1 md:grid-cols-2 gap-2.5">
      {#if recentPayments.length === 0}
        <div class="col-span-2 py-8 text-center text-muted-foreground text-xs select-none">
          {m.finance_overview_no_payments()}
        </div>
      {:else}
        {#each recentPayments as p (p.id)}
          <div class="flex items-center justify-between p-2.5 rounded-xl border border-border/40 bg-card hover:bg-muted/20 transition-colors">
            <div class="flex items-center gap-2 text-start min-w-0">
              <div class="h-6 w-6 rounded-lg bg-emerald-500/10 border border-emerald-500/20 flex items-center justify-center text-emerald-500 shrink-0 select-none">
                <CreditCard size={12} />
              </div>
              <div class="flex flex-col min-w-0">
                <span class="font-bold text-foreground truncate max-w-[150px] text-[10px]">{p.studentName}</span>
                <span class="text-[8px] text-muted-foreground font-mono flex items-center gap-1.5 select-none">
                  <span>{p.receiptNumber}</span>
                  <span>•</span>
                  <span>{getPaymentMethodLabel(p.method)}</span>
                </span>
              </div>
            </div>
            
            <div class="flex flex-col items-end shrink-0 text-end">
              <span class="font-bold font-mono text-emerald-500 text-[10px]">+{formatCurrency(Number(p.amount), p.currency_code || "YER")}</span>
              <span class="text-[7px] text-muted-foreground select-none font-mono mt-0.5">
                {p.paid_at ? p.paid_at.split("T")[0] : p.created_at ? p.created_at.split("T")[0] : ""}
              </span>
            </div>
          </div>
        {/each}
      {/if}
    </div>
  </CardContent>
</Card>
