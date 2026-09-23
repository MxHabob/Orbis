<script lang="ts">
  import { Heart } from "@orbis/icons";

  interface Props {
    timeSlots: any[];
    locale?: string;
    compact?: boolean;
  }

  let { timeSlots = [], locale = "ar", compact = false }: Props = $props();

  const isArabic = $derived(locale === "ar");

  function formatTime(val: string): string {
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
</script>

<div
  class="flex flex-col shrink-0 select-none border-e border-border/60 bg-muted/20 {compact
    ? 'w-20'
    : 'w-28'}"
>
  <!-- Time Gutter Cells matching Slots -->
  {#each timeSlots as slot (slot.id)}
    {@const isBreak = Boolean(slot.is_break)}
    <div
      class="flex flex-col items-center justify-center border-b border-border/40 px-2 text-center transition-colors {isBreak
        ? 'bg-primary/5 min-h-[38px]'
        : compact
          ? 'min-h-[64px]'
          : 'min-h-[84px]'}"
    >
      {#if isBreak}
        <div
          class="flex items-center justify-center gap-1 text-[10px] font-semibold text-primary"
        >
          <Heart size={11} class="shrink-0" />
          <span class="truncate"
            >{slot.name_ar ||
              slot.name ||
              (isArabic ? "استراحة" : "Break")}</span
          >
        </div>
        <span class="text-[9px] text-muted-foreground font-mono">
          {formatTime(slot.start_time)}
        </span>
      {:else}
        <div class="flex items-center justify-center gap-1">
          <span class="text-[10px] font-bold text-center text-foreground">
            {slot.name_ar ||
              slot.name ||
              (isArabic
                ? `الحصة ${slot.slot_number}`
                : `Period ${slot.slot_number}`)}
          </span>
          {#if slot.duration_minutes}
            <span class="text-[9px] text-muted-foreground/70">
              {slot.duration_minutes}{isArabic ? "د" : "m"}
            </span>
          {/if}
        </div>
        <span
          class="text-[10px] font-medium text-muted-foreground font-mono mt-0.5"
        >
          {formatTime(slot.start_time)}
        </span>
        <span class="text-[9px] text-muted-foreground/60 font-mono">
          {formatTime(slot.end_time)}
        </span>
      {/if}
    </div>
  {/each}
</div>
