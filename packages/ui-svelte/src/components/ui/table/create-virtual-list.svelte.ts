// ── createVirtualList ───────────────────────────────────────────────
// Lightweight row virtualizer for Svelte 5.
// Renders only visible rows + overscan for 60fps scrolling on 10k+ rows.
// Inspired by @tanstack/react-virtual but built natively with $state/$derived.

import type {
  VirtualItem,
  VirtualListOptions,
  VirtualListState,
} from "./table-types.js";

/**
 * Creates a reactive virtualized list.
 *
 * @example
 * ```svelte
 * <script>
 * const virtual = createVirtualList({
 *   count: rows.length,
 *   estimateSize: 45,
 *   overscan: 10,
 *   getScrollElement: () => containerEl,
 * });
 * </script>
 *
 * <div bind:this={containerEl} style="height:600px; overflow:auto">
 *   <div style="height:{virtual.totalSize}px; position:relative">
 *     {#each virtual.virtualItems as item (item.index)}
 *       <div style="position:absolute; top:{item.start}px; height:{item.size}px; width:100%">
 *         {rows[item.index].name}
 *       </div>
 *     {/each}
 *   </div>
 * </div>
 * ```
 */
export function createVirtualList(
  options: VirtualListOptions,
): VirtualListState {
  // ── Internal State ──────────────────────────────────────────────

  let scrollTop = $state(0);
  let containerHeight = $state(0);
  const overscan = options.overscan ?? 10;

  // ── Size Resolution ─────────────────────────────────────────────

  function getItemSize(index: number): number {
    if (typeof options.estimateSize === "function") {
      return options.estimateSize(index);
    }
    return options.estimateSize;
  }

  // ── Derived: Total Scrollable Height ────────────────────────────

  const totalSize = $derived.by(() => {
    let total = 0;
    for (let i = 0; i < options.count; i++) {
      total += getItemSize(i);
    }
    return total;
  });

  // ── Derived: Visible Items ──────────────────────────────────────

  const virtualItems = $derived.by((): VirtualItem[] => {
    if (options.count === 0 || containerHeight === 0) return [];

    const items: VirtualItem[] = [];
    let offset = 0;

    // Find the first visible index
    let startIndex = 0;
    for (let i = 0; i < options.count; i++) {
      const size = getItemSize(i);
      if (offset + size > scrollTop) {
        startIndex = i;
        break;
      }
      offset += size;
      if (i === options.count - 1) {
        startIndex = i;
      }
    }

    // Apply overscan above
    const overscanStart = Math.max(0, startIndex - overscan);

    // Recalculate offset for overscanStart
    offset = 0;
    for (let i = 0; i < overscanStart; i++) {
      offset += getItemSize(i);
    }

    // Render from overscanStart until we pass the viewport bottom + overscan
    const viewportBottom = scrollTop + containerHeight;
    let endReached = false;
    let overscanRemaining = overscan;

    for (let i = overscanStart; i < options.count; i++) {
      const size = getItemSize(i);
      items.push({ index: i, start: offset, size });
      offset += size;

      // If we've passed the viewport bottom, count down overscan
      if (offset > viewportBottom) {
        if (endReached) {
          overscanRemaining--;
          if (overscanRemaining <= 0) break;
        } else {
          endReached = true;
        }
      }
    }

    return items;
  });

  // ── Scroll Event Binding ────────────────────────────────────────

  let scrollElement: HTMLElement | null = null;
  let resizeObserver: ResizeObserver | null = null;
  let ticking = false;

  function onScroll() {
    if (!ticking) {
      ticking = true;
      requestAnimationFrame(() => {
        if (scrollElement) {
          scrollTop = scrollElement.scrollTop;
        }
        ticking = false;
      });
    }
  }

  function attach() {
    scrollElement = options.getScrollElement();
    if (!scrollElement) return;

    containerHeight = scrollElement.clientHeight;
    scrollTop = scrollElement.scrollTop;

    scrollElement.addEventListener("scroll", onScroll, { passive: true });

    resizeObserver = new ResizeObserver((entries) => {
      for (const entry of entries) {
        containerHeight = entry.contentRect.height;
      }
    });
    resizeObserver.observe(scrollElement);
  }

  function detach() {
    if (scrollElement) {
      scrollElement.removeEventListener("scroll", onScroll);
    }
    if (resizeObserver) {
      resizeObserver.disconnect();
      resizeObserver = null;
    }
    scrollElement = null;
  }

  // Auto-attach when getScrollElement returns an element
  // In Svelte 5, use $effect in the consuming component to call attach/detach.
  // But we also expose them so the DataTable can call them in onMount.

  // ── Scroll To Index ─────────────────────────────────────────────

  function scrollToIndex(index: number, behavior: ScrollBehavior = "smooth") {
    const el = options.getScrollElement();
    if (!el || index < 0 || index >= options.count) return;

    let offset = 0;
    for (let i = 0; i < index; i++) {
      offset += getItemSize(i);
    }

    el.scrollTo({ top: offset, behavior });
  }

  // ── Public API ──────────────────────────────────────────────────

  return {
    get virtualItems() {
      return virtualItems;
    },
    get totalSize() {
      return totalSize;
    },
    scrollToIndex,
    /** Call in onMount to start observing scroll */
    attach,
    /** Call in onDestroy to cleanup */
    detach,
  } as VirtualListState & { attach: () => void; detach: () => void };
}
