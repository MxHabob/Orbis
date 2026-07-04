<script lang="ts">
/**
 * RulerBar — Horizontal or vertical ruler with mm markings.
 * Tracks cursor position with a red indicator.
 */
import type { ViewportTransform } from "@docforge/core";
import { mmToPx } from "@docforge/core/canvas";

let {
  axis,
  pageSizeMm,
  viewport,
  offsetX,
  offsetY,
  cursorMm,
}: {
  axis: "horizontal" | "vertical";
  pageSizeMm: number;
  viewport: ViewportTransform;
  offsetX: number;
  offsetY: number;
  cursorMm: number;
} = $props();

const RULER_SIZE = 24; // px
const isHorizontal = $derived(axis === "horizontal");

// Generate tick marks every 5mm, labels every 10mm
const ticks = $derived(generateTicks());

function generateTicks() {
  const result: { pos: number; label?: string; major: boolean }[] = [];
  const step = 5; // mm
  for (let mm = 0; mm <= pageSizeMm; mm += step) {
    const px = mmToPx(mm, viewport.dpi, viewport.zoom);
    const isMajor = mm % 10 === 0;
    result.push({
      pos: px,
      label: isMajor ? String(mm) : undefined,
      major: isMajor,
    });
  }
  return result;
}

const cursorPx = $derived(mmToPx(Math.max(0, cursorMm), viewport.dpi, viewport.zoom));
</script>

{#if isHorizontal}
  <!-- Horizontal ruler (top) -->
  <div
    class="absolute top-0 h-6 bg-card border-b border-border/40 overflow-hidden z-10"
    style="left: {RULER_SIZE}px; right: 0;"
  >
    <svg width="100%" height={RULER_SIZE} class="block">
      <g transform="translate({offsetX}, 0)">
        {#each ticks as tick}
          <line
            x1={tick.pos}
            y1={tick.major ? 8 : 14}
            x2={tick.pos}
            y2={RULER_SIZE}
            stroke="var(--color-muted-foreground)"
            stroke-width="0.5"
            opacity={tick.major ? 0.5 : 0.25}
          />
          {#if tick.label}
            <text
              x={tick.pos + 2}
              y="10"
              fill="var(--color-muted-foreground)"
              font-size="8"
              opacity="0.6"
            >
              {tick.label}
            </text>
          {/if}
        {/each}

        <!-- Cursor indicator -->
        {#if cursorMm >= 0 && cursorMm <= pageSizeMm}
          <line
            x1={cursorPx}
            y1="0"
            x2={cursorPx}
            y2={RULER_SIZE}
            stroke="#ef4444"
            stroke-width="1"
          />
        {/if}
      </g>
    </svg>
  </div>
{:else}
  <!-- Vertical ruler (left) -->
  <div
    class="absolute start-0 w-6 bg-card border-e border-border/40 overflow-hidden z-10"
    style="top: {RULER_SIZE}px; bottom: 0;"
  >
    <svg width={RULER_SIZE} height="100%" class="block">
      <g transform="translate(0, {offsetY})">
        {#each ticks as tick}
          <line
            x1={tick.major ? 8 : 14}
            y1={tick.pos}
            x2={RULER_SIZE}
            y2={tick.pos}
            stroke="var(--color-muted-foreground)"
            stroke-width="0.5"
            opacity={tick.major ? 0.5 : 0.25}
          />
          {#if tick.label}
            <text
              x="2"
              y={tick.pos - 2}
              fill="var(--color-muted-foreground)"
              font-size="7"
              opacity="0.6"
            >
              {tick.label}
            </text>
          {/if}
        {/each}

        <!-- Cursor indicator -->
        {#if cursorMm >= 0 && cursorMm <= pageSizeMm}
          <line
            x1="0"
            y1={cursorPx}
            x2={RULER_SIZE}
            y2={cursorPx}
            stroke="#ef4444"
            stroke-width="1"
          />
        {/if}
      </g>
    </svg>
  </div>
{/if}
