<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Sheet, SheetContent, SheetFooter } from "@orbis/ui/sheet";
import {
  Collapse,
  DocumentPage,
  Download,
  Expand,
  Trash,
  ArchiveBox,
  Sparkles,
  Refresh,
} from "@orbis/icons";
import type { DocumentDto } from "$lib/api";

let {
  open = $bindable(false),
  document,
  url,
  isTrash = false,
  ondelete,
  onarchive,
  onrestore,
  onpurge,
}: {
  open: boolean;
  document: DocumentDto | null;
  url: string;
  isTrash?: boolean;
  ondelete?: (id: string) => void;
  onarchive?: (id: string) => void;
  onrestore?: (id: string) => void;
  onpurge?: (id: string) => void;
} = $props();

let imageWidth = $state(0);
let imageHeight = $state(0);
let imageZoom = $state(1);
let imagePanX = $state(0);
let imagePanY = $state(0);
let isPanning = $state(false);
let panStartX = 0;
let panStartY = 0;
let panOriginX = 0;
let panOriginY = 0;

const mimeType = $derived(document?.mime_type?.toLowerCase() ?? "");
const isImage = $derived(mimeType.startsWith("image/"));
const isVideo = $derived(mimeType.startsWith("video/"));
const isAudio = $derived(mimeType.startsWith("audio/"));
const isPdf = $derived(mimeType === "application/pdf" || document?.extension?.toLowerCase() === "pdf");
const documentId = $derived(document?.id);

$effect(() => {
  documentId;
  imageZoom = 1;
  imagePanX = 0;
  imagePanY = 0;
  imageWidth = 0;
  imageHeight = 0;
});

function formatDate(value?: string | null) {
  if (!value) return "-";
  return new Intl.DateTimeFormat(undefined, {
    dateStyle: "medium",
    timeStyle: "short",
  }).format(new Date(value));
}

function formatSize(bytes?: number | null) {
  if (!bytes || bytes <= 0) return "0 KB";
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(2)} MB`;
}

function removeDocument() {
  if (!document) return;
  open = false;
  if (onpurge) {
    onpurge(document.id);
  } else if (ondelete) {
    ondelete(document.id);
  }
}

function restoreDocument() {
  if (!document) return;
  open = false;
  onrestore?.(document.id);
}

function archiveDoc() {
  if (!document) return;
  onarchive?.(document.id);
}

function setImageZoom(value: number) {
  imageZoom = Math.min(4, Math.max(0.5, value));
}

function handleCanvasWheel(event: WheelEvent) {
  if (!isImage) return;
  event.preventDefault();
  setImageZoom(imageZoom + (event.deltaY < 0 ? 0.25 : -0.25));
}

function startPan(event: PointerEvent) {
  if (!isImage || event.button !== 0) return;
  event.preventDefault();
  isPanning = true;
  panStartX = event.clientX;
  panStartY = event.clientY;
  panOriginX = imagePanX;
  panOriginY = imagePanY;
  window.addEventListener("pointermove", movePan);
  window.addEventListener("pointerup", endPan);
  window.addEventListener("pointercancel", endPan);
}

function movePan(event: PointerEvent) {
  if (!isPanning) return;
  imagePanX = panOriginX + event.clientX - panStartX;
  imagePanY = panOriginY + event.clientY - panStartY;
}

function endPan() {
  isPanning = false;
  window.removeEventListener("pointermove", movePan);
  window.removeEventListener("pointerup", endPan);
  window.removeEventListener("pointercancel", endPan);
}
</script>

<Sheet bind:open>
  <SheetContent class="flex flex-col h-full sm:max-w-xl md:max-w-2xl overflow-hidden p-0 gap-0">
    {#if document}
      <!-- Header -->
      <div class="flex items-center justify-between border-b border-border/40 px-6 py-4">
        <div class="flex items-center gap-3 min-w-0">
          <div class="p-2 rounded-lg bg-primary/10 text-primary shrink-0">
            <DocumentPage class="w-5 h-5" />
          </div>
          <div class="min-w-0">
            <h3 class="font-semibold text-foreground text-sm truncate leading-tight">{document.name}</h3>
            <p class="text-xs text-muted-foreground mt-0.5 flex items-center gap-2">
              <span class="uppercase font-mono font-medium">{document.extension || "FILE"}</span>
              <span>•</span>
              <span>{formatSize(document.file_size_bytes)}</span>
              {#if isTrash}
                <span>•</span>
                <span class="text-rose-500 font-medium">{m.operations_ui_in_trash()}</span>
              {/if}
            </p>
          </div>
        </div>

        {#if isImage}
          <div class="flex items-center gap-1 bg-muted/40 rounded-lg p-1 border border-border/30">
            <Button
              variant="ghost"
              size="icon"
              class="h-7 w-7 text-muted-foreground hover:text-foreground"
              onclick={() => setImageZoom(imageZoom - 0.25)}
              title={m.operations_ui_minimize()}
            >
              <Collapse class="h-3.5 w-3.5" />
            </Button>
            <span class="text-[11px] font-mono font-medium px-1 text-muted-foreground">{Math.round(imageZoom * 100)}%</span>
            <Button
              variant="ghost"
              size="icon"
              class="h-7 w-7 text-muted-foreground hover:text-foreground"
              onclick={() => setImageZoom(imageZoom + 0.25)}
              title={m.operations_ui_maximize()}
            >
              <Expand class="h-3.5 w-3.5" />
            </Button>
          </div>
        {/if}
      </div>

      <!-- Preview Stage -->
      <div class="relative flex-1 min-h-[320px] bg-muted/20 flex items-center justify-center overflow-hidden">
        {#if isImage}
          <!-- svelte-ignore a11y_no_static_element_interactions -->
          <div
            class="relative flex h-full w-full items-center justify-center overflow-hidden select-none"
            onwheel={handleCanvasWheel}
            onpointerdown={startPan}
          >
            {#if url}
              <img
                src={url}
                alt={document.name}
                class="max-h-full max-w-full object-contain transition-transform duration-75"
                style="transform: translate({imagePanX}px, {imagePanY}px) scale({imageZoom}); cursor: {isPanning ? 'grabbing' : 'grab'};"
                onload={(event) => {
                  const target = event.currentTarget as HTMLImageElement;
                  imageWidth = target.naturalWidth;
                  imageHeight = target.naturalHeight;
                }}
              />
            {:else}
              <div class="text-center p-6 text-muted-foreground">
                <DocumentPage class="w-12 h-12 mx-auto mb-2 opacity-40" />
                <p class="text-xs">{m.operations_ui_preview_loading()}</p>
              </div>
            {/if}
          </div>
        {:else if isVideo}
          {#if url}
            <video
              src={url}
              controls
              autoplay={false}
              class="max-h-full max-w-full rounded-lg bg-black"
            >
              <track kind="captions" />
            </video>
          {/if}
        {:else if isAudio}
          {#if url}
            <div class="p-8 w-full max-w-md bg-card rounded-2xl border border-border/50 shadow-sm text-center">
              <div class="w-16 h-16 rounded-full bg-primary/10 text-primary flex items-center justify-center mx-auto mb-4">
                <Sparkles class="w-8 h-8" />
              </div>
              <h4 class="font-medium text-sm mb-4 truncate text-foreground">{document.name}</h4>
              <audio src={url} controls class="w-full"></audio>
            </div>
          {/if}
        {:else if isPdf}
          {#if url}
            <iframe
              src={url}
              title={document.name}
              class="w-full h-full border-0"
            ></iframe>
          {/if}
        {:else}
          <div class="text-center p-8 max-w-sm">
            <div class="w-16 h-16 rounded-2xl bg-muted/60 text-muted-foreground flex items-center justify-center mx-auto mb-3">
              <DocumentPage class="w-8 h-8" />
            </div>
            <h4 class="font-medium text-sm text-foreground mb-1">{document.name}</h4>
            <p class="text-xs text-muted-foreground">{m.operations_ui_preview_not_available_download_hint()}</p>
          </div>
        {/if}
      </div>

      <!-- Metadata Attributes -->
      <div class="border-t border-border/40 bg-card/40 px-6 py-4 space-y-3">
        <h4 class="text-xs font-semibold text-muted-foreground uppercase tracking-wider">{m.operations_ui_file_details()}</h4>
        
        <div class="grid grid-cols-2 gap-x-4 gap-y-2 text-xs">
          <div>
            <span class="text-[10px] font-semibold uppercase text-muted-foreground block">{m.operations_ui_file_type()}</span>
            <span class="font-mono text-foreground mt-0.5 block truncate">{document.mime_type || m.operations_ui_unspecified()}</span>
          </div>
          
          <div>
            <span class="text-[10px] font-semibold uppercase text-muted-foreground block">{m.operations_ui_size()}</span>
            <span class="text-foreground mt-0.5 block">{formatSize(document.file_size_bytes)}</span>
          </div>

          <div>
            <span class="text-[10px] font-semibold uppercase text-muted-foreground block">{m.operations_ui_category()}</span>
            <span class="text-foreground mt-0.5 block truncate">{document.category || m.operations_ui_general_category()}</span>
          </div>

          <div>
            <span class="text-[10px] font-semibold uppercase text-muted-foreground block">{m.operations_ui_date_added()}</span>
            <span class="text-foreground mt-0.5 block truncate">{formatDate(document.created_at)}</span>
          </div>

          {#if isTrash && document.deleted_at}
            <div class="col-span-2">
              <span class="text-[10px] font-semibold uppercase text-rose-500 block">{m.operations_ui_deleted_date()}</span>
              <span class="text-rose-600 dark:text-rose-400 font-medium mt-0.5 block truncate">{formatDate(document.deleted_at)}</span>
            </div>
          {/if}

          {#if document.checksum_sha256}
            <div class="col-span-2">
              <span class="text-[10px] font-semibold uppercase text-muted-foreground block">{m.operations_ui_checksum_sha256()}</span>
              <span class="font-mono text-[10px] text-muted-foreground block truncate" title={document.checksum_sha256}>{document.checksum_sha256}</span>
            </div>
          {/if}

          {#if document.storage_provider}
            <div>
              <span class="text-[10px] font-semibold uppercase text-muted-foreground block">{m.operations_ui_storage_provider()}</span>
              <span class="text-foreground mt-0.5 block font-mono capitalize">{document.storage_provider}</span>
            </div>
          {/if}

          {#if isImage && imageWidth > 0}
            <div>
              <span class="text-[10px] font-semibold uppercase text-muted-foreground block">{m.operations_ui_image_dimensions()}</span>
              <span class="font-mono text-foreground mt-0.5 block">{m.operations_ui_pixel_dimensions({ width: imageWidth, height: imageHeight })}</span>
            </div>
          {/if}
        </div>
      </div>

      <!-- Action Footer -->
      <SheetFooter class="shrink-0 flex-row justify-between items-center gap-2 border-t border-border/40 bg-card px-6 py-3">
        <div class="flex items-center gap-2">
          {#if isTrash}
            <Button variant="default" size="sm" class="bg-emerald-600 hover:bg-emerald-700 text-white" onclick={restoreDocument}>
              <Refresh class="w-3.5 h-3.5 me-1.5" />
              <span>{m.operations_ui_restore_file()}</span>
            </Button>
          {:else if onarchive && !document.is_archived}
            <Button variant="outline" size="sm" onclick={archiveDoc}>
              <ArchiveBox class="w-3.5 h-3.5 me-1.5" />
              <span>{m.operations_ui_archive_file()}</span>
            </Button>
          {/if}
        </div>

        <div class="flex items-center gap-2">
          <Button
            variant="outline"
            size="sm"
            onclick={() => {
              const link = window.document.createElement("a");
              link.href = url;
              link.download = document.name;
              link.click();
            }}
            disabled={!url}
          >
            <Download class="w-3.5 h-3.5 me-1.5" />
            <span>{m.operations_ui_download_file()}</span>
          </Button>

          {#if isTrash}
            <Button variant="destructive" size="sm" onclick={removeDocument}>
              <Trash class="w-3.5 h-3.5 me-1.5" />
              <span>{m.operations_ui_trash_permanently_delete()}</span>
            </Button>
          {:else}
            <Button variant="destructive" size="sm" onclick={removeDocument}>
              <Trash class="w-3.5 h-3.5 me-1.5" />
              <span>{m.common_action_delete()}</span>
            </Button>
          {/if}
        </div>
      </SheetFooter>
    {/if}
  </SheetContent>
</Sheet>
