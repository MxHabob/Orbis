<!--
@component ReadinessGauge
A circular progress gauge for exam session readiness score.
Usage: <ReadinessGauge score={75} />
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
let { score = 0, size = 80 }: { score: number; size?: number } = $props();

const radius = $derived((size - 8) / 2);
const circumference = $derived(2 * Math.PI * radius);
const dashOffset = $derived(circumference - (score / 100) * circumference);

const color = $derived(
  score >= 80
    ? "text-emerald-500"
    : score >= 50
      ? "text-amber-500"
      : "text-red-500",
);

const strokeColor = $derived(
  score >= 80
    ? "stroke-emerald-500"
    : score >= 50
      ? "stroke-amber-500"
      : "stroke-red-500",
);

const label = $derived(
  score >= 80
    ? m.operations_ui_readiness_ready()
    : score >= 50
      ? m.operations_ui_readiness_partial()
      : m.operations_ui_readiness_not_ready(),
);
</script>

<div class="relative inline-flex items-center justify-center select-none" style="width: {size}px; height: {size}px;">
  <svg class="transform -rotate-90" width={size} height={size}>
    <!-- Background circle -->
    <circle
      cx={size / 2}
      cy={size / 2}
      r={radius}
      fill="none"
      stroke="currentColor"
      stroke-width="4"
      class="text-muted/30"
    />
    <!-- Progress circle -->
    <circle
      cx={size / 2}
      cy={size / 2}
      r={radius}
      fill="none"
      stroke-width="4"
      stroke-linecap="round"
      class={strokeColor}
      style="stroke-dasharray: {circumference}; stroke-dashoffset: {dashOffset}; transition: stroke-dashoffset 0.8s ease-in-out;"
    />
  </svg>
  <div class="absolute flex flex-col items-center">
    <span class="text-sm font-bold font-mono {color}">{Math.round(score)}%</span>
    <span class="text-[7px] text-muted-foreground font-bold uppercase tracking-wider">{label}</span>
  </div>
</div>
