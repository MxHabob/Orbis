<script lang="ts">
import * as m from "$lib/paraglide/messages";
/**
 * AssetPanel — Image/logo/stamp library.
 * Browse, upload, and drag assets onto the canvas.
 */
import type { EditorStore } from "../stores/editor-store.svelte";
import { Input } from "@orbis/ui-svelte/input";
import { Button } from "@orbis/ui-svelte/button";
import { Badge } from "@orbis/ui-svelte/badge";
import {
  Tooltip,
  TooltipTrigger,
  TooltipContent,
  TooltipProvider,
} from "@orbis/ui-svelte/tooltip";
import {
  ImageIcon,
  Search,
  Plus,
  Trash2,
  Upload,
  Loader2,
} from "@lucide/svelte";
import {
  useDocuments,
  useUploadDocument,
  useDeleteDocument,
  resolveDocumentUrl,
} from "$lib/features/documents/queries";

let {
  store,
}: {
  store: EditorStore;
} = $props();

let searchQuery = $state("");
let fileInput: HTMLInputElement;

// --- Queries & Mutations ---
const documentsQuery = useDocuments({ entityType: "template" });
const uploadMutation = useUploadDocument();
const deleteMutation = useDeleteDocument();

// Flatten pages into a single array of documents
let serverAssets = $derived(
  documentsQuery.data?.pages.flatMap((p) => p.items) || [],
);

// Filter by search query
let filteredAssets = $derived(
  serverAssets.filter((doc) => {
    if (!searchQuery) return true;
    const q = searchQuery.toLowerCase();
    return (
      doc.name?.toLowerCase().includes(q) ||
      doc.category?.toLowerCase().includes(q)
    );
  }),
);

// ── Asset Drag ───────────────────────────────────────────────

async function handleAssetDragStart(e: DragEvent, doc: any) {
  e.dataTransfer?.setData("application/x-docforge-asset", doc.id);
  const srcUrl = await resolveDocumentUrl(doc.url);
  if (srcUrl) {
    e.dataTransfer?.setData("text/uri-list", srcUrl);
  }
  if (e.dataTransfer) e.dataTransfer.effectAllowed = "copy";
}

// ── Insert asset as Image node ───────────────────────────────

async function insertAssetAsImage(doc: any) {
  const srcUrl = await resolveDocumentUrl(doc.url);
  store.addAsset(doc.id, "image");
  store.addElement("Image", {
    assetRef: doc.id,
    src: srcUrl || "",
    width: 40,
    height: 30,
    fit: "contain",
  });
}

// ── Remove asset ─────────────────────────────────────────────

function removeAsset(doc: any) {
  if (confirm(m.template_confirm_delete_asset?.() || "Are you sure?")) {
    deleteMutation.mutate(doc.id);
  }
}

// ── Upload ───────────────────────────────────────

function handleFileSelect(e: Event) {
  const target = e.target as HTMLInputElement;
  if (!target.files?.length) return;
  const file = target.files[0];

  uploadMutation.mutate(
    {
      file,
      entity_type: "template",
      entity_id: store.document.meta.id || "shared",
      category: "image",
      visibility: "private",
    },
    {
      onSuccess: (uploaded: any) => {
        if (uploaded?.id) {
          store.addAsset(uploaded.id, "image");
        }
        if (fileInput) fileInput.value = "";
      },
    },
  );
}

function triggerUpload() {
  fileInput?.click();
}
</script>

<TooltipProvider delayDuration={300}>
<div class="flex flex-col h-full text-start">
  <!-- Header -->
  <div class="px-3 py-2 border-b border-border/40 flex items-center justify-between">
    <div class="flex items-center gap-2">
      <ImageIcon size={13} class="text-primary" />
      <h3 class="text-xs font-bold text-foreground">{m.template_title_assets()}</h3>
    </div>
    <div class="flex items-center gap-1">
      <Badge variant="outline" class="text-[8px]">{filteredAssets.length}</Badge>
      <Tooltip>
        <TooltipTrigger>
          <Button variant="ghost" size="icon-xs" class="cursor-pointer h-5 w-5" onclick={triggerUpload} disabled={uploadMutation.isPending}>
            {#if uploadMutation.isPending}
              <Loader2 size={11} class="animate-spin" />
            {:else}
              <Upload size={11} />
            {/if}
          </Button>
        </TooltipTrigger>
        <TooltipContent>{m.template_btn_add_asset()}</TooltipContent>
      </Tooltip>
      <input 
        type="file" 
        bind:this={fileInput} 
        class="hidden" 
        accept="image/*" 
        onchange={handleFileSelect} 
      />
    </div>
  </div>

  <!-- Search -->
  <div class="px-2 py-1.5 border-b border-border/30">
    <div class="relative">
      <Search size={11} class="absolute inset-s-2 top-1/2 -translate-y-1/2 text-muted-foreground" />
      <Input
        type="text"
        placeholder={m.template_search_placeholder()}
        bind:value={searchQuery}
        class="h-6 text-[10px] ps-7"
      />
    </div>
  </div>

  <!-- Asset Grid -->
  <div class="grow overflow-y-auto scrollbar-hide p-2 flex flex-col gap-2">
    {#if documentsQuery.isLoading && serverAssets.length === 0}
      <div class="flex justify-center p-4">
        <Loader2 class="h-4 w-4 animate-spin text-muted-foreground" />
      </div>
    {:else if filteredAssets.length === 0}
      <div class="flex flex-col items-center justify-center py-8 px-4 gap-2">
        <ImageIcon size={20} class="text-muted-foreground/30" />
        <p class="text-[10px] text-muted-foreground text-center">
          {searchQuery ? m.template_msg_no_results() : m.template_msg_no_assets()}
        </p>
      </div>
    {:else}
      <div class="grid grid-cols-2 gap-1.5">
        {#each filteredAssets as doc (doc.id)}
          <!-- svelte-ignore a11y_no_static_element_interactions -->
          <div
            class="relative group rounded-lg border border-border/40 bg-card/30 hover:border-primary/40 cursor-grab transition-all overflow-hidden flex flex-col"
            draggable="true"
            ondragstart={(e) => handleAssetDragStart(e, doc)}
          >
            <!-- Thumbnail -->
            <div class="aspect-4/3 bg-muted/20 flex items-center justify-center overflow-hidden">
              {#await resolveDocumentUrl(doc.url)}
                <Loader2 class="h-4 w-4 animate-spin text-muted-foreground" />
              {:then srcUrl}
                <img
                  src={srcUrl}
                  alt={doc.name}
                  class="w-full h-full object-cover"
                  loading="lazy"
                />
              {:catch}
                <ImageIcon size={20} class="text-muted-foreground/30" />
              {/await}
            </div>

            <!-- Info -->
            <div class="px-1.5 py-1 flex items-center justify-between bg-card">
              <div class="flex flex-col min-w-0">
                <span class="text-[8px] font-medium text-foreground truncate">{doc.name}</span>
                <span class="text-[7px] text-muted-foreground capitalize">{doc.extension?.toUpperCase() || 'IMG'}</span>
              </div>
            </div>

            <!-- Hover Actions -->
            <div class="absolute top-1 inset-e-1 opacity-0 group-hover:opacity-100 transition-opacity flex gap-0.5">
              <button type="button" class="p-0.5 rounded bg-background/80 hover:bg-accent cursor-pointer" onclick={() => insertAssetAsImage(doc)}>
                <Plus size={10} class="text-primary" />
              </button>
              <button type="button" class="p-0.5 rounded bg-background/80 hover:bg-accent cursor-pointer" onclick={() => removeAsset(doc)}>
                <Trash2 size={10} class="text-destructive" />
              </button>
            </div>
          </div>
        {/each}
      </div>
      
      {#if documentsQuery.hasNextPage}
        <Button 
          variant="ghost" 
          size="xs" 
          class="w-full text-[10px] mt-2 cursor-pointer"
          disabled={documentsQuery.isFetchingNextPage}
          onclick={() => documentsQuery.fetchNextPage()}
        >
          {#if documentsQuery.isFetchingNextPage}
            <Loader2 class="me-1 h-3 w-3 animate-spin" /> {m.template_loading?.() || "Loading..."}
          {:else}
            {m.template_load_more?.() || "Load More"}
          {/if}
        </Button>
      {/if}
    {/if}
  </div>

  <!-- Footer hint -->
  <div class="px-3 py-1.5 border-t border-border/30 bg-muted/10">
    <p class="text-[8px] text-muted-foreground text-center">
      {m.template_msg_drag_asset()}
    </p>
  </div>
</div>
</TooltipProvider>
