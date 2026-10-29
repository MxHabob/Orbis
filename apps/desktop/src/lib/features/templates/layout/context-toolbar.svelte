<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { untrack } from "svelte";
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
import {
  AdjustmentsHorizontal,
  AlignHorizontal,
  AlignVertical,
  ArrowForward,
  ArrowLeft,
  Bold,
  Component,
  Eye,
  FloppyDisk,
  Grid,
  Italic,
  Knobs,
  Minus,
  Plus,
  Underline,
  UndoAction,
} from "@orbis/icons";
import { Badge } from "@orbis/ui/badge";
import { Button } from "@orbis/ui/button";
import { ComboboxDropdown } from "@orbis/ui/combobox-dropdown";
import { Input } from "@orbis/ui/input";
import { Popover, PopoverContent, PopoverTrigger } from "@orbis/ui/popover";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
} from "@orbis/ui/select";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@orbis/ui/tooltip";
import type { EditorStore } from "../stores/editor-store.svelte";

let {
  store,
  templateName = "",
  isPropertiesVisible = false,
  ontoggleproperties,
  onback,
  onsave,
  onpreview,
  onrename,
}: {
  store: EditorStore;
  templateName?: string;
  isPropertiesVisible?: boolean;
  ontoggleproperties?: () => void;
  onback?: () => void;
  onsave: () => void;
  onpreview: () => void;
  onrename?: (newName: string) => void;
} = $props();

const FONT_FAMILIES = [
  { id: "rubik", label: "Rubik", value: "Rubik" },
  { id: "tahoma", label: "Tahoma", value: "Tahoma" },
  { id: "arial", label: "Arial", value: "Arial" },
];

const FONT_SIZES = [
  6, 8, 10, 12, 14, 16, 18, 21, 24, 28, 32, 36, 42, 48, 56, 64, 72, 80, 88, 96,
  104, 120, 144,
];
let fontSizePopoverOpen = $state(false);

const node = $derived(store.selectedNode);
const nodeProps = $derived(
  node && "props" in node ? (node as any).props || {} : {},
);
const hasSelection = $derived(store.selectedIds.length > 0);
const isMultiSelect = $derived(store.selectedIds.length > 1);

const allSelectedTypes = $derived(
  Array.from(new Set(store.selectedNodes.map((n) => n.type))),
);
const commonType = $derived(
  allSelectedTypes.length === 1 ? allSelectedTypes[0] : null,
);

// Use commonType if multiple items are selected so we can show text/shape tools for multiple items
const nodeType = $derived(isMultiSelect ? commonType : (node?.type ?? null));

function updateProp(key: string, value: unknown) {
  store.updateSelectedProps({ props: { ...nodeProps, [key]: value } } as any);
}

// Keep track of local name value to prevent cursor jump during typing
let editingName = $state(untrack(() => templateName));
$effect(() => {
  editingName = templateName;
});

function handleAlign(
  type: "left" | "center" | "right" | "top" | "middle" | "bottom",
) {
  if (store.selectedNodes.length < 2) return;

  let minX = Infinity,
    minY = Infinity,
    maxX = -Infinity,
    maxY = -Infinity;

  for (const node of store.selectedNodes) {
    const pos = (node as any).position;
    if (pos) {
      if (pos.x < minX) minX = pos.x;
      if (pos.y < minY) minY = pos.y;
      if (pos.x + (pos.width || 0) > maxX) maxX = pos.x + (pos.width || 0);
      if (pos.y + (pos.height || 0) > maxY) maxY = pos.y + (pos.height || 0);
    }
  }

  if (minX === Infinity) return;

  const centerX = minX + (maxX - minX) / 2;
  const centerY = minY + (maxY - minY) / 2;

  const updates = [];
  for (const node of store.selectedNodes) {
    const pos = (node as any).position;
    if (pos) {
      let newX = pos.x;
      let newY = pos.y;
      const w = pos.width || 0;
      const h = pos.height || 0;

      if (type === "left") newX = minX;
      else if (type === "center") newX = centerX - w / 2;
      else if (type === "right") newX = maxX - w;
      else if (type === "top") newY = minY;
      else if (type === "middle") newY = centerY - h / 2;
      else if (type === "bottom") newY = maxY - h;

      if (newX !== pos.x || newY !== pos.y) {
        updates.push({
          id: node.id,
          x: Math.round(newX * 10) / 10,
          y: Math.round(newY * 10) / 10,
        });
      }
    }
  }

  if (updates.length > 0) {
    store.moveNodes(updates);
  }
}
</script>

<TooltipProvider delayDuration={200}>
<div class="ctx-toolbar">
  <!-- Left: Back + Title -->
  <div class="ctx-group shrink-0">
    {#if onback}
      <Tooltip>
        <TooltipTrigger>
          <Button variant="outline" size="icon-sm" class="cursor-pointer" onclick={onback}>
            <ArrowLeft size={15} class="rtl:rotate-180" />
          </Button>
        </TooltipTrigger>
        <TooltipContent>{m.template_tooltip_back()}</TooltipContent>
      </Tooltip>
    {/if}
    {#if onrename}
      <div class="ctx-title-wrapper" title={m.common_action_edit()}>
        <span class="ctx-title-hidden" aria-hidden="true">{editingName || "..."}</span>
        <input
          type="text"
          dir="auto"
          class="ctx-title-input"
          bind:value={editingName}
          maxlength="50"
          onblur={(e) => onrename(e.currentTarget.value)}
          onkeydown={(e) => {
            if (e.key === 'Enter') {
              e.currentTarget.blur();
            }
          }}
          placeholder="..."
        />
      </div>
    {:else}
      <span class="ctx-title">{templateName || "..."}</span>
    {/if}
    <div class="ctx-dirty-dot" class:is-dirty={store.isDirty}></div>
  </div>

  <div class="ctx-divider"></div>

  <!-- Center: Contextual Controls -->
  <div class="ctx-center">
    {#if nodeType === "Text"}
      <!-- ═══ TEXT TOOLBAR ═══ -->
      <div class="ctx-group">
        <div class="w-[110px]">
          <ComboboxDropdown
            items={FONT_FAMILIES}
            selectedItem={FONT_FAMILIES.find(f => f.value === (nodeProps.fontFamily || "Rubik")) || { id: "custom", label: nodeProps.fontFamily || "Rubik", value: nodeProps.fontFamily || "Rubik" }}
            onSelect={(item) => updateProp("fontFamily", item.value)}
            class="h-8"
            placeholder={m.shared_ui_font()}
          />
        </div>
        <div class="ctx-number-input">
          <button type="button" class="ctx-num-btn" onclick={() => updateProp("fontSize", Math.max(6, (nodeProps.fontSize || 14) - 1))}>
            <Minus size={10} />
          </button>

          <Popover bind:open={fontSizePopoverOpen}>
            <PopoverTrigger>
              {#snippet child({ props })}
                <input
                  {...props}
                  type="number"
                  value={nodeProps.fontSize || 14}
                  oninput={(e) => updateProp("fontSize", parseInt(e.currentTarget.value) || 14)}
                  class="flex h-8 w-[48px] text-xs text-center rounded-md border border-transparent bg-transparent px-1 transition-colors hover:border-border focus:border-border focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
                  min="6" max="300"
                />
              {/snippet}
            </PopoverTrigger>
            <PopoverContent class="w-[80px] p-0" align="center">
              <div class="flex flex-col max-h-[250px] overflow-y-auto py-1 scrollbar-thin">
                {#each FONT_SIZES as size (size)}
                  <button
                    type="button"
                    class="px-2 py-1.5 text-xs text-center hover:bg-muted font-sans cursor-pointer transition-colors"
                    class:bg-muted={size === nodeProps.fontSize}
                    onclick={() => { updateProp("fontSize", size); fontSizePopoverOpen = false; }}
                  >
                    {size}
                  </button>
                {/each}
              </div>
            </PopoverContent>
          </Popover>

          <button type="button" class="ctx-num-btn" onclick={() => updateProp("fontSize", Math.min(300, (nodeProps.fontSize || 14) + 1))}>
            <Plus size={10} />
          </button>
        </div>
      </div>

      <div class="ctx-divider"></div>

      <div class="ctx-group">
        <button type="button" class="ctx-format-btn" class:active={nodeProps.fontWeight === "700" || nodeProps.fontWeight === "bold"} onclick={() => updateProp("fontWeight", nodeProps.fontWeight === "700" ? "400" : "700")}>
          <Bold size={13} />
        </button>
        <button type="button" class="ctx-format-btn" class:active={nodeProps.fontStyle === "italic"} onclick={() => updateProp("fontStyle", nodeProps.fontStyle === "italic" ? "normal" : "italic")}>
          <Italic size={13} />
        </button>
        <button type="button" class="ctx-format-btn" class:active={nodeProps.textDecoration === "underline"} onclick={() => updateProp("textDecoration", nodeProps.textDecoration === "underline" ? "none" : "underline")}>
          <Underline size={13} />
        </button>
      </div>

      <div class="ctx-divider"></div>

      <div class="ctx-group">
        <input
          type="color"
          value={nodeProps.color || "#000000"}
          oninput={(e) => updateProp("color", e.currentTarget.value)}
          class="ctx-color-swatch"
        />
      </div>

      <div class="ctx-divider"></div>

      <div class="ctx-group">
        <Tooltip>
          <TooltipTrigger>
            <button type="button" class="ctx-format-btn" onclick={() => {
              const current = nodeProps.textAlign || "start";
              const next = current === "start" ? "center" : current === "center" ? "end" : "start";
              updateProp("textAlign", next);
            }}>
              <AlignHorizontal size={13} />
            </button>
          </TooltipTrigger>
          <TooltipContent>{m.template_auto_alignment()}</TooltipContent>
        </Tooltip>
      </div>

    {:else if nodeType === "Shape"}
      <!-- ═══ SHAPE TOOLBAR ═══ -->
      <div class="ctx-group">
        <span class="ctx-label">{m.template_prop_fill()}</span>
        <input
          type="color"
          value={nodeProps.fillColor || "#e2e8f0"}
          oninput={(e) => updateProp("fillColor", e.currentTarget.value)}
          class="ctx-color-swatch"
        />
        <span class="ctx-label">{m.template_prop_border()}</span>
        <input
          type="color"
          value={nodeProps.strokeColor || "#94a3b8"}
          oninput={(e) => updateProp("strokeColor", e.currentTarget.value)}
          class="ctx-color-swatch"
        />
        <Input
          type="number"
          value={nodeProps.strokeWidth ?? 1}
          oninput={(e) => updateProp("strokeWidth", parseFloat(e.currentTarget.value) || 1)}
          class="h-8 w-[48px] text-xs"
          min="0" max="20" step="0.5"
        />
        <span class="ctx-label">{m.template_prop_corners()}</span>
        <Input
          type="number"
          value={nodeProps.borderRadius ?? 0}
          oninput={(e) => updateProp("borderRadius", parseFloat(e.currentTarget.value) || 0)}
          class="h-8 w-[48px] text-xs"
          min="0" max="50"
        />
      </div>

    {:else if nodeType === "Image"}
      <!-- ═══ IMAGE TOOLBAR ═══ -->
      <div class="ctx-group">
        <span class="ctx-label">{m.template_prop_fit()}</span>
        <Select
          type="single"
          value={nodeProps.fit || "contain"}
          onValueChange={(value) => value && updateProp("fit", value)}
        >
          <SelectTrigger class="h-8 min-w-[84px] rounded-md px-2 text-xs">
            {#if nodeProps.fit === "cover"}
              {m.shared_ui_fit_cover()}
            {:else if nodeProps.fit === "fill"}
              {m.shared_ui_fit_fill()}
            {:else if nodeProps.fit === "none"}
              {m.shared_ui_none()}
            {:else}
              {m.shared_ui_fit_contain()}
            {/if}
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="contain">{m.shared_ui_fit_contain()}</SelectItem>
            <SelectItem value="cover">{m.shared_ui_fit_cover()}</SelectItem>
            <SelectItem value="fill">{m.shared_ui_fit_fill()}</SelectItem>
            <SelectItem value="none">{m.shared_ui_none()}</SelectItem>
          </SelectContent>
        </Select>
        <span class="ctx-label">{m.template_color_opacity()}</span>
        <Input
          type="number"
          value={Math.round((nodeProps.opacity ?? 1) * 100)}
          oninput={(e) => updateProp("opacity", (parseInt(e.currentTarget.value) || 100) / 100)}
          class="h-8 w-[52px] text-xs"
          min="0" max="100"
        />
        <span class="text-[9px] text-muted-foreground">%</span>
      </div>

    {:else if (nodeType === "Barcode" || nodeType === "QRCode")}
      <!-- ═══ BARCODE TOOLBAR ═══ -->
      <div class="ctx-group">
        <span class="ctx-label">{m.template_prop_data()}</span>
        <Input
          type="text"
          value={nodeProps.data || ""}
          oninput={(e) => updateProp("data", e.currentTarget.value)}
          class="h-8 w-[180px] text-xs font-mono"
        />
        {#if nodeType === "Barcode"}
          <span class="ctx-label">{m.template_prop_format()}</span>
            <Select
              type="single"
              value={nodeProps.format || "Code128"}
              onValueChange={(value) => value && updateProp("format", value)}
          >
              <SelectTrigger class="h-8 min-w-[90px] rounded-md px-2 text-xs">
                {nodeProps.format || "Code128"}
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="Code128">Code 128</SelectItem>
                <SelectItem value="Code39">Code 39</SelectItem>
                <SelectItem value="EAN13">EAN-13</SelectItem>
                <SelectItem value="EAN8">EAN-8</SelectItem>
              </SelectContent>
            </Select>
        {/if}
      </div>

    {:else if isMultiSelect}
      <div class="ctx-group">
        <Badge variant="secondary" class="text-[9px]">{store.selectedIds.length}{m.templates_selected_items()}</Badge>
      </div>
    {:else if !hasSelection}
      <!-- ═══ DEFAULT / PAGE TOOLBAR ═══ -->
      <div class="ctx-group">
        <Badge variant="outline" class="text-[9px] font-mono">
          {store.document?.meta?.pageSetup?.size || "A4"}
          {store.document?.meta?.pageSetup?.orientation === "landscape" ? "▬" : "▮"}
        </Badge>
      </div>
    {/if}

    <!-- ═══ MULTI-SELECT APPENDED TOOLS ═══ -->
    {#if isMultiSelect}
      {#if nodeType !== null}
        <div class="ctx-divider"></div>
      {/if}
      <div class="ctx-group">
        <Tooltip>
          <TooltipTrigger>
            <button type="button" class="ctx-format-btn">
              <Component size={13} />
            </button>
          </TooltipTrigger>
          <TooltipContent>{m.template_auto_group()}</TooltipContent>
        </Tooltip>
      </div>
      <div class="ctx-group">
        <Popover>
          <PopoverTrigger>
            {#snippet child({ props })}
              <Button {...props} variant="ghost" size="sm" class="cursor-pointer text-xs h-8 px-2.5">
                {m.template_auto_position()}
              </Button>
            {/snippet}
          </PopoverTrigger>
          <PopoverContent class="w-auto p-2" align="center">
            <div class="flex items-center gap-1">
              {#each [
                { icon: AlignHorizontal, type: "left" as const },
                { icon: AlignHorizontal, type: "center" as const },
                { icon: AlignHorizontal, type: "right" as const },
                { icon: AlignVertical, type: "top" as const },
                { icon: AlignVertical, type: "middle" as const },
                { icon: AlignVertical, type: "bottom" as const },
              ] as btn, i (i)}
                {@const ButtonIcon = btn.icon}
                <button type="button" class="ctx-format-btn" onclick={() => handleAlign(btn.type)}>
                  <ButtonIcon size={13} />
                </button>
              {/each}
            </div>
          </PopoverContent>
        </Popover>
      </div>

    {/if}
  </div>

  <!-- Right: Always-visible actions -->
  <div class="ctx-group shrink-0">
    {#if ontoggleproperties}
      <Tooltip>
        <TooltipTrigger>
          <Button
            variant="ghost"
            size="icon-sm"
            class={isPropertiesVisible ? "cursor-pointer text-primary" : "cursor-pointer"}
            aria-pressed={isPropertiesVisible}
            onclick={ontoggleproperties}
          >
            <AdjustmentsHorizontal size={15} />
          </Button>
        </TooltipTrigger>
        <TooltipContent>{isPropertiesVisible ? m.template_inspector_hide_properties() : m.template_inspector_show_properties()}</TooltipContent>
      </Tooltip>
    {/if}
    <Tooltip>
      <TooltipTrigger>
        <Button variant="ghost" size="icon-xs" class="cursor-pointer" onclick={() => store.undo()} disabled={!store.canUndo}>
          <UndoAction size={14} />
        </Button>
      </TooltipTrigger>
      <TooltipContent>{m.template_tooltip_undo()}</TooltipContent>
    </Tooltip>
    <Tooltip>
      <TooltipTrigger>
        <Button variant="ghost" size="icon-xs" class="cursor-pointer" onclick={() => store.redo()} disabled={!store.canRedo}>
          <ArrowForward size={14} />
        </Button>
      </TooltipTrigger>
      <TooltipContent>{m.template_tooltip_redo()}</TooltipContent>
    </Tooltip>

    <div class="ctx-divider"></div>

    <Tooltip>
      <TooltipTrigger>
        <Button variant="ghost" size="icon-xs" class="cursor-pointer" onclick={() => store.gridVisible = !store.gridVisible}>
          <Grid size={13} class={store.gridVisible ? "text-primary" : ""} />
        </Button>
      </TooltipTrigger>
      <TooltipContent>{m.template_tooltip_grid()}</TooltipContent>
    </Tooltip>
    <Tooltip>
      <TooltipTrigger>
        <Button variant="ghost" size="icon-xs" class="cursor-pointer" onclick={() => store.snapEnabled = !store.snapEnabled}>
          <Knobs size={13} class={store.snapEnabled ? "text-primary" : ""} />
        </Button>
      </TooltipTrigger>
      <TooltipContent>{m.template_tooltip_snap()}</TooltipContent>
    </Tooltip>
    <Tooltip>
      <TooltipTrigger>
        <Button variant="ghost" size="icon-xs" class="cursor-pointer" onclick={() => store.rulersVisible = !store.rulersVisible}>
          <AdjustmentsHorizontal size={13} class={store.rulersVisible ? "text-primary" : ""} />
        </Button>
      </TooltipTrigger>
      <TooltipContent>{m.template_tooltip_rulers()}</TooltipContent>
    </Tooltip>

    <div class="ctx-divider"></div>

    <Button variant="ghost" size="xs" class="cursor-pointer text-[10px]" onclick={onpreview}>
      <Eye size={13} class="me-1" />
      {m.template_btn_preview()}
    </Button>
    <Button variant="lime" size="xs" class="cursor-pointer text-[10px]" onclick={onsave} disabled={!store.isDirty}>
      <FloppyDisk size={13} class="me-1" />
      {m.template_btn_save()}
    </Button>
  </div>
</div>
</TooltipProvider>

<style>
  .ctx-toolbar {
    height: 54px;
    min-height: 54px;
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 0 12px;
    background: var(--color-card);
    border-bottom: 1px solid var(--color-border);
    flex-shrink: 0;
    user-select: none;
    overflow-x: auto;
    z-index: 40;
  }

  .ctx-toolbar::-webkit-scrollbar { display: none; }

  .ctx-group {
    display: flex;
    align-items: center;
    gap: 6px;
    flex-shrink: 0;
  }

  .ctx-divider {
    width: 1px;
    height: 24px;
    background: var(--color-border);
    margin: 0 2px;
    flex-shrink: 0;
  }

  .ctx-center {
    flex-grow: 1;
    display: flex;
    align-items: center;
    gap: 7px;
    min-width: 0;
    overflow-x: auto;
  }

  .ctx-center::-webkit-scrollbar { display: none; }

  .ctx-title {
    font-size: 13px;
    font-weight: 650;
    color: var(--color-foreground);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 250px;
  }

  .ctx-title-wrapper {
    position: relative;
    display: inline-grid;
    align-items: center;
    min-width: 130px;
    max-width: 250px;
  }

  .ctx-title-hidden {
    visibility: hidden;
    white-space: pre;
    font-size: 13px;
    font-weight: 650;
    padding: 2px 4px;
  }

  .ctx-title-input {
    position: absolute;
    top: 0;
    inset-inline-start: 0;
    width: 100%;
    height: 100%;
    font-size: 13px;
    font-weight: 650;
    color: var(--color-foreground);
    background: transparent;
    border: 1px solid transparent;
    border-radius: 4px;
    padding: 2px 4px;
    outline: none;
    transition: all 0.2s;
    text-overflow: ellipsis;
  }

  .ctx-title-input:hover {
    border-color: color-mix(in srgb, var(--color-border) 60%, transparent);
  }

  .ctx-title-input:focus {
    border-color: var(--color-primary);
    background: var(--color-background);
  }

  .ctx-dirty-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: var(--color-primary);
    flex-shrink: 0;
    opacity: 0;
    transition: opacity 0.2s ease-in-out;
  }

  .ctx-dirty-dot.is-dirty {
    opacity: 1;
  }

  .ctx-label {
    font-size: 10px;
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
    width: 30px;
    height: 30px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: transparent;
    border: none;
    border-radius: 7px;
    color: var(--color-muted-foreground);
    cursor: pointer;
    transition: all 0.1s;
  }

  .ctx-format-btn:hover { background: var(--color-accent); color: var(--color-foreground); }
  .ctx-format-btn.active {
    background: color-mix(in srgb, var(--color-primary) 10%, var(--color-card));
    color: var(--color-primary);
  }

  .ctx-color-swatch {
    width: 28px;
    height: 28px;
    border: 2px solid color-mix(in srgb, var(--color-border) 50%, transparent);
    border-radius: 4px;
    cursor: pointer;
    padding: 0;
    background: none;
  }
  .ctx-color-swatch::-webkit-color-swatch-wrapper { padding: 1px; }
  .ctx-color-swatch::-webkit-color-swatch { border: none; border-radius: 2px; }

  @media (max-width: 900px) {
    .ctx-toolbar {
      gap: 5px;
      padding-inline: 8px;
    }

    .ctx-title-wrapper {
      min-width: 100px;
    }
  }

</style>
