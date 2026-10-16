<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { confirmStore } from "$lib/stores";
import { LoadingIndicator, toast } from "@orbis/ui";
/**
 * AssetPanel — Image/logo/stamp library.
 * Browse, upload, and drag assets onto the canvas.
 */
import {
    getDocumentSource,
    resolveDocumentUrl,
    useDeleteDocument,
    useDocuments,
    useUploadFile,
} from "$lib/features/documents/queries";
import { ArrowUpTray, Photo, Plus, Search, Trash } from "@orbis/icons";
import { Badge } from "@orbis/ui/badge";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import {
    Tooltip,
    TooltipContent,
    TooltipProvider,
    TooltipTrigger,
} from "@orbis/ui/tooltip";
import type { EditorStore } from "../stores/editor-store.svelte";

let {
  store,
}: {
  store: EditorStore;
} = $props();

let searchQuery = $state("");
let fileInput: HTMLInputElement;

// --- Queries & Mutations ---
const documentsQuery = useDocuments(() => ({ entityType: "template" }));
const uploadMutation = useUploadFile();
const deleteMutation = useDeleteDocument();

// Flatten pages into a single array of documents
let serverAssets = $derived(
  documentsQuery.data?.pages.flatMap((p) => p.items) || [],
);

// AdjustmentsHorizontal by search query
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
  const srcUrl = await resolveDocumentUrl(getDocumentSource(doc), doc.id);
  if (srcUrl) {
    e.dataTransfer?.setData("text/uri-list", srcUrl);
  }
  if (e.dataTransfer) e.dataTransfer.effectAllowed = "copy";
}

// ── Insert asset as Image node ───────────────────────────────

async function insertAssetAsImage(doc: any) {
  const srcUrl = await resolveDocumentUrl(getDocumentSource(doc), doc.id);
  store.addAsset(doc.id, "image");
  store.addElement("Image", {
    assetRef: doc.id,
    src: srcUrl || "",
    width: 40,
    height: 30,
    fit: "contain",
  });
}

async function setAssetAsBackground(doc: any) {
  const srcUrl = await resolveDocumentUrl(getDocumentSource(doc), doc.id);
  store.setPageBackground(doc.id, srcUrl);
}

// ── Remove asset ─────────────────────────────────────────────

async function removeAsset(doc: any) {
  const confirmed = await confirmStore.prompt({
    title: m.template_confirm_delete_asset(),
    description: m.template_confirm_delete_asset(),
    confirmText: m.common_action_delete(),
    variant: "destructive",
  });
  if (confirmed) {
    deleteMutation.mutate(doc.id);
  }
}

// ── ArrowUpTray ───────────────────────────────────────

function handleFileSelect(e: Event) {
  const target = e.target as HTMLInputElement;
  if (!target.files?.length) return;
  const file = target.files[0];
  const uploadToast = toast.loading(m.template_asset_uploading(), { duration: 0 });

  uploadMutation.mutate(
    {
      file,
      name: file.name,
      entity_type: "template",
      entity_id: store.document.meta.id || "shared",
      category: "image",
      visibility: "private",
    },
    {
      onSuccess: (uploaded) => {
        if (!uploaded?.id) {
          uploadToast.update({
            title: m.template_asset_upload_missing_id(),
            variant: "error",
            duration: 5000,
          });
          return;
        }

        store.addAsset(uploaded.id, "image");
        uploadToast.update({
          title: m.template_asset_upload_success(),
          variant: "success",
          duration: 4000,
        });
      },
      onError: (error) => {
        uploadToast.update({
          title: error instanceof Error ? error.message : m.template_asset_upload_failed(),
          variant: "error",
          duration: 5000,
        });
      },
      onSettled: () => {
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
  <div class="px-3 py-2.5 border-b border-border flex items-center justify-between">
    <div class="flex items-center gap-2">
      <Photo size={15} class="text-muted-foreground" />
      <h3 class="text-sm font-semibold text-foreground">{m.template_title_assets()}</h3>
    </div>
    <div class="flex items-center gap-1">
      <Badge variant="outline" class="text-[10px]">{filteredAssets.length}</Badge>
      {#if store.activePage?.children.some((child) => child.type === "Image" && child.props.layer === "background")}
        <Button
          variant="ghost"
          size="icon-xs"
          class="cursor-pointer"
          title={m.template_remove_background()}
          aria-label={m.template_remove_background()}
          onclick={() => store.clearPageBackground()}
        >
          <Trash size={14} />
        </Button>
      {/if}
      <Tooltip>
        <TooltipTrigger>
          <Button variant="ghost" size="icon-sm" class="cursor-pointer" onclick={triggerUpload} disabled={uploadMutation.isPending}>
            {#if uploadMutation.isPending}
              <LoadingIndicator   size={14} />
            {:else}
              <ArrowUpTray size={14} />
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
  <div class="px-3 py-2 border-b border-border">
    <div class="relative">
      <Search size={14} class="absolute inset-s-2.5 top-1/2 -translate-y-1/2 text-muted-foreground" />
      <Input
        type="text"
        placeholder={m.template_search_placeholder()}
        bind:value={searchQuery}
        class="h-9 text-xs ps-8"
      />
    </div>
  </div>

  <!-- Asset Grid -->
  <div class="grow overflow-y-auto scrollbar-hide p-3 flex flex-col gap-3">
    {#if documentsQuery.isLoading && serverAssets.length === 0}
      <div class="flex justify-center p-4">
        <LoadingIndicator class="h-4 w-4 text-muted-foreground"  size={16} />
      </div>
    {:else if filteredAssets.length === 0}
      <div class="flex flex-col items-center justify-center py-8 px-4 gap-2">
        <Photo size={20} class="text-muted-foreground/30" />
        <p class="text-xs text-muted-foreground text-center">
          {searchQuery ? m.template_msg_no_results() : m.template_msg_no_assets()}
        </p>
      </div>
    {:else}
      <div class="grid grid-cols-2 gap-2">
        {#each filteredAssets as doc (doc.id)}
          <!-- svelte-ignore a11y_no_static_element_interactions -->
          <div
            class="relative group rounded-md border border-border bg-card hover:bg-muted/30 cursor-grab transition-colors overflow-hidden flex flex-col"
            draggable="true"
            ondragstart={(e) => handleAssetDragStart(e, doc)}
          >
            <!-- Thumbnail -->
            <div class="aspect-4/3 bg-muted/40 flex items-center justify-center overflow-hidden">
              {#await resolveDocumentUrl(getDocumentSource(doc), doc.id)}
                <LoadingIndicator class="h-4 w-4 text-muted-foreground"  size={16} />
              {:then srcUrl}
                <img
                  src={srcUrl}
                  alt={doc.name}
                  class="w-full h-full object-cover"
                  loading="lazy"
                />
              {:catch}
                <Photo size={20} class="text-muted-foreground/30" />
              {/await}
            </div>

            <!-- Info -->
            <div class="px-2 py-1.5 flex items-center justify-between bg-card">
              <div class="flex flex-col min-w-0">
                <span class="text-[11px] font-medium text-foreground truncate">{doc.name}</span>
                <span class="text-[10px] text-muted-foreground capitalize">{doc.extension?.toUpperCase() || 'IMG'}</span>
              </div>
            </div>

            <!-- Hover Actions -->
            <div class="absolute top-1 inset-e-1 opacity-0 group-hover:opacity-100 transition-opacity flex gap-0.5">
              <button
                type="button"
                class="rounded bg-background/90 p-1.5 hover:bg-accent cursor-pointer"
                title={m.template_set_as_background()}
                aria-label={m.template_set_as_background()}
                onclick={() => setAssetAsBackground(doc)}
              >
                <Photo size={13} class="text-primary" />
              </button>
              <button type="button" class="rounded bg-background/90 p-1.5 hover:bg-accent cursor-pointer" onclick={() => insertAssetAsImage(doc)}>
                <Plus size={13} class="text-primary" />
              </button>
              <button type="button" class="rounded bg-background/90 p-1.5 hover:bg-accent cursor-pointer" onclick={() => removeAsset(doc)}>
                <Trash size={13} class="text-destructive" />
              </button>
            </div>
          </div>
        {/each}
      </div>
      
      {#if documentsQuery.hasNextPage}
        <Button 
          variant="ghost" 
          size="xs" 
          class="w-full text-xs mt-2 cursor-pointer"
          disabled={documentsQuery.isFetchingNextPage}
          onclick={() => documentsQuery.fetchNextPage()}
        >
          {#if documentsQuery.isFetchingNextPage}
            <LoadingIndicator class="me-1 h-3 w-3"  size={12} /> {m.template_loading()}
          {:else}
            {m.template_load_more()}
          {/if}
        </Button>
      {/if}
    {/if}
  </div>

  <!-- Footer hint -->
  <div class="px-3 py-2 border-t border-border">
    <p class="text-[10px] text-muted-foreground text-center">
      {m.template_msg_drag_asset()}
    </p>
  </div>
</div>
</TooltipProvider>
