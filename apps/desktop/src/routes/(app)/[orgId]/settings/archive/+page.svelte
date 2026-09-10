<script lang="ts">
import * as m from "$lib/paraglide/messages";
import {
  useDocuments,
  useUploadDocument,
  useDeleteDocument,
  resolveDocumentUrl,
  getDocumentSource,
} from "$lib/features/documents/queries";
import { page } from "$app/state";
import { api } from "$lib/api";
import { Button } from "@orbis/ui/button";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
} from "@orbis/ui/select";
import ArchivePreviewSheet from "$lib/features/documents/archive-preview-sheet.svelte";
import ArchiveDocumentCard from "$lib/features/documents/archive-document-card.svelte";
import ArchiveFilters from "$lib/features/documents/archive-filters.svelte";
import ArchiveLoading from "$lib/features/documents/archive-loading.svelte";
import SettingsBottomBar from "$lib/shared/components/settings-bottom-bar.svelte";
import { DataTable, CheckboxCell } from "$lib/tables";
import type { OrbisColumnDef, GridActionEvent } from "@orbis/svelte-grid";
import { confirmStore } from "$lib/stores";
import { File, LayoutGrid, List, Loader2, Upload } from "@lucide/svelte";
import type { Documents } from "@orbis/sdk";
import { createOperationToast } from "$lib/shared/components/operation-toast";

let selectedCategory = $state("");
let selectedAssetType = $state("");
let selectedSchoolYear = $state("");
let selectedTerm = $state("");
let uploadCategory = $state("events");
let viewMode = $state<"grid" | "table">("grid");
let selectedIds = $state<Set<string>>(new Set());
let academicPeriods = $state<any[]>([]);

const archiveCategories = [
  ["events", "الفعاليات"],
  ["students", "الطلاب"],
  ["staff", "الموظفون"],
  ["certificates", "الشهادات"],
  ["administration", "الإدارة"],
  ["academic", "أكاديمي"],
  ["archive", "غير مصنف"],
] as const;
let fileInput: HTMLInputElement | undefined = $state();
let searchQuery = $state("");
let debouncedSearchQuery = $state("");
let isPreviewOpen = $state(false);
let selectedDocument = $state<Documents | null>(null);
let selectedDocumentUrl = $state("");
let archiveScroller: HTMLDivElement | undefined = $state();

$effect(() => {
  api
    .listAcademicPeriods()
    .then((periods) => {
      academicPeriods = periods || [];
    })
    .catch(() => {
      academicPeriods = [];
    });
});

$effect(() => {
  const nextSearch = searchQuery.trim();
  const timeout = setTimeout(() => {
    debouncedSearchQuery = nextSearch;
  }, 250);
  return () => clearTimeout(timeout);
});

const documentsQuery = useDocuments(() => ({
  entityType: "archive",
  category: selectedCategory,
  search: debouncedSearchQuery,
  assetType: selectedAssetType,
  schoolYear: selectedSchoolYear,
  term: selectedTerm,
}));
const uploadMutation = useUploadDocument({ invalidateOnSuccess: false });
const deleteMutation = useDeleteDocument();
const orgId = $derived(page.params.orgId ?? "");

async function loadNextPage() {
  if (!documentsQuery.hasNextPage || documentsQuery.isFetchingNextPage) return;
  await documentsQuery.fetchNextPage();
}

function infiniteScroll(node: HTMLDivElement) {
  const observer = new IntersectionObserver(
    ([entry]) => {
      if (entry?.isIntersecting) loadNextPage();
    },
    { root: archiveScroller ?? null, rootMargin: "320px 0px" },
  );

  observer.observe(node);
  return { destroy: () => observer.disconnect() };
}

async function handleDelete(id: string) {
  const confirmed = await confirmStore.prompt({
    description: m.common_docs_delete_confirm(),
    title: m.common_docs_delete_confirm(),
    confirmText: m.common_action_delete(),
    variant: "destructive",
  });
  if (confirmed) {
    deleteMutation.mutate(id);
  }
}

async function handleBulkDelete() {
  const confirmed = await confirmStore.prompt({
    description: `سيتم حذف ${selectedCount} ملفًا محددًا.`,
    title: "حذف الملفات المحددة",
    confirmText: m.common_action_delete(),
    variant: "destructive",
  });
  if (!confirmed) return;

  await Promise.all(
    [...selectedIds].map((id) => deleteMutation.mutateAsync(id)),
  );
  selectedIds = new Set();
  await documentsQuery.refetch();
}

// Flatten pages into a single array of documents
let documents = $derived(
  documentsQuery.data?.pages.flatMap((p) => p.items) || [],
);
let availableCategories = $derived.by(() => {
  const labels = new Map(archiveCategories);
  const values = new Set(
    documents.map((document) => document.category).filter(Boolean),
  );
  return [...values]
    .sort()
    .map((value) => [value, labels.get(value) || value] as const);
});
function periodOption(period: any) {
  const value = String(
    period.code || period.name || period.name_en || period.name_ar || period.id,
  );
  const label = period.name_ar || period.name || period.name_en || value;
  return [value, label] as const;
}
let archiveTerms = $derived(
  academicPeriods
    .filter((period) => period.period_type === "term")
    .map(periodOption),
);
let archiveSchoolYears = $derived(
  academicPeriods
    .filter(
      (period) =>
        period.period_type === "academic_year" || period.period_type === "year",
    )
    .map(periodOption),
);
let groupedDocs = $derived(
  documents.reduce(
    (acc: Record<string, Documents[]>, doc: Documents) => {
      const cat = doc.category || "uncategorized";
      if (!acc[cat]) acc[cat] = [];
      acc[cat].push(doc);
      return acc;
    },
    {} as Record<string, Documents[]>,
  ),
);
let groupedEntries = $derived(
  Object.entries(groupedDocs) as [string, Documents[]][],
);
let allVisibleSelected = $derived(
  documents.length > 0 &&
    documents.every((document) => selectedIds.has(document.id)),
);
let selectedCount = $derived(selectedIds.size);
let tableSource = $derived(
  documents.map((document) => ({
    ...document,
    __selected: selectedIds.has(document.id),
    size_display: `${((document.size || 0) / 1024).toFixed(1)} KB`,
    type_display:
      document.asset_type || document.extension?.toUpperCase() || "FILE",
    date_display: document.created_at
      ? new Intl.DateTimeFormat(undefined, { dateStyle: "medium" }).format(
          new Date(document.created_at),
        )
      : "-",
  })),
);
let archiveTableColumns = $derived<OrbisColumnDef[]>([
  {
    id: "__selected",
    prop: "__selected",
    header: "",
    name: "",
    width: 44,
    size: 44,
    cell: CheckboxCell,
    sort: false,
  },
  {
    id: "name",
    prop: "name",
    header: "الملف",
    name: "الملف",
    width: 300,
    size: 300,
  },
  {
    id: "category",
    prop: "category",
    header: "التصنيف",
    name: "التصنيف",
    width: 140,
    size: 140,
  },
  {
    id: "type_display",
    prop: "type_display",
    header: "النوع",
    name: "النوع",
    width: 110,
    size: 110,
  },
  {
    id: "size_display",
    prop: "size_display",
    header: "الحجم",
    name: "الحجم",
    width: 110,
    size: 110,
  },
  {
    id: "date_display",
    prop: "date_display",
    header: "التاريخ",
    name: "التاريخ",
    width: 150,
    size: 150,
  },
]);

function openFilePicker() {
  fileInput?.click();
}

function toggleDocument(id: string, selected: boolean) {
  const next = new Set(selectedIds);
  if (selected) next.add(id);
  else next.delete(id);
  selectedIds = next;
}

function toggleAllVisible() {
  if (allVisibleSelected) {
    selectedIds = new Set();
    return;
  }
  selectedIds = new Set(documents.map((document) => document.id));
}

function handleTableAction(event: GridActionEvent) {
  if (event.actionId === "select-all") {
    toggleAllVisible();
    return;
  }
  if (event.actionId === "dblclick" && event.row?.id) {
    const document = documents.find((item) => item.id === String(event.row.id));
    if (document) openPreview(document);
    return;
  }
  if (event.actionId !== "toggle-select" && event.actionId !== "select") return;
  const id = String(event.row?.id || "");
  if (!id) return;
  toggleDocument(id, !selectedIds.has(id));
}

async function openPreview(doc: Documents) {
  selectedDocument = doc;
  selectedDocumentUrl = await resolveDocumentUrl(getDocumentSource(doc));
  isPreviewOpen = true;
}

async function handleFileSelect(event: Event) {
  const target = event.currentTarget as HTMLInputElement;
  const file = target.files?.[0];
  if (!file) return;

  const operation = createOperationToast({
    title: `${m.common_docs_btn_upload()} (${target.files?.length ?? 1})`,
    message: file.name,
    actionLabel: m.common_control_refresh(),
    onAction: () => documentsQuery.refetch(),
  });
  const files = Array.from(target.files ?? []);
  operation.update(5, "Preparing files...");

  try {
    let completed = 0;
    await Promise.all(
      files.map(async (selectedFile) => {
        await uploadMutation.mutateAsync({
          file: selectedFile,
          entity_type: "archive",
          entity_id: orgId || "organization",
          category: uploadCategory,
          visibility: "private",
        });
        completed += 1;
        operation.update(
          10 + (completed / files.length) * 85,
          `${completed} / ${files.length} uploaded`,
        );
      }),
    );
    await documentsQuery.refetch();
    operation.success(`${files.length} files uploaded`);
  } catch (error) {
    operation.error((error as any)?.message || "Failed to upload files");
  } finally {
    target.value = "";
  }
}
</script>

<div bind:this={archiveScroller} class="flex flex-col gap-6 p-6 h-full overflow-auto scrollbar-hide">
  <div class="flex flex-col gap-3 border-b border-border/40 pb-4 lg:flex-row lg:items-center">
    <ArchiveFilters
      bind:searchQuery
      bind:selectedCategory
      bind:selectedAssetType
      bind:selectedSchoolYear
      bind:selectedTerm
      categories={availableCategories.length > 0 ? availableCategories : archiveCategories}
      terms={archiveTerms}
      schoolYears={archiveSchoolYears}
    />
    <div class="flex shrink-0 items-center gap-2">
      <div class="flex items-center rounded-md border border-border bg-background p-0.5" role="group" aria-label="طريقة عرض الأرشيف">
        <Button variant={viewMode === "grid" ? "secondary" : "ghost"} size="icon-xs" onclick={() => viewMode = "grid"} title="عرض شبكي" aria-label="عرض شبكي">
          <LayoutGrid class="h-3.5 w-3.5" />
        </Button>
        <Button variant={viewMode === "table" ? "secondary" : "ghost"} size="icon-xs" onclick={() => viewMode = "table"} title="عرض جدول" aria-label="عرض جدول">
          <List class="h-3.5 w-3.5" />
        </Button>
      </div>
      <Select type="single" value={uploadCategory} onValueChange={(value: string | undefined) => uploadCategory = value ?? "events"}>
        <SelectTrigger class="h-8 w-32 text-xs"><span class="truncate">{archiveCategories.find(([value]) => value === uploadCategory)?.[1] || "تصنيف الرفع"}</span></SelectTrigger>
        <SelectContent align="end">
          {#each archiveCategories as [value, label]}
            <SelectItem value={value}>{label}</SelectItem>
          {/each}
        </SelectContent>
      </Select>
      <Button variant="default" size="sm" onclick={openFilePicker} disabled={uploadMutation.isPending}>
        {#if uploadMutation.isPending}<Loader2 size={14} class="me-1.5 animate-spin" />{:else}<Upload size={14} class="me-1.5" />{/if}
        {m.common_docs_btn_upload()}
      </Button>
      <input bind:this={fileInput} type="file" accept="image/*,video/*" multiple class="hidden" onchange={handleFileSelect} />
    </div>
  </div>

  <!-- Loading Skeletons (Uses base Skeleton component styles) -->
  {#if documentsQuery.isLoading && documents.length === 0}
    <ArchiveLoading />
  {:else if documents.length === 0}
    <!-- Empty State -->
    <div class="flex flex-col items-center justify-center py-20 text-center bg-muted/10 rounded-2xl border border-dashed border-border/60">
      <div class="w-12 h-12 rounded-2xl bg-muted/40 flex items-center justify-center mb-3">
        <File class="h-6 w-6 text-muted-foreground/60" />
      </div>
      <h3 class="text-base font-semibold text-foreground">{m.settings_archive_empty_title()}</h3>
      <p class="text-xs text-muted-foreground max-w-sm mt-1">{m.settings_archive_empty_desc()}</p>
      <Button variant="outline" size="sm" class="mt-3" onclick={openFilePicker} disabled={uploadMutation.isPending}>
        <Upload size={14} class="me-1.5" />
        {m.common_docs_btn_upload()}
      </Button>
    </div>
  {:else}
    {#if viewMode === "grid"}
      {#each groupedEntries as [category, docs]}
        <div class="space-y-4">
          <h2 class="text-xs font-semibold text-muted-foreground uppercase tracking-wider border-b border-border/40 pb-2">{category}</h2>
          <div class="grid grid-cols-2 gap-4 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6">
            {#each docs as doc (doc.id)}
              <ArchiveDocumentCard document={doc} selected={selectedIds.has(doc.id)} onpreview={openPreview} ondelete={handleDelete} ontoggle={toggleDocument} />
            {/each}
          </div>
        </div>
      {/each}
    {:else}
      <DataTable
        columns={archiveTableColumns}
        source={tableSource}
        height="min(70vh, 620px)"
        rowSize={44}
        rtl={true}
        onaction={handleTableAction}
        emptyTitle="لا توجد ملفات"
      />
    {/if}
    
    <!-- Infinite scroll sentinel and manual fallback -->
    <div use:infiniteScroll class="h-px w-full" aria-hidden="true"></div>
    {#if documentsQuery.hasNextPage}
      <div class="flex justify-center pt-6 pb-4">
        <Button 
          variant="outline" 
          size="sm"
          disabled={documentsQuery.isFetchingNextPage}
          onclick={loadNextPage}
          class="text-xs gap-2 rounded-xl"
        >
          {#if documentsQuery.isFetchingNextPage}
            <Loader2 class="h-3.5 w-3.5 animate-spin" />
            <span>{m.settings_archive_loading_more()}</span>
          {:else}
            <span>{m.settings_archive_load_more()}</span>
          {/if}
        </Button>
      </div>
    {/if}
  {/if}
</div>

<SettingsBottomBar
  visible={selectedCount > 0}
  isDirty={selectedCount > 0}
  isSaving={deleteMutation.isPending}
  message={`${selectedCount} ملف محدد`}
  saveLabel="حذف المحدد"
  savingLabel="جاري الحذف..."
  resetLabel="إلغاء التحديد"
  onsave={handleBulkDelete}
  onreset={() => selectedIds = new Set()}
/>

<ArchivePreviewSheet
  bind:open={isPreviewOpen}
  document={selectedDocument}
  url={selectedDocumentUrl}
  ondelete={handleDelete}
/> 

