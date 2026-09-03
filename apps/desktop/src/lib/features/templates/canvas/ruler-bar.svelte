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
  containerSizePx,
}: {
  axis: "horizontal" | "vertical";
  pageSizeMm: number;
  viewport: ViewportTransform;
  offsetX: number;
  offsetY: number;
  cursorMm: number;
  containerSizePx: number;
} = $props();

const RULER_SIZE = 24; // px
const isHorizontal = $derived(axis === "horizontal");

// We need to figure out what range of mm is visible.
// We can use a helper to convert px to mm:
import { pxToMm } from "@docforge/core/canvas";

const ticks = $derived(generateTicks());

function generateTicks() {
  const result: { pos: number; label?: string; major: boolean }[] = [];

  const offset = isHorizontal ? offsetX : offsetY;

  // startPx is 0, endPx is containerSizePx
  const startPx = -RULER_SIZE; // slight buffer
  const endPx = containerSizePx + RULER_SIZE; // slight buffer

  // Calculate the mm at start and end of the visible ruler area
  const startMm = pxToMm(startPx - offset, viewport.dpi, viewport.zoom);
  const endMm = pxToMm(endPx - offset, viewport.dpi, viewport.zoom);

  // We want nice steps. Normally step is 5mm or 10mm.
  // If we zoom out a lot, drawing ticks every 5mm might be too dense and slow!
  // Let's adjust the step based on zoom.
  let step = 5;
  let majorStep = 10;

  if (viewport.zoom < 0.3) {
    step = 50;
    majorStep = 100;
  } else if (viewport.zoom < 0.6) {
    step = 10;
    majorStep = 50;
  }

  // Snap start to the nearest step
  const firstTickMm = Math.floor(startMm / step) * step;
  const lastTickMm = Math.ceil(endMm / step) * step;

  for (let mm = firstTickMm; mm <= lastTickMm; mm += step) {
    const px = mmToPx(mm, viewport.dpi, viewport.zoom);
    const isMajor = mm % majorStep === 0;
    result.push({
      pos: px,
      label: isMajor ? String(Math.round(mm)) : undefined,
      major: isMajor,
    });
  }
  return result;
}

const cursorPx = $derived(
  mmToPx(cursorMm, viewport.dpi, viewport.zoom), // Allow negative cursor
);
</script>

{#if isHorizontal}
  <!-- Horizontal ruler (top) -->
  <div
    class="absolute top-0 h-6 bg-card border-b border-border/40 overflow-hidden z-10"
    style="inset-inline-start: {RULER_SIZE}px; inset-inline-end: 0;"
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
        {#if cursorMm !== undefined}
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
    class="absolute inset-s-0 w-6 bg-card border-e border-border/40 overflow-hidden z-10"
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
        {#if cursorMm !== undefined}
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
