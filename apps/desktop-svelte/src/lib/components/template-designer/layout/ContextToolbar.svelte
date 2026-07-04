<script lang="ts">
/**
 * ContextToolbar — Contextual top toolbar that changes based on selection.
 *
 * Shows different controls based on what's selected:
 *  - No selection → page tools + undo/redo + zoom + save
 *  - Text → font family, size, weight, color, alignment
 *  - Image → fit, opacity
 *  - Shape → fill, stroke, border-radius
 *  - Multi-select → align, distribute
 */
import type { EditorStore, EditorTool } from "../stores/editor-store.svelte";
import { Button } from "@orbis/ui-svelte/button";
import { Badge } from "@orbis/ui-svelte/badge";
import { Input } from "@orbis/ui-svelte/input";
import { Separator } from "@orbis/ui-svelte/separator";
import {
  Tooltip,
  TooltipTrigger,
  TooltipContent,
  TooltipProvider,
} from "@orbis/ui-svelte/tooltip";
import {
  ArrowLeft,
  Undo2,
  Redo2,
  Save,
  Eye,
  ZoomIn,
  ZoomOut,
  Maximize,
  Grid3x3,
  Ruler,
  Magnet,
  Bold,
  Italic,
  Underline,
  AlignLeft,
  AlignCenter,
  AlignRight,
  AlignStartVertical,
  AlignCenterVertical,
  AlignEndVertical,
  Minus,
  Plus,
} from "lucide-svelte";

let {
  store,
  templateName = "",
  onback,
  onsave,
  onpreview,
}: {
  store: EditorStore;
  templateName?: string;
  onback?: () => void;
  onsave: () => void;
  onpreview: () => void;
} = $props();

const node = $derived(store.selectedNode);
const nodeType = $derived(node?.type ?? null);
const props = $derived(node && "props" in node ? (node as any).props || {} : {});
const hasSelection = $derived(store.selectedIds.length > 0);
const isMultiSelect = $derived(store.selectedIds.length > 1);

function updateProp(key: string, value: unknown) {
  store.updateSelectedProps({ props: { ...props, [key]: value } } as any);
}
</script>

<TooltipProvider delayDuration={200}>
<div class="ctx-toolbar">
  <!-- Left: Back + Title -->
  <div class="ctx-group shrink-0">
    {#if onback}
      <Tooltip>
        <TooltipTrigger>
          <Button variant="ghost" size="icon-xs" class="cursor-pointer" onclick={onback}>
            <ArrowLeft size={15} class="rtl:rotate-180" />
          </Button>
        </TooltipTrigger>
        <TooltipContent>رجوع</TooltipContent>
      </Tooltip>
    {/if}
    <span class="ctx-title">{templateName || "..."}</span>
    {#if store.isDirty}
      <div class="ctx-dirty-dot"></div>
    {/if}
  </div>

  <Separator orientation="vertical" class="h-5 mx-1.5" />

  <!-- Center: Contextual Controls -->
  <div class="ctx-center">
    {#if nodeType === "Text" && !isMultiSelect}
      <!-- ═══ TEXT TOOLBAR ═══ -->
      <div class="ctx-group">
        <Input
          type="text"
          value={props.fontFamily || "Rubik"}
          oninput={(e) => updateProp("fontFamily", e.currentTarget.value)}
          class="h-6 w-[90px] text-[10px] font-medium"
        />
        <div class="ctx-number-input">
          <button type="button" class="ctx-num-btn" onclick={() => updateProp("fontSize", Math.max(6, (props.fontSize || 14) - 1))}>
            <Minus size={10} />
          </button>
          <Input
            type="number"
            value={props.fontSize || 14}
            oninput={(e) => updateProp("fontSize", parseInt(e.currentTarget.value) || 14)}
            class="h-6 w-[40px] text-[10px] text-center"
            min="6" max="200"
          />
          <button type="button" class="ctx-num-btn" onclick={() => updateProp("fontSize", Math.min(200, (props.fontSize || 14) + 1))}>
            <Plus size={10} />
          </button>
        </div>
      </div>

      <Separator orientation="vertical" class="h-5 mx-1" />

      <div class="ctx-group">
        <button type="button" class="ctx-format-btn" class:active={props.fontWeight === "700" || props.fontWeight === "bold"} onclick={() => updateProp("fontWeight", props.fontWeight === "700" ? "400" : "700")}>
          <Bold size={13} />
        </button>
        <button type="button" class="ctx-format-btn" class:active={props.fontStyle === "italic"} onclick={() => updateProp("fontStyle", props.fontStyle === "italic" ? "normal" : "italic")}>
          <Italic size={13} />
        </button>
        <button type="button" class="ctx-format-btn" class:active={props.textDecoration === "underline"} onclick={() => updateProp("textDecoration", props.textDecoration === "underline" ? "none" : "underline")}>
          <Underline size={13} />
        </button>
      </div>

      <Separator orientation="vertical" class="h-5 mx-1" />

      <div class="ctx-group">
        <input
          type="color"
          value={props.color || "#000000"}
          oninput={(e) => updateProp("color", e.currentTarget.value)}
          class="ctx-color-swatch"
        />
      </div>

      <Separator orientation="vertical" class="h-5 mx-1" />

      <div class="ctx-group">
        <button type="button" class="ctx-format-btn" class:active={props.textAlign === "start" || !props.textAlign} onclick={() => updateProp("textAlign", "start")}>
          <AlignRight size={13} />
        </button>
        <button type="button" class="ctx-format-btn" class:active={props.textAlign === "center"} onclick={() => updateProp("textAlign", "center")}>
          <AlignCenter size={13} />
        </button>
        <button type="button" class="ctx-format-btn" class:active={props.textAlign === "end"} onclick={() => updateProp("textAlign", "end")}>
          <AlignLeft size={13} />
        </button>
      </div>

    {:else if nodeType === "Shape" && !isMultiSelect}
      <!-- ═══ SHAPE TOOLBAR ═══ -->
      <div class="ctx-group">
        <span class="ctx-label">تعبئة</span>
        <input
          type="color"
          value={props.fillColor || "#e2e8f0"}
          oninput={(e) => updateProp("fillColor", e.currentTarget.value)}
          class="ctx-color-swatch"
        />
        <span class="ctx-label">حدود</span>
        <input
          type="color"
          value={props.strokeColor || "#94a3b8"}
          oninput={(e) => updateProp("strokeColor", e.currentTarget.value)}
          class="ctx-color-swatch"
        />
        <Input
          type="number"
          value={props.strokeWidth ?? 1}
          oninput={(e) => updateProp("strokeWidth", parseFloat(e.currentTarget.value) || 1)}
          class="h-6 w-[40px] text-[10px]"
          min="0" max="20" step="0.5"
        />
        <span class="ctx-label">زوايا</span>
        <Input
          type="number"
          value={props.borderRadius ?? 0}
          oninput={(e) => updateProp("borderRadius", parseFloat(e.currentTarget.value) || 0)}
          class="h-6 w-[40px] text-[10px]"
          min="0" max="50"
        />
      </div>

    {:else if nodeType === "Image" && !isMultiSelect}
      <!-- ═══ IMAGE TOOLBAR ═══ -->
      <div class="ctx-group">
        <span class="ctx-label">ملاءمة</span>
        <select
          value={props.fit || "contain"}
          onchange={(e) => updateProp("fit", e.currentTarget.value)}
          class="ctx-select"
        >
          <option value="contain">Contain</option>
          <option value="cover">Cover</option>
          <option value="fill">Fill</option>
          <option value="none">None</option>
        </select>
        <span class="ctx-label">شفافية</span>
        <Input
          type="number"
          value={Math.round((props.opacity ?? 1) * 100)}
          oninput={(e) => updateProp("opacity", (parseInt(e.currentTarget.value) || 100) / 100)}
          class="h-6 w-[45px] text-[10px]"
          min="0" max="100"
        />
        <span class="text-[9px] text-muted-foreground">%</span>
      </div>

    {:else if (nodeType === "Barcode" || nodeType === "QRCode") && !isMultiSelect}
      <!-- ═══ BARCODE TOOLBAR ═══ -->
      <div class="ctx-group">
        <span class="ctx-label">البيانات</span>
        <Input
          type="text"
          value={props.data || ""}
          oninput={(e) => updateProp("data", e.currentTarget.value)}
          class="h-6 w-[160px] text-[10px] font-mono"
        />
        {#if nodeType === "Barcode"}
          <span class="ctx-label">الصيغة</span>
          <select
            value={props.format || "Code128"}
            onchange={(e) => updateProp("format", e.currentTarget.value)}
            class="ctx-select"
          >
            <option value="Code128">Code 128</option>
            <option value="Code39">Code 39</option>
            <option value="EAN13">EAN-13</option>
            <option value="EAN8">EAN-8</option>
          </select>
        {/if}
      </div>

    {:else if isMultiSelect}
      <!-- ═══ MULTI-SELECT TOOLBAR ═══ -->
      <div class="ctx-group">
        <Badge variant="secondary" class="text-[9px]">{store.selectedIds.length} عناصر محددة</Badge>
      </div>
      <Separator orientation="vertical" class="h-5 mx-1" />
      <div class="ctx-group">
        {#each [
          { icon: AlignLeft, action: "left", label: "يسار" },
          { icon: AlignCenter, action: "center", label: "وسط" },
          { icon: AlignRight, action: "right", label: "يمين" },
          { icon: AlignStartVertical, action: "top", label: "أعلى" },
          { icon: AlignCenterVertical, action: "middle", label: "وسط" },
          { icon: AlignEndVertical, action: "bottom", label: "أسفل" },
        ] as item (item.action)}
          <Tooltip>
            <TooltipTrigger>
              <button type="button" class="ctx-format-btn">
                <svelte:component this={item.icon} size={13} />
              </button>
            </TooltipTrigger>
            <TooltipContent>{item.label}</TooltipContent>
          </Tooltip>
        {/each}
      </div>

    {:else}
      <!-- ═══ DEFAULT / PAGE TOOLBAR ═══ -->
      <div class="ctx-group">
        <Badge variant="outline" class="text-[9px] font-mono">
          {store.document.meta.pageSetup.size}
          {store.document.meta.pageSetup.orientation === "landscape" ? "▬" : "▮"}
        </Badge>
      </div>
    {/if}
  </div>

  <!-- Right: Always-visible actions -->
  <div class="ctx-group shrink-0">
    <Tooltip>
      <TooltipTrigger>
        <Button variant="ghost" size="icon-xs" class="cursor-pointer" onclick={() => store.undo()} disabled={!store.canUndo}>
          <Undo2 size={14} />
        </Button>
      </TooltipTrigger>
      <TooltipContent>تراجع (Ctrl+Z)</TooltipContent>
    </Tooltip>
    <Tooltip>
      <TooltipTrigger>
        <Button variant="ghost" size="icon-xs" class="cursor-pointer" onclick={() => store.redo()} disabled={!store.canRedo}>
          <Redo2 size={14} />
        </Button>
      </TooltipTrigger>
      <TooltipContent>إعادة (Ctrl+Y)</TooltipContent>
    </Tooltip>

    <Separator orientation="vertical" class="h-5 mx-1" />

    <Tooltip>
      <TooltipTrigger>
        <Button variant="ghost" size="icon-xs" class="cursor-pointer" onclick={() => store.gridVisible = !store.gridVisible}>
          <Grid3x3 size={13} class={store.gridVisible ? "text-primary" : ""} />
        </Button>
      </TooltipTrigger>
      <TooltipContent>الشبكة</TooltipContent>
    </Tooltip>
    <Tooltip>
      <TooltipTrigger>
        <Button variant="ghost" size="icon-xs" class="cursor-pointer" onclick={() => store.snapEnabled = !store.snapEnabled}>
          <Magnet size={13} class={store.snapEnabled ? "text-primary" : ""} />
        </Button>
      </TooltipTrigger>
      <TooltipContent>الالتصاق</TooltipContent>
    </Tooltip>
    <Tooltip>
      <TooltipTrigger>
        <Button variant="ghost" size="icon-xs" class="cursor-pointer" onclick={() => store.rulersVisible = !store.rulersVisible}>
          <Ruler size={13} class={store.rulersVisible ? "text-primary" : ""} />
        </Button>
      </TooltipTrigger>
      <TooltipContent>المساطر</TooltipContent>
    </Tooltip>

    <Separator orientation="vertical" class="h-5 mx-1" />

    <Button variant="ghost" size="xs" class="cursor-pointer text-[10px]" onclick={onpreview}>
      <Eye size={13} class="me-1" />
      معاينة
    </Button>
    <Button variant="lime" size="xs" class="cursor-pointer text-[10px]" onclick={onsave} disabled={!store.isDirty}>
      <Save size={13} class="me-1" />
      حفظ
    </Button>
  </div>
</div>
</TooltipProvider>

<style>
  .ctx-toolbar {
    height: 42px;
    display: flex;
    align-items: center;
    gap: 4px;
    padding: 0 8px;
    background: var(--color-card);
    border-bottom: 1px solid color-mix(in srgb, var(--color-border) 30%, transparent);
    flex-shrink: 0;
    user-select: none;
    overflow-x: auto;
  }

  .ctx-toolbar::-webkit-scrollbar { display: none; }

  .ctx-group {
    display: flex;
    align-items: center;
    gap: 3px;
  }

  .ctx-center {
    flex-grow: 1;
    display: flex;
    align-items: center;
    gap: 4px;
    min-width: 0;
    overflow-x: auto;
  }

  .ctx-center::-webkit-scrollbar { display: none; }

  .ctx-title {
    font-size: 12px;
    font-weight: 700;
    color: var(--color-foreground);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 180px;
  }

  .ctx-dirty-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: var(--color-primary);
    flex-shrink: 0;
  }

  .ctx-label {
    font-size: 9px;
    color: var(--color-muted-foreground);
    white-space: nowrap;
  }

  .ctx-number-input {
    display: flex;
    align-items: center;
    gap: 0;
  }

  .ctx-num-btn {
    width: 22px;
    height: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: transparent;
    border: 1px solid color-mix(in srgb, var(--color-border) 40%, transparent);
    color: var(--color-muted-foreground);
    cursor: pointer;
    transition: all 0.1s;
  }

  .ctx-num-btn:first-child { border-radius: 4px 0 0 4px; border-inline-end: none; }
  .ctx-num-btn:last-child { border-radius: 0 4px 4px 0; border-inline-start: none; }
  .ctx-num-btn:hover { background: var(--color-accent); color: var(--color-foreground); }

  .ctx-format-btn {
    width: 28px;
    height: 26px;
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

  .ctx-format-btn:hover { background: var(--color-accent); color: var(--color-foreground); }
  .ctx-format-btn.active { background: var(--color-primary); color: var(--color-primary-foreground); }

  .ctx-color-swatch {
    width: 24px;
    height: 24px;
    border: 2px solid color-mix(in srgb, var(--color-border) 50%, transparent);
    border-radius: 4px;
    cursor: pointer;
    padding: 0;
    background: none;
  }
  .ctx-color-swatch::-webkit-color-swatch-wrapper { padding: 1px; }
  .ctx-color-swatch::-webkit-color-swatch { border: none; border-radius: 2px; }

  .ctx-select {
    height: 24px;
    font-size: 10px;
    border: 1px solid color-mix(in srgb, var(--color-border) 40%, transparent);
    border-radius: 4px;
    background: var(--color-background);
    color: var(--color-foreground);
    padding: 0 6px;
    cursor: pointer;
  }
</style>
