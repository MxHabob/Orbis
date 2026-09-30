<!--
  @component TrashDocumentRow
  Table row representation for soft-deleted documents.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Checkbox } from "@orbis/ui/checkbox";
import {
  DocumentPage,
  Eye,
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

const extension = $derived(document.extension?.toUpperCase() || "FILE");

function formatSize(bytes?: number | null) {
  if (!bytes || bytes <= 0) return "0 KB";
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}

function formatDate(value?: string | null) {
  if (!value) return "-";
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

<tr
  class="group border-b border-border/40 hover:bg-muted/30 transition-colors {selected ? 'bg-primary/5' : ''}"
>
  <!-- Multi-select Checkbox -->
  <td class="w-10 px-4 py-3 text-center">
    {#if ontoggle}
      <Checkbox
        checked={selected}
        onCheckedChange={(value) => ontoggle?.(document.id, value === true)}
        aria-label={`تحديد ${document.name}`}
      />
    {/if}
  </td>

  <!-- File Icon & Name -->
  <td class="px-4 py-3 min-w-[240px]">
    <div
      class="flex items-center gap-3 cursor-pointer"
      onclick={() => onpreview(document)}
      onkeydown={(e) => e.key === "Enter" && onpreview(document)}
      role="button"
      tabindex="0"
    >
      <div class="flex h-8 w-8 shrink-0 items-center justify-center rounded-lg bg-rose-500/10 text-rose-500">
        <DocumentPage class="h-4 w-4" />
      </div>

      <div class="min-w-0">
        <p class="font-medium text-xs text-foreground truncate group-hover:text-primary transition-colors" title={document.name}>
          {document.name}
        </p>
        <p class="text-[10px] text-muted-foreground font-mono uppercase">
          {extension}
        </p>
      </div>
    </div>
  </td>

  <!-- Category -->
  <td class="px-4 py-3 text-xs text-muted-foreground">
    <span class="rounded bg-muted/60 px-2 py-0.5 text-[11px] capitalize">
      {document.category || "عام"}
    </span>
  </td>

  <!-- File Size -->
  <td class="px-4 py-3 text-xs font-mono text-muted-foreground">
    {formatSize(document.file_size_bytes)}
  </td>

  <!-- Deletion Date -->
  <td class="px-4 py-3 text-xs text-muted-foreground">
    <div class="flex items-center gap-1.5 text-rose-600/90 dark:text-rose-400/90">
      <ClockRotateRight class="w-3.5 h-3.5 shrink-0" />
      <span>{formatDate(document.deleted_at)}</span>
    </div>
  </td>

  <!-- Actions -->
  <td class="px-4 py-3 text-end">
    <div class="flex items-center justify-end gap-1">
      <Button
        variant="ghost"
        size="icon"
        class="h-7 w-7 text-muted-foreground hover:text-foreground"
        onclick={() => onpreview(document)}
        title="معاينة"
      >
        <Eye class="h-3.5 w-3.5" />
      </Button>

      <Button
        variant="ghost"
        size="icon"
        class="h-7 w-7 text-emerald-600 hover:text-emerald-700 hover:bg-emerald-500/10"
        onclick={() => onrestore(document.id)}
        title="استعادة الملف"
      >
        <Refresh class="h-3.5 w-3.5" />
      </Button>

      <Button
        variant="ghost"
        size="icon"
        class="h-7 w-7 text-muted-foreground hover:text-destructive hover:bg-destructive/10"
        onclick={() => onpurge(document.id)}
        title="حذف نهائي"
      >
        <Trash class="h-3.5 w-3.5" />
      </Button>
    </div>
  </td>
</tr>
