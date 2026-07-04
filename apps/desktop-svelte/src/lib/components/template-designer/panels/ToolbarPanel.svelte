<script lang="ts">
/**
 * ToolbarPanel — Top toolbar for the template designer.
 * Contains tool selection, alignment, undo/redo, zoom, and view toggles.
 */
import type { EditorStore, EditorTool } from "../stores/editor-store.svelte";
import { Button } from "@orbis/ui-svelte/button";
import { Separator } from "@orbis/ui-svelte/separator";
import { Badge } from "@orbis/ui-svelte/badge";
import {
  Tooltip,
  TooltipTrigger,
  TooltipContent,
  TooltipProvider,
} from "@orbis/ui-svelte/tooltip";
import {
  MousePointer2,
  Type,
  ImageIcon,
  Square,
  LayoutGrid,
  Table,
  Barcode,
  QrCode,
  Minus,
  Hand,
  AlignLeft,
  AlignCenter,
  AlignRight,
  AlignStartVertical,
  AlignCenterVertical,
  AlignEndVertical,
  Undo2,
  Redo2,
  ZoomIn,
  ZoomOut,
  Maximize,
  Grid3x3,
  Ruler,
  Copy,
  Trash2,
  Group,
  Save,
  Eye,
} from "lucide-svelte";
import * as m from "$lib/paraglide/messages";

let {
  store,
  onsave,
  onpreview,
}: {
  store: EditorStore;
  onsave: () => void;
  onpreview: () => void;
} = $props();

const tools: { id: EditorTool; icon: any; label: string }[] = [
  { id: "select", icon: MousePointer2, label: "Select" },
  { id: "text", icon: Type, label: "Text" },
  { id: "image", icon: ImageIcon, label: "Image" },
  { id: "shape", icon: Square, label: "Shape" },
  { id: "frame", icon: LayoutGrid, label: "Frame" },
  { id: "table", icon: Table, label: "Table" },
  { id: "barcode", icon: Barcode, label: "Barcode" },
  { id: "qrcode", icon: QrCode, label: "QR Code" },
  { id: "separator", icon: Minus, label: "Separator" },
  { id: "pan", icon: Hand, label: "Pan" },
];

function selectTool(id: EditorTool) {
  if (id === "select" || id === "pan") {
    store.activeTool = id;
  } else {
    // Insert mode: add element and switch back to select
    store.addElement(id.charAt(0).toUpperCase() + id.slice(1));
    store.activeTool = "select";
  }
}
</script>

<div class="h-11 border-b border-border bg-card/60 px-2 flex items-center gap-1 flex-nowrap select-none overflow-x-auto scrollbar-hide">
  <!-- Tool Buttons -->
  <TooltipProvider delayDuration={300}>
    <div class="flex items-center gap-0.5">
      {#each tools as tool (tool.id)}
        <Tooltip>
          <TooltipTrigger>
            <Button
              variant={store.activeTool === tool.id ? "default" : "ghost"}
              size="icon-xs"
              class="cursor-pointer"
              onclick={() => selectTool(tool.id)}
            >
              <svelte:component this={tool.icon} size={14} />
            </Button>
          </TooltipTrigger>
          <TooltipContent>{tool.label}</TooltipContent>
        </Tooltip>
      {/each}
    </div>

    <Separator orientation="vertical" class="h-6 mx-1" />

    <!-- Alignment Tools -->
    <div class="flex items-center gap-0.5">
      {#each [
        { icon: AlignLeft, action: "left", label: "Align Left" },
        { icon: AlignCenter, action: "center", label: "Align Center" },
        { icon: AlignRight, action: "right", label: "Align Right" },
        { icon: AlignStartVertical, action: "top", label: "Align Top" },
        { icon: AlignCenterVertical, action: "middle", label: "Align Middle" },
        { icon: AlignEndVertical, action: "bottom", label: "Align Bottom" },
      ] as item (item.action)}
        <Tooltip>
          <TooltipTrigger>
            <Button
              variant="ghost"
              size="icon-xs"
              class="cursor-pointer"
              disabled={store.selectedIds.length < 2}
            >
              <svelte:component this={item.icon} size={13} />
            </Button>
          </TooltipTrigger>
          <TooltipContent>{item.label}</TooltipContent>
        </Tooltip>
      {/each}
    </div>

    <Separator orientation="vertical" class="h-6 mx-1" />

    <!-- Edit Actions -->
    <div class="flex items-center gap-0.5">
      <Tooltip>
        <TooltipTrigger>
          <Button variant="ghost" size="icon-xs" class="cursor-pointer" onclick={() => store.duplicateSelected()} disabled={store.selectedIds.length === 0}>
            <Copy size={13} />
          </Button>
        </TooltipTrigger>
        <TooltipContent>Duplicate (Ctrl+D)</TooltipContent>
      </Tooltip>

      <Tooltip>
        <TooltipTrigger>
          <Button variant="ghost" size="icon-xs" class="cursor-pointer" onclick={() => store.deleteSelected()} disabled={store.selectedIds.length === 0}>
            <Trash2 size={13} />
          </Button>
        </TooltipTrigger>
        <TooltipContent>Delete (Del)</TooltipContent>
      </Tooltip>
    </div>

    <Separator orientation="vertical" class="h-6 mx-1" />

    <!-- Undo / Redo -->
    <div class="flex items-center gap-0.5">
      <Tooltip>
        <TooltipTrigger>
          <Button variant="ghost" size="icon-xs" class="cursor-pointer" onclick={() => store.undo()} disabled={!store.canUndo}>
            <Undo2 size={14} />
          </Button>
        </TooltipTrigger>
        <TooltipContent>Undo (Ctrl+Z)</TooltipContent>
      </Tooltip>

      <Tooltip>
        <TooltipTrigger>
          <Button variant="ghost" size="icon-xs" class="cursor-pointer" onclick={() => store.redo()} disabled={!store.canRedo}>
            <Redo2 size={14} />
          </Button>
        </TooltipTrigger>
        <TooltipContent>Redo (Ctrl+Y)</TooltipContent>
      </Tooltip>
    </div>

    <!-- Spacer -->
    <div class="flex-grow"></div>

    <!-- View Controls -->
    <div class="flex items-center gap-1">
      <Tooltip>
        <TooltipTrigger>
          <Button variant="ghost" size="icon-xs" class="cursor-pointer" onclick={() => store.gridVisible = !store.gridVisible}>
            <Grid3x3 size={13} class={store.gridVisible ? "text-primary" : ""} />
          </Button>
        </TooltipTrigger>
        <TooltipContent>Grid</TooltipContent>
      </Tooltip>

      <Tooltip>
        <TooltipTrigger>
          <Button variant="ghost" size="icon-xs" class="cursor-pointer" onclick={() => store.rulersVisible = !store.rulersVisible}>
            <Ruler size={13} class={store.rulersVisible ? "text-primary" : ""} />
          </Button>
        </TooltipTrigger>
        <TooltipContent>Rulers</TooltipContent>
      </Tooltip>

      <Separator orientation="vertical" class="h-6 mx-1" />

      <!-- Zoom Controls -->
      <Button variant="ghost" size="icon-xs" class="cursor-pointer" onclick={() => store.zoomOut()}>
        <ZoomOut size={13} />
      </Button>
      <Badge variant="outline" class="text-[9px] font-mono min-w-[40px] justify-center">
        {store.zoomPercent}%
      </Badge>
      <Button variant="ghost" size="icon-xs" class="cursor-pointer" onclick={() => store.zoomIn()}>
        <ZoomIn size={13} />
      </Button>
      <Button variant="ghost" size="icon-xs" class="cursor-pointer" onclick={() => store.zoomToFit(800, 600)}>
        <Maximize size={13} />
      </Button>
    </div>

    <Separator orientation="vertical" class="h-6 mx-1" />

    <!-- Preview & Save -->
    <div class="flex items-center gap-1 shrink-0">
      <Button variant="outline" size="xs" class="cursor-pointer" onclick={onpreview}>
        <Eye size={13} class="me-1" />
        Preview
      </Button>
      <Button variant="lime" size="xs" class="cursor-pointer" onclick={onsave} disabled={!store.isDirty}>
        <Save size={13} class="me-1" />
        Save
      </Button>
    </div>
  </TooltipProvider>
</div>
