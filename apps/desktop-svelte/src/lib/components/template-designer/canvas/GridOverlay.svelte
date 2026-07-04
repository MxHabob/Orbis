<script lang="ts">
/**
 * GridOverlay — Renders a dot or line grid on the canvas page.
 */
import type { ViewportTransform } from "@docforge/core";
import { mmToPx } from "@docforge/core";

let {
  widthPx,
  heightPx,
  gridSizeMm,
  viewport,
}: {
  widthPx: number;
  heightPx: number;
  gridSizeMm: number;
  viewport: ViewportTransform;
} = $props();

const gridStepPx = $derived(mmToPx(gridSizeMm, viewport.dpi, viewport.zoom));
const patternId = $derived(`grid-${gridSizeMm}`);
</script>

<defs>
  <pattern
    id={patternId}
    width={gridStepPx}
    height={gridStepPx}
    patternUnits="userSpaceOnUse"
  >
    <circle
      cx={gridStepPx / 2}
      cy={gridStepPx / 2}
      r="0.7"
      fill="#94a3b8"
      opacity="0.25"
    />
  </pattern>
</defs>

<rect
  width={widthPx}
  height={heightPx}
  fill="url(#{patternId})"
  pointer-events="none"
/>
