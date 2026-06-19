<!--
@component Resizer
Draggable handle for resizing panels in a split-pane layout.
Persists the user's preferred width to localStorage.

Inspired by GitButler's Resizer component.

@example
```svelte
<Resizer
  viewport={panelEl}
  direction="right"
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
		direction,
		minWidth,
		maxWidth,
		defaultValue,
		persistId,
		onWidth,
	}: {
		viewport: HTMLElement | undefined;
		direction: 'left' | 'right';
		minWidth: number;
		maxWidth: number;
		defaultValue: number;
		persistId?: string;
		onWidth?: (widthRem: number) => void;
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
			if (!isNaN(val) && val >= minWidth && val <= maxWidth) {
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

		document.body.style.cursor = 'ew-resize';
		document.body.style.userSelect = 'none';
	}

	function onPointerMove(e: PointerEvent) {
		if (!isDragging || !viewport) return;

		const delta = direction === 'right' ? e.clientX - startX : startX - e.clientX;
		const newWidthPx = startWidth + delta;
		const newWidthRem = pxToRem(newWidthPx);
		const clamped = Math.max(minWidth, Math.min(maxWidth, newWidthRem));

		onWidth?.(clamped);
	}

	function onPointerUp(e: PointerEvent) {
		if (!isDragging) return;
		isDragging = false;

		document.body.style.cursor = '';
		document.body.style.userSelect = '';

		if (dragTarget && dragTarget.hasPointerCapture(e.pointerId)) {
			dragTarget.releasePointerCapture(e.pointerId);
		}
		dragTarget = null;

		// Persist final width
		if (persistId && viewport) {
			const currentRem = pxToRem(viewport.getBoundingClientRect().width);
			const clamped = Math.max(minWidth, Math.min(maxWidth, currentRem));
			localStorage.setItem(`resizer:${persistId}`, String(clamped));
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
	class:right={direction === 'right'}
	class:left={direction === 'left'}
	class:dragging={isDragging}
	onpointerdown={onPointerDown}
	ondblclick={onDoubleClick}
>
	<div class="resizer-line"></div>
</div>

<style>
	.resizer {
		position: absolute;
		top: 0;
		z-index: var(--z-lifted, 2);
		width: 10px;
		height: 100%;
		touch-action: none;
		cursor: ew-resize;
	}

	:global(.desktop) .resizer {
		cursor: ew-resize !important;
	}

	.resizer.right {
		right: -5px;
	}

	.resizer.left {
		left: -5px;
	}

	.resizer-line {
		position: absolute;
		top: 8px;
		bottom: 8px;
		left: 50%;
		width: 2px;
		transform: translateX(-50%);
		border-radius: 1px;
		background-color: var(--resizer-idle);
		transition: background-color var(--transition-medium);
	}

	.resizer:hover .resizer-line {
		background-color: var(--resizer-hover);
	}

	.resizer.dragging .resizer-line,
	.resizer:active .resizer-line {
		background-color: var(--resizer-active);
	}
</style>
