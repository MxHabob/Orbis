<script lang="ts">
interface Weekday {
  value?: number;
  day?: number;
  label: string;
  isToday?: boolean;
  dateLabel?: string;
}

interface Props {
  weekdays: Weekday[];
  activeDay?: number | null;
  lessonsByDay?: Record<number, number>;
  locale?: string;
  onDaySelect?: (day: number) => void;
}

let {
  weekdays = [],
  activeDay = null,
  lessonsByDay = {},
  locale = "ar",
  onDaySelect,
}: Props = $props();

const isArabic = $derived(locale === "ar");
</script>

{#each weekdays as day, idx (day.value ?? day.day ?? idx)}
  {@const dayValue = day.value ?? day.day ?? idx + 1}
  {@const isSelected = activeDay === dayValue}
  {@const count = lessonsByDay[dayValue] ?? 0}
  <button
    type="button"
    onclick={() => onDaySelect?.(dayValue)}
    class="sticky top-0 z-20 flex h-11 min-w-0 items-center justify-center gap-2 border-b border-e border-border bg-card px-3 text-center transition-colors last:border-e-0 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-inset focus-visible:ring-ring {day.isToday
      ? 'bg-primary/[0.06] font-semibold'
      : isSelected
        ? 'bg-accent/50 font-semibold'
        : 'hover:bg-muted/30 font-medium'}"
  >
    <div class="flex min-w-0 items-center justify-center gap-1.5 truncate">
      {#if day.isToday}
        <span
          class="inline-block h-1.5 w-1.5 shrink-0 rounded-full bg-primary"
          title={isArabic ? "اليوم" : "Today"}
        ></span>
      {/if}
      <span
        class="truncate text-xs {day.isToday
          ? 'text-primary'
          : 'text-foreground'}"
      >
        {day.label}
      </span>
      {#if day.dateLabel}
        <span class="text-[11px] font-normal text-muted-foreground tabular-nums">
          {day.dateLabel}
        </span>
      {/if}
    </div>

    {#if count > 0}
      <span
        class="shrink-0 rounded-sm border border-border/70 bg-muted/50 px-1.5 py-0.5 font-mono text-[10px] font-medium text-muted-foreground tabular-nums leading-none"
        title={isArabic ? `${count} حصص` : `${count} lessons`}
      >
        {count}
      </span>
    {/if}
  </button>
{/each}
