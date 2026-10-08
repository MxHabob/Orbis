<!--
  @component ArchiveManager
  Enterprise Digital Archive, Folder Explorer & Curriculum AI Knowledge Base.
  Fully rebuilt with Svelte 5 runes, hierarchical folder browsing, and multi-view operations.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { goto } from "$app/navigation";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import {
  Folder,
  FolderOpen,
  NewFolderAlt,
  DocumentPage,
  ArrowUpTray,
  Search,
  Grid,
  Bars3BottomLeft,
  Trash,
  XMark,
} from "@orbis/icons";
import { confirmStore } from "$lib/stores";
import { toast } from "@orbis/ui";
import { getLocale } from "$lib/paraglide/runtime";
import { formatDate } from "$lib/core/i18n";
import {
  OrbisGrid,
  type GridActionEvent,
  type OrbisColumnDef,
} from "@orbis/svelte-grid";
import { api } from "$lib/api";
import {
  useFolders,
  useFolderContents,
  useCurriculumFolders,
  useDocuments,
  useArchiveDocument,
  useDeleteDocument,
  useTrashDocument,
  useTrashDocuments,
  useDeleteFolder,
  resolveDocumentUrl,
  getDocumentSource,
} from "$lib/features/documents/queries";
import FolderCard from "$lib/features/documents/folder-card.svelte";
import FolderBreadcrumbs from "$lib/features/documents/folder-breadcrumbs.svelte";
import FolderCreateModal from "$lib/features/documents/folder-create-modal.svelte";
import ArchiveDocumentCard from "$lib/features/documents/archive-document-card.svelte";
import ArchiveSelectionCell from "$lib/features/documents/archive-selection-cell.svelte";
import ArchivePreviewSheet from "$lib/features/documents/archive-preview-sheet.svelte";
import ArchiveLoading from "$lib/features/documents/archive-loading.svelte";
import SettingsBottomBar from "$lib/shared/components/settings-bottom-bar.svelte";
import SearchableMultiFilter, {
  type SearchableMultiFilterGroup,
} from "$lib/shared/components/searchable-multi-filter.svelte";
import SplitActionMenu from "$lib/shared/components/split-action-menu.svelte";
import type { DocumentDto, FolderDto } from "$lib/api";

let {
  orgId,
  folderId = null,
}: {
  orgId: string;
  folderId?: string | null;
} = $props();

// View and State Management
let viewMode = $state<"grid" | "table">("table");
let searchQuery = $state("");
let selectedFilters = $state<string[]>([]);
let selectedIds = $state<Set<string>>(new Set());

// Modals & Sheets
let isCreateFolderModalOpen = $state(false);
let isPreviewOpen = $state(false);
let selectedDocument = $state<DocumentDto | null>(null);
let selectedDocumentUrl = $state("");

// Drag and Drop Upload
let isDragOverContainer = $state(false);
let fileInput: HTMLInputElement | undefined = $state();

// Queries
const isRoot = $derived(!folderId);

// 1. If in a specific folder, fetch its contents (subfolders + documents)
const folderContentsQuery = useFolderContents(() => folderId || "");

// 2. If at root, fetch root folders
const rootFoldersQuery = useFolders(() => undefined);

// 3. Curriculum folders for quick knowledge base access
const curriculumFoldersQuery = useCurriculumFolders();

// 4. Documents query (if root or general)
const rootDocumentsQuery = useDocuments(() => (isRoot ? undefined : { entityId: folderId || undefined }));

// Active Data Derivatives
const currentFolder = $derived<FolderDto | null>(
  !isRoot && folderContentsQuery.data?.folder ? folderContentsQuery.data.folder : null
);

const subfolders = $derived<FolderDto[]>(
  isRoot
    ? (rootFoldersQuery.data || [])
    : (folderContentsQuery.data?.subfolders || [])
);

const rawDocuments = $derived<DocumentDto[]>(
  isRoot
    ? (rootDocumentsQuery.data?.pages.flatMap((p) => p.items) || [])
    : (folderContentsQuery.data?.documents || [])
);

const isLoading = $derived(
  isRoot
    ? rootFoldersQuery.isLoading || rootDocumentsQuery.isLoading
    : folderContentsQuery.isLoading
);

// Filtering & Search
const filteredFolders = $derived.by(() => {
  const q = searchQuery.trim().toLowerCase();
  if (!q) return subfolders;
  return subfolders.filter((f) => f.name.toLowerCase().includes(q) || f.path.toLowerCase().includes(q));
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

const filteredDocuments = $derived.by(() => {
  const q = searchQuery.trim().toLowerCase();
  return rawDocuments.filter((doc) => {
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

    // Search Query Filter
    if (q) {
      const matchName = doc.name.toLowerCase().includes(q);
      const matchDesc = (doc.description || "").toLowerCase().includes(q);
      const matchTags = (doc.tags || "").toLowerCase().includes(q);
      if (!matchName && !matchDesc && !matchTags) return false;
    }

    return true;
  });
});

const gridSource = $derived(
  filteredDocuments.map((doc) => ({
    ...doc,
    id: doc.id,
    selected: selectedIds.has(doc.id),
    toggleSelection: toggleSelect,
    sizeLabel: formatSize(doc.file_size_bytes),
    created_at_display: formatDate(doc.created_at, "datetime"),
  })),
);

const documentColumns = $derived<OrbisColumnDef[]>([
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
    header: m.settings_ui_archive_file_name(),
    width: 320,
    minWidth: 180,
    flex: 2,
    readonly: true,
    cellClass: "font-medium text-foreground",
  },
  {
    id: "category",
    header: m.settings_ui_archive_category(),
    width: 150,
    readonly: true,
    cellClass: "text-muted-foreground",
  },
  {
    id: "sizeLabel",
    header: m.settings_ui_archive_size(),
    width: 100,
    align: "end",
    readonly: true,
    cellClass: "font-mono text-[11px] text-muted-foreground",
  },
  {
    id: "created_at_display",
    prop: "created_at_display",
    header: m.settings_ui_archive_upload_date(),
    width: 160,
    readonly: true,
    cellClass: "text-muted-foreground",
  },
  {
    id: "actions",
    header: m.settings_ui_archive_actions(),
    width: 160,
    flex: 0,
    type: "action",
    meta: {
      items: [
        { id: "preview", icon: "Eye", label: m.operations_ui_preview() },
        { id: "download", icon: "Download", label: m.operations_ui_download() },
        {
          id: "archive",
          icon: "ArchiveBox",
          label: m.operations_ui_archive(),
          visible: (row) => row.is_archived !== true,
        },
        {
          id: "delete",
          icon: "Trash",
          label: m.common_action_delete(),
          variant: "destructive",
        },
      ],
    },
  },
]);

function formatSize(bytes?: number | null): string {
  if (!bytes || bytes <= 0) return "0 KB";
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}

function handleGridAction(event: GridActionEvent) {
  const doc = rawDocuments.find((item) => item.id === String(event.row.id));
  if (!doc) return;

  switch (event.actionId) {
    case "preview":
      void handlePreview(doc);
      break;
    case "download":
      void downloadDocument(doc);
      break;
    case "archive":
      void handleArchiveDoc(doc.id);
      break;
    case "delete":
      void handleDeleteDoc(doc.id);
      break;
  }
}

async function downloadDocument(doc: DocumentDto) {
  try {
    const url = await resolveDocumentUrl(getDocumentSource(doc));
    const link = document.createElement("a");
    link.href = url;
    link.download = doc.name;
    link.rel = "noreferrer";
    document.body.append(link);
    link.click();
    link.remove();
  } catch (error) {
    console.error("Failed to download archived document:", error);
    toast.error(m.settings_ui_archive_download_error());
  }
}

// Category options derived from existing documents
const availableCategories = $derived.by(() => {
  const set = new Set<string>();
  for (const d of rawDocuments) {
    if (d.category) set.add(d.category);
  }
  return Array.from(set);
});

const archiveFilterGroups = $derived<SearchableMultiFilterGroup[]>([
  {
    id: "file-types",
    label: m.operations_ui_file_type(),
    options: [
      { value: "type:pdf", label: m.settings_ui_archive_pdf_documents() },
      { value: "type:image", label: m.settings_ui_archive_images() },
      { value: "type:video", label: m.settings_ui_archive_videos() },
      { value: "type:audio", label: m.settings_ui_archive_audio_recordings() },
    ],
  },
  ...(availableCategories.length > 0
    ? [{
        id: "categories",
        label: m.settings_ui_archive_category(),
        options: availableCategories.map((category) => ({
          value: `category:${category}`,
          label: category,
        })),
      }]
    : []),
]);

// Trash Query for Badge Counter
const trashDocumentsQuery = useTrashDocuments();
const trashedDocsCount = $derived(trashDocumentsQuery.data?.length || 0);
const archiveMenuActions = $derived([
  {
    id: "new-folder",
    label: m.settings_ui_archive_new_folder(),
    icon: NewFolderAlt,
    onSelect: () => (isCreateFolderModalOpen = true),
  },
  {
    id: "trash",
    label: m.settings_ui_archive_trash_title(),
    icon: Trash,
    badge: trashedDocsCount > 0 ? String(trashedDocsCount) : undefined,
    onSelect: () => goto(`/${orgId}/settings/archive/trash`),
  },
]);
const uploadCategory = $derived(
  selectedCategories.length === 1 ? selectedCategories[0] : "archive",
);

// Mutations
const archiveDocMutation = useArchiveDocument();
const deleteDocMutation = useDeleteDocument();
const trashDocMutation = useTrashDocument();
const deleteFolderMutation = useDeleteFolder();

// Navigation Handlers
function navigateToFolder(targetFolderId?: string | null) {
  if (!targetFolderId) {
    goto(`/${orgId}/settings/archive`);
  } else {
    goto(`/${orgId}/settings/archive/${targetFolderId}`);
  }
}

// Preview Handler
async function handlePreview(doc: DocumentDto) {
  selectedDocument = doc;
  selectedDocumentUrl = await resolveDocumentUrl(getDocumentSource(doc));
  isPreviewOpen = true;
}

// Delete / Trash Document Handler
async function handleDeleteDoc(id: string) {
  const confirmed = await confirmStore.prompt({
    title: m.settings_ui_archive_trash_title(),
    description: m.settings_ui_archive_trash_description(),
    confirmText: m.settings_ui_archive_move_to_trash(),
    variant: "destructive",
  });
  if (confirmed) {
    try {
      await trashDocMutation.mutateAsync(id);
      toast.success(m.settings_ui_archive_document_trashed());
      selectedIds.delete(id);
      selectedIds = new Set(selectedIds);
      trashDocumentsQuery.refetch();
      if (!isRoot) folderContentsQuery.refetch();
    } catch (e: any) {
      toast.error(e?.message || m.settings_ui_archive_delete_file_error());
    }
  }
}

// Archive Document Handler
async function handleArchiveDoc(id: string) {
  try {
    await archiveDocMutation.mutateAsync({ id, reason: m.settings_ui_archive_manual_reason() });
    toast.success(m.settings_ui_archive_document_archived());
    if (!isRoot) folderContentsQuery.refetch();
  } catch (e: any) {
    toast.error(e?.message || m.settings_ui_archive_archive_file_error());
  }
}

// Delete Folder Handler
async function handleDeleteFolder(f: FolderDto) {
  const confirmed = await confirmStore.prompt({
    title: m.settings_ui_archive_delete_folder_title({ name: f.name }),
    description: m.settings_ui_archive_delete_folder_description(),
    confirmText: m.settings_ui_archive_delete_folder(),
    variant: "destructive",
  });
  if (confirmed) {
    try {
      await deleteFolderMutation.mutateAsync(f.id);
      toast.success(m.settings_ui_archive_folder_deleted({ name: f.name }));
      if (isRoot) {
        rootFoldersQuery.refetch();
      } else {
        folderContentsQuery.refetch();
      }
    } catch (e: any) {
      toast.error(e?.message || m.settings_ui_archive_delete_folder_error());
    }
  }
}

// Multi-select Handlers
function toggleSelect(id: string, isSelected: boolean) {
  if (isSelected) {
    selectedIds.add(id);
  } else {
    selectedIds.delete(id);
  }
  selectedIds = new Set(selectedIds);
}

function selectAllDocuments() {
  if (selectedIds.size === filteredDocuments.length) {
    selectedIds = new Set();
  } else {
    selectedIds = new Set(filteredDocuments.map((d) => d.id));
  }
}

async function handleBulkDelete() {
  const count = selectedIds.size;
  if (count === 0) return;
  const confirmed = await confirmStore.prompt({
    title: m.settings_ui_archive_delete_files_title({ count }),
    description: m.settings_ui_archive_delete_files_description({ count }),
    confirmText: m.settings_ui_archive_delete_all(),
    variant: "destructive",
  });
  if (!confirmed) return;

  try {
    await Promise.all(Array.from(selectedIds).map((id) => deleteDocMutation.mutateAsync(id)));
    toast.success(m.settings_ui_archive_files_deleted({ count }));
    selectedIds = new Set();
    if (!isRoot) folderContentsQuery.refetch();
  } catch (e: any) {
    toast.error(m.settings_ui_archive_bulk_delete_error());
  }
}

// Helper for bounded concurrent asynchronous tasks
async function runWithConcurrency<T, R>(
  items: T[],
  limit: number,
  fn: (item: T, index: number) => Promise<R>
): Promise<R[]> {
  const results: R[] = [];
  let index = 0;
  const workers = Array.from({ length: Math.min(limit, items.length) }, async () => {
    while (index < items.length) {
      const currentIndex = index++;
      results[currentIndex] = await fn(items[currentIndex], currentIndex);
    }
  });
  await Promise.all(workers);
  return results;
}

// Drag and Drop & Upload Files with Live Progress Toast & Concurrency Control
async function handleFilesUpload(fileList: FileList | File[]) {
  const files = Array.from(fileList || []);
  if (files.length === 0) return;

  const total = files.length;
  let completedCount = 0;
  let successCount = 0;
  let failCount = 0;

  const toastHandle = toast(m.settings_ui_archive_upload_files(), {
    variant: "progress",
    progress: 0,
    description: `0 / ${total}`,
    duration: Number.POSITIVE_INFINITY,
  });

  async function uploadSingle(file: File) {
    try {
      await api.uploadFile({
        file,
        name: file.name,
        folder_id: folderId ?? undefined,
        category: uploadCategory,
        visibility: "internal",
      });
      successCount++;
    } catch (e) {
      console.error(`Failed to upload ${file.name}:`, e);
      failCount++;
    } finally {
      completedCount++;
      const progressPercent = Math.round((completedCount / total) * 100);
      toastHandle.update({
        title: m.settings_ui_archive_upload_files(),
        description: m.settings_ui_archive_upload_progress({
          completed: completedCount,
          total,
          filename: file.name,
        }),
        progress: progressPercent,
        variant: "progress",
      });
    }
  }

  // Upload with bounded concurrency (2 parallel uploads) to prevent memory & IPC bottlenecks
  await runWithConcurrency(files, 2, uploadSingle);

  if (successCount > 0) {
    toastHandle.update({
      title: m.settings_ui_archive_files_uploaded({ uploaded: successCount, total }),
      description: failCount > 0
        ? m.settings_ui_archive_upload_fail_count({ count: failCount })
        : undefined,
      progress: 100,
      variant: "success",
      duration: 4000,
    });

    if (isRoot) {
      rootDocumentsQuery.refetch();
      rootFoldersQuery.refetch();
    } else {
      folderContentsQuery.refetch();
    }
  } else {
    toastHandle.update({
      title: m.settings_ui_archive_upload_error(),
      description: undefined,
      variant: "error",
      duration: 5000,
    });
  }
}

function onFileInputChange(e: Event) {
  const target = e.target as HTMLInputElement;
  if (target.files && target.files.length > 0) {
    const files = Array.from(target.files);
    target.value = "";
    handleFilesUpload(files);
  }
}

function onContainerDragOver(e: DragEvent) {
  e.preventDefault();
  isDragOverContainer = true;
}

function onContainerDragLeave() {
  isDragOverContainer = false;
}

function onContainerDrop(e: DragEvent) {
  e.preventDefault();
  isDragOverContainer = false;
  if (e.dataTransfer?.files && e.dataTransfer.files.length > 0) {
    const files = Array.from(e.dataTransfer.files);
    handleFilesUpload(files);
  }
}
</script>

<input
  bind:this={fileInput}
  type="file"
  multiple
  class="hidden"
  onchange={onFileInputChange}
/>

<!-- Main File Explorer Container -->
<div
  role="region"
  aria-label={m.settings_ui_archive_explorer()}
  ondragover={onContainerDragOver}
  ondragleave={onContainerDragLeave}
  ondrop={onContainerDrop}
  class="relative flex flex-col h-full w-full overflow-hidden bg-background text-foreground text-start select-none
    {isDragOverContainer ? 'ring-2 ring-primary ring-inset bg-primary/5' : ''}"
>
  <!-- Top Navigation & Action Header -->
  <header class="shrink-0 p-4 border-b border-border/50 bg-card/40 backdrop-blur-sm space-y-3">
    <!-- Breadcrumbs & Quick Creation Actions -->
    <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-3">
      <FolderBreadcrumbs
        {orgId}
        {currentFolder}
        onnavigate={navigateToFolder}
      />

      <div class="flex items-center flex-wrap gap-2 w-full sm:w-auto justify-end">
        <SplitActionMenu
          label={m.settings_ui_archive_upload_files()}
          menuLabel={m.ui_copy_more_options_2179e961()}
          icon={ArrowUpTray}
          actions={archiveMenuActions}
          onprimary={() => fileInput?.click()}
        />
      </div>
    </div>

    <!-- Search & Filter Toolbar -->
    <div class="flex flex-col md:flex-row items-stretch md:items-center justify-between gap-2.5 pt-1">
      <!-- Search Input -->
      <div class="relative flex-1 max-w-md">
        <Search class="w-4 h-4 absolute inset-s-3 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none" />
        <Input
          bind:value={searchQuery}
          placeholder={m.settings_ui_archive_search_placeholder()}
          class="w-full ps-9 pe-8 text-xs bg-background/80 h-8"
        />
        {#if searchQuery}
          <button
            type="button"
            onclick={() => searchQuery = ""}
            class="absolute inset-e-2.5 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground"
          >
            <XMark class="w-3.5 h-3.5" />
          </button>
        {/if}
      </div>

      <!-- Filter Controls & View Switcher -->
      <div class="flex items-center flex-wrap gap-2 text-xs">
        <SearchableMultiFilter
          groups={archiveFilterGroups}
          bind:selectedValues={selectedFilters}
          placeholder={m.settings_ui_archive_filters()}
          searchPlaceholder={m.settings_ui_archive_filter_search()}
          emptyText={m.settings_ui_archive_no_matching_filters()}
        />

        <!-- View Switcher -->
        <div class="flex items-center p-0.5 rounded-lg border border-border/60 bg-muted/40">
          <Button
            variant="ghost"
            size="icon"
            onclick={() => viewMode = "grid"}
            class="h-7 w-7 rounded-md transition-all {viewMode === 'grid' ? 'bg-background text-foreground shadow-xs' : 'text-muted-foreground hover:text-foreground'}"
            title={m.settings_ui_archive_grid_view()}
          >
            <Grid class="w-3.5 h-3.5" />
          </Button>
          <Button
            variant="ghost"
            size="icon"
            onclick={() => viewMode = "table"}
            class="h-7 w-7 rounded-md transition-all {viewMode === 'table' ? 'bg-background text-foreground shadow-xs' : 'text-muted-foreground hover:text-foreground'}"
            title={m.settings_ui_archive_list_view()}
          >
            <Bars3BottomLeft class="w-3.5 h-3.5" />
          </Button>
        </div>
      </div>
    </div>
  </header>

  <!-- Main Content Viewport -->
  <main class="flex-1 overflow-y-auto p-6 space-y-6 scrollbar-hide">
    {#if isLoading}
      <ArchiveLoading />
    {:else if filteredFolders.length === 0 && filteredDocuments.length === 0}
      <!-- Empty State -->
      <div class="flex flex-col items-center justify-center py-20 text-center text-muted-foreground">
        <div class="w-16 h-16 rounded-2xl bg-muted/50 flex items-center justify-center mb-4 text-muted-foreground/60 shadow-inner">
          <FolderOpen class="w-8 h-8" />
        </div>
        <h3 class="font-bold text-base text-foreground">{m.settings_ui_archive_folder_empty()}</h3>
        <p class="text-xs text-muted-foreground max-w-sm mt-1">
          {m.settings_ui_archive_empty_description()}
        </p>
        <div class="flex items-center gap-2 mt-5">
          <Button variant="outline" size="sm" onclick={() => isCreateFolderModalOpen = true}>
            <NewFolderAlt class="w-3.5 h-3.5 me-1.5" />
            <span>{m.settings_ui_archive_create_folder()}</span>
          </Button>
          <Button size="sm" onclick={() => fileInput?.click()}>
            <ArrowUpTray class="w-3.5 h-3.5 me-1.5" />
            <span>{m.settings_ui_archive_upload_files()}</span>
          </Button>
        </div>
      </div>
    {:else}
      <!-- 1. Folders Section -->
      {#if filteredFolders.length > 0}
        <section class="space-y-3">
          <div class="flex items-center justify-between">
            <h4 class="text-xs font-bold uppercase tracking-wider text-muted-foreground flex items-center gap-1.5">
              <Folder class="w-3.5 h-3.5" />
              <span>{m.settings_ui_archive_folders_count({ count: filteredFolders.length })}</span>
            </h4>
          </div>

          <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-3.5">
            {#each filteredFolders as folder (folder.id)}
              <FolderCard
                {folder}
                onopen={(f) => navigateToFolder(f.id)}
                ondelete={handleDeleteFolder}
              />
            {/each}
          </div>
        </section>
      {/if}

      <!-- 2. Documents Section -->
      {#if filteredDocuments.length > 0}
        <section class="space-y-3">
          <div class="flex items-center justify-between">
            <h4 class="text-xs font-bold uppercase tracking-wider text-muted-foreground flex items-center gap-1.5">
              <DocumentPage class="w-3.5 h-3.5 text-primary" />
              <span>{m.settings_ui_archive_documents_count({ count: filteredDocuments.length })}</span>
            </h4>

            {#if viewMode === "table"}
              <button
                type="button"
                onclick={selectAllDocuments}
                class="text-xs text-primary hover:underline font-medium"
              >
                {selectedIds.size === filteredDocuments.length ? m.settings_ui_archive_deselect_all() : m.settings_ui_archive_select_all()}
              </button>
            {/if}
          </div>

          {#if viewMode === "grid"}
            <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-3.5">
              {#each filteredDocuments as doc (doc.id)}
                <ArchiveDocumentCard
                  document={doc}
                  selected={selectedIds.has(doc.id)}
                  onpreview={handlePreview}
                  ondelete={handleDeleteDoc}
                  onarchive={handleArchiveDoc}
                  ontoggle={toggleSelect}
                />
              {/each}
            </div>
          {:else}
            <!-- Table View -->
            <div class="h-[min(65vh,680px)] min-h-[360px] overflow-hidden rounded-xl border border-border/60 bg-card">
              <OrbisGrid
                columns={documentColumns}
                source={gridSource}
                emptyTitle={m.settings_ui_archive_folder_empty()}
                emptyDescription={m.settings_ui_archive_empty_description()}
                rtl={getLocale() === "ar"}
                readonly
                onaction={handleGridAction}
              />
            </div>
          {/if}
        </section>
      {/if}
    {/if}
  </main>
</div>

<!-- Modals -->
<FolderCreateModal
  bind:open={isCreateFolderModalOpen}
  parentId={folderId}
  oncreated={(f) => {
    if (isRoot) {
      rootFoldersQuery.refetch();
    } else {
      folderContentsQuery.refetch();
    }
  }}
/>

<ArchivePreviewSheet
  bind:open={isPreviewOpen}
  document={selectedDocument}
  url={selectedDocumentUrl}
  ondelete={handleDeleteDoc}
  onarchive={handleArchiveDoc}
/>

<!-- Floating Selection Bottom Bar -->
<SettingsBottomBar
  visible={selectedIds.size > 0}
  isDirty={selectedIds.size > 0}
  message={m.settings_ui_archive_selected_items({ count: selectedIds.size })}
  saveLabel={m.settings_ui_archive_delete_selected()}
  resetLabel={m.settings_ui_archive_clear_selection()}
  saveVariant="destructive"
  statusVariant="rose"
  showReset={true}
  onsave={handleBulkDelete}
  onreset={() => (selectedIds = new Set())}
/>
