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
    toast.success("تم استعادة الملف بنجاح");
    selectedIds.delete(id);
    selectedIds = new Set(selectedIds);
    trashQuery.refetch();
  } catch (e: any) {
    toast.error(e?.message || "فشل في استعادة الملف");
  }
}

// Purge single document
async function handlePurgeDoc(id: string) {
  const confirmed = await confirmStore.prompt({
    title: "حذف نهائي",
    description: "هل أنت متأكد من حذف هذا الملف نهائياً من القرص وقاعدة البيانات؟ لا يمكن التراجع عن هذا الإجراء.",
    confirmText: "حذف نهائي",
    variant: "destructive",
  });
  if (confirmed) {
    try {
      await purgeMutation.mutateAsync(id);
      toast.success("تم حذف الملف نهائياً");
      selectedIds.delete(id);
      selectedIds = new Set(selectedIds);
      trashQuery.refetch();
    } catch (e: any) {
      toast.error(e?.message || "فشل في حذف الملف");
    }
  }
}

// Empty entire trash
async function handleEmptyTrash() {
  if (trashedDocs.length === 0) return;

  const confirmed = await confirmStore.prompt({
    title: "تفريغ سلة المحذوفات بالكامل",
    description: `سيتم حذف جميع الملفات المحذوفة (${trashedDocs.length} ملف) نهائياً من النظام. هذا الإجراء لا رجعة فيه.`,
    confirmText: "تفريغ السلة نهائياً",
    variant: "destructive",
  });

  if (confirmed) {
    try {
      const promises = trashedDocs.map((doc) => purgeMutation.mutateAsync(doc.id));
      await Promise.all(promises);
      toast.success("تم تفريغ سلة المحذوفات بالكامل");
      selectedIds = new Set();
      trashQuery.refetch();
    } catch (e: any) {
      toast.error(e?.message || "حدث خطأ أثناء تفريغ سلة المحذوفات");
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
    toast.success(`تم استعادة ${count} ملفات بنجاح`);
    selectedIds = new Set();
    trashQuery.refetch();
  } catch (e: any) {
    toast.error(e?.message || "فشل في استعادة الملفات المحددة");
  }
}

// Batch purge
async function handleBatchPurge() {
  if (selectedIds.size === 0) return;
  const count = selectedIds.size;
  const confirmed = await confirmStore.prompt({
    title: `حذف نهائي لـ ${count} ملفات`,
    description: "هل أنت متأكد من حذف الملفات المحددة نهائياً؟ لا يمكن التراجع عن هذه العملية.",
    confirmText: "حذف نهائي",
    variant: "destructive",
  });

  if (confirmed) {
    try {
      const promises = Array.from(selectedIds).map((id) => purgeMutation.mutateAsync(id));
      await Promise.all(promises);
      toast.success(`تم حذف ${count} ملفات نهائياً`);
      selectedIds = new Set();
      trashQuery.refetch();
    } catch (e: any) {
      toast.error(e?.message || "فشل في حذف الملفات المحددة");
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
            <span>الأرشيف الرقمي</span>
          </button>
          <span>/</span>
          <span class="text-foreground font-medium flex items-center gap-1 text-rose-500">
            <Trash class="w-3.5 h-3.5" />
            <span>سلة المحذوفات</span>
          </span>
        </nav>

        <div class="flex items-center gap-2">
          <h1 class="text-xl font-bold text-foreground">سلة المحذوفات</h1>
          <span class="rounded-full bg-rose-500/10 text-rose-600 dark:text-rose-400 px-2.5 py-0.5 text-xs font-semibold">
            {trashedDocs.length} ملف
          </span>
        </div>
      </div>

      <!-- Header Action Buttons -->
      <div class="flex items-center gap-2">
        <Button
          variant="outline"
          size="sm"
          onclick={() => trashQuery.refetch()}
          title="تحديث البيانات"
          class="h-9"
        >
          <Refresh class="w-4 h-4 {trashQuery.isFetching ? 'animate-spin' : ''}" />
          <span class="hidden sm:inline ms-1.5">تحديث</span>
        </Button>

        {#if trashedDocs.length > 0}
          <Button
            variant="destructive"
            size="sm"
            onclick={handleEmptyTrash}
            class="h-9"
          >
            <Trash class="w-4 h-4 me-1.5" />
            <span>تفريغ السلة بالكامل</span>
          </Button>
        {/if}

        <Button
          variant="outline"
          size="sm"
          onclick={() => goto(`/${orgId}/settings/archive`)}
          class="h-9"
        >
          <FolderOpen class="w-4 h-4 me-1.5 text-primary" />
          <span>العودة للأرشيف</span>
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
          placeholder="بحث في سلة المحذوفات..."
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
      <div class="flex items-center gap-2 flex-wrap">
        <!-- Asset Type Filter -->
        <select
          bind:value={selectedAssetType}
          class="h-8 rounded-lg border border-border bg-background px-2.5 text-xs text-foreground focus:outline-none focus:ring-1 focus:ring-primary"
        >
          <option value="all">جميع الأنواع</option>
          <option value="image">صور</option>
          <option value="pdf">مستندات PDF</option>
          <option value="video">فيديو</option>
          <option value="audio">صوت</option>
        </select>

        <!-- Category Filter -->
        {#if availableCategories.length > 0}
          <select
            bind:value={selectedCategory}
            class="h-8 rounded-lg border border-border bg-background px-2.5 text-xs text-foreground focus:outline-none focus:ring-1 focus:ring-primary capitalize"
          >
            <option value="all">جميع التصنيفات</option>
            {#each availableCategories as cat}
              <option value={cat}>{cat}</option>
            {/each}
          </select>
        {/if}

        <!-- View Switcher (Grid / Table) -->
        <div class="flex items-center rounded-lg border border-border bg-muted/40 p-0.5">
          <Button
            variant="ghost"
            size="icon"
            class="h-7 w-7 rounded-md {viewMode === 'grid' ? 'bg-background shadow-xs text-foreground' : 'text-muted-foreground'}"
            onclick={() => (viewMode = "grid")}
            title="عرض شبكي"
          >
            <Grid class="w-3.5 h-3.5" />
          </Button>

          <Button
            variant="ghost"
            size="icon"
            class="h-7 w-7 rounded-md {viewMode === 'table' ? 'bg-background shadow-xs text-foreground' : 'text-muted-foreground'}"
            onclick={() => (viewMode = "table")}
            title="عرض جدولي"
          >
            <Bars3BottomLeft class="w-3.5 h-3.5" />
          </Button>
        </div>
      </div>
    </div>
  </header>

  <!-- Multi-Select Floating Toolbar -->
  {#if selectedIds.size > 0}
    <div class="shrink-0 bg-primary/10 border-b border-primary/20 px-6 py-2.5 flex items-center justify-between text-xs transition-all">
      <div class="flex items-center gap-3">
        <span class="font-medium text-foreground">
          تم تحديد <strong class="text-primary">{selectedIds.size}</strong> من أصل {filteredDocs.length} ملف
        </span>
        <button
          type="button"
          class="text-xs text-muted-foreground hover:text-foreground underline"
          onclick={() => (selectedIds = new Set())}
        >
          إلغاء التحديد
        </button>
      </div>

      <div class="flex items-center gap-2">
        <Button
          variant="outline"
          size="sm"
          class="h-7 text-xs text-emerald-600 border-emerald-500/30 hover:bg-emerald-500/10"
          onclick={handleBatchRestore}
        >
          <Refresh class="w-3.5 h-3.5 me-1.5" />
          <span>استعادة المحدد</span>
        </Button>

        <Button
          variant="destructive"
          size="sm"
          class="h-7 text-xs"
          onclick={handleBatchPurge}
        >
          <Trash class="w-3.5 h-3.5 me-1.5" />
          <span>حذف نهائي للمحدد</span>
        </Button>
      </div>
    </div>
  {/if}

  <!-- Main Content Stage -->
  <main class="flex-1 overflow-y-auto p-6">
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

        <h3 class="text-base font-semibold text-foreground mb-1">سلة المحذوفات فارغة</h3>
        <p class="text-xs text-muted-foreground mb-6 leading-relaxed">
          {searchQuery
            ? "لا توجد ملفات محذوفة تطابق معايير البحث المحددة."
            : "لا توجد ملفات محذوفة حالياً. الملفات التي تقوم بحذفها من الأرشيف ستظهر هنا مع إمكانية استعادتها."}
        </p>

        <Button
          variant="outline"
          size="sm"
          onclick={() => goto(`/${orgId}/settings/archive`)}
        >
          <FolderOpen class="w-4 h-4 me-1.5 text-primary" />
          <span>تصفح الأرشيف</span>
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
                    aria-label="تحديد الكل"
                  />
                </th>
                <th class="px-4 py-3 text-start">الملف</th>
                <th class="px-4 py-3 text-start">التصنيف</th>
                <th class="px-4 py-3 text-start">الحجم</th>
                <th class="px-4 py-3 text-start">تاريخ الحذف</th>
                <th class="px-4 py-3 text-end">إجراءات</th>
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
