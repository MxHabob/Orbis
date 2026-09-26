<script lang="ts">
  import { Heart } from "@orbis/icons";
  import { ARABIC_PERIOD_NAMES, ENGLISH_PERIOD_NAMES } from "./utils";

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
    slot ? [slot] : Array.isArray(timeSlots) ? timeSlots : []
  );

  function formatTime(val?: string): string {
    if (!val) return "";
    const clean = val.slice(0, 5);
    const [hStr, mStr] = clean.split(":");
    let h = parseInt(hStr, 10);
    const m = mStr || "00";
    if (isNaN(h)) return val;
    const period = h >= 12 ? (isArabic ? "م" : "PM") : isArabic ? "ص" : "AM";
    h = h % 12 || 12;
    return `${h}:${m} ${period}`;
  }

  function getSlotTitle(s: any): string {
    if (isArabic) {
      if (s.name_ar) return s.name_ar;
      if (s.display_name_ar) return s.display_name_ar;
      if (s.display_name) return s.display_name;
      const num = s.period_number ?? s.slot_index ?? s.slot_number ?? 1;
      return ARABIC_PERIOD_NAMES[num] || `الحصة ${num}`;
    } else {
      if (s.name) return s.name;
      if (s.display_name_en) return s.display_name_en;
      if (s.display_name) return s.display_name;
      const num = s.period_number ?? s.slot_index ?? s.slot_number ?? 1;
      return ENGLISH_PERIOD_NAMES[num] || `Period ${num}`;
    }
  }
</script>

{#each slotsList as s (s.id ?? s.slot_index ?? Math.random())}
  {@const isBreak = Boolean(s.is_break)}
  <div
    class="flex flex-col shrink-0 items-center justify-center border-e border-border/60 bg-muted/20 px-2 text-center select-none transition-colors {compact
      ? 'w-20'
      : 'w-28'} {className}"
  >
    {#if isBreak}
      <div
        class="flex items-center justify-center gap-1 text-[10px] font-semibold text-primary"
      >
        <Heart size={11} class="shrink-0" />
        <span class="truncate"
          >{s.name_ar ||
            s.name ||
            (isArabic ? "استراحة" : "Break")}</span
        >
      </div>
      {#if s.start_time}
        <span class="text-[9px] text-muted-foreground font-mono mt-0.5">
          {formatTime(s.start_time)}
        </span>
      {/if}
    {:else}
      <div class="flex items-center justify-center gap-1">
        <span class="text-[11px] font-bold text-center text-foreground">
          {getSlotTitle(s)}
        </span>
        {#if s.duration_minutes}
          <span class="text-[9px] text-muted-foreground/70 font-normal">
            {s.duration_minutes}{isArabic ? "د" : "m"}
          </span>
        {/if}
      </div>
      {#if s.start_time && s.end_time}
        <span
          class="text-[10px] font-medium text-muted-foreground font-mono mt-0.5 leading-tight"
        >
          {formatTime(s.start_time)}
        </span>
        <span class="text-[9px] text-muted-foreground/60 font-mono leading-tight">
          {formatTime(s.end_time)}
        </span>
      {/if}
    {/if}
  </div>
{/each}
