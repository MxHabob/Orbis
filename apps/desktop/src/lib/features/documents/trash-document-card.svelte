<!--
  @component TrashDocumentCard
  Card display for soft-deleted documents with restore and permanent purge actions.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Checkbox } from "@orbis/ui/checkbox";
import {
  getDocumentSource,
  resolveDocumentUrl,
} from "$lib/features/documents/queries";
import {
  DocumentPage,
  Eye,
  Play,
  Trash,
  Refresh,
  ClockRotateRight,
} from "@orbis/icons";
import type { DocumentDto } from "$lib/api";

let {
  document,
  selected = false,
  onpreview,
  onrestore,
  onpurge,
  ontoggle,
}: {
  document: DocumentDto;
  selected?: boolean;
  onpreview: (document: DocumentDto) => void;
  onrestore: (id: string) => void;
  onpurge: (id: string) => void;
  ontoggle?: (id: string, selected: boolean) => void;
} = $props();

const mimeType = $derived(document.mime_type?.toLowerCase() ?? "");
const isImage = $derived(mimeType.startsWith("image/"));
const isVideo = $derived(mimeType.startsWith("video/"));
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

function formatDate(value?: string | null) {
  if (!value) return "";
  try {
    return new Intl.DateTimeFormat(undefined, {
      month: "short",
      day: "numeric",
      hour: "2-digit",
      minute: "2-digit",
    }).format(new Date(value));
  } catch {
    return value;
  }
}
</script>

<article
  class="group relative flex flex-col justify-between rounded-xl border border-border/60 bg-card/70 p-3 hover:border-rose-500/40 hover:shadow-md transition-all duration-200"
>
  <!-- Media / Thumbnail Container -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="relative cursor-pointer overflow-hidden rounded-lg bg-muted/40 aspect-[4/3] flex items-center justify-center border border-border/30"
    use:loadWhenVisible
    role="button"
    tabindex="0"
    onclick={() => onpreview(document)}
    onkeydown={(event) => event.key === "Enter" && onpreview(document)}
  >
    <!-- Multi-select checkbox -->
    {#if ontoggle}
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <div
        class="absolute inset-s-2 top-2 z-10 rounded-md bg-background/90 p-1 shadow-sm opacity-80 group-hover:opacity-100 transition-opacity"
        onclick={stopPropagation}
      >
        <Checkbox
          checked={selected}
          onCheckedChange={(value) => ontoggle?.(document.id, value === true)}
          aria-label={m.operations_ui_document_select({ name: document.name })}
        />
      </div>
    {/if}

    <!-- Format / Extension Badge -->
    <div class="absolute inset-e-2 top-2 z-10">
      <span class="rounded px-1.5 py-0.5 text-[9px] font-mono font-bold bg-background/90 text-foreground/80 shadow-sm uppercase">
        {extension}
      </span>
    </div>

    <!-- Trashed Warning Banner -->
    <div class="absolute inset-x-0 bottom-0 z-10 bg-gradient-to-t from-rose-950/80 via-rose-950/40 to-transparent p-1.5 pt-4 text-center">
      <div class="flex items-center justify-center gap-1 text-[10px] font-medium text-rose-200">
        <ClockRotateRight class="w-3 h-3 text-rose-400" />
        <span>{formatDate(document.deleted_at)}</span>
      </div>
    </div>

    <!-- Media Rendering -->
    {#if isNearViewport && isImage}
      {#await resolveDocumentUrl(source) then mediaUrl}
        <img
          src={mediaUrl}
          alt={document.name}
          loading="lazy"
          decoding="async"
          width="320"
          height="240"
          class="h-full w-full object-cover grayscale group-hover:grayscale-0 transition-all duration-300 group-hover:scale-105 opacity-80 group-hover:opacity-100"
        />
      {:catch}
        <DocumentPage class="m-auto h-8 w-8 text-muted-foreground/60" />
      {/await}
    {:else if isNearViewport && isVideo}
      {#await resolveDocumentUrl(source) then sourceUrl}
        <video src={sourceUrl} muted preload="none" class="h-full w-full object-cover opacity-80"></video>
        <div class="pointer-events-none absolute inset-0 flex items-center justify-center bg-black/30">
          <span class="rounded-full bg-black/60 p-2 text-white"><Play class="h-4 w-4 fill-current" /></span>
        </div>
      {/await}
    {:else}
      <div class="flex flex-col items-center justify-center text-muted-foreground/50 group-hover:text-muted-foreground transition-colors">
        <DocumentPage class="h-10 w-10" />
      </div>
    {/if}
  </div>

  <!-- Document Meta & Controls -->
  <div class="mt-2.5 flex flex-col gap-1.5">
    <!-- File Title -->
    <div class="flex items-start justify-between gap-1">
      <button
        type="button"
        class="text-xs font-semibold text-foreground/90 truncate hover:text-foreground cursor-pointer"
        title={document.name}
        onclick={() => onpreview(document)}
      >
        {document.name}
      </button>
    </div>

    <!-- Metadata Details -->
    <div class="flex items-center justify-between text-[11px] text-muted-foreground">
      <span class="font-mono">{formatSize(document.file_size_bytes)}</span>
      <span class="rounded bg-muted px-1.5 py-0.5 text-[10px] text-muted-foreground capitalize">
        {document.category || m.operations_ui_general_category()}
      </span>
    </div>

    <!-- Quick Action Buttons -->
    <div class="mt-1.5 flex items-center justify-between gap-1 pt-1.5 border-t border-border/40">
      <Button
        variant="ghost"
        size="sm"
        class="h-7 px-2 text-xs text-muted-foreground hover:text-foreground"
        onclick={() => onpreview(document)}
        title={m.operations_ui_preview()}
      >
        <Eye class="w-3.5 h-3.5 me-1" />
        <span>{m.operations_ui_preview()}</span>
      </Button>

      <div class="flex items-center gap-1">
        <Button
          variant="outline"
          size="sm"
          class="h-7 px-2 text-xs text-emerald-600 hover:text-emerald-700 hover:bg-emerald-500/10 border-emerald-500/30"
          onclick={() => onrestore(document.id)}
          title={m.operations_ui_restore_file()}
        >
          <Refresh class="w-3.5 h-3.5 me-1" />
          <span>{m.operations_ui_restore()}</span>
        </Button>

        <Button
          variant="ghost"
          size="icon"
          class="h-7 w-7 text-muted-foreground hover:text-destructive hover:bg-destructive/10"
          onclick={() => onpurge(document.id)}
          title={m.operations_ui_trash_permanently_delete()}
        >
          <Trash class="w-3.5 h-3.5" />
        </Button>
      </div>
    </div>
  </div>
</article>
