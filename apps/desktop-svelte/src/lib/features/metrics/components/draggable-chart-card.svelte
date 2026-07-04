<script lang="ts">
import type { Snippet } from "svelte";
import type { ColSpan } from "../utils/chart-types";

let {
  id,
  isEditing = false,
  onResize,
  children,
}: {
  id: string;
  isEditing?: boolean;
  onResize: (colSpan: ColSpan) => void;
  children: Snippet;
} = $props();

let cardEl = $state<HTMLDivElement | null>(null);

const SNAP_THRESHOLDS: { max: number; colSpan: ColSpan }[] = [
  { max: 5 / 12, colSpan: 4 },
  { max: 7 / 12, colSpan: 6 },
  { max: 10 / 12, colSpan: 8 },
  { max: Infinity, colSpan: 12 },
];

function snapToColSpan(fraction: number): ColSpan {
  for (const { max, colSpan } of SNAP_THRESHOLDS) {
    if (fraction < max) return colSpan;
  }
  return 12;
}

function startResize(e: PointerEvent, side: "left" | "right") {
  if (!isEditing) return;
  e.stopPropagation();
  e.preventDefault();

  if (!cardEl) return;

  const gridEl = cardEl.closest("[data-metrics-grid]") as HTMLElement;
  if (!gridEl) return;

  const gridWidth = gridEl.getBoundingClientRect().width;
  const initialWidth = cardEl.getBoundingClientRect().width;
  const initialMouseX = e.clientX;
  let currentSnap: ColSpan | null = null;

  const onPointerMove = (moveEvent: PointerEvent) => {
    const delta = moveEvent.clientX - initialMouseX;
    const desiredWidth =
      side === "right" ? initialWidth + delta : initialWidth - delta;
    const fraction = Math.max(0.1, desiredWidth / gridWidth);
    const snapped = snapToColSpan(fraction);

    if (snapped !== currentSnap) {
      currentSnap = snapped;
      onResize(snapped);
    }
  };

  const onPointerUp = () => {
    document.removeEventListener("pointermove", onPointerMove);
    document.removeEventListener("pointerup", onPointerUp);
    document.body.style.cursor = "";
    document.body.style.userSelect = "";
  };

  document.body.style.cursor = "col-resize";
  document.body.style.userSelect = "none";
  document.addEventListener("pointermove", onPointerMove);
  document.addEventListener("pointerup", onPointerUp);
}
</script>

<div
	{id}
	bind:this={cardEl}
	class="relative transition-opacity duration-150 scroll-mt-24"
>
	{@render children()}

	{#if isEditing}
		<!-- Resize Handles -->
		<div
			class="absolute inset-y-0 left-0 w-4 z-20 cursor-col-resize group/resize-l select-none"
			onpointerdown={(e) => startResize(e, "left")}
			role="presentation"
		>
			<div class="absolute top-1/2 -translate-y-1/2 left-0.5 w-1 h-8 rounded-full bg-border opacity-60 group-hover/resize-l:opacity-100 transition-opacity"></div>
		</div>
		<div
			class="absolute inset-y-0 right-0 w-4 z-20 cursor-col-resize group/resize-r select-none"
			onpointerdown={(e) => startResize(e, "right")}
			role="presentation"
		>
			<div class="absolute top-1/2 -translate-y-1/2 right-0.5 w-1 h-8 rounded-full bg-border opacity-60 group-hover/resize-r:opacity-100 transition-opacity"></div>
		</div>
	{/if}
</div>
