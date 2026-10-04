<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Checkbox } from "@orbis/ui/checkbox";
import {
  getDocumentSource,
  resolveDocumentUrl,
} from "$lib/features/documents/queries";
import { DocumentPage, Download, Eye, Play, SoundHigh, Trash, ArchiveBox } from "@orbis/icons";
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
const isVideo = $derived(mimeType.startsWith("video/"));
const isAudio = $derived(mimeType.startsWith("audio/"));
const isPdf = $derived(mimeType === "application/pdf" || document.extension?.toLowerCase() === "pdf");
const extension = $derived(document.extension?.toUpperCase() || "FILE");
const source = $derived(getDocumentSource(document));

function formatDate(value?: string) {
  if (!value) return "-";
  return new Intl.DateTimeFormat(undefined, { dateStyle: "medium" }).format(
    new Date(value),
  );
}

function formatSize(bytes?: number | null) {
  if (!bytes || bytes <= 0) return "0 KB";
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}
</script>

<div class="grid grid-cols-[auto_minmax(0,2.5fr)_minmax(100px,1fr)_minmax(80px,0.8fr)_minmax(100px,1fr)_auto] items-center gap-3 border-b border-border/40 px-3 py-2.5 text-xs transition-colors hover:bg-muted/40">
  <!-- Checkbox -->
  {#if ontoggle}
    <Checkbox checked={selected} onCheckedChange={(value) => ontoggle?.(document.id, value === true)} aria-label={m.operations_ui_document_select({ name: document.name })} />
  {/if}

  <!-- Name & Icon -->
  <button type="button" class="flex min-w-0 items-center gap-2.5 text-start cursor-pointer group" onclick={() => onpreview(document)}>
    <span class="flex h-8 w-8 shrink-0 items-center justify-center rounded-lg bg-muted/60 text-muted-foreground group-hover:bg-primary/10 group-hover:text-primary transition-colors">
      {#if isVideo}
        <Play class="h-4 w-4" />
      {:else if isAudio}
        <SoundHigh class="h-4 w-4" />
      {:else if isPdf}
        <DocumentPage class="h-4 w-4 text-rose-500" />
      {:else}
        <DocumentPage class="h-4 w-4" />
      {/if}
    </span>
    <span class="min-w-0">
      <span class="block truncate font-medium text-foreground group-hover:text-primary transition-colors" title={document.name}>{document.name}</span>
      <span class="block truncate text-[10px] text-muted-foreground font-mono uppercase">{extension}</span>
    </span>
  </button>

  <!-- Category -->
  <span class="truncate text-muted-foreground">{document.category || m.operations_ui_general_category()}</span>

  <!-- File Size -->
  <span class="text-muted-foreground font-mono text-[11px]">{formatSize(document.file_size_bytes)}</span>

  <!-- Created Date -->
  <span class="text-muted-foreground">{formatDate(document.created_at)}</span>

  <!-- Actions -->
  <div class="flex items-center justify-end gap-1">
    <Button variant="ghost" size="icon-xs" onclick={() => onpreview(document)} title={m.operations_ui_preview()}>
      <Eye class="h-3.5 w-3.5" />
    </Button>

    {#await resolveDocumentUrl(source) then sourceUrl}
      <a href={sourceUrl} download={document.name} class="inline-flex items-center justify-center h-7 w-7 rounded-md text-muted-foreground hover:bg-muted hover:text-foreground transition-colors" title={m.operations_ui_download()}>
        <Download class="h-3.5 w-3.5" />
      </a>
    {/await}

    {#if onarchive && !document.is_archived}
      <Button variant="ghost" size="icon-xs" onclick={() => onarchive?.(document.id)} title={m.operations_ui_archive()}>
        <ArchiveBox class="h-3.5 w-3.5" />
      </Button>
    {/if}

    {#if ondelete}
      <Button variant="ghost" size="icon-xs" onclick={() => ondelete?.(document.id)} title={m.common_action_delete()} class="text-destructive hover:bg-destructive/10">
        <Trash class="h-3.5 w-3.5" />
      </Button>
    {/if}
  </div>
</div>
