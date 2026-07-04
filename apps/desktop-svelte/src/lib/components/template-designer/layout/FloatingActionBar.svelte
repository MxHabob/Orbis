<script lang="ts">
/**
 * FloatingActionBar — Appears above the selected element on the canvas.
 * Shows quick actions: delete, duplicate, copy, lock, layer ordering, more.
 * Positioned relative to the element's bounding box in screen coordinates.
 */
import type { EditorStore } from "../stores/editor-store.svelte";
import { mmToPx } from "@docforge/core";
import {
  Tooltip,
  TooltipTrigger,
  TooltipContent,
  TooltipProvider,
} from "@orbis/ui-svelte/tooltip";
import {
  Trash2,
  Copy,
  ClipboardCopy,
  Lock,
  Unlock,
  ArrowUpToLine,
  ArrowDownToLine,
  MoreHorizontal,
} from "lucide-svelte";

let {
  store,
  offsetX = 0,
  offsetY = 0,
  canvasRect,
}: {
  store: EditorStore;
  offsetX?: number;
  offsetY?: number;
  canvasRect?: DOMRect;
} = $props();

const node = $derived(store.selectedNode);
const hasSelection = $derived(store.selectedIds.length === 1 && node);
const vp = $derived(store.viewport);

// Calculate position in screen px based on element bounds
const pos = $derived.by(() => {
  if (!node || !("position" in node)) return null;
  const p = (node as any).position;
  if (!p) return null;

  const x = mmToPx(p.x ?? 0, vp.dpi, vp.zoom) + offsetX;
  const y = mmToPx(p.y ?? 0, vp.dpi, vp.zoom) + offsetY;
  const w = typeof p.width === "number" ? mmToPx(p.width, vp.dpi, vp.zoom) : 100;

  return {
    x: x + w / 2, // center horizontally
    y: y - 40,     // above the element
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
            <Copy size={13} />
          </button>
        </TooltipTrigger>
        <TooltipContent>تكرار</TooltipContent>
      </Tooltip>

      <Tooltip>
        <TooltipTrigger>
          <button type="button" class="fab-btn" onclick={() => store.deleteSelected()}>
            <Trash2 size={13} />
          </button>
        </TooltipTrigger>
        <TooltipContent>حذف</TooltipContent>
      </Tooltip>

      <Tooltip>
        <TooltipTrigger>
          <button type="button" class="fab-btn" class:active={isLocked} onclick={() => store.updateSelectedProps({ locked: !isLocked })}>
            {#if isLocked}
              <Lock size={13} />
            {:else}
              <Unlock size={13} />
            {/if}
          </button>
        </TooltipTrigger>
        <TooltipContent>{isLocked ? "فتح القفل" : "قفل"}</TooltipContent>
      </Tooltip>

      <div class="fab-divider"></div>

      <Tooltip>
        <TooltipTrigger>
          <button type="button" class="fab-btn" onclick={() => store.bringToFront()}>
            <ArrowUpToLine size={13} />
          </button>
        </TooltipTrigger>
        <TooltipContent>للأمام</TooltipContent>
      </Tooltip>

      <Tooltip>
        <TooltipTrigger>
          <button type="button" class="fab-btn" onclick={() => store.sendToBack()}>
            <ArrowDownToLine size={13} />
          </button>
        </TooltipTrigger>
        <TooltipContent>للخلف</TooltipContent>
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
    border: 1px solid color-mix(in srgb, var(--color-border) 40%, transparent);
    border-radius: 8px;
    padding: 3px;
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.25);
    z-index: 50;
    pointer-events: auto;
    animation: fab-pop 0.15s ease-out;
  }

  @keyframes fab-pop {
    from { opacity: 0; transform: translateX(-50%) translateY(4px) scale(0.95); }
    to { opacity: 1; transform: translateX(-50%) translateY(0) scale(1); }
  }

  .fab-btn {
    width: 28px;
    height: 28px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: transparent;
    border: none;
    border-radius: 5px;
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
