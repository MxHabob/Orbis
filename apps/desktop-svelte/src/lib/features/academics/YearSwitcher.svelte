<!--
@component YearSwitcher
Compact year selector dropdown for the sidebar/header.
Shows the current year and allows switching between available years.
-->
<script lang="ts">
import { academicContext } from "$lib/stores";
import * as m from "$lib/paraglide/messages";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
  SelectSeparator,
} from "@orbis/ui-svelte/select";
import { Badge } from "@orbis/ui-svelte/badge";
import { CalendarDays, Archive, Loader2 } from "lucide-svelte";

let { orgId }: { orgId: string } = $props();

const store = academicContext;

$effect(() => {
  if (orgId) {
    store.loadCurrentYear();
    store.loadYears(orgId);
  }
});

function handleSwitch(periodId: string) {
  store.switchYear(periodId);
}

const currentLabel = $derived(store.yearLabel || m.year_current());
</script>

<div class="px-3 py-2">
  <div class="flex items-center gap-2 text-xs text-muted-foreground mb-1.5 ps-1">
    <CalendarDays class="size-3.5" />
    <span class="font-medium">{m.year_current()}</span>
  </div>

  <Select
    type="single"
    value={store.currentYear?.id ?? ""}
    onValueChange={(v) => { if (v) handleSwitch(v); }}
  >
    <SelectTrigger
      class="w-full h-9 text-sm bg-card border-border/60 hover:bg-accent/50 transition-colors"
      disabled={store.isSwitching}
    >
      {#if store.isSwitching}
        <div class="flex items-center gap-2">
          <Loader2 class="size-3.5 animate-spin" />
          <span>{m.year_switching()}</span>
        </div>
      {:else}
        <span class="truncate">{currentLabel}</span>
      {/if}
    </SelectTrigger>
    <SelectContent class="max-h-[280px]">
      {#each store.years as yearInfo}
        <SelectItem value={yearInfo.year.id} class="text-sm">
          <div class="flex items-center justify-between gap-2 w-full">
            <span class="truncate">{yearInfo.year.name}</span>
            <div class="flex items-center gap-1.5 shrink-0">
              {#if yearInfo.year.status === "archived"}
                <Archive class="size-3 text-muted-foreground" />
              {/if}
              {#if yearInfo.is_active_in_app}
                <Badge variant="success" class="text-[10px] px-1.5 py-0">
                  {m.year_status_active()}
                </Badge>
              {/if}
            </div>
          </div>
        </SelectItem>
      {/each}
    </SelectContent>
  </Select>
</div>
