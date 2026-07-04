<script lang="ts">
/**
 * SnapLinesOverlay — Renders smart alignment snap lines during element drag.
 * Shows horizontal/vertical guide lines when an element edge or center aligns
 * with another element or the page boundary.
 */
import type { SnapLine } from "@docforge/core";
import { mmToPx } from "@docforge/core";
import type { ViewportTransform } from "@docforge/core";

let {
  snapLines,
  viewport,
}: {
  snapLines: SnapLine[];
  viewport: ViewportTransform;
} = $props();
</script>

<g class="snap-lines-overlay" pointer-events="none">
  {#each snapLines as line}
    {@const isVertical = line.axis === "vertical"}
    {#if isVertical}
      <line
        x1={mmToPx(line.position, viewport.dpi, viewport.zoom)}
        y1={mmToPx(line.start, viewport.dpi, viewport.zoom)}
        x2={mmToPx(line.position, viewport.dpi, viewport.zoom)}
        y2={mmToPx(line.end, viewport.dpi, viewport.zoom)}
        stroke="#f43f5e"
        stroke-width="0.8"
        stroke-dasharray="3,3"
        opacity="0.7"
      />
      <!-- Distance marker -->
      <circle
        cx={mmToPx(line.position, viewport.dpi, viewport.zoom)}
        cy={mmToPx((line.start + line.end) / 2, viewport.dpi, viewport.zoom)}
        r="2"
        fill="#f43f5e"
        opacity="0.8"
      />
    {:else}
      <line
        x1={mmToPx(line.start, viewport.dpi, viewport.zoom)}
        y1={mmToPx(line.position, viewport.dpi, viewport.zoom)}
        x2={mmToPx(line.end, viewport.dpi, viewport.zoom)}
        y2={mmToPx(line.position, viewport.dpi, viewport.zoom)}
        stroke="#f43f5e"
        stroke-width="0.8"
        stroke-dasharray="3,3"
        opacity="0.7"
      />
      <circle
        cx={mmToPx((line.start + line.end) / 2, viewport.dpi, viewport.zoom)}
        cy={mmToPx(line.position, viewport.dpi, viewport.zoom)}
        r="2"
        fill="#f43f5e"
        opacity="0.8"
      />
    {/if}
  {/each}
</g>
