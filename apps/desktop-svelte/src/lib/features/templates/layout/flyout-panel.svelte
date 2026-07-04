<script lang="ts">
/**
 * FlyoutPanel — Sliding panel overlay.
 * Slides from the side rail over the canvas.
 * Contains the active panel content.
 */
import type { EditorStore } from "../stores/editor-store.svelte";
import { Button } from "@orbis/ui-svelte/button";
import { X } from "lucide-svelte";

// Panels
import AssetPanel from "../panels/AssetPanel.svelte";
import LayersPanel from "../panels/LayersPanel.svelte";
import VariablesPanel from "../panels/VariablesPanel.svelte";
import HistoryPanel from "../panels/HistoryPanel.svelte";
import ElementsPanel from "../panels/ElementsPanel.svelte";
import TextPresetsPanel from "../panels/TextPresetsPanel.svelte";
import PageSetupPanel from "../panels/PageSetupPanel.svelte";
import {
  Tabs,
  TabsList,
  TabsTrigger,
  TabsContent,
} from "@orbis/ui-svelte/tabs";
import {
  Layers,
  Variable,
  History,
  Search,
} from "lucide-svelte";
import { Input } from "@orbis/ui-svelte/input";

let {
  store,
}: {
  store: EditorStore;
} = $props();

const isOpen = $derived(store.activeRailPanel !== null);
const panel = $derived(store.activeRailPanel);

// Panel titles
const panelTitles: Record<string, string> = {
  templates: "القوالب",
  elements: "العناصر",
  text: "النص",
  uploads: "الأصول والصور",
  tools: "الأدوات",
  "page-setup": "إعدادات الصفحة",
};

let toolsTab = $state("layers");
let searchQuery = $state("");

function close() {
  store.activeRailPanel = null;
}

// Resize logic
let isResizing = $state(false);
let resizeStartX = 0;
let resizeStartWidth = 0;

function startResize(e: MouseEvent) {
  e.preventDefault();
  isResizing = true;
  resizeStartX = e.clientX;
  resizeStartWidth = store.flyoutWidth;
  window.addEventListener("mousemove", onResize);
  window.addEventListener("mouseup", stopResize);
}

function onResize(e: MouseEvent) {
  if (!isResizing) return;
  // RTL-aware: resize handle is on the start side (left in LTR, right in RTL)
  const dx = e.clientX - resizeStartX;
  store.flyoutWidth = resizeStartWidth - dx;
}

function stopResize() {
  isResizing = false;
  window.removeEventListener("mousemove", onResize);
  window.removeEventListener("mouseup", stopResize);
}
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
{#if isOpen}
  <div
    class="flyout-container"
    style="width: {store.flyoutWidth}px;"
    onkeydown={(e) => e.key === "Escape" && close()}
  >
    <!-- Header -->
    <div class="flyout-header">
      <h3 class="flyout-title">{panelTitles[panel ?? ""] ?? ""}</h3>
      <Button variant="ghost" size="icon-xs" class="cursor-pointer shrink-0" onclick={close}>
        <X size={14} />
      </Button>
    </div>

    <!-- Content -->
    <div class="flyout-content">
      {#if panel === "uploads"}
        <AssetPanel {store} />

      {:else if panel === "tools"}
        <Tabs bind:value={toolsTab} class="flex flex-col h-full">
          <TabsList class="flex h-8 p-0 bg-muted/20 border-b border-border/30 rounded-none shrink-0 mx-3 mt-1">
            <TabsTrigger value="layers" class="flex-1 text-[9px] gap-1 h-full rounded-none data-[state=active]:bg-card cursor-pointer">
              <Layers size={11} />
              الطبقات
            </TabsTrigger>
            <TabsTrigger value="variables" class="flex-1 text-[9px] gap-1 h-full rounded-none data-[state=active]:bg-card cursor-pointer">
              <Variable size={11} />
              المتغيرات
            </TabsTrigger>
            <TabsTrigger value="history" class="flex-1 text-[9px] gap-1 h-full rounded-none data-[state=active]:bg-card cursor-pointer">
              <History size={11} />
              السجل
            </TabsTrigger>
          </TabsList>
          <TabsContent value="layers" class="flex-grow overflow-hidden mt-0">
            <LayersPanel {store} />
          </TabsContent>
          <TabsContent value="variables" class="flex-grow overflow-hidden mt-0">
            <VariablesPanel {store} />
          </TabsContent>
          <TabsContent value="history" class="flex-grow overflow-hidden mt-0">
            <HistoryPanel {store} />
          </TabsContent>
        </Tabs>

      {:else if panel === "templates"}
        <div class="flyout-placeholder">
          <div class="px-3 py-2">
            <div class="relative">
              <Search size={12} class="absolute start-2 top-1/2 -translate-y-1/2 text-muted-foreground" />
              <Input type="text" placeholder="ابحث في القوالب..." bind:value={searchQuery} class="h-7 text-[10px] ps-7" />
            </div>
          </div>
          <div class="flex flex-col items-center justify-center py-12 px-4 gap-2">
            <p class="text-[10px] text-muted-foreground text-center">
              قريباً — مكتبة قوالب جاهزة
            </p>
          </div>
        </div>

      {:else if panel === "elements"}
        <ElementsPanel {store} />

      {:else if panel === "text"}
        <TextPresetsPanel {store} />

      {:else if panel === "page-setup"}
        <PageSetupPanel {store} />
      {/if}
    </div>

    <!-- Resize handle -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="flyout-resize-handle" onmousedown={startResize}></div>
  </div>
{/if}

<style>
  .flyout-container {
    position: absolute;
    inset-inline-start: 0; /* adjacent to the rail */
    top: 0;
    bottom: 0;
    z-index: 30;
    background: var(--color-card);
    border-inline-end: 1px solid color-mix(in srgb, var(--color-border) 40%, transparent);
    display: flex;
    flex-direction: column;
    box-shadow: 4px 0 16px rgba(0, 0, 0, 0.15);
    animation: flyout-slide-in 0.2s ease-out;
  }

  @keyframes flyout-slide-in {
    from {
      transform: translateX(-30px);
      opacity: 0;
    }
    to {
      transform: translateX(0);
      opacity: 1;
    }
  }

  :global([dir="rtl"]) .flyout-container {
    box-shadow: -4px 0 16px rgba(0, 0, 0, 0.15);
  }

  :global([dir="rtl"]) .flyout-container {
    animation: flyout-slide-in-rtl 0.2s ease-out;
  }

  @keyframes flyout-slide-in-rtl {
    from {
      transform: translateX(30px);
      opacity: 0;
    }
    to {
      transform: translateX(0);
      opacity: 1;
    }
  }

  .flyout-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 10px 12px;
    border-bottom: 1px solid color-mix(in srgb, var(--color-border) 30%, transparent);
    flex-shrink: 0;
  }

  .flyout-title {
    font-size: 13px;
    font-weight: 700;
    color: var(--color-foreground);
  }

  .flyout-content {
    flex-grow: 1;
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }

  .flyout-placeholder {
    display: flex;
    flex-direction: column;
    height: 100%;
  }

  .flyout-resize-handle {
    position: absolute;
    inset-inline-end: -3px;
    top: 0;
    bottom: 0;
    width: 6px;
    cursor: col-resize;
    z-index: 10;
  }

  .flyout-resize-handle:hover {
    background: color-mix(in srgb, var(--color-primary) 30%, transparent);
  }
</style>
