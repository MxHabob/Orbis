<!--
  @component TrashManager
  Dedicated Recycle Bin & Trash Manager for soft-deleted documents.
  Built with Svelte 5 runes, batch operations, restoration, and permanent purge.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { goto } from "$app/navigation";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import {
  Trash,
  Refresh,
  Search,
  Grid,
  Bars3BottomLeft,
  FolderOpen,
  Check,
  XMark,
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
import ArchiveSelectionCell from "$lib/features/documents/archive-selection-cell.svelte";
import ArchivePreviewSheet from "$lib/features/documents/archive-preview-sheet.svelte";
import ArchiveLoading from "$lib/features/documents/archive-loading.svelte";
import SettingsBottomBar from "$lib/shared/components/settings-bottom-bar.svelte";
import SearchableMultiFilter, {
  type SearchableMultiFilterGroup,
} from "$lib/shared/components/searchable-multi-filter.svelte";
import SplitActionMenu from "$lib/shared/components/split-action-menu.svelte";
import {
  OrbisGrid,
  type GridActionEvent,
  type OrbisColumnDef,
} from "@orbis/svelte-grid";
import { getLocale } from "$lib/paraglide/runtime";
import type { DocumentDto } from "$lib/api";

let {
  orgId,
}: {
  orgId: string;
} = $props();

// View and filter states
let viewMode = $state<"grid" | "table">("table");
let searchQuery = $state("");
let selectedFilters = $state<string[]>([]);
let selectedIds = $state<Set<string>>(new Set());

// Preview sheet states
let isPreviewOpen = $state(false);
let selectedDocument = $state<DocumentDto | null>(null);
let selectedDocumentUrl = $state("");

// Queries & Mutations
const trashQuery = useTrashDocuments();
const restoreMutation = useRestoreDocument();
const purgeMutation = usePurgeDocument();

const trashedDocs = $derived<DocumentDto[]>(trashQuery.data || []);
const isLoading = $derived(trashQuery.isLoading);
const trashMenuActions = $derived([
  {
    id: "refresh",
    label: m.operations_ui_refresh(),
    icon: Refresh,
    disabled: trashQuery.isFetching,
    onSelect: () => trashQuery.refetch(),
  },
  {
    id: "empty-trash",
    label: m.operations_ui_trash_empty_action(),
    icon: Trash,
    destructive: true,
    disabled: trashedDocs.length === 0,
    onSelect: handleEmptyTrash,
  },
]);

// Available categories derived from trashed items
const availableCategories = $derived.by(() => {
  const set = new Set<string>();
  for (const doc of trashedDocs) {
    if (doc.category) set.add(doc.category);
  }
  return Array.from(set);
});

const selectedFileTypes = $derived(
  selectedFilters
    .filter((value) => value.startsWith("type:"))
    .map((value) => value.slice("type:".length)),
);
const selectedCategories = $derived(
  selectedFilters
    .filter((value) => value.startsWith("category:"))
    .map((value) => value.slice("category:".length)),
);

const trashFilterGroups = $derived<SearchableMultiFilterGroup[]>([
  {
    id: "file-types",
    label: m.operations_ui_file_type(),
    options: [
      { value: "type:pdf", label: m.operations_ui_pdf_documents() },
      { value: "type:image", label: m.operations_ui_image() },
      { value: "type:video", label: m.operations_ui_video() },
      { value: "type:audio", label: m.operations_ui_audio() },
    ],
  },
  ...(availableCategories.length > 0
    ? [{
        id: "categories",
        label: m.operations_ui_category(),
        options: availableCategories.map((category) => ({
          value: `category:${category}`,
          label: category,
        })),
      }]
    : []),
]);

// Filtered items
const filteredDocs = $derived.by(() => {
  const q = searchQuery.trim().toLowerCase();
  return trashedDocs.filter((doc) => {
    if (
      selectedCategories.length > 0 &&
      !selectedCategories.includes(doc.category || "")
    ) {
      return false;
    }

    if (selectedFileTypes.length > 0) {
      const mime = doc.mime_type?.toLowerCase() || "";
      const extension = doc.extension?.toLowerCase();
      const matchesType = selectedFileTypes.some((type) => {
        if (type === "pdf") return mime === "application/pdf" || extension === "pdf";
        return mime.startsWith(`${type}/`);
      });
      if (!matchesType) return false;
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
const allFilteredSelected = $derived(
  filteredDocs.length > 0 && filteredDocs.every((doc) => selectedIds.has(doc.id)),
);

const trashGridSource = $derived(
  filteredDocs.map((doc) => ({
    ...doc,
    id: doc.id,
    selected: selectedIds.has(doc.id),
    toggleSelection: toggleSelect,
    sizeLabel: formatSize(doc.file_size_bytes),
  })),
);

const trashColumns: OrbisColumnDef[] = [
  {
    id: "selected",
    header: "",
    width: 42,
    flex: 0,
    readonly: true,
    sort: false,
    cell: ArchiveSelectionCell,
  },
  {
    id: "name",
    header: m.operations_ui_file(),
    width: 320,
    minWidth: 180,
    flex: 2,
    readonly: true,
    cellClass: "font-medium text-foreground",
  },
  {
    id: "category",
    header: m.operations_ui_category(),
    width: 150,
    readonly: true,
    cellClass: "text-muted-foreground",
  },
  {
    id: "sizeLabel",
    header: m.operations_ui_size(),
    width: 100,
    align: "end",
    readonly: true,
    cellClass: "font-mono text-[11px] text-muted-foreground",
  },
  {
    id: "deleted_at",
    header: m.operations_ui_deleted_date(),
    width: 160,
    type: "date",
    readonly: true,
    cellClass: "text-muted-foreground",
  },
  {
    id: "actions",
    header: m.operations_ui_actions(),
    width: 128,
    flex: 0,
    type: "action",
    meta: {
      items: [
        { id: "preview", icon: "Eye", label: m.operations_ui_preview() },
        { id: "restore", icon: "Refresh", label: m.operations_ui_restore() },
        {
          id: "purge",
          icon: "Trash",
          label: m.operations_ui_trash_permanently_delete(),
          variant: "destructive",
        },
      ],
    },
  },
];

function formatSize(bytes?: number | null): string {
  if (!bytes || bytes <= 0) return "0 KB";
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}

function handleGridAction(event: GridActionEvent) {
  const doc = trashedDocs.find((item) => item.id === String(event.row.id));
  if (!doc) return;

  if (event.actionId === "preview") void handlePreview(doc);
  if (event.actionId === "restore") void handleRestoreDoc(doc.id);
  if (event.actionId === "purge") void handlePurgeDoc(doc.id);
}

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
  const visibleIds = filteredDocs.map((doc) => doc.id);
  if (allFilteredSelected) {
    selectedIds = new Set(
      Array.from(selectedIds).filter((id) => !visibleIds.includes(id)),
    );
  } else {
    selectedIds = new Set([...selectedIds, ...visibleIds]);
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

      <SplitActionMenu
        label={m.operations_ui_return_to_archive()}
        menuLabel={m.operations_ui_actions()}
        icon={FolderOpen}
        actions={trashMenuActions}
        onprimary={() => goto(`/${orgId}/settings/archive`)}
      />
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
        <SearchableMultiFilter
          groups={trashFilterGroups}
          bind:selectedValues={selectedFilters}
          placeholder={m.settings_ui_archive_filters()}
          searchPlaceholder={m.settings_ui_archive_filter_search()}
          emptyText={m.settings_ui_archive_no_matching_filters()}
        />

        <Button
          variant="ghost"
          size="sm"
          class="h-8 px-2 text-xs text-muted-foreground"
          disabled={filteredDocs.length === 0}
          onclick={selectAll}
        >
          {allFilteredSelected
            ? m.operations_ui_clear_selection()
            : m.operations_ui_select_all()}
        </Button>

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
        <div class="h-[min(65vh,680px)] min-h-[360px] overflow-hidden rounded-xl border border-border/60 bg-card">
          <OrbisGrid
            columns={trashColumns}
            source={trashGridSource}
            loading={false}
            emptyTitle={m.operations_ui_trash_empty_heading()}
            emptyDescription={m.operations_ui_trash_empty_description()}
            rtl={getLocale() === "ar"}
            readonly
            onaction={handleGridAction}
          />
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
