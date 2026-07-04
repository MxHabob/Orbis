<script lang="ts">
/**
 * CanvasPage — Renders a single page's child nodes recursively.
 */
import type { PageNode, TemplateNode } from "@docforge/core";
import type { EditorStore } from "../stores/editor-store.svelte";
import CanvasNode from "./CanvasNode.svelte";

let {
  page,
  store,
  pageSizePx,
}: {
  page: PageNode;
  store: EditorStore;
  pageSizePx: { width: number; height: number };
} = $props();
</script>

<g class="canvas-page" data-page-id={page.id}>
  {#each page.children as child (child.id)}
    <CanvasNode node={child} {store} {pageSizePx} />
  {/each}
</g>
