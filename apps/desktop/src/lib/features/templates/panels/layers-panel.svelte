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
import {
  AdjustmentsHorizontal,
  Bold,
  CheckBox,
  CheckSquare,
  ChevronDown,
  ChevronRight,
  DocumentPage,
  Eye,
  EyeSlash,
  Grid,
  H1,
  Hashtag,
  Lock,
  Minus,
  Photo,
  Plus,
} from "@orbis/icons";

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
      return H1;
    case "Image":
      return Photo;
    case "Shape":
      return CheckBox;
    case "Frame":
    case "Grid":
      return Grid;
    case "Table":
      return Grid;
    case "Barcode":
      return Hashtag;
    case "QRCode":
      return CheckSquare;
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
  <div class="px-3 py-2.5 border-b border-border flex items-center justify-between">
    <h3 class="text-sm font-semibold text-foreground">{m.template_panel_layers()}</h3>
    <div class="flex items-center gap-1">
      <span class="text-[11px] text-muted-foreground">
        {store.activePageIndex + 1}/{store.pageCount}
      </span>
      <Button variant="ghost" size="icon-sm" class="cursor-pointer" onclick={() => store.addPage()}>
        <Plus size={14} />
      </Button>
    </div>
  </div>

  <!-- Pages tabs -->
  {#if store.pageCount > 1}
    <div class="flex items-center gap-1.5 px-3 py-2 border-b border-border overflow-x-auto scrollbar-hide">
      {#each store.pages as page, i (page.id)}
        <button
          type="button"
          class="px-2.5 py-1 rounded-md text-[11px] transition-colors cursor-pointer shrink-0
            {store.activePageIndex === i
              ? 'bg-muted text-foreground font-medium'
              : 'text-muted-foreground hover:bg-muted/60'}"
          onclick={() => store.setActivePage(i)}
        >
          {i + 1}
        </button>
      {/each}
    </div>
  {/if}

  <!-- Tree -->
  <div class="flex-grow overflow-y-auto scrollbar-hide px-2 py-2">
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
            class="w-full flex items-center gap-2 py-1.5 px-2 rounded-md text-xs transition-colors cursor-pointer group
              {isSelected ? 'bg-primary/10 text-primary font-medium' : 'text-foreground hover:bg-muted/60'}"
            style="padding-inline-start: {depth * 12 + 6}px;"
            aria-expanded={isContainer ? !isCollapsed : undefined}
            onclick={(event) => {
              if ((event.target as HTMLElement).closest(".collapse-toggle")) {
                toggleCollapse(node.id);
              } else {
                store.select(node.id);
              }
            }}
            onkeydown={(event) => {
              if (
                isContainer &&
                ((event.key === "ArrowRight" && isCollapsed) ||
                  (event.key === "ArrowLeft" && !isCollapsed))
              ) {
                event.preventDefault();
                toggleCollapse(node.id);
              }
            }}
          >
            <!-- Collapse toggle -->
            {#if isContainer && (node as any).children?.length > 0}
              <span class="collapse-toggle shrink-0 cursor-pointer">
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
            <IconComp size={14} class="shrink-0 {iconColor}" />

            <!-- Name -->
            <span class="truncate flex-grow">
              {node.name || node.id}
            </span>

            <!-- Visibility toggle (on hover) -->
            <span class="opacity-0 group-hover:opacity-100 transition-opacity shrink-0 p-0.5">
              {#if node.visible === false}
                <EyeSlash size={12} class="text-muted-foreground/60" />
              {:else}
                <Eye size={12} class="text-muted-foreground/50" />
              {/if}
            </span>

            <!-- Lock indicator -->
            {#if node.locked}
              <Lock size={11} class="text-muted-foreground/50 shrink-0" />
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
        <div class="text-center py-8 text-xs text-muted-foreground">
          {m.template_msg_empty_page()}
        </div>
      {/if}
    {/if}
  </div>
</div>
