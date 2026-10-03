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
  Sparkles,
  AcademicCap,
  Trash,
  ArchiveBox,
  Refresh,
  XMark,
  Check,
  CheckCircle,
} from "@orbis/icons";
import { confirmStore } from "$lib/stores";
import { toast } from "@orbis/ui";
import {
  useFolders,
  useFolderContents,
  useCurriculumFolders,
  useDocuments,
  useUploadFile,
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

// Trash Query for Badge Counter
const trashDocumentsQuery = useTrashDocuments();
const trashedDocsCount = $derived(trashDocumentsQuery.data?.length || 0);

// Mutations
const uploadFileMutation = useUploadFile();
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
    title: "نقل إلى سلة المحذوفات",
    description: "هل تود نقل هذا الملف إلى سلة المحذوفات؟ يمكنك استعادته لاحقاً.",
    confirmText: "نقل للسلة",
    variant: "destructive",
  });
  if (confirmed) {
    try {
      await trashDocMutation.mutateAsync(id);
      toast.success("تم نقل الملف إلى سلة المحذوفات");
      selectedIds.delete(id);
      selectedIds = new Set(selectedIds);
      trashDocumentsQuery.refetch();
      if (!isRoot) folderContentsQuery.refetch();
    } catch (e: any) {
      toast.error(e?.message || "فشل في حذف الملف");
    }
  }
}

// Archive Document Handler
async function handleArchiveDoc(id: string) {
  try {
    await archiveDocMutation.mutateAsync({ id, reason: "أرشفة يدوية من المستكشف" });
    toast.success("تم أرشفة الملف بنجاح");
    if (!isRoot) folderContentsQuery.refetch();
  } catch (e: any) {
    toast.error(e?.message || "فشل في أرشفة الملف");
  }
}

// Delete Folder Handler
async function handleDeleteFolder(f: FolderDto) {
  const confirmed = await confirmStore.prompt({
    title: `حذف المجلد "${f.name}"`,
    description: "سيتم حذف المجلد وكافة المجلدات الفرعية. هل تود الاستمرار؟",
    confirmText: "حذف المجلد",
    variant: "destructive",
  });
  if (confirmed) {
    try {
      await deleteFolderMutation.mutateAsync(f.id);
      toast.success(`تم حذف المجلد "${f.name}"`);
      if (isRoot) {
        rootFoldersQuery.refetch();
      } else {
        folderContentsQuery.refetch();
      }
    } catch (e: any) {
      toast.error(e?.message || "فشل في حذف المجلد");
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
    title: `حذف ${count} ملفاً`,
    description: `سيتم حذف ${count} ملفاً محدداً بشكل دائم. هل تود المتابعة؟`,
    confirmText: "حذف الكل",
    variant: "destructive",
  });
  if (!confirmed) return;

  try {
    await Promise.all(Array.from(selectedIds).map((id) => deleteDocMutation.mutateAsync(id)));
    toast.success(`تم حذف ${count} ملفاً بنجاح`);
    selectedIds = new Set();
    if (!isRoot) folderContentsQuery.refetch();
  } catch (e: any) {
    toast.error("حدث خطأ أثناء الحذف الجماعي");
  }
}

// Drag and Drop & Upload Files
async function handleFilesUpload(files: FileList | File[]) {
  if (!files || files.length === 0) return;

  const total = files.length;
  let successCount = 0;

  for (let i = 0; i < files.length; i++) {
    const file = files[i];
    try {
      await uploadFileMutation.mutateAsync({
        file,
        name: file.name,
        folder_id: folderId ?? undefined,
        category: selectedCategory !== "all" ? selectedCategory : "archive",
        visibility: "internal",
      });
      successCount++;
    } catch (e) {
      console.error(`Failed to upload ${file.name}:`, e);
    }
  }

  if (successCount > 0) {
    toast.success(`تم رفع ${successCount} من أصل ${total} ملف بنجاح`);
    if (isRoot) {
      rootDocumentsQuery.refetch();
    } else {
      folderContentsQuery.refetch();
    }
  } else {
    toast.error("فشل رفع الملفات");
  }
}

function onFileInputChange(e: Event) {
  const target = e.target as HTMLInputElement;
  if (target.files) {
    handleFilesUpload(target.files);
    target.value = "";
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
  if (e.dataTransfer?.files) {
    handleFilesUpload(e.dataTransfer.files);
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
  aria-label="مستكشف الملفات والمستندات"
  ondragover={onContainerDragOver}
  ondragleave={onContainerDragLeave}
  ondrop={onContainerDrop}
  class="relative flex flex-col h-full w-full overflow-hidden bg-background text-foreground text-start select-none
    {isDragOverContainer ? 'ring-2 ring-primary ring-inset bg-primary/5' : ''}"
>
  <!-- Top Navigation & Action Header -->
  <header class="shrink-0 p-5 border-b border-border/50 bg-card/40 backdrop-blur-sm space-y-4">
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
          class="gap-1.5 text-xs shadow-sm hover:border-rose-500/40 hover:text-rose-600 dark:hover:text-rose-400"
          title="سلة المحذوفات"
        >
          <Trash class="w-3.5 h-3.5 text-rose-500" />
          <span>سلة المحذوفات</span>
          {#if trashedDocsCount > 0}
            <span class="rounded-full bg-rose-500/15 px-1.5 py-0.5 text-[10px] font-bold text-rose-600 dark:text-rose-400">
              {trashedDocsCount}
            </span>
          {/if}
        </Button>

        <!-- New Folder Button -->
        <Button
          variant="outline"
          size="sm"
          onclick={() => isCreateFolderModalOpen = true}
          class="gap-1.5 text-xs shadow-sm"
        >
          <NewFolderAlt class="w-3.5 h-3.5 text-amber-500" />
          <span>مجلد جديد</span>
        </Button>

        <!-- Upload Files Button -->
        <Button
          size="sm"
          onclick={() => fileInput?.click()}
          class="gap-1.5 text-xs shadow-sm"
        >
          <ArrowUpTray class="w-3.5 h-3.5" />
          <span>رفع ملفات</span>
        </Button>
      </div>
    </div>

    <!-- Search & Filter Toolbar -->
    <div class="flex flex-col md:flex-row items-stretch md:items-center justify-between gap-3">
      <!-- Search Input -->
      <div class="relative flex-1 max-w-md">
        <Search class="w-4 h-4 absolute right-3 top-2.5 text-muted-foreground pointer-events-none" />
        <Input
          bind:value={searchQuery}
          placeholder="ابحث في المستندات والمجلدات..."
          class="w-full pr-9 text-xs bg-background/80 h-9"
        />
        {#if searchQuery}
          <button
            type="button"
            onclick={() => searchQuery = ""}
            class="absolute left-2.5 top-2.5 text-muted-foreground hover:text-foreground"
          >
            <XMark class="w-3.5 h-3.5" />
          </button>
        {/if}
      </div>

      <!-- Filter Controls & View Switcher -->
      <div class="flex items-center flex-wrap gap-2 text-xs">
        <!-- Asset Type Filter -->
        <select
          bind:value={selectedAssetType}
          class="h-9 px-2.5 rounded-lg border border-border/60 bg-background text-xs text-foreground focus:outline-none focus:ring-1 focus:ring-primary cursor-pointer"
        >
          <option value="all">جميع أنواع الملفات</option>
          <option value="pdf">مستندات PDF</option>
          <option value="image">الصور</option>
          <option value="video">الفيديوهات</option>
          <option value="audio">التسجيلات الصوتية</option>
        </select>

        <!-- Category Filter -->
        {#if availableCategories.length > 0}
          <select
            bind:value={selectedCategory}
            class="h-9 px-2.5 rounded-lg border border-border/60 bg-background text-xs text-foreground focus:outline-none focus:ring-1 focus:ring-primary cursor-pointer"
          >
            <option value="all">كافة التصنيفات</option>
            {#each availableCategories as cat}
              <option value={cat}>{cat}</option>
            {/each}
          </select>
        {/if}

        <!-- View Switcher -->
        <div class="flex items-center p-0.5 rounded-lg border border-border/60 bg-muted/40">
          <button
            type="button"
            onclick={() => viewMode = "grid"}
            class="p-1.5 rounded-md transition-all {viewMode === 'grid' ? 'bg-background text-foreground shadow-xs' : 'text-muted-foreground hover:text-foreground'}"
            title="عرض الشبكة"
          >
            <Grid class="w-3.5 h-3.5" />
          </button>
          <button
            type="button"
            onclick={() => viewMode = "table"}
            class="p-1.5 rounded-md transition-all {viewMode === 'table' ? 'bg-background text-foreground shadow-xs' : 'text-muted-foreground hover:text-foreground'}"
            title="عرض القائمة"
          >
            <Bars3BottomLeft class="w-3.5 h-3.5" />
          </button>
        </div>
      </div>
    </div>
  </header>

  <!-- Multi-Select Bulk Actions Bar (Sticky Float) -->
  {#if selectedIds.size > 0}
    <div class="px-5 py-2.5 bg-primary/10 border-b border-primary/20 flex items-center justify-between gap-3 text-xs animate-in slide-in-from-top-2 duration-150">
      <div class="flex items-center gap-2">
        <span class="font-semibold text-primary">تم تحديد {selectedIds.size} عنصر</span>
        <button
          type="button"
          onclick={() => selectedIds = new Set()}
          class="text-muted-foreground hover:text-foreground underline text-[11px]"
        >
          إلغاء التحديد
        </button>
      </div>

      <div class="flex items-center gap-2">
        <Button variant="destructive" size="xs" onclick={handleBulkDelete} class="gap-1.5">
          <Trash class="w-3 h-3" />
          <span>حذف المحدد</span>
        </Button>
      </div>
    </div>
  {/if}

  <!-- Main Content Viewport -->
  <main class="flex-1 overflow-y-auto p-6 space-y-6 scrollbar-thin">
    {#if isLoading}
      <ArchiveLoading />
    {:else if filteredFolders.length === 0 && filteredDocuments.length === 0}
      <!-- Empty State -->
      <div class="flex flex-col items-center justify-center py-20 text-center text-muted-foreground">
        <div class="w-16 h-16 rounded-2xl bg-muted/50 flex items-center justify-center mb-4 text-muted-foreground/60 shadow-inner">
          <FolderOpen class="w-8 h-8" />
        </div>
        <h3 class="font-bold text-base text-foreground">المجلد فارغ</h3>
        <p class="text-xs text-muted-foreground max-w-sm mt-1">
          لم يتم العثور على أي ملفات أو مجلدات هنا. اسحب الملفات وأفلتها هنا أو انقر على "رفع ملفات".
        </p>
        <div class="flex items-center gap-2 mt-5">
          <Button variant="outline" size="sm" onclick={() => isCreateFolderModalOpen = true}>
            <NewFolderAlt class="w-3.5 h-3.5 me-1.5 text-amber-500" />
            <span>إنشاء مجلد</span>
          </Button>
          <Button size="sm" onclick={() => fileInput?.click()}>
            <ArrowUpTray class="w-3.5 h-3.5 me-1.5" />
            <span>رفع ملفات</span>
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
              <span>المجلدات ({filteredFolders.length})</span>
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
              <span>الملفات والمستندات ({filteredDocuments.length})</span>
            </h4>

            {#if viewMode === "table"}
              <button
                type="button"
                onclick={selectAllDocuments}
                class="text-xs text-primary hover:underline font-medium"
              >
                {selectedIds.size === filteredDocuments.length ? "إلغاء تحديد الكل" : "تحديد الكل"}
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
                <span>اسم الملف</span>
                <span>التصنيف</span>
                <span>الحجم</span>
                <span>تاريخ الرفع</span>
                <span class="text-end">الإجراءات</span>
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
