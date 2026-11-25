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
import { onDestroy, onMount } from "svelte";
import * as m from "$lib/paraglide/messages";

let {
  viewport,
  label,
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
  label?: string;
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
let width = $state<number>();
let startX = 0;
let startWidth = 0;
let requestedWidth: number | undefined;
let pointerId: number | undefined;
let draggedViewport: HTMLElement | undefined;
let growsTowardRight = false;
let rootFontSize = 16;

function restorePersistedWidth() {
  if (!persistId) return;
  const pxStorageKey = `resizer:${persistId}:px`;
  const legacyStorageKey = `resizer:${persistId}`;
  const legacyViewportKey = `resizer:viewport-${persistId.replace(/^viewport-/, "")}`;
  let stored = localStorage.getItem(pxStorageKey);
  let isPixelValue = true;
  if (!stored) {
    stored = localStorage.getItem(legacyStorageKey);
    isPixelValue = false;
  }
  if (!stored) {
    stored = localStorage.getItem(legacyViewportKey);
    isPixelValue = false;
  }
  if (stored) {
    const val = parseFloat(stored);
    const base =
      parseFloat(getComputedStyle(document.documentElement).fontSize) || 16;
    const valRem = isPixelValue ? val / base : val;
    if (
      !isNaN(valRem) &&
      ((collapsible && valRem === collapsedWidth) ||
        (valRem >= minWidth && valRem <= maxWidth))
    ) {
      reportWidth(valRem);
    }
  }
}

onMount(restorePersistedWidth);
onDestroy(finishDrag);

function reportWidth(value: number) {
  width = value;
  onWidth?.(value);
}

function persistWidth(value: number) {
  if (persistId) {
    const base = parseFloat(getComputedStyle(document.documentElement).fontSize) || 16;
    localStorage.setItem(`resizer:${persistId}:px`, String(value * base));
  }
}

function growsTowardRightForViewport() {
  const isRtl = viewport && getComputedStyle(viewport).direction === "rtl";
  return edge === "end" ? !isRtl : Boolean(isRtl);
}

function onKeyDown(event: KeyboardEvent) {
  if (!viewport || isDragging || event.altKey || event.ctrlKey || event.metaKey) return;
  const base = parseFloat(getComputedStyle(document.documentElement).fontSize) || 16;
  const current = viewport.getBoundingClientRect().width / base;
  const collapsed = collapsible && current <= collapsedWidth;
  let next: number;
  switch (event.key) {
    case "ArrowLeft":
    case "ArrowRight": {
      const grows = (event.key === "ArrowRight") === growsTowardRightForViewport();
      next = !grows && collapsed
        ? collapsedWidth
        : Math.max(minWidth, Math.min(maxWidth, current + (grows ? 1 : -1)));
      break;
    }
    case "Home": next = collapsible ? collapsedWidth : minWidth; break;
    case "End": next = maxWidth; break;
    case "Enter": next = collapsible && !collapsed ? collapsedWidth : defaultValue; break;
    default: return;
  }
  event.preventDefault();
  reportWidth(next);
  persistWidth(next);
}

function onPointerDown(e: PointerEvent) {
  if (!viewport || pointerId !== undefined || !e.isPrimary || e.button !== 0) return;
  e.preventDefault();
  const target = e.currentTarget as HTMLElement;
  target.setPointerCapture(e.pointerId);
  pointerId = e.pointerId;
  draggedViewport = viewport;
  isDragging = true;
  startX = e.clientX;
  startWidth = viewport.getBoundingClientRect().width;
  growsTowardRight = growsTowardRightForViewport();
  rootFontSize = parseFloat(getComputedStyle(document.documentElement).fontSize) || 16;
  requestedWidth = undefined;

  document.body.style.cursor = "ew-resize";
  document.body.style.userSelect = "none";
  viewport.classList.add("resizer-dragging");
}

function onPointerMove(e: PointerEvent) {
  if (e.pointerId !== pointerId) return;
  const delta = growsTowardRight ? e.clientX - startX : startX - e.clientX;
  const newWidthPx = startWidth + delta;
  const newWidthRem = newWidthPx / rootFontSize;

  let targetWidth = newWidthRem;
  if (collapsible && targetWidth < collapseThreshold) {
    targetWidth = collapsedWidth;
  } else {
    targetWidth = Math.max(minWidth, Math.min(maxWidth, targetWidth));
  }

  requestedWidth = targetWidth;
  reportWidth(targetWidth);
}

function finishDrag() {
  if (pointerId === undefined) return;
  pointerId = undefined;
  isDragging = false;

  document.body.style.cursor = "";
  document.body.style.userSelect = "";

  draggedViewport?.classList.remove("resizer-dragging");
  draggedViewport = undefined;

  if (requestedWidth !== undefined) persistWidth(requestedWidth);
}

function onPointerEnd(event: PointerEvent) {
  if (event.pointerId === pointerId) finishDrag();
}

function onDoubleClick() {
  reportWidth(defaultValue);
  persistWidth(defaultValue);
}
</script>

<!-- A movable ARIA separator is a keyboard-focusable widget. -->
<!-- svelte-ignore a11y_no_noninteractive_tabindex, a11y_no_noninteractive_element_interactions -->
<div
  role="separator"
  tabindex="0"
  aria-orientation="vertical"
  aria-label={label ?? m.shared_ui_resize_panel()}
  aria-valuemin={collapsible ? collapsedWidth : minWidth}
  aria-valuemax={maxWidth}
  aria-valuenow={Math.min(maxWidth, Math.max(collapsible ? collapsedWidth : minWidth, width ?? defaultValue))}
  onkeydown={onKeyDown}
	class="resizer"
  class:start={edge === 'start'}
  class:end={edge === 'end'}
  class:inside
	class:dragging={isDragging}
	class:cursor-col-resize={isDragging}
	onpointerdown={onPointerDown}
	onpointermove={onPointerMove}
	onpointerup={onPointerEnd}
	onpointercancel={onPointerEnd}
	onlostpointercapture={onPointerEnd}
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

	.resizer:focus-visible .resizer-line,
	.resizer:hover .resizer-line {
		background-color: hsl(260 70% 50% / 0.3);
	}

	.resizer.dragging .resizer-line,
	.resizer:active .resizer-line {
		background-color: hsl(260 70% 50% / 0.6);
	}
</style>
