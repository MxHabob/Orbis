<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Input } from "@orbis/ui/input";
import { Popover, PopoverContent, PopoverTrigger } from "@orbis/ui/popover";
import { Calendar, Check, ChevronDown } from "@orbis/icons";
import {
  getDateRangeForMonth,
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

function updateSingleDay(date: string) {
  if (!date) return;
  value = { preset: "custom", fromDate: date, toDate: date };
}

function updateMonth(month: string) {
  if (month) value = getDateRangeForMonth(month);
}

function updateYearToDate(date: string) {
  if (!date) return;
  value = {
    preset: "year_to_date",
    fromDate: `${date.slice(0, 4)}-01-01`,
    toDate: date,
  };
}

function updateDate(field: "fromDate" | "toDate", date: string) {
  value = { ...value, preset: "custom", [field]: date };
}
</script>

<Popover bind:open={isOpen}>
  <div
    class="inline-flex h-8 max-w-full items-center overflow-hidden rounded-md border border-input bg-background shadow-xs"
  >
    <PopoverTrigger>
      <button
        type="button"
        class="inline-flex h-8 items-center gap-2 px-2.5 text-xs font-medium outline-none hover:bg-accent"
        aria-label={m.finance_ui_filter_by_date()}
        aria-haspopup="dialog"
      >
        <Calendar size={14} class="shrink-0 text-muted-foreground" />
        <span class="truncate">{selectedLabel}</span>
        <ChevronDown size={14} class="shrink-0 text-muted-foreground" />
      </button>
    </PopoverTrigger>
    <span class="h-5 w-px shrink-0 bg-border" aria-hidden="true"></span>
    {#if value.preset === "today"}
      <Input
        type="date"
        value={value.fromDate}
        aria-label={m.finance_ui_date()}
        class="h-7 w-36 border-0 bg-transparent px-2 text-xs shadow-none focus-visible:ring-0"
        onchange={(event) => updateSingleDay(event.currentTarget.value)}
      />
    {:else if value.preset === "this_month"}
      <Input
        type="month"
        value={value.fromDate.slice(0, 7)}
        aria-label={m.finance_ui_this_month()}
        class="h-7 w-36 border-0 bg-transparent px-2 text-xs shadow-none focus-visible:ring-0"
        onchange={(event) => updateMonth(event.currentTarget.value)}
      />
    {:else if value.preset === "year_to_date"}
      <Input
        type="date"
        value={value.toDate}
        min={value.fromDate}
        aria-label={m.finance_ui_year_to_date()}
        class="h-7 w-36 border-0 bg-transparent px-2 text-xs shadow-none focus-visible:ring-0"
        onchange={(event) => updateYearToDate(event.currentTarget.value)}
      />
    {:else}
      <div class="flex items-center gap-1 px-1.5">
        <Input
          type="date"
          value={value.fromDate}
          max={value.toDate || undefined}
          aria-label={m.finance_ui_date_from()}
          class="h-7 w-32 border-0 bg-transparent px-1.5 text-xs shadow-none focus-visible:ring-0"
          onchange={(event) => updateDate("fromDate", event.currentTarget.value)}
        />
        <span class="text-muted-foreground" aria-hidden="true">–</span>
        <Input
          type="date"
          value={value.toDate}
          min={value.fromDate || undefined}
          aria-label={m.finance_ui_to_date()}
          class="h-7 w-32 border-0 bg-transparent px-1.5 text-xs shadow-none focus-visible:ring-0"
          onchange={(event) => updateDate("toDate", event.currentTarget.value)}
        />
      </div>
    {/if}
  </div>
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

  </PopoverContent>
</Popover>
