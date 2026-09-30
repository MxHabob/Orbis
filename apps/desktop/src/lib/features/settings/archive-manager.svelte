<!--
  @component ArchiveManager
  Enterprise Digital Archive & Student Document Repository.
  Features:
  - Student-specific document search and filtering with declarative queries (zero $effect).
  - Authoritative ClassSelect and PeriodSelect (Year & Term) from Academics domain.
  - Dynamic category and asset type filtering.
  - Hybrid Grid View & Virtualized OrbisGrid Table View.
  - Fast multi-file upload with progress tracking and direct entity association.
  - Full preview drawer with pan/zoom and batch management.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { createQuery } from "@tanstack/svelte-query";
import {
  useDocuments,
  useUploadDocument,
  useDeleteDocument,
  resolveDocumentUrl,
  getDocumentSource,
} from "$lib/features/documents/queries";
import { api, type ClassDto, type StudentDetailDto, type DocumentDto } from "$lib/api";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Badge } from "@orbis/ui/badge";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
} from "@orbis/ui/select";
import ArchivePreviewSheet from "$lib/features/documents/archive-preview-sheet.svelte";
import ArchiveDocumentCard from "$lib/features/documents/archive-document-card.svelte";
import SettingsBottomBar from "$lib/shared/components/settings-bottom-bar.svelte";
import { type OrbisColumnDef, type GridActionEvent, type ActionItem, OrbisGrid } from "@orbis/svelte-grid";
import { confirmStore } from "$lib/stores";
import {
  AdjustmentsHorizontal,
  ArrowPath,
  ArrowUpTray,
  Document,
  Grid,
  QueueList,
  Search,
  User,
  XMark,
  Funnel,
} from "@orbis/icons";
import { createOperationToast } from "$lib/shared/components/operation-toast";
import ArchiveLoading from "../documents/archive-loading.svelte";
import { ClassSelect, PeriodSelect } from "$lib/features/academics";
import { l } from "$lib/core/i18n";

let { orgId }: { orgId: string } = $props();

// Filter States
let searchQuery = $state("");
let selectedCategory = $state("");
let selectedAssetType = $state("");
let selectedSchoolYear = $state<string | null>("all");
let selectedTerm = $state<string | null>("all");
let selectedClassId = $state<string | null>("all");
let uploadCategory = $state("events");
let viewMode = $state<"grid" | "table">("grid");
let selectedIds = $state<Set<string>>(new Set());

// Student Specific Search State
let studentSearchQuery = $state("");
let isStudentSearchFocused = $state(false);
let selectedStudent = $state<StudentDetailDto | null>(null);

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
let isPreviewOpen = $state(false);
let selectedDocument = $state<DocumentDto | null>(null);
let selectedDocumentUrl = $state("");
let archiveScroller: HTMLDivElement | undefined = $state();

// Declarative Student Search Query
const studentSearchQueryResults = createQuery(() => ({
  queryKey: ["archive-student-search", studentSearchQuery.trim()],
  queryFn: () => api.listStudents({ search: studentSearchQuery.trim(), pageSize: 6 }),
  enabled: studentSearchQuery.trim().length >= 2,
  staleTime: 30 * 1000,
}));

const studentSearchResults = $derived<StudentDetailDto[]>(
  studentSearchQueryResults.data?.items || [],
);
const isSearchingStudents = $derived(studentSearchQueryResults.isLoading);

function getStudentDisplayName(s: StudentDetailDto) {
  const ar = [s.person?.first_name_ar,s.person.middle_name_ar ,s.person?.last_name_ar].filter(Boolean).join(" ");
  const en = [s.person?.full_name, s.person.middle_name, s.person.last_name].filter(Boolean).join(" ");
  return ar || en || s.student?.student_number || s.student?.id;
}

// Documents Query
const documentsQuery = useDocuments(() => ({
  entityType: selectedStudent ? "student" : "archive",
  category: selectedCategory,
  search: searchQuery.trim(),
  assetType: selectedAssetType,
  schoolYear: selectedSchoolYear === "all" ? "" : selectedSchoolYear ?? "",
  term: selectedTerm === "all" ? "" : selectedTerm ?? "",
  entityId: selectedStudent?.student?.id,
}));

const uploadMutation = useUploadDocument({ invalidateOnSuccess: false });
const deleteMutation = useDeleteDocument();

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

// Raw Documents Flattened
const rawDocuments = $derived(
  documentsQuery.data?.pages.flatMap((p) => p.items) || [],
);

// Declarative Client Filtering
const documents = $derived.by(() => {
  const q = searchQuery.trim().toLowerCase();
  return rawDocuments.filter((doc) => {
    // Student Filter
    if (selectedStudent) {
      const sId = selectedStudent.student?.id;
      const sUserId = selectedStudent.student?.user_id;
      const sNum = selectedStudent.student?.student_number?.toLowerCase();
      const docEntityId = doc.entity_id || "";
      const docName = doc.name.toLowerCase();

      const matchesStudent =
        docEntityId === sId ||
        docEntityId === sUserId ||
        (sNum && docName.includes(sNum)) ||
        (sNum && (doc.tags || "").toLowerCase().includes(sNum));

      if (!matchesStudent) return false;
    }

    // Category Filter
    if (selectedCategory && doc.category !== selectedCategory) {
      return false;
    }

    // Asset Type Filter
    if (selectedAssetType) {
      if (selectedAssetType === "image" && !doc.mime_type?.startsWith("image/")) return false;
      if (selectedAssetType === "video" && !doc.mime_type?.startsWith("video/")) return false;
      if (selectedAssetType === "document" && (doc.mime_type?.startsWith("image/") || doc.mime_type?.startsWith("video/"))) return false;
    }

    // School Year Filter
    if (selectedSchoolYear && selectedSchoolYear !== "all") {
      if (doc.academic_year_id && doc.academic_year_id !== selectedSchoolYear) {
        return false;
      }
    }

    // Term Filter
    if (selectedTerm && selectedTerm !== "all") {
      if (doc.term_id && doc.term_id !== selectedTerm) {
        return false;
      }
    }

    // General Search
    if (q) {
      const matchName = doc.name.toLowerCase().includes(q);
      const matchDesc = (doc.description || "").toLowerCase().includes(q);
      const matchTags = (doc.tags || "").toLowerCase().includes(q);
      if (!matchName && !matchDesc && !matchTags) return false;
    }

    return true;
  });
});

// Category options
let availableCategories = $derived.by(() => {
  const labels = new Map(archiveCategories);
  const values = new Set(
    rawDocuments.map((document) => document.category).filter(Boolean),
  );
  return [...values]
    .sort()
    .map((value) => [value, labels.get(value) || value] as const);
});

let groupedDocs = $derived(
  documents.reduce(
    (acc: Record<string, DocumentDto[]>, doc: DocumentDto) => {
      const cat = doc.category || "uncategorized";
      if (!acc[cat]) acc[cat] = [];
      acc[cat].push(doc);
      return acc;
    },
    {} as Record<string, DocumentDto[]>,
  ),
);

let groupedEntries = $derived(
  Object.entries(groupedDocs) as [string, DocumentDto[]][],
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
    size_display: `${((document.file_size_bytes || 0) / 1024).toFixed(1)} KB`,
    type_display:
      document.asset_type || document.extension?.toUpperCase() || "FILE",
    date_display: document.created_at
      ? new Intl.DateTimeFormat(undefined, { dateStyle: "medium" }).format(
          new Date(document.created_at),
        )
      : "-",
  })),
);

const tableRowActions: ActionItem[] = [
  { id: "preview", label: "معاينة", icon: "Eye" },
  { id: "delete", label: "حذف", icon: "Trash2", variant: "destructive" },
];

let archiveTableColumns = $derived<OrbisColumnDef[]>([
  {
    id: "__selected",
    prop: "__selected",
    header: "",
    name: "",
    width: 44,
    size: 44,
    sort: false,
  },
  {
    id: "name",
    prop: "name",
    header: "الملف والمستند",
    name: "الملف والمستند",
    width: 280,
    size: 280,
    cellClass: "font-semibold",
  },
  {
    id: "category",
    prop: "category",
    header: "التصنيف",
    name: "التصنيف",
    width: 140,
    size: 140,
    type: "badge",
    align: "center",
  },
  {
    id: "type_display",
    prop: "type_display",
    header: "النوع",
    name: "النوع",
    width: 110,
    size: 110,
    align: "center",
  },
  {
    id: "size_display",
    prop: "size_display",
    header: "الحجم",
    name: "الحجم",
    width: 110,
    size: 110,
    align: "center",
  },
  {
    id: "date_display",
    prop: "date_display",
    header: "تاريخ الإضافة",
    name: "تاريخ الإضافة",
    width: 150,
    size: 150,
    align: "center",
  },
  {
    id: "actions",
    prop: "actions",
    header: "إجراءات",
    name: "إجراءات",
    width: 110,
    size: 110,
    type: "action",
    align: "center",
    readonly: true,
    meta: { items: tableRowActions },
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
  if (event.actionId === "preview" && event.row?.id) {
    const document = documents.find((item) => item.id === String(event.row.id));
    if (document) openPreview(document);
    return;
  }
  if (event.actionId === "delete" && event.row?.id) {
    handleDelete(String(event.row.id));
    return;
  }
  if (event.actionId !== "toggle-select" && event.actionId !== "select") return;
  const id = String(event.row?.id || "");
  if (!id) return;
  toggleDocument(id, !selectedIds.has(id));
}

async function openPreview(doc: DocumentDto) {
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
    const entityType = selectedStudent ? "student" : "archive";
    const entityId = selectedStudent ? selectedStudent.student.id : orgId || "organization";

    await Promise.all(
      files.map(async (selectedFile) => {
        await uploadMutation.mutateAsync({
          file: selectedFile,
          entity_type: entityType,
          entity_id: entityId,
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

const hasActiveFilters = $derived(
  Boolean(
    searchQuery ||
    selectedCategory ||
    selectedAssetType ||
    (selectedSchoolYear && selectedSchoolYear !== "all") ||
    (selectedTerm && selectedTerm !== "all") ||
    (selectedClassId && selectedClassId !== "all") ||
    selectedStudent
  )
);

function clearAllFilters() {
  searchQuery = "";
  selectedCategory = "";
  selectedAssetType = "";
  selectedSchoolYear = "all";
  selectedTerm = "all";
  selectedClassId = "all";
  selectedStudent = null;
  studentSearchQuery = "";
}
</script>

<div bind:this={archiveScroller} class="flex flex-col gap-5 p-6 h-full overflow-auto scrollbar-hide text-start">
  
  <!-- Comprehensive Archive Filtering & Action Bar -->
  <div class="flex flex-col gap-3.5 border-b border-border/50 pb-4">
    
    <!-- Top Row: General Search + Student Search Autocomplete + View Switches & Upload -->
    <div class="flex flex-col lg:flex-row items-stretch lg:items-center justify-between gap-3">
      
      <!-- Search Fields Container -->
      <div class="flex flex-col sm:flex-row items-stretch sm:items-center gap-2.5 flex-1 max-w-2xl">
        
        <!-- General Document Search -->
        <div class="relative flex-1 min-w-[180px]">
          <Search size={14} class="absolute inset-s-3 top-1/2 -translate-y-1/2 text-muted-foreground/70 pointer-events-none" />
          <Input
            bind:value={searchQuery}
            placeholder={l("البحث باسم الملف أو الوصف...", "Search file name or description...")}
            class="h-9 w-full pe-3 ps-9 text-xs bg-background"
          />
        </div>

        <!-- Student-Specific Search Dropdown -->
        <div class="relative flex-1 min-w-[220px]">
          <User size={14} class="absolute inset-s-3 top-1/2 -translate-y-1/2 text-muted-foreground/70 pointer-events-none" />
          <Input
            bind:value={studentSearchQuery}
            placeholder={l("بحث وثائق طالب معين (اسم أو رقم)...", "Filter by specific student...")}
            class="h-9 w-full pe-3 ps-9 text-xs bg-background"
            onfocus={() => isStudentSearchFocused = true}
            onblur={() => setTimeout(() => (isStudentSearchFocused = false), 250)}
          />

          <!-- Live Student Results Popover -->
          {#if isStudentSearchFocused && studentSearchResults.length > 0}
            <div class="absolute z-50 top-10 inset-x-0 bg-popover border border-border/80 shadow-lg rounded-xl overflow-hidden max-h-60 overflow-y-auto">
              <div class="p-1.5 flex flex-col gap-0.5">
                {#each studentSearchResults as student (student.student?.id)}
                  <button
                    type="button"
                    class="w-full px-3 py-2 text-start flex items-center justify-between gap-2 rounded-lg hover:bg-muted text-xs transition-colors cursor-pointer"
                    onmousedown={() => {
                      selectedStudent = student;
                      studentSearchQuery = "";
                    }}
                  >
                    <div class="flex flex-col">
                      <span class="font-bold text-foreground">{getStudentDisplayName(student)}</span>
                      <span class="text-[10px] font-mono text-muted-foreground">{student.student?.student_number}</span>
                    </div>
                    <Badge variant="outline" class="text-[10px]">
                      {student.student?.status || "طالب"}
                    </Badge>
                  </button>
                {/each}
              </div>
            </div>
          {/if}
        </div>
      </div>

      <!-- Right Controls: View Switcher, Category Upload & Upload Button -->
      <div class="flex shrink-0 items-center gap-2">
        <div class="flex items-center rounded-lg border border-border bg-background p-0.5" role="group" aria-label="طريقة عرض الأرشيف">
          <Button
            variant={viewMode === "grid" ? "secondary" : "ghost"}
            size="icon-xs"
            onclick={() => viewMode = "grid"}
            title="عرض شبكي"
            aria-label="عرض شبكي"
          >
            <Grid class="h-3.5 w-3.5" />
          </Button>
          <Button
            variant={viewMode === "table" ? "secondary" : "ghost"}
            size="icon-xs"
            onclick={() => viewMode = "table"}
            title="عرض جدول"
            aria-label="عرض جدول"
          >
            <QueueList class="h-3.5 w-3.5" />
          </Button>
        </div>

        <Select
          type="single"
          value={uploadCategory}
          onValueChange={(value: string | undefined) => uploadCategory = value ?? "events"}
        >
          <SelectTrigger class="h-9 w-32 text-xs">
            <span class="truncate">{archiveCategories.find(([value]) => value === uploadCategory)?.[1] || "تصنيف الرفع"}</span>
          </SelectTrigger>
          <SelectContent align="end">
            {#each archiveCategories as [value, label]}
              <SelectItem value={value}>{label}</SelectItem>
            {/each}
          </SelectContent>
        </Select>

        <Button variant="default" size="sm" class="h-9 text-xs font-bold gap-1.5" onclick={openFilePicker} disabled={uploadMutation.isPending}>
          {#if uploadMutation.isPending}
            <ArrowPath size={14} class="animate-spin" />
          {:else}
            <ArrowUpTray size={14} />
          {/if}
          <span>{m.common_docs_btn_upload()}</span>
        </Button>
        <input bind:this={fileInput} type="file" accept="image/*,video/*,application/pdf" multiple class="hidden" onchange={handleFileSelect} />
      </div>
    </div>

    <!-- Active Student Filter Banner -->
    {#if selectedStudent}
      <div class="flex items-center justify-between px-3 py-1.5 bg-primary/10 border border-primary/25 rounded-lg text-xs">
        <div class="flex items-center gap-2">
          <User class="w-3.5 h-3.5 text-primary" />
          <span class="text-muted-foreground">{l("تصفية وثائق الطالب:", "Filtered by Student:")}</span>
          <span class="font-bold text-foreground">{getStudentDisplayName(selectedStudent)}</span>
          <span class="text-xs font-mono text-primary font-semibold">({selectedStudent.student?.student_number})</span>
        </div>
        <button
          type="button"
          onclick={() => selectedStudent = null}
          class="text-muted-foreground hover:text-foreground cursor-pointer"
          title={l("إلغاء التصفية", "Clear filter")}
        >
          <XMark class="w-4 h-4" />
        </button>
      </div>
    {/if}

    <!-- Bottom Filter Selectors Row (Academics ClassSelect & PeriodSelect + Category & Type) -->
    <div class="flex flex-wrap items-center gap-2 pt-1">
      <div class="flex items-center gap-1.5 text-muted-foreground text-xs pe-1">
        <Funnel class="w-3.5 h-3.5" />
        <span>{l("الفلاتر الأكاديمية:", "Academic Filters:")}</span>
      </div>

      <!-- Class Select (Reusable Academics Component) -->
      <div class="w-44">
        <ClassSelect
          bind:value={selectedClassId}
          showAllOption={true}
          allOptionLabel={l("كافة الصفوف", "All Classes")}
          placeholder={l("كافة الصفوف", "All Classes")}
        />
      </div>

      <!-- Academic Year Select (Reusable PeriodSelect) -->
      <div class="w-44">
        <PeriodSelect
          bind:value={selectedSchoolYear}
          periodType="academic_year"
          showAllOption={true}
          allOptionLabel={l("كافة الأعوام", "All Academic Years")}
          placeholder={l("كافة الأعوام", "All Academic Years")}
        />
      </div>

      <!-- Term Select (Reusable PeriodSelect) -->
      <div class="w-40">
        <PeriodSelect
          bind:value={selectedTerm}
          periodType="term"
          showAllOption={true}
          allOptionLabel={l("كافة الفصول", "All Terms")}
          placeholder={l("كافة الفصول", "All Terms")}
        />
      </div>

      <!-- Category Filter -->
      <Select
        type="single"
        value={selectedCategory || "all"}
        onValueChange={(value) => selectedCategory = value === "all" ? "" : value ?? ""}
      >
        <SelectTrigger class="h-9 w-36 text-xs bg-background">
          <span class="truncate">{selectedCategory ? (availableCategories.find(([v]) => v === selectedCategory)?.[1] || selectedCategory) : l("كافة التصنيفات", "All Categories")}</span>
        </SelectTrigger>
        <SelectContent align="end">
          <SelectItem value="all">{l("كافة التصنيفات", "All Categories")}</SelectItem>
          {#each (availableCategories.length > 0 ? availableCategories : archiveCategories) as [value, label]}
            <SelectItem value={value}>{label}</SelectItem>
          {/each}
        </SelectContent>
      </Select>

      <!-- Asset Type Filter -->
      <Select
        type="single"
        value={selectedAssetType || "all"}
        onValueChange={(value) => selectedAssetType = value === "all" ? "" : value ?? ""}
      >
        <SelectTrigger class="h-9 w-32 text-xs bg-background">
          <span class="truncate">{selectedAssetType === "image" ? "صور" : selectedAssetType === "video" ? "فيديو" : selectedAssetType === "document" ? "وثائق" : l("كل الأنواع", "All Types")}</span>
        </SelectTrigger>
        <SelectContent align="end">
          <SelectItem value="all">{l("كل الأنواع", "All Types")}</SelectItem>
          <SelectItem value="image">{l("صور", "Images")}</SelectItem>
          <SelectItem value="video">{l("فيديو", "Videos")}</SelectItem>
          <SelectItem value="document">{l("وثائق ومستندات", "Documents")}</SelectItem>
        </SelectContent>
      </Select>

      {#if hasActiveFilters}
        <Button variant="ghost" size="sm" class="h-9 gap-1 text-xs text-muted-foreground hover:text-foreground" onclick={clearAllFilters}>
          <XMark size={13} />
          <span>{l("إعادة تعيين", "Reset")}</span>
        </Button>
      {/if}
    </div>
  </div>

  <!-- Loading Skeletons -->
  {#if documentsQuery.isLoading && rawDocuments.length === 0}
    <ArchiveLoading />
  {:else if documents.length === 0}
    <!-- Empty State -->
    <div class="flex flex-col items-center justify-center py-20 text-center bg-muted/10 rounded-2xl border border-dashed border-border/60">
      <div class="w-12 h-12 rounded-2xl bg-muted/40 flex items-center justify-center mb-3">
        <Document class="h-6 w-6 text-muted-foreground/60" />
      </div>
      <h3 class="text-base font-semibold text-foreground">{m.settings_archive_empty_title()}</h3>
      <p class="text-xs text-muted-foreground max-w-sm mt-1">{m.settings_archive_empty_desc()}</p>
      <Button variant="outline" size="sm" class="mt-3 text-xs" onclick={openFilePicker} disabled={uploadMutation.isPending}>
        <ArrowUpTray size={14} class="me-1.5" />
        {m.common_docs_btn_upload()}
      </Button>
    </div>
  {:else}
    {#if viewMode === "grid"}
      {#each groupedEntries as [category, docs]}
        <div class="space-y-4">
          <h2 class="text-xs font-semibold text-muted-foreground uppercase tracking-wider border-b border-border/40 pb-2">
            {archiveCategories.find(([c]) => c === category)?.[1] || category}
          </h2>
          <div class="grid grid-cols-2 gap-4 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6">
            {#each docs as doc (doc.id)}
              <ArchiveDocumentCard
                document={doc}
                selected={selectedIds.has(doc.id)}
                onpreview={openPreview}
                ondelete={handleDelete}
                ontoggle={toggleDocument}
              />
            {/each}
          </div>
        </div>
      {/each}
    {:else}
      <div class="rounded-xl border border-border/80 bg-background overflow-hidden">
        <OrbisGrid
          columns={archiveTableColumns}
          source={tableSource}
          height="min(70vh, 620px)"
          rowSize={44}
          rtl={true}
          onaction={handleTableAction}
          emptyTitle="لا توجد ملفات"
        />
      </div>
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
            <ArrowPath class="h-3.5 w-3.5 animate-spin" />
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
