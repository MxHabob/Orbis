<script lang="ts">
/**
 * PagesPanel — Page management with thumbnails, add/remove/reorder.
 * Shows mini page previews and allows page navigation.
 */
import type { EditorStore } from "../stores/editor-store.svelte";
import type { PageNode } from "@docforge/core";
import { Button } from "@orbis/ui-svelte/button";
import { Badge } from "@orbis/ui-svelte/badge";
import {
  Tooltip,
  TooltipTrigger,
  TooltipContent,
  TooltipProvider,
} from "@orbis/ui-svelte/tooltip";
import {
  DropdownMenu,
  DropdownMenuTrigger,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
} from "@orbis/ui-svelte/dropdown-menu";
import {
  Plus,
  Trash2,
  Copy,
  MoreVertical,
  FileText,
  ChevronUp,
  ChevronDown,
  ArrowUp,
  ArrowDown,
} from "lucide-svelte";

let {
  store,
}: {
  store: EditorStore;
} = $props();

// ── Drag Reorder State ───────────────────────────────────────

let dragOverIndex = $state<number | null>(null);
let dragSourceIndex = $state<number | null>(null);

function handlePageDragStart(e: DragEvent, index: number) {
  dragSourceIndex = index;
  e.dataTransfer?.setData("text/plain", String(index));
  if (e.dataTransfer) e.dataTransfer.effectAllowed = "move";
}

function handlePageDragOver(e: DragEvent, index: number) {
  e.preventDefault();
  if (e.dataTransfer) e.dataTransfer.dropEffect = "move";
  dragOverIndex = index;
}

function handlePageDragLeave() {
  dragOverIndex = null;
}

function handlePageDrop(e: DragEvent, toIndex: number) {
  e.preventDefault();
  if (dragSourceIndex !== null && dragSourceIndex !== toIndex) {
    store.executeCommand({
      type: "reorder-page",
      description: `Move page ${dragSourceIndex + 1} to position ${toIndex + 1}`,
      descriptionAr: `نقل صفحة ${dragSourceIndex + 1} إلى الموضع ${toIndex + 1}`,
      execute: (doc) => {
        const pages = [...doc.pages];
        const [moved] = pages.splice(dragSourceIndex!, 1);
        pages.splice(toIndex, 0, moved!);
        return { ...doc, pages };
      },
      undo: (doc) => {
        const pages = [...doc.pages];
        const [moved] = pages.splice(toIndex, 1);
        pages.splice(dragSourceIndex!, 0, moved!);
        return { ...doc, pages };
      },
    });
    store.setActivePage(toIndex);
  }
  dragOverIndex = null;
  dragSourceIndex = null;
}

function handlePageDragEnd() {
  dragOverIndex = null;
  dragSourceIndex = null;
}

// ── Actions ──────────────────────────────────────────────────

function duplicatePage(index: number) {
  const page = store.pages[index];
  if (!page) return;
  const clonedPage: PageNode = JSON.parse(JSON.stringify(page));
  // Generate new IDs for the cloned page and all children
  clonedPage.id = `page-${Date.now()}`;
  reassignIds(clonedPage);

  store.executeCommand({
    type: "duplicate-page",
    description: `Duplicate page ${index + 1}`,
    descriptionAr: `تكرار صفحة ${index + 1}`,
    execute: (doc) => {
      const pages = [...doc.pages];
      pages.splice(index + 1, 0, clonedPage);
      return { ...doc, pages };
    },
    undo: (doc) => ({
      ...doc,
      pages: doc.pages.filter((p) => p.id !== clonedPage.id),
    }),
  });
  store.setActivePage(index + 1);
}

function movePage(fromIndex: number, direction: "up" | "down") {
  const toIndex = direction === "up" ? fromIndex - 1 : fromIndex + 1;
  if (toIndex < 0 || toIndex >= store.pages.length) return;

  store.executeCommand({
    type: "reorder-page",
    description: `Move page ${direction}`,
    descriptionAr: `نقل صفحة ${direction === "up" ? "للأعلى" : "للأسفل"}`,
    execute: (doc) => {
      const pages = [...doc.pages];
      const [moved] = pages.splice(fromIndex, 1);
      pages.splice(toIndex, 0, moved!);
      return { ...doc, pages };
    },
    undo: (doc) => {
      const pages = [...doc.pages];
      const [moved] = pages.splice(toIndex, 1);
      pages.splice(fromIndex, 0, moved!);
      return { ...doc, pages };
    },
  });
  store.setActivePage(toIndex);
}

function reassignIds(node: any) {
  if (node.children && Array.isArray(node.children)) {
    for (const child of node.children) {
      child.id = `${child.type.toLowerCase()}-${Date.now()}-${Math.random().toString(36).slice(2, 7)}`;
      reassignIds(child);
    }
  }
}
</script>

<TooltipProvider delayDuration={300}>
<div class="flex flex-col h-full text-start">
  <!-- Header -->
  <div class="px-3 py-2 border-b border-border/40 flex items-center justify-between">
    <div class="flex items-center gap-2">
      <FileText size={13} class="text-primary" />
      <h3 class="text-xs font-bold text-foreground">الصفحات</h3>
    </div>
    <div class="flex items-center gap-1">
      <Badge variant="outline" class="text-[8px]">{store.pageCount}</Badge>
      <Tooltip>
        <TooltipTrigger>
          <Button variant="ghost" size="icon-xs" class="cursor-pointer h-5 w-5" onclick={() => store.addPage()}>
            <Plus size={11} />
          </Button>
        </TooltipTrigger>
        <TooltipContent>إضافة صفحة</TooltipContent>
      </Tooltip>
    </div>
  </div>

  <!-- Page Thumbnails -->
  <div class="flex-grow overflow-y-auto scrollbar-hide p-2 flex flex-col gap-2">
    {#each store.pages as page, i (page.id)}
      {@const isActive = store.activePageIndex === i}
      {@const isDragOver = dragOverIndex === i}
      {@const childCount = page.children?.length ?? 0}

      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div
        class="relative group rounded-lg border-2 transition-all cursor-pointer select-none
          {isActive ? 'border-primary shadow-sm bg-primary/5' : 'border-border/40 hover:border-border bg-card/30'}
          {isDragOver ? 'border-primary/50 bg-primary/10' : ''}"
        draggable="true"
        ondragstart={(e) => handlePageDragStart(e, i)}
        ondragover={(e) => handlePageDragOver(e, i)}
        ondragleave={handlePageDragLeave}
        ondrop={(e) => handlePageDrop(e, i)}
        ondragend={handlePageDragEnd}
        onclick={() => store.setActivePage(i)}
      >
        <!-- Page thumbnail -->
        <div class="p-2 flex items-start gap-2">
          <!-- Mini page preview -->
          <div class="w-[50px] h-[68px] bg-white rounded border border-border/60 shrink-0 flex items-center justify-center shadow-sm">
            {#if childCount > 0}
              <div class="w-[38px] flex flex-col gap-0.5 p-1">
                {#each page.children.slice(0, 5) as child}
                  <div class="h-1 rounded-sm {
                    child.type === 'Text' ? 'bg-emerald-300' :
                    child.type === 'Image' ? 'bg-violet-300' :
                    child.type === 'Shape' ? 'bg-blue-300' :
                    child.type === 'Table' ? 'bg-cyan-300' :
                    'bg-gray-200'
                  }" style="width: {40 + Math.random() * 60}%;"></div>
                {/each}
                {#if childCount > 5}
                  <div class="text-[5px] text-muted-foreground text-center">+{childCount - 5}</div>
                {/if}
              </div>
            {:else}
              <span class="text-[7px] text-muted-foreground/50">فارغة</span>
            {/if}
          </div>

          <!-- Page info -->
          <div class="flex flex-col flex-grow min-w-0 py-0.5">
            <div class="flex items-center gap-1">
              <span class="text-[10px] font-semibold {isActive ? 'text-primary' : 'text-foreground'}">
                صفحة {i + 1}
              </span>
              {#if isActive}
                <Badge variant="success" class="text-[7px] h-3 px-1">نشطة</Badge>
              {/if}
            </div>
            <span class="text-[8px] text-muted-foreground">
              {childCount} {childCount === 1 ? "عنصر" : "عناصر"}
            </span>
            <span class="text-[7px] text-muted-foreground/60 font-mono truncate mt-0.5">
              {page.id}
            </span>
          </div>

          <!-- Actions menu -->
          <div class="opacity-0 group-hover:opacity-100 transition-opacity shrink-0">
            <DropdownMenu>
              <DropdownMenuTrigger>
                <Button variant="ghost" size="icon-xs" class="cursor-pointer h-5 w-5">
                  <MoreVertical size={11} />
                </Button>
              </DropdownMenuTrigger>
              <DropdownMenuContent align="end" class="min-w-[140px]">
                <DropdownMenuItem class="text-[10px] cursor-pointer" onclick={() => duplicatePage(i)}>
                  <Copy size={11} class="me-2" />
                  تكرار الصفحة
                </DropdownMenuItem>
                {#if i > 0}
                  <DropdownMenuItem class="text-[10px] cursor-pointer" onclick={() => movePage(i, "up")}>
                    <ArrowUp size={11} class="me-2" />
                    نقل للأعلى
                  </DropdownMenuItem>
                {/if}
                {#if i < store.pages.length - 1}
                  <DropdownMenuItem class="text-[10px] cursor-pointer" onclick={() => movePage(i, "down")}>
                    <ArrowDown size={11} class="me-2" />
                    نقل للأسفل
                  </DropdownMenuItem>
                {/if}
                {#if store.pages.length > 1}
                  <DropdownMenuSeparator />
                  <DropdownMenuItem class="text-[10px] text-destructive cursor-pointer" onclick={() => store.removePage(i)}>
                    <Trash2 size={11} class="me-2" />
                    حذف الصفحة
                  </DropdownMenuItem>
                {/if}
              </DropdownMenuContent>
            </DropdownMenu>
          </div>
        </div>
      </div>
    {/each}
  </div>

  <!-- Add Page Button -->
  <div class="px-3 py-2 border-t border-border/30">
    <Button variant="outline" size="xs" class="w-full cursor-pointer text-[10px]" onclick={() => store.addPage()}>
      <Plus size={11} class="me-1" />
      إضافة صفحة جديدة
    </Button>
  </div>
</div>
</TooltipProvider>
