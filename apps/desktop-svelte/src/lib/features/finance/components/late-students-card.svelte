<script lang="ts">
import { Card } from "@orbis/ui-svelte";
import { Button } from "@orbis/ui-svelte/button";
import { Badge } from "@orbis/ui-svelte/badge";
import { Avatar, AvatarFallback } from "@orbis/ui-svelte/avatar";
import { formatCurrency } from "$lib/core/i18n";
import { localizeHref } from "$lib/paraglide/runtime";
import * as finance from "$lib/paraglide/messages/finance";

let {
  lateStudents,
  orgId,
}: {
  lateStudents: any[];
  orgId: string;
} = $props();
</script>

<Card class="lg:col-span-2 p-5 bg-card border border-border/60 flex flex-col justify-between text-start shadow-sm min-h-[315px] h-full transition-all duration-200 card-tactile">
  <div class="flex items-center justify-between w-full select-none text-[9px] font-bold text-muted-foreground/80 mb-3">
    <div class="flex items-center gap-1.5 font-sans font-extrabold uppercase tracking-wider text-[8px] text-muted-foreground/75">
      <span class="h-1.5 w-1.5 rounded-full bg-destructive animate-pulse"></span>
      {finance.overview_late_students ? finance.overview_late_students() : "الطلاب المتأخرين عن السداد"}
    </div>
    <div class="flex items-center gap-2">
      <a href={localizeHref(`/${orgId}/finance/fees`)}>
        <Button variant="outline" size="xs" class="h-6 text-[9px] font-bold cursor-pointer">
          {finance.overview_btn_assign_fee ? finance.overview_btn_assign_fee() : "فرض رسوم"}
        </Button>
      </a>
      <a href={localizeHref(`/${orgId}/finance/fees`)}>
        <Button variant="outline" size="xs" class="h-6 text-[9px] font-bold cursor-pointer">
          {finance.overview_view_all ? finance.overview_view_all() : "عرض الكل"}
        </Button>
      </a>
    </div>
  </div>

  <div class="flex flex-col gap-2 mt-2 flex-grow justify-center">
    {#if lateStudents.length === 0}
      <div class="py-8 text-center text-muted-foreground text-xs select-none">
        {finance.overview_no_late_students()}
      </div>
    {:else}
      {#each lateStudents.slice(0, 3) as item (item.feeId)}
        <div class="flex items-center justify-between p-2.5 rounded-xl border border-border/40 bg-card/30 hover:bg-card/50 transition-all duration-200">
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
                  {finance.overview_days_overdue ? finance.overview_days_overdue({ days: item.daysLate }) : `متأخر ${item.daysLate} يوم`}
                </span>
              {:else}
                <span class="text-[7px] text-muted-foreground select-none font-semibold mt-0.5">
                  يستحق قريباً
                </span>
              {/if}
            </div>
          </div>
        </div>
      {/each}
    {/if}
  </div>
</Card>
