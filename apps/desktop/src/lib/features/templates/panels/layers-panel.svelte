<script lang="ts">
import * as m from "$lib/paraglide/messages";
/**
 * LayersPanel — Tree view of document elements with reordering.
 * Shows the hierarchy of the active page.
 */
import type { EditorStore } from "../stores/editor-store.svelte";
import type { TemplateNode } from "@docforge/core";
import { isContainerNode } from "@docforge/core";
import { Button } from "@orbis/ui/button";
import { AdjustmentsHorizontal, Bold, CheckBox, ChevronDown, ChevronRight, DocumentPage, Eye, EyeSlash, Grid, Lock, Minus, Photo, Plus } from "@orbis/icons";

let {
  store,
}: {
  store: EditorStore;
} = $props();

// Collapsed state for tree nodes
let collapsed = $state<Set<string>>(new Set());

function toggleCollapse(id: string) {
  const next = new Set(collapsed);
  if (next.has(id)) next.delete(id);
  else next.add(id);
  collapsed = next;
}

function getIcon(type: string) {
  switch (type) {
    case "Text":
      return Type;
    case "Image":
      return ImageIcon;
    case "Shape":
      return CheckBox;
    case "Frame":
    case "Grid":
      return Grid;
    case "Table":
      return Grid;
    case "Barcode":
      return Barcode;
    case "QRCode":
      return QrCode;
    case "Separator":
      return Minus;
    default:
      return DocumentPage;
  }
}

function getIconColor(type: string): string {
  switch (type) {
    case "Text":
      return "text-emerald-500";
    case "Image":
      return "text-violet-500";
    case "Shape":
      return "text-blue-500";
    case "Frame":
    case "Grid":
      return "text-amber-500";
    case "Table":
      return "text-cyan-500";
    case "Barcode":
      return "text-orange-500";
    case "QRCode":
      return "text-purple-500";
    default:
      return "text-muted-foreground";
  }
}
</script>

<div class="flex flex-col h-full text-start">
  <!-- Header -->
  <div class="px-3 py-2 border-b border-border/40 flex items-center justify-between">
    <h3 class="text-xs font-bold text-foreground">{m.template_panel_layers()}</h3>
    <div class="flex items-center gap-1">
      <span class="text-[9px] text-muted-foreground">
        {store.activePageIndex + 1}/{store.pageCount}
      </span>
      <Button variant="ghost" size="icon-xs" class="cursor-pointer h-5 w-5" onclick={() => store.addPage()}>
        <Plus size={11} />
      </Button>
    </div>
  </div>

  <!-- Pages tabs -->
  {#if store.pageCount > 1}
    <div class="flex items-center gap-1 px-2 py-1.5 border-b border-border/30 overflow-x-auto scrollbar-hide">
      {#each store.pages as page, i (page.id)}
        <button
          type="button"
          class="px-2 py-0.5 rounded text-[9px] transition-all cursor-pointer shrink-0
            {store.activePageIndex === i
              ? 'bg-primary text-primary-foreground font-semibold'
              : 'bg-muted/30 text-muted-foreground hover:bg-accent'}"
          onclick={() => store.setActivePage(i)}
        >
          {i + 1}
        </button>
      {/each}
    </div>
  {/if}

  <!-- Tree -->
  <div class="flex-grow overflow-y-auto scrollbar-hide px-1 py-1">
    {#if store.activePage}
      {#snippet treeNode(node: TemplateNode, depth: number)}
        {@const isContainer = isContainerNode(node)}
        {@const isCollapsed = collapsed.has(node.id)}
        {@const isSelected = store.selectedIds.includes(node.id)}
        {@const IconComp = getIcon(node.type)}
        {@const iconColor = getIconColor(node.type)}

        <div class="flex flex-col">
          <!-- Node row -->
          <button
            type="button"
            class="w-full flex items-center gap-1.5 py-1 px-1.5 rounded text-[10px] transition-all cursor-pointer group
              {isSelected ? 'bg-primary/10 text-primary font-semibold' : 'text-muted-foreground hover:bg-accent/50'}"
            style="padding-inline-start: {depth * 12 + 6}px;"
            onclick={() => store.select(node.id)}
          >
            <!-- Collapse toggle -->
            {#if isContainer && (node as any).children?.length > 0}
              <!-- svelte-ignore a11y_no_static_element_interactions -->
              <span
                class="shrink-0 cursor-pointer"
                onclick={(e) => { e.stopPropagation(); toggleCollapse(node.id); }}
              >
                {#if isCollapsed}
                  <ChevronRight size={10} />
                {:else}
                  <ChevronDown size={10} />
                {/if}
              </span>
            {:else}
              <span class="w-[10px] shrink-0"></span>
            {/if}

            <!-- Icon -->
            <svelte:component this={IconComp} size={11} class="shrink-0 {iconColor}" />

            <!-- Name -->
            <span class="truncate flex-grow">
              {node.name || node.id}
            </span>

            <!-- Visibility toggle (on hover) -->
            <span class="opacity-0 group-hover:opacity-100 transition-opacity shrink-0">
              {#if node.visible === false}
                <EyeSlash size={10} class="text-muted-foreground/50" />
              {:else}
                <Eye size={10} class="text-muted-foreground/30" />
              {/if}
            </span>

            <!-- Lock indicator -->
            {#if node.locked}
              <Lock size={9} class="text-muted-foreground/50 shrink-0" />
            {/if}
          </button>

          <!-- Children (if expanded) -->
          {#if isContainer && !isCollapsed && (node as any).children}
            {#each (node as any).children as child (child.id)}
              {@render treeNode(child, depth + 1)}
            {/each}
          {/if}
        </div>
      {/snippet}

      {#each store.activePage.children as child (child.id)}
        {@render treeNode(child, 0)}
      {/each}

      {#if store.activePage.children.length === 0}
        <div class="text-center py-8 text-[10px] text-muted-foreground">
          {m.template_msg_empty_page()}
        </div>
      {/if}
    {/if}
  </div>
</div>
