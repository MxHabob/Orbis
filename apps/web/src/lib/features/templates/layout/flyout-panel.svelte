<script lang="ts">
import * as m from "$lib/paraglide/messages";
/**
 * FlyoutPanel — Sliding panel overlay.
 * Slides from the side rail over the canvas.
 * Contains the active panel content.
 */
import PanelResizeHandle from "./panel-resize-handle.svelte";
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

function close() {
  store.activeRailPanel = null;
}

</script>

{#if isOpen}
  <aside
    class="flyout-container"
    aria-label={panelTitles[panel ?? ""]}
    style="width: {store.flyoutWidth}px;"
  >
    <!-- Header -->
    <div class="flyout-header">
      <h3 class="flyout-title">{panelTitles[panel ?? ""] ?? ""}</h3>
      <Button variant="ghost" size="icon-sm" aria-label={m.common_action_close()} onclick={close}>
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

    <PanelResizeHandle
      edge="end"
      width={store.flyoutWidth}
      min={280}
      max={480}
      label={m.template_inspector_resize_panel()}
      onresize={(width) => { store.flyoutWidth = width; }}
    />
  </aside>
{/if}

<style>
  .flyout-container {
    position: relative;
    z-index: 15;
    max-width: min(480px, 34vw);
    min-height: 0;
    background: var(--color-card);
    border-inline-end: 1px solid var(--color-border);
    display: flex;
    flex-direction: column;
    flex: 0 0 auto;
  }
  .flyout-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    min-height: 52px;
    padding: 8px 12px;
    border-block-end: 1px solid var(--color-border);
    flex-shrink: 0;
  }
  .flyout-title {
    margin: 0;
    font-size: 13px;
    font-weight: 650;
  }
  .flyout-content {
    flex: 1;
    min-height: 0;
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }
  @media (max-width: 900px) {
    .flyout-container {
      position: absolute;
      inset-block: 0;
      inset-inline-start: 64px;
      max-width: calc(100% - 64px);
      box-shadow: 0 8px 28px color-mix(in srgb, var(--color-foreground) 14%, transparent);
    }
  }
</style>