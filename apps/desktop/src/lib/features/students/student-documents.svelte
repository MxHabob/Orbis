<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import {
  ArrowUpTray,
  DocumentPage,
  Refresh,
  Search,
  XMark,
} from "@orbis/icons";
import { confirmStore } from "$lib/stores";
import { toast } from "@orbis/ui";
import {
  api,
  extractErrorMessage,
  queryKeys,
  type DocumentDto,
} from "$lib/api";
import { useQueryClient } from "@tanstack/svelte-query";
import {
  getDocumentSource,
  resolveDocumentUrl,
  useDocumentsByEntity,
  useStudentFolder,
  useTrashDocument,
} from "$lib/features/documents/queries";
import ArchiveDocumentCard from "$lib/features/documents/archive-document-card.svelte";
import ArchivePreviewSheet from "$lib/features/documents/archive-preview-sheet.svelte";
import ArchiveLoading from "$lib/features/documents/archive-loading.svelte";
import StudentSectionState from "$lib/features/students/components/student-section-state.svelte";

let { studentId }: { orgId: string; studentId: string } = $props();

const queryClient = useQueryClient();
const studentFolderQuery = useStudentFolder(() => studentId);
const documentsQuery = useDocumentsByEntity(
  () => "student",
  () => studentId,
);
const trashMutation = useTrashDocument();

let searchQuery = $state("");
let isUploading = $state(false);
let fileInput: HTMLInputElement | undefined = $state();
let isPreviewOpen = $state(false);
let selectedDocument = $state<DocumentDto | null>(null);
let selectedDocumentUrl = $state("");

const documents = $derived(
  (documentsQuery.data ?? []).filter(
    (document) => !document.deleted_at && !document.is_archived,
  ),
);
const isLoading = $derived(documentsQuery.isLoading);
const queryError = $derived(
  documentsQuery.error ? extractErrorMessage(documentsQuery.error) : null,
);

const filteredDocuments = $derived.by(() => {
  const query = searchQuery.trim().toLowerCase();
  if (!query) return documents;

  return documents.filter((document) => {
    const searchableText = [
      document.name,
      document.description || "",
      document.category || "",
      document.tags || "",
    ]
      .join(" ")
      .toLowerCase();

    return searchableText.includes(query);
  });
});

function openFilePicker() {
  if (!isUploading) fileInput?.click();
}

async function ensureStudentFolder() {
  return studentFolderQuery.data ?? api.getOrCreateStudentFolder(studentId);
}

async function handleFilesPicked(event: Event) {
  const input = event.currentTarget as HTMLInputElement;
  const files = Array.from(input.files ?? []);
  input.value = "";
  if (files.length === 0 || isUploading) return;

  isUploading = true;
  const errors: string[] = [];
  let uploadedCount = 0;

  try {
    const folder = await ensureStudentFolder();

    for (const file of files) {
      try {
        await api.uploadFile({
          file,
          name: file.name,
          entity_type: "student",
          entity_id: studentId,
          folder_id: folder.id,
          category: "general",
          asset_type: "document",
          visibility: "private",
        });
        uploadedCount += 1;
      } catch (error) {
        console.error(
          `Failed to upload student document "${file.name}":`,
          error,
        );
        errors.push(`${file.name}: ${extractErrorMessage(error)}`);
      }
    }

    if (uploadedCount > 0) {
      toast.success(m.common_docs_upload_success());
      await documentsQuery.refetch();
      await queryClient.invalidateQueries({
        queryKey: queryKeys.folder.contents(folder.id),
      });
    }

    if (errors.length > 0) {
      toast.error(errors.join("\n"));
    }
  } catch (error) {
    toast.error(extractErrorMessage(error));
  } finally {
    isUploading = false;
  }
}

async function handlePreview(document: DocumentDto) {
  try {
    selectedDocument = document;
    selectedDocumentUrl = await resolveDocumentUrl(
      getDocumentSource(document),
      document.id,
    );
    isPreviewOpen = true;
  } catch (error) {
    console.error("Failed to load student document preview:", error);
    toast.error(m.common_loading_error());
  }
}

async function handleDeleteDocument(id: string) {
  const confirmed = await confirmStore.prompt({
    title: m.common_docs_trash_title(),
    description: m.common_docs_trash_confirm(),
    confirmText: m.common_action_delete(),
    variant: "destructive",
  });

  if (!confirmed) return;

  try {
    await trashMutation.mutateAsync(id);
    toast.success(m.common_docs_trash_success());
    await documentsQuery.refetch();
  } catch (error) {
    toast.error(extractErrorMessage(error));
  }
}
</script>

<input
  bind:this={fileInput}
  type="file"
  multiple
  class="hidden"
  onchange={handleFilesPicked}
/>

<div class="flex h-full w-full flex-col overflow-hidden bg-background text-foreground text-start">
  <header class="flex items-center justify-between">
    <div class="relative min-w-0 flex-1">
      <Search class="pointer-events-none absolute inset-s-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
      <Input
        bind:value={searchQuery}
        placeholder={m.common_header_search()}
        class="h-8 w-full bg-background/70 ps-9 pe-8 text-xs"
      />
      {#if searchQuery}
        <Button
          type="button"
          variant="ghost"
          size="icon"
          onclick={() => (searchQuery = "")}
          class="absolute inset-e-1.5 top-1/2 h-7 w-7 -translate-y-1/2 text-muted-foreground hover:text-foreground"
          aria-label={m.common_filter_clear()}
        >
          <XMark class="h-3.5 w-3.5" />
        </Button>
      {/if}
    </div>

    <Button
      variant="ghost"
      size="icon"
      class="h-8 w-8"
      disabled={documentsQuery.isFetching}
      onclick={() => documentsQuery.refetch()}
      title={m.common_refresh()}
    >
      <Refresh class="h-4 w-4 {documentsQuery.isFetching ? 'animate-spin' : ''}" />
    </Button>

    <Button size="sm" disabled={isUploading} onclick={openFilePicker}>
      <ArrowUpTray class="me-1.5 h-3.5 w-3.5" />
      {m.common_docs_btn_upload()}
    </Button>
  </header>

  <main class="flex-1 overflow-y-auto p-4">
    {#if isLoading}
      <ArchiveLoading />
    {:else if queryError}
      <div class="flex min-h-64 flex-col items-center justify-center gap-3 text-center">
        <p class="text-sm font-semibold text-destructive">{m.common_loading_error()}</p>
        <p class="max-w-lg text-xs text-muted-foreground">{queryError}</p>
        <Button variant="outline" size="sm" onclick={() => documentsQuery.refetch()}>
          <Refresh class="me-1.5 h-3.5 w-3.5" />
          {m.common_retry()}
        </Button>
      </div>
    {:else if documents.length === 0}
      <StudentSectionState
        icon={DocumentPage}
        title={m.common_docs_empty()}
        description={m.common_docs_empty_hint()}
      >
        {#snippet action()}
          <Button size="sm" disabled={isUploading} onclick={openFilePicker}>
            <ArrowUpTray class="me-1.5 h-3.5 w-3.5" />
            {m.common_docs_add_btn()}
          </Button>
        {/snippet}
      </StudentSectionState>
    {:else if filteredDocuments.length === 0}
      <StudentSectionState
        icon={Search}
        title={m.common_search_no_results()}
      >
        {#snippet action()}
          <Button variant="outline" size="sm" onclick={() => (searchQuery = "")}>
            {m.common_filter_clear()}
          </Button>
        {/snippet}
      </StudentSectionState>
    {:else}
      <div class="grid grid-cols-2 gap-3.5 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6">
        {#each filteredDocuments as document (document.id)}
          <ArchiveDocumentCard
            {document}
            onpreview={handlePreview}
            ondelete={handleDeleteDocument}
          />
        {/each}
      </div>
    {/if}
  </main>
</div>

<ArchivePreviewSheet
  bind:open={isPreviewOpen}
  document={selectedDocument}
  url={selectedDocumentUrl}
  ondelete={handleDeleteDocument}
/>
