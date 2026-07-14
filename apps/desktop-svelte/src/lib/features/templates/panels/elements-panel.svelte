<script lang="ts">
import * as m from "$lib/paraglide/messages";
/**
 * ElementsPanel — Shapes and elements library with drag-and-drop.
 * Provides quick access to all draggable element types.
 */
import type { EditorStore } from "../stores/editor-store.svelte";
import { Input } from "@orbis/ui-svelte/input";
import { Badge } from "@orbis/ui-svelte/badge";
import {
  Search,
  Type,
  Image,
  Square,
  Circle,
  Minus,
  Table,
  Barcode,
  QrCode,
  Frame,
} from "lucide-svelte";

let {
  store,
}: {
  store: EditorStore;
} = $props();

let searchQuery = $state("");

type ElementItem = {
  id: string;
  type: string;
  icon: any;
  label: string;
  labelAr: string;
  category: string;
};

const elements: ElementItem[] = [
  {
    id: "text",
    type: "Text",
    icon: Type,
    label: "Text",
    labelAr: m.template_auto_ea433dcf(),
    category: "basic",
  },
  {
    id: "image",
    type: "Image",
    icon: Image,
    label: "Image",
    labelAr: m.template_auto_22d88250(),
    category: "basic",
  },
  {
    id: "shape-rect",
    type: "Shape",
    icon: Square,
    label: "Rectangle",
    labelAr: m.template_auto_57d1e62c(),
    category: "shapes",
  },
  {
    id: "shape-circle",
    type: "Shape",
    icon: Circle,
    label: "Circle",
    labelAr: m.template_auto_f5fd5064(),
    category: "shapes",
  },
  {
    id: "separator",
    type: "Separator",
    icon: Minus,
    label: "Separator",
    labelAr: m.template_auto_66080a01(),
    category: "basic",
  },
  {
    id: "table",
    type: "Table",
    icon: Table,
    label: "Table",
    labelAr: m.template_auto_64b2ab32(),
    category: "data",
  },
  {
    id: "barcode",
    type: "Barcode",
    icon: Barcode,
    label: "Barcode",
    labelAr: m.template_auto_a8f62212(),
    category: "data",
  },
  {
    id: "qrcode",
    type: "QRCode",
    icon: QrCode,
    label: "QR Code",
    labelAr: m.template_auto_6fe77abd(),
    category: "data",
  },
  {
    id: "frame",
    type: "Frame",
    icon: Frame,
    label: "Frame",
    labelAr: m.template_auto_3545d651(),
    category: "layout",
  },
];

const categories = [
  { id: m.template_auto_11742414() },
  { id: m.template_auto_69a047fb() },
  { id: m.template_auto_cc9c9fcb() },
  { id: m.template_auto_75201405() },
];

const filteredElements = $derived(
  searchQuery
    ? elements.filter(
        (el) =>
          el.label.toLowerCase().includes(searchQuery.toLowerCase()) ||
          el.labelAr.includes(searchQuery),
      )
    : elements,
);

function handleDragStart(e: DragEvent, item: ElementItem) {
  let elementType = item.type;
  e.dataTransfer?.setData("application/x-docforge-element", elementType);
  e.dataTransfer?.setData("text/plain", elementType);
  if (e.dataTransfer) e.dataTransfer.effectAllowed = "copy";

  // Extra data for shape type
  if (item.id === "shape-circle") {
    e.dataTransfer?.setData("application/x-docforge-shape", "circle");
  }
}
</script>

<div class="elements-panel">
  <!-- Search -->
  <div class="px-3 py-2">
    <div class="relative">
      <Search size={12} class="absolute start-2 top-1/2 -translate-y-1/2 text-muted-foreground" />
      <Input
        type="text"
        placeholder=m.template_auto_9a078b42()
        bind:value={searchQuery}
        class="h-7 text-[10px] ps-7"
      />
    </div>
  </div>

  <!-- Elements Grid -->
  <div class="elements-content">
    {#each categories as category}
      {@const catElements = filteredElements.filter((el) => el.category === category.id)}
      {#if catElements.length > 0}
        <div class="category-section">
          <span class="category-label">{category.label}</span>
          <div class="elements-grid">
            {#each catElements as item (item.id)}
              <!-- svelte-ignore a11y_no_static_element_interactions -->
              <div
                class="element-card"
                draggable="true"
                ondragstart={(e) => handleDragStart(e, item)}
              >
                <div class="element-icon">
                  <svelte:component this={item.icon} size={20} strokeWidth={1.5} />
                </div>
                <span class="element-label">{item.labelAr}</span>
              </div>
            {/each}
          </div>
        </div>
      {/if}
    {/each}
  </div>

  <!-- Hint -->
  <div class="px-3 py-2 border-t border-border/20">
    <p class="text-[8px] text-muted-foreground text-center">
      {m.template_auto_023b8490()}
    </p>
  </div>
</div>

<style>
  .elements-panel {
    display: flex;
    flex-direction: column;
    height: 100%;
  }

  .elements-content {
    flex-grow: 1;
    overflow-y: auto;
    padding: 0 12px 12px;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .elements-content::-webkit-scrollbar { display: none; }

  .category-section {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .category-label {
    font-size: 9px;
    font-weight: 700;
    color: var(--color-muted-foreground);
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .elements-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 6px;
  }

  .element-card {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
    padding: 10px 4px 8px;
    border-radius: 8px;
    border: 1px solid color-mix(in srgb, var(--color-border) 30%, transparent);
    background: color-mix(in srgb, var(--color-muted) 15%, transparent);
    cursor: grab;
    transition: all 0.15s ease;
    user-select: none;
  }

  .element-card:hover {
    background: var(--color-accent);
    border-color: color-mix(in srgb, var(--color-border) 60%, transparent);
    transform: translateY(-1px);
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
  }

  .element-card:active {
    cursor: grabbing;
    transform: scale(0.97);
  }

  .element-icon {
    color: var(--color-muted-foreground);
  }

  .element-card:hover .element-icon {
    color: var(--color-foreground);
  }

  .element-label {
    font-size: 9px;
    color: var(--color-muted-foreground);
    white-space: nowrap;
  }
</style>
