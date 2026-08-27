<script lang="ts">
import * as m from "$lib/paraglide/messages";
/**
 * PageSetupPanel — Page size, orientation, margins, background color.
 */
import type { EditorStore } from "../stores/editor-store.svelte";
import { Input } from "@orbis/ui-svelte/input";
import { Label } from "@orbis/ui-svelte/label";
import { Button } from "@orbis/ui-svelte/button";
import { Separator } from "@orbis/ui-svelte/separator";
import ColorPicker from "../shared/color-picker.svelte";
import { FileText, RotateCcw } from "@lucide/svelte";

let {
  store,
}: {
  store: EditorStore;
} = $props();

const pageSetup = $derived(store.document.meta.pageSetup);
const margins = $derived(pageSetup.margins);

const pageSizes = [
  { id: "A4", label: "A4 (210×297)", w: 210, h: 297 },
  { id: "A3", label: "A3 (297×420)", w: 297, h: 420 },
  { id: "A5", label: "A5 (148×210)", w: 148, h: 210 },
  { id: "Letter", label: "Letter (216×279)", w: 216, h: 279 },
  { id: "Legal", label: "Legal (216×356)", w: 216, h: 356 },
  { id: "ID_CARD", label: m.template_size_id_card(), w: 53.98, h: 85.6 },
  { id: "BADGE", label: m.template_size_badge(), w: 54, h: 86 },
  { id: "Custom", label: m.template_size_custom(), w: 0, h: 0 },
];

function updatePageSetup(updates: Record<string, unknown>) {
  store.updatePageSetup(updates as any);
}

function updateMargin(side: string, value: number) {
  updatePageSetup({
    margins: { ...margins, [side]: Math.max(0, value) },
  });
}

function toggleOrientation() {
  updatePageSetup({
    orientation:
      pageSetup.orientation === "portrait" ? "landscape" : "portrait",
  });
}
</script>

<div class="page-setup-panel">
  <div class="panel-content">
    <!-- Page Size -->
    <div class="section">
      <Label class="text-[10px] font-semibold text-muted-foreground">{m.template_prop_page_size()}</Label>
      <div class="size-grid">
        {#each pageSizes as size (size.id)}
          <button
            type="button"
            class="size-card"
            class:active={pageSetup.size === size.id}
            onclick={() => updatePageSetup({ size: size.id })}
          >
            <div class="size-icon" class:landscape={pageSetup.orientation === "landscape" && pageSetup.size === size.id}>
              <FileText size={14} />
            </div>
            <div class="size-info">
              <span class="size-name">{size.id}</span>
              {#if size.w > 0}
                <span class="size-dims">{size.w}×{size.h}</span>
              {/if}
            </div>
          </button>
        {/each}
      </div>
    </div>

    <Separator class="opacity-30" />

    <!-- Orientation -->
    <div class="section">
      <Label class="text-[10px] font-semibold text-muted-foreground">{m.template_prop_page_orientation()}</Label>
      <div class="orientation-row">
        <button
          type="button"
          class="orient-btn"
          class:active={pageSetup.orientation === "portrait"}
          onclick={() => updatePageSetup({ orientation: "portrait" })}
        >
          <div class="orient-icon portrait-icon"></div>
          <span>{m.template_orientation_vert()}</span>
        </button>
        <button
          type="button"
          class="orient-btn"
          class:active={pageSetup.orientation === "landscape"}
          onclick={() => updatePageSetup({ orientation: "landscape" })}
        >
          <div class="orient-icon landscape-icon"></div>
          <span>{m.template_orientation_horz()}</span>
        </button>
      </div>
    </div>

    <Separator class="opacity-30" />

    <!-- Margins -->
    <div class="section">
      <Label class="text-[10px] font-semibold text-muted-foreground">{m.template_prop_margins()}</Label>
      <div class="margins-grid">
        <div class="margin-field">
          <span class="margin-label">{m.template_margin_top()}</span>
          <Input
            type="number"
            value={margins.top}
            oninput={(e) => updateMargin("top", parseFloat(e.currentTarget.value) || 0)}
            class="h-6 text-[10px] text-center"
            min="0" max="100" step="1"
          />
        </div>
        <div class="margin-field">
          <span class="margin-label">{m.template_margin_bottom()}</span>
          <Input
            type="number"
            value={margins.bottom}
            oninput={(e) => updateMargin("bottom", parseFloat(e.currentTarget.value) || 0)}
            class="h-6 text-[10px] text-center"
            min="0" max="100" step="1"
          />
        </div>
        <div class="margin-field">
          <span class="margin-label">{m.template_margin_right()}</span>
          <Input
            type="number"
            value={margins.right}
            oninput={(e) => updateMargin("right", parseFloat(e.currentTarget.value) || 0)}
            class="h-6 text-[10px] text-center"
            min="0" max="100" step="1"
          />
        </div>
        <div class="margin-field">
          <span class="margin-label">{m.template_margin_left()}</span>
          <Input
            type="number"
            value={margins.left}
            oninput={(e) => updateMargin("left", parseFloat(e.currentTarget.value) || 0)}
            class="h-6 text-[10px] text-center"
            min="0" max="100" step="1"
          />
        </div>
      </div>
    </div>

    <Separator class="opacity-30" />

    <!-- Custom Size (only when Custom is selected) -->
    {#if pageSetup.size === "Custom"}
      <div class="section">
        <Label class="text-[10px] font-semibold text-muted-foreground">{m.template_prop_custom_dims()}</Label>
        <div class="custom-dims">
          <div class="margin-field">
            <span class="margin-label">{m.template_prop_width()}</span>
            <Input
              type="number"
              value={pageSetup.width ?? 210}
              oninput={(e) => updatePageSetup({ width: parseFloat(e.currentTarget.value) || 210 })}
              class="h-6 text-[10px] text-center"
              min="20" max="1000"
            />
          </div>
          <div class="margin-field">
            <span class="margin-label">{m.template_prop_height()}</span>
            <Input
              type="number"
              value={pageSetup.height ?? 297}
              oninput={(e) => updatePageSetup({ height: parseFloat(e.currentTarget.value) || 297 })}
              class="h-6 text-[10px] text-center"
              min="20" max="1000"
            />
          </div>
        </div>
      </div>
    {/if}
  </div>
</div>

<style>
  .page-setup-panel {
    display: flex;
    flex-direction: column;
    height: 100%;
  }

  .panel-content {
    flex-grow: 1;
    overflow-y: auto;
    padding: 8px 12px;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .panel-content::-webkit-scrollbar { display: none; }

  .section {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .size-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 4px;
  }

  .size-card {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 8px;
    border: 1px solid color-mix(in srgb, var(--color-border) 25%, transparent);
    border-radius: 6px;
    background: transparent;
    cursor: pointer;
    transition: all 0.12s;
    text-align: start;
  }

  .size-card:hover {
    background: color-mix(in srgb, var(--color-accent) 60%, transparent);
    border-color: color-mix(in srgb, var(--color-border) 50%, transparent);
  }

  .size-card.active {
    background: color-mix(in srgb, var(--color-primary) 12%, transparent);
    border-color: var(--color-primary);
  }

  .size-icon {
    color: var(--color-muted-foreground);
    transition: transform 0.15s;
  }

  .size-icon.landscape {
    transform: rotate(90deg);
  }

  .size-card.active .size-icon {
    color: var(--color-primary);
  }

  .size-info {
    display: flex;
    flex-direction: column;
    gap: 0;
  }

  .size-name {
    font-size: 10px;
    font-weight: 600;
    color: var(--color-foreground);
  }

  .size-dims {
    font-size: 8px;
    color: var(--color-muted-foreground);
    font-family: monospace;
  }

  .orientation-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 6px;
  }

  .orient-btn {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
    padding: 10px;
    border: 1px solid color-mix(in srgb, var(--color-border) 25%, transparent);
    border-radius: 8px;
    background: transparent;
    cursor: pointer;
    font-size: 9px;
    color: var(--color-muted-foreground);
    transition: all 0.12s;
  }

  .orient-btn:hover {
    background: color-mix(in srgb, var(--color-accent) 60%, transparent);
  }

  .orient-btn.active {
    background: color-mix(in srgb, var(--color-primary) 12%, transparent);
    border-color: var(--color-primary);
    color: var(--color-primary);
  }

  .orient-icon {
    border: 1.5px solid currentColor;
    border-radius: 2px;
  }

  .portrait-icon {
    width: 16px;
    height: 22px;
  }

  .landscape-icon {
    width: 22px;
    height: 16px;
  }

  .margins-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 6px;
  }

  .margin-field {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .margin-label {
    font-size: 8px;
    color: var(--color-muted-foreground);
  }

  .custom-dims {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 6px;
  }
</style>
