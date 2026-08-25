<script lang="ts">
import type { Snippet } from "svelte";
import { cn } from "../../../utils.js";

export type MetricTone = "primary" | "success" | "warning" | "destructive" | "muted";

export type MetricSegment = {
  label: string;
  value: number;
  tone?: MetricTone;
  displayValue?: string;
};

let {
  variant = "single",
  title,
  metadata,
  action,
  value = 0,
  max = 100,
  segments = [],
  tone = "primary",
  class: className,
}: {
  variant?: "single" | "segmented";
  title?: Snippet;
  metadata?: Snippet;
  action?: Snippet;
  value?: number;
  max?: number;
  segments?: readonly MetricSegment[];
  tone?: MetricTone;
  class?: string;
} = $props();

const percentage = $derived(Math.min(100, Math.max(0, (value / Math.max(max, 1)) * 100)));
const segmentTotal = $derived(
  Math.max(segments.reduce((total, segment) => total + Math.max(0, segment.value), 0), 1),
);

function toneClass(segmentTone: MetricTone): string {
  switch (segmentTone) {
    case "success":
      return "bg-brand-lime";
    case "warning":
      return "bg-brand-indigo";
    case "destructive":
      return "bg-destructive";
    case "muted":
      return "bg-muted-foreground/35";
    default:
      return "bg-primary";
  }
}
</script>

<div class={cn("min-w-0 font-sans", className)}>
  {#if variant === "segmented"}
    <div class="flex min-w-0 flex-col gap-2">
      {#if title || metadata || action}
        <div class="flex min-w-0 items-start justify-between gap-3">
          <div class="min-w-0">
            {#if title}<div class="truncate text-xs font-semibold text-foreground">{@render title()}</div>{/if}
            {#if metadata}<div class="truncate text-[10px] text-muted-foreground">{@render metadata()}</div>{/if}
          </div>
          {#if action}<div class="shrink-0">{@render action()}</div>{/if}
        </div>
      {/if}
      <div class="flex h-2 w-full gap-1 overflow-hidden rounded-full bg-muted/50" aria-label="Metric breakdown">
        {#each segments as segment (segment.label)}
          <div
            class={cn("h-full min-w-0 transition-[width] duration-300", toneClass(segment.tone ?? "primary"))}
            style={`width: ${(Math.max(0, segment.value) / segmentTotal) * 100}%`}
            title={`${segment.label}: ${segment.displayValue ?? segment.value}`}
          ></div>
        {/each}
      </div>
      {#if segments.length > 0}
        <div class="flex flex-wrap gap-x-4 gap-y-1 text-[10px] text-muted-foreground">
          {#each segments as segment (segment.label)}
            <span class="inline-flex min-w-0 items-center gap-1.5">
              <span class={cn("size-1.5 shrink-0 rounded-full", toneClass(segment.tone ?? "primary"))}></span>
              <span class="truncate">{segment.label}</span>
              <span class="font-mono text-foreground">{segment.displayValue ?? segment.value}</span>
            </span>
          {/each}
        </div>
      {/if}
    </div>
  {:else}
    <div class="flex min-w-0 flex-col gap-2">
      <div class="flex min-w-0 items-start justify-between gap-3">
        <div class="min-w-0">
          {#if title}<div class="truncate text-xs font-semibold text-foreground">{@render title()}</div>{/if}
          {#if metadata}<div class="truncate text-[10px] text-muted-foreground">{@render metadata()}</div>{/if}
        </div>
        {#if action}<div class="shrink-0">{@render action()}</div>{/if}
      </div>
      <div class="flex items-center justify-between gap-3 text-[10px] text-muted-foreground">
        <span class="font-mono text-sm font-semibold text-foreground">{Math.round(percentage)}%</span>
        <span>{value} / {max}</span>
      </div>
      <div class="h-2 w-full overflow-hidden rounded-full bg-muted/50">
        <div class={cn("h-full rounded-full transition-[width] duration-300", toneClass(tone))} style={`width: ${percentage}%`}></div>
      </div>
    </div>
  {/if}
</div>
