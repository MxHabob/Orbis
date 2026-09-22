<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Checkbox } from "@orbis/ui/checkbox";
import {
  getDocumentSource,
  resolveDocumentUrl,
} from "$lib/features/documents/queries";
import { DocumentPage, Download, Play, SoundHigh, Trash } from "@orbis/icons";
import type { Documents } from "@orbis/sdk";

let {
  document,
  selected = false,
  onpreview,
  ondelete,
  ontoggle,
}: {
  document: Documents;
  selected?: boolean;
  onpreview: (document: Documents) => void;
  ondelete: (id: string) => void;
  ontoggle: (id: string, selected: boolean) => void;
} = $props();

const mimeType = $derived(document.mime_type?.toLowerCase() ?? "");
const isVideo = $derived(mimeType.startsWith("video/"));
const isAudio = $derived(mimeType.startsWith("audio/"));
const extension = $derived(document.extension?.toUpperCase() || "FILE");
const source = $derived(getDocumentSource(document));

function formatDate(value?: string) {
  if (!value) return "-";
  return new Intl.DateTimeFormat(undefined, { dateStyle: "medium" }).format(
    new Date(value),
  );
}
</script>

<div class="grid grid-cols-[minmax(0,2fr)_minmax(90px,0.8fr)_minmax(90px,0.8fr)_minmax(90px,0.8fr)_auto] items-center gap-3 border-b border-border/40 px-3 py-2.5 text-xs transition-colors hover:bg-muted/30">
  <div class="flex min-w-0 items-center gap-2">
    <Checkbox checked={selected} onCheckedChange={(value: boolean | "indeterminate") => ontoggle(document.id, value === true)} aria-label={`تحديد ${document.name}`} />
    <button type="button" class="flex min-w-0 items-center gap-2 text-start" onclick={() => onpreview(document)}>
    <span class="flex h-8 w-8 shrink-0 items-center justify-center rounded-md bg-muted/60 text-muted-foreground">
      {#if isVideo}<Play class="h-4 w-4" />{:else if isAudio}<SoundHigh class="h-4 w-4" />{:else}<DocumentPage class="h-4 w-4" />{/if}
    </span>
    <span class="min-w-0">
      <span class="block truncate font-medium text-foreground" title={document.name}>{document.name}</span>
      <span class="block truncate text-[10px] text-muted-foreground">{extension}</span>
    </span>
    </button>
  </div>
  <span class="truncate text-muted-foreground">{document.category || "archive"}</span>
  <span class="text-muted-foreground">{document.asset_type || extension}</span>
  <span class="text-muted-foreground">{formatDate(document.created_at)}</span>
  <div class="flex items-center justify-end gap-1">
    {#await resolveDocumentUrl(source) then sourceUrl}
      <a href={sourceUrl} download={document.name} class="rounded-md p-1.5 text-muted-foreground transition-colors hover:bg-muted hover:text-foreground" title={m.settings_archive_action_download()} aria-label={m.settings_archive_action_download()}>
        <Download class="h-3.5 w-3.5" />
      </a>
    {/await}
    <Button variant="ghost" size="icon-sm" onclick={() => ondelete(document.id)} title={m.common_action_delete()} aria-label={m.common_action_delete()}>
      <Trash class="h-3.5 w-3.5 text-destructive" />
    </Button>
  </div>
</div>
