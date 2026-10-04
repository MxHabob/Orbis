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
  { id: "select", icon: CursorArrowRays, label: m.shared_ui_tool_select(), draggable: false },
  { id: "text", icon: H1, label: m.shared_ui_tool_text(), draggable: true },
  { id: "image", icon: Photo, label: m.shared_ui_tool_image(), draggable: true },
  { id: "shape", icon: CheckBox, label: m.shared_ui_tool_shape(), draggable: true },
  { id: "frame", icon: Grid, label: m.shared_ui_tool_frame(), draggable: true },
  { id: "table", icon: Grid, label: m.shared_ui_tool_table(), draggable: true },
  { id: "barcode", icon: Hashtag, label: m.shared_ui_tool_barcode(), draggable: true },
  { id: "qrcode", icon: CheckSquare, label: m.shared_ui_tool_qr_code(), draggable: true },
  { id: "separator", icon: Minus, label: m.shared_ui_tool_separator(), draggable: true },
  { id: "pan", icon: Wrench, label: m.shared_ui_tool_pan(), draggable: false },
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
        {@const ToolIcon = tool.icon}
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
                <ToolIcon size={14} />
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
        { icon: AlignHorizontal, action: "left", label: m.shared_ui_align_left() },
        { icon: AlignHorizontal, action: "center", label: m.shared_ui_align_center() },
        { icon: AlignHorizontal, action: "right", label: m.shared_ui_align_right() },
        { icon: AlignVertical, action: "top", label: m.shared_ui_align_top() },
        { icon: AlignVertical, action: "middle", label: m.shared_ui_align_middle() },
        { icon: AlignVertical, action: "bottom", label: m.shared_ui_align_bottom() },
      ] as item (item.action)}
        {@const ItemIcon = item.icon}
        <Tooltip>
          <TooltipTrigger>
            <Button
              variant="ghost"
              size="icon-xs"
              class="cursor-pointer"
              disabled={store.selectedIds.length < 2}
            >
              <ItemIcon size={13} />
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
        <TooltipContent>{m.shared_ui_duplicate_shortcut()}</TooltipContent>
      </Tooltip>

      <Tooltip>
        <TooltipTrigger>
          <Button variant="ghost" size="icon-xs" class="cursor-pointer" onclick={() => store.deleteSelected()} disabled={store.selectedIds.length === 0}>
            <Trash size={13} />
          </Button>
        </TooltipTrigger>
        <TooltipContent>{m.shared_ui_delete_shortcut()}</TooltipContent>
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
        <TooltipContent>{m.shared_ui_undo_shortcut()}</TooltipContent>
      </Tooltip>

      <Tooltip>
        <TooltipTrigger>
          <Button variant="ghost" size="icon-xs" class="cursor-pointer" onclick={() => store.redo()} disabled={!store.canRedo}>
            <ArrowForward size={14} />
          </Button>
        </TooltipTrigger>
        <TooltipContent>{m.shared_ui_redo_shortcut()}</TooltipContent>
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
        <TooltipContent>{m.shared_ui_grid()}</TooltipContent>
      </Tooltip>

      <Tooltip>
        <TooltipTrigger>
          <Button variant="ghost" size="icon-xs" class="cursor-pointer" onclick={() => store.rulersVisible = !store.rulersVisible}>
            <AdjustmentsHorizontal size={13} class={store.rulersVisible ? "text-primary" : ""} />
          </Button>
        </TooltipTrigger>
        <TooltipContent>{m.shared_ui_rulers()}</TooltipContent>
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
        {m.shared_ui_preview()}
      </Button>
      <Button variant="lime" size="xs" class="cursor-pointer" onclick={onsave} disabled={!store.isDirty}>
        <FloppyDisk size={13} class="me-1" />
        {m.common_action_save()}
      </Button>
    </div>
  </TooltipProvider>
</div>
