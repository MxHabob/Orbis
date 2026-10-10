<!--
@component Resizer
Draggable handle for resizing panels in a split-pane layout.
Persists the user's preferred width to localStorage.

Inspired by GitButler's Resizer component.

@example
```svelte
<Resizer
  viewport={panelEl}
  edge="end"
  minWidth={16}
  maxWidth={40}
  defaultValue={24}
  persistId="sidebar-width"
  onWidth={(w) => (preferredWidth = w)}
/>
```
-->
<script lang="ts">
let {
  viewport,
  edge,
  minWidth,
  maxWidth,
  defaultValue,
  persistId,
  inside = false,
  collapsible = true,
  onWidth,
  collapseThreshold = minWidth * 0.75,
  collapsedWidth = 3,
}: {
  viewport: HTMLElement | undefined;
  edge: "start" | "end";
  minWidth: number;
  maxWidth: number;
  defaultValue: number;
  persistId?: string;
  inside?: boolean;
  collapsible?: boolean;
  onWidth?: (widthRem: number) => void;
  collapseThreshold?: number;
  collapsedWidth?: number;
} = $props();

let isDragging = $state(false);
let startX = 0;
let startWidth = 0;
let dragTarget: HTMLElement | null = null;

// Restore persisted width on mount
$effect(() => {
  if (!persistId) return;
  const stored = localStorage.getItem(`resizer:${persistId}`);
  if (stored) {
    const val = parseFloat(stored);
    if (
      !isNaN(val) &&
      ((collapsible && val === collapsedWidth) ||
        (val >= minWidth && val <= maxWidth))
    ) {
      onWidth?.(val);
    }
  }
});

function pxToRem(px: number): number {
  const base = parseFloat(getComputedStyle(document.documentElement).fontSize);
  return px / base;
}

function onPointerDown(e: PointerEvent) {
  if (!viewport) return;
  e.preventDefault();
  isDragging = true;
  startX = e.clientX;
  startWidth = viewport.getBoundingClientRect().width;

  dragTarget = e.currentTarget as HTMLElement;
  dragTarget.setPointerCapture(e.pointerId);

  document.body.style.cursor = "ew-resize";
  document.body.style.userSelect = "none";
  viewport.classList.add("resizer-dragging");
}

function onPointerMove(e: PointerEvent) {
  if (!isDragging || !viewport) return;

  const isRtl = getComputedStyle(viewport).direction === "rtl";
  const growsTowardRight = edge === "end" ? !isRtl : isRtl;
  const delta = growsTowardRight ? e.clientX - startX : startX - e.clientX;
  const newWidthPx = startWidth + delta;
  const newWidthRem = pxToRem(newWidthPx);

  let targetWidth = newWidthRem;
  if (collapsible && targetWidth < collapseThreshold) {
    targetWidth = collapsedWidth;
  } else {
    targetWidth = Math.max(minWidth, Math.min(maxWidth, targetWidth));
  }

  onWidth?.(targetWidth);
}

function onPointerUp(e: PointerEvent) {
  if (!isDragging) return;
  isDragging = false;

  document.body.style.cursor = "";
  document.body.style.userSelect = "";

  if (dragTarget && dragTarget.hasPointerCapture(e.pointerId)) {
    dragTarget.releasePointerCapture(e.pointerId);
  }
  dragTarget = null;

  if (viewport) {
    viewport.classList.remove("resizer-dragging");
  }

  // Persist final width
  if (persistId && viewport) {
    const currentRem = pxToRem(viewport.getBoundingClientRect().width);
    let targetWidth = currentRem;
    if (collapsible && targetWidth < collapseThreshold) {
      targetWidth = collapsedWidth;
    } else {
      targetWidth = Math.max(minWidth, Math.min(maxWidth, targetWidth));
    }
    localStorage.setItem(`resizer:${persistId}`, String(targetWidth));
  }
}

function onDoubleClick() {
  onWidth?.(defaultValue);
  if (persistId) {
    localStorage.setItem(`resizer:${persistId}`, String(defaultValue));
  }
}
</script>

<svelte:window on:pointermove={onPointerMove} on:pointerup={onPointerUp} />

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
	class="resizer"
  class:start={edge === 'start'}
  class:end={edge === 'end'}
  class:inside
	class:dragging={isDragging}
	class:cursor-col-resize={isDragging}
	onpointerdown={onPointerDown}
	ondblclick={onDoubleClick}
>
	<div class="resizer-line"></div>
</div>

<style>
	.resizer {
		position: absolute;
		top: 0;
		z-index: 2;
		width: 10px;
		height: 100%;
		touch-action: none;
		cursor: ew-resize;
	}

	:global(.desktop) .resizer {
		cursor: ew-resize !important;
	}

  .resizer.end {
    inset-inline-end: -5px;
	}

  .resizer.start {
		inset-inline-start: -5px;
	}

  .resizer.start.inside {
    inset-inline-start: 0;
  }

  .resizer.end.inside {
    inset-inline-end: 0;
  }

	.resizer-line {
		position: absolute;
		top: 8px;
		bottom: 8px;
		left: 50%;
		width: 2px;
		transform: translateX(-50%);
		border-radius: 1px;
		background-color: transparent;
		transition: background-color 0.15s ease;
	}

	.resizer:hover .resizer-line {
		background-color: hsl(260 70% 50% / 0.3);
	}

	.resizer.dragging .resizer-line,
	.resizer:active .resizer-line {
		background-color: hsl(260 70% 50% / 0.6);
	}
</style>
