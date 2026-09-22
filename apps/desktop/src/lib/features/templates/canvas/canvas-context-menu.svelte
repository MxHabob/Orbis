<script lang="ts">
import * as m from "$lib/paraglide/messages";
/**
 * CanvasContextMenu — Right-click context menu for canvas elements.
 */
import type { EditorStore } from "../stores/editor-store.svelte";
import { copyNodes, cutNodes, pasteNodes } from "../stores/clipboard-manager";
import { AdjustmentsHorizontal, BarsArrowUp, Clip, Clipboard, Eye, EyeSlash, Keyframes, Lock, Pencil, Trash } from "@orbis/icons";

let {
  store,
  x,
  y,
  visible,
  onclose,
}: {
  store: EditorStore;
  x: number;
  y: number;
  visible: boolean;
  onclose: () => void;
} = $props();

const hasSelection = $derived(store.selectedIds.length > 0);
const selectedNode = $derived(store.selectedNode);
const isLocked = $derived(selectedNode?.locked === true);
const isHidden = $derived(selectedNode?.visible === false);

async function handleCopy() {
  await copyNodes(store.document, store.selectedIds);
  onclose();
}

async function handleCut() {
  const cmd = await cutNodes(store.document, store.selectedIds);
  if (cmd) {
    store.executeCommand(cmd);
    store.clearSelection();
  }
  onclose();
}

async function handlePaste() {
  const parentId = store.activePage?.id;
  if (!parentId) return;
  const cmd = await pasteNodes(parentId);
  if (cmd) {
    store.executeCommand(cmd);
  }
  onclose();
}

function handleDelete() {
  store.deleteSelected();
  onclose();
}

function handleDuplicate() {
  store.duplicateSelected();
  onclose();
}

function handleToggleLock() {
  store.updateSelectedProps({ locked: !isLocked });
  onclose();
}

function handleToggleVisibility() {
  store.updateSelectedProps({ visible: isHidden });
  onclose();
}

function handleBringToFront() {
  store.updateSelectedProps({ zIndex: 999 });
  onclose();
}

function handleSendToBack() {
  store.updateSelectedProps({ zIndex: 0 });
  onclose();
}

function handleSelectAll() {
  store.selectAll();
  onclose();
}
</script>

{#if visible}
  <!-- Backdrop -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="fixed inset-0 z-50"
    onclick={onclose}
    oncontextmenu={(e) => { e.preventDefault(); onclose(); }}
  ></div>

  <!-- Menu -->
  <div
    class="fixed z-50 min-w-[180px] rounded-lg border border-border bg-popover p-1 shadow-lg animate-in fade-in zoom-in-95 duration-100"
    style="left: {x}px; top: {y}px;"
  >
    {#if hasSelection}
      <button type="button" class="context-menu-item" onclick={handleCopy}>
        <Clipboard size={12} />
        <span>{m.template_action_copy()}</span>
        <span class="context-menu-shortcut">Ctrl+C</span>
      </button>
      <button type="button" class="context-menu-item" onclick={handleCut}>
        <Clip size={12} />
        <span>{m.template_action_cut()}</span>
        <span class="context-menu-shortcut">Ctrl+X</span>
      </button>
    {/if}

    <button type="button" class="context-menu-item" onclick={handlePaste}>
      <Clipboard size={12} />
      <span>{m.template_action_paste()}</span>
      <span class="context-menu-shortcut">Ctrl+V</span>
    </button>

    {#if hasSelection}
      <div class="h-px bg-border/50 my-1"></div>

      <button type="button" class="context-menu-item" onclick={handleDuplicate}>
        <Clipboard size={12} />
        <span>{m.template_action_duplicate()}</span>
        <span class="context-menu-shortcut">Ctrl+D</span>
      </button>

      <button type="button" class="context-menu-item text-destructive" onclick={handleDelete}>
        <Trash size={12} />
        <span>{m.template_action_delete()}</span>
        <span class="context-menu-shortcut">Del</span>
      </button>

      <div class="h-px bg-border/50 my-1"></div>

      <button type="button" class="context-menu-item" onclick={handleBringToFront}>
        <BarsArrowUp size={12} />
        <span>{m.template_action_bring_to_front()}</span>
      </button>

      <button type="button" class="context-menu-item" onclick={handleSendToBack}>
        <BarsArrowUp size={12} />
        <span>{m.template_action_send_to_back()}</span>
      </button>

      <div class="h-px bg-border/50 my-1"></div>

      <button type="button" class="context-menu-item" onclick={handleToggleLock}>
        {#if isLocked}
          <Lock size={12} />
          <span>{m.template_action_unlock()}</span>
        {:else}
          <Lock size={12} />
          <span>{m.template_action_lock()}</span>
        {/if}
      </button>

      <button type="button" class="context-menu-item" onclick={handleToggleVisibility}>
        {#if isHidden}
          <Eye size={12} />
          <span>{m.template_action_show()}</span>
        {:else}
          <EyeSlash size={12} />
          <span>{m.template_action_hide()}</span>
        {/if}
      </button>
    {:else}
      <div class="h-px bg-border/50 my-1"></div>
      <button type="button" class="context-menu-item" onclick={handleSelectAll}>
        <Keyframes size={12} />
        <span>{m.template_action_select_all()}</span>
        <span class="context-menu-shortcut">Ctrl+A</span>
      </button>
    {/if}
  </div>
{/if}

<style>
  .context-menu-item {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    padding: 5px 8px;
    border-radius: 4px;
    font-size: 11px;
    color: var(--color-foreground);
    cursor: pointer;
    transition: background-color 0.1s;
    border: none;
    background: none;
    text-align: start;
  }
  .context-menu-item:hover {
    background-color: var(--color-accent);
  }
  .context-menu-shortcut {
    margin-inline-start: auto;
    font-size: 9px;
    color: var(--color-muted-foreground);
    font-family: monospace;
  }
</style>
