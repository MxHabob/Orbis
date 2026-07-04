<script lang="ts">
/**
 * SnapLinesOverlay — Enhanced snap lines with color coding.
 *
 * Colors:
 *   - Center alignment: magenta/pink (#ec4899)
 *   - Edge alignment: cyan/blue (#06b6d4)
 *   - Page boundary: green (#22c55e)
 *
 * Shows distance markers at midpoints.
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

// Color based on snap type
function getSnapColor(line: SnapLine): string {
  const type = (line as any).snapType || "edge";
  switch (type) {
    case "center": return "#ec4899"; // pink — center alignment
    case "page": return "#22c55e";   // green — page boundary
    default: return "#06b6d4";       // cyan — edge alignment
  }
}
</script>

<g class="snap-lines-overlay" pointer-events="none">
  {#each snapLines as line}
    {@const isVertical = line.axis === "vertical"}
    {@const color = getSnapColor(line)}
    {#if isVertical}
      <line
        x1={mmToPx(line.position, viewport.dpi, viewport.zoom)}
        y1={mmToPx(line.start, viewport.dpi, viewport.zoom)}
        x2={mmToPx(line.position, viewport.dpi, viewport.zoom)}
        y2={mmToPx(line.end, viewport.dpi, viewport.zoom)}
        stroke={color}
        stroke-width="0.8"
        stroke-dasharray="3,3"
        opacity="0.8"
      />
      <!-- Distance marker -->
      <circle
        cx={mmToPx(line.position, viewport.dpi, viewport.zoom)}
        cy={mmToPx((line.start + line.end) / 2, viewport.dpi, viewport.zoom)}
        r="2.5"
        fill={color}
        opacity="0.9"
      />
    {:else}
      <line
        x1={mmToPx(line.start, viewport.dpi, viewport.zoom)}
        y1={mmToPx(line.position, viewport.dpi, viewport.zoom)}
        x2={mmToPx(line.end, viewport.dpi, viewport.zoom)}
        y2={mmToPx(line.position, viewport.dpi, viewport.zoom)}
        stroke={color}
        stroke-width="0.8"
        stroke-dasharray="3,3"
        opacity="0.8"
      />
      <circle
        cx={mmToPx((line.start + line.end) / 2, viewport.dpi, viewport.zoom)}
        cy={mmToPx(line.position, viewport.dpi, viewport.zoom)}
        r="2.5"
        fill={color}
        opacity="0.9"
      />
    {/if}
  {/each}
</g>
