<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Checkbox } from "@orbis/ui/checkbox";
import {
  getDocumentSource,
  resolveDocumentUrl,
} from "$lib/features/documents/queries";
import { DocumentPage, Download, Eye, Play, SoundHigh, Trash, ArchiveBox, Sparkles } from "@orbis/icons";
import type { DocumentDto } from "$lib/api";

let {
  document,
  selected = false,
  onpreview,
  ondelete,
  onarchive,
  ontoggle,
}: {
  document: DocumentDto;
  selected?: boolean;
  onpreview: (document: DocumentDto) => void;
  ondelete?: (id: string) => void;
  onarchive?: (id: string) => void;
  ontoggle?: (id: string, selected: boolean) => void;
} = $props();

const mimeType = $derived(document.mime_type?.toLowerCase() ?? "");
const isImage = $derived(mimeType.startsWith("image/"));
const isVideo = $derived(mimeType.startsWith("video/"));
const isAudio = $derived(mimeType.startsWith("audio/"));
const isPdf = $derived(mimeType === "application/pdf" || document.extension?.toLowerCase() === "pdf");
const source = $derived(getDocumentSource(document));
const extension = $derived(document.extension?.toUpperCase() || "FILE");
let isNearViewport = $state(false);

function stopPropagation(event: MouseEvent) {
  event.stopPropagation();
}

function loadWhenVisible(node: HTMLDivElement) {
  const observer = new IntersectionObserver(
    ([entry]) => {
      if (!entry?.isIntersecting) return;
      isNearViewport = true;
      observer.disconnect();
    },
    { rootMargin: "240px" },
  );
  observer.observe(node);
  return { destroy: () => observer.disconnect() };
}

function formatSize(bytes?: number | null) {
  if (!bytes || bytes <= 0) return "0 KB";
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}
</script>

<article class="archive-card group flex flex-col justify-between rounded-xl border border-border/50 bg-card/60 p-2.5 hover:border-border hover:shadow-md transition-all duration-200 overflow-hidden min-w-0 max-w-full">
  <!-- Media / Thumbnail Container -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="archive-media relative cursor-pointer overflow-hidden rounded-lg bg-muted/40 flex items-center justify-center w-full border border-border/30"
    use:loadWhenVisible
    role="button"
    tabindex="0"
    onclick={() => onpreview(document)}
    onkeydown={(event) => event.key === "Enter" && onpreview(document)}
  >
    <!-- Multi-select checkbox -->
    {#if ontoggle}
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <div class="absolute inset-s-2 top-2 z-10 rounded-md bg-background/85 p-1 shadow-sm opacity-80 group-hover:opacity-100 transition-opacity" onclick={stopPropagation}>
        <Checkbox checked={selected} onCheckedChange={(value) => ontoggle?.(document.id, value === true)} aria-label={m.operations_ui_document_select({ name: document.name })} />
      </div>
    {/if}

    <!-- Format / Extension Badge (Top-End) -->
    <div class="absolute inset-e-2 top-2 z-10">
      <span class="rounded px-1.5 py-0.5 text-[9px] font-mono font-bold bg-background/90 text-foreground/80 shadow-sm uppercase">
        {extension}
      </span>
    </div>

    <!-- Media Rendering -->
    {#if isNearViewport && isImage}
      {#await resolveDocumentUrl(source, document.id) then mediaUrl}
        <img src={mediaUrl} alt={document.name} loading="lazy" decoding="async" width="320" height="240" class="h-full w-full object-cover transition-transform duration-300 group-hover:scale-105" />
      {:catch}
        <DocumentPage class="m-auto h-8 w-8 text-muted-foreground/60" />
      {/await}
    {:else if isNearViewport && isVideo}
      {#await resolveDocumentUrl(source, document.id) then sourceUrl}
        <video src={sourceUrl} muted preload="none" class="h-full w-full object-cover" aria-label={document.name}></video>
        <div class="pointer-events-none absolute inset-0 flex items-center justify-center bg-black/20">
          <span class="rounded-full bg-black/60 p-2 text-white"><Play class="h-4 w-4 fill-current" /></span>
        </div>
      {/await}
    {:else if isAudio}
      <div class="flex h-full flex-col items-center justify-center gap-2 text-muted-foreground/70">
        <SoundHigh class="h-10 w-10 text-primary/70" strokeWidth={1.5} />
        <span class="text-[10px] font-medium">{m.operations_ui_audio_file()}</span>
      </div>
    {:else if isPdf}
      <div class="flex h-full flex-col items-center justify-center gap-1.5 text-muted-foreground/70">
        <DocumentPage class="h-10 w-10 text-rose-500/80" strokeWidth={1.5} />
        <span class="text-[10px] font-semibold text-rose-600 dark:text-rose-400">{m.operations_ui_pdf_document()}</span>
      </div>
    {:else}
      <div class="flex h-full flex-col items-center justify-center gap-2 text-muted-foreground/70">
        <DocumentPage class="h-10 w-10 text-primary/60" strokeWidth={1.5} />
        <span class="rounded bg-muted px-2 py-0.5 text-[10px] font-semibold">{extension}</span>
      </div>
    {/if}

    <!-- Hover Actions Overlay -->
    <div class="absolute inset-0 flex items-center justify-center gap-1.5 bg-background/70 opacity-0 backdrop-blur-[2px] transition-opacity group-hover:opacity-100">
      <Button
        variant="secondary"
        size="icon-xs"
        onclick={(event) => { stopPropagation(event); onpreview(document); }}
        title={m.operations_ui_preview()}
      >
        <Eye class="h-3.5 w-3.5" />
      </Button>

      {#if isNearViewport}
        {#await resolveDocumentUrl(source, document.id) then sourceUrl}
          <a
            href={sourceUrl}
            download={document.name}
            class="inline-flex items-center justify-center h-7 w-7 rounded-md bg-secondary text-secondary-foreground hover:bg-secondary/80 text-xs shadow-sm transition-colors"
            title={m.settings_archive_action_download()}
            onclick={stopPropagation}
          >
            <Download class="h-3.5 w-3.5" />
          </a>
        {/await}
      {/if}

      {#if onarchive && !document.is_archived}
        <Button
          variant="outline"
          size="icon-xs"
          onclick={(event) => { stopPropagation(event); onarchive?.(document.id); }}
          title={m.operations_ui_archive()}
        >
          <ArchiveBox class="h-3.5 w-3.5" />
        </Button>
      {/if}

      {#if ondelete}
        <Button
          variant="destructive"
          size="icon-xs"
          onclick={(event) => { stopPropagation(event); ondelete?.(document.id); }}
          title={m.common_action_delete()}
        >
          <Trash class="h-3.5 w-3.5" />
        </Button>
      {/if}
    </div>
  </div>

  <!-- Document Metadata -->
  <div class="min-w-0 w-full pt-2 text-start overflow-hidden">
    <p class="truncate block w-full text-xs font-semibold text-foreground" title={document.name}>
      {document.name}
    </p>
    <div class="mt-1 flex items-center justify-between gap-2 text-[10px] text-muted-foreground min-w-0">
      <span class="truncate font-medium">{document.category || m.operations_ui_general_category()}</span>
      <span class="shrink-0 font-mono">{formatSize(document.file_size_bytes)}</span>
    </div>
  </div>
</article>

<style>
  .archive-card {
    content-visibility: auto;
    contain-intrinsic-size: 240px;
  }

  .archive-media {
    aspect-ratio: 4 / 3;
  }
</style>
