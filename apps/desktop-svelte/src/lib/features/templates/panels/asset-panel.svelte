<script lang="ts">
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
  GripVertical,
  Upload,
  FolderOpen,
  Stamp,
  Star,
} from "lucide-svelte";

let {
  store,
}: {
  store: EditorStore;
} = $props();

let searchQuery = $state("");
let showUpload = $state(false);

// ── Derived ──────────────────────────────────────────────────

const assets = $derived(store.document.assets);
const assetEntries = $derived(
  Object.entries(assets)
    .filter(([key, asset]) => {
      if (!searchQuery) return true;
      const q = searchQuery.toLowerCase();
      return (
        key.toLowerCase().includes(q) ||
        (asset.name?.toLowerCase().includes(q) ?? false) ||
        (asset.type?.toLowerCase().includes(q) ?? false)
      );
    })
    .sort((a, b) => a[0].localeCompare(b[0])),
);

// ── Icons by type ────────────────────────────────────────────

function getAssetIcon(type: string) {
  switch (type) {
    case "logo": return Star;
    case "stamp": return Stamp;
    case "background": return FolderOpen;
    default: return ImageIcon;
  }
}

// ── Asset Drag ───────────────────────────────────────────────

function handleAssetDragStart(e: DragEvent, assetRef: string) {
  e.dataTransfer?.setData("application/x-docforge-asset", assetRef);
  const asset = assets[assetRef];
  if (asset?.url) {
    e.dataTransfer?.setData("text/uri-list", asset.url);
  }
  if (e.dataTransfer) e.dataTransfer.effectAllowed = "copy";
}

// ── Insert asset as Image node ───────────────────────────────

function insertAssetAsImage(assetRef: string) {
  const asset = assets[assetRef];
  if (!asset) return;
  store.addElement("Image", {
    assetRef,
    src: asset.url || "",
    width: 40,
    height: 30,
    fit: "contain",
  });
}

// ── Remove asset ─────────────────────────────────────────────

function removeAsset(assetRef: string) {
  const prevAssets = { ...store.document.assets };
  const { [assetRef]: _, ...remaining } = prevAssets;
  store.executeCommand({
    type: "remove-asset",
    description: `Remove asset ${assetRef}`,
    descriptionAr: `حذف الأصل ${assetRef}`,
    execute: (doc) => ({ ...doc, assets: remaining }),
    undo: (doc) => ({ ...doc, assets: prevAssets }),
  });
}

// ── Upload placeholder ───────────────────────────────────────

let newAssetName = $state("");
let newAssetUrl = $state("");
let newAssetType = $state("image");

function addAssetManually() {
  if (!newAssetName.trim() || !newAssetUrl.trim()) return;
  const key = newAssetName.trim().replace(/\s+/g, "_").toLowerCase();
  const updatedAssets = {
    ...store.document.assets,
    [key]: {
      name: newAssetName.trim(),
      url: newAssetUrl.trim(),
      type: newAssetType,
      mimeType: "image/png",
    },
  };
  store.executeCommand({
    type: "add-asset",
    description: `Add asset ${key}`,
    descriptionAr: `إضافة أصل ${key}`,
    execute: (doc) => ({ ...doc, assets: updatedAssets }),
    undo: (doc) => ({ ...doc, assets: store.document.assets }),
  });
  newAssetName = "";
  newAssetUrl = "";
  newAssetType = "image";
  showUpload = false;
}
</script>

<TooltipProvider delayDuration={300}>
<div class="flex flex-col h-full text-start">
  <!-- Header -->
  <div class="px-3 py-2 border-b border-border/40 flex items-center justify-between">
    <div class="flex items-center gap-2">
      <ImageIcon size={13} class="text-primary" />
      <h3 class="text-xs font-bold text-foreground">الأصول</h3>
    </div>
    <div class="flex items-center gap-1">
      <Badge variant="outline" class="text-[8px]">{assetEntries.length}</Badge>
      <Tooltip>
        <TooltipTrigger>
          <Button variant="ghost" size="icon-xs" class="cursor-pointer h-5 w-5" onclick={() => showUpload = !showUpload}>
            <Upload size={11} />
          </Button>
        </TooltipTrigger>
        <TooltipContent>إضافة أصل</TooltipContent>
      </Tooltip>
    </div>
  </div>

  <!-- Search -->
  <div class="px-2 py-1.5 border-b border-border/30">
    <div class="relative">
      <Search size={11} class="absolute start-2 top-1/2 -translate-y-1/2 text-muted-foreground" />
      <Input
        type="text"
        placeholder="بحث..."
        bind:value={searchQuery}
        class="h-6 text-[10px] ps-7"
      />
    </div>
  </div>

  <!-- Add Asset Form -->
  {#if showUpload}
    <div class="px-3 py-2 border-b border-border/30 bg-muted/20 flex flex-col gap-1.5">
      <Input
        type="text"
        placeholder="اسم الأصل"
        bind:value={newAssetName}
        class="h-6 text-[10px]"
      />
      <Input
        type="text"
        placeholder="رابط الصورة"
        bind:value={newAssetUrl}
        class="h-6 text-[10px] font-mono"
      />
      <div class="flex items-center gap-1.5">
        <select
          bind:value={newAssetType}
          class="h-6 text-[10px] rounded border border-border bg-background px-1.5 flex-grow"
        >
          <option value="image">Image</option>
          <option value="logo">Logo</option>
          <option value="stamp">Stamp</option>
          <option value="signature">Signature</option>
          <option value="background">Background</option>
        </select>
        <Button variant="lime" size="xs" class="cursor-pointer h-6 text-[10px]" onclick={addAssetManually}>
          إضافة
        </Button>
      </div>
    </div>
  {/if}

  <!-- Asset Grid -->
  <div class="flex-grow overflow-y-auto scrollbar-hide p-2">
    {#if assetEntries.length === 0}
      <div class="flex flex-col items-center justify-center py-8 px-4 gap-2">
        <ImageIcon size={20} class="text-muted-foreground/30" />
        <p class="text-[10px] text-muted-foreground text-center">
          {searchQuery ? "لا توجد نتائج" : "لا توجد أصول — أضف صوراً وشعارات"}
        </p>
      </div>
    {:else}
      <div class="grid grid-cols-2 gap-1.5">
        {#each assetEntries as [key, asset] (key)}
          {@const AssetIcon = getAssetIcon(asset.type || "image")}
          <!-- svelte-ignore a11y_no_static_element_interactions -->
          <div
            class="relative group rounded-lg border border-border/40 bg-card/30 hover:border-primary/40 cursor-grab transition-all overflow-hidden"
            draggable="true"
            ondragstart={(e) => handleAssetDragStart(e, key)}
          >
            <!-- Thumbnail -->
            <div class="aspect-[4/3] bg-muted/20 flex items-center justify-center overflow-hidden">
              {#if asset.url}
                <img
                  src={asset.url}
                  alt={asset.name || key}
                  class="w-full h-full object-contain"
                  loading="lazy"
                />
              {:else}
                <svelte:component this={AssetIcon} size={20} class="text-muted-foreground/30" />
              {/if}
            </div>

            <!-- Info -->
            <div class="px-1.5 py-1 flex items-center justify-between">
              <div class="flex flex-col min-w-0">
                <span class="text-[8px] font-medium text-foreground truncate">{asset.name || key}</span>
                <span class="text-[7px] text-muted-foreground capitalize">{asset.type || "image"}</span>
              </div>
            </div>

            <!-- Hover Actions -->
            <div class="absolute top-1 end-1 opacity-0 group-hover:opacity-100 transition-opacity flex gap-0.5">
              <button type="button" class="p-0.5 rounded bg-background/80 hover:bg-accent cursor-pointer" onclick={() => insertAssetAsImage(key)}>
                <Plus size={10} class="text-primary" />
              </button>
              <button type="button" class="p-0.5 rounded bg-background/80 hover:bg-accent cursor-pointer" onclick={() => removeAsset(key)}>
                <Trash2 size={10} class="text-destructive" />
              </button>
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </div>

  <!-- Footer hint -->
  <div class="px-3 py-1.5 border-t border-border/30 bg-muted/10">
    <p class="text-[8px] text-muted-foreground text-center">
      اسحب الأصل إلى اللوحة لإنشاء عنصر صورة
    </p>
  </div>
</div>
</TooltipProvider>
