<script lang="ts">
  interface Weekday {
    value: number; // 1 = Sunday, 2 = Monday, etc.
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

<div class="flex flex-1 min-w-0 border-b border-border/60 bg-muted/10">
  {#each weekdays as day (day.value)}
    {@const isSelected = activeDay === day.value}
    {@const count = lessonsByDay[day.value] ?? 0}
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      onclick={() => onDaySelect?.(day.value)}
      class="flex-1 min-w-0 h-10 flex items-center justify-center gap-2 px-3 border-e border-border/40 text-center transition-colors last:border-e-0 {day.isToday
        ? 'bg-primary/5 font-bold'
        : isSelected
          ? 'bg-accent/40 font-bold'
          : 'hover:bg-muted/30 font-medium'}"
    >
      <div class="flex items-center justify-center gap-1.5 min-w-0 truncate">
        <span class="text-xs truncate text-foreground {day.isToday ? 'text-primary' : ''}">
          {day.label}
        </span>
        {#if day.dateLabel}
          <span class="text-[10px] text-muted-foreground font-normal">
            {day.dateLabel}
          </span>
        {/if}
        {#if day.isToday}
          <span class="inline-block h-1.5 w-1.5 rounded-full bg-primary shrink-0" title={isArabic ? "اليوم" : "Today"}></span>
        {/if}
      </div>

      {#if count > 0}
        <span
          class="shrink-0 rounded-full bg-muted px-1.5 py-0.2 text-[9px] font-semibold text-muted-foreground"
          title={isArabic ? `${count} حصص` : `${count} lessons`}
        >
          {count}
        </span>
      {/if}
    </div>
  {/each}
</div>
