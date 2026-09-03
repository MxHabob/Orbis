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
import { Button } from "@orbis/ui-svelte/button";
import ArchivePreviewSheet from "$lib/features/documents/archive-preview-sheet.svelte";
import ArchiveDocumentCard from "$lib/features/documents/archive-document-card.svelte";
import ArchiveFilters from "$lib/features/documents/archive-filters.svelte";
import ArchiveLoading from "$lib/features/documents/archive-loading.svelte";
import { confirmStore } from "$lib/stores";
import { File, Loader2, RefreshCw, Upload } from "@lucide/svelte";
import type { Documents } from "@orbis/sdk-core";
import { createOperationToast } from "$lib/shared/components/operation-toast";

let selectedCategory = $state("");
let selectedAssetType = $state("");
let selectedSchoolYear = $state("");
let selectedTerm = $state("");
let uploadCategory = $state("events");

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
let isPreviewOpen = $state(false);
let selectedDocument = $state<Documents | null>(null);
let selectedDocumentUrl = $state("");
let archiveScroller: HTMLDivElement | undefined = $state();
const documentsQuery = useDocuments(() => ({
  entityType: "archive",
  category: selectedCategory,
  search: searchQuery,
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

// Flatten pages into a single array of documents
let documents = $derived(
  documentsQuery.data?.pages.flatMap((p) => p.items) || [],
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

function openFilePicker() {
  fileInput?.click();
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
  <!-- Page Header -->
  <div class="flex items-center justify-between pb-4 border-b border-border/40">
    <div>
      <h1 class="text-xl font-bold tracking-tight text-foreground">{m.settings_archive_title()}</h1>
      <p class="text-xs text-muted-foreground mt-1">
        {m.settings_archive_desc()}
      </p>
    </div>
    <div class="flex items-center gap-2">
      <Button variant="outline" size="icon-sm" onclick={() => documentsQuery.refetch()} disabled={documentsQuery.isFetching} title={m.common_control_refresh()}>
        <RefreshCw size={14} class={documentsQuery.isFetching ? "animate-spin" : ""} />
      </Button>
      <select bind:value={uploadCategory} class="hidden h-8 rounded-md border border-border bg-background px-2 text-xs text-foreground sm:block" aria-label="تصنيف الملفات الجديدة">
        {#each archiveCategories as [value, label]}
          <option value={value}>{label}</option>
        {/each}
      </select>
      <Button variant="default" size="sm" onclick={openFilePicker} disabled={uploadMutation.isPending}>
        {#if uploadMutation.isPending}<Loader2 size={14} class="me-1.5 animate-spin" />{:else}<Upload size={14} class="me-1.5" />{/if}
        {m.common_docs_btn_upload()}
      </Button>
      <input bind:this={fileInput} type="file" accept="image/*,video/*" multiple class="hidden" onchange={handleFileSelect} />
    </div>
  </div>

  <ArchiveFilters
    bind:searchQuery
    bind:selectedCategory
    bind:selectedAssetType
    bind:selectedSchoolYear
    bind:selectedTerm
    categories={archiveCategories}
  />

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
    <!-- Document Groups -->
    {#each groupedEntries as [category, docs]}
      <div class="space-y-4">
        <h2 class="text-xs font-semibold text-muted-foreground uppercase tracking-wider border-b border-border/40 pb-2">{category}</h2>
        <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-4">
          {#each docs as doc (doc.id)}
            <ArchiveDocumentCard document={doc} onpreview={openPreview} ondelete={handleDelete} />
          {/each}
        </div>
      </div>
    {/each}
    
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

<ArchivePreviewSheet
  bind:open={isPreviewOpen}
  document={selectedDocument}
  url={selectedDocumentUrl}
  ondelete={handleDelete}
/> 

