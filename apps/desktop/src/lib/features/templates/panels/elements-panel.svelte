<script lang="ts">
import * as m from "$lib/paraglide/messages";
/**
 * ElementsPanel — Shapes and elements library with drag-and-drop.
 * Provides quick access to all draggable element types.
 */
import type { EditorStore } from "../stores/editor-store.svelte";
import { Input } from "@orbis/ui/input";
import { AdjustmentsHorizontal, AlignVertical, ArrowDownTray, ArrowPath, CheckBox, CheckCircle, CheckSquare, Component, Cube, Grid, H1, Hashtag, Merge, Minus, Photo, Search, Sidebar } from "@orbis/icons";

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
    icon: H1,
    label: "Text",
    labelAr: m.template_element_text(),
    category: "basic",
  },
  {
    id: "image",
    type: "Image",
    icon: Photo,
    label: "Image",
    labelAr: m.template_element_image(),
    category: "basic",
  },
  {
    id: "shape-rect",
    type: "Shape",
    icon: CheckBox,
    label: "Rectangle",
    labelAr: m.template_element_rect(),
    category: "shapes",
  },
  {
    id: "shape-circle",
    type: "Shape",
    icon: AdjustmentsHorizontal,
    label: "Circle",
    labelAr: m.template_element_circle(),
    category: "shapes",
  },
  {
    id: "separator",
    type: "Separator",
    icon: Minus,
    label: "Separator",
    labelAr: m.template_element_divider(),
    category: "basic",
  },
  {
    id: "table",
    type: "Table",
    icon: Grid,
    label: "Table",
    labelAr: m.template_element_table(),
    category: "data",
  },
  {
    id: "barcode",
    type: "Barcode",
    icon: Hashtag,
    label: "Barcode",
    labelAr: m.template_element_barcode(),
    category: "data",
  },
  {
    id: "qrcode",
    type: "QRCode",
    icon: CheckSquare,
    label: "QR Code",
    labelAr: m.template_element_qrcode(),
    category: "data",
  },
  {
    id: "frame",
    type: "Frame",
    icon: Cube,
    label: "Frame",
    labelAr: m.template_element_frame(),
    category: "layout",
  },
  {
    id: "grid",
    type: "Grid",
    icon: Grid,
    label: "Grid",
    labelAr: "شبكة",
    category: "layout",
  },
  {
    id: "repeater",
    type: "Repeater",
    icon: ArrowPath,
    label: "Repeater",
    labelAr: "مكرر",
    category: "logic",
  },
  {
    id: "conditional",
    type: "Conditional",
    icon: Merge,
    label: "Conditional",
    labelAr: "شرط",
    category: "logic",
  },
  {
    id: "component",
    type: "ComponentRef",
    icon: Component,
    label: "Component",
    labelAr: "مكوّن",
    category: "layout",
  },
  {
    id: "page-break",
    type: "PageBreak",
    icon: ArrowDownTray,
    label: "Page Break",
    labelAr: "فاصل صفحة",
    category: "flow",
  },
  {
    id: "header",
    type: "Header",
    icon: Sidebar,
    label: "Header",
    labelAr: "رأس الصفحة",
    category: "flow",
  },
  {
    id: "footer",
    type: "Footer",
    icon: Sidebar,
    label: "Footer",
    labelAr: "تذييل الصفحة",
    category: "flow",
  },
  {
    id: "watermark",
    type: "Watermark",
    icon: CheckCircle,
    label: "Watermark",
    labelAr: "علامة مائية",
    category: "basic",
  },
  {
    id: "spacer",
    type: "Spacer",
    icon: AlignVertical,
    label: "Spacer",
    labelAr: "مسافة",
    category: "layout",
  },
];

const categories = [
  { id: "basic", label: m.template_category_basic() },
  { id: "shapes", label: m.template_category_shapes() },
  { id: "data", label: m.template_category_data() },
  { id: "layout", label: m.template_category_layout() },
  { id: "logic", label: m.template_category_logic() },
  { id: "flow", label: m.template_category_flow() },
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
      <Search size={12} class="absolute inset-s-2 top-1/2 -translate-y-1/2 text-muted-foreground" />
      <Input
        type="text"
        placeholder=m.template_search_elements()
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
                  <item.icon size={20} strokeWidth={1.5} />
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
      {m.template_msg_drag_element()}
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
