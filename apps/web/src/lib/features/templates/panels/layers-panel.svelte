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
import { Input } from "@orbis/ui/input";
import {
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

let search = $state("");
const filter = $derived(search.trim().toLocaleLowerCase());

function matches(node: TemplateNode): boolean {
  if (!filter) return true;
  if ([node.name, node.id, node.type].some((value) => value?.toLocaleLowerCase().includes(filter))) {
    return true;
  }
  return isContainerNode(node) && node.children.some(matches);
}
</script>

<div class="flex flex-col h-full text-start">
  <!-- Header -->
  <div class="px-3 py-2.5 border-b border-border flex items-center gap-2">
    <Input type="search" class="h-8" aria-label={m.common_header_search()} placeholder={m.common_header_search()} bind:value={search} />
    <div class="flex items-center gap-1">
      <span class="text-[11px] text-muted-foreground">
        {store.activePageIndex + 1}/{store.pageCount}
      </span>
      <Button variant="ghost" size="icon-sm" aria-label={m.shared_ui_add_page()} onclick={() => store.addPage()}>
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
        {@const isCollapsed = !filter && collapsed.has(node.id)}
        {@const isSelected = store.selectedIds.includes(node.id)}
        {@const IconComp = getIcon(node.type)}

        <div class="flex flex-col">
          <div class="group flex min-w-0 items-center">
          <button
            type="button"
            class="min-w-0 flex-1 flex items-center gap-2 py-1.5 px-2 rounded-md text-xs transition-colors cursor-pointer
              {isSelected ? 'bg-primary/10 text-primary font-medium' : 'text-foreground hover:bg-muted/60'}"
            style="padding-inline-start: {depth * 12 + 6}px;"
            aria-expanded={isContainer ? !isCollapsed : undefined}
            aria-pressed={isSelected}
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
                event.stopPropagation();
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
            <IconComp size={14} class="shrink-0 text-muted-foreground" />

            <!-- Name -->
            <span class="truncate flex-grow">
              {node.name || node.id}
            </span>

            {#if node.locked}
              <Lock size={11} class="text-muted-foreground shrink-0" />
            {/if}
          </button>
          <Button
            variant="ghost"
            size="icon-xs"
            class="shrink-0"
            aria-label={node.visible === false ? m.template_action_show() : m.template_action_hide()}
            onclick={() => store.updateNodeProps(node.id, { visible: node.visible === false })}
          >
            {#if node.visible === false}
              <EyeSlash size={13} />
            {:else}
              <Eye size={13} />
            {/if}
          </Button>
          </div>

          <!-- Children (if expanded) -->
          {#if isContainer && !isCollapsed}
            {#each node.children.filter(matches) as child (child.id)}
              {@render treeNode(child, depth + 1)}
            {/each}
          {/if}
        </div>
      {/snippet}

      {#each store.activePage.children.filter(matches) as child (child.id)}
        {@render treeNode(child, 0)}
      {/each}

      {#if store.activePage.children.length === 0}
        <div class="text-center py-8 text-xs text-muted-foreground">
          {m.template_msg_empty_page()}
        </div>
      {:else if !store.activePage.children.some(matches)}
        <p class="py-8 text-center text-xs text-muted-foreground">{m.template_msg_no_results()}</p>
      {/if}
    {/if}
  </div>
</div>
