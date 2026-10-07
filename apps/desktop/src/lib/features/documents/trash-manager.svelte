<!--
  @component TrashManager
  Dedicated Recycle Bin & Trash Manager for soft-deleted documents.
  Built with Svelte 5 runes, batch operations, restoration, and permanent purge.
-->
<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";
import { goto } from "$app/navigation";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import {
  Trash,
  Refresh,
  Search,
  Grid,
  Bars3BottomLeft,
  ChevronLeft,
  ChevronRight,
  FolderOpen,
  Check,
  CheckCircle,
  XMark,
  ClockRotateRight,
  ArchiveBox,
} from "@orbis/icons";
import { confirmStore } from "$lib/stores";
import { toast } from "@orbis/ui";
import {
  useTrashDocuments,
  useRestoreDocument,
  usePurgeDocument,
  resolveDocumentUrl,
  getDocumentSource,
} from "$lib/features/documents/queries";
import TrashDocumentCard from "$lib/features/documents/trash-document-card.svelte";
import TrashDocumentRow from "$lib/features/documents/trash-document-row.svelte";
import ArchivePreviewSheet from "$lib/features/documents/archive-preview-sheet.svelte";
import ArchiveLoading from "$lib/features/documents/archive-loading.svelte";
import SettingsBottomBar from "$lib/shared/components/settings-bottom-bar.svelte";
import type { DocumentDto } from "$lib/api";

let {
  orgId,
}: {
  orgId: string;
} = $props();

// View and filter states
let viewMode = $state<"grid" | "table">("grid");
let searchQuery = $state("");
let selectedCategory = $state<string>("all");
let selectedAssetType = $state<string>("all");
let selectedIds = $state<Set<string>>(new Set());

// Preview sheet states
let isPreviewOpen = $state(false);
let selectedDocument = $state<DocumentDto | null>(null);
let selectedDocumentUrl = $state("");

// Filter Labels for Selects
const assetTypeLabel = $derived.by(() => {
  switch (selectedAssetType) {
    case "pdf": return m.operations_ui_pdf_documents();
    case "image": return m.operations_ui_image();
    case "video": return m.operations_ui_video();
    case "audio": return m.operations_ui_audio();
    default: return m.operations_ui_all_types();
  }
});

const categoryLabel = $derived(
  selectedCategory === "all" ? m.operations_ui_all_categories() : selectedCategory
);

// Queries & Mutations
const trashQuery = useTrashDocuments();
const restoreMutation = useRestoreDocument();
const purgeMutation = usePurgeDocument();

const trashedDocs = $derived<DocumentDto[]>(trashQuery.data || []);
const isLoading = $derived(trashQuery.isLoading);

// Available categories derived from trashed items
const availableCategories = $derived.by(() => {
  const set = new Set<string>();
  for (const doc of trashedDocs) {
    if (doc.category) set.add(doc.category);
  }
  return Array.from(set);
});

// Filtered items
const filteredDocs = $derived.by(() => {
  const q = searchQuery.trim().toLowerCase();
  return trashedDocs.filter((doc) => {
    // Category filter
    if (selectedCategory !== "all" && doc.category !== selectedCategory) {
      return false;
    }

    // Asset type filter
    if (selectedAssetType !== "all") {
      const mime = doc.mime_type?.toLowerCase() || "";
      if (selectedAssetType === "image" && !mime.startsWith("image/")) return false;
      if (selectedAssetType === "video" && !mime.startsWith("video/")) return false;
      if (selectedAssetType === "audio" && !mime.startsWith("audio/")) return false;
      if (selectedAssetType === "pdf" && mime !== "application/pdf" && doc.extension?.toLowerCase() !== "pdf") return false;
    }

    // Search query
    if (q) {
      const matchName = doc.name.toLowerCase().includes(q);
      const matchDesc = (doc.description || "").toLowerCase().includes(q);
      const matchTags = (doc.tags || "").toLowerCase().includes(q);
      if (!matchName && !matchDesc && !matchTags) return false;
    }

    return true;
  });
});

// Preview handler
async function handlePreview(doc: DocumentDto) {
  selectedDocument = doc;
  selectedDocumentUrl = await resolveDocumentUrl(getDocumentSource(doc));
  isPreviewOpen = true;
}

// Restore single document
async function handleRestoreDoc(id: string) {
  try {
    await restoreMutation.mutateAsync(id);
    toast.success(m.operations_ui_trash_restore_success());
    selectedIds.delete(id);
    selectedIds = new Set(selectedIds);
    trashQuery.refetch();
  } catch (e: any) {
    toast.error(e?.message || m.operations_ui_trash_restore_error());
  }
}

// Purge single document
async function handlePurgeDoc(id: string) {
  const confirmed = await confirmStore.prompt({
    title: m.operations_ui_trash_permanently_delete(),
    description: m.operations_ui_trash_permanently_delete_confirm(),
    confirmText: m.operations_ui_trash_permanently_delete(),
    variant: "destructive",
  });
  if (confirmed) {
    try {
      await purgeMutation.mutateAsync(id);
      toast.success(m.operations_ui_trash_delete_success());
      selectedIds.delete(id);
      selectedIds = new Set(selectedIds);
      trashQuery.refetch();
    } catch (e: any) {
      toast.error(e?.message || m.operations_ui_trash_delete_error());
    }
  }
}

// Empty entire trash
async function handleEmptyTrash() {
  if (trashedDocs.length === 0) return;

  const confirmed = await confirmStore.prompt({
    title: m.operations_ui_trash_empty_title(),
    description: m.operations_ui_trash_empty_confirm({ count: trashedDocs.length }),
    confirmText: m.operations_ui_trash_empty_action(),
    variant: "destructive",
  });

  if (confirmed) {
    try {
      const promises = trashedDocs.map((doc) => purgeMutation.mutateAsync(doc.id));
      await Promise.all(promises);
      toast.success(m.operations_ui_trash_empty_success());
      selectedIds = new Set();
      trashQuery.refetch();
    } catch (e: any) {
      toast.error(e?.message || m.operations_ui_trash_empty_error());
    }
  }
}

// Multi-select handlers
function toggleSelect(id: string, isSelected: boolean) {
  if (isSelected) {
    selectedIds.add(id);
  } else {
    selectedIds.delete(id);
  }
  selectedIds = new Set(selectedIds);
}

function selectAll() {
  if (selectedIds.size === filteredDocs.length) {
    selectedIds = new Set();
  } else {
    selectedIds = new Set(filteredDocs.map((d) => d.id));
  }
}

// Batch restore
async function handleBatchRestore() {
  if (selectedIds.size === 0) return;
  const count = selectedIds.size;
  try {
    const promises = Array.from(selectedIds).map((id) => restoreMutation.mutateAsync(id));
    await Promise.all(promises);
    toast.success(m.operations_ui_trash_restore_many_success({ count }));
    selectedIds = new Set();
    trashQuery.refetch();
  } catch (e: any) {
    toast.error(e?.message || m.operations_ui_trash_restore_many_error());
  }
}

// Batch purge
async function handleBatchPurge() {
  if (selectedIds.size === 0) return;
  const count = selectedIds.size;
  const confirmed = await confirmStore.prompt({
    title: m.operations_ui_trash_delete_many_title({ count }),
    description: m.operations_ui_trash_delete_many_confirm(),
    confirmText: m.operations_ui_trash_permanently_delete(),
    variant: "destructive",
  });

  if (confirmed) {
    try {
      const promises = Array.from(selectedIds).map((id) => purgeMutation.mutateAsync(id));
      await Promise.all(promises);
      toast.success(m.operations_ui_trash_delete_many_success({ count }));
      selectedIds = new Set();
      trashQuery.refetch();
    } catch (e: any) {
      toast.error(e?.message || m.operations_ui_trash_delete_many_error());
    }
  }
}
</script>

<div class="h-full flex flex-col bg-background overflow-hidden">
  <!-- Top Navigation & Actions Bar -->
  <header class="shrink-0 border-b border-border/40 bg-card/60 backdrop-blur px-6 py-4">
    <div class="flex flex-col gap-4 md:flex-row md:items-center md:justify-between">
      <!-- Breadcrumbs & Heading -->
      <div>
        <nav class="flex items-center gap-1.5 text-xs text-muted-foreground mb-1">
          <button
            type="button"
            class="hover:text-foreground transition-colors flex items-center gap-1"
            onclick={() => goto(`/${orgId}/settings/archive`)}
          >
            <ArchiveBox class="w-3.5 h-3.5" />
            <span>{m.operations_ui_documents_archive()}</span>
          </button>
          <span>/</span>
          <span class="text-foreground font-medium flex items-center gap-1 text-rose-500">
            <Trash class="w-3.5 h-3.5" />
            <span>{m.operations_ui_trash_title()}</span>
          </span>
        </nav>

        <div class="flex items-center gap-2">
          <h1 class="text-xl font-bold text-foreground">{m.operations_ui_trash_title()}</h1>
          <span class="rounded-full bg-rose-500/10 text-rose-600 dark:text-rose-400 px-2.5 py-0.5 text-xs font-semibold">
            {m.operations_ui_trash_file_count({ count: trashedDocs.length })}
          </span>
        </div>
      </div>

      <!-- Header Action Buttons -->
      <div class="flex items-center gap-2">
        <Button
          variant="outline"
          size="sm"
          onclick={() => trashQuery.refetch()}
          title={m.operations_ui_refresh_data()}
          class="h-9"
        >
          <LoadingIndicator class="w-4 h-4 {trashQuery.isFetching ? '' : ''}"  size={16} />
          <span class="hidden sm:inline ms-1.5">{m.operations_ui_refresh()}</span>
        </Button>

        {#if trashedDocs.length > 0}
          <Button
            variant="destructive"
            size="sm"
            onclick={handleEmptyTrash}
            class="h-9"
          >
            <Trash class="w-4 h-4 me-1.5" />
            <span>{m.operations_ui_trash_empty_action()}</span>
          </Button>
        {/if}

        <Button
          variant="outline"
          size="sm"
          onclick={() => goto(`/${orgId}/settings/archive`)}
          class="h-9"
        >
          <FolderOpen class="w-4 h-4 me-1.5 text-primary" />
          <span>{m.operations_ui_return_to_archive()}</span>
        </Button>
      </div>
    </div>

    <!-- Search & Filter Controls -->
    <div class="mt-4 flex flex-wrap items-center justify-between gap-3 pt-3 border-t border-border/30">
      <!-- Search Input -->
      <div class="relative w-full max-w-xs">
        <Search class="absolute inset-s-3 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground" />
        <Input
          type="search"
          placeholder={m.operations_ui_trash_search_placeholder()}
          bind:value={searchQuery}
          class="ps-9 pe-8 h-8 text-xs bg-background/80"
        />
        {#if searchQuery}
          <button
            type="button"
            class="absolute inset-e-2.5 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground"
            onclick={() => (searchQuery = "")}
          >
            <XMark class="w-3.5 h-3.5" />
          </button>
        {/if}
      </div>

      <!-- Filters & View Switcher -->
      <div class="flex items-center gap-2 flex-wrap text-xs">
        <!-- Asset Type Filter -->
        <Select type="single" bind:value={selectedAssetType}>
          <SelectTrigger class="h-8 min-w-32 text-xs">
            <span class="truncate">{assetTypeLabel}</span>
          </SelectTrigger>
          <SelectContent align="end">
            <SelectItem value="all">{m.operations_ui_all_types()}</SelectItem>
            <SelectItem value="pdf">{m.operations_ui_pdf_documents()}</SelectItem>
            <SelectItem value="image">{m.operations_ui_image()}</SelectItem>
            <SelectItem value="video">{m.operations_ui_video()}</SelectItem>
            <SelectItem value="audio">{m.operations_ui_audio()}</SelectItem>
          </SelectContent>
        </Select>

        <!-- Category Filter -->
        {#if availableCategories.length > 0}
          <Select type="single" bind:value={selectedCategory}>
            <SelectTrigger class="h-8 min-w-32 text-xs">
              <span class="truncate">{categoryLabel}</span>
            </SelectTrigger>
            <SelectContent align="end">
              <SelectItem value="all">{m.operations_ui_all_categories()}</SelectItem>
              {#each availableCategories as cat}
                <SelectItem value={cat}>{cat}</SelectItem>
              {/each}
            </SelectContent>
          </Select>
        {/if}

        <!-- View Switcher (Grid / Table) -->
        <div class="flex items-center rounded-lg border border-border/60 bg-muted/40 p-0.5">
          <Button
            variant="ghost"
            size="icon"
            class="h-7 w-7 rounded-md {viewMode === 'grid' ? 'bg-background shadow-xs text-foreground' : 'text-muted-foreground'}"
            onclick={() => (viewMode = "grid")}
            title={m.operations_ui_grid_view()}
          >
            <Grid class="w-3.5 h-3.5" />
          </Button>

          <Button
            variant="ghost"
            size="icon"
            class="h-7 w-7 rounded-md {viewMode === 'table' ? 'bg-background shadow-xs text-foreground' : 'text-muted-foreground'}"
            onclick={() => (viewMode = "table")}
            title={m.operations_ui_table_view()}
          >
            <Bars3BottomLeft class="w-3.5 h-3.5" />
          </Button>
        </div>
      </div>
    </div>
  </header>

  <!-- Main Content Stage -->
  <main class="flex-1 overflow-y-auto p-6 scrollbar-hide">
    {#if isLoading}
      <ArchiveLoading />
    {:else if filteredDocs.length === 0}
      <!-- Empty Trash State -->
      <div class="h-full flex flex-col items-center justify-center text-center max-w-sm mx-auto py-16">
        <div class="relative mb-4">
          <div class="w-20 h-20 rounded-2xl bg-muted/60 flex items-center justify-center text-muted-foreground/60 shadow-inner">
            <Trash class="w-10 h-10" />
          </div>
          <div class="absolute -bottom-1 -end-1 w-6 h-6 rounded-full bg-emerald-500 text-white flex items-center justify-center text-xs shadow">
            <Check class="w-3.5 h-3.5" />
          </div>
        </div>

        <h3 class="text-base font-semibold text-foreground mb-1">{m.operations_ui_trash_empty_heading()}</h3>
        <p class="text-xs text-muted-foreground mb-6 leading-relaxed">
          {searchQuery
            ? m.operations_ui_trash_no_search_results()
            : m.operations_ui_trash_empty_description()}
        </p>

        <Button
          variant="outline"
          size="sm"
          onclick={() => goto(`/${orgId}/settings/archive`)}
        >
          <FolderOpen class="w-4 h-4 me-1.5 text-primary" />
          <span>{m.operations_ui_browse_archive()}</span>
        </Button>
      </div>
    {:else}
      <!-- Content Views -->
      {#if viewMode === "grid"}
        <!-- Grid View -->
        <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-4">
          {#each filteredDocs as doc (doc.id)}
            <TrashDocumentCard
              document={doc}
              selected={selectedIds.has(doc.id)}
              onpreview={handlePreview}
              onrestore={handleRestoreDoc}
              onpurge={handlePurgeDoc}
              ontoggle={toggleSelect}
            />
          {/each}
        </div>
      {:else}
        <!-- Table View -->
        <div class="rounded-xl border border-border/50 bg-card overflow-hidden shadow-xs">
          <table class="w-full text-start border-collapse">
            <thead>
              <tr class="border-b border-border/60 bg-muted/50 text-[11px] font-semibold text-muted-foreground uppercase tracking-wider">
                <th class="w-10 px-4 py-3 text-center">
                  <input
                    type="checkbox"
                    checked={selectedIds.size === filteredDocs.length && filteredDocs.length > 0}
                    onchange={selectAll}
                    class="rounded border-border"
                    aria-label={m.operations_ui_select_all()}
                  />
                </th>
                <th class="px-4 py-3 text-start">{m.operations_ui_file()}</th>
                <th class="px-4 py-3 text-start">{m.operations_ui_category()}</th>
                <th class="px-4 py-3 text-start">{m.operations_ui_size()}</th>
                <th class="px-4 py-3 text-start">{m.operations_ui_deleted_date()}</th>
                <th class="px-4 py-3 text-end">{m.operations_ui_actions()}</th>
              </tr>
            </thead>
            <tbody>
              {#each filteredDocs as doc (doc.id)}
                <TrashDocumentRow
                  document={doc}
                  selected={selectedIds.has(doc.id)}
                  onpreview={handlePreview}
                  onrestore={handleRestoreDoc}
                  onpurge={handlePurgeDoc}
                  ontoggle={toggleSelect}
                />
              {/each}
            </tbody>
          </table>
        </div>
      {/if}
    {/if}
  </main>
</div>

<!-- Preview Sheet (With Trash Mode active) -->
<ArchivePreviewSheet
  bind:open={isPreviewOpen}
  document={selectedDocument}
  url={selectedDocumentUrl}
  isTrash={true}
  onrestore={handleRestoreDoc}
  onpurge={handlePurgeDoc}
/>

<!-- Floating Multi-Select Bottom Bar -->
{#snippet trashActions()}
  <Button
    type="button"
    variant="ghost"
    size="sm"
    class="h-8 px-2.5 text-xs font-medium text-muted-foreground hover:text-foreground cursor-pointer"
    onclick={() => (selectedIds = new Set())}
  >
    {m.operations_ui_clear_selection()}
  </Button>
  <Button
    type="button"
    variant="outline"
    size="sm"
    class="h-8 px-3 text-xs font-semibold text-emerald-600 border-emerald-500/30 hover:bg-emerald-500/10 cursor-pointer"
    onclick={handleBatchRestore}
  >
    <Refresh class="w-3.5 h-3.5 me-1.5" />
    <span>{m.operations_ui_trash_restore_selected()}</span>
  </Button>
  <Button
    type="button"
    variant="destructive"
    size="sm"
    class="h-8 px-3 text-xs font-semibold cursor-pointer shadow-xs"
    onclick={handleBatchPurge}
  >
    <Trash class="w-3.5 h-3.5 me-1.5" />
    <span>{m.operations_ui_trash_delete_selected()}</span>
  </Button>
{/snippet}

<SettingsBottomBar
  visible={selectedIds.size > 0}
  isDirty={selectedIds.size > 0}
  message={m.operations_ui_trash_selection_count({ selected: selectedIds.size, total: filteredDocs.length })}
  statusVariant="rose"
  actions={trashActions}
/>
