<script lang="ts">
import { Heart } from "@orbis/icons";
import {
  ARABIC_PERIOD_NAMES,
  ENGLISH_PERIOD_NAMES,
  formatTimeSlotTime,
} from "./utils.js";

interface Props {
  slot?: any;
  timeSlots?: any[];
  locale?: string;
  compact?: boolean;
  class?: string;
}

let {
  slot,
  timeSlots,
  locale = "ar",
  compact = false,
  class: className = "",
}: Props = $props();

const isArabic = $derived(locale === "ar");

const slotsList = $derived(
  slot ? [slot] : Array.isArray(timeSlots) ? timeSlots : [],
);

function getSlotTitle(s: any): string {
  if (isArabic) {
    if (s.name_ar) return s.name_ar;
    if (s.display_name_ar) return s.display_name_ar;
    if (s.display_name) return s.display_name;
    const num = s.period_number ?? s.slot_index ?? s.slot_number ?? 1;
    return ARABIC_PERIOD_NAMES[num] || `الحصة ${num}`;
  }
  if (s.name) return s.name;
  if (s.display_name_en) return s.display_name_en;
  if (s.display_name) return s.display_name;
  const num = s.period_number ?? s.slot_index ?? s.slot_number ?? 1;
  return ENGLISH_PERIOD_NAMES[num] || `Period ${num}`;
}
</script>

{#each slotsList as s, idx (s.id ?? s.slot_index ?? idx)}
  {@const isBreak = Boolean(s.is_break)}
  <div
    class="sticky inset-s-0 z-10 flex shrink-0 flex-col items-center justify-center border-b border-e border-border bg-card px-2 py-2 text-center select-none {compact
      ? 'w-20'
      : 'w-28'} {isBreak ? 'bg-muted/30' : 'bg-muted/15'} {className}"
  >
    {#if isBreak}
      <div
        class="flex items-center justify-center gap-1 text-[11px] font-semibold text-primary/90"
      >
        <Heart size={11} class="shrink-0" />
        <span class="truncate">
          {s.name_ar || s.name || (isArabic ? "استراحة" : "Break")}
        </span>
      </div>
      {#if s.start_time}
        <span class="mt-0.5 font-mono text-[10px] text-muted-foreground tabular-nums">
          {formatTimeSlotTime(s.start_time, isArabic)}
        </span>
      {/if}
    {:else}
      <span class="truncate text-xs font-semibold text-foreground leading-tight">
        {getSlotTitle(s)}
      </span>
      {#if s.start_time && s.end_time}
        <span class="mt-1 font-mono text-[10px] text-muted-foreground tabular-nums leading-tight">
          {formatTimeSlotTime(s.start_time, isArabic)}
        </span>
        <span class="font-mono text-[10px] text-muted-foreground/70 tabular-nums leading-tight">
          {formatTimeSlotTime(s.end_time, isArabic)}
        </span>
      {/if}
    {/if}
  </div>
{/each}
