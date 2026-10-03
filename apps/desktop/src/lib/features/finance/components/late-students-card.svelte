<script lang="ts">
import * as m from "$lib/paraglide/messages";
import {
  Card,
  CardHeader,
  CardTitle,
  CardAction,
  CardContent,
} from "@orbis/ui/card";
import { Button } from "@orbis/ui/button";
import { Avatar, AvatarFallback } from "@orbis/ui/avatar";
import { formatCurrency } from "$lib/core/i18n";
import { localizeHref } from "$lib/paraglide/runtime";

let {
  lateStudents,
  orgId,
}: {
  lateStudents: any[];
  orgId: string;
} = $props();
</script>

<Card class="lg:col-span-2 flex flex-col justify-between text-start shadow-sm min-h-[315px] h-full">
  <CardHeader class="py-3 px-4">
    <div class="flex items-center gap-2">
      <span class="h-2 w-2 rounded-full bg-destructive animate-pulse"></span>
      <CardTitle class="text-xs font-bold uppercase tracking-wider text-foreground">
        {m.finance_overview_late_students()}
      </CardTitle>
    </div>

    <CardAction>
      <div class="flex items-center gap-1.5">
        <a href={localizeHref(`/${orgId}/finance/fees`)}>
          <Button variant="outline" size="xs" class="h-6 text-[9px] font-bold cursor-pointer">
            {m.common_fee_button_assign()}
          </Button>
        </a>
        <a href={localizeHref(`/${orgId}/finance/fees`)}>
          <Button variant="outline" size="xs" class="h-6 text-[9px] font-bold cursor-pointer">
            {m.common_overview_view_all()}
          </Button>
        </a>
      </div>
    </CardAction>
  </CardHeader>

  <CardContent class="p-4 flex flex-col gap-2 flex-grow justify-center">
    {#if lateStudents.length === 0}
      <div class="py-8 text-center text-muted-foreground text-xs select-none">
        {m.finance_overview_no_late_students()}
      </div>
    {:else}
      {#each lateStudents.slice(0, 3) as item (item.feeId)}
        <div class="flex items-center justify-between p-2.5 rounded-xl border border-border/40 bg-card hover:bg-muted/20 transition-all duration-200">
          <div class="flex items-center gap-2.5 text-start min-w-0">
            <Avatar class="h-7 w-7 border border-border/70 select-none">
              <AvatarFallback class="text-[9px] font-bold bg-primary/5 text-primary">
                {item.studentName.slice(0, 2).toUpperCase()}
              </AvatarFallback>
            </Avatar>
            <div class="flex flex-col min-w-0">
              <span class="font-bold text-foreground truncate max-w-[150px] text-[11px]">{item.studentName}</span>
              <span class="text-[8px] text-muted-foreground font-mono select-none flex items-center gap-1">
                <span>{item.studentNum}</span>
                <span>•</span>
                <span>{item.className}</span>
              </span>
            </div>
          </div>
          
          <div class="flex items-center gap-3 shrink-0">
            <div class="flex flex-col items-end text-end">
              <span class="font-bold font-mono text-foreground text-[10px]">{formatCurrency(item.remaining, "YER")}</span>
              {#if item.daysLate > 0}
                <span class="text-[7px] font-mono text-destructive select-none font-bold mt-0.5">
                  {m.finance_overview_days_overdue({ days: item.daysLate })}
                </span>
              {:else}
                <span class="text-[7px] text-muted-foreground select-none font-semibold mt-0.5">
                  {m.finance_overview_due_soon()}
                </span>
              {/if}
            </div>
          </div>
        </div>
      {/each}
    {/if}
  </CardContent>
</Card>
