<script lang="ts">
/**
 * ToolbarPanel — Top toolbar for the template designer.
 * Contains tool selection, alignment, undo/redo, zoom, and view toggles.
 */
import type { EditorStore, EditorTool } from "../stores/editor-store.svelte";
import { Button } from "@orbis/ui/button";
import { Separator } from "@orbis/ui/separator";
import { Badge } from "@orbis/ui/badge";
import {
  Tooltip,
  TooltipTrigger,
  TooltipContent,
  TooltipProvider,
} from "@orbis/ui/tooltip";
import { AdjustmentsHorizontal, AlignHorizontal, AlignVertical, ArrowForward, Bold, CheckBox, CheckSquare, Clipboard, Collapse, CursorArrowRays, Expand, Eye, FloppyDisk, Grid, H1, Hashtag, Minus, Pencil, Photo, Trash, UndoAction, Wrench } from "@orbis/icons";
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

const tools: {
  id: EditorTool;
  icon: any;
  label: string;
  draggable: boolean;
}[] = [
  { id: "select", icon: CursorArrowRays, label: "Select", draggable: false },
  { id: "text", icon: H1, label: "Text", draggable: true },
  { id: "image", icon: Photo, label: "Image", draggable: true },
  { id: "shape", icon: CheckBox, label: "Shape", draggable: true },
  { id: "frame", icon: Grid, label: "Frame", draggable: true },
  { id: "table", icon: Grid, label: "Table", draggable: true },
  { id: "barcode", icon: Hashtag, label: "Barcode", draggable: true },
  { id: "qrcode", icon: CheckSquare, label: "QR Code", draggable: true },
  { id: "separator", icon: Minus, label: "Separator", draggable: true },
  { id: "pan", icon: Wrench, label: "Pan", draggable: false },
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

function handleToolDragStart(e: DragEvent, toolId: string) {
  const elementType = toolId.charAt(0).toUpperCase() + toolId.slice(1);
  e.dataTransfer?.setData("application/x-docforge-element", elementType);
  e.dataTransfer?.setData("text/plain", elementType);
  if (e.dataTransfer) e.dataTransfer.effectAllowed = "copy";
}
</script>

<div class="h-11 border-b border-border bg-card/60 px-2 flex items-center gap-1 flex-nowrap select-none overflow-x-auto scrollbar-hide">
  <!-- Tool Buttons -->
  <TooltipProvider delayDuration={300}>
    <div class="flex items-center gap-0.5">
      {#each tools as tool (tool.id)}
        <Tooltip>
          <TooltipTrigger>
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <div
              draggable={tool.draggable}
              ondragstart={(e) => tool.draggable && handleToolDragStart(e, tool.id)}
              class={tool.draggable ? 'cursor-grab' : ''}
            >
              <Button
                variant={store.activeTool === tool.id ? "default" : "ghost"}
                size="icon-xs"
                class="cursor-pointer pointer-events-auto"
                onclick={() => selectTool(tool.id)}
              >
                <svelte:component this={tool.icon} size={14} />
              </Button>
            </div>
          </TooltipTrigger>
          <TooltipContent>{tool.label}</TooltipContent>
        </Tooltip>
      {/each}
    </div>

    <Separator orientation="vertical" class="h-6 mx-1" />

    <!-- Alignment Tools -->
    <div class="flex items-center gap-0.5">
      {#each [
        { icon: AlignHorizontal, action: "left", label: "Align Left" },
        { icon: AlignHorizontal, action: "center", label: "Align Center" },
        { icon: AlignHorizontal, action: "right", label: "Align Right" },
        { icon: AlignVertical, action: "top", label: "Align Top" },
        { icon: AlignHorizontalVertical, action: "middle", label: "Align Middle" },
        { icon: AlignVertical, action: "bottom", label: "Align Bottom" },
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

    <!-- Pencil Actions -->
    <div class="flex items-center gap-0.5">
      <Tooltip>
        <TooltipTrigger>
          <Button variant="ghost" size="icon-xs" class="cursor-pointer" onclick={() => store.duplicateSelected()} disabled={store.selectedIds.length === 0}>
            <Clipboard size={13} />
          </Button>
        </TooltipTrigger>
        <TooltipContent>Duplicate (Ctrl+D)</TooltipContent>
      </Tooltip>

      <Tooltip>
        <TooltipTrigger>
          <Button variant="ghost" size="icon-xs" class="cursor-pointer" onclick={() => store.deleteSelected()} disabled={store.selectedIds.length === 0}>
            <Trash size={13} />
          </Button>
        </TooltipTrigger>
        <TooltipContent>Delete (Del)</TooltipContent>
      </Tooltip>
    </div>

    <Separator orientation="vertical" class="h-6 mx-1" />

    <!-- Undo -->
    <div class="flex items-center gap-0.5">
      <Tooltip>
        <TooltipTrigger>
          <Button variant="ghost" size="icon-xs" class="cursor-pointer" onclick={() => store.undo()} disabled={!store.canUndo}>
            <UndoAction size={14} />
          </Button>
        </TooltipTrigger>
        <TooltipContent>Undo (Ctrl+Z)</TooltipContent>
      </Tooltip>

      <Tooltip>
        <TooltipTrigger>
          <Button variant="ghost" size="icon-xs" class="cursor-pointer" onclick={() => store.redo()} disabled={!store.canRedo}>
            <ArrowForward size={14} />
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
            <Grid size={13} class={store.gridVisible ? "text-primary" : ""} />
          </Button>
        </TooltipTrigger>
        <TooltipContent>Grid</TooltipContent>
      </Tooltip>

      <Tooltip>
        <TooltipTrigger>
          <Button variant="ghost" size="icon-xs" class="cursor-pointer" onclick={() => store.rulersVisible = !store.rulersVisible}>
            <AdjustmentsHorizontal size={13} class={store.rulersVisible ? "text-primary" : ""} />
          </Button>
        </TooltipTrigger>
        <TooltipContent>Rulers</TooltipContent>
      </Tooltip>

      <Separator orientation="vertical" class="h-6 mx-1" />

      <!-- Zoom Controls -->
      <Button variant="ghost" size="icon-xs" class="cursor-pointer" onclick={() => store.zoomOut()}>
        <Collapse size={13} />
      </Button>
      <Badge variant="outline" class="text-[9px] font-mono min-w-[40px] justify-center">
        {store.zoomPercent}%
      </Badge>
      <Button variant="ghost" size="icon-xs" class="cursor-pointer" onclick={() => store.zoomIn()}>
        <Expand size={13} />
      </Button>
      <Button variant="ghost" size="icon-xs" class="cursor-pointer" onclick={() => store.zoomToFit(800, 600)}>
        <Expand size={13} />
      </Button>
    </div>

    <Separator orientation="vertical" class="h-6 mx-1" />

    <!-- Preview & FloppyDisk -->
    <div class="flex items-center gap-1 shrink-0">
      <Button variant="outline" size="xs" class="cursor-pointer" onclick={onpreview}>
        <Eye size={13} class="me-1" />
        Preview
      </Button>
      <Button variant="lime" size="xs" class="cursor-pointer" onclick={onsave} disabled={!store.isDirty}>
        <FloppyDisk size={13} class="me-1" />
        FloppyDisk
      </Button>
    </div>
  </TooltipProvider>
</div>
