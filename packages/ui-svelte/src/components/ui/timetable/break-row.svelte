<script lang="ts">
  import { Coffee, Sun, Utensils, Moon } from "@lucide/svelte";

  interface Props {
    slot: any;
    weekdaysCount: number;
    locale?: string;
  }

  let { slot, weekdaysCount = 5, locale = "ar" }: Props = $props();

  const isArabic = $derived(locale === "ar");

  const breakName = $derived(
    slot.name_ar || slot.name || (isArabic ? "استراحة" : "Break")
  );

  function formatTime(val: string): string {
    if (!val) return "";
    const clean = val.slice(0, 5);
    const [hStr, mStr] = clean.split(":");
    let h = parseInt(hStr, 10);
    const m = mStr || "00";
    if (isNaN(h)) return val;
    const period = h >= 12 ? (isArabic ? "م" : "PM") : (isArabic ? "ص" : "AM");
    h = h % 12 || 12;
    return `${h}:${m} ${period}`;
  }

  const timeLabel = $derived.by(() => {
    const s = slot.start_time;
    const e = slot.end_time;
    return s && e ? `${formatTime(s)} - ${formatTime(e)}` : "";
  });
</script>

<div
  class="flex min-h-[38px] w-full items-center justify-center gap-2 border-b border-border/40 bg-primary/5 px-4 text-xs font-semibold text-muted-foreground/90 transition-colors select-none"
  style="background-image: repeating-linear-gradient(45deg, transparent, transparent 10px, rgba(0,0,0,0.015) 10px, rgba(0,0,0,0.015) 20px);"
>
  {#if slot.break_type === "lunch"}
    <Utensils size={13} class="text-primary" />
  {:else if slot.break_type === "prayer"}
    <Moon size={13} class="text-primary" />
  {:else if slot.break_type === "morning"}
    <Sun size={13} class="text-primary" />
  {:else}
    <Coffee size={13} class="text-primary" />
  {/if}

  <span class="tracking-wide text-foreground/80">{breakName}</span>

  {#if timeLabel}
    <span class="text-[10px] text-muted-foreground font-mono">({timeLabel})</span>
  {/if}
</div>
