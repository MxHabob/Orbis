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

const i18n = {
  academicsGradesRefresh: m.academics_grades_refresh(),
  commonLoadingError: m.common_loading_error(),
  commonRefresh: m.common_refresh(),
  commonRetry: m.common_retry(),
  docActionCreateReq: m.common_docs_action_create_req(),
  docActionOpenFile: m.common_docs_action_open_file(),
  docActionShowInFolder: m.common_docs_action_show_in_folder(),
  docAddBtn: m.common_docs_add_btn(),
  docBtnUpload: m.common_docs_btn_upload(),
  docColDate: m.common_docs_col_date(),
  docColName: m.common_docs_col_name(),
  docCreateSuccess: m.common_docs_create_success(),
  docDeleteConfirm: m.common_docs_delete_confirm(),
  docDeleteSuccess: m.common_docs_delete_success(),
  docDetailDescLabel: m.common_docs_detail_desc_label(),
  docDetailDocLabel: m.common_docs_detail_doc_label(),
  docEmpty: m.common_docs_empty(),
  docEmptyHint: m.common_docs_empty_hint(),
  docFormDescLabel: m.common_docs_form_desc_label(),
  docFormDescPlaceholder: m.common_docs_form_desc_placeholder(),
  docFormNameArLabel: m.common_docs_form_name_ar_label(),
  docFormNameArPlaceholder: m.common_docs_form_name_ar_placeholder(),
  docFormNameEnLabel: m.common_docs_form_name_en_label(),
  docFormNameRequired: m.common_docs_form_name_required(),
  docFormUrlLabel: m.common_docs_form_url_label(),
  docFormUrlPlaceholder: m.common_docs_form_url_placeholder(),
  docStatusPending: m.common_docs_status_pending(),
  docStatusRequirement: m.common_docs_status_requirement(),
  docStatusUploaded: m.common_docs_status_uploaded(),
  docSubtitle: m.common_docs_subtitle(),
  docTitle: m.common_docs_title(),
  docUploadDialogTitle: m.common_docs_upload_dialog_title(),
  docUploadSuccess: m.common_docs_upload_success(),
  docUploadUrlRequired: m.common_docs_upload_url_required(),
};

const documentStateLabels = {
  title: m.common_docs_title(),
  subtitle: m.common_docs_subtitle(),
  actionCreateReq: m.common_docs_action_create_req(),
  actionOpenFile: m.common_docs_action_open_file(),
  addBtn: m.common_docs_add_btn(),
  uploadBtn: m.common_docs_btn_upload(),
  empty: m.common_docs_empty(),
  emptyHint: m.common_docs_empty_hint(),
  formNameArLabel: m.common_docs_form_name_ar_label(),
  formNameArPlaceholder: m.common_docs_form_name_ar_placeholder(),
  formNameEnLabel: m.common_docs_form_name_en_label(),
  formDescLabel: m.common_docs_form_desc_label(),
  formDescPlaceholder: m.common_docs_form_desc_placeholder(),
  formUrlLabel: m.common_docs_form_url_label(),
  formUrlPlaceholder: m.common_docs_form_url_placeholder(),
  statusRequirement: m.common_docs_status_requirement(),
  statusPending: m.common_docs_status_pending(),
  statusUploaded: m.common_docs_status_uploaded(),
  colName: m.common_docs_col_name(),
  colDate: m.common_docs_col_date(),
  uploadDialogTitle: m.common_docs_upload_dialog_title(),
  uploadSuccess: m.common_docs_upload_success(),
  createSuccess: m.common_docs_create_success(),
  deleteConfirm: m.common_docs_delete_confirm(),
  deleteSuccess: m.common_docs_delete_success(),
  detailDocLabel: m.common_docs_detail_doc_label(),
  detailDescLabel: m.common_docs_detail_desc_label(),
  uploadUrlRequired: m.common_docs_upload_url_required(),
};

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
    title: m.common_docs_delete_success(),
    description: m.common_docs_delete_confirm(),
    confirmText: m.common_action_delete(),
    variant: "destructive",
  });

  if (!confirmed) return;

  try {
    await trashMutation.mutateAsync(id);
    toast.success(m.common_docs_delete_success());
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
        <button
          type="button"
          onclick={() => (searchQuery = "")}
          class="absolute inset-e-2.5 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground"
          aria-label={m.common_filter_clear()}
        >
          <XMark class="h-3.5 w-3.5" />
        </button>
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
      <div class="flex min-h-80 flex-col items-center justify-center text-center text-muted-foreground">
        <div class="mb-4 flex h-16 w-16 items-center justify-center rounded-2xl bg-muted/50 text-muted-foreground/60 shadow-inner">
          <DocumentPage class="h-8 w-8" />
        </div>
        <h2 class="text-sm font-bold text-foreground">{m.common_docs_empty()}</h2>
        <p class="mt-1 max-w-sm text-xs text-muted-foreground">
          {m.common_docs_empty_hint()}
        </p>
        <Button class="mt-5" size="sm" disabled={isUploading} onclick={openFilePicker}>
          <ArrowUpTray class="me-1.5 h-3.5 w-3.5" />
          {m.common_docs_add_btn()}
        </Button>
      </div>
    {:else if filteredDocuments.length === 0}
      <div class="flex min-h-64 flex-col items-center justify-center text-center text-muted-foreground">
        <Search class="mb-3 h-7 w-7 opacity-50" />
        <h2 class="text-sm font-semibold text-foreground">{m.common_search_no_results()}</h2>
        <Button class="mt-3" variant="outline" size="sm" onclick={() => (searchQuery = "")}>
          {m.common_filter_clear()}
        </Button>
      </div>
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
