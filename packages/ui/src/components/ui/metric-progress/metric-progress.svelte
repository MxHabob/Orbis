<script lang="ts">
import type { Snippet, ComponentType } from "svelte";
import { cn } from "../../../utils.js";
import { Card } from "../card/index.js";

export type MetricTone = "primary" | "success" | "warning" | "destructive" | "muted" | "info" | "purple";

export type MetricSegment = {
  label: string;
  value: number;
  tone?: MetricTone;
  colorClass?: string;
  displayValue?: string;
};

let {
  variant = "single",
  title,
  subtitle,
  metadata,
  action,
  icon: IconComponent,
  value = 0,
  displayValue,
  max = 100,
  showProgress = true,
  segments = [],
  tone = "primary",
  card = false,
  class: className,
}: {
  variant?: "single" | "segmented";
  title?: string | Snippet;
  subtitle?: string | Snippet;
  metadata?: string | Snippet;
  action?: Snippet;
  icon?: any;
  value?: number;
  displayValue?: string | number;
  max?: number;
  showProgress?: boolean;
  segments?: readonly MetricSegment[];
  tone?: MetricTone;
  card?: boolean;
  class?: string;
} = $props();

const percentage = $derived(Math.min(100, Math.max(0, (value / Math.max(max, 1)) * 100)));
const segmentTotal = $derived(
  Math.max(segments.reduce((total, segment) => total + Math.max(0, segment.value), 0), 1),
);

function toneClass(segmentTone?: MetricTone, customColor?: string): string {
  if (customColor) return customColor;
  switch (segmentTone) {
    case "success":
      return "bg-emerald-500";
    case "warning":
      return "bg-amber-500";
    case "destructive":
      return "bg-rose-500";
    case "info":
      return "bg-blue-500";
    case "purple":
      return "bg-purple-500";
    case "muted":
      return "bg-muted-foreground/35";
    default:
      return "bg-primary";
  }
}
</script>

{#snippet content()}
  <div class={cn("flex flex-col gap-3 font-sans text-start w-full min-w-0", !card && className)}>
    
    <!-- Header: Title, Icon, Subtitle, and Actions -->
    {#if title || subtitle || metadata || IconComponent || action}
      <div class="flex items-start justify-between gap-2">
        <div class="flex flex-col min-w-0 flex-1">
          
          <!-- Title & Icon -->
          <div class="flex items-center gap-1.5 text-xs font-medium text-muted-foreground">
            {#if IconComponent}
              <IconComponent size={14} class="opacity-80 shrink-0" />
            {/if}
            {#if typeof title === "string"}
              <span class="truncate font-semibold text-foreground">{title}</span>
            {:else if title}
              <div class="truncate font-semibold text-foreground">{@render title()}</div>
            {/if}
          </div>

          <!-- Subtitle / Metadata -->
          {#if typeof subtitle === "string"}
            <span class="text-[10px] text-muted-foreground mt-0.5 truncate">{subtitle}</span>
          {:else if subtitle}
            <div class="text-[10px] text-muted-foreground mt-0.5 truncate">{@render subtitle()}</div>
          {:else if typeof metadata === "string"}
            <span class="text-[10px] text-muted-foreground mt-0.5 truncate">{metadata}</span>
          {:else if metadata}
            <div class="text-[10px] text-muted-foreground mt-0.5 truncate">{@render metadata()}</div>
          {/if}

        </div>

        {#if action}
          <div class="shrink-0">{@render action()}</div>
        {/if}
      </div>
    {/if}

    <!-- Metric Value (e.g. 90%, 75%, or custom displayValue) -->
    {#if displayValue !== undefined || (variant === "single" && !showProgress)}
      <div class="flex items-baseline justify-between">
        <span class="text-2xl font-bold tracking-tight text-foreground font-mono">
          {displayValue ?? value}
        </span>
      </div>
    {/if}

    <!-- Progress Bar (Segmented or Single) -->
    {#if variant === "segmented" || segments.length > 0}
      <div class="flex flex-col gap-2">
        <!-- Segmented Bar -->
        <div class="flex h-2 w-full gap-0.5 overflow-hidden rounded-full bg-muted/40" aria-label="Metric breakdown">
          {#each segments as segment (segment.label)}
            <div
              class={cn("h-full min-w-0 transition-[width] duration-300", toneClass(segment.tone, segment.colorClass))}
              style={`width: ${(Math.max(0, segment.value) / segmentTotal) * 100}%`}
              title={`${segment.label}: ${segment.displayValue ?? segment.value}`}
            ></div>
          {/each}
        </div>

        <!-- Legend / Indicators -->
        {#if segments.length > 0}
          <div class="flex flex-wrap items-center gap-x-3 gap-y-1 text-[10px] text-muted-foreground">
            {#each segments as segment (segment.label)}
              <span class="inline-flex min-w-0 items-center gap-1.5">
                <span class={cn("size-1.5 shrink-0 rounded-full", toneClass(segment.tone, segment.colorClass))}></span>
                <span class="truncate">{segment.label}</span>
                {#if segment.displayValue}
                  <span class="font-mono text-foreground">{segment.displayValue}</span>
                {/if}
              </span>
            {/each}
          </div>
        {/if}
      </div>
    {:else if showProgress}
      <div class="flex flex-col gap-1.5">
        {#if displayValue === undefined}
          <div class="flex items-center justify-between gap-3 text-[10px] text-muted-foreground">
            <span class="font-mono text-base font-bold text-foreground">{Math.round(percentage)}%</span>
            <span>{value} / {max}</span>
          </div>
        {/if}
        <div class="h-2 w-full overflow-hidden rounded-full bg-muted/40">
          <div 
            class={cn("h-full rounded-full transition-[width] duration-300", toneClass(tone))} 
            style={`width: ${percentage}%`}
          ></div>
        </div>
      </div>
    {/if}

  </div>
{/snippet}

{#if card}
  <Card class={cn("p-4 shadow-sm border-border/50", className)}>
    {@render content()}
  </Card>
{:else}
  {@render content()}
{/if}
