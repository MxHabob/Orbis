<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import {
  Sheet,
  SheetContent,
  SheetFooter,
} from "@orbis/ui/sheet";
import { Download, File, Trash2, ZoomIn, ZoomOut, Maximize2 } from "@lucide/svelte";
import type { Documents } from "@orbis/sdk-core";

let {
  open = $bindable(false),
  document,
  url,
  ondelete,
}: {
  open: boolean;
  document: Documents | null;
  url: string;
  ondelete: (id: string) => void;
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

const isImage = $derived(document?.mime_type?.startsWith("image/") ?? false);
const isVideo = $derived(document?.mime_type?.startsWith("video/") ?? false);
const isAudio = $derived(document?.mime_type?.startsWith("audio/") ?? false);
const isPdf = $derived(document?.mime_type === "application/pdf");
const documentId = $derived(document?.id);

$effect(() => {
  documentId;
  imageZoom = 1;
  imagePanX = 0;
  imagePanY = 0;
  imageWidth = 0;
  imageHeight = 0;
});

function formatDate(value?: string) {
  if (!value) return "-";
  return new Intl.DateTimeFormat(undefined, { dateStyle: "medium", timeStyle: "short" }).format(new Date(value));
}

function removeDocument() {
  if (!document) return;
  open = false;
  ondelete(document.id);
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
  const target = event.currentTarget as HTMLElement;
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

function endPan(event: PointerEvent) {
  if (!isPanning) return;
  isPanning = false;
  window.removeEventListener("pointermove", movePan);
  window.removeEventListener("pointerup", endPan);
  window.removeEventListener("pointercancel", endPan);
}
</script>

<Sheet bind:open>
  <SheetContent
    contentClass="sm:max-w-5xl lg:max-w-275"
    class="flex h-full flex-col gap-0 overflow-hidden p-0"
  >
    {#if document}
      <div class="relative min-h-0 grow overflow-auto bg-muted/20 p-6">
        {#if isImage}
          <div class="absolute inset-e-4 top-4 z-10 flex items-center gap-1 rounded-md border border-border/60 bg-background/90 p-1 shadow-sm">
            <Button variant="ghost" size="icon-xs" onclick={() => setImageZoom(imageZoom - 0.25)} disabled={imageZoom <= 0.5} title={m.archive_zoom_out()} aria-label={m.archive_zoom_out()}>
              <ZoomOut size={14} />
            </Button>
            <span class="min-w-12 text-center text-[10px] font-medium text-muted-foreground">{Math.round(imageZoom * 100)}%</span>
            <Button variant="ghost" size="icon-xs" onclick={() => setImageZoom(imageZoom + 0.25)} disabled={imageZoom >= 4} title={m.archive_zoom_in()} aria-label={m.archive_zoom_in()}>
              <ZoomIn size={14} />
            </Button>
            <Button variant="ghost" size="icon-xs" onclick={() => { imageZoom = 1; imagePanX = 0; imagePanY = 0; }} disabled={imageZoom === 1 && imagePanX === 0 && imagePanY === 0} title={m.archive_zoom_reset()} aria-label={m.archive_zoom_reset()}>
              <Maximize2 size={14} />
            </Button>
          </div>
        {/if}

        <div
          class="flex min-h-full items-center justify-center {isPanning ? 'cursor-grabbing' : 'cursor-grab'}"
          style="touch-action: none;"
          role="application"
          aria-label={document.name}
          onwheel={handleCanvasWheel}
          onpointerdown={startPan}
        >
          {#if isImage}
            <img
              src={url}
              alt={document.name}
              draggable="false"
              class="max-h-[68vh] max-w-full object-contain shadow-sm transition-transform duration-150"
              style={`transform: translate(${imagePanX}px, ${imagePanY}px) scale(${imageZoom}); cursor: ${isPanning ? "grabbing" : "grab"};`}
              onload={(event) => {
                const image = event.currentTarget as HTMLImageElement;
                imageWidth = image.naturalWidth;
                imageHeight = image.naturalHeight;
              }}
            />
          {:else if isVideo}
            <!-- Uploaded videos do not currently provide a captions track. -->
            <!-- svelte-ignore a11y_media_has_caption -->
            <video src={url} controls preload="metadata" class="max-h-[68vh] max-w-full bg-black shadow-sm" aria-label={document.name}></video>
          {:else if isAudio}
            <audio src={url} controls preload="metadata" class="w-full max-w-xl" aria-label={document.name}></audio>
          {:else if isPdf}
            <iframe src={url} title={document.name} class="h-[68vh] min-h-105 w-full border-0 bg-background"></iframe>
          {:else}
            <div class="flex flex-col items-center gap-3 text-center text-muted-foreground">
              <File class="h-16 w-16" strokeWidth={1.25} />
              <p class="text-sm">{document.mime_type || m.archive_preview_unsupported()}</p>
            </div>
          {/if}
        </div>
      </div>

      <div class="grid shrink-0 grid-cols-2 gap-x-6 gap-y-3 border-t border-border/40 px-6 py-4 text-start sm:grid-cols-3">
        <div class="col-span-2 sm:col-span-3">
          <p class="mt-1 truncate text-xs text-foreground" title={document.name}>{document.name}</p>
        </div>
        <div>
          <p class="text-[10px] font-semibold uppercase text-muted-foreground">{m.archive_detail_type()}</p>
          <p class="mt-1 truncate text-xs text-foreground">{document.mime_type || "-"}</p>
        </div>
        <div>
          <p class="text-[10px] font-semibold uppercase text-muted-foreground">{m.archive_detail_size()}</p>
          <p class="mt-1 text-xs text-foreground">{((document.size || 0) / 1024).toFixed(1)} KB</p>
        </div>
        <div>
          <p class="text-[10px] font-semibold uppercase text-muted-foreground">{m.archive_detail_category()}</p>
          <p class="mt-1 truncate text-xs text-foreground">{document.category || "-"}</p>
        </div>
        {#if isImage}
          <div>
            <p class="text-[10px] font-semibold uppercase text-muted-foreground">{m.archive_detail_dimensions()}</p>
            <p class="mt-1 text-xs text-foreground">{imageWidth > 0 ? `${imageWidth} x ${imageHeight} px` : "-"}</p>
          </div>
        {/if}
        <div>
          <p class="text-[10px] font-semibold uppercase text-muted-foreground">{m.archive_detail_uploaded()}</p>
          <p class="mt-1 text-xs text-foreground">{formatDate(document.created_at)}</p>
        </div>
        <div class="col-span-2 sm:col-span-1">
          <p class="text-[10px] font-semibold uppercase text-muted-foreground">{m.archive_detail_visibility()}</p>
          <p class="mt-1 text-xs text-foreground">{document.visibility || "-"}</p>
        </div>
      </div>

      <SheetFooter class="shrink-0 flex-row justify-end gap-2 border-t border-border/40 px-6 py-4">
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
          <Download size={14} class="me-1.5" />
          {m.settings_archive_action_download()}
        </Button>
        <Button variant="destructive" size="sm" onclick={removeDocument}>
          <Trash2 size={14} class="me-1.5" />
          {m.common_action_delete()}
        </Button>
      </SheetFooter>
    {/if}
  </SheetContent>
</Sheet>
