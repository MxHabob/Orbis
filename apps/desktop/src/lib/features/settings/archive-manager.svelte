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
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
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
import ArchiveDocumentRow from "$lib/features/documents/archive-document-row.svelte";
import ArchivePreviewSheet from "$lib/features/documents/archive-preview-sheet.svelte";
import ArchiveLoading from "$lib/features/documents/archive-loading.svelte";
import SettingsBottomBar from "$lib/shared/components/settings-bottom-bar.svelte";
import type { DocumentDto, FolderDto } from "$lib/api";

let {
  orgId,
  folderId = null,
}: {
  orgId: string;
  folderId?: string | null;
} = $props();

// View and State Management
let viewMode = $state<"grid" | "table">("grid");
let searchQuery = $state("");
let selectedAssetType = $state<string>("all");
let selectedCategory = $state<string>("all");
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

const filteredDocuments = $derived.by(() => {
  const q = searchQuery.trim().toLowerCase();
  return rawDocuments.filter((doc) => {
    // Category Filter
    if (selectedCategory !== "all" && doc.category !== selectedCategory) {
      return false;
    }

    // Asset Type Filter
    if (selectedAssetType !== "all") {
      const mime = doc.mime_type?.toLowerCase() || "";
      if (selectedAssetType === "image" && !mime.startsWith("image/")) return false;
      if (selectedAssetType === "video" && !mime.startsWith("video/")) return false;
      if (selectedAssetType === "audio" && !mime.startsWith("audio/")) return false;
      if (selectedAssetType === "pdf" && mime !== "application/pdf" && doc.extension?.toLowerCase() !== "pdf") return false;
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

// Category options derived from existing documents
const availableCategories = $derived.by(() => {
  const set = new Set<string>();
  for (const d of rawDocuments) {
    if (d.category) set.add(d.category);
  }
  return Array.from(set);
});

// Filter Labels for Selects
const assetTypeLabel = $derived.by(() => {
  switch (selectedAssetType) {
    case "pdf": return m.settings_ui_archive_pdf_documents();
    case "image": return m.settings_ui_archive_images();
    case "video": return m.settings_ui_archive_videos();
    case "audio": return m.settings_ui_archive_audio_recordings();
    default: return m.settings_ui_archive_all_file_types();
  }
});

const categoryLabel = $derived(
  selectedCategory === "all" ? m.settings_ui_archive_all_categories() : selectedCategory
);

// Trash Query for Badge Counter
const trashDocumentsQuery = useTrashDocuments();
const trashedDocsCount = $derived(trashDocumentsQuery.data?.length || 0);

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
    await archiveDocMutation.mutateAsync({ id, reason: "أرشفة يدوية من المستكشف" });
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
        category: selectedCategory !== "all" ? selectedCategory : "archive",
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
        description: `تم رفع ${completedCount} من أصل ${total} (${file.name})`,
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
      description: failCount > 0 ? `تعذر رفع ${failCount} ملفات` : undefined,
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
        <!-- Trash Bin Button -->
        <Button
          variant="outline"
          size="sm"
          onclick={() => goto(`/${orgId}/settings/archive/trash`)}
          class="gap-1.5 text-xs shadow-xs hover:border-rose-500/40 hover:text-rose-600 dark:hover:text-rose-400"
          title={m.settings_ui_archive_trash_title()}
        >
          <Trash class="w-3.5 h-3.5 text-rose-500" />
          <span>{m.settings_ui_archive_trash_title()}</span>
          {#if trashedDocsCount > 0}
            <span class="rounded-full bg-rose-500/15 px-1.5 py-0.2 text-[10px] font-bold text-rose-600 dark:text-rose-400">
              {trashedDocsCount}
            </span>
          {/if}
        </Button>

        <!-- New Folder Button -->
        <Button
          variant="outline"
          size="sm"
          onclick={() => isCreateFolderModalOpen = true}
          class="gap-1.5 text-xs shadow-xs"
        >
          <NewFolderAlt class="w-3.5 h-3.5 text-amber-500" />
          <span>{m.settings_ui_archive_new_folder()}</span>
        </Button>

        <!-- Upload Files Button -->
        <Button
          size="sm"
          onclick={() => fileInput?.click()}
          class="gap-1.5 text-xs shadow-xs"
        >
          <ArrowUpTray class="w-3.5 h-3.5" />
          <span>{m.settings_ui_archive_upload_files()}</span>
        </Button>
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
        <!-- Asset Type Filter -->
        <Select type="single" bind:value={selectedAssetType}>
          <SelectTrigger class="h-8 min-w-32 text-xs">
            <span class="truncate">{assetTypeLabel}</span>
          </SelectTrigger>
          <SelectContent align="end">
            <SelectItem value="all">{m.settings_ui_archive_all_file_types()}</SelectItem>
            <SelectItem value="pdf">{m.settings_ui_archive_pdf_documents()}</SelectItem>
            <SelectItem value="image">{m.settings_ui_archive_images()}</SelectItem>
            <SelectItem value="video">{m.settings_ui_archive_videos()}</SelectItem>
            <SelectItem value="audio">{m.settings_ui_archive_audio_recordings()}</SelectItem>
          </SelectContent>
        </Select>

        <!-- Category Filter -->
        {#if availableCategories.length > 0}
          <Select type="single" bind:value={selectedCategory}>
            <SelectTrigger class="h-8 min-w-32 text-xs">
              <span class="truncate">{categoryLabel}</span>
            </SelectTrigger>
            <SelectContent align="end">
              <SelectItem value="all">{m.settings_ui_archive_all_categories()}</SelectItem>
              {#each availableCategories as cat}
                <SelectItem value={cat}>{cat}</SelectItem>
              {/each}
            </SelectContent>
          </Select>
        {/if}

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
            <NewFolderAlt class="w-3.5 h-3.5 me-1.5 text-amber-500" />
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
              <Folder class="w-3.5 h-3.5 text-amber-500" />
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
            <div class="border border-border/50 rounded-xl overflow-hidden bg-card/60 divide-y divide-border/40">
              <div class="grid grid-cols-[auto_minmax(0,2.5fr)_minmax(100px,1fr)_minmax(80px,0.8fr)_minmax(100px,1fr)_auto] items-center gap-3 px-3 py-2 bg-muted/40 text-[11px] font-semibold text-muted-foreground uppercase">
                <span class="w-4"></span>
                <span>{m.settings_ui_archive_file_name()}</span>
                <span>{m.settings_ui_archive_category()}</span>
                <span>{m.settings_ui_archive_size()}</span>
                <span>{m.settings_ui_archive_upload_date()}</span>
                <span class="text-end">{m.settings_ui_archive_actions()}</span>
              </div>
              {#each filteredDocuments as doc (doc.id)}
                <ArchiveDocumentRow
                  document={doc}
                  selected={selectedIds.has(doc.id)}
                  onpreview={handlePreview}
                  ondelete={handleDeleteDoc}
                  onarchive={handleArchiveDoc}
                  ontoggle={toggleSelect}
                />
              {/each}
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
