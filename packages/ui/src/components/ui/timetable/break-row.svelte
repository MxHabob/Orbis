<script lang="ts">
import { Heart, LightBulb } from "@orbis/icons";
import { formatTimeRange } from "./utils.js";

interface Props {
  slot: any;
  weekdaysCount?: number;
  locale?: string;
}

let { slot, weekdaysCount = 5, locale = "ar" }: Props = $props();

const isArabic = $derived(locale === "ar");

const breakName = $derived(
  slot.name_ar ||
    slot.name ||
    slot.display_name ||
    (isArabic ? "استراحة" : "Break"),
);

const timeLabel = $derived(
  formatTimeRange(slot.start_time, slot.end_time, isArabic),
);
</script>

<div
  class="flex min-h-9 w-full items-center justify-center gap-2 border-b border-border/50 bg-muted/25 px-3 py-1.5 text-xs font-medium text-muted-foreground select-none"
  style="grid-column: span {Math.max(1, weekdaysCount)} / span {Math.max(1, weekdaysCount)}; background-image: repeating-linear-gradient(-45deg, transparent, transparent 10px, color-mix(in srgb, var(--border) 30%, transparent) 10px, color-mix(in srgb, var(--border) 30%, transparent) 20px);"
>
  {#if slot.break_type === "prayer" || slot.break_type === "morning"}
    <LightBulb size={13} class="shrink-0 text-primary/80" />
  {:else}
    <Heart size={13} class="shrink-0 text-primary/80" />
  {/if}

  <span class="font-semibold text-foreground/85">{breakName}</span>

  {#if timeLabel}
    <span class="font-mono text-[11px] text-muted-foreground tabular-nums">
      ({timeLabel})
    </span>
  {/if}
</div>
