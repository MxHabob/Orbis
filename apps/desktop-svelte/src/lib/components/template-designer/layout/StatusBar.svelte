<script lang="ts">
/**
 * StatusBar — Bottom bar with zoom slider, page info, cursor position.
 * Replaces the inline status bar in CanvasRoot.
 */
import type { EditorStore } from "../stores/editor-store.svelte";
import { Button } from "@orbis/ui-svelte/button";
import {
  Tooltip,
  TooltipTrigger,
  TooltipContent,
  TooltipProvider,
} from "@orbis/ui-svelte/tooltip";
import {
  ZoomIn,
  ZoomOut,
  Maximize2,
  Magnet,
} from "lucide-svelte";

let {
  store,
  cursorMm = { x: 0, y: 0 },
  containerWidth = 800,
  containerHeight = 600,
}: {
  store: EditorStore;
  cursorMm?: { x: number; y: number };
  containerWidth?: number;
  containerHeight?: number;
} = $props();
</script>

<TooltipProvider delayDuration={200}>
<div class="status-bar">
  <!-- Left: Zoom -->
  <div class="status-group">
    <Tooltip>
      <TooltipTrigger>
        <button type="button" class="status-btn" onclick={() => store.zoomOut()}>
          <ZoomOut size={13} />
        </button>
      </TooltipTrigger>
      <TooltipContent>تصغير</TooltipContent>
    </Tooltip>

    <input
      type="range"
      min="10"
      max="400"
      value={store.zoomPercent}
      oninput={(e) => store.setZoom(parseInt(e.currentTarget.value) / 100)}
      class="zoom-slider"
    />

    <span class="status-text font-mono">{store.zoomPercent}%</span>

    <Tooltip>
      <TooltipTrigger>
        <button type="button" class="status-btn" onclick={() => store.zoomIn()}>
          <ZoomIn size={13} />
        </button>
      </TooltipTrigger>
      <TooltipContent>تكبير</TooltipContent>
    </Tooltip>

    <Tooltip>
      <TooltipTrigger>
        <button type="button" class="status-btn" onclick={() => store.zoomToFit(containerWidth, containerHeight)}>
          <Maximize2 size={12} />
        </button>
      </TooltipTrigger>
      <TooltipContent>ملء الشاشة</TooltipContent>
    </Tooltip>
  </div>

  <!-- Center: Page info -->
  <div class="status-group">
    <span class="status-text">
      الصفحة {store.activePageIndex + 1} / {store.pageCount}
    </span>
  </div>

  <!-- Right: Cursor + Snap -->
  <div class="status-group">
    <span class="status-text font-mono text-muted-foreground">
      X:{cursorMm.x.toFixed(1)}  Y:{cursorMm.y.toFixed(1)}
    </span>
    <span class="status-divider"></span>
    <Tooltip>
      <TooltipTrigger>
        <button
          type="button"
          class="status-btn"
          class:active={store.snapEnabled}
          onclick={() => store.snapEnabled = !store.snapEnabled}
        >
          <Magnet size={12} />
        </button>
      </TooltipTrigger>
      <TooltipContent>{store.snapEnabled ? "إيقاف الالتصاق" : "تفعيل الالتصاق"}</TooltipContent>
    </Tooltip>
    {#if store.selectedNode}
      <span class="status-divider"></span>
      <span class="status-text text-foreground font-medium truncate max-w-[120px]">
        {store.selectedNode.name || store.selectedNode.id}
      </span>
    {/if}
  </div>
</div>
</TooltipProvider>

<style>
  .status-bar {
    height: 28px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 10px;
    background: color-mix(in srgb, var(--color-card) 90%, transparent);
    border-top: 1px solid color-mix(in srgb, var(--color-border) 25%, transparent);
    flex-shrink: 0;
    user-select: none;
    backdrop-filter: blur(8px);
    z-index: 10;
  }

  .status-group {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .status-text {
    font-size: 10px;
    color: var(--color-muted-foreground);
    white-space: nowrap;
  }

  .status-btn {
    width: 22px;
    height: 22px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: transparent;
    border: none;
    border-radius: 4px;
    color: var(--color-muted-foreground);
    cursor: pointer;
    transition: all 0.1s;
  }

  .status-btn:hover {
    background: var(--color-accent);
    color: var(--color-foreground);
  }

  .status-btn.active {
    color: var(--color-primary);
  }

  .status-divider {
    width: 1px;
    height: 14px;
    background: color-mix(in srgb, var(--color-border) 25%, transparent);
  }

  .zoom-slider {
    width: 80px;
    height: 16px;
    -webkit-appearance: none;
    appearance: none;
    background: transparent;
    border-radius: 3px;
    outline: none;
    cursor: pointer;
    position: relative;
  }

  .zoom-slider::-webkit-slider-runnable-track {
    height: 3px;
    background: color-mix(in srgb, var(--color-muted-foreground) 25%, transparent);
    border-radius: 3px;
  }

  .zoom-slider::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
    width: 12px;
    height: 12px;
    border-radius: 50%;
    background: var(--color-foreground);
    cursor: pointer;
    margin-top: -4.5px;
    transition: transform 0.1s;
  }

  .zoom-slider::-webkit-slider-thumb:hover {
    transform: scale(1.2);
  }
</style>
