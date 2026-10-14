<script lang="ts">
import * as m from "$lib/paraglide/messages";
/**
 * FlyoutPanel — Sliding panel overlay.
 * Slides from the side rail over the canvas.
 * Contains the active panel content.
 */
import type { EditorStore } from "../stores/editor-store.svelte";
import { Button } from "@orbis/ui/button";
import { ClockRotateRight, Code, Keyframes, XMark } from "@orbis/icons";

// Panels
import AssetPanel from "../panels/asset-panel.svelte";
import LayersPanel from "../panels/layers-panel.svelte";
import VariablesPanel from "../panels/variables-panel.svelte";
import HistoryPanel from "../panels/history-panel.svelte";
import ElementsPanel from "../panels/elements-panel.svelte";
import TextPresetsPanel from "../panels/text-presets-panel.svelte";
import PageSetupPanel from "../panels/page-setup-panel.svelte";
import ReadyTemplatesPanel from "../panels/ready-templates-panel.svelte";
import { Tabs, TabsList, TabsTrigger, TabsContent } from "@orbis/ui/tabs";

let {
  store,
  orgId,
}: {
  store: EditorStore;
  orgId: string;
} = $props();

const isOpen = $derived(store.activeRailPanel !== null);
const panel = $derived(store.activeRailPanel);

// Panel titles
const panelTitles: Record<string, string> = {
  templates: m.common_td_panel_pages(),
  elements: m.common_td_panel_elements(),
  text: m.common_td_panel_text(),
  uploads: m.common_td_panel_media(),
  tools: m.common_td_panel_tools(),
  "page-setup": m.common_td_panel_settings(),
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
      <Button variant="ghost" size="icon-sm" class="cursor-pointer shrink-0" onclick={close}>
        <XMark size={16} />
      </Button>
    </div>

    <!-- Content -->
    <div class="flyout-content">
      {#if panel === "uploads"}
        <AssetPanel {store} />

      {:else if panel === "tools"}
        <Tabs bind:value={toolsTab} class="flex flex-col h-full">
          <TabsList class="flex h-9 p-1 bg-muted/40 border border-border rounded-md shrink-0 mx-3 mt-3">
            <TabsTrigger value="layers" class="flex-1 text-[11px] gap-1.5 h-full rounded data-[state=active]:bg-card data-[state=active]:shadow-sm cursor-pointer">
              <Keyframes size={14} />
              {m.template_panel_layers()}
            </TabsTrigger>
            <TabsTrigger value="variables" class="flex-1 text-[11px] gap-1.5 h-full rounded data-[state=active]:bg-card data-[state=active]:shadow-sm cursor-pointer">
              <Code size={14} />
              {m.template_panel_variables()}
            </TabsTrigger>
            <TabsTrigger value="history" class="flex-1 text-[11px] gap-1.5 h-full rounded data-[state=active]:bg-card data-[state=active]:shadow-sm cursor-pointer">
              <ClockRotateRight size={14} />
              {m.template_panel_history()}
            </TabsTrigger>
          </TabsList>
          <TabsContent value="layers" class="grow overflow-hidden mt-0">
            <LayersPanel {store} />
          </TabsContent>
          <TabsContent value="variables" class="grow overflow-hidden mt-0">
            <VariablesPanel {store} />
          </TabsContent>
          <TabsContent value="history" class="grow overflow-hidden mt-0">
            <HistoryPanel {store} />
          </TabsContent>
        </Tabs>

      {:else if panel === "templates"}
        <ReadyTemplatesPanel {orgId} />

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
    border-inline-end: 1px solid var(--color-border);
    display: flex;
    flex-direction: column;
    box-shadow: 4px 0 12px color-mix(in srgb, var(--color-foreground) 7%, transparent);
    animation: flyout-slide-in 0.14s ease-out;
  }

  @keyframes flyout-slide-in {
    from {
      transform: translateX(-15px);
      opacity: 0;
    }
    to {
      transform: translateX(0);
      opacity: 1;
    }
  }

  :global([dir="rtl"]) .flyout-container {
    box-shadow: -4px 0 12px color-mix(in srgb, var(--color-foreground) 7%, transparent);
    animation: flyout-slide-in-rtl 0.14s ease-out;
  }

  @keyframes flyout-slide-in-rtl {
    from {
      transform: translateX(15px);
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
    min-height: 52px;
    padding: 8px 12px;
    border-bottom: 1px solid var(--color-border);
    background: var(--color-card);
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
