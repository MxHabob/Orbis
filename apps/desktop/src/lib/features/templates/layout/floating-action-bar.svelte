<script lang="ts">
import * as m from "$lib/paraglide/messages";
/**
 * FloatingActionBar — Appears above the selected element on the canvas.
 * Shows quick actions: delete, duplicate, copy, lock, layer ordering, more.
 * Positioned relative to the element's bounding box in screen coordinates.
 */
import type { EditorStore } from "../stores/editor-store.svelte";
import { mmToPx } from "@docforge/core/canvas";
import type { Position } from "@docforge/core";
import {
  Tooltip,
  TooltipTrigger,
  TooltipContent,
  TooltipProvider,
} from "@orbis/ui/tooltip";
import {
  AdjustmentsHorizontal,
  BarsArrowUp,
  Clipboard,
  EllipsisHorizontal,
  Lock,
  Trash,
} from "@orbis/icons";

let {
  store,
  offsetX = 0,
  offsetY = 0,
  pageSizePx,
  containerSize,
  rulersVisible = false,
}: {
  store: EditorStore;
  offsetX?: number;
  offsetY?: number;
  pageSizePx: { width: number; height: number };
  containerSize: { width: number; height: number };
  rulersVisible?: boolean;
} = $props();

const node = $derived(store.selectedNode);
const hasSelection = $derived(store.selectedIds.length === 1 && node);
const vp = $derived(store.viewport);

// Calculate position in screen px based on element bounds
const pos = $derived.by(() => {
  if (!node) return null;
  const p =
    ("props" in node
      ? (node.props as { position?: Position }).position
      : undefined) ?? (node as { position?: Position }).position;
  if (!p) return null;

  const pageOriginX = offsetX + (rulersVisible ? 24 : 0);
  const pageOriginY = offsetY + (rulersVisible ? 24 : 0);
  const x = pageOriginX + mmToPx(p.x ?? 0, vp.dpi, vp.zoom);
  const y = pageOriginY + mmToPx(p.y ?? 0, vp.dpi, vp.zoom);
  const w =
    typeof p.width === "string" && p.width.endsWith("%")
      ? (Number.parseFloat(p.width) / 100) * pageSizePx.width
      : mmToPx(Number(p.width) || 50, vp.dpi, vp.zoom);
  const h =
    p.height == null
      ? mmToPx(20, vp.dpi, vp.zoom)
      : typeof p.height === "string" && p.height.endsWith("%")
        ? (Number.parseFloat(p.height) / 100) * pageSizePx.height
        : mmToPx(Number(p.height) || 20, vp.dpi, vp.zoom);
  const halfBarWidth = 92;
  const localX = x + w / 2;
  const clampedX = Math.max(
    halfBarWidth,
    Math.min(containerSize.width - halfBarWidth, localX),
  );
  const aboveY = y - 40;
  const localY = aboveY >= (rulersVisible ? 24 : 0) ? aboveY : y + h + 8;

  return {
    x: clampedX,
    y: Math.min(localY, containerSize.height - 40),
  };
});

const isLocked = $derived(node?.locked ?? false);
</script>

{#if hasSelection && pos}
  <TooltipProvider delayDuration={200}>
    <div
      class="fab-container"
      style="left: {pos.x}px; top: {pos.y}px;"
    >
      <Tooltip>
        <TooltipTrigger>
          <button type="button" class="fab-btn" onclick={() => store.duplicateSelected()}>
            <Clipboard size={13} />
          </button>
        </TooltipTrigger>
        <TooltipContent>{m.template_tooltip_duplicate()}</TooltipContent>
      </Tooltip>

      <Tooltip>
        <TooltipTrigger>
          <button type="button" class="fab-btn" onclick={() => store.deleteSelected()}>
            <Trash size={13} />
          </button>
        </TooltipTrigger>
        <TooltipContent>{m.template_tooltip_delete()}</TooltipContent>
      </Tooltip>

      <Tooltip>
        <TooltipTrigger>
          <button type="button" class="fab-btn" class:active={isLocked} onclick={() => store.updateSelectedProps({ locked: !isLocked })}>
            {#if isLocked}
              <Lock size={13} />
            {:else}
              <Lock size={13} />
            {/if}
          </button>
        </TooltipTrigger>
        <TooltipContent>{isLocked ? m.template_tooltip_unlock() : m.template_tooltip_lock()}</TooltipContent>
      </Tooltip>

      <div class="fab-divider"></div>

      <Tooltip>
        <TooltipTrigger>
          <button type="button" class="fab-btn" onclick={() => store.bringToFront()}>
            <BarsArrowUp size={13} />
          </button>
        </TooltipTrigger>
        <TooltipContent>{m.template_tooltip_bring_forward()}</TooltipContent>
      </Tooltip>

      <Tooltip>
        <TooltipTrigger>
          <button type="button" class="fab-btn" onclick={() => store.sendToBack()}>
            <BarsArrowUp size={13} />
          </button>
        </TooltipTrigger>
        <TooltipContent>{m.template_tooltip_send_backward()}</TooltipContent>
      </Tooltip>
    </div>
  </TooltipProvider>
{/if}

<style>
  .fab-container {
    position: absolute;
    transform: translateX(-50%);
    display: flex;
    align-items: center;
    gap: 1px;
    background: var(--color-popover);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    padding: 4px;
    box-shadow: 0 2px 8px color-mix(in srgb, var(--color-foreground) 12%, transparent);
    z-index: 50;
    pointer-events: auto;
    animation: fab-pop 0.15s ease-out;
  }

  @keyframes fab-pop {
    from { opacity: 0; transform: translateX(-50%) translateY(4px) scale(0.95); }
    to { opacity: 1; transform: translateX(-50%) translateY(0) scale(1); }
  }

  .fab-btn {
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: transparent;
    border: none;
    border-radius: var(--radius-sm);
    color: var(--color-muted-foreground);
    cursor: pointer;
    transition: all 0.1s;
  }

  .fab-btn:hover {
    background: var(--color-accent);
    color: var(--color-foreground);
  }

  .fab-btn.active {
    color: var(--color-primary);
  }

  .fab-divider {
    width: 1px;
    height: 18px;
    background: color-mix(in srgb, var(--color-border) 30%, transparent);
    margin: 0 2px;
  }
</style>
