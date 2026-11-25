<!--
@component CapacityIndicator
Visual capacity bar for class enrollment.

Usage:
  <CapacityIndicator current={28} max={30} />
-->
<script lang="ts">
import { cn } from "@orbis/ui/utils";

interface Props {
  current: number;
  max: number;
  class?: string;
  showLabel?: boolean;
  size?: "sm" | "default";
}

let {
  current,
  max,
  class: className = "",
  showLabel = true,
  size = "default",
}: Props = $props();

let pct = $derived(max > 0 ? Math.min((current / max) * 100, 100) : 0);
let available = $derived(Math.max(max - current, 0));

let colorClass = $derived(
  pct >= 95
    ? "bg-red-500"
    : pct >= 80
      ? "bg-amber-500"
      : pct >= 60
        ? "bg-yellow-500"
        : "bg-emerald-500",
);

let textColor = $derived(
  pct >= 95
    ? "text-red-600"
    : pct >= 80
      ? "text-amber-600"
      : "text-muted-foreground",
);

let barHeight = $derived(size === "sm" ? "h-1.5" : "h-2");
</script>

<div class={cn("flex items-center gap-2", className)}>
  <div class={cn("flex-1 rounded-full bg-muted/50 overflow-hidden", barHeight)}>
    <div
      class={cn("h-full rounded-full transition-all duration-500", colorClass)}
      style="width: {pct}%"
    ></div>
  </div>
  {#if showLabel}
    <span class={cn("text-xs font-medium tabular-nums whitespace-nowrap", textColor)}>
      {current}/{max}
    </span>
  {/if}
</div>
