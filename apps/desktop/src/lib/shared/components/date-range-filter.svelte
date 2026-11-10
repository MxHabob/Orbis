<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Popover, PopoverContent, PopoverTrigger } from "@orbis/ui/popover";
import { Calendar, Check, ChevronDown } from "@orbis/icons";
import {
  getDateRangeForPreset,
  type DateRangeFilterValue,
  type ResolvedDateRangePreset,
} from "./date-range";

let { value = $bindable() }: { value: DateRangeFilterValue } = $props();
let isOpen = $state(false);

const presets: {
  id: ResolvedDateRangePreset;
  label: () => string;
}[] = [
  { id: "today", label: m.finance_ui_today },
  { id: "this_month", label: m.finance_ui_this_month },
  { id: "year_to_date", label: m.finance_ui_year_to_date },
];

const selectedLabel = $derived.by(() => {
  if (value.preset === "today") return m.finance_ui_today();
  if (value.preset === "this_month") return m.finance_ui_this_month();
  if (value.preset === "year_to_date") return m.finance_ui_year_to_date();
  return m.finance_ui_custom_range();
});

function selectPreset(preset: ResolvedDateRangePreset) {
  value = getDateRangeForPreset(preset);
  isOpen = false;
}

function selectCustomRange() {
  value = { ...value, preset: "custom" };
}

function updateDate(field: "fromDate" | "toDate", date: string) {
  value = { ...value, preset: "custom", [field]: date };
}
</script>

<Popover bind:open={isOpen}>
  <PopoverTrigger>
    <Button
      variant="outline"
      size="sm"
      class="h-8 max-w-full gap-2 px-2.5 text-xs font-medium"
      aria-label={m.finance_ui_filter_by_date()}
      aria-haspopup="dialog"
    >
      <Calendar size={14} class="shrink-0 text-muted-foreground" />
      <span class="truncate">{selectedLabel}</span>
      <span class="h-4 w-px shrink-0 bg-border" aria-hidden="true"></span>
      <ChevronDown size={14} class="shrink-0 text-muted-foreground" />
    </Button>
  </PopoverTrigger>
  <PopoverContent
    align="end"
    sideOffset={6}
    class="w-[min(320px,calc(100vw-2rem))] space-y-1 rounded-xl p-2 text-start"
  >
    <p class="px-2 py-1 text-[11px] font-semibold text-muted-foreground">
      {m.finance_ui_filter_by_date()}
    </p>
    {#each presets as preset (preset.id)}
      <button
        type="button"
        class="flex h-9 w-full items-center gap-2 rounded-md px-2 text-start text-sm transition-colors hover:bg-accent hover:text-accent-foreground"
        aria-pressed={value.preset === preset.id}
        onclick={() => selectPreset(preset.id)}
      >
        <span class="flex size-4 shrink-0 items-center justify-center">
          {#if value.preset === preset.id}<Check size={14} />{/if}
        </span>
        <span>{preset.label()}</span>
      </button>
    {/each}
    <div class="my-1 border-t border-border"></div>
    <button
      type="button"
      class="flex h-9 w-full items-center gap-2 rounded-md px-2 text-start text-sm transition-colors hover:bg-accent hover:text-accent-foreground"
      aria-pressed={value.preset === "custom"}
      onclick={selectCustomRange}
    >
      <span class="flex size-4 shrink-0 items-center justify-center">
        {#if value.preset === "custom"}<Check size={14} />{/if}
      </span>
      <span>{m.finance_ui_custom_range()}</span>
    </button>

    {#if value.preset === "custom"}
      <div class="grid grid-cols-2 gap-3 border-t border-border px-2 pt-3">
        <label class="min-w-0 space-y-1.5 text-xs font-medium">
          <span>{m.finance_ui_date_from()}</span>
          <Input
            type="date"
            value={value.fromDate}
            max={value.toDate || undefined}
            aria-label={m.finance_ui_date_from()}
            class="h-8 min-w-0 px-2 text-xs font-mono"
            onchange={(event) => updateDate("fromDate", event.currentTarget.value)}
          />
        </label>
        <label class="min-w-0 space-y-1.5 text-xs font-medium">
          <span>{m.finance_ui_to_date()}</span>
          <Input
            type="date"
            value={value.toDate}
            min={value.fromDate || undefined}
            aria-label={m.finance_ui_to_date()}
            class="h-8 min-w-0 px-2 text-xs font-mono"
            onchange={(event) => updateDate("toDate", event.currentTarget.value)}
          />
        </label>
      </div>
    {/if}
  </PopoverContent>
</Popover>
